use mimir_common::types::{U256,H256};
use mimir_crypto::Address;
use std::sync::Arc;
use std::rc::Rc;


/// trait representing a visitor with knowledge or interest 
/// in a *specific* block.
///
pub trait BlockVisitor {

    /// output generated upon visitation
    type Out;

    /// visit a validator address
    fn visit_validator(&self, ident: &Address) -> Self::Out;

    /// visit a router address
    fn visit_router(&self, ident: &Address) -> Self::Out;

    /// visit a notary address
    fn visit_notary(&self, ident: &Address) -> Self::Out;

    /// visit a block number
    fn visit_number(&self, number: &U256) -> Self::Out;

    /// visit a block hash
    fn visit_hash(&self, hash: &H256) -> Self::Out;

    /// get reference to target block number if exists
    fn get_number(&self) -> Option<&U256>;

    /// get reference to target block hash if exists
    fn get_hash(&self) -> Option<&H256>;
}


impl<'a,T> BlockVisitor for &'a T where T: BlockVisitor + ?Sized {

    type Out = <T as BlockVisitor>::Out;

    fn visit_validator(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_validator(self,ident) }

    fn visit_router(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_router(self,ident) }

    fn visit_notary(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_notary(self,ident) }

    fn visit_number(&self, number: &U256) -> Self::Out { <T as BlockVisitor>::visit_number(self,number) }

    fn visit_hash(&self, hash: &H256) -> Self::Out { <T as BlockVisitor>::visit_hash(self,hash) }

    fn get_number(&self) -> Option<&U256> { <T as BlockVisitor>::get_number(self) }

    fn get_hash(&self) -> Option<&H256> { <T as BlockVisitor>::get_hash(self) }
}


impl<T> BlockVisitor for Box<T> where T: BlockVisitor + ?Sized {

    type Out = <T as BlockVisitor>::Out;

    fn visit_validator(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_validator(self,ident) }

    fn visit_router(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_router(self,ident) }

    fn visit_notary(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_notary(self,ident) }

    fn visit_number(&self, number: &U256) -> Self::Out { <T as BlockVisitor>::visit_number(self,number) }

    fn visit_hash(&self, hash: &H256) -> Self::Out { <T as BlockVisitor>::visit_hash(self,hash) }

    fn get_number(&self) -> Option<&U256> { <T as BlockVisitor>::get_number(self) }

    fn get_hash(&self) -> Option<&H256> { <T as BlockVisitor>::get_hash(self) }
}


impl<T> BlockVisitor for Arc<T> where T: BlockVisitor + ?Sized {

    type Out = <T as BlockVisitor>::Out;

    fn visit_validator(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_validator(self,ident) }

    fn visit_router(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_router(self,ident) }

    fn visit_notary(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_notary(self,ident) }

    fn visit_number(&self, number: &U256) -> Self::Out { <T as BlockVisitor>::visit_number(self,number) }

    fn visit_hash(&self, hash: &H256) -> Self::Out { <T as BlockVisitor>::visit_hash(self,hash) }

    fn get_number(&self) -> Option<&U256> { <T as BlockVisitor>::get_number(self) }

    fn get_hash(&self) -> Option<&H256> { <T as BlockVisitor>::get_hash(self) }
}


impl<T> BlockVisitor for Rc<T> where T: BlockVisitor + ?Sized {

    type Out = <T as BlockVisitor>::Out;

    fn visit_validator(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_validator(self,ident) }

    fn visit_router(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_router(self,ident) }

    fn visit_notary(&self, ident: &Address) -> Self::Out { <T as BlockVisitor>::visit_notary(self,ident) }

    fn visit_number(&self, number: &U256) -> Self::Out { <T as BlockVisitor>::visit_number(self,number) }

    fn visit_hash(&self, hash: &H256) -> Self::Out { <T as BlockVisitor>::visit_hash(self,hash) }

    fn get_number(&self) -> Option<&U256> { <T as BlockVisitor>::get_number(self) }

    fn get_hash(&self) -> Option<&H256> { <T as BlockVisitor>::get_hash(self) }
}

