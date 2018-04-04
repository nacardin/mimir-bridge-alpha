use mimir_types::{Signature,Address};
use mimir_proto::seal::Sealer;
use mimir_crypto::secp256k1::{
    Verifier,
    Error,
};
use mimir_crypto::Keccak256;
use mimir_util::unix_time;
use common::{
    CMD,
    Role,
};
use std::mem;


/// signed command.
///
/// all commands specify a target address/role, a time range in
/// which they are valid, and are signed by their originator.
///
/// the actual name of the command determines how its parameters
/// are utilized (e.g.; the address in `AUTH` is taken to be the
/// address of the entity requesting authorization, whereas the
/// address in `KICK` is the address of the entity to be kicked).
///
#[derive(Debug,Copy,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct Command {
    pub flag: CMD,
    pub addr: Address,
    pub role: Role,
    pub from: u64,
    pub upto: u64,
    pub seal: Signature,
}


impl Command { 

    /// build  a new command w/ specified sealer & params.
    #[inline]
    pub fn new<S: Sealer>(sealer: S, flag: CMD, role: Role, from: u64, upto: u64) -> Self {
        let addr = sealer.address();
        let hash = hash_elems(flag,&addr,role,from,upto);
        let seal = sealer.sign(&hash);
        Command { flag, addr, role, from, upto, seal }
    }

    /// build a new command valid for some duration starting now.
    #[inline]
    fn with_dur<S: Sealer>(sealer: S, flag: CMD, role: Role, dur: u64) -> Self {
        let from = unix_time();
        let upto = from + dur;
        Self::new(sealer,flag,role,from,upto)
    }

    /// construct `KICK` operation.
    #[inline]
    pub fn kick<S: Sealer>(sealer: S, role: Role) -> Self {
        Self::with_dur(sealer,CMD::KICK,role,3600)
    }

    /// construct `AUTH` operation.
    #[inline]
    pub fn auth<S: Sealer>(sealer: S, role: Role) -> Self {
        Self::with_dur(sealer,CMD::AUTH,role,20)
    }

    /// construct `CONN` operation.
    #[inline]
    pub fn conn<S: Sealer>(sealer: S, role: Role) -> Self {
        Self::with_dur(sealer,CMD::CONN,role,512)
    }

    /// construct `WORK` operation.
    #[inline]
    pub fn work<S: Sealer>(sealer: S, role: Role) -> Self {
        Self::with_dur(sealer,CMD::WORK,role,512)
    }

    /// attempt to recover signer address.
    #[inline]
    pub fn ecrecover(&self) -> Result<Address,Error> {
        let verifier = Default::default();
        self.recover_with(&verifier)
    }

    /// attempt to recover signer address with specified `Verifier`.
    #[inline]
    pub fn recover_with(&self, verifier: &Verifier) -> Result<Address,Error> {
        let hash = hash_elems(self.flag,&self.addr,self.role,self.from,self.upto);
        verifier.ecrecover(&hash,&self.seal)
    }
}


#[inline]
fn hash_elems(flag: CMD, addr: &Address, role: Role, from: u64, upto: u64) -> [u8;32] {
    let from_bytes: [u8;8] = unsafe { mem::transmute(from.to_be()) };
    let upto_bytes: [u8;8] = unsafe { mem::transmute(upto.to_be()) };
    let role_bytes = role.as_ref().as_bytes();
    let flag_bytes = flag.as_ref().as_bytes();
    let mut hasher = Keccak256::default();
    hasher.absorb(flag_bytes);
    hasher.absorb(&addr);
    hasher.absorb(role_bytes);
    hasher.absorb(&from_bytes);
    hasher.absorb(&upto_bytes);
    hasher.finish()
}

