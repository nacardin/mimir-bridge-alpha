use mimir_crypto::Keccak256;
use mimir_types::Address;
use mimir_util::hex;
use common::{
    Channel,
    Role,
    MSG,
};
use std::str::FromStr;


// msg format: <flag> <addr> <dest> [<data>]
// example: QUERY 0x9469d56752abf5120c568FF2F94175841B829ee7 ANY {"foo":"bar"}
//


pub struct Message {
    pub flag: MSG,
    pub addr: Address,
    pub dest: Option<Address>,
    pub data: Option<String>,
}


impl Message {

    /// get channel that this message should be published to.
    ///
    pub fn channel(&self) -> Channel {
        let role = self.flag.consumer();
        if let Some(addr) = self.dest {
            Channel::Direct { addr, role }
        } else {
            Channel::Shared { role }
        }
    }
}


#[inline]
fn hash_elems(flag: MSG, addr: &Address, dest: &Channel, data: Option<&str>) -> [u8;32] { 
    let flag_bytes = flag.as_ref().as_bytes();
    let mut hasher = Keccak256::default();
    hasher.absorb(flag_bytes);
    hasher.absorb(&addr);
    hasher.absorb(dest.to_string().as_ref());
    if let Some(s) = data {
        hasher.absorb(s.as_ref())
    }
    hasher.finish()
}
