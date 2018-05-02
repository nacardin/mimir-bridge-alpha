use mimir_types::{Signature,Address,U256};
use mimir_proto::seal::Sealer;
use mimir_crypto::secp256k1::{
    Verifier,
    Error,
};
use mimir_crypto::Keccak256;
use mimir_util::unix_time;
use common::Role;


/// payload of the `AUTH` operation.
///
/// This datastructure represents an argument of identity.
/// The sender signs their claimed address, role, and the
/// current timestamp (to help combat replay attacks).
///
/// Because of the included timestamp, this structure should
/// be lazily initialized immediately prior to broadcast.
///
#[derive(Debug,Copy,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct Auth {
    pub addr: Address,
    pub role: Role,
    pub time: U256,
    pub seal: Signature,
}


impl Auth {

    pub fn new<S: Sealer>(sealer: S, role: Role) -> Self {
        let addr = sealer.address();
        let time = unix_time().into();
        let hash = hash_elems(&addr,role,time);
        let seal = sealer.sign(&hash);
        Auth { addr, role, time, seal }
    }

    pub fn check_seal(&self) -> Result<(),Error> {
        let verifier = Default::default();
        self.check_with(&verifier)
    }

    pub fn check_with(&self,verifier: &Verifier) -> Result<(),Error> {
        let hash = hash_elems(&self.addr,self.role,self.time);
        let address = verifier.ecrecover(&hash,&self.seal)?;
        if self.addr == address {
            Ok(())
        } else {
            Err(Error::AddressMismatch)
        }
    }
}


fn hash_elems(addr: &Address, role: Role, time: U256) -> [u8;32] {
    let role_bytes = role.as_ref().as_bytes();
    let mut hasher = Keccak256::default();
    hasher.absorb(&addr);
    hasher.absorb(role_bytes);
    hasher.absorb(&time);
    hasher.finish()
}

