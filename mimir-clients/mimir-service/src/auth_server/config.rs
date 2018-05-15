use std::path::Path;
use mimir_util::toml::{self,TomlError};
use auth_server::SeedSources;
use std::net::SocketAddr;
use std::str::FromStr;
use url::Url;
use url_serde;

/// default config file value
pub(crate) const DEFAULT_CONFIG: &'static str = include_str!("include/config.toml");


/// auth-server configuration values
///
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Config {
    /// seeding configuration; must exist else no seeding will occur
    pub seeding: SeedSources,

    /// socket-address for redis instance
    #[serde(rename = "redis-address", default = "defaults::redis_address")]
    pub redis_address: SocketAddr,

    /// url of ethereum websocket rpc
    #[serde(rename = "websocket-rpc", default = "defaults::websocket_rpc", with = "url_serde")]
    pub websocket_rpc: Url,
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
    use url::Url;

    /// default redis address
    pub fn redis_address() -> SocketAddr {
        "127.0.0.1:6379".parse().expect("default redis address must parse") 
    }

    /// default rpc address
    pub fn websocket_rpc() -> Url {
        "ws://127.0.0.1:8546".parse().expect("default rpc address must parse")
    }
}


#[cfg(test)]
mod tests {
    use auth_server::config::{
        DEFAULT_CONFIG,
        Config,
    };

    #[test]
    fn parse_default() {
        let config: Config = DEFAULT_CONFIG.parse()
            .expect("default config must parse");
        assert!(config.seeding.len() > 0,"default must include *some* seeding configuration");
    }
}
