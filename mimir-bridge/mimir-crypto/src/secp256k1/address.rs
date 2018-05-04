use mimir_util::hex;
use rand::{Rand,Rng};
use secp256k1::{Signer,Public};
use keccak256::Keccak256;
use std::fmt;

/// ethereum style address.
#[derive(Hash,Default,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Address(pub [u8;20]);

newtype!(Address,[u8;20],[u8]);

hex_array!(Address,20);


impl<'a> From<&'a Public> for Address {

    #[inline]
    fn from(public: &'a Public) -> Self {
        let hash = Keccak256::hash(public.as_ref());
        let mut buff = [0u8;20];
        buff.copy_from_slice(&hash[12..]);
        Address(buff)
    }
}


impl fmt::Display for Address {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff = [0u8;40];
        let hex_str = hex::as_str(self.as_ref(),&mut buff);
        if !f.alternate() { f.write_str("0x")?; }
        f.write_str(hex_str)
    }
}


impl Rand for Address {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let signer: Signer = rng.gen();
        signer.address()
    }
}

