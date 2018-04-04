use rand::{Rand,Rng};
use secp256k1::{Signer,Public};
use keccak256::Keccak256;


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


impl Rand for Address {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let signer: Signer = rng.gen();
        signer.address()
    }
}

