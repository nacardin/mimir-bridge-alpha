//! `try_ready_stream` macro submodule.
//!


/// stream variant of the standard `try_ready` macro.
/// 
/// this macro makes it easy to write combinators for streams by short-circuiting
/// the return type of the
/// [`Stream::poll`](https://docs.rs/futures/0.1.18/futures/stream/trait.Stream.html#required-methods) 
/// method in a similar manner to the `?` operator or `try!` macro:
/// 
/// ```
/// # #[macro_use] extern crate mimir_common;
/// # fn main() { }
/// extern crate futures;
///
/// use futures::{Stream,Poll,Async};
/// use std::fmt::Debug;
///
/// struct StreamWrapper<S> {
///     inner: S
/// }
///
/// impl<S> Stream for StreamWrapper<S> where S: Stream {
/// 
///     type Item = S::Item;
///
///     type Error = S::Error;
///
///     fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
///         let item = try_ready_stream!(self.inner.poll());
///         // operate on item as you would inside synchronous code...
///         Ok(Async::Ready(Some(item)))
///     }
/// }
/// ```
/// 
/// *note*: macro expects the `futures` crate to be available in the crate root.
///
#[macro_export]
macro_rules! try_ready_stream {
    ($e:expr) => (match $e {
        Ok(::futures::Async::Ready(Some(item))) => item,
        Ok(::futures::Async::Ready(None)) => return Ok(::futures::Async::Ready(None)),
        Ok(::futures::Async::NotReady) => return Ok(::futures::Async::NotReady),
        Err(e) => return Err(From::from(e)),
    })
}


