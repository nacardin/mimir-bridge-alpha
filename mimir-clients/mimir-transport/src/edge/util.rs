use futures::{Stream,Async,Poll};
use tokio_timer::{
    Interval,
    Error,
};
use std::time::Instant;


/// simple rate-limiting stream combinator.
/// 
/// limits an arbitrary stream to produce at most one item
/// per interval.
///
pub struct Limit<S> {
    interval: Interval,
    instant: Option<Instant>,
    stream: S,
    state: LimitState,
}


impl<S> Limit<S> {

    /// limit `stream` to produce at most one item per
    /// tick of `interval`.
    pub fn new(interval: Interval, stream: S) -> Self {
        let instant = None;
        let state = LimitState::AwaitNextInterval;
        Self { interval, instant, stream, state }
    }

    /// get the latest `Instant` yielded by the inner
    /// `Interval`.
    ///
    /// returns `None` until the first `Instant` is yielded.
    ///
    pub fn last_instant(&self) -> Option<Instant> { self.instant.clone() }
}


#[derive(Debug,Copy,Clone)]
enum LimitState {
    AwaitNextInterval,
    PollInnerStream
}


impl<S> Stream for Limit<S> where S: Stream, S::Error: From<Error> {

    type Item = S::Item;

    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        loop {
            match self.state {
                LimitState::AwaitNextInterval => {
                    let instant = try_ready_stream!(self.interval.poll());
                    self.instant = Some(instant);
                    self.state = LimitState::PollInnerStream;
                },
                LimitState::PollInnerStream => {
                    let item = try_ready_stream!(self.stream.poll());
                    self.state = LimitState::AwaitNextInterval;
                    return Ok(Async::Ready(Some(item)));
                },
            }
        }
    }
}
