use serde::{self,Serialize,Serializer,Deserialize,Deserializer};
use mimir_common::types::Either;
use mimir_common::util::hex;
use mimir_crypto::Address;
use types::{
    ParseError,
    Role
};
use std::str::FromStr;
use std::fmt;



/// identity of an entity
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Identity {
    /// address of entity
    pub address: Address,
    /// role of entity
    pub role: Role,
}


impl Identity {

    /// instantiate new identity
    pub fn new(address: Address, role: Role) -> Self { Self { address, role } }

    /// get shared channel for this identity
    pub fn shared_channel(&self) -> Channel {
        Channel::Shared { role: self.role }
    }

    /// get direct channel for this identity
    pub fn direct_channel(&self) -> Channel {
        Channel::Direct { role: self.role, addr: self.address }
    }
}


impl FromStr for Identity {

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let mut split = s.splitn(2,"::");
        match (split.next(),split.next()) {
            (Some(role),Some(address)) => {
                let role: Role = role.parse()
                    .map_err(|_|ParseError::BadRoleVariant)?;
                let address: Address = address.parse()
                    .map_err(|_|ParseError::BadAddress)?;
                Ok(Self { address, role })
            },
            _ => {
                Err(ParseError::MissingVal)
            }
        }
    }
}


impl fmt::Display for Identity {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.role.as_ref())?;
        f.write_str("::")?;
        let mut buff = [0u8;40];
        let hex_str = hex::as_str(&self.address,&mut buff);
        f.write_str(hex_str)
    }
}


impl Serialize for Identity {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string()) // TODO: avoid intermediate allocation
    }
}


impl<'de> Deserialize<'de> for Identity {

    fn deserialize<D>(deserializer: D) -> Result<Self,D::Error> where D: Deserializer<'de> {
        let target: Either<&str,String> = Deserialize::deserialize(deserializer)?;
        let target_str: &str = target.as_ref();
        target_str.parse().map_err(serde::de::Error::custom)
    }
}
