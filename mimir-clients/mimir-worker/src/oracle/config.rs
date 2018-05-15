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

    /// address of the worker-login api portal
    #[serde(rename = "login-portal", default = "defaults::login_portal", with = "url_serde")]
    pub login_portal: Url,
   
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
            login_portal: defaults::login_portal(),
            fund_portal: defaults::fund_portal(),
        }
    }
}


// TODO: update defaults once alpha deplpyment values have semi-stabilized.
//
mod defaults {
    use mimir_types::Address;
    use url::Url;
   
    // 0x6b75a9D62C8C3Bf9A8febED2E937f7aa10AeBb86
    const MIMIR: [u8;20] = [ 
        0x6b, 0x75, 0xa9, 0xD6, 0x2C, 0x8C, 0x3B, 0xf9, 0xA8, 0xfe,
        0xbE, 0xD2, 0xE9, 0x37, 0xf7, 0xaa, 0x10, 0xAe, 0xBb, 0x86
    ];
    
    const HOST: &'static str = "b2i.io";

    pub fn mimir_contract() -> Address { Address::from(MIMIR) }

    pub fn websocket_rpc() -> Url {
        "ws://127.0.0.1:8546".parse().expect("default rpc address must parse")
    }

    pub fn bridge_portal() -> Url {
        //let url_string = format!("wss://{}:8080/mimir-bridge",HOST);
        let url_string = format!("wss://{}:8080/",HOST);
        Url::parse(&url_string).expect("default always valid")
    }

    pub fn login_portal() -> Url { 
        let url_string = format!("https://{}/worker-login",HOST);
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
