//! service which monitors blockchain state & seeds permission
//! information to `edge-server` instances.
//!
extern crate mimir_service;
extern crate mimir_transport;
extern crate mimir_node;
extern crate tokio_timer;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;


use tokio_timer::Deadline;
use std::time::Duration;
use mimir_service::auth_seeder::{
    SeedLoader,
    apply_seeding,
    Options,
    Config,
};
use mimir_transport::edge::LeaseConfig;
use mimir_transport::redis;
use tokio_core::reactor::Core;
use futures::{Future,Stream};
use log::LevelFilter;


fn main() {

    let opt = Options::from_args();

    let config = Config::init(&opt.config).unwrap();

    init_logger(opt.log_level);

    info!("running with {:?}",config);

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let node = mimir_node::node::ws(config.websocket_rpc.as_ref(),&handle)
        .expect("unable to connect to websocket rpc"); 

    let redis = core.run(redis::spawn_nonblock(&config.redis_address,handle.clone()))
        .expect("unable to connect to redis instance");

    let seed_loader = SeedLoader::new(node.transport().to_owned(),config.seeding.clone());

    let lease_config: LeaseConfig = Default::default();

    let mut counter = 0;

    let seed_timeout = Duration::from_millis(2048);

    let work = lease_config.seeding_interval().map_err(|e|error!("timer error {:?}",e))
        .for_each(move |instant| {
            let redis_handle = redis.clone();
            counter += 1;
            let show_seeding = if counter > 10 { counter = 0; true } else { false };
            let seed_work = seed_loader.get_seed_states().map_err(|e|error!("in seed loader {:?}",e))
                .for_each(move |seed_state| {
                    if show_seeding {
                        info!("{}-conn {:?}",seed_state.role,seed_state.conn);
                        info!("{}-auth {:?}",seed_state.role,seed_state.auth);
                    }
                    apply_seeding(redis_handle.clone(),seed_state)
                        .map_err(|e|error!("in `apply_seeding` {:?}",e))
                });
            Deadline::new(
                seed_work,
                instant + seed_timeout
                )
                .map_err(|e|error!("during seeding {:?}",e))
        });

/*
    let work = lease_config.seeding_interval().map_err(|e|error!("timer error {:?}",e))
        .map(move |_| {
            seed_loader.get_seed_states()
                .map_err(|e|error!("in seed loader {:?}",e))
        })
        .flatten()
        .for_each(move |seed_state| {
            counter += 1;
            if counter > 10 {
                counter = 0;
                info!("{}-conn {:?}",seed_state.role,seed_state.conn);
                info!("{}-auth {:?}",seed_state.role,seed_state.auth);
            }
            let seed_work = apply_seeding(redis.clone(),seed_state)
                .map_err(|e|error!("in `apply_seeding` {:?}",e));
            handle.spawn(seed_work);
            Ok(())
        });
*/
    core.run(work).unwrap()
}


fn init_logger(loglevel: LevelFilter) {
    if let LevelFilter::Off = loglevel {
        env_logger::init();
    } else {
        env_logger::Builder::from_default_env()
                .filter(Some("auth_seeder"),loglevel)
                .filter(Some("mimir_service"),loglevel)
                .filter(Some("mimir_node"),loglevel)
                .filter(Some("mimir_transport"),loglevel)
                .init();
    }
}
