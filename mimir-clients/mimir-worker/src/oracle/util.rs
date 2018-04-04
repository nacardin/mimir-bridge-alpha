use mimir_node::rpc::SimpleRecord;
use mimir_proto::message::{Message,Payload};
use mimir_proto::visit::BlockState;
use mimir_types::{U256,H256,Address};
use oracle::simple::SimpleRequest;
use serde_json::{self,Error};
use std::sync::Arc;


/// placeholder deserialization target for `on_block` op.
///
#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct SimpleBlock {
    bhash: H256,
    bnum: U256,
}


impl Into<Arc<BlockState>> for SimpleBlock {

    fn into(self) -> Arc<BlockState> {
        BlockState::default()
            .number(self.bnum)
            .hash(self.bhash)
            .into()
    }
}


/// deserialization target for oracle operations
#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(tag = "op", content = "msg")]
pub enum OracleOp { 
    #[serde(rename = "query")]
    Query(SimpleRequest),

    #[serde(rename = "on_block")]
    Block(SimpleBlock),
}


/// helper for procedural message construction.
#[derive(Debug,Clone)]
pub struct MessageBuilder {
    address: Address,
    blind: H256,
    number: Option<U256>,
    hash: Option<H256>,
}


impl MessageBuilder {

    pub fn new(address: Address, blind: H256) -> Self {
        let (number,hash) = Default::default();
        MessageBuilder { address, blind, number, hash }
    }

    pub fn number(mut self, number: U256) -> Self { self.number = Some(number); self }

    pub fn hash(mut self, hash: H256) -> Self { self.hash = Some(hash); self }

    pub fn finish(self, record: SimpleRecord) -> Result<Message,Error> {
        let record = serde_json::to_string(&record)?;
        let MessageBuilder { address, blind, number, hash } = self;
        let number = number.unwrap_or_else(Default::default);
        let hash = hash.unwrap_or_else(Default::default);
        let payload = Payload { record, address, number, hash };
        let mut message = Message::new(payload);
        message.blind.push(blind);
        Ok(message)
    }
}

