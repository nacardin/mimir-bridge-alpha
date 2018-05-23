extern crate mimir_transport;
extern crate mimir_worker;
extern crate mimir_node;
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate serde_json;
extern crate env_logger;
#[macro_use]
extern crate log;

use futures::{Future,Stream,Sink};
use tokio_core::reactor::Core;
use tokio_timer::{Delay,Interval};
use std::time::{Instant,Duration};
use log::LevelFilter;

use mimir_worker::common::KeyStore;
use mimir_transport::{ws,edge};
use mimir_transport::common::{
    Message,
    Command,
    Role,
    MSG,
};use mimir_worker::oracle::{
    SimpleOracle,
    Options,
    Config,
};


fn main() {
    // load command line options
    let mut opt = Options::from_args(); 

    // set up logger & give some initial data
    init_logger(opt.log_level);
    
    // load configuration & key files
    let conf = Config::init(&opt.config).unwrap();
    let sealer = KeyStore::init(&opt.keys)
        .unwrap().sealer().unwrap();

    info!("oracle::{:#} starting...",sealer.address());
    debug!("using options {:?}",opt);
    debug!("using config {:?}",conf);

    // set up event loop and futures-aware timer thread. 
    let mut core = Core::new().unwrap();
    let handle = core.handle();
   
    // set up basic oracle client handle.
    let node = mimir_node::node::ws(conf.websocket_rpc.as_ref(),&handle).unwrap();
    let oracle = SimpleOracle::new(sealer,node);


    if !opt.skip_all {
        // unless working on a local dev chain, the client should
        // verify that the local node is sychronized before continuing...
        if !opt.skip_sync {
            info!("verifying sync state...");
            let poll_sync = Interval::new(Instant::now(),Duration::from_millis(512));
            let await_sync = oracle.node().util().await_sync(poll_sync);
            core.run(await_sync).unwrap();
            info!("local node appears synced");
        } else {
            info!("skipping sync checks...");
        }

        // check worker account balance
        let worker_address = oracle.sealer().address(); 
        let balance_check = oracle.node().eth().balance(worker_address.as_ref().into(),None);
        let balance = core.run(balance_check).unwrap();

        info!("worker balance {:?} (wei)",balance);
        if balance.is_zero() {
            opt.auto_fund = true;
        }

        if opt.auto_fund {

            let identify = Command::identify(Role::Oracle,oracle.sealer());

            let fund_work = ws::client::connect(&handle,&conf.fund_portal,identify.to_string())
                .and_then(|client| client.flush());

            let _ = core.run(fund_work).expect("auto-funding failed");

            for i in 0.. {
                let sleep = Delay::new(Instant::now() + Duration::from_secs(10));
                core.run(sleep).unwrap();
                let balance_check = oracle.node().eth().balance(worker_address.as_ref().into(),None);
                if core.run(balance_check).unwrap() > balance { break; }
                if i > 10 {
                    panic!("funding TX failed; faucet encountered error or is depleted");
                }
            }
        } else {
            debug!("skipping auto-funding...")
        }


        // check if worker is in "bound" state (locked stake)
        let stake_check = oracle.check_bound_state(conf.mimir_contract);
        let is_bound = core.run(stake_check).unwrap();

        if !is_bound {
            opt.lock_stake = true;
        }

        // if worker is running for first time, it will need to
        // lock stake in order to serve in the system.
        // TODO: add direct stake checks.  currently uses account balance as a 
        // proxy for stake locking. this may cause a client to enter an unrecoverable 
        // state if it fails after funding but before locking. 
        if opt.lock_stake {
            info!("locking worker stake...");
            let tx_work = oracle.lock_stake(conf.mimir_contract);
            let receipt = core.run(tx_work).unwrap();
            info!("lock-stake transaction mined {:?}",receipt.transaction_hash);
        } else {
            info!("stake appears locked...");
        }
    } else {
        info!("skipping all startup checks...");
    }


    let identify = Command::identify(Role::Oracle,oracle.sealer());

    let connect = ws::client::connect(&handle,&conf.bridge_portal,identify.to_string());

    let oracle_address = oracle.sealer().address();

    let poll_block = Interval::new(Instant::now(),Duration::from_secs(1));

    let block_stream = mimir_node::helpers::BlockStream::new(poll_block,oracle.node().transport().to_owned());

    let block_stream = mimir_node::helpers::Lag::new(block_stream,1);

    let monitor_blocks = block_stream.for_each(|block| {
        if let Some(number) = block.number {
            debug!("new block {:?}",number);
        }
        // TODO update oracle blockstate
        Ok(())
    });

    let work = connect.map_err(|e|error!("while connecting {}",e))
        .and_then(move |client| {
            let (tx,rx) = edge::split_client(client);
            let serve = rx.map_err(|e|error!("in incoming msg string {}",e))
                .filter_map(move |operation| {
                    match operation.expect_message(MSG::QUERY) {
                        Ok(message) => {
                            match serde_json::from_str(message.msg_payload()) {
                                Ok(request) => {
                                    let work = oracle.serve_request(request)
                                        .map_err(|e|error!("oracle error {:?}",e));
                                    Some(work)
                                },
                                Err(err) => {
                                    warn!("got error `{}` while parsing `{}`",err,message);
                                    None
                                },
                            }
                        },
                        Err(op) => {
                            warn!("unexpected operation {}",op);
                            None
                        }
                    }
                })
                .buffer_unordered(128)
                .filter_map(move |message| {
                    match serde_json::to_string(&message) {
                        Ok(msg_string) => {
                            let msg = format!("NOTARIZE {} {}",oracle_address,msg_string);
                            Some(Message::from_string(msg).expect("always valid `Message` object").into())
                        },
                        Err(err) => {
                            warn!("got error `{}` while serializing `{:?}`",err,message);
                            None
                        }
                    }
                })
                .forward(tx.sink_map_err(|e|error!("in outgoing msg sink {}",e)))
                .map(|_|());
            let work = serve.select(monitor_blocks.map_err(|e|error!("in block monitor {:?}",e)))
                .map_err(|_|()).map(|_|());
            work
        });

    // do work.
    core.run(work).unwrap()
}



fn init_logger(loglevel: LevelFilter) {
    if let LevelFilter::Off = loglevel {
        env_logger::init();
    } else {
        env_logger::Builder::from_default_env()
                .filter(Some("mimir_oracle"),loglevel)
                .filter(Some("mimir_worker"),loglevel)
                .filter(Some("mimir_node"),loglevel)
                .filter(Some("mimir_transport"),loglevel)
                .init();
    }
}

