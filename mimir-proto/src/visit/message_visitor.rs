use message::cert::{Oracle,Notary,Blind,Clear,Route,Verify};
use message::Payload;


/// Trait for processing messages with visitor pattern.
/// 
/// *note*: the `'v` lifetime allows visitors to be implemented which
/// hold internal references to the elements they visit.  as such, a
/// message must always outlive its visitors.
pub trait MessageVisitor<'v> {

    /// output produced upon visitation.
    type Out;

    /// visit a message payload.
    fn visit_payload(&mut self, payload: &'v Payload) -> Self::Out;

    /// visit an oracle cert.
    fn visit_oracle(&mut self, cert: &'v Oracle) -> Self::Out;

    /// visit an notary cert.
    fn visit_notary(&mut self, cert: &'v Notary) -> Self::Out;
 
    /// visit an hash-blinding cert.   
    fn visit_blind(&mut self, cert: &'v Blind) -> Self::Out;
 
    /// visit a blind-clearing cert.   
    fn visit_clear(&mut self, cert: &'v Clear) -> Self::Out;
 
    /// visit an routing cert.   
    fn visit_route(&mut self, cert: &'v Route) -> Self::Out;
 
    /// visit a verification cert.   
    fn visit_verify(&mut self, cert: &'v Verify) -> Self::Out;
}


impl<'a,'v: 'a,T> MessageVisitor<'v> for &'a mut T where T: MessageVisitor<'v> + ?Sized {

    type Out = T::Out;

    fn visit_payload(&mut self, payload: &'v Payload) -> Self::Out { <T as MessageVisitor>::visit_payload(self,payload) }

    fn visit_oracle(&mut self, cert: &'v Oracle) -> Self::Out { <T as MessageVisitor>::visit_oracle(self,cert) }

    fn visit_notary(&mut self, cert: &'v Notary) -> Self::Out { <T as MessageVisitor>::visit_notary(self,cert) }
 
    fn visit_blind(&mut self, cert: &'v Blind) -> Self::Out { <T as MessageVisitor>::visit_blind(self,cert) }
 
    fn visit_clear(&mut self, cert: &'v Clear) -> Self::Out { <T as MessageVisitor>::visit_clear(self,cert) }
 
    fn visit_route(&mut self, cert: &'v Route) -> Self::Out { <T as MessageVisitor>::visit_route(self,cert) }
 
    fn visit_verify(&mut self, cert: &'v Verify) -> Self::Out { <T as MessageVisitor>::visit_verify(self,cert) }
}


impl<'v,T> MessageVisitor<'v> for Box<T> where T: MessageVisitor<'v> + ?Sized {

    type Out = T::Out;

    fn visit_payload(&mut self, payload: &'v Payload) -> Self::Out { <T as MessageVisitor>::visit_payload(self,payload) }

    fn visit_oracle(&mut self, cert: &'v Oracle) -> Self::Out { <T as MessageVisitor>::visit_oracle(self,cert) }

    fn visit_notary(&mut self, cert: &'v Notary) -> Self::Out { <T as MessageVisitor>::visit_notary(self,cert) }
 
    fn visit_blind(&mut self, cert: &'v Blind) -> Self::Out { <T as MessageVisitor>::visit_blind(self,cert) }
 
    fn visit_clear(&mut self, cert: &'v Clear) -> Self::Out { <T as MessageVisitor>::visit_clear(self,cert) }
 
    fn visit_route(&mut self, cert: &'v Route) -> Self::Out { <T as MessageVisitor>::visit_route(self,cert) }
 
    fn visit_verify(&mut self, cert: &'v Verify) -> Self::Out { <T as MessageVisitor>::visit_verify(self,cert) }
}
