//! visitor traits for abstracting over common functionality.
//!
use message::cert::{Oracle,Notary,Blind,Clear,Route,Verify};
use mimir_types::{U256,H256,Address};
use message::Payload;


mod message_visitor;
mod block_visitor;
mod message;
mod block;
mod cert;
mod util;


pub use self::message_visitor::MessageVisitor;
pub use self::block_visitor::BlockVisitor;
pub use self::message::{
    MessageVoyeur,
    ByteVisitor,
    HashVisitor,
};
pub use self::block::BlockState;
pub use self::cert::{CertVisitor,CertError};
pub use self::util::apply;




/// fake visitor for debugging, or explicitly
/// ignoring the result of visitation.
#[derive(Default,Debug,Clone)]
pub struct EmptyVisitor;


// blockvisitor implementation which returns true on all
// visitations, and zeroes for number/hash.
impl BlockVisitor for EmptyVisitor {

    type Out = bool;
    
    fn visit_validator(&self, _: &Address) -> Self::Out { true }

    fn visit_router(&self, _: &Address) -> Self::Out { true }

    fn visit_notary(&self, _: &Address) -> Self::Out { true }

    fn visit_number(&self, _: &U256) -> Self::Out { true }

    fn visit_hash(&self, _: &H256) -> Self::Out { true }

    fn get_number(&self) -> Option<&U256> { None }

    fn get_hash(&self) -> Option<&H256> { None }
}


impl<'v> MessageVisitor<'v> for EmptyVisitor {

    type Out = ();

    fn visit_payload(&mut self, _: &Payload) -> Self::Out { }

    fn visit_oracle(&mut self, _: &Oracle) -> Self::Out { }

    fn visit_notary(&mut self, _: &Notary) -> Self::Out { }
 
    fn visit_blind(&mut self, _: &Blind) -> Self::Out { }
 
    fn visit_clear(&mut self, _: &Clear) -> Self::Out { }
 
    fn visit_route(&mut self, _: &Route) -> Self::Out { }
 
    fn visit_verify(&mut self, _: &Verify) -> Self::Out { }
}

