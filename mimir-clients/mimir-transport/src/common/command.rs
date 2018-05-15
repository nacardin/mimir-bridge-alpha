use mimir_types::{Signature,Address,U256};
use mimir_proto::seal::Sealer;
use mimir_crypto::secp256k1::{
    Verifier,
    Error,
};
use mimir_crypto::Keccak256;
use mimir_util::unix_time;
use common::{
    ParseError,
    Identity,
    Channel,
    Role,
    CMD,
};
use std::str::FromStr;
use std::fmt::{self,Write};


/// a command sent to be consumed by an edge node
///
/// ```
/// #
/// extern crate mimir_transport;
/// use mimir_transport::common::Command;
/// # fn main() {
/// 
/// let raw = "DEBUG oracle::00a329c0648769a73afac7f9381e08fb43dbea72 0x5af09f79 some message";
///
/// let cmd: Command = raw.parse().unwrap();
/// 
/// // the `DEBUG` command is an unsigned variant
/// assert!(cmd.seal.is_none());
///
/// // the `to_string` method produces a valid serializaiton
/// assert_eq!(&cmd.to_string(),raw);
/// 
/// // `DEBUG` treats all data after the timestamp as an arbitrary string
/// assert_eq!(&cmd.data.unwrap(),"some message");
/// # }
/// ```
///
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Command {
    /// flag indicating the command variant
    pub flag: CMD,
    
    /// destination/target of this command
    pub dest: Identity,
    
    /// unix timestamp at which command was produced
    pub time: U256,
    
    /// arbitrary payload data, if any exists
    pub data: Option<String>,
    
    /// signature of preceeding fields (required by some variants)
    pub seal: Option<Signature>,
}


impl Command {

    fn new_signed<S: Sealer>(flag: CMD, dest: Identity, data: Option<String>, sealer: S) -> Self {
        debug_assert!(flag.signed_variant(),"should only be called on signed variants");
        let time = unix_time().into();
        let hash = hash_elems(flag,dest.role,dest.address,time,data.as_ref());
        let seal = Some(sealer.sign(&hash));
        Self { flag, dest, time, data, seal }
    }

    fn new_unsigned(flag: CMD, dest: Identity, data: Option<String>) -> Self {
        let time = unix_time().into();
        let seal = None;
        Self { flag, dest, time, data, seal }
    }

    pub fn identify<S: Sealer>(role: Role, sealer: S) -> Self {
        let dest = Identity::new(sealer.address(),role);
        Self::new_signed(CMD::IDENTIFY,dest,None,sealer)
    }

    pub fn kick<S: Sealer>(dest: Identity, sealer: S) -> Self {
        Self::new_signed(CMD::KICK,dest,None,sealer)
    }

    pub fn debug(dest: Identity, data: String) -> Self {
        Self::new_unsigned(CMD::DEBUG,dest,Some(data))
    }

    pub fn cmd_variant(&self) -> CMD { self.flag }

    pub fn dest_channel(&self) -> Channel { self.dest.direct_channel() }

    pub fn recover(&self) -> Result<Option<Address>,Error> {
        let verifier = Default::default();
        self.recover_with(&verifier)
    }

    pub fn recover_with(&self, verifier: &Verifier) -> Result<Option<Address>,Error> {
        if let Some(ref seal) = self.seal {
            let hash = hash_elems(self.flag,self.dest.role,self.dest.address,self.time,self.data.as_ref());
            let addr = verifier.ecrecover(&hash,seal)?;
            Ok(Some(addr))
        } else {
            Ok(None)
        }
    }
}


const DELIM: char = ' ';

impl FromStr for Command {

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let mut split = s.splitn(4,DELIM);
        match (split.next(),split.next(),split.next(),split.next()) {
            (Some(flag),Some(dest),Some(time),tail) => {
                let flag: CMD = flag.parse()
                    .map_err(|_|ParseError::BadCmdVariant)?;
                let dest: Identity = dest.parse()?;
                let time: U256 = time.parse()
                    .map_err(|_|ParseError::BadTimestamp)?;
                let (data,seal) = if flag.signed_variant() {
                    if let Some(tail) = tail {
                        let mut rsplit = tail.rsplitn(2,DELIM);
                        let seal = rsplit.next().ok_or(ParseError::MissingVal)
                            .and_then(|s: &str| {
                                s.parse().map_err(|_|ParseError::BadSignature)
                            })?;
                        (rsplit.next().map(|s|s.to_string()),Some(seal))
                    } else {
                        return Err(ParseError::MissingVal);
                    }
                } else {
                    (tail.map(|s|s.to_string()),None)
                };
                Ok(Self { flag, dest, time, data, seal })
            },
            _ => Err(ParseError::MissingVal)
        }
    }
}


impl fmt::Display for Command {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.flag.as_ref())?;
        f.write_char(DELIM)?;
        self.dest.fmt(f)?;
        f.write_char(DELIM)?;
        self.time.fmt(f)?;
        if let Some(ref data) = self.data {
            f.write_char(DELIM)?;
            f.write_str(data)?;
        }
        if let Some(ref seal) = self.seal {
            f.write_char(DELIM)?;
            seal.fmt(f)?;
        }
        Ok(())
    }
}


fn hash_elems(flag: CMD, role: Role, addr: Address, time: U256, data: Option<&String>) -> [u8;32] {
    let flag_bytes = flag.as_ref().as_bytes();
    let role_bytes = role.as_ref().as_bytes();
    let mut hasher = Keccak256::default();
    hasher.absorb(flag_bytes);
    hasher.absorb(role_bytes);
    hasher.absorb(&addr); 
    hasher.absorb(&time);
    if let Some(ref data) = data {
        hasher.absorb(data.as_ref());
    }
    hasher.finish()
}

