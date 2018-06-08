extern crate mimir_transport;
extern crate tokio;

use mimir_transport::ws;
use tokio::prelude::*;
use std::env;

fn main() {

    let addr = env::args().nth(1).unwrap_or_else(|| "ws://127.0.0.1:8888".to_string())
        .parse().unwrap();

    println!("connecting to {}...",addr);

    let client = ws::connect(addr).and_then(|conn| {
        let message = "hello".into();
        println!("sending message: {:?}",message);
        conn.send(message).and_then(|conn| {
            conn.into_future().map(|(msg,_)| {
                let message = msg.expect("server must reply");
                println!("got response: {:?}",message);
            }).map_err(|(err,_)| err)
        })
    }).map_err(|err| println!("client error: {:?}",err));

    tokio::run(client);
}

