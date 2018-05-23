use futures::{Stream,Async,Poll};
use std::collections::VecDeque;


pub struct Lag<S,I> {
    stream: S,
    items: VecDeque<I>,
    count: usize,
}


impl<S,I> Lag<S,I> {

    pub fn new(stream: S, count: usize) -> Self {
        let items = VecDeque::with_capacity(count + 1);
        Self { stream, items, count }
    }
}


impl<S: Stream> Stream for Lag<S,S::Item> {

    type Item = S::Item;

    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        loop {
            // attempt to acquire next item from inner stream
            let next_item = try_ready_stream!(self.stream.poll());
            // push item into ring buffer
            self.items.push_back(next_item);
            // we should never have more than `count + 1` items.
            debug_assert!(self.items.len() <= self.count + 1);
            // if we've passed our lag threshold, yield oldest item
            if self.items.len() > self.count {
                let item = self.items.pop_front().expect("buffer is never empty");
                return Ok(Async::Ready(Some(item)));
            }
        }
    }
}
