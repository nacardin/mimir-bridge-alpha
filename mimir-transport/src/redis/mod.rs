//! fundamental building blocks for redis based producer/consumer models.
//!
//! This module exposes separate "blocking" and "non-blocking" redis handles
//! for the blocking and non-blocking subsets of redis commands (both types of
//! handle are asynchronous rust code).
//!
//! TODO: example
//!

pub(crate) mod commands;
mod pop_stream;
mod push_sink;
mod blocking;
mod nonblock;


pub use self::blocking::{
    RedisBlocking,
    BlockingHandle,
    Blocking,
};
pub use self::nonblock::{
    RedisNonBlock,
    NonBlockHandle,
    NonBlock,
};

pub use self::pop_stream::PopStream;
pub use self::push_sink::PushSink;

pub use redis_async::client::paired::PairedConnection;
pub use redis_async::error::Error;
use redis_async::resp;

use futures::future::Future;
use std::net::SocketAddr;


/// spawn new blocking redis handle
///
pub fn spawn_blocking(address: &SocketAddr) -> SendBox<BlockingHandle> {
    BlockingHandle::new(address)
}


/// spanw new nonblocking redis handle
///
pub fn spawn_nonblock<E>(address: &SocketAddr) -> SendBox<NonBlockHandle> {
    NonBlockHandle::new(address)
}

pub type SendBox<T> = Box<Future<Item = T, Error = Error> + Send>;

pub struct PairedConnectionBoxFuture(PairedConnection);

impl PairedConnectionBoxFuture {
    pub fn send<T>(&self, msg: resp::RespValue) -> SendBox<T> where T: resp::FromResp + Send + 'static {
        Box::new(self.0.send(msg))
    }
}
