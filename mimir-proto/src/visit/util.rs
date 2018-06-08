//! helpers related to applying visitors
//!
use message::{Message,STEP};
use visit::MessageVisitor;


macro_rules! try_next_or {
    ($iter:expr,$ret:expr) => {
        if let Some(val) = $iter.next() {
            val
        } else {
            return $ret;
        }
    }
}


/// apply a visitor to a function.
///
/// performs the minimal logic necessary to apply a visitor to a
/// message.  returns the value of the step at which application could
/// no longer continue.
pub fn apply<'v,V>(mut visitor: V, message: &'v Message) -> STEP where V: MessageVisitor<'v,Out=()> {
    // destructure message, and generate iterators for all certs.
    let &Message { ref payload, ref verify, ref notary, ref route, ref blind } = message;
    let mut verify_certs = verify.iter();
    let mut notary_certs = notary.iter();
    let mut route_certs = route.iter();
    let mut blind_certs = blind.iter();
    // pass message payload to visitor.
    visitor.visit_payload(payload);
    // apply certs until an iterator returns none
    for step in (0..).into_iter().map(|i| STEP::new(i)) {
        match step {
            STEP::ORACLE => visitor.visit_oracle(try_next_or!(verify_certs,step)),
            
            STEP::NOTARY => visitor.visit_notary(try_next_or!(notary_certs,step)), 
            
            STEP::BLIND => visitor.visit_blind(try_next_or!(blind_certs,step)),

            STEP::CLEAR => visitor.visit_clear(try_next_or!(blind_certs,step)),

            STEP::ROUTE => visitor.visit_route(try_next_or!(route_certs,step)),
            
            STEP::VERIFY => visitor.visit_verify(try_next_or!(verify_certs,step))
        }
    }
    unreachable!()
}

