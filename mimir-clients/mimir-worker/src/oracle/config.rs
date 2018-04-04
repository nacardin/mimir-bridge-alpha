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
    
    const MIMIR: [u8;20] = [
        0x66, 0xb4, 0x00, 0xf2, 0x82, 0xFD, 0xf7, 0x9F, 0xC0, 0x7b, 
        0xFc, 0xC0, 0x04, 0xF3, 0x66, 0xC0, 0x8d, 0x14, 0xD0, 0x7a
    ];
    
    const HOST: &'static str = "b2i.io";

    pub fn mimir_contract() -> Address { Address::from(MIMIR) }

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
