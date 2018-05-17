use structopt::StructOpt;
use std::path::PathBuf;
use log::LevelFilter;
use url::Url;


/// command line args
#[derive(Debug,Clone,StructOpt)]
#[structopt(name = "mimir-notary", about = "simple mimir-bridge notary client")]
pub struct Options {

    /// set max log level
    #[structopt(long = "log-level",default_value = "info")]
    pub log_level: LevelFilter,

    /// set mimir-bridge websocket portal
    #[structopt(long = "bridge-portal",default_value = "ws://127.0.0.1:8888/")]
    pub bridge_portal: Url,

    /// path to key file
    #[structopt(long = "key-store", default_value = "notary-keys.toml", parse(from_os_str))]
    pub keys: PathBuf,
}


impl Options {

    pub fn from_args() -> Self { <Self as StructOpt>::from_args() }
}

