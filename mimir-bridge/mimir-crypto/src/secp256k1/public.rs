use _secp256k1::key::PublicKey;
use secp256k1::Signer;
use rand::{Rand,Rng};


/// `secp256-k1` public key.
pub struct Public(pub [u8;64]);

newtype!(Public,[u8;64],[u8]);

hex_array!(Public,64);

array_impls!(
    Public => Hash,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,
);


impl Default for Public {
    
    fn default() -> Self { Public([0u8;64]) }
}


impl<'a> From<&'a PublicKey> for Public {

    #[inline]
    fn from(key: &'a PublicKey) -> Self {
        let serialized = key.serialize_uncompressed();
        let mut buff = [0u8;64];
        buff.copy_from_slice(&serialized[1..65]);
        Public(buff)
    }
}


impl Rand for Public {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let signer: Signer = rng.gen();
        signer.public()
    }
}

