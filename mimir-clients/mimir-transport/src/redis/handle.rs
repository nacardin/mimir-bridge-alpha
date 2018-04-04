use redis_async::client::paired::{
    PairedConnection,
    SendBox,
};
use std::sync::Arc;


/// cloneable redis handle.
///
/// *note*: blocking redis ops are not available for this handle since
/// they would block all instances of the handle while waiting.
///
#[derive(Clone)]
pub struct RedisHandle {
    inner: Arc<PairedConnection>,
}


impl RedisHandle {

    pub fn new(conn: PairedConnection) -> Self {
        let inner = Arc::new(conn);
        RedisHandle { inner }
    }

    /// LPUSH -- left push item to specified list.
    ///
    pub fn lpush(&self, list: String, item: String) -> SendBox<i64> {
        self.inner.send(resp_array!["LPUSH",list,item])
    }
}


impl From<PairedConnection> for RedisHandle {

    fn from(conn: PairedConnection) -> Self { Self::new(conn) }
}



