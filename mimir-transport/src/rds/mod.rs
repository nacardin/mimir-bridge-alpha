//! fundamental building blocks for redis based producer/consumer models.
//!
//! This module exposes separate "blocking" and "non-blocking" redis handles
//! for the blocking and non-blocking subsets of redis commands (both types of
//! handle are asynchronous rust code).
//!
//! ```
//! extern crate mimir_transport;
//! extern crate tokio;
//! 
//! use mimir_transport::rds as redis;
//! use tokio::prelude::*;
//! 
//! # fn example() {
//! 
//! let address = "127.0.0.1:6379".parse().unwrap();
//! 
//! let targets = vec!["some-list-0".to_string(),"some-list-1".to_string()];
//! 
//! let redis_task = redis::spawn_blocking(&address).and_then(|redis_handle| {
//! 
//!     redis::pop_stream(redis_handle, targets)
//!         .for_each(|(list, value)| {
//!             println!("list {}, value {}", list, value);
//!             Ok(())
//!           })
//! }).map_err(|x| {
//! 
//!     println!("error: {:?}", x);
//!     ()
//! });
//! 
//! tokio::run(redis_task);
//! 
//! # }
//! ```
//!

use redis_async::error::Error;
use std::net::SocketAddr;
use tokio::prelude::*;
use std::iter;

pub(crate) mod commands;
mod handles;
mod util;

pub use self::handles::{
    RedisBlocking,
    RedisNonBlock,
};
use self::util::DropSink;


/// spawn a blocking redis handle
///
pub fn spawn_blocking(addr: &SocketAddr) -> impl Future<Item=RedisBlocking,Error=Error> {
    RedisBlocking::connect(addr)
}


/// spawn a non-blocking redis handle
///
pub fn spawn_nonblock(addr: &SocketAddr) -> impl Future<Item=RedisNonBlock,Error=Error> {
    RedisNonBlock::connect(addr)
}

/// build a stream which pops from one or more redis lists
///
pub fn pop_stream(redis_handle: RedisBlocking, lists: Vec<String>) -> impl Stream<Item=(String,String),Error=Error> {
    stream::iter_ok(iter::repeat(()))
        .and_then(move |()| {
            redis_handle.brpop(lists.clone(),None)
        })
}


/// build a sink which pushes to redis lists
///
pub fn push_sink(redis_handle: RedisNonBlock) -> impl Sink<SinkItem=(String,String),SinkError=Error> {
    let base: DropSink<(),Error> = Default::default();
    base.with(move |(key,val)| {
        redis_handle.lpush(key,Some(val)).map(|_|())
    })
}




