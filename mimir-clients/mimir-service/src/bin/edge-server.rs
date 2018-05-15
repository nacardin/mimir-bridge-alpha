extern crate mimir_transport;
extern crate mimir_service;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;

use tokio_core::reactor::Core;
use futures::{Future,Stream};
use mimir_service::edge_server::{
    Options,
    Config,
};
use mimir_transport::{
    redis,
    edge,
    ws
};
use log::LevelFilter;
use std::time;

fn main() {
    let opt = Options::from_args();

    let config = Config::init(&opt.config).unwrap();

    init_logger(opt.log_level);

    info!("running with {:?}",config);    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let ws_server = ws::server::bind(&handle,config.serve_address).unwrap();

    let redis_handle = core.run(redis::spawn_nonblock(&config.redis_address,handle.clone()))
        .expect("unable to spawn redis handle");

    let base_interval = time::Duration::from_secs(1);

    let lease_server = edge::LeaseServer::new(redis_handle,base_interval);

    let auth_server = edge::Policy::new(lease_server,config.policies.clone());

    let work = ws_server.for_each(|(client,address)| {
        info!("incoming connection from {}",address);
        let conn_handle = handle.clone();
        let conn_work = edge::serve_connection(
            auth_server.clone(),
            client,
            config.redis_address,
            conn_handle
            ).map_err(|e|error!("in `serve_connection` {:?}",e));
        handle.spawn(conn_work);
        Ok(())
    });

    core.run(work).unwrap();
}


fn init_logger(loglevel: LevelFilter) {
    if let LevelFilter::Off = loglevel {
        env_logger::init();
    } else {
        env_logger::Builder::from_default_env()
                .filter(Some("edge_server"),loglevel)
                .filter(Some("mimir_service"),loglevel)
                .filter(Some("mimir_transport"),loglevel)
                .init();
    }
}
