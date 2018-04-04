extern crate mimir_transport;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;

use tokio_core::reactor::Core;
use futures::{future,Future};
use futures::sync::oneshot;
use mimir_transport::ws;
use log::LevelFilter;



fn main() {
    init_logger(false);

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let address = "ws://127.0.0.1:8888".parse().unwrap();

    let mut conn_work = Vec::new();

    for i in 0..512 {
        let message = format!("hello from {}",i);
        let connect = ws::client::connect(&handle,&address,message.clone());
        let capture_handle = handle.clone();
        let (tx,rx) = oneshot::channel();
        let mut killswitch = Some(tx);
        let msg_handler = move |msg: String| {
            if msg == message {
                killswitch.take().map(|tx|{
                    tx.send(()).expect("receiver exists")
                });
            }
            future::ok(None)
        };
        let keepalive = rx.map_err(move |_| error!("sender {} unexpectedly canceled",i));
        let work = connect.map_err(move |e| error!("connection {} err {:?} ",i,e))
            .and_then(move |client| { 
                let client_handle = ws::client::spawn(&capture_handle,client,msg_handler);
                keepalive.select(client_handle).map(move |_|info!("client {} done...",i))
                    .map_err(|_|())
            });
        conn_work.push(work);
    }
    
    let work = future::join_all(conn_work);

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
            .filter(Some("echo_client"),loglevel)
            .filter(Some("mimir_transport"),loglevel)
            .init();
}
