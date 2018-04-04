//! basic message visitors
//!
use message::cert::{Oracle,Notary,Blind,Clear,Route,Verify};
use message::{Message,Payload};
use visit::MessageVisitor;
use mimir_crypto::{Hasher,HashVoyeur};
use mimir_types::H256;

/// visitor which retains references to visited elements
/// for later examination.
#[derive(Debug,Clone)]
pub struct MessageVoyeur<'v> {
    payload: Option<&'v Payload>,
    verify: Vec<&'v Verify>,
    notary: Vec<&'v Notary>,
    route: Vec<&'v Route>,
    blind: Vec<&'v H256>
}

impl<'v> MessageVoyeur<'v> {

    /// try to get an owned `Message` from
    /// current internal state.  returns `None`
    /// if no payload has been visited yet.
    pub fn as_msg(&self) -> Option<Message> {
        if let Some(ref inner) = self.payload {
            let payload = (*inner).to_owned();
            let verify = self.verify.iter()
                .map(|cert| (*cert).to_owned()).collect();
            let notary = self.notary.iter()
                .map(|cert| (*cert).to_owned()).collect();
            let route = self.route.iter()
                .map(|cert| (*cert).to_owned()).collect();
            let blind = self.blind.iter()
                .map(|cert| (*cert).to_owned()).collect();
            let msg = Message { payload, verify, notary, route, blind };
            Some(msg)
        } else {
            None
        }
    }
}


impl<'v> MessageVisitor<'v> for MessageVoyeur<'v> {

    /// output produced upon visitation.
    type Out = ();

    /// visit a message payload.
    fn visit_payload(&mut self, payload: &'v Payload) -> Self::Out {
        self.payload = Some(payload);
    }

    /// visit an oracle cert.
    fn visit_oracle(&mut self, cert: &'v Oracle) -> Self::Out {
        self.visit_verify(cert)
    }

    /// visit an notary cert.
    fn visit_notary(&mut self, cert: &'v Notary) -> Self::Out {
        self.notary.push(cert);
    }
 
    /// visit an hash-blinding cert.   
    fn visit_blind(&mut self, cert: &'v Blind) -> Self::Out {
        self.blind.push(cert);
    }
 
    /// visit a blind-clearing cert.   
    fn visit_clear(&mut self, cert: &'v Clear) -> Self::Out {
        self.visit_blind(cert)
    }

    /// visit an routing cert.   
    fn visit_route(&mut self, cert: &'v Route) -> Self::Out {
        self.route.push(cert);
    }
 
    /// visit a verification cert.   
    fn visit_verify(&mut self, cert: &'v Verify) -> Self::Out {
        self.verify.push(cert);
    }
}



/// visitor for collecting the bytes of a message
pub type ByteVisitor = HashVisitor<HashVoyeur>;


/// visitor for hashing a message.
pub struct HashVisitor<T> {
    /// internal hasher instance
    pub hasher: T
}


impl<T> HashVisitor<T> {

    /// unwrap inner hasher
    pub fn into_inner(self) -> T {
        let HashVisitor { hasher } = self;
        hasher
    }
}


impl<T> HashVisitor<T> where T: Hasher {

    /// consume visitor, returning the result of
    /// the inner hasher.
    pub fn finish(self) -> T::Out {
        let HashVisitor { hasher } = self;
        hasher.finish()
    }
}


impl HashVisitor<HashVoyeur> {

    /// get reference to inner byte collector
    pub fn as_bytes(&self) -> &[u8] {
        self.hasher.as_ref()
    }
}



impl<T> Default for HashVisitor<T> where T: Default {

    fn default() -> Self {
        let hasher = Default::default();
        HashVisitor { hasher }
    }
}



impl<'v,T> MessageVisitor<'v> for HashVisitor<T> where T: Hasher {

    
    type Out = ();

    
    fn visit_payload(&mut self, payload: &Payload) -> Self::Out {
        let &Payload { ref record, ref address, ref number, ref hash } = payload;
        self.hasher.absorb(record.as_ref());
        self.hasher.absorb(address.as_ref());
        self.hasher.absorb(number.as_ref());
        self.hasher.absorb(hash.as_ref());
    }

    
    fn visit_oracle(&mut self, cert: &Oracle) -> Self::Out {
        self.visit_verify(cert)
    }

    
    fn visit_notary(&mut self, cert: &Notary) -> Self::Out {
        let &Notary { ref sig } = cert;
        self.hasher.absorb(sig.as_ref())
    }
 
    
    fn visit_blind(&mut self, cert: &Blind) -> Self::Out {
        self.hasher.absorb(cert.as_ref())
    }
 
    
    fn visit_clear(&mut self, cert: &Clear) -> Self::Out {
        self.hasher.absorb(cert.as_ref())
    }
 
    
    fn visit_route(&mut self, cert: &Route) -> Self::Out {
        let &Route { ref val, ref sig } = cert;
        for address in val.iter() {
            self.hasher.absorb(address.as_ref()); 
        }
        self.hasher.absorb(sig.as_ref())
    }


    fn visit_verify(&mut self, cert: &Verify) -> Self::Out {
        let &Verify { ref val, ref sig } = cert;
        self.hasher.absorb(&[*val]);
        self.hasher.absorb(sig.as_ref());
    }
}

