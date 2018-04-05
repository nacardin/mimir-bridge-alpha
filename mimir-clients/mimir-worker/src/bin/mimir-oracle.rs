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

use serde_json::Value;
use futures::future::{self,Future,IntoFuture,Either};
use tokio_core::reactor::Core;
use tokio_timer::Timer;
use std::time::Duration;
use log::LevelFilter;

use mimir_worker::common::KeyStore;
use mimir_transport::common::Auth;
use mimir_transport::ws;
use mimir_worker::oracle::{
    SimpleOracle,
    OracleOp,
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

    // check balance of worker account
    let worker_address = oracle.sealer().address();
    let balance_check = oracle.node().eth().balance(worker_address.as_ref().into(),None);
    let balance = core.run(balance_check).unwrap();
 
    // assume we're on first run if balance is zero
    if balance.is_zero() {
        info!("account balance is zero, assuming uninitialized...");
        opt.auto_fund = true;
        opt.lock_stake = true;
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
        assert!(rsp.status().is_success(),"faucet rsp must be OK");
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
        info!("assuming stake as locked...");
    }

    // call login portal and request a certificate for logging
    // onto the main api.
    info!("attempting login against {}",conf.login_portal); 
    let Auth { addr, role, time, seal } = oracle.gen_auth();
    let authorize = json!({
        "msg": {
            "addr": addr,
            "role": role,
            "time": time,
        },
        "sig": seal,
    });
    debug!("login with auth {}",authorize);
    let mut rsp = client.post(conf.login_portal.clone())
        .json(&authorize).send()
        .expect("login portal must be active");
    assert!(rsp.status().is_success(),"login rsp must be OK");
    let rsp_json: Value = serde_json::from_str(&rsp.text().unwrap()).unwrap();
    let login_json = json!({
        "op" : "login",
        "msg": rsp_json
    });
    let login = serde_json::to_string(&login_json).unwrap(); 


    // execute handshake against main bridge api portal.
    info!("attempting handshake against {}",conf.bridge_portal);
    let connect = ws::client::connect(&handle,&conf.bridge_portal,login.into());
    let client = core.run(connect).unwrap();

    // build primary work future.
    let work = ws::client::spawn(&handle,client,move |text: String| {
        trace!("incoming text {}",text);
        let parse: Result<OracleOp,_> = serde_json::from_str(&text);
        let remap: Result<_,_> = parse.map_err(|e| error!("parse error {:?}",e))
            .map(|operation| {
                match operation {
                    OracleOp::Query(request) => {
                        info!("serving {:?}",request);
                        let work = oracle.serve_request(request)
                            .map_err(|e| error!("oracle error {:?}",e))
                            .map(|message| Some(message));
                        Either::A(work)
                    },
                    OracleOp::Block(block_info) => {
                        oracle.set_block(block_info);
                        Either::B(future::ok(None))
                    },
                } 
            });
        let work = remap.into_future().flatten()
            .and_then(|rslt| { 
                if let Some(message) = rslt {
                    info!("sending {:?}",message);
                    let msg = json!({
                        "op": "notarize",
                        "msg": message
                    }); 
                    match serde_json::to_string(&msg) {
                        Ok(msg) => Ok(Some(msg)),
                        Err(e) => {
                            error!("serialization failed {:?}",e);
                            Ok(None)
                        },
                    }
                } else {
                    Ok(None)
                }
            })
            .then(|rslt| Ok(rslt.unwrap_or(None)));
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

