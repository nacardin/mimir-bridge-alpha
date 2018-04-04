use tokio_core::reactor::Handle;
use websocket::client::builder::Url;
use websocket::client::async::Client;
use websocket::stream::async::Stream as WsStream;
use websocket::{async,ClientBuilder,OwnedMessage,WebSocketError};
use futures::future::{self,Either};
use futures::{Stream,Sink,Future};
use futures::sync::oneshot::{self,SpawnHandle};
use ws::PROTOCOL;


/// type alias for a websocket client.
///
pub type SimpleClient = Client<Box<async::Stream + Send>>;


/// helper function which builds a client and executes the initial login operation.
///
pub fn connect(handle: &Handle, address: &Url, login: String) -> Box<Future<Item=SimpleClient,Error=WebSocketError>> {
    debug!("configuring connector for {:?}",address);
    // build connection future w/ appropriate headers & login msg.
    // TODO: implement a ping/pong post login to confirm success.
    let connect = ClientBuilder::from_url(address)
        .add_protocol(PROTOCOL)
        .async_connect(None,handle)
        .and_then(move |(client,_)| {
            debug!("sending login message {}",login);
            client.send(OwnedMessage::Text(login))
        });
    // this is a one-off operation, so get that sweet sweet type erasure.
    Box::new(connect)
}


/// helper function for spawning clients.
///
/// This function spawns a task which manages proper ping/pong behavior, logs basic errors,
/// and forwards handling of `text` variant websocket messages to a provided closure.  The
/// closure should return a future which yields an `Option<String>` on success.  The `Some`
/// variant will broadcast the supplied string as a `text` variant message, and the `None`
/// variant will send nothing.  Errors from the generated future are logged but not suppressed,
/// so non-fatal errors should be captured and dealt with internally.
///
/// WARNING: this function is a work-in-progress helper.  the produced task currently hangs on
/// close.  also, it does no pings of its own.  also, it accepts a trait-object based client
/// instead of a concrete client (e.g. either `TcpStream` based or `TlsStream` based).
///
pub fn spawn<W,F,S>(handle: &Handle, client: Client<S>, mut work: W) -> SpawnHandle<(),()> where W: FnMut(String) -> F + 'static, F: Future<Item=Option<String>,Error=()> + 'static, S: WsStream + 'static {
    let (sink,stream) = client.split();
    let (tx,rx) = oneshot::channel::<()>();
    let mut killswitch = Some(tx);
    let kill_rx = rx.map_err(|_| warn!("killswitch tx handle deallocated"));
    let msg_stream = stream.map_err(|e| error!("client stream error {:?}",e))
        .and_then(move |message: OwnedMessage| {
            match message {
                OwnedMessage::Text(text) => {
                    let msg_work = (work)(text)
                        .map(|out| out.map(|txt| OwnedMessage::Text(txt)));
                    Either::A(msg_work)
                },
                OwnedMessage::Binary(bytes) => {
                    warn!("ignoring binary message {:?}",bytes);
                    Either::B(future::ok(None))
                },
                OwnedMessage::Close(data) => {
                    info!("got close message {:?}",data);
                    if let Some(tx) = killswitch.take() {
                        info!("triggering killswitch...");
                        let _ = tx.send(()).map_err(|_| {
                            warn!("killswitch rx handle deallocated");
                        });
                        let msg = OwnedMessage::Close(data);
                        Either::B(future::ok(Some(msg)))
                    } else {
                    Either::B(future::ok(None))
                    }
                },
                OwnedMessage::Ping(bytes) => {
                    trace!("handling ping {:?}",bytes);
                    let msg = OwnedMessage::Pong(bytes);
                    Either::B(future::ok(Some(msg)))
                },
                OwnedMessage::Pong(bytes) => {
                    warn!("ignoring pong {:?}",bytes);
                    Either::B(future::ok(None))
                },
            }
        })
        .filter_map(|rslt: Option<OwnedMessage>| rslt);
    let logged_sink = sink.sink_map_err(|e| error!("client sink error {:?}",e));
    let forward = msg_stream.forward(logged_sink)
        .map(|_|info!("shutting down (ok)"))
        .map_err(|_|warn!("shutting down (err)"));
    let task = forward.select(kill_rx)
        .map(|_|()).map_err(|_|());
    info!("spawning websocket client task...");
    oneshot::spawn(task,handle)
}

