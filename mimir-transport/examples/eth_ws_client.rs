extern crate mimir_transport;
extern crate tokio;
extern crate env_logger;
#[macro_use]
extern crate log;

use mimir_transport::ws;
use tokio::prelude::*;
use std::env;
use log::LevelFilter;

fn main() {
    init_logger(false);

    let addr = env::args().nth(1).unwrap_or_else(|| "ws://127.0.0.1:8546".to_string())
        .parse().unwrap();

    println!("connecting to {}...",addr);

    let client = ws::connect(addr).and_then(|conn| {
        let message = r#"{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":83}"#.into();
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


fn init_logger(debug: bool) {
    let loglevel = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    env_logger::Builder::new()
            .filter(Some("redis_consumer"),loglevel)
            .filter(Some("mimir_transport"),loglevel)
            .init();
}