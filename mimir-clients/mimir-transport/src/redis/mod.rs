//! fundamental building blocks for redis based producer/consumer models.
//!
//! This module exposes separate "blocking" and "non-blocking" redis handles
//! for the blocking and non-blocking subsets of redis commands (both types of
//! handle are asynchronous rust code).
//!
//! ```
//! extern crate mimir_transport;
//! extern crate tokio_core;
//! 
//! use mimir_transport::redis::spawn_nonblock;
//! use tokio_core::reactor::Core;
//! # fn example() {
//!
//! let mut core = Core::new().unwrap();
//!
//! let handle = core.handle();
//!
//! let address = "127.0.0.1:6379".parse().unwrap();
//! 
//! // create new redis connection
//! let redis = core.run(spawn_nonblock(&address,handle))
//!     .expect("unable to connect to redis instance");
//! 
//! // execute non-blocking right-pop from list
//! match core.run(redis.rpop("some-list")).unwrap() {
//!     Some(item) => println!("popped item from list: {}",item),
//!     None => println!("list is currently empty"),
//! }
//!
//! # }
//! # fn main() { }
//! ``` 
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

pub use redis_async::client::paired::SendBox;
pub use redis_async::error::Error;

use futures::future::{Future,Executor};
use std::net::SocketAddr;


/// spawn new blocking redis handle
///
pub fn spawn_blocking<E>(address: &SocketAddr, executor: E) -> Box<Future<Item=BlockingHandle,Error=Error>>
        where E: Executor<Box<Future<Item=(),Error=()> + Send>> + 'static {
    BlockingHandle::new(address,executor)
}


/// spanw new nonblocking redis handle
///
pub fn spawn_nonblock<E>(address: &SocketAddr, executor: E) -> Box<Future<Item=NonBlockHandle,Error=Error>>
        where E: Executor<Box<Future<Item=(),Error=()> + Send>> + 'static {
    NonBlockHandle::new(address,executor)
}

