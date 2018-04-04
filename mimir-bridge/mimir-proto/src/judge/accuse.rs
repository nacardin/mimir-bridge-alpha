//! types & utils related to accusation.
//!
use visit::{self,ByteVisitor,MessageVisitor};
use mimir_crypto::{Signer,Keccak256};
use mimir_types::Signature;
use message::Message;
use message::cert::{
    Oracle,
    Notary,
    Route,
    Verify
};


// accusation flag.
const ACCFLAG: u8 = 0x01;


/// enum representing a potential accusative action. 
///
#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Accuse {
    /// accuse an oracle cert.
    Oracle {
        /// preceeding message state.
        inner: Message,
        
        /// cert of accused.
        cert: Oracle,
        
        /// outer notary cert.
        seal: Option<Notary>
    },

    /// accuse a verify cert.
    Verify {
        /// preceeding message state.
        inner: Message,
        
        /// cert of accused.
        cert: Verify
    },
    
    /// accuse a route cert.
    Route {
        /// preceeding message state.
        inner: Message,
        
        /// cert of accused.
        cert: Route
    }
}


/// a signed accusation, ready for compilation into a contract call.
pub struct Accusation {
    /// payload offset pointer
    pub payload: usize,

    /// accused cert offset pointer
    pub accused: usize,

    /// accuser cert offset pointer
    pub accuser: usize,

    /// accusation bytes
    pub bytes: Vec<u8>
}


/// generate an accusation against an oracle cert
///
pub fn oracle<S>(signer: S, inner: &Message, cert: &Oracle, seal: &Notary) -> Accusation where S: Signer<Msg=[u8;32],Sig=Signature> {
    // get basic accusation info
    let (payload,accused,mut visitor) = begin_accusation(inner);
    // visit cert of accused
    visitor.visit_oracle(&cert);
    // visit notary seal
    visitor.visit_notary(&seal);
    // pass off to finalizer
    finish_accusation(signer,payload,accused,visitor)
}



/// generate an accusation against a verify cert
///
pub fn verify<S>(signer: S, inner: &Message, cert: &Verify) -> Accusation where S: Signer<Msg=[u8;32],Sig=Signature> {
    // get basic accusation info
    let (payload,accused,mut visitor) = begin_accusation(inner);
    // visit cert of accused
    visitor.visit_verify(&cert);
    // pass off to finalizer
    finish_accusation(signer,payload,accused,visitor)
}



/// generate an accusation against a route cert
///
pub fn route<S>(signer: S, inner: &Message, cert: &Route) -> Accusation where S: Signer<Msg=[u8;32],Sig=Signature> {
    // get basic accusation info
    let (payload,accused,mut visitor) = begin_accusation(inner);
    // visit cert of accused
    visitor.visit_route(&cert);
    // pass off to finalizer
    finish_accusation(signer,payload,accused,visitor)
}



/// initial logic common to all accusation builders
///
/// returned values are payload offset pointer, accused cert
/// offset pointer, and partially seeded visitor.
fn begin_accusation(inner: &Message) -> (usize,usize,ByteVisitor) {
    // calculate payload offset pointer
    let payload = inner.payload.record.as_bytes().len();
    // initialize visitor for message byte collection
    let mut visitor = ByteVisitor::default();
    // apply visitor to message
    visit::apply(&mut visitor,inner);
    // calculate accused cert offset pointer
    let accused = visitor.as_bytes().len();
    // pass back to caller for completion
    (payload,accused,visitor)
}



/// finalization logic common to all accusation builders
fn finish_accusation<S>(signer: S, payload: usize, accused: usize, visitor: ByteVisitor) -> Accusation 
        where S: Signer<Msg=[u8;32],Sig=Signature> {
    // unwrap byte visitor
    let mut bytes = visitor.finish();
    // calculate accuser cert offset
    let accuser = bytes.len();
    // append accusation flag to message bytes
    bytes.push(ACCFLAG);
    // hash message bytes + flag
    let hash = Keccak256::hash(&bytes);
    // generate accuser signature
    let sig = signer.sign(&hash);
    // append accuser signature to message bytes
    bytes.extend_from_slice(sig.as_ref());
    // return completed accusation object
    Accusation { payload, accused, accuser, bytes }
}

