use mimir_util::hex::{self,ParseHexError};
use mimir_util::types::Either;
use serde::de::{self,Deserialize,Deserializer};
use serde::ser::{Serialize,Serializer};
use rand::{Rand,Rng};
use std::str::FromStr;
use std::fmt;


/// variable size buffer of bytes.
///
/// This type is a thin wrapper around a `Vec<u8>` which implements
/// custom hexadecimal serialization, deserialization, and display:
/// 
/// ```
/// # extern crate mimir_types;
/// # fn main() {
/// extern crate serde_json;
/// use mimir_types::primitive::Bytes;
/// use std::collections::HashMap;
///
/// let bytes: Bytes = "0xdeadbeef".parse().unwrap();
/// 
/// let mut map = HashMap::new();
/// 
/// map.insert("some-hex".to_string(),bytes);
/// 
/// let json = serde_json::to_string(&map).unwrap();
/// 
/// assert_eq!(&json,r#"{"some-hex":"0xdeadbeef"}"#);
/// # }
/// ```
///
#[derive(Default,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct Bytes(pub Vec<u8>);

newtype!(Bytes,Vec<u8>,Vec<u8>);


impl FromStr for Bytes {

    type Err = ParseHexError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let hex_bytes: &[u8] = s.trim().trim_left_matches("0x").as_ref();
        if hex_bytes.len() % 2 == 0 {
            let mut buff = vec![0u8;hex_bytes.len() / 2];
            hex::from(hex_bytes,&mut buff)?;
            Ok(Bytes(buff))
        } else {
            Err(ParseHexError::InvalidSize)
        }
    }
}


impl Serialize for Bytes {

    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok,S::Error> {
        let mut buff = vec![0u8;2 + self.len() * 2];
        buff[..2].copy_from_slice(b"0x");
        hex::into(self.as_ref(),&mut buff[2..]);
        let hex_str = ::std::str::from_utf8(&buff)
            .expect("always valid UTF-8 bytes");
        serializer.serialize_str(hex_str)
    }
}


impl<'de> Deserialize<'de> for Bytes {

    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self,D::Error> {
        let target: Either<&str,String> = Deserialize::deserialize(deserializer)?;
        Bytes::from_str(target.as_ref()).map_err(de::Error::custom)
    }
}


impl Rand for Bytes {

    fn rand<R: Rng>(rng: &mut R) -> Self { 
        let len = rng.gen::<usize>() % 128;
        let mut buf = vec![0u8;len];
        for byte in buf.iter_mut() {
            *byte = rng.gen();
        }
        Bytes(buf)
    }
}


impl fmt::Debug for Bytes {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff = vec![0u8;self.len() * 2];
        let hex_str = hex::as_str(self.as_ref(),&mut buff);
        f.write_str("0x")?;
        f.write_str(hex_str)
    }
}

impl fmt::Display for Bytes {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self,f)
    }
}


#[cfg(test)]
mod tests {
    use primitive::Bytes;
    use rand;

    #[test]
    fn debug() {
        let mut bytes: Bytes = vec![0u8;5].into();
        bytes.copy_from_slice(b"hello");
        assert_eq!(format!("{:?}",bytes),"0x68656c6c6f");
    }

    fn assert_slice(_: &[u8]) { }
    fn assert_slice_mut(_: &mut [u8]) { }
    fn assert_vec(_: &Vec<u8>) { }
    fn assert_vec_mut(_: &mut Vec<u8>) { }

    #[test]
    fn coerce() {
        let mut bytes = Bytes::default();
        assert_slice(&bytes);
        assert_slice_mut(&mut bytes);
        assert_vec(&bytes);
        assert_vec_mut(&mut bytes);
    }

    #[test]
    fn random() {
        let bytes: Bytes = rand::random();
        let byte_str = format!("{:?}",bytes);
        assert_eq!(bytes,byte_str.parse().unwrap());
    }
}

