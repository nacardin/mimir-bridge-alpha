use websocket::async::stream::Stream as WsStream;
use websocket::server::upgrade::async::Upgrade;
use websocket::result::WebSocketError;
use futures::{Stream,Async,Poll};
use std::net::SocketAddr;
use ws::server::Handshake;
use ws::PROTOCOL;


/// stream combinator for protocol handshakes.
///
pub struct Protocol<T> {
    inner: T,
    proto: String,
}


impl<T> Protocol<T> {

    pub fn new(inner: T) -> Self {
        Self::with_proto(inner,PROTOCOL.into())
    }

    pub fn with_proto(inner: T, proto: String) -> Self {
        Protocol { inner, proto }
    }
}


impl<T> From<T> for Protocol<T> {

    fn from(inner: T) -> Self { Self::new(inner) }
}


impl<T,S> Stream for Protocol<T> 
    where T: Stream<Item=(Upgrade<S>,SocketAddr)>, 
          WebSocketError: From<T::Error>, 
          S: WsStream + 'static 
{
    type Item = Handshake<S>;

    type Error = WebSocketError;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        let (upgrade,addr) = try_ready_stream!(self.inner.poll());
        let work = Handshake::with_proto(upgrade,addr,&self.proto);
        Ok(Async::Ready(Some(work)))
    }
}

