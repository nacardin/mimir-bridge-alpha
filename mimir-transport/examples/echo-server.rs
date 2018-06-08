extern crate mimir_transport;
extern crate tokio;

use mimir_transport::ws;
use tokio::prelude::*;
use std::env;

fn main() {

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8888".to_string())
        .parse().unwrap();

    println!("binding to {}...",addr);

    let server = ws::listener(&addr).map_err(|err| println!("server error: {:?}",err))
        .for_each(|conn| {
            let (tx,rx) = conn.split();
            let work = rx.inspect(|msg| println!("echoing msg: {:?}",msg))
                .forward(tx).map(|(_tx,_rx)| ())
                .map_err(|err| println!("conn error: {:?}",err));
            tokio::spawn(work);
            Ok(())
        });

    tokio::run(server);
}
