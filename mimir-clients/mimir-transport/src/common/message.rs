use mimir_types::Address;
use common::{
    ParseError,
    Channel,
    MSG,
};
use std::str::FromStr;
use std::fmt;


/// message value
///
/// ```
/// #
/// extern crate mimir_transport;
/// use mimir_transport::common::Message;
/// # fn main() {
///
/// let buf = "QUERY 0x9469d56752abf5120c568FF2F94175841B829ee7 some-payload".to_string();
///
/// let msg = Message::from_string(buf).unwrap();
/// 
/// assert_eq!(&msg.dest_channel().to_string(),"oracle::work");
///
/// assert_eq!(msg.msg_payload(),"some-payload");
/// # }
/// ```
/// 
#[derive(Debug,Clone)]
pub struct Message {
    metadata: MessageData,
    inner: String,
}


impl Message {

    /// generate raw message from unprocessed string
    pub fn from_string(inner: String) -> Result<Self,ParseError> {
        let metadata = inner.parse()?;
        Ok(Self { metadata, inner })
    }

    /// convert to inner string buffer
    pub fn into_inner(self) -> String { self.inner }

    /// get message variant
    pub fn msg_variant(&self) -> MSG { self.metadata.variant }

    /// get address of sender
    pub fn source_address(&self) -> &Address { &self.metadata.source }

    /// get address of destination (if any)
    pub fn dest_address(&self) -> Option<&Address> { self.metadata.dest.as_ref() }

    /// get destination channel of message
    pub fn dest_channel(&self) -> Channel {
        let role = self.msg_variant().consumer();
        if let Some(&addr) = self.dest_address() {
            Channel::Direct { addr, role }
        } else {
            Channel::Shared { role }
        }
    }

    /// get length of payload in bytes
    pub fn payload_size(&self) -> usize { self.metadata.size }

    /// get message payload subslice
    pub fn msg_payload(&self) -> &str {
        debug_assert!(self.inner.len() >= self.payload_size());
        let index = self.inner.len().saturating_sub(self.payload_size());
        &self.inner[index..]
    }
}


impl Into<String> for Message {

    fn into(self) -> String { self.into_inner() }
}


impl fmt::Display for Message {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.inner)
    }
}


/// raw message metadata
///
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct MessageData {
    /// message variant flag
    variant: MSG,
    /// source address
    source: Address,
    /// destination address (if any)
    dest: Option<Address>,
    /// size of message payload
    size: usize,
}


impl FromStr for MessageData {

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let mut split = s.splitn(3,' ');
        if let (Some(msg),Some(src),Some(tail)) = (split.next(),split.next(),split.next()) {
            let variant: MSG = msg.parse().map_err(|_|ParseError::BadMsgVariant)?;
            let source: Address = src.parse().map_err(|_|ParseError::BadAddress)?;
            let (dest,size) = if variant.directed() {
                let mut split = tail.splitn(2,' ');
                if let (Some(dest),Some(tail)) = (split.next(),split.next()) {
                    let address = dest.parse().map_err(|_|ParseError::BadAddress)?;
                    (Some(address),tail.len())
                } else {
                    return Err(ParseError::MissingVal);
                }
            } else {
                (None,tail.len())
            };
            Ok(MessageData { variant, source, dest, size })
        } else {
            Err(ParseError::MissingVal) 
        }
    }
}

