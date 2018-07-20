use std::marker::PhantomData;
use tokio::prelude::*;


/// a sink which drops all items it is given
///
pub struct DropSink<I,E> {
    item: PhantomData<I>,
    error: PhantomData<E>,
}


impl<I,E> Default for DropSink<I,E> {

    fn default() -> Self {
        Self {
            item: PhantomData,
            error: PhantomData,
        }
    }
}


impl<I,E> Sink for DropSink<I,E> {

    type SinkItem = I;

    type SinkError = E;

    fn start_send(&mut self, _item: Self::SinkItem) -> Result<AsyncSink<Self::SinkItem>,Self::SinkError> {
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(),Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

