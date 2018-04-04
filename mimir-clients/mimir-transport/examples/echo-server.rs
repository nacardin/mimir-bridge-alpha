extern crate mimir_transport;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;

use tokio_core::reactor::Core;
use futures::{future,Stream};
use mimir_transport::ws;
use log::LevelFilter;


fn main() {
    init_logger(false);

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let addr = "127.0.0.1:8888".parse().unwrap();

    let server = ws::server::bind(&handle,addr).unwrap();

    let work = server.for_each(|(client,address)| {
        info!("connection established with {}",address);
        ws::client::spawn(&handle,client,move |msg| {
            info!("from: {} message: {}",address,msg);
            future::ok(Some(msg))
        }).forget();
        Ok(())
    });

    core.run(work).unwrap();
}

fn init_logger(debug: bool) {
    let loglevel = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    env_logger::Builder::new()
            .filter(Some("echo_server"),loglevel)
            .filter(Some("mimir_transport"),loglevel)
            .init();
}
