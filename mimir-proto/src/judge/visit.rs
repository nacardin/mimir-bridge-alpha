use visit::{
    MessageVisitor,
    MessageVoyeur,
    BlockVisitor,
    CertVisitor,
    CertError,
};
use judge::{Accuse,JudgeError};
use mimir_crypto::Address;
use message::Payload;
use message::cert::{
    Oracle,
    Notary,
    Blind,
    Clear,
    Route,
    Verify,
};


/// visitor which passes judement(s) on a message.
///
#[derive(Debug)]
pub struct JudgeVisitor<'v,B> {
    /// buffer of potential accusations
    accuse: Vec<Accuse>,

    /// inner visitor which performs cryptographic checks
    inner: CertVisitor,

    /// snapshot of currently visited message state
    state: MessageVoyeur<'v>,

    /// verifies set membership for block
    block: B,

    /// indicates truth of message payload
    truth: bool,
    
    /// result indicating whether an error
    /// has been seen
    seen: Result<(),JudgeError>,
}


impl<'v,B> Default for JudgeVisitor<'v,B> where B: Default {

    fn default() -> Self { Self::new(Default::default()) } 
}


impl<'v,B> JudgeVisitor<'v,B> {

    /// instantiate new judge instance
    pub fn new(block: B) -> Self {
        let (accuse,inner,state) = Default::default();
        let (truth,seen) = (true,Ok(()));
        Self { accuse, inner, state, block, truth, seen }
    }

    /// check if internal state-tracking values
    /// are all in thier `ok` cases.
    pub fn is_ok(&self) -> bool {
        self.truth && self.seen.is_ok()
    }

    /// consume visitor, returning collected
    /// message bytes and potential accusations.
    pub fn finish(self) -> (Vec<u8>,Vec<Accuse>) {
        let bytes = self.inner.finish();
        (bytes,self.accuse)
    }

    /// visit an error.  updates internal error tracker
    /// if an earlier error is not already set.
    fn visit_error<E>(&mut self, err: &E) where E: Into<JudgeError> + Clone {
        if self.seen.is_ok() {
            self.seen = Err(err.clone().into())
        }
    }
}


impl<'v,B> JudgeVisitor<'v,B> where B: BlockVisitor<Out=bool> {

    fn validator_exists(&mut self, addr: &Address) -> bool {
        let exists = self.block.visit_validator(addr);
        if !exists { self.visit_error(addr); }
        exists
    }

    fn notary_exists(&mut self, addr: &Address) -> bool {
        let exists = self.block.visit_notary(addr);
        if !exists { self.visit_error(addr); }
        exists
    }

    fn router_exists(&mut self, addr: &Address) -> bool {
        let exists = self.block.visit_router(addr);
        if !exists { self.visit_error(addr); }
        exists
    }
}


impl<'v,B> MessageVisitor<'v> for JudgeVisitor<'v,B> where B: BlockVisitor<Out=bool> {

    /// output produced upon visitation.
    type Out = ();

    /// visit a message payload.
    fn visit_payload(&mut self, payload: &'v Payload) -> Self::Out {
        let _ = self.inner.visit_payload(payload)
            .map_err(|e| { self.visit_error(&e); e });
        self.state.visit_payload(payload);
    }

    /// visit an oracle cert.
    fn visit_oracle(&mut self, cert: &'v Oracle) -> Self::Out {
        let preprocess = self.inner.visit_oracle(cert)
            .map_err(|e| { self.visit_error(&e); e });
        match preprocess {
            // match cases where address is known
            Ok(Some(addr)) | Err(CertError::Flag { addr }) => {
                // if validator exists and state is bad,
                // attempt to build accusation.
                if self.validator_exists(&addr) && !self.is_ok() { 
                    if let Some(inner) = self.state.as_msg() {
                        let cert = cert.clone();
                        let seal = None;
                        let acc = Accuse::Oracle { inner, cert, seal };
                        self.accuse.push(acc);
                    } else {
                        // this state is not necessarily an error... but
                        // shouldn't happen during normal use either.
                        warn!("[judge-oracle] missing message payload");
                    }
                }
            },
            // otherwise, ignore
            _ => { }
        }
        // update interval message state record
        self.state.visit_oracle(cert);
    }

    /// visit an notary cert.
    fn visit_notary(&mut self, cert: &'v Notary) -> Self::Out { 
        let preprocess = self.inner.visit_notary(cert)
            .map_err(|e| { self.visit_error(&e); e });
        match preprocess {
            // match cases where address is known
            Ok(Some(addr)) | Err(CertError::Flag { addr }) => {
                let _ = self.notary_exists(&addr);
            },
            // otherwise, ignore
            _ => { }
        }
        // update interval message state record
        self.state.visit_notary(cert);
    }
 
    /// visit a hash-blinding cert.   
    fn visit_blind(&mut self, cert: &'v Blind) -> Self::Out {
        let _ = self.inner.visit_blind(cert)
            .map_err(|e| { self.visit_error(&e); e });
        self.state.visit_blind(cert);
    }
 
    /// visit a blind-clearing cert.   
    fn visit_clear(&mut self, cert: &'v Clear) -> Self::Out {
        let _ = self.inner.visit_clear(cert)
            .map_err(|e| { self.visit_error(&e); e });
        self.state.visit_clear(cert);
    }
 
    /// visit a routing cert.   
    fn visit_route(&mut self, cert: &'v Route) -> Self::Out {
        let preprocess = self.inner.visit_route(cert)
            .map_err(|e| { self.visit_error(&e); e });
        match preprocess {
            // match cases where address is known
            Ok(Some(addr)) | Err(CertError::Flag { addr }) => {
                // iteratively pass route to `validator_exists` check.
                for address in cert.val.iter() {
                    // can ignore result here.  inner state will be
                    // `err` case if any validators did not exist.
                    let _ = self.validator_exists(&address);
                }
                // if router exists and state is bad,
                // attempt to build accusation.
                if self.router_exists(&addr) && !self.is_ok() { 
                    if let Some(inner) = self.state.as_msg() {
                        let cert = cert.clone();
                        let acc = Accuse::Route { inner, cert };
                        self.accuse.push(acc);
                    } else {
                        // this state is not necessarily an error... but
                        // shouldn't happen during normal use either.
                        warn!("[judge-route] missing message payload");
                    }
                }
            },
            // otherwise, ignore
            _ => { }
        }
        // update interval message state record
        self.state.visit_route(cert);
    }
 
    /// visit a verification cert.   
    fn visit_verify(&mut self, cert: &'v Verify) -> Self::Out {
        let preprocess = self.inner.visit_verify(cert)
            .map_err(|e| { self.visit_error(&e); e });
        match preprocess {
            // match cases where address is known
            Ok(Some(addr)) | Err(CertError::Flag { addr }) => {
                // if validator exists and state is bad,
                // attempt to build accusation.
                if self.validator_exists(&addr) && !self.is_ok() { 
                    if let Some(inner) = self.state.as_msg() {
                        let cert = cert.clone();
                        let acc = Accuse::Verify { inner, cert };
                        self.accuse.push(acc);
                    } else {
                        // this state is not necessarily an error... but
                        // shouldn't happen during normal use either.
                        warn!("[judge-verify] missing message payload");
                    }
                }
            },
            // otherwise, ignore
            _ => { }
        }
        // update interval message state record
        self.state.visit_verify(cert);
    }
}


