extern crate mimir_transport;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;

use tokio_core::reactor::Core;
use futures::{Future,Stream};
use mimir_transport::edge;
use mimir_transport::ws;
use log::LevelFilter;


fn main() {
    init_logger(false);

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let server_addr = "127.0.0.1:8888".parse().unwrap();

    let redis_addr = "127.0.0.1:6379".parse().unwrap();

    let ws_server = ws::server::bind(&handle,server_addr).unwrap();

    let auth_server = edge::Policy::new(edge::DebugAuthServer,Default::default());

    let work = ws_server.for_each(|(client,address)| {
        info!("incoming connection from {}",address);
        let conn_handle = handle.clone();
        let conn_work = edge::serve_connection(
            auth_server.clone(),
            client,
            redis_addr,
            conn_handle
            ).map_err(|e|error!("in spawn work: {:?}",e));
        handle.spawn(conn_work);
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
            .filter(Some("edge_server"),loglevel)
            .filter(Some("mimir_transport"),loglevel)
            .init();
}
