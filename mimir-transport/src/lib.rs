#[macro_use]
extern crate mimir_common;
extern crate mimir_crypto;
extern crate mimir_proto;
extern crate tokio_tungstenite;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate url;
#[macro_use]
extern crate log;

pub(crate) mod error;
pub mod service;
pub mod redis;
//pub mod util;
pub mod rds; // temp
//pub mod eth;
//pub mod rpc;
pub mod ws;

pub use error::Error;

