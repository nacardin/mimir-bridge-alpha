//! message visitors for cert verification
//!
use message::cert::{Oracle,Notary,Blind,Clear,Route,Verify};
use message::Payload;
use visit::MessageVisitor;
use visit::message::ByteVisitor;
use mimir_crypto::secp256k1::Error as SigError;
use mimir_crypto::secp256k1::Verifier;
use mimir_crypto::Keccak256;
use mimir_types::Address;
use std::{fmt,error};


/// message visitor for cert verification
///
/// returns the recovered address for passing certs which
/// contain signatures, and `None` for those which don't.
/// if cert fails one or more checks, an error describing
/// the issue is returned.
///
#[derive(Default)]
pub struct CertVisitor {
    verifier: Verifier,
    hasher: CertHasher,
    blind: Option<Blind>
}


impl CertVisitor {

    /// consume visitor, returning vector of
    /// visited bytes
    pub fn finish(self) -> Vec<u8> {
        self.hasher.inner.finish()
    }
}


impl<'v> MessageVisitor<'v> for CertVisitor {
    
    
    type Out = Result<Option<Address>,CertError>;

    
    fn visit_payload(&mut self, payload: &Payload) -> Self::Out {
        let _ = self.hasher.visit_payload(payload);
        Ok(None)
    }

    
    fn visit_oracle(&mut self, cert: &Oracle) -> Self::Out {
        let hash = self.hasher.visit_oracle(cert)
            .expect("always returns a value");
        let addr = self.verifier.ecrecover(&hash,&cert.sig)?;
        if cert.val == 0 {
            Ok(Some(addr))
        } else {
            Err(CertError::Flag { addr })
        }
    }

    
    fn visit_notary(&mut self, cert: &Notary) -> Self::Out {
        let hash = self.hasher.visit_notary(cert)
            .expect("always returns a value");
        let addr = self.verifier.ecrecover(&hash,&cert.sig)?;
        Ok(Some(addr))
    }
 
    
    fn visit_blind(&mut self, cert: &Blind) -> Self::Out {
        let _ = self.hasher.visit_blind(cert);
        self.blind = Some(cert.clone());
        Ok(None)
    }
 
    
    fn visit_clear(&mut self, cert: &Clear) -> Self::Out {
        let _ = self.hasher.visit_clear(cert);
        if let Some(blind) = self.blind.take() {
            let hash = Keccak256::hash(cert);
            if blind.as_ref() == hash.as_ref() {
                Ok(None)
            } else {
                Err(CertError::InvalidBlind)
            }
        } else {
            Err(CertError::MissingBlind)
        }
    }
 
    
    fn visit_route(&mut self, cert: &Route) -> Self::Out {
        let hash = self.hasher.visit_route(cert)
            .expect("always returns a value");
        let addr = self.verifier.ecrecover(&hash,&cert.sig)?;
        Ok(Some(addr))
    }
 
    
    fn visit_verify(&mut self, cert: &Verify) -> Self::Out {
        let hash = self.hasher.visit_verify(cert)
            .expect("always returns a value");
        let addr = self.verifier.ecrecover(&hash,&cert.sig)?;
        Ok(Some(addr))
    }
}



/// visitor for generating signature recovery hashes
///
/// returns the hash necessary for signature recovery on certs
/// which contain signatures, and `None` for all others.
#[derive(Default)]
struct CertHasher {
    inner: ByteVisitor
}


impl<'v> MessageVisitor<'v> for CertHasher {
   

    type Out = Option<[u8;32]>;

    
    fn visit_payload(&mut self, payload: &Payload) -> Self::Out {
        self.inner.visit_payload(payload);
        None
    }

    
    fn visit_oracle(&mut self, cert: &Oracle) -> Self::Out {
        self.visit_verify(cert)
    }

    
    fn visit_notary(&mut self, cert: &Notary) -> Self::Out {
        let hash = Keccak256::hash(self.inner.as_bytes());
        self.inner.visit_notary(cert);
        Some(hash)
    }
 
    
    fn visit_blind(&mut self, cert: &Blind) -> Self::Out {
        self.inner.visit_blind(cert);
        None
    }
 
    
    fn visit_clear(&mut self, cert: &Clear) -> Self::Out {
        self.inner.visit_clear(cert);
        None
    }
 
    
    fn visit_route(&mut self, cert: &Route) -> Self::Out {
        let mut hasher = Keccak256::default();
        hasher.absorb(self.inner.as_bytes());
        for address in cert.val.iter() {
            hasher.absorb(address.as_ref()); 
        }
        let hash = hasher.finish();
        self.inner.visit_route(cert);
        Some(hash)
    }
 
    
    fn visit_verify(&mut self, cert: &Verify) -> Self::Out {
        let mut hasher = Keccak256::default();
        hasher.absorb(self.inner.as_bytes());
        hasher.absorb(&[cert.val]);
        let hash = hasher.finish();
        self.inner.visit_oracle(cert);
        Some(hash)
    }
}



/// error returned by cert visitor
///
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum CertError {
    /// requester blind was invalid
    InvalidBlind,
    
    /// got a `clear` cert with no
    /// preceeding `blind` cert.
    MissingBlind,
    
    /// signature was invalid
    Sig {
        /// error raised by verifier
        error: SigError
    }, 
    
    /// cert contained invalid flag 
    Flag {
        /// identity of cert generator
        addr: Address
    }
}


impl CertError {

    /// get simple static error message
    pub fn as_str(&self) -> &'static str {
        match *self {
            CertError::InvalidBlind => "invalid hash-blind",
            CertError::MissingBlind => "missing hash-blind",
            CertError::Sig { .. } => "bad signature",
            CertError::Flag { .. } => "illegal flag"
        }
    }
}


impl From<SigError> for CertError {

    fn from(error: SigError) -> Self {
        CertError::Sig { error }
    }
}

impl fmt::Display for CertError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CertError::InvalidBlind => f.write_str(self.as_str()),
            CertError::MissingBlind => f.write_str(self.as_str()),
            CertError::Sig { ref error } => error.fmt(f),
            CertError::Flag { .. } => f.write_str(self.as_str())
        }
    }
}

impl error::Error for CertError {

    fn description(&self) -> &str {
        self.as_str()
    }
}


