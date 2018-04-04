use _secp256k1::key::SecretKey;
use secp256k1::Signer;
use rand::{Rand,Rng};


/// `secp256-k1` secret key.
#[derive(Hash,Default,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Secret(pub [u8;32]);

newtype!(Secret,[u8;32],[u8]);

hex_array!(Secret,32);


impl<'a> From<&'a SecretKey> for Secret {

    #[inline]
    fn from(key: &'a SecretKey) -> Self {
        let mut buff = [0u8;32];
        buff.copy_from_slice(&key[0..32]);
        Secret(buff)
    }
}


impl Rand for Secret {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let signer: Signer = rng.gen();
        signer.secret()
    }
}

