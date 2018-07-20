extern crate mimir_transport;
extern crate futures;
extern crate tokio;
extern crate env_logger;
#[macro_use]
extern crate log;

use mimir_transport::rds as redis;
use futures::Stream;
use futures::Future;
use log::LevelFilter;


fn main() {
    init_logger(false);

    let address = "127.0.0.1:6379".parse().unwrap();

    let targets = vec!["some-list-0".to_string(),"some-list-1".to_string()];

    println!("targets: {:?}",targets);

    let redis_handle = redis::spawn_blocking(&address).and_then(|redis_handle| {

        let work = redis::pop_stream(redis_handle,targets)
            .for_each(|(list,value)| {
                info!("got value `{}` from list `{}`",value,list);
                Ok(())
            });

        work.map_err(|x| {
            error!("stream terminated with error: {}", x);
            x
        })
    })
    .map_err(|x| {
        error!("error: {}", x);
        ()
    });

    tokio::run(redis_handle);

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
