//! basic block visitors
//!
use mimir_common::types::{U256,H256};
use mimir_crypto::Address;
use std::collections::HashSet;
use visit::BlockVisitor;


/// context object with knowledge about a specific block.
///
/// When acting as a visitor, `BlockState` will return a bool indicating whether or not the 
/// visited value was "good" (e.g. `true` for validators which are in the known validator
/// set, and `false` for validators which are not).  A value of `true` is returned if a 
/// value about which no information is known is encountered (e.g. if the value of the 
/// `validators` field is `None`).  This behavior is intended to allow `BlockState` to
/// serve as a visitor which rejects information that it knows to be bad, allowing all
/// else to pass.
///
/// ```
/// # 
/// extern crate mimir_proto;
/// use mimir_proto::visit::BlockVisitor;
/// use mimir_proto::visit::BlockState;
/// # fn main() {
///
/// let block_number = "0xdeadbeef".parse().unwrap();
/// 
/// let block_visitor = BlockState::new()
///     .number(block_number);
/// 
/// assert_eq!(block_visitor.get_number(),Some(&block_number));
/// assert_eq!(block_visitor.get_hash(),None);
/// # }
/// ```
///
#[derive(Debug,Clone,Default,PartialEq,Eq)]
pub struct BlockState {
    /// allowable validator set for this block.
    pub validators: Option<HashSet<Address>>,

    /// allowable notary set for this block.
    pub notaries: Option<HashSet<Address>>,

    /// alowable router set for this block.
    pub routers: Option<HashSet<Address>>,
    
    /// block number
    pub number: Option<U256>,

    /// block hash
    pub hash: Option<H256>,
}


impl BlockState {

    /// get new (blank) instance
    pub fn new() -> Self { Default::default() }

    /// configure with a validator set
    pub fn validators(mut self, set: HashSet<Address>) -> Self { self.validators = Some(set); self }

    /// configure with a notary set
    pub fn notaries(mut self, set: HashSet<Address>) -> Self { self.notaries = Some(set); self }

    /// configure with a router set
    pub fn routers(mut self, set: HashSet<Address>) -> Self { self.routers = Some(set); self }

    /// configure with block number
    pub fn number(mut self, val: U256) -> Self { self.number = Some(val); self }

    /// configure with block hash
    pub fn hash(mut self, val: H256) -> Self { self.hash = Some(val); self }
}


impl BlockVisitor for BlockState {

    /// output generated upon visitation
    type Out = bool;

    /// visit a validator address
    fn visit_validator(&self, ident: &Address) -> Self::Out {
        match self.validators {
            Some(ref set) => set.contains(ident),
            None => true,
        }
    }

    /// visit a router address
    fn visit_router(&self, ident: &Address) -> Self::Out {
        match self.routers {
            Some(ref set) => set.contains(ident),
            None => true,
        }
    }

    /// visit a notary address
    fn visit_notary(&self, ident: &Address) -> Self::Out {
        match self.routers {
            Some(ref set) => set.contains(ident),
            None => true,
        }
    }

    /// visit a block number
    fn visit_number(&self, number: &U256) -> Self::Out {
        match self.number {
            Some(ref val) => val == number,
            None => true,
        }
    }

    /// visit a block hash
    fn visit_hash(&self, hash: &H256) -> Self::Out {
        match self.hash {
            Some(ref val) => val == hash,
            None => true,
        }
    }

    /// get reference to target block number
    fn get_number(&self) -> Option<&U256> {
        self.number.as_ref()
    }

    /// get reference to target block hash.
    fn get_hash(&self) -> Option<&H256> {
        self.hash.as_ref()
    }
}
