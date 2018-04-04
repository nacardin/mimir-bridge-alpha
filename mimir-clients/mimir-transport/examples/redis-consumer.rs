extern crate mimir_transport;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;

use mimir_transport::redis::spawn;
use tokio_core::reactor::Core;
use futures::{Future,Stream};
use log::LevelFilter;


fn main() {
    init_logger(false);

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let address = "127.0.0.1:6379".parse().unwrap();

    let targets = ["some-list-0","some-list-1"];
    
    let work = spawn::watcher(&address,handle,targets)
        .map_err(|e| error!("unable to spawn watcher {:?}",e))
        .and_then(|watcher| {
            watcher.for_each(|item| {
                    info!("got item {:?}",item);
                    Ok(())
                })
                .map_err(|e| error!("stream error {:?}",e))        
        });
 
    core.run(work).unwrap();

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
