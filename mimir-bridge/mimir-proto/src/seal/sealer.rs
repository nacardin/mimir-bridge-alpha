//! abstraction representing a type capable of seal generation.
//!
use mimir_types::{Signature,Address};
use message::{Message,DESTS};
use mimir_crypto::Signer;
use message::cert::{
    Oracle,
    Notary,
    Route,
    Verify,
};
use seal;


/// signed cert generator
///
/// signing is the final step of message processing, thereby
/// 'sealing' the message and comitting to its contents (or, at the
/// very least, the contribution made by the signer).
///
/// This trait should not be directly implemented.  Any type which implements
/// `Signer<Sig=Signature,Msg=[u8;32],Pub=Address>` automatically implements this
/// trait via blanket impl.
///
pub trait Sealer {


    /// sign supplied message
    fn sign(&self, msg: &[u8;32]) -> Signature;

    /// get copy of assicated address
    fn address(&self) -> Address;

    /// seal message with an oracle cert
    fn seal_oracle(&self, message: &Message) -> Oracle;

    /// seal message with a notary cert
    fn seal_notary(&self, message: &Message) -> Notary;

    /// seal message with a route cert
    fn seal_route(&self, message: &Message, route: [Address;DESTS]) -> Route;

    /// seal message with a verify cert
    fn seal_verify(&self, message: &Message, flag: u8) -> Verify;
}


impl<T> Sealer for T where T: Signer<Sig=Signature,Msg=[u8;32],Pub=Address> {
 
    /// sign supplied message
    fn sign(&self, msg: &[u8;32]) -> Signature { <Self as Signer>::sign(self,msg) }

    /// get copy of assicated address
    fn address(&self) -> Address { <Self as Signer>::identify(self) }

    /// seal message with an oracle cert
    fn seal_oracle(&self, message: &Message) -> Oracle { seal::oracle(self,message) }

    /// seal message with a notary cert
    fn seal_notary(&self, message: &Message) -> Notary { seal::notary(self,message) }

    /// seal message with a route cert
    fn seal_route(&self, message: &Message, route: [Address;DESTS]) -> Route { seal::route(self,message,route) }

    /// seal message with a verify cert
    fn seal_verify(&self, message: &Message, flag: u8) -> Verify { seal::verify(self,message,flag) }
}

