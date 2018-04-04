//! utilities related to the signed subset of certs
//!
use mimir_types::{Signature,Address};
use mimir_crypto::Signer;
use visit::{self,ByteVisitor};
use message::cert::{Oracle,Notary,Verify,Route};
use message::{Message,DESTS};


/// generate oracle cert for message
#[inline]
pub fn oracle<S>(signer: S, message: &Message) -> Oracle where S: Signer<Msg=[u8;32],Sig=Signature> {
    let mut visitor = ByteVisitor::default();
    visit::apply(&mut visitor,message);
    let bytes = visitor.finish();
    raw::oracle(signer,&bytes)
}


/// generate notary cert for message
#[inline]
pub fn notary<S>(signer: S, message: &Message) -> Notary where S: Signer<Msg=[u8;32],Sig=Signature> {
    let mut visitor = ByteVisitor::default();
    visit::apply(&mut visitor,message);
    let bytes = visitor.finish();
    raw::notary(signer,&bytes)
}


/// generate verify cert for message
#[inline]
pub fn verify<S>(signer: S, message: &Message, val: u8) -> Verify where S: Signer<Msg=[u8;32],Sig=Signature> {
    let mut visitor = ByteVisitor::default();
    visit::apply(&mut visitor,message);
    let bytes = visitor.finish();
    raw::verify(signer,&bytes,val)
}


/// generate route cert for message
#[inline]
pub fn route<S>(signer: S, message: &Message, val: [Address;DESTS]) -> Route where S: Signer<Msg=[u8;32],Sig=Signature> {
    let mut visitor = ByteVisitor::default();
    visit::apply(&mut visitor,message);
    let bytes = visitor.finish();
    raw::route(signer,&bytes,val)
}



/// utilities for generating certs of raw bytes
///
pub mod raw {
    use mimir_types::{Signature,Address};
    use mimir_crypto::{Keccak256,Signer};
    use message::cert::{Verify,Oracle,Notary,Route};
    use message::DESTS;
    
    
    /// required flag for oracle cert
    pub(crate) const ORACLE_FLAG: u8 = 0x00;


    /// build oracle cert of raw message bytes
    #[inline]
    pub fn oracle<S>(signer: S, bytes: &[u8]) -> Oracle where S: Signer<Msg=[u8;32],Sig=Signature> {
        verify(signer,bytes,ORACLE_FLAG)
    }


    /// build notary cert of raw message bytes
    #[inline]
    pub fn notary<S>(signer: S, bytes: &[u8]) -> Notary where S: Signer<Msg=[u8;32],Sig=Signature> {
        let hash = Keccak256::hash(bytes);
        let sig = signer.sign(&hash);
        Notary { sig }
    }


    /// build verify cert of raw message bytes
    #[inline]
    pub fn verify<S>(signer: S, bytes: &[u8], val: u8) -> Verify where S: Signer<Msg=[u8;32],Sig=Signature> {
        let mut hasher = Keccak256::default();
        hasher.absorb(bytes);
        hasher.absorb(&[val]);
        let hash = hasher.finish();
        let sig = signer.sign(&hash);
        Verify { sig, val }
    }


    /// build route cert
    #[inline]
    pub fn route<S>(signer: S, bytes: &[u8], val: [Address;DESTS]) -> Route where S: Signer<Msg=[u8;32],Sig=Signature> {
        let mut hasher = Keccak256::default();
        hasher.absorb(bytes);
        for address in val.iter() {
            hasher.absorb(address.as_ref())
        }
        let hash = hasher.finish();
        let sig = signer.sign(&hash);
        Route { sig, val }
    }
}


