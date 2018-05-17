extern crate mimir_transport;
extern crate mimir_worker;
extern crate mimir_node;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
#[macro_use]
extern crate log;


use mimir_transport::edge;
use mimir_worker::{common,faucet};
use tokio_core::reactor::Core;
use futures::future::{self,Either};
use futures::{Future,Stream};
use mimir_transport::ws;
use log::LevelFilter;


fn main() {

    let opt = faucet::Options::from_args();

    init_logger(opt.log_level);

    let sealer = if opt.dev_keys {
        common::KeyStore::dev_account()
            .sealer().unwrap()
    } else {
        common::KeyStore::init(&opt.keys)
            .expect("unable to parse keystore")
            .sealer().unwrap()
    };

    info!("initializing faucet::{:#}...",sealer.address());

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let node = mimir_node::node::ws(opt.websocket_rpc.as_ref(),&handle)
        .expect("node websocket rpc must be available");
    
    // TODO: await sync

    let mut faucet = faucet::Funder::new(sealer,node);

    let ws_server = ws::server::bind(&handle,opt.serve_address).unwrap();

    let work = ws_server.map_err(|error| error.into())
        .and_then(|(client,address)| {
            info!("incoming connection from {}",address);
            edge::init_server_side_client(client)
        })
        .and_then(move |(ident,tx,rx)| {
            if let Some(fund_work) = faucet.get_fund_work(ident.address) {
                info!("accepting fund request for {}",ident);
                let fund_work = fund_work
                    .map_err(|err| {
                        error!("during funding work {:?}",err);
                        "web3 error during funding attempt".into()
                    })
                    .and_then(move |tx_hash| {
                        info!("{} funded with tx {}",ident,tx_hash);
                        let _ = (tx,rx); // TODO: return success response
                        Ok(())
                    });
                Either::A(fund_work)
            } else {
                info!("rejecting fund request for {}",ident);
                // simply drop client handles...
                Either::B(future::ok(()))
            }
        })
        .for_each(|_| Ok(()));

    core.run(work).unwrap();
}


fn init_logger(loglevel: LevelFilter) {
    if let LevelFilter::Off = loglevel {
        env_logger::init();
    } else {
        env_logger::Builder::from_default_env()
                .filter(Some("faucet_server"),loglevel)
                .filter(Some("mimir_service"),loglevel)
                .filter(Some("mimir_transport"),loglevel)
                .init();
    }
}
