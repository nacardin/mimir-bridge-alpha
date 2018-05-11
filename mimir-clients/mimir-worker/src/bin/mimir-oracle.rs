extern crate mimir_transport;
extern crate mimir_worker;
extern crate mimir_node;
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
#[macro_use]
extern crate serde_json;
extern crate reqwest;
extern crate env_logger;
#[macro_use]
extern crate log;

use futures::{Future,Stream,Sink};
use tokio_core::reactor::Core;
use tokio_timer::Timer;
use std::time::Duration;
use log::LevelFilter;

use mimir_worker::common::KeyStore;
use mimir_transport::{ws,edge};
use mimir_transport::common::{
    Message,
    Command,
    Auth,
    Role,
    MSG,
};use mimir_worker::oracle::{
    SimpleOracle,
    Options,
    Config,
};


fn main() {
    // load command line options & config values
    let mut opt = Options::from_args();
    let conf = Config::init(&opt.config).unwrap();

    // set up logger & give some initial data
    init_logger(opt.log_level);
    info!("oracle starting...");
    debug!("using options {:?}",opt);
    debug!("using config {:?}",conf);

    // set up event loop and futures-aware timer thread. 
    let mut core = Core::new().unwrap();
    let handle = core.handle(); 
    let timer = Timer::default();
   
    // set up basic oracle client handle.
    let sealer = KeyStore::init(&opt.keys)
        .unwrap().sealer().unwrap(); 
    let node = mimir_node::node::ipc(&opt.ipc,&handle).unwrap();
    let oracle = SimpleOracle::new(sealer,node);
     
    // set up synchronous http client for handling simple
    // api calls during init sequence.
    let client = reqwest::Client::new();

    if !opt.skip_all {
        // unless working on a local dev chain, the client should
        // verify that the local node is sychronized before continuing...
        if !opt.skip_sync {
            info!("verifying sync state...");
            let poll_sync = timer.interval(Duration::from_millis(500));
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

        // attempt to auto-fund against testnet faucet if `auto-fund` uption
        // was set (may be set by command line or balance check).
        if opt.auto_fund {
            info!("attempting auto-funding against {}",conf.fund_portal);
            let Auth { addr, role, time, seal } = oracle.gen_auth();
            let authorize = json!({
                "msg": {
                    "addr": addr,
                    "role": role,
                    "time": time,
                },
                "sig": seal,
            });
            debug!("auto-fund with auth {}",authorize);
            let rsp = client.post(conf.fund_portal.clone())
                .json(&authorize).send()
                .expect("faucet must be active");
            if !rsp.status().is_success() {
                error!("bad faucet response {:?}",rsp);
                panic!("cannot continue without funding");
            }
            info!("facet OK, waiting on funding tx...");
            for i in 0.. {
                let sleep = timer.sleep(Duration::from_secs(10));
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
            serve
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

