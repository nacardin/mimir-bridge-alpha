//! 256 bit ecc on the k1 curve
//!
use _secp256k1::{Secp256k1,ContextFlag,Message,RecoverableSignature,RecoveryId};
use _secp256k1::key::{PublicKey,SecretKey};
use rand::{self,Rand,Rng};
use std::borrow::Borrow;
use std::mem;

mod signature;
mod address;
mod public;
mod secret;

pub use self::signature::Signature;
pub use self::address::Address;
pub use self::public::Public;
pub use self::secret::Secret;


// ----------------------------------------------------------------


lazy_static! {
    /// signing/verification context
    static ref SECP256K1: Secp256k1 = {
        let caps = ContextFlag::Full;
        let context = Secp256k1::with_caps(caps);
        context
    };
}


// ----------------------------------------------------------------


/// shortcut for generating test keys.
pub fn keygen() -> (Public,Secret) {
    let signer: Signer = rand::random();
    let public = signer.public();
    let secret = signer.secret();
    (public,secret)
}


/// address recovery convenience function
pub fn ecrecover(msg: &[u8;32], sig: &Signature) -> Result<Address,Error> {
    let verifier = Verifier::default();
    verifier.ecrecover(msg,sig)
}


// ----------------------------------------------------------------


/// context for verification operations
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Verifier {
    ctx: &'static Secp256k1
}


impl Verifier {

    /// attempt to recover address from msg/sig.
    pub fn ecrecover(&self, msg: &[u8;32], sig: &Signature) -> Result<Address,Error> {
        let public = self.recover(msg,sig)?;
        Ok(Address::from(&public))
    }

    /// attempt to recover public key from msg/sig.
    pub fn recover(&self, msg: &[u8;32], sig: &Signature) -> Result<Public,Error> {
        let recoverable = self.recoverable(sig)?;
        let message: &Message = unsafe { mem::transmute(msg) };
        if let Ok(pk) = self.ctx.recover(message,&recoverable) {
            Ok(Public::from(&pk))
        } else {
            Err(Error::RecoveryFailed)
        }
    }

    /// attempt to convert signature to `RecoverableSignature`.
    fn recoverable(&self, sig: &Signature) -> Result<RecoverableSignature,Error> {
        let v = match sig.get_v() {
            v @ 0...26 => v,
            v => v - 27
        };
        if let Ok(rid) = RecoveryId::from_i32(v as i32) {
            if let Ok(rec) = RecoverableSignature::from_compact(self.ctx,&sig[0..64],rid) {
                Ok(rec)
            } else {
                Err(Error::BadSignatureFmt)
            }
        } else {
            Err(Error::BadRecoveryByte)
        } 
    }
}


impl Default for Verifier {

    fn default() -> Self {
        let ctx = &SECP256K1;
        Verifier { ctx }
    }
}


// ----------------------------------------------------------------


/// simple persistent signing instance. 
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Signer {
    key: SecretKey,
    ctx: &'static Secp256k1
}


impl Signer {

    /// attempt to build a signer from some secret
    pub fn new<S>(secret: S) -> Result<Self,Error> where S: Borrow<Secret> {
        let ctx = &SECP256K1;
        Self::with_context(ctx,secret)
    }

    /// attempt to build a signer from specified secret and context
    fn with_context<S>(ctx: &'static Secp256k1, secret: S) -> Result<Self,Error> where S: Borrow<Secret> {
        let secret = secret.borrow();
        if let Ok(key) = SecretKey::from_slice(ctx,secret.as_ref()) {
            Ok(Signer { key, ctx })
        } else {
            Err(Error::InvalidSecret)
        }
    }

    /// sign a message
    #[inline]
    pub fn sign(&self, msg: &[u8;32]) -> Signature {
        let msg: &Message = unsafe { mem::transmute(msg) };
        let recoverable = self.ctx.sign_recoverable(msg,&self.key)
            .expect("context flags always set to `full`");
        let (id,body) = recoverable.serialize_compact(self.ctx);
        let mut buff = [0u8;65];
        buff[0..64].copy_from_slice(&body[0..64]);
        buff[64] = id.to_i32() as u8;
        buff[64] += 27;
        Signature::from(buff)
    }

    /// derive secret upon which signer was based
    #[inline]
    pub fn secret(&self) -> Secret {
        (&self.key).into()
    }

    /// derive public key
    #[inline]
    pub fn public(&self) -> Public {
        let key = PublicKey::from_secret_key(self.ctx,&self.key)
            .expect("context flags always set to `full`");
        Public::from(&key)
    }

    /// derive public address
    #[inline]
    pub fn address(&self) -> Address {
        let public = self.public();
        Address::from(&public)
    }
}



impl ::Signer for Signer {

    type Msg = [u8;32];

    type Sig = Signature;

    type Pub = Address;

    #[inline]
    fn sign(&self, msg: &Self::Msg) -> Self::Sig { self.sign(msg) }

    #[inline]
    fn identify(&self) -> Self::Pub { self.address() }
}



impl Rand for Signer {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let ctx = &SECP256K1;
        loop {
            let buff: [u8;32] = rng.gen();
            let secret = Secret(buff);
            // if signer instantiates ok, secret is valid
            if let Ok(signer) = Signer::with_context(ctx,secret) {
                return signer;
            }
        }
    }
}



// ----------------------------------------------------------------

simple_error!(
    Error,
    BadRecoveryByte => "recovery byte invalid",
    BadSignatureFmt => "format invalid",
    RecoveryFailed => "recovery operation failed",
    InvalidSecret => "malformed secret key",
    AddressMismatch => "recovered unexpected address",
);


#[cfg(test)]
mod tests {
    use secp256k1::{self,Address,Secret,Signer};
    use keccak256::Keccak256;

    /// parity dev chain default account address
    const ADDRESS: Address = Address([
        0x00, 0xa3, 0x29, 0xc0, 0x64, 0x87, 0x69, 0xa7, 0x3a, 0xfa, 
        0xc7, 0xf9, 0x38, 0x1e, 0x08, 0xfb, 0x43, 0xdb, 0xea, 0x72
    ]);

    /// parity dev chain default account secret
    const SECRET: Secret = Secret([
        0x4d, 0x5d, 0xb4, 0x10, 0x7d, 0x23, 0x7d, 0xf6,
        0xa3, 0xd5, 0x8e, 0xe5, 0xf7, 0x0a, 0xe6, 0x3d, 
        0x73, 0xd7, 0x65, 0x8d, 0x40, 0x26, 0xf2, 0xee, 
        0xfd, 0x2f, 0x20, 0x4c, 0x81, 0x68, 0x2c, 0xb7
    ]);


    #[test]
    fn address() {
        let signer = Signer::new(SECRET).unwrap();
        let address = signer.address();
        assert_eq!(address,ADDRESS);
    }

    #[test]
    fn secret() {
        let signer = Signer::new(SECRET).unwrap();
        let secret = signer.secret();
        assert_eq!(secret,SECRET);
    }

    #[test]
    fn ecrecover() {
        let signer = Signer::new(SECRET).unwrap();
        let msg = Keccak256::hash(b"hello world");
        let sig = signer.sign(&msg);
        let address = secp256k1::ecrecover(&msg,&sig).unwrap();
        assert_eq!(address,ADDRESS);
    }
}



