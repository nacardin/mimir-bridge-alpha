use structopt::StructOpt;
use std::net::SocketAddr;
use std::path::PathBuf;
use log::LevelFilter;
use url::Url;


/// command line args
#[derive(Debug,Clone,StructOpt)]
#[structopt(name = "faucet-server", about = "faucet server for mimir-bridge dev/test deployments")]
pub struct Options {

    /// max log level
    #[structopt(long = "log-level",default_value = "info")]
    pub log_level: LevelFilter,

    /// use parity development account keys
    #[structopt(long = "dev-keys")]
    pub dev_keys: bool,

    /// ethereum websocket rpc address
    #[structopt(long = "websocket-rpc",default_value = "ws://127.0.0.1:8546")]
    pub websocket_rpc: Url,
 
    /// socket-address of tcp listener
    #[structopt(long = "serve-address",default_value = "127.0.0.1:7979")]
    pub serve_address: SocketAddr,

    /// path to key file
    #[structopt(long = "key-store", default_value = "faucet-keys.toml", parse(from_os_str))]
    pub keys: PathBuf,
}


impl Options {

    pub fn from_args() -> Self { <Self as StructOpt>::from_args() }
}
