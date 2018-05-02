use futures::{Sink,Async,Poll,StartSend,AsyncSink};
use futures::stream::{Stream,FuturesUnordered};
use redis::{
    RedisNonBlock,
    NonBlockHandle,
    SendBox,
    Error,
};


/// sink which pushes items to lists
///
/// 
pub struct PushSink<T> {
    inner: T,
    work: FuturesUnordered<SendBox<i64>>,
}


impl PushSink<NonBlockHandle> {

    /// build new `PushSink` with specified redis handle
    pub fn new(inner: NonBlockHandle) -> Self {
        let work = FuturesUnordered::new();
        Self { inner, work }
    }
}


impl<T> Sink for PushSink<T> where T: RedisNonBlock {

    type SinkItem = (String,String);

    type SinkError = Error;

    fn start_send(&mut self, (key,val): Self::SinkItem) -> StartSend<Self::SinkItem,Self::SinkError> {
        let send_work = self.inner.lpush(key,Some(val));
        self.work.push(send_work);
        let _: Async<_> = self.poll_complete()?;
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(),Self::SinkError> {
        loop {
            match self.work.poll()? {
                Async::Ready(Some(_)) => {
                    // continue polling...
                },
                Async::Ready(None) => {
                    // all futures completed, sink is empty...
                    return Ok(Async::Ready(()));
                },
                Async::NotReady => {
                    // no further progress can be made...
                    return Ok(Async::NotReady);
                }
            }
        }
    }
}

