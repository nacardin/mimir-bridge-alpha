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




