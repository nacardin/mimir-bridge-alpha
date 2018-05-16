use futures::future::{self,Executor};
use futures::{Future,Stream,Sink};
use redis::{self,NonBlockHandle,BlockingHandle};
use websocket::stream::async::Stream as WsStream;
use websocket::client::async::Client;
use websocket::OwnedMessage;
use common::{
    Operation,
    Identity,
    CMD,
};
use edge::{
    OperationFilter,
    AuthServer,
    Policy,
    Error,
};
use std::net::SocketAddr;


// TODO: convert the various trait objects below into concrete types.
// `Sender` and `Receiver` aliases are the top priority since they are
// polled at least once each per message.


//pub type Sender = Box<Sink<SinkItem=Operation,SinkError=Error>>;


pub trait Sender: Sink<SinkItem=Operation,SinkError=Error> { }

impl<T> Sender for T where T: Sink<SinkItem=Operation,SinkError=Error> { }

pub trait Receiver: Stream<Item=Operation,Error=Error> { }

impl<T> Receiver for T where T: Stream<Item=Operation,Error=Error> { }


//pub type Receiver = Box<Stream<Item=Operation,Error=Error>>;


/// attempt to serve a client connection
///
pub fn serve_connection<A,S,E>(auth_server: Policy<A>, client: Client<S>, redis_addr: SocketAddr, executor: E) -> impl Future<Item=(),Error=Error>
        where A: AuthServer<Error=Error> + 'static, 
              S: WsStream + 'static, 
              E: Executor<Box<Future<Item=(),Error=()> + Send>> + Clone + 'static {
    let work = init_server_side_client(client).and_then(move |(ident,client_tx,client_rx)| {
            let conn_work = init_redis(redis_addr,executor,ident)
                .and_then(move |(redis_tx,redis_rx)| {
                    build_connection((client_tx,client_rx),(redis_tx,redis_rx),ident)
                });
            auth_server.while_authorized(ident,conn_work)
        });
    work
}


/// configure websocket client handles & execute `IDENTIFY` handshake
///
pub fn init_server_side_client<S>(client: Client<S>) -> impl Future<Item=(Identity,impl Sender, impl Receiver),Error=Error>
        where S: WsStream + 'static {
    let (sender,receiver) = split_client(client);
    let work = server_side_handshake(receiver).map(move |(ident,receiver)| {
        (ident,sender,receiver)
    });
    work
}


/// initialize appropriate redis connection pair for specified identity
///
pub fn init_redis<E>(address: SocketAddr, executor: E, ident: Identity) -> impl Future<Item=(impl Sender, impl Receiver),Error=Error>
        where E: Executor<Box<Future<Item=(),Error=()> + Send>> + Clone + 'static {
    let work = future::lazy(move || {
        let spawn_nonblock = redis::spawn_nonblock(&address,executor.clone());
        let spawn_blocking = redis::spawn_blocking(&address,executor); 
        let init_work = spawn_nonblock.join(spawn_blocking).map_err(|e|Error::from(e))
            .map(move |(nonblock,blocking)| configure_redis(nonblock,blocking,ident));
        init_work
    });
    work
}


/// build top-level connection future
///
pub fn build_connection(client_handles: (impl Sender,impl Receiver), redis_handles: (impl Sender,impl Receiver), ident: Identity) -> impl Future<Item=(),Error=Error> {
    let filter_a = OperationFilter::new(ident);
    let filter_b = filter_a.clone();
    let (client_tx,client_rx) = client_handles;
    let (redis_tx,redis_rx) = redis_handles;
    let incoming = client_rx.and_then(move |op| {
        filter_a.filter_incoming(op)
    }).filter_map(|op|op).forward(redis_tx).map(|_|());
    let outgoing = redis_rx.and_then(move |op| {
        filter_b.filter_outgoing(op)
    }).filter_map(|op|op).forward(client_tx).map(|_|());
    let work = incoming.select(outgoing)
        .map(|_|()).map_err(|(err,_)|err);
    work
}


fn configure_redis(nonblock: NonBlockHandle, blocking: BlockingHandle, ident: Identity) -> (impl Sender,impl Receiver) {
    let sender = redis::PushSink::new(nonblock).with(|op: Operation| {
        let channel: String = op.dest_channel().to_string();
        let payload: String = op.into();
        Ok((channel,payload))
    });
    let channels: Vec<_> = [ident.direct_channel(),ident.shared_channel()]
        .iter().map(|chnl| chnl.to_string()).collect();
    let receiver = redis::PopStream::new(blocking,channels).map_err(|e|Error::from(e))
        .and_then(|(_,msg): (String,String)| {
            Operation::from_string(msg).map_err(|e|Error::from(e))
        });
    (Box::new(sender),Box::new(receiver))
}


// TODO: handle non-text msg variants & proper ping/pong behavior
pub fn split_client<S>(client: Client<S>) -> (impl Sender,impl Receiver)
        where S: WsStream + 'static {
    let (tx,rx) = client.split();
    let sender = tx.with(|op: Operation| {
        let text: String = op.into();
        Ok(OwnedMessage::Text(text))
    });
    let receiver = rx.map_err(|e|Error::from(e))
        .and_then(|msg: OwnedMessage| {
            match msg {
                OwnedMessage::Text(text) => {
                    Operation::from_string(text)
                        .map_err(|e|Error::from(e))
                },
                _ => Err(Error::Other("handler for non-text variants not yet implemented"))
            }
        });
    (Box::new(sender),Box::new(receiver))
}


// TODO: add handshake timeout...
fn server_side_handshake<S>(client_stream: S) -> impl Future<Item=(Identity,S),Error=Error>
        where S: Stream<Item=Operation,Error=Error> + 'static {
    let work = client_stream.into_future()
        .map_err(|(error,_)|error)
        .and_then(|(item,stream)| {
            if let Some(op) = item {
                process_handshake(op)
                    .map(move |ident|(ident,stream))
            } else {
                Err("stream terminated prior to handshake".into())
            }
        });
    work
}


fn process_handshake(op: Operation) -> Result<Identity,Error> {
    let cmd = op.expect_command(CMD::IDENTIFY)
        .map_err(|_|"handshake variant must be `CMD::IDENTIFY`")?;
    if let Some(address) = cmd.recover().map_err(|_|"unrecoverable signature")? {
        if cmd.dest.address == address {
            Ok(cmd.dest)
        } else {
            Err("signer address does not match claim".into())
        }
    } else {
        Err("missing signature in handshake".into())
    }
}

