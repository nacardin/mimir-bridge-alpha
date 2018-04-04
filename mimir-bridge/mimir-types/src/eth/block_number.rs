use serde::de::{Deserialize,Deserializer};
use serde::ser::{Serialize,Serializer};
use mimir_util::types::Either;



/// ethereum block number parameter.
///
/// this type models the block number parameter used by many ethereum jsonrpc
/// methods, and implements custom serialization/deserialization to reflect
/// the expected json representation:
///
/// ```
/// # extern crate mimir_types; 
/// # fn main() {
/// extern crate serde_json;
/// use mimir_types::eth::BlockNumber;
///
/// let blocks_json = r#"["latest",12345]"#;
///
/// let blocks: Vec<BlockNumber> = serde_json::from_str(blocks_json).unwrap();
/// 
/// assert_eq!(&blocks,&[BlockNumber::Latest,BlockNumber::Number(12345)]);
///
/// assert_eq!(blocks_json,serde_json::to_string(&blocks).unwrap());
/// # }
/// ```
///
#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub enum BlockNumber {
    /// most recently mined block
    Latest,
    /// earliest available block
    Earliest,
    /// bending block
    Pending,
    /// specific block number
    Number(u64)
}


/// implementation detail for named block ser/de.
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
enum NamedParam {
    Latest,
    Earliest,
    Pending
}


impl Default for BlockNumber {

    fn default() -> Self { BlockNumber::Latest } 
}


impl Serialize for BlockNumber {

    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok,S::Error> {
        let target: Either<NamedParam,u64> = match *self {
            BlockNumber::Latest => Either::A(NamedParam::Latest),
            BlockNumber::Earliest => Either::A(NamedParam::Earliest),
            BlockNumber::Pending => Either::A(NamedParam::Pending),
            BlockNumber::Number(num) => Either::B(num),
        };
        target.serialize(serializer)
    }
}


impl<'de> Deserialize<'de> for BlockNumber {

    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self,D::Error> {
        let target: Either<NamedParam,u64> = Deserialize::deserialize(deserializer)?;
        let block_number = match target {
            Either::A(NamedParam::Latest) => BlockNumber::Latest,
            Either::A(NamedParam::Earliest) => BlockNumber::Earliest,
            Either::A(NamedParam::Pending) => BlockNumber::Pending,
            Either::B(num) => BlockNumber::Number(num),
        };
        Ok(block_number)
    }
}

