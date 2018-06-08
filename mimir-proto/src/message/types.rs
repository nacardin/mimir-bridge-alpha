//! Datatypes used to represent various message states/components.
//!
//! This module contains the core types necessary to represent messages
//! and message components.  Types in this module must implement all traits
//! necessary for serialization, deserialization, hashing, etc...
//!
use mimir_common::types::{U256,H256};
use mimir_crypto::Address;
use message::cert;


/// generic request value.
/// 
/// the request object encodes some arbitrary query, the address of
/// the requester, and the initial blinding hash.
///
#[derive(Default,Debug,Copy,Clone,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct Request<Q> {
    /// address of requester.
    pub address: Address,

    /// initial blind seed.
    pub blind: H256,

    /// query being made.
    pub query: Q,
}


/// core values of a message.
///
/// The payload object describes an instance of a served query.
/// the `record` field contians the actual rpc call/response record.
/// this datastructure is generated by the oracle and is unchanged
/// by subsequent steps.
///
#[derive(Debug,Clone,Default,PartialEq,Eq,Serialize,Deserialize)]
pub struct Payload {
    /// query record serialized as json. this component is treated 
    /// as a black-box bytearray by most systems (only validators
    /// directly examine query contents).
    pub record: String,
    
    /// address of the requester.
    pub address: Address,
    
    /// current block number during which
    /// the request was resolved.
    pub number: U256,
    
    /// current block hash during which
    /// the request was resolved.
    pub hash: H256
}



/// the message object; generated by the oracle, and extended
/// by subsequent parties (admin,router,validator,etc...).
///
#[derive(Debug,Clone,Default,PartialEq,Eq,Serialize,Deserialize)]
pub struct Message {
    /// core payload of the message.
    pub payload: Payload,
    
    /// certs of message payload.
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub verify: Vec<cert::Verify>,
    
    /// certs of message passthrough.
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notary: Vec<cert::Notary>,
    
    /// certs of message routing.
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub route: Vec<cert::Route>,
    
    /// user-blind seeds/reveals.
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blind: Vec<H256>
}


impl Message {

    /// build new message with empty certs
    pub fn new(payload: Payload) -> Self {
        Message {
            payload: payload,
            verify: Vec::new(),
            notary: Vec::new(),
            route: Vec::new(),
            blind: Vec::new()
        }
    }
}

impl From<Payload> for Message {

    fn from(payload: Payload) -> Self {
        Message::new(payload)
    }
}


