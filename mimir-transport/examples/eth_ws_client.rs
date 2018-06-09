extern crate mimir_transport;
extern crate tokio;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;

use mimir_transport::ws;
use tokio::prelude::*;
use std::env;
use log::LevelFilter;

fn main() {
    init_logger(false);

    let url = env::args().nth(1).unwrap_or_else(|| "ws://127.0.0.1:8546".to_string())
        .parse().unwrap();

    println!("connecting to {}...",url);

    let task = ws::eth_rpc_client::connect(url).and_then(|mut client| {

        println!("calling eth_blockNumber");

        let params: Option<&String> = None;

        let a = client.call("eth_blockNumber", params).and_then(|res| {
            println!("response: {:?}",res);
            futures::future::ok(())
        });

        println!("called eth_blockNumber");

        println!("calling eth_blockNumber");

        let b = client.call("eth_blockNumber", params).and_then(|res| {
            println!("response: {:?}",res);
            futures::future::ok(())
        });

        println!("called eth_blockNumber");

        println!("calling eth_blockNumber");

        let c = client.call("eth_blockNumber", params).and_then(|res| {
            println!("response: {:?}",res);
            futures::future::ok(())
        });

        println!("called eth_blockNumber");
        
        a.join(b).join(c)
        .map(|_| println!("jobs finished"))
        .map_err(|err| {
            println!("jobs finished with error: {:?}", err);
            err
        })
    })
    .map(|_| println!("task finished"))
    .map_err(|err| println!("task finished with error: {:?}", err));

    tokio::run(task);
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