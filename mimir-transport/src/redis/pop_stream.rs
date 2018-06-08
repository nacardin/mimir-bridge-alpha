use futures::{Stream,Async,Poll};
use redis::{
    RedisBlocking,
    BlockingHandle,
    SendBox,
    Error,
};


/// stream of elements popped from one or more lists.
///
/// produces tuples of the form `(key,value)`, where `key` is one
/// of the list keys being watched, and `value` is the value popped
/// from the list.
///
pub struct PopStream<T> {
    inner: T,
    keys: Vec<String>,
    work: Option<SendBox<(String,String)>>
}


impl PopStream<BlockingHandle> {

    /// instantiate new `PopStream` instance for specified keys
    ///
    /// *note*: all keys must correspond to list values, and the
    /// `BlockingHandle` instance must not have any outstanding futures
    /// against it.
    ///
    pub fn new(inner: BlockingHandle, keys: Vec<String>) -> Self {
        let work = None;
        Self { inner, keys, work }
    }
}


impl<T> PopStream<T> where T: RedisBlocking {

    #[inline]
    fn get_work_mut(&mut self) -> &mut SendBox<(String,String)> {
        if self.work.is_none() {
            self.work = Some(self.inner.brpop(self.keys.clone()));
        }
        self.work.as_mut().expect("is always `Some` variant")
    }
}


impl<T> Stream for PopStream<T> where T: RedisBlocking {

    type Item = (String,String);

    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        // get reference to work future & poll
        let item = try_ready!(self.get_work_mut().poll());

        // if we got this far, current work future is complete &
        // must be discarded.
        let _ = self.work.take();

        // return produced item
        Ok(Async::Ready(Some(item)))
    }
}
