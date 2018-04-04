use websocket::stream::async::Stream as WsStream;
use websocket::server::upgrade::async::Upgrade;
use websocket::server::upgrade::HyperIntoWsError;
use websocket::server::InvalidConnection;
use websocket::result::WebSocketError;
use futures::{task,Stream,Async,Poll};
use std::net::SocketAddr;



/// incoming connection stream combinator.
/// 
/// logs incoming connections errors, suppressing errors
/// which pertain to the individual connection (as opposed
/// to the underlying tcp stream).  Typically intended to be
/// used with `websocket::server::async::Incoming`.
///
pub struct Filter<T> {
    inner: T,
}


impl<T> Filter<T> {

    /// instantiate new filter instance.
    ///
    pub fn new(inner: T) -> Self { Filter { inner } }

    /// consume filer, yielding inner stream.
    ///
    pub fn into_inner(self) -> T { let Filter { inner } = self; inner }
}


impl<T> From<T> for Filter<T> {

    fn from(inner: T) -> Self { Self::new(inner) }
}


impl<T,S,B> Stream for Filter<T> where T: Stream<Item=(Upgrade<S>,SocketAddr),Error=InvalidConnection<S,B>>, S: WsStream + 'static {

    type Item = (Upgrade<S>,SocketAddr);

    type Error = WebSocketError;


    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        match self.inner.poll() {
            Ok(incoming) => Ok(incoming),
            Err(invalid) => {
                if let HyperIntoWsError::Io(error) = invalid.error {
                    // io errors indicate problem with the server's underlying tcp listener
                    error!("problem in tcp listener {:?}",error);
                    Err(From::from(error))
                } else {
                    // other errors relate to the specific incoming connection, so drop connection
                    // & yield back to the event loop.
                    warn!("bad incoming connection {:?}",invalid.error);
                    task::current().notify();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

