use mimir_util::hex::{self,ParseHexError};
use mimir_util::types::Either;
use serde::de::{self,Deserialize,Deserializer};
use serde::ser::{Serialize,Serializer};
use std::str::FromStr;
use std::{fmt,mem};


/// 256 bit unsigned integer. 
///
/// This type is a thin wrapper around a `[u8;32]` which implements
/// custom hexadecimal serialization, deserialization, and display:
/// 
/// ```
/// # extern crate mimir_types;
/// # fn main() {
/// use mimir_types::primitive::U256;
/// 
/// let numeric: u64 = 0x12345;
///
/// let hex_str = "0x12345";
/// 
/// let from_hex: U256 = hex_str.parse().unwrap();
/// 
/// let from_int: U256 = numeric.into();
///
/// assert_eq!(from_hex,from_int);
///
/// assert_eq!(format!("{:?}",from_hex),hex_str);
/// # }
/// ```
///
#[derive(Rand,Default,Copy,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct U256(pub [u8;32]);

newtype!(U256,[u8;32],[u8]);


impl FromStr for U256 {

    type Err = ParseHexError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let hex_bytes: &[u8] = s.trim().trim_left_matches("0x").as_ref();
        match hex_bytes.len() {
            len @ 1...64 => {
                let mut buff = [0u8;32];
                let start = 32 - len / 2;
                if len % 2 == 0 {
                    hex::from(hex_bytes,&mut buff[start..])?;
                } else {
                    hex::from(&[b'0',hex_bytes[0]],&mut buff[(start - 1)..start])?;
                    hex::from(&hex_bytes[1..],&mut buff[start..])?;
                }
                Ok(U256(buff))
            },
            _ => Err(ParseHexError::InvalidSize)
        }
    }
}


impl Serialize for U256 {

    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok,S::Error> {
        if let Some((idx,val)) = self.iter().enumerate().find(|&(_,v)| *v > 0u8) {
            let mut buff = [0u8;66];
            let start_hex = (idx * 2) + 2;
            hex::into(&self[idx..],&mut buff[start_hex..]);
            let start_pfx = if *val < 0x10 { start_hex - 1 } else { start_hex - 2 };
            buff[start_pfx..(start_pfx + 2)].copy_from_slice(b"0x");
            let hex = ::std::str::from_utf8(&buff[start_pfx..])
                .expect("always valid UTF-8");
            serializer.serialize_str(hex)
        } else {
            serializer.serialize_str("0x0")
        }
    }
}


impl<'de> Deserialize<'de> for U256 {

    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self,D::Error> {
        let target: Either<&str,String> = Deserialize::deserialize(deserializer)?;
        U256::from_str(target.as_ref()).map_err(de::Error::custom)
    }
}


impl fmt::Debug for U256 {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.as_ref().iter().any(|val| *val > 0) {
            let mut buff = [0u8;64];
            let hex_str = hex::as_str(self.as_ref(),&mut buff)
                .trim_left_matches("0");
            f.write_str("0x")?;
            f.write_str(hex_str)
        } else {
            f.write_str("0x0")
        }
    }
}


impl fmt::Display for U256 {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self,f)
    }
}


impl From<u16> for U256 {

    fn from(val: u16) -> Self { Self::from(val as u64) }
}


impl From<u32> for U256 {
    
    fn from(val: u32) -> Self { Self::from(val as u64) }
}


impl From<u64> for U256 {

    fn from(val: u64) -> Self {
        let bytes: [u8;8] = unsafe { mem::transmute(val.to_be()) };
        let mut buffer = [0u8;32];
        buffer[24..].copy_from_slice(&bytes);
        U256::from(buffer)
    }
}



#[cfg(test)]
mod tests {
    use primitive::U256;
    use serde_json;
    use rand;

    macro_rules! hex_vector {
        ($( $hex:tt ),+ ) => {
            vec![
                $(
                    (<U256 as From<u64>>::from($hex),stringify!($hex))
                ),+
            ]
        }
    }

    #[test]
    fn targets() {
        let targets = hex_vector![
            0x0,0x1,0x2,0x3,0x4,0x5,0x6,0x7,
            0x8,0x9,0xa,0xb,0xc,0xd,0xe,0xf,
            0x123456789abcdef,0xdeadbeef
        ];
        for &(num,hex) in targets.iter() {
            check_pair(num,hex);
        } 
    }

    #[test]
    fn random() {
        for _ in 0..32 {
            let rand_val: u64 = rand::random();
            let int = U256::from(rand_val);
            let hex = format!("{:#x}",rand_val);
            check_pair(int,&hex);
        }
    }

    fn check_pair(int: U256, hex: &str) {
        let into_hex = format!("{:?}",int);
        let into_int = hex.parse().unwrap();
        assert_eq!(int,into_int,"assert parse hex");
        assert_eq!(hex,into_hex,"assert format hex");
        let json_val = json!(int);
        let json_str = serde_json::to_string(&json_val).unwrap();
        assert_eq!(json_str,format!(r#""{}""#,hex),"serialize json");
        let from_val: U256 = serde_json::from_value(json_val).unwrap();
        let from_str: U256 = serde_json::from_str(&json_str).unwrap();
        assert_eq!(from_val,from_str,"deserialize json");
    }
}

