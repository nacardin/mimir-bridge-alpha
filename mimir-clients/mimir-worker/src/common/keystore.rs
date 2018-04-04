use mimir_util::toml::{self,TomlError};
use mimir_types::{Address,Secret};
use mimir_crypto::secp256k1::{Signer,Error};
use common::ArcSealer;
use std::path::Path;
use std::sync::Arc;
use rand;


/// serialization/deserialization target for worker key storage.
///
/// WARNING: this is an **unencrypted** keystore.  it is intended for
/// use in alpha clients working on testnets.  ye be warned.
///
#[derive(Default,Debug,Clone,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct KeyStore {
    pub address: Address,
    pub secret: Secret,
}


impl KeyStore {

    pub fn sealer(&self) -> Result<ArcSealer,Error> {
        let inner = Signer::new(&self.secret)?;
        Ok(Arc::new(inner))
    }

    pub fn init<P>(path: P) -> Result<Self,TomlError> where P: AsRef<Path> {
        if path.as_ref().is_file() {
            Self::load(path)
        } else {
            let signer: Signer = rand::random();
            let (address,secret) = (signer.address(),signer.secret());
            let keystore = KeyStore { address, secret };
            keystore.save(path)?;
            Ok(keystore)
        }
    }

    pub fn load<P>(path: P) -> Result<Self,TomlError> where P: AsRef<Path> { toml::load(path) }

    pub fn save<P>(&self, path: P) -> Result<(),TomlError> where P: AsRef<Path> { toml::save(self,path) }
}
