//! asynchronous cross-task services
//!
//! The `spawn` function can be used to create an asynchronous service with
//! (optional) mutable state.  A cloneable/sendable handle is returned which
//! allows for asynchronous querying of the service from multiple tasks.
//! 
//! ## example
//!
//! The below example uses `spawn` to create a service which allows multiple tasks
//! to make insertions and deletions against a single shared hashmap via a cloneable
//! handle.
//!
//! ```
//! extern crate mimir_transport;
//! extern crate tokio;
//!
//! use std::collections::HashMap;
//! use mimir_transport::service;
//! use tokio::prelude::*;
//!
//!
//! #[derive(Debug)]
//! enum Op<K,V> {
//!     Put(K,V),
//!     Take(K),
//! }
//!
//! # fn main() {
//! 
//! // `service::spawn` cannot be called outside of the event-loop, so
//! // we wrap our setup logic in a closure to defer execution.
//! let spawn_service = || {
//!     let mut data = HashMap::new();
//!     let handle = service::spawn(move |op| {
//!         match op {
//!             Op::Put(key,val) => Ok(data.insert(key,val)),
//!             Op::Take(key) => Ok(data.remove(key)),
//!         }
//!     });
//!     Ok(handle)
//! };
//!
//!
//! let make_requests = future::lazy(spawn_service).and_then(|handle| {
//!     
//!     let requests = vec![
//!         handle.call(Op::Put("hello","world")),
//!         handle.call(Op::Put("hello","world!")),
//!         handle.call(Op::Take("hello")),
//!     ];
//!
//!     future::collect(requests).then(|result| {
//!         assert_eq!(result.unwrap(),vec![None,Some("world"),Some("world!")]);
//!         Ok(())
//!     })
//! });
//!
//! tokio::run(make_requests);
//!
//! # }
//! ```
//!
use futures::sync::{mpsc,oneshot};
use tokio::prelude::*;
use tokio;
use std::{fmt,error};


/// spawn a service to the event loop
///
/// spawns a threadsafe service to the current event loop, returning a 
/// cloneable/sendable handle.  see module-level documentation for example usage.
///
/// ## panics
///
/// this function will panic if called outside of an event-loop.
///
pub fn spawn<S,Req,Rsp>(mut service: S) -> ServiceHandle<Req,Rsp> 
        where S: Service<Req,Rsp,Error=()> + Send + 'static, S::Future: Send + 'static,
              Req: Send + 'static, Rsp: Send + 'static {
    let (tx,rx) = mpsc::unbounded();
    let handle = ServiceHandle::new(tx);
    let serve_request = move |request: Request<_,_>| {
        let Request { call, tx } = request;
        service.call(call).and_then(move |rslt| {
            let _ = tx.send(rslt);
            Ok(())
        })
    };
    let work = rx.map(serve_request)
        .buffer_unordered(64)
        .for_each(|()| Ok(()));
    tokio::spawn(work);
    handle
}


/// an asynchronous service
///
pub trait Service<Req,Rsp> {

    /// error raised if service fails
    type Error;

    /// type of future this service returns
    type Future: Future<Item=Rsp,Error=Self::Error>;

    /// execute a call against this service
    fn call(&mut self, req: Req) -> Self::Future;
}


impl<F,T,Req> Service<Req,T::Item> for F where F: FnMut(Req) -> T, T: IntoFuture {

    type Error = T::Error;

    type Future = T::Future;

    fn call(&mut self, req: Req) -> Self::Future {
        (self)(req).into_future()
    }
}



#[derive(Debug)]
struct Request<C,R> {
    call: C,
    tx: oneshot::Sender<R>,
}



#[derive(Debug,Clone)]
pub struct ServiceHandle<C,R> {
    inner: mpsc::UnboundedSender<Request<C,R>>,
}



impl<C,R> ServiceHandle<C,R> {

    fn new(inner: mpsc::UnboundedSender<Request<C,R>>) -> Self { Self { inner } }

    pub fn call(&self, call: C) -> impl Future<Item=R,Error=Error> {
        let (tx,rx) = oneshot::channel();
        let req = Request { call, tx };
        self.inner.unbounded_send(req).into_future().from_err()
            .and_then(move |()| rx.from_err())
    }
}



#[derive(Debug,Copy,Clone)]
pub enum Error {
    SendError,
    Canceled
}


impl Error {

    fn as_str(&self) -> &'static str {
        match self {
            Error::SendError => "unable to send request (receiver dropped)",
            Error::Canceled => "request canceled (rsp channel dropped)",
        }
    }
}


impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}


impl error::Error for Error {

    fn description(&self) -> &str { self.as_str() }
}


impl<T> From<mpsc::SendError<T>> for Error {

    fn from(_: mpsc::SendError<T>) -> Self { Error::SendError }
}


impl From<oneshot::Canceled> for Error {

    fn from(_: oneshot::Canceled) -> Self { Error::Canceled }
}
