use mimir_common::util::hex;
use secp256k1::Signer;
use rand::{Rand,Rng};
use std::fmt;

/// recoverable signature in form `(r,s,v)`
pub struct Signature(pub [u8;65]);

newtype!(Signature,[u8;65],[u8]);

hex_array!(Signature,65);

array_impls!(
    Signature => Hash,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,
);


impl Default for Signature {
    
    fn default() -> Self { Signature([0u8;65]) }
}

impl Signature {

    /// extract the `v` component of the signature.
    #[inline]
    pub fn get_v(&self) -> u8 { self.0[64] }
    
    /// extract the `r` component of the signature.
    #[inline]
    pub fn get_r(&self) -> &[u8] { &self.0[0..32] }
    
    /// extract the `s` component of the signature.
    #[inline]
    pub fn get_s(&self) -> &[u8] { &self.0[32..64] }

}


impl fmt::Display for Signature {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff = [0u8;130];
        let hex_str = hex::as_str(self.as_ref(),&mut buff);
        if !f.alternate() { f.write_str("0x")?; }
        f.write_str(hex_str)
    }
}


impl Rand for Signature {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let signer: Signer = rng.gen();
        let msg: [u8;32] = rng.gen();
        signer.sign(&msg)
    }
}

