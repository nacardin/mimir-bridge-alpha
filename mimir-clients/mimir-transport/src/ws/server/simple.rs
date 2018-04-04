use websocket::stream::async::Stream as WsStream;
use websocket::server::async::{Server,Incoming};
use websocket::client::async::Client;
use websocket::result::WebSocketError;
use websocket::async::TcpStream;
use futures::stream::BufferUnordered;
use futures::{Stream,Async,Poll};
use tokio_core::reactor::Handle;
use std::net::SocketAddr;
use ws::server::{Filter,Protocol};
use ws::PROTOCOL;
use std::io;


pub fn bind(handle: &Handle, address: SocketAddr) -> io::Result<SimpleServer<TcpStream>> {
    SimpleBuilder::default().address(address).finish(handle)
}



#[derive(Default)]
pub struct SimpleBuilder {
    protocol: Option<String>,
    pending: Option<usize>,
    address: Option<SocketAddr>,

}


impl SimpleBuilder {

    pub fn protocol(mut self, proto: String) -> Self { self.protocol = Some(proto); self }

    pub fn pending(mut self, max: usize) -> Self { self.pending = Some(max); self }

    pub fn address(mut self, addr: SocketAddr) -> Self { self.address = Some(addr); self }

    pub fn finish(mut self, handle: &Handle) -> io::Result<SimpleServer<TcpStream>> {
        let address = self.address.take().unwrap_or_else(||{
            SocketAddr::from(([127,0,0,1],8888))
        });
        let base_server = Server::bind(address,handle)?;
        let simple = self.finish_with(base_server.incoming());
        Ok(simple)
    }

    pub fn finish_with<S>(self, incoming: Incoming<S>) -> SimpleServer<S> where S: WsStream + 'static {
        let protocol = self.protocol.unwrap_or_else(||PROTOCOL.into());
        let pending = self.pending.unwrap_or(32);
        let filter = Filter::from(incoming);
        let proto = Protocol::with_proto(filter,protocol);
        let inner = proto.buffer_unordered(pending);
        SimpleServer { inner }
    }
}



/// simple server implementation.
///
pub struct SimpleServer<S> where S: WsStream + 'static {
    inner: BufferUnordered<Protocol<Filter<Incoming<S>>>>,
}


impl<S> SimpleServer<S> where S: WsStream + 'static {

    pub fn new(incoming: Incoming<S>) -> Self {
        let filter: Filter<_> = incoming.into();
        let proto: Protocol<_> = filter.into();
        let inner = proto.buffer_unordered(32);
        SimpleServer { inner }
    }
}


impl<S> From<Incoming<S>> for SimpleServer<S> where S: WsStream + 'static {

    fn from(incoming: Incoming<S>) -> Self { SimpleServer::new(incoming) }
}


impl<S> Stream for SimpleServer<S> where S:  WsStream + 'static {

    type Item = (Client<S>,SocketAddr);

    type Error = WebSocketError;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        loop {
            if let Some((client,addr)) = try_ready_stream!(self.inner.poll()) {
                return Ok(Async::Ready(Some((client,addr))));
            } else {
                // got output of a rejection handshake,
                // poll inner stream again...
            }
        }
    }
}

