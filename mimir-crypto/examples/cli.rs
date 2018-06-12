#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate mimir_crypto;
extern crate rand;
extern crate toml;

use mimir_crypto::secp256k1::Signer;
use mimir_crypto::secp256k1::Public;
use mimir_crypto::secp256k1::Secret;
use mimir_crypto::secp256k1::Address;
use std::fs;

use docopt::Docopt;

const USAGE: &'static str = r#"
Mimir-Crypto cli
           ____
          /\   \
         /  \___\
        _\  / __/_
       /\ \/_/\   \
      /  \__/  \___\
     _\  /  \  / __/_
    /\ \/___/\/_/\   \
   /  \___\    /  \___\
  _\  /   /_  _\__/ __/_
 /\ \/___/  \/\   \/\   \
/  \___\ \___\ \___\ \___\
\  /   / /   / /   / /   /
 \/___/\/___/\/___/\/___/

Usage:
crypto-cli keygen
"#;

#[derive(Debug, Deserialize)]
struct Args {
    cmd_keygen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Keys {
    public: Public,
    secret: Secret,
    address: Address,
}

fn main() {
    println!("{}", USAGE);
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_keygen == true {
        retrieve_keys();
    // store_keys(keygen()).unwrap()
    } else {
        println!("Something is wrong with keygen");
    }
}

fn keygen() -> Keys {
    let signer: Signer = rand::random();
    Keys {
        public: signer.public(),
        secret: signer.secret(),
        address: signer.address(),
    }
}

fn store_keys(keys: Keys) -> Result<(), Error> {
    let toml = toml::to_string(&keys)?;
    fs::write("keys.toml", toml)?;
    Ok(())
}

fn retrieve_keys() -> Result<Keys, Error> {
    let read = fs::read_to_string("keys.toml")?;
    let toml: Keys = toml::from_str(&read)?;
    Ok(toml)
}

fn sign_transaction(keys: Keys) {}

// -------------------------------

use std::{fmt, error};

#[derive(Debug)]
pub enum Error {
    Error(Box<error::Error>),
    Message(&'static str),
}


impl Error {
    pub fn message(msg: &'static str) -> Self {
        Error::Message(msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Error(err) => err.fmt(f),
            Error::Message(msg) => f.write_str(msg),
        }
    }
}


impl<T> From<T> for Error
where
    T: error::Error + 'static,
{
    fn from(err: T) -> Self {
        Error::Error(Box::new(err))
    }
}
