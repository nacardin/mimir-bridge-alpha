use std::path::Path;
use mimir_util::toml::{self,TomlError};
use mimir_transport::edge::AuthPolicy;
use mimir_transport::common::Role;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;


/// default config file value
pub(crate) const DEFAULT_CONFIG: &'static str = include_str!("include/config.toml");


/// auth-server configuration values
///
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Config { 
    /// per-role auth policies
    #[serde(default)]
    pub policies: HashMap<Role,AuthPolicy>,

    /// socket-address for tcp listener
    #[serde(rename = "serve-address", default = "defaults::serve_address")]
    pub serve_address: SocketAddr,

    /// socket-address for redis instance
    #[serde(rename = "redis-address", default = "defaults::redis_address")]
    pub redis_address: SocketAddr,
}


impl FromStr for Config {

    type Err = TomlError;

    fn from_str(s: &str) -> Result<Self,Self::Err> { toml::from_str(s) }
}


impl Config {

    /// attempt to load from target if exists, else return default value.
    ///
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self,TomlError> {
        if path.as_ref().exists() { 
            toml::load(path) 
        } else {
            let config = DEFAULT_CONFIG.parse()
                .expect("default config must parse");
            Ok(config)
        }
    }

    /// save to target file.
    ///
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(),TomlError> { toml::save(self,path) }
}


mod defaults {
    use std::net::SocketAddr;

    /// default server address
    pub fn serve_address() -> SocketAddr {
        "127.0.0.1:8888".parse().expect("default server address must parse") 
    }

    /// default redis address
    pub fn redis_address() -> SocketAddr {
        "127.0.0.1:6379".parse().expect("default redis address must parse") 
    }
}


#[cfg(test)]
mod tests {
    use edge_server::config::{
        DEFAULT_CONFIG,
        Config,
    };

    #[test]
    fn parse_default() {
        let _config: Config = DEFAULT_CONFIG.parse()
            .expect("default config must parse");
    }
}
