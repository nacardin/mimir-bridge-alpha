use structopt::StructOpt;
use std::path::PathBuf;
use log::LevelFilter;


/// command line args
#[derive(Debug,Clone,StructOpt)]
#[structopt(name = "mimir-oracle", about = "simple mimir-bridge oracle client")]
pub struct Options {

    /// set max log level
    #[structopt(long = "log-level",default_value = "info")]
    pub log_level: LevelFilter,

    /// skip all startup checks
    #[structopt(long = "skip-all-checks")]
    pub skip_all: bool,

    /// fund via faucet (testnet)
    #[structopt(long = "auto-fund")]
    pub auto_fund: bool,

    /// skip blockchain sync checks
    #[structopt(long = "skip-sync")]
    pub skip_sync: bool,

    /// lock stake (set to true on first exec)
    #[structopt(long = "lock-stake")]
    pub lock_stake: bool,

    /// path to config file
    #[structopt(long = "config", default_value = "config.toml", parse(from_os_str))]
    pub config: PathBuf,

    /// path to key file
    #[structopt(long = "key-store", default_value = "oracle-keys.toml", parse(from_os_str))]
    pub keys: PathBuf,
}


impl Options {

    pub fn from_args() -> Self { <Self as StructOpt>::from_args() }
}

