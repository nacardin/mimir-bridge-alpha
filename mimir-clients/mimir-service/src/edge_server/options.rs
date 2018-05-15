use structopt::StructOpt;
use log::LevelFilter;
use std::path::PathBuf;


/// command line args
#[derive(Debug,Clone,StructOpt)]
#[structopt(name = "edge-server", about = "mimir-bridge edge server")]
pub struct Options {

    /// set max log level
    #[structopt(long = "log-level",default_value = "info")]
    pub log_level: LevelFilter,

    /// path to config file
    #[structopt(long = "config", default_value = "config.toml", parse(from_os_str))]
    pub config: PathBuf,
}


impl Options {

    pub fn from_args() -> Self { <Self as StructOpt>::from_args() }
}

