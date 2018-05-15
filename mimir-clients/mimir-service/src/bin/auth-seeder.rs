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


use mimir_service::auth_seeder::{
    SeedLoader,
    apply_seeding,
    Options,
    Config,
};
use mimir_transport::redis;
use std::time::{Duration,Instant};
use tokio_core::reactor::Core;
use tokio_timer::Interval;
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

    let seeding_interval = Interval::new(Instant::now(),Duration::from_secs(2));

    let mut counter = 0;

    let work = seeding_interval.map_err(|e|error!("timer error {:?}",e))
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
