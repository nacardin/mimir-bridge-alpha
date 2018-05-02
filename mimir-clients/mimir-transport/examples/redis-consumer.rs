extern crate mimir_transport;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;

use mimir_transport::redis;
use tokio_core::reactor::Core;
use futures::Stream;
use log::LevelFilter;


fn main() {
    init_logger(false);

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let address = "127.0.0.1:6379".parse().unwrap();

    let targets = vec!["some-list-0".to_string(),"some-list-1".to_string()];

    println!("targets: {:?}",targets);

    let redis_handle = core.run(redis::spawn_blocking(&address,handle))
        .expect("unable to initialize redis connection");

    let work = redis::PopStream::new(redis_handle,targets)
        .for_each(|(list,value)| {
            info!("got value `{}` from list `{}`",value,list);
            Ok(())
        });
 
    core.run(work).expect("stream terminated with error");

    info!("work complete, shutting down...");
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
