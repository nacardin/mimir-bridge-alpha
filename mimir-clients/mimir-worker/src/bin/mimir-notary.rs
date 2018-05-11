extern crate mimir_transport;
extern crate mimir_worker;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate env_logger;
#[macro_use]
extern crate log;

use futures::{Future,Stream,Sink};
use tokio_core::reactor::Core;
use log::LevelFilter;

use mimir_worker::common::KeyStore;
use mimir_worker::notary::{
    Options,
    Notary,
};
use mimir_transport::{ws,edge};
use mimir_transport::common::{
    Message,
    Command,
    Role,
    MSG,
};



fn main() {

    let opt = Options::from_args();

    init_logger(opt.log_level);

    info!("notary starting...");
    debug!("using options {:?}",opt);

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let sealer = KeyStore::init(&opt.keys)
        .unwrap().sealer().unwrap();
    let notary = Notary::new(sealer);

    let identify = Command::identify(Role::Notary,notary.sealer());

    let connect = ws::client::connect(&handle,&opt.bridge_portal,identify.to_string());

    let client = core.run(connect).expect("bridge-portal connection error");

    let (tx,rx) = edge::split_client(client);

    let notary_address = notary.sealer().address();

    let notarize = rx.filter_map(|operation| {
            match operation.expect_message(MSG::NOTARIZE) {
                Ok(message) => {
                    match serde_json::from_str(message.msg_payload()) {
                        Ok(request) => {
                            let notarize = notary.notarize(request)
                                .map(|msg|Some(msg)).or_else(|err| {
                                    warn!("notary error {}",err);
                                    Ok(None)
                                });
                            Some(notarize)
                        },
                        Err(error) => {
                            warn!("got `{}` while parsing `{}`",error,message);
                            None
                        }
                    }
                },
                Err(op) => {
                    warn!("unexpected operation `{}`",op);
                    None
                }
            }
        }) 
        .buffer_unordered(128)
        .filter_map(|opt: Option<_>|opt)
        .filter_map(|message| {
            match serde_json::to_string(&message) {
                Ok(msg_string) => {
                    let msg = format!("YIELD {} {} {}",notary_address,message.payload.address,msg_string);
                    Some(Message::from_string(msg).expect("always valid `Message` object").into())
                },
                Err(error) => {
                    warn!("got error `{}` while serializing `{:?}`",error,message);
                    None
                },
            }
        })
        .map_err(|e|error!("in rx stream {}",e))
        .forward(tx.sink_map_err(|e|error!("in tx sink {}",e)))
        .map(|_|());

    core.run(notarize).unwrap();
}


fn init_logger(loglevel: LevelFilter) {
    if let LevelFilter::Off = loglevel {
        env_logger::init();
    } else {
        env_logger::Builder::from_default_env()
                .filter(Some("mimir_notary"),loglevel)
                .filter(Some("mimir_worker"),loglevel)
                .filter(Some("mimir_transport"),loglevel)
                .init();
    }
}
