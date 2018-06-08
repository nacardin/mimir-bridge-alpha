use std::fmt;

/// fixed-size 256 bit bytearray.
///
/// This type is a thin wrapper around a `[u8;32]` which implements
/// custom hexadecimal serialization, deserialization, and display:
/// 
/// ```
/// # extern crate mimir_common;
/// # fn main() {
/// use mimir_common::types::H256;
///
/// let hex_str = "0x1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8";
/// 
/// let hash: H256 = hex_str.parse().unwrap();
///
/// assert_eq!(format!("{:?}",hash),hex_str);
/// # }
/// ```
///
#[derive(Rand,Default,Copy,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct H256(pub [u8;32]);

// implement basic newtype traits
newtype!(H256,[u8;32],[u8]);

// implement hex array traits
hex_array!(H256,32);


impl fmt::Display for H256 {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self,f)
    }
}


#[cfg(test)]
mod tests {
    use types::H256;
    use rand;

    #[test]
    fn sanity() {
        for _ in 0..16 {
            let hash: H256 = rand::random();
            let hash_str = format!("{:?}",hash);
            assert_eq!(hash,hash_str.parse().unwrap());
        }
    }
}

