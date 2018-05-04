use std::time::{Duration,Instant};
use tokio_timer::Interval;
use edge::AuthRenewal;


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

    /// get new `Instant` representing the current expiration cutoff
    pub fn expire_cutoff(&self) -> Instant { Instant::now() - self.expiry }
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
    pub fn get_auth_state(&self) -> LeaseLevel {
        match (self.conn_lease.is_some(),self.auth_lease.is_some()) {
            (true,true) => LeaseLevel::FullAbilities,
            (true,false) => LeaseLevel::ConnectOnly,
            _ => LeaseLevel::RevokeAll,
        }
    }

    /// apply an expiration cutoff
    pub fn apply_expiration(&mut self, cutoff: Instant) {
        if self.conn_lease.map(|lease| lease < cutoff).unwrap_or(false) {
            let _ = self.conn_lease.take();
        }
        if self.auth_lease.map(|lease| lease < cutoff).unwrap_or(false) {
            let _ = self.auth_lease.take();
        }
    }

    /// apply a renewal at a specified instant
    pub fn apply_renewal(&mut self, renewal: AuthRenewal) {
        let AuthRenewal { connectivity, authority, instant } = renewal;
        if connectivity { self.update_conn_lease(instant); }
        if authority { self.update_auth_lease(instant); }
    }

    fn update_conn_lease(&mut self, instant: Instant) { self.conn_lease = Some(instant); }

    fn update_auth_lease(&mut self, instant: Instant) { self.auth_lease = Some(instant); }
}
