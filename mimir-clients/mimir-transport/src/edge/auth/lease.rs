use futures::future::Future;
use futures::{Stream,Async,Poll};
use std::time::{Duration,Instant};
use tokio_timer::Interval;
use redis::RedisNonBlock;
use common::Identity;
use edge::util::Limit;
use edge::Error;
use edge::auth::{
    AuthServer,
    RenewalStream,
    Renewal,
};


/// `AuthServer` implementor which relies on "lease" style 
/// polling & renewal patterns to enforce permissions.
///
pub struct LeaseServer<T> {
    redis_handle: T,
    lease_config: LeaseConfig,
}

impl<T> LeaseServer<T> {

    pub fn new(redis_handle: T, base_duration: Duration) -> Self {
        let lease_config = LeaseConfig::new(base_duration);
        Self { redis_handle, lease_config }
    }
}


impl<T> AuthServer for LeaseServer<T> where T: RedisNonBlock + Clone + 'static {

    type Error = Error;

    type AcquireFuture = AcquireLease<T>;

    type HoldFuture = HoldLease<T>;

    fn authorize(&self, identity: Identity) -> Self::AcquireFuture {
        let config = self.lease_config.clone();
        let redis = self.redis_handle.clone();
        let stream = LeaseStream::new(config,redis,identity);
        AcquireLease::new(stream)
    }
}


/// future which drives acquisition of auth lease
///
pub struct AcquireLease<T> {
    inner: Option<LeaseStream<T>>
}


impl<T> AcquireLease<T> {

    fn new(inner: LeaseStream<T>) -> Self { Self { inner: Some(inner) } }
}


impl<T> Future for AcquireLease<T> where T: RedisNonBlock {

    type Item = HoldLease<T>;

    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        loop {
            let rslt = try_ready!(self.inner.as_mut()
                .expect("no polling past completion").poll());
            match rslt {
                Some(LeaseLevel::ConnectOnly) => {
                    // acquisiton in progress... continue polling...
                },
                Some(LeaseLevel::FullAbilities) => {
                    // full lease acquired... yield holding future...
                    let inner = self.inner.take()
                        .expect("must still be `Some`");
                    return Ok(Async::Ready(HoldLease::new(inner)));
                },
                _ => {
                    // acquisition failed... drop inner stream
                    // and return error...
                    let _ = self.inner.take();
                    return Err("unable to acquire auth lease".into());
                }
            }
        }
    }
}


/// future which drives maintenance of auth lease
///
pub struct HoldLease<T> {
    inner: Option<LeaseStream<T>>
}


impl<T> HoldLease<T> {

    fn new(inner: LeaseStream<T>) -> Self { Self { inner: Some(inner) } }
}


impl<T> Future for HoldLease<T> where T: RedisNonBlock {

    type Item = ();

    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        loop {
            let rslt = try_ready!(self.inner.as_mut()
                .expect("no polling past completion").poll());
            match rslt {
                Some(LeaseLevel::FullAbilities) => {
                    // full auth lease still held... keep polling...
                },
                Some(_) => {
                    // full lease lost... drop inner stream and resolve
                    // future...
                    let _ = self.inner.take();
                    return Ok(Async::Ready(()));
                },
                None => {
                    // unexpected stream termination, return error...
                    let _ = self.inner.take();
                    return Err("lease stream terminated unexpectedly".into());
                }
            }
        }
    }
}



/// stream which periodically renews and re-calculates leases.
///
/// yields an instance of `LeaseLevel` after each renewal/recalculation.
///
pub struct LeaseStream<T> {
    config: LeaseConfig,
    inner: Limit<RenewalStream<T>>,
    state: LeaseState,
}


impl<T> LeaseStream<T> {

    /// create a new lease stream for `ident`
    /// 
    pub fn new(config: LeaseConfig, redis: T, ident: Identity) -> Self {
        let renewals = RenewalStream::new(redis,ident);
        let inner = Limit::new(config.polling_interval(),renewals);
        let state = Default::default();
        Self { config, inner, state }
    }
}


impl<T> Stream for LeaseStream<T> where T: RedisNonBlock {

    type Item = LeaseLevel;

    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        // poll for next renewal value
        let renewal = try_ready_stream!(self.inner.poll());
        // get instant at which renewal process began
        let instant = self.inner.last_instant()
            .unwrap_or_else(Instant::now);
        // update lease state with latest renewals
        self.state.renew_at(renewal,instant);
        // get expiration relative to latest renewal
        let cutoff = self.config.expire_cutoff_at(instant);
        // apply expiration cutoff to state
        self.state.expire_at(cutoff);
        // return newly calculated lease level
        Ok(Async::Ready(Some(self.state.get_lease_level())))
    }
}


