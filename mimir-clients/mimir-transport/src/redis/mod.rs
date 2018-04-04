//! fundamental building blocks for redis based
//! producer/consumer models.
//!

mod handle;
mod watch;

pub use self::handle::RedisHandle;
pub use self::watch::WatchCommand;


/// helpers used to directly instantiate
/// common connection abstractions.
pub mod spawn {
    use futures::future::{Future,Executor};
    use redis_async::error::Error; 
    use redis_async::client;
    use std::net::SocketAddr; 
    use redis::RedisHandle;
    use redis::WatchCommand;


    
    type FutureHandle = Box<Future<Item=RedisHandle,Error=Error>>;


    /// spawn a coneable handle to a redis connection.
    ///
    pub fn handle<E>(addr: &SocketAddr, executor: E) -> FutureHandle
        where E: Executor<Box<Future<Item=(),Error=()> + Send>> + 'static
    {
        let pair_conn = client::paired_connect(addr,executor);
        let handle = pair_conn.map(|conn| RedisHandle::from(conn));
        Box::new(handle)
    }

    type FutureWatch = Box<Future<Item=WatchCommand<(String,String)>,Error=Error>>;

    /// spawn a stream which consumes and yields elements
    /// from one or more redis lists.
    ///
    pub fn watcher<E,L,S>(addr: &SocketAddr, executor: E, lists: L) -> FutureWatch
        where E: Executor<Box<Future<Item=(),Error=()> + Send>> + 'static,
              L: AsRef<[S]> + 'static, S: AsRef<str> + 'static
    {
        let pair_conn = client::paired_connect(addr,executor);
        let watcher = pair_conn.map(move |conn| WatchCommand::brpop(lists.as_ref(),conn));
        Box::new(watcher)
    }
}

