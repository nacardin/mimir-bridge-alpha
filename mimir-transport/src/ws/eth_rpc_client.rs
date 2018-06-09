use ws;
use tokio::prelude::*;
use url::Url;
use error::Error;
use ws::types::Message;
use futures::{self, Stream, Sink};
use futures::sync::oneshot;
use std::collections::HashMap;
use serde_json::Value;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::executor;
use tokio::executor::Spawn;
use serde::Serialize;

static JSON_RPC_VERSION: &'static str = "2.0";

type WebSocketConn = ws::WebSocketConn<Item=Message, Error=Error, SinkItem=Message, SinkError=Error>;

type RpcResult = Result<Value,Value>;

pub struct Client {
    sink: Box<Sink<SinkItem=Message, SinkError=Error>>,
    spawn: Box<Spawn>,
    pending_requests: Arc<Mutex<HashMap<i64, oneshot::Sender<RpcResult>>>>,
    counter: i64
}

impl Client {
    fn new(connection: Box<WebSocketConn>) -> Self {

        let (sink, stream) = connection.split();

        let pending_requests = Arc::new(Mutex::new(HashMap::new()));
        let pending_requests2 = pending_requests.clone();

        let mut receive_task = stream.for_each(move |msg| {
            println!("receive_task {:?}", msg);
            let mut pending_requests = pending_requests2.lock().unwrap();
            resolve(&mut pending_requests, msg)
        })
        .map(|_| { () })
        .map_err(|_| { () });

        receive_task.poll().unwrap();

        let spawn = executor::spawn(receive_task);

        Self {
            sink: Box::new(sink),
            spawn: Box::new(spawn),
            pending_requests: pending_requests,
            counter: 0
        }
    }
    pub fn call<'a, M: AsRef<str>, P: Serialize>(&mut self, method_name: M, params: Option<&'a P>) -> impl Future<Item=RpcResult,Error=Error> {
        self.counter = self.counter + 1;

        let rpc_request = JsonRpcRequest {
            jsonrpc: JSON_RPC_VERSION,
            method: method_name.as_ref(),
            params: params,
            id: self.counter
        };

        let (sender, receiver) = oneshot::channel::<RpcResult>();
        
        let message = Message::encode_json(&rpc_request).unwrap();

        let mut pending_requests = self.pending_requests.lock().unwrap();
        pending_requests.insert(self.counter, sender);

        println!("sending message {:?}", message);
        self.sink.start_send(message).unwrap();
        self.sink.poll_complete().unwrap();        

        receiver.map_err(|_error| Error::message("err"))

    }
}

fn resolve(pending_requests: &mut HashMap<i64, oneshot::Sender<RpcResult>>, msg: Message) -> impl Future<Item=(),Error=Error> {

    let rpc_response: JsonRpcResponse = msg.parse_json().unwrap();

    println!("resolving rpc_response {:?}", rpc_response);

    if rpc_response.jsonrpc == JSON_RPC_VERSION {
        let id = rpc_response.id;
        let sender = pending_requests.remove(&id).unwrap();
        let result = {
            if let Some(error) = rpc_response.error {
                sender.send(Err(error))
            } else if let Some(result) = rpc_response.result {
                sender.send(Ok(result))
            } else {
                panic!()
            }
        };
        match result {
            Ok(_) => futures::future::ok(()),
            Err(e) => {
                println!("rcp error {:?}", e);
                panic!(e)
            }
        }
    } else {
        println!("asd {:?}", rpc_response.jsonrpc);
        panic!()
    }

}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<Value>,
    id: i64
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<'a, T: Serialize + 'a> {
    jsonrpc: &'a str,
    method: &'a str,
    params: Option<&'a T>,
    id: i64
}

pub fn connect(url: Url) -> impl Future<Item=Client, Error=Error> {
    ws::connect(url).map(|conn| {
        Client::new(Box::new(conn))
    })
}