use redis_async::client::{self,PairedConnection};
use redis_async::error::Error;
use std::net::SocketAddr;
use tokio::prelude::*;
use super::commands;

/// redis handle for blocking commands
///
pub struct RedisBlocking {
    inner: PairedConnection
}


impl RedisBlocking {

    /// connect to a redis instance
    pub fn connect(addr: &SocketAddr) -> impl Future<Item=Self,Error=Error> {
        client::paired_connect(addr).map(|inner| Self { inner } )
    }

    /// BRPOP -- blocking right-handed pop from one or more lists
    pub fn brpop<K: IntoIterator<Item=String>>(&self, keys: K, timeout: Option<u64>) -> impl Future<Item=(String,String),Error=Error> {
        self.inner.send(commands::brpop(keys,timeout))
    }
}


/// redis handle for nonblocking commands
///
#[derive(Clone)]
pub struct RedisNonBlock {
    inner: PairedConnection
}


impl RedisNonBlock {

    /// connect to a redis instance
    pub fn connect(addr: &SocketAddr) -> impl Future<Item=Self,Error=Error> {
        client::paired_connect(addr).map(|inner| Self { inner } )
    }

    /// SADD -- add member(s) to set
    pub fn sadd<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> impl Future<Item=i64,Error=Error> {
        self.inner.send(commands::sadd(key,members))
    }

    /// SREM -- remove matching member(s) from set
    pub fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> impl Future<Item=i64,Error=Error> {
        self.inner.send(commands::srem(key,members))
    }

    /// SMOVE -- move matching member from one set to another
    pub fn smove<K: Into<String>, M: Into<String>>(&self, source: K, dest: K, member: M) -> impl Future<Item=i64,Error=Error> {
        self.inner.send(commands::smove(source,dest,member))
    }

    /// RPOP -- right-handed pop from a member from list
    pub fn rpop<K: Into<String>>(&self, key: K) -> impl Future<Item=Option<String>,Error=Error> {
        self.inner.send(commands::rpop(key))
    }

    /// LPUSH -- left-handed push member(s) to list
    pub fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> impl Future<Item=i64,Error=Error> {
        self.inner.send(commands::lpush(key,values))
    }
}
