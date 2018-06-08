use futures::future::{Future,Executor};
use redis_async::client::paired::{
    PairedConnection,
    SendBox,
};
use redis_async::error::Error;
use redis_async::client;
use redis::commands;
use std::net::SocketAddr;

/// wrapper which restricts a redis connection to blocking commands only
///
pub struct Blocking<T> {
    inner: T
}

/// exclusive redis handle for execution of blocking redis commands
///
pub type BlockingHandle = Blocking<PairedConnection>;


impl Blocking<PairedConnection> {

    /// instantiate new blocking handle instance
    pub fn new<E>(addr: &SocketAddr, executor: E) -> Box<Future<Item=Self,Error=Error>> 
            where E: Executor<Box<Future<Item=(),Error=()> + Send>> + 'static {
        let work = client::paired_connect(addr,executor)
            .map(|inner| Self { inner });
        Box::new(work)
    }
}


impl<T> Blocking<T> where T: RedisBlocking {

    /// BRPOP -- blocking right-handed pop for one or more lists
    pub fn brpop<K: IntoIterator<Item=String>>(&mut self, keys: K) -> SendBox<(String,String)> { self.inner.brpop(keys) }

    /// same as BRPOP, but with timeout supplied.  `None` is returned if timeout is reached.
    pub fn brpop_with_timeout<K: IntoIterator<Item=String>>(&mut self, keys: K, timeout: u64) -> SendBox<Option<(String,String)>> {
        self.inner.brpop_with_timeout(keys,timeout)
    }
}


impl<T> RedisBlocking for Blocking<T> where T: RedisBlocking {

    fn brpop<K: IntoIterator<Item=String>>(&mut self, keys: K) -> SendBox<(String,String)> { self.brpop(keys) }

    fn brpop_with_timeout<K: IntoIterator<Item=String>>(&mut self, keys: K, timeout: u64) -> SendBox<Option<(String,String)>> {
        self.brpop_with_timeout(keys,timeout)
    }
}


/// blocking redis commands
pub trait RedisBlocking {

    /// BRPOP -- blocking right-handed pop for one or more lists
    fn brpop<K: IntoIterator<Item=String>>(&mut self, keys: K) -> SendBox<(String,String)>;

    /// same as BRPOP, but with timeout supplied.  `None` is returned if timeout is reached.
    fn brpop_with_timeout<K: IntoIterator<Item=String>>(&mut self, keys: K, timeout: u64) -> SendBox<Option<(String,String)>>;
}


impl RedisBlocking for PairedConnection {

    fn brpop<K: IntoIterator<Item=String>>(&mut self, keys: K) -> SendBox<(String,String)> {
        self.send(commands::brpop(keys,None))
    }

    fn brpop_with_timeout<K: IntoIterator<Item=String>>(&mut self, keys: K, timeout: u64) -> SendBox<Option<(String,String)>> {
        self.send(commands::brpop(keys,Some(timeout)))
    }
}


impl<'a,T> RedisBlocking for &'a mut T where T: RedisBlocking {

    fn brpop<K: IntoIterator<Item=String>>(&mut self, keys: K) -> SendBox<(String,String)> {
        <T as RedisBlocking>::brpop(self,keys)
    }

    fn brpop_with_timeout<K: IntoIterator<Item=String>>(&mut self, keys: K, timeout: u64) -> SendBox<Option<(String,String)>> {
        <T as RedisBlocking>::brpop_with_timeout(self,keys,timeout)
    }
}


impl<T> RedisBlocking for Box<T> where T: RedisBlocking {

    fn brpop<K: IntoIterator<Item=String>>(&mut self, keys: K) -> SendBox<(String,String)> {
        <T as RedisBlocking>::brpop(self,keys)
    }

    fn brpop_with_timeout<K: IntoIterator<Item=String>>(&mut self, keys: K, timeout: u64) -> SendBox<Option<(String,String)>> {
        <T as RedisBlocking>::brpop_with_timeout(self,keys,timeout)
    }
}