/*
impl<T> AuthServer for LeaseServer<T> where T: RedisNonBlock + Clone {

    type Error = Error;

    type AcquireFuture = Box<Future<Item=Self::HoldFuture,Error=Self::Error>>;

    type HoldFuture = Box<Future<Item=(),Error=Self::Error>>;

    fn authorize(&self, identity: Identity) -> Self::AcquireFuture {
        // TODO...
    }
}
*/


/// default multiplier for polling interval
const POLLING: u32 = 3;
/// default multiplier for seeding interval
const SEEDING: u32 = 7;
/// default multiplier for expiry interval
const EXPIRY: u32 = 11;


/// basic configuration of relavent time intervals
/// for lease seeding/renewal.
///
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct LeaseConfig {
    /// duration upon which lease is polled for renewal
    polling: Duration,

    /// duration between lease seedings
    seeding: Duration,
 
    /// duration after which lease is considered expired
    expiry: Duration,
}


impl LeaseConfig {

    /// instantiate new configuration from base duration value
    ///
    pub fn new(base_duration: Duration) -> Self {
        debug_assert!(base_duration > Duration::new(0,0),"base duration must be nonzero");
        let polling = base_duration * POLLING;
        let seeding = base_duration * SEEDING;
        let expiry = base_duration * EXPIRY;
        Self::from_parts(polling,seeding,expiry)
    }

    /// instantiate new configuration from components
    ///
    pub fn from_parts(polling: Duration, seeding: Duration, expiry: Duration) -> Self {
        debug_assert!(seeding > polling * 2,"seeding should be greater than 2x polling");
        debug_assert!(expiry > seeding + seeding / 2,"expiry should be greater than 1.5x seeding");
        debug_assert!(expiry < seeding * 2,"expiry should be less than 2x seeding");
        Self { polling, seeding, expiry }
    }

    /// get new `Interval` for lease polling
    pub fn polling_interval(&self) -> Interval { Interval::new(Instant::now(),self.polling) }

    /// get new `Interval` for seed triggering
    pub fn seeding_interval(&self) -> Interval { Interval::new(Instant::now(),self.seeding) }

    /// get new `Instant` representing expiration cutoff at specified time
    pub fn expire_cutoff_at(&self, instant: Instant) -> Instant { instant - self.expiry }
}


impl Default for LeaseConfig {

    fn default() -> Self { Self::new(Duration::from_millis(1024)) }
}



/// possible permissioning levels as determined by leases held
///
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum LeaseLevel {
    /// keep connection open, pending full authorization.
    ConnectOnly,
    
    /// authorized for all role capabilities
    FullAbilities,
    
    /// halt any previously authorized activites & disconnect
    RevokeAll
}


/// record of observed auth acquisitions/renewals (leases)
///
#[derive(Default,Debug,Clone,PartialEq,Eq)]
pub struct LeaseState {
    conn_lease: Option<Instant>,
    auth_lease: Option<Instant>,
}



impl LeaseState {

    /// get permission level as determined by currently held leases
    ///
    pub fn get_lease_level(&self) -> LeaseLevel {
        match (self.conn_lease.is_some(),self.auth_lease.is_some()) {
            (true,true) => LeaseLevel::FullAbilities,
            (true,false) => LeaseLevel::ConnectOnly,
            _ => LeaseLevel::RevokeAll,
        }
    }

    /// apply an expiration cutoff
    pub(crate) fn expire_at(&mut self, cutoff: Instant) {
        if self.conn_lease.map(|lease| lease < cutoff).unwrap_or(false) {
            let _ = self.conn_lease.take();
        }
        if self.auth_lease.map(|lease| lease < cutoff).unwrap_or(false) {
            let _ = self.auth_lease.take();
        }
    }

    /// apply renewal for specified instant
    pub(crate) fn renew_at(&mut self, renewal: Renewal, instant: Instant) {
        let Renewal { connectivity, authority } = renewal;
        if connectivity { self.update_conn_lease(instant); }
        if authority { self.update_auth_lease(instant); }
    }

    fn update_conn_lease(&mut self, instant: Instant) { self.conn_lease = Some(instant); }

    fn update_auth_lease(&mut self, instant: Instant) { self.auth_lease = Some(instant); }
}
