use futures::future::{Future,Executor};
use redis_async::client::paired::{
    PairedConnection,
    SendBox,
};
use redis_async::error::Error;
use redis_async::client;
use redis::commands;
use std::net::SocketAddr;
use std::sync::Arc;
use std::rc::Rc;


/// wrapper which restricts a redis connection to non-blocking commands only
///
#[derive(Clone)]
pub struct NonBlock<T> {
    inner: T
}


/// shareable redis handle for execution of non-blocking redis commands
pub type NonBlockHandle = NonBlock<Arc<PairedConnection>>;


impl NonBlockHandle {

    /// instantiate new nonblocking handle instance
    pub fn new<E>(addr: &SocketAddr, executor: E) -> Box<Future<Item=Self,Error=Error>> 
            where E: Executor<Box<Future<Item=(),Error=()> + Send>> + 'static {
        let work = client::paired_connect(addr,executor)
            .map(|conn| {
                let inner = Arc::new(conn);
                Self { inner }
            });
        Box::new(work)
    }
}


impl<T> NonBlock<T> where T: RedisNonBlock {
    /// SREM -- remove matching member(s) from set
    pub fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64> { self.inner.srem(key,members) }

    /// RPOP -- non-blocking right-handed pop
    pub fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>> { self.inner.rpop(key) }

    /// LPUSH -- non-blocking left-handed push
    pub fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64> { self.inner.lpush(key,values) }
}


impl<T> RedisNonBlock for NonBlock<T> where T: RedisNonBlock {
 
    fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64> { self.srem(key,members) }

    fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>> { self.rpop(key) }

    fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64> { self.lpush(key,values) }

}


/// nonblocking redis commands
pub trait RedisNonBlock {
    
    /// SREM -- remove matching member(s) from set
    fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64>;

    /// RPOP -- non-blocking right-handed pop
    fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>>;

    /// LPUSH -- non-blocking left-handed push
    fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64>;
}


impl RedisNonBlock for PairedConnection {

    fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64> {
        self.send(commands::srem(key,members))
    }

    fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>> {
        self.send(commands::rpop(key))
    }

    fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64> {
        self.send(commands::lpush(key,values))
    }
}


impl<'a,T> RedisNonBlock for &'a T where T: RedisNonBlock {

    fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64> {
        <T as RedisNonBlock>::srem(self,key,members)
    }

    fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>> {
        <T as RedisNonBlock>::rpop(self,key)
    }

    fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64> {
        <T as RedisNonBlock>::lpush(self,key,values)
    }
}


impl<T> RedisNonBlock for Arc<T> where T: RedisNonBlock {

    fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64> {
        <T as RedisNonBlock>::srem(self,key,members)
    }

    fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>> {
        <T as RedisNonBlock>::rpop(self,key)
    }

    fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64> {
        <T as RedisNonBlock>::lpush(self,key,values)
    }
}


impl<T> RedisNonBlock for Rc<T> where T: RedisNonBlock {

    fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64> {
        <T as RedisNonBlock>::srem(self,key,members)
    }

    fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>> {
        <T as RedisNonBlock>::rpop(self,key)
    }

    fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64> {
        <T as RedisNonBlock>::lpush(self,key,values)
    }
}


impl<T> RedisNonBlock for Box<T> where T: RedisNonBlock {

    fn srem<K: Into<String>, M: IntoIterator<Item=String>>(&self, key: K, members: M) -> SendBox<i64> {
        <T as RedisNonBlock>::srem(self,key,members)
    }

    fn rpop<K: Into<String>>(&self, key: K) -> SendBox<Option<String>> {
        <T as RedisNonBlock>::rpop(self,key)
    }

    fn lpush<K: Into<String>, V: IntoIterator<Item=String>>(&self, key: K, values: V) -> SendBox<i64> {
        <T as RedisNonBlock>::lpush(self,key,values)
    }
}

