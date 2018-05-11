extern crate mimir_transport;
extern crate mimir_crypto;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;


use mimir_transport::{ws,edge};
use mimir_transport::common::{
    Command,
    Role,
};
use mimir_crypto::secp256k1::{
    Signer,
    keygen,
};
use tokio_core::reactor::Core;
use futures::{future,Future,Stream,Sink};
use log::LevelFilter;



fn main() {
    init_logger(false);

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let address = "ws://127.0.0.1:8888".parse().unwrap();

    let mut conn_work = Vec::new();

    for i in 0..32 {
        let (_,secret) = keygen();
        let signer = Signer::new(secret).unwrap();
        let identify = Command::identify(Role::Admin,&signer);
        let debug = Command::debug(identify.dest,format!("edge-client test {:02}",i));
        let kick = Command::kick(identify.dest,&signer);
        let connect = ws::client::connect(&handle,&address,identify.to_string());
        let work = connect.map_err(|e|e.into())
            .and_then(move |client| {
                let (tx,rx) = edge::split_client(client);
                let sub_work = tx.send(debug.into())
                    .and_then(|tx|tx.send(kick.into()))
                    .join(
                        rx.for_each(move |op| {
                            info!("incoming[{:02}] {}",i,op);
                            Ok(())
                        })
                    );
                sub_work
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
            .filter(Some("edge_client"),loglevel)
            .filter(Some("mimir_transport"),loglevel)
            .init();
}
