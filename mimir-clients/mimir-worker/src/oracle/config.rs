use std::path::Path;
use mimir_util::toml::{self,TomlError};
use mimir_types::Address;
use url::Url;
use url_serde;


/// oracle configuration values.
///
#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct Config {
    /// ethereum address of the primary mimir worker api contract
    #[serde(rename = "mimir-contract", default = "defaults::mimir_contract")]
    pub mimir_contract: Address,

    /// url of ethereum websocket rpc
    #[serde(rename = "websocket-rpc", default = "defaults::websocket_rpc", with = "url_serde")]
    pub websocket_rpc: Url,
   
    /// address of the primary bridge api portal
    #[serde(rename = "bridge-portal", default = "defaults::bridge_portal", with = "url_serde")]
    pub bridge_portal: Url,

    /// address of the auto-funding api portal
    #[serde(rename = "fund-portal", default = "defaults::fund_portal", with = "url_serde")]
    pub fund_portal: Url,
}


impl Config {

    /// attempt to load from target if exists, else return default value.
    ///
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self,TomlError> {
        if path.as_ref().exists() { 
            toml::load(path) 
        } else { 
            Ok(Default::default()) 
        }
    }

    /// save to target file.
    ///
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(),TomlError> { toml::save(self,path) }
}


impl Default for Config {

    fn default() -> Self {
        Config {
            mimir_contract: defaults::mimir_contract(),
            websocket_rpc: defaults::websocket_rpc(),
            bridge_portal: defaults::bridge_portal(),
            fund_portal: defaults::fund_portal(),
        }
    }
}


// TODO: update defaults once alpha deplpyment values have semi-stabilized.
//
mod defaults {
    use mimir_types::Address;
    use url::Url; 

    const HOST: &'static str = "b2i.io";

    pub fn mimir_contract() -> Address {
        "0xc0D9ac1dA08c9744DF9aa09699cC5bf7DFc29AC6".parse()
            .expect("default mimir address must parse")
    }

    pub fn websocket_rpc() -> Url {
        "ws://127.0.0.1:8546".parse().expect("default rpc address must parse")
    }

    pub fn bridge_portal() -> Url {
        //let url_string = format!("wss://{}:8080/mimir-bridge",HOST);
        let url_string = format!("wss://{}:8080/",HOST);
        Url::parse(&url_string).expect("default always valid")
    }

    pub fn fund_portal() -> Url {
        let url_string = format!("https://{}/faucet",HOST);
        Url::parse(&url_string).expect("default always valid")
    }
}


#[cfg(test)]
mod tests {
    use oracle::Config;

    #[test]
    fn defaults() {
        // make sure default values parse
        // without panic...
        let _ = Config::default();
    }
}
