use futures::{Future,Stream,Async,Poll};
use redis_async::client::paired::SendBox;
use redis::RedisNonBlock;
use common::Identity;
use edge::auth::util::{
    conn_lease_keys,
    auth_lease_keys,
};
use edge::Error;


/// stream which attempts to renew auth leases when polled
pub struct RenewalStream<T> {
    inner: T,
    ident: Identity,
    conn: bool,
    work: Option<SendBox<i64>>,
}



impl<T> RenewalStream<T> {

    pub fn new(inner: T, ident: Identity) -> Self {
        let (conn,work) = Default::default();
        Self { inner, ident, conn, work }
    }
}

impl<T> RenewalStream<T> where T: RedisNonBlock {

    /// construct future for `conn` acquisition
    fn conn_work(&self) -> SendBox<i64> {
        let (src,dst) = conn_lease_keys(self.ident.role);
        self.inner.smove(src,dst,self.ident.to_string())
    }

    /// construct future for `auth` acquisition
    fn auth_work(&self) -> SendBox<i64> {
        let (src,dst) = auth_lease_keys(self.ident.role);
        self.inner.smove(src,dst,self.ident.to_string())
    }

    /// poll current work future, instantiating new work
    /// if appropriate
    fn poll_work(&mut self) -> Poll<bool,Error> {
        if !self.work.is_some() {
            if self.conn {
                // `conn` acquired, attempt to acquire
                // `auth`...
                self.work = Some(self.auth_work());
            } else {
                // `conn` not yet held...
                self.work = Some(self.conn_work());
            }
        }
        let rslt = try_ready!(self.work.as_mut()
            .expect("`work` must be `Some`").poll());
        Ok(Async::Ready(rslt > 0))
    }
}


impl<T> Stream for RenewalStream<T> where T: RedisNonBlock {

    type Item = Renewal;

    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        // poll current work (or instantiate if none exists)
        let rslt = try_ready!(self.poll_work());
        // if we got this far, the current work future
        // has been depleted and must be discarded
        let _ = self.work.take();
        if self.conn {
            // conn was previously acquired... clear
            // state variable and return renewal...
            self.conn = false;
            let renewal = Renewal {
                connectivity: true,
                authority: rslt
            };
            Ok(Async::Ready(Some(renewal)))
        } else if rslt {
            // successfully acquired conn, update state
            // variable and re-poll...
            self.conn = true;
            self.poll()
        } else {
            // failed to acquire conn, return an empty
            // renewal...
            Ok(Async::Ready(Some(Default::default())))
        }
    }
}


/// struct indicating which, if any, leases were
/// successfully renewed
#[derive(Default,Debug,Copy,Clone)]
pub struct Renewal {
    pub connectivity: bool,
    pub authority: bool,
}

