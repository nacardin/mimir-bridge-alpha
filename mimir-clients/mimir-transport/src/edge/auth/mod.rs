
pub(crate) mod util;
mod lease;
mod renew;
mod policy;

pub(crate) use self::renew::{
    RenewalStream,
    Renewal,
};
pub use self::lease::{
    LeaseServer,
    AcquireLease,
    HoldLease,
};
pub use self::policy::{
    AuthPolicy,
    Policy,
};

use futures::{future,Future,Async,Poll};
use common::Identity;
use edge::Error;


/// cool kids make the rules, they don't break the rules! 
///
pub trait AuthServer {

    /// errors raised by this auth server
    type Error;

    /// future which attempts to acquire authorization
    type AcquireFuture: Future<Item=Self::HoldFuture,Error=Self::Error> + 'static;

    /// future which must be polled to maintain authorization.
    /// authorization is lost when future resolves.
    type HoldFuture: Future<Item=(),Error=Self::Error> + 'static;

    /// acquire authorization
    fn authorize(&self, identity: Identity) -> Self::AcquireFuture;

    /// run specified future while authorization is held
    fn while_authorized<F>(&self, identity: Identity, work: F) -> Box<Future<Item=(),Error=Self::Error>> 
            where F: Future<Item=()> + 'static, Self::Error: From<F::Error> {
        let acquire = self.authorize(identity);
        let work = acquire.and_then(move |hold: Self::HoldFuture| {
            info!("got auth for {}",identity);
            hold.select(work.map_err(|e|Self::Error::from(e))).map(|_|())
                .map_err(|(e,_)|e)
        });
        Box::new(work)
    }
}


#[derive(Debug,Copy,Clone)]
pub struct DebugAuthServer;

/// future which never resolves
pub(crate) struct NeverFuture;


impl Future for NeverFuture {

    type Item = ();

    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> { Ok(Async::NotReady) }
}


impl AuthServer for DebugAuthServer {

    type Error = Error;

    type AcquireFuture = Box<Future<Item=Self::HoldFuture,Error=Self::Error>>;

    type HoldFuture = Box<Future<Item=(),Error=Self::Error>>;

    fn authorize(&self, identity: Identity) -> Self::AcquireFuture {
        Box::new(future::ok(()).map(move |_| {
            warn!("unconditionally authorizing {}",identity);
            Box::new(NeverFuture) as Box<Future<Item=(),Error=Error>>
        }))
    }
}

