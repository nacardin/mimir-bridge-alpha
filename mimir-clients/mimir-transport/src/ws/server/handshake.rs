use websocket::server::upgrade::async::Upgrade;
use websocket::client::async::{Client,ClientNew};
use websocket::result::WebSocketError;
use websocket::stream::async::Stream as WsStream;
use futures::future::Future;
use futures::{Async,Poll};
use std::net::SocketAddr;
use ws::PROTOCOL;


use websocket::async::futures::sink::Send as SinkSend;
use websocket::codec::http::HttpServerCodec;
use websocket::client::async::Framed;


type ClientReject<S> = SinkSend<Framed<S,HttpServerCodec>>;


/// future which manages the basic websocket protocol handshake.
/// 
/// resolves to client handle & address if incoming connection implements
/// the expected protocol, and otherwise resolves to `None`.
///
pub struct Handshake<S> where S: WsStream + 'static {
    address: Option<SocketAddr>,
    inner: Inner<S>,
}


impl<S> Handshake<S> where S: WsStream + 'static {


    /// instantiate new handshake future.
    ///
    pub fn new(upgrade: Upgrade<S>, address: SocketAddr) -> Self {
        Self::with_proto(upgrade,address,PROTOCOL)
    }


    /// instantiate new handshake with custom protocol.
    ///
    pub fn with_proto(upgrade: Upgrade<S>, address: SocketAddr, proto: &str) -> Self {
        if upgrade.protocols().iter().any(|p| p == proto) {
            let work = upgrade.use_protocol(PROTOCOL).accept();
            Self::accept(work,address)
        } else {
            Self::reject(upgrade.reject())
        }
    }

    pub(crate) fn accept(work: ClientNew<S>, address: SocketAddr) -> Self { 
        let inner = Inner::Accept { work };
        let address = Some(address);
        Handshake { address, inner }
    }

    pub(crate) fn reject(work: ClientReject<S>) -> Self {
        let address = Default::default();
        let inner = Inner::Reject { work };
        Handshake { address, inner }
    }
}


impl<S> Future for Handshake<S> where S: WsStream + 'static {

    type Item = Option<(Client<S>,SocketAddr)>;

    type Error = WebSocketError;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        match self.inner {
            Inner::Accept { ref mut work } => {
                let (client,_) = try_ready!(work.poll());
                let address = self.address.take()
                    .expect("no polling past completion");
                Ok(Async::Ready(Some((client,address))))
            },
            Inner::Reject { ref mut work } => {
                let _ = try_ready!(work.poll());
                Ok(Async::Ready(None))
            },
        }
    }
}


enum Inner<S> where S: WsStream + 'static {
    Accept {
        work: ClientNew<S>,
    },
    Reject {
        work: ClientReject<S>,
    }
}

