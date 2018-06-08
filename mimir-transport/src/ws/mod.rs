//! asynchronous websocket communications
//!
//! Simple websocket based communication facilities for remote Mimir Bridge
//! client connections.
//!
//! ## server
//!
//! ```
//! extern crate mimir_transport;
//! extern crate tokio;
//! 
//! use mimir_transport::ws;
//! use tokio::prelude::*;
//! 
//! # fn example() {
//!
//! let addr = "127.0.0.1:8888".parse().unwrap();
//!
//! let echo_server = ws::listener(&addr).map_err(|e| println!("listener error: {:?}",e))
//!     .for_each(|conn| {
//!         let (tx,rx) = conn.split();
//!         let echo_all = rx.inspect(|msg| println!("echoing {:?}",msg))
//!             .forward(tx).then(|_| Ok(()));
//!         tokio::spawn(echo_all);
//!         Ok(())
//!     });
//!
//! tokio::run(echo_server);
//! # }
//! # fn main() { }
//! ```
//!
//! ## client
//!
//! ```
//! extern crate mimir_transport;
//! extern crate tokio;
//! 
//! use mimir_transport::ws;
//! use tokio::prelude::*;
//! 
//! # fn example() {
//!
//! let url = "ws://127.0.0.1:8888".parse().unwrap();
//!
//! let say_hello = ws::connect(url)
//!     .and_then(|conn| conn.send("hello".into()))
//!     .then(|_|Ok(()));
//!
//! tokio::run(say_hello);
//! # }
//! # fn main() { }
//! ```

pub(crate) mod util;
mod types;


pub use self::types::Message;

use tokio_tungstenite::{
    connect_async,
    accept_async,
};
use tokio::net::TcpListener;
use tokio::prelude::*;
use url::Url;
use self::util::Connection;
use ::Error;
use std::time::Duration;
use std::net::SocketAddr;


/// a websocket connection
///
pub trait WebSocketConn: Stream<Item=Message,Error=Error> + Sink<SinkItem=Message,SinkError=Error> + Send { }

impl<T> WebSocketConn for T where T: Stream<Item=Message,Error=Error> + Sink<SinkItem=Message,SinkError=Error> + Send { }


/// asynchronously connect to server (`ws://*` or `wss://*`)
///
pub fn connect(url: Url) -> impl Future<Item=impl WebSocketConn, Error=Error> + Send {
    connect_async(url).from_err()
        .map(|(ws_conn,_)| {
            let max_idle = Duration::from_millis(4096);
            Connection::new(ws_conn,max_idle)
        })
}


/// listen for incoming websocket handshakes
///
pub fn listener(addr: &SocketAddr) -> impl Stream<Item=impl WebSocketConn, Error=Error> + Send {
    TcpListener::bind(addr).map(|listener| listener.incoming())
        .into_future().flatten_stream().from_err::<Error>()
        .map(|tcp_stream| {
            // TODO: add timeout (see `accept`)
            accept(tcp_stream).then(|rslt| {
                // log & then discard failed upgrade attempts...
                match rslt {
                    Ok(ws_conn) => Ok(Some(ws_conn)),
                    Err(error) => {
                        error!("ws upgrade failed with {:?}",error);
                        Ok(None)
                    },
                }
            })
        })
        // buffer pending upgrades (TODO: make configurable)
        .buffer_unordered(64)
        // filter out failed upgrades...
        .filter_map(|rslt: Option<_>| rslt)
}



/// attempt to upgrade tcp stream (or similar) to websocket
///
// TODO: add (optional?) timeout
fn accept<T>(tcp_stream: T) -> impl Future<Item=impl WebSocketConn, Error=Error> + Send where T: AsyncRead + AsyncWrite + Send {
    accept_async(tcp_stream).from_err()
        .map(|ws_conn| {
            let max_idle = Duration::from_millis(2048);
            Connection::new(ws_conn,max_idle)
        })
}

