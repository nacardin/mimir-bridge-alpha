use futures::future::{self,Either};
use futures::{Future,Stream,Async,Poll};
use tokio_timer::Interval;
use mimir_types::Address;
use redis::RedisNonBlock;
use std::time::{Instant};
use common::Role;
use edge::Error;


/// stream which periodically attempts to acquire/renew
/// permissions for a specified entitiy.
///
pub struct AuthStream<T> {
    service: T,
    interval: Interval,
    address: Address,
    role: Role,
    state: StreamState,
}


enum StreamState {
    AwaitNextInterval,
    PollAuth { 
        work: Option<Box<Future<Item=Option<AuthRenewal>,Error=Error>>> 
    }
}


impl<T> AuthStream<T> {

    pub fn new(service: T, interval: Interval, address: Address, role: Role) -> Self {
        let state = StreamState::AwaitNextInterval;
        Self { service, interval, address, role, state }
    }
}



impl<T> Stream for AuthStream<T> where T: RedisNonBlock + Clone + 'static {

    type Item = AuthRenewal;

    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        loop {
            let next_state = match self.state {
                StreamState::AwaitNextInterval => {
                    let instant = try_ready_stream!(self.interval.poll());
                    let work = poll_auth(self.service.clone(),instant,self.address,self.role);
                    StreamState::PollAuth { work: Some(work) }
                },
                StreamState::PollAuth { ref mut work } => {
                    if work.is_some() {
                        let renewal = try_ready!(work.as_mut()
                            .expect("always `Some` variant").poll());
                        let _ = work.take();
                        if let Some(renewal) = renewal {
                            return Ok(Async::Ready(Some(renewal)));
                        } else {
                            StreamState::AwaitNextInterval
                        }
                    } else {
                        StreamState::AwaitNextInterval
                    }
                }
            };
            self.state = next_state;
        }
    }
}


/// description of newly acquired/renewed permissions
///
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct AuthRenewal {
    pub(crate) connectivity: bool,
    pub(crate) authority: bool,
    pub(crate) instant: Instant,
}


fn poll_auth<T>(redis: T, time: Instant, addr: Address, role: Role) -> Box<Future<Item=Option<AuthRenewal>,Error=Error>>
        where T: RedisNonBlock + 'static {
    // attempt to acquire `conn` lease (modeled as an
    // SMOVE from a set w/ suffix `-lease` to a set with
    // suffix `-taken`).
    let (src,dst) = conn_lease_keys(role); 
    let conn_future = redis.smove(src,dst,format!("{:#}",addr))
        .map(|moved| moved == 1);

    // conditionally chain an identical acquisition attempt
    // against an `auth` lease if `conn` lease was successfully
    // acquired
    let work = conn_future.and_then(move |conn: bool| {
            if conn {
                let (src,dst) = auth_lease_keys(role);
                let auth_future = redis.smove(src,dst,format!("{:#}",addr))
                    .map(|moved| Some(moved == 1)); 
                Either::A(auth_future)
            } else {
                Either::B(future::ok(None))
            }
        })
        .map(move |rslt: Option<bool>| {
            rslt.map(|auth:bool| {
                AuthRenewal {
                    connectivity: true,
                    authority: auth,
                    instant: time
                }
            })
        }).map_err(|e|Error::from(e));
    Box::new(work)
}


// construct source and destination keys for `conn` lease
// acquisition attempt
fn conn_lease_keys(role: Role) -> (String,String) {
    let mut src = role.to_string();
    src.push_str("::conn-lease");
    let mut dst = role.to_string();
    dst.push_str("::conn-taken");
    (src,dst)
}


// construct source and destination keys for `auth` lease
// acquisition attempt
fn auth_lease_keys(role: Role) -> (String,String) {
    let mut src = role.to_string();
    src.push_str("::auth-lease");
    let mut dst = role.to_string();
    dst.push_str("::auth-taken");
    (src,dst)
}
