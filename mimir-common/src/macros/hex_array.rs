//! `hex_array` macro submodule.
//!


/// implement hex ser/de for byte-array newtypes.
/// 
/// implements `Serialize`, `Deserialize`, `FromStr`, and
/// `Debug` for byte-array newtypes. 
///
/// ```
/// # #[macro_use] extern crate mimir_common;
/// # fn main() {
/// struct Foo([u8;4]);
/// 
/// newtype!(Foo,[u8;4],[u8]);
/// 
/// hex_array!(Foo,4);
/// 
/// let foo: Foo = "0xdeadbeef".parse().unwrap();
///
/// assert_eq!("fmt: 0xdeadbeef",format!("fmt: {:?}",foo));
/// # }
/// ```
///
#[macro_export]
macro_rules! hex_array {
    ($name:ident,$len:expr) => {
        impl ::std::str::FromStr for $name {

            type Err = $crate::util::hex::ParseHexError;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                let hex_bytes: &[u8] = s.trim().trim_left_matches("0x").as_ref();
                if hex_bytes.len() == $len * 2 {
                    let mut buff = [0u8;$len];
                    $crate::util::hex::from(hex_bytes,&mut buff)?;
                    Ok($name(buff))
                } else {
                    Err($crate::util::hex::ParseHexError::InvalidSize)
                }
            }
        }


        impl $crate::serde::Serialize for $name {

            fn serialize<S: $crate::serde::Serializer>(&self, serializer: S) -> Result<S::Ok,S::Error> {
                let mut buff = [0u8;2 + $len * 2];
                buff[..2].copy_from_slice(b"0x");
                $crate::util::hex::into(self.as_ref(),&mut buff[2..]);
                let hex_str = ::std::str::from_utf8(&buff)
                    .expect("always valid UTF-8 bytes");
                serializer.serialize_str(hex_str)
            }
        }


        impl<'de> $crate::serde::Deserialize<'de> for $name {

            fn deserialize<D: $crate::serde::Deserializer<'de>>(deserializer: D) -> Result<Self,D::Error> {
                let target: $crate::types::Either<&str,String> = $crate::serde::Deserialize::deserialize(deserializer)?;
                ::std::str::FromStr::from_str(target.as_ref()).map_err($crate::serde::de::Error::custom)
            }
        }


        impl ::std::fmt::Debug for $name {

            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut buff = [0u8;$len * 2];
                let hex_str = $crate::util::hex::as_str(self.as_ref(),&mut buff);
                f.write_str("0x")?;
                f.write_str(hex_str)
            }
        }
    }
}
