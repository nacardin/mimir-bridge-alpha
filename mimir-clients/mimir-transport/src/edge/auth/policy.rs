use common::{Identity,Role};
use edge::auth::{AuthServer,NeverFuture};
use edge::Error;
use std::collections::HashMap;
use futures::future::{self,Either};
use futures::Future;

simple_unit!(
    AuthPolicy, "per-role login/auth policy",
    Standard => "standard",
    AllowAll => "allow-all",
    DenyAll  => "deny-all",
);


/// `AuthServer` combinator which enforces simple per-role policies
///
/// This combinator allows simple role-based policies (e.g. `deny-all`)
/// to be enforced on incoming connections, transparent to the underlying
/// `AuthServer` implementation.
///
#[derive(Debug,Clone)]
pub struct Policy<T> {
    auth_server: T,
    policies: HashMap<Role,AuthPolicy>,
}


impl<T> Policy<T> {

    pub fn new(auth_server: T, policies: HashMap<Role,AuthPolicy>) -> Self {
        Self { auth_server, policies }
    }

    fn get_policy(&self, role: Role) -> AuthPolicy {
        match self.policies.get(&role) {
            Some(&policy) => policy,
            None => AuthPolicy::Standard,
        }
    }
}


impl<T> Policy<T> where T: AuthServer, Error: From<T::Error> {

    /// acquire authorization
    ///
    pub fn authorize(&self, identity: Identity) -> impl Future<Item=impl Future<Item=(),Error=Error>,Error=Error> {
        match self.get_policy(identity.role) {
            AuthPolicy::Standard => {
                // forward to inner `auth_server` implementation...
                // acquisition and subsequent holding futures must both be
                // wrapped in error conversions and instances of `Either::A`
                // in order to keep type consistent with other match arms.
                let work = self.auth_server.authorize(identity)
                    .map_err(|e|Error::from(e))
                    .map(|hold| {
                        let hold_work = hold.map_err(|e|Error::from(e));
                        Either::A(hold_work)
                    });
                Either::A(work)
            },
            AuthPolicy::AllowAll => {
                // unconditionally allow all connections for specified role...
                // only usefuly for debugging/testing purposes (obviously).
                let hold = Either::B(NeverFuture);
                Either::B(future::ok(hold))
            },
            AuthPolicy::DenyAll => {
                // unconditionally deny all connections for specified role...
                // typically useful if a given endpoint is intended to only
                // service a subset of the possible roles.
                let msg = "role-connection policy set to `deny-all`";
                Either::B(future::err(Error::Other(msg)))
            },
        }
    }


    /// run supplied future while authorization is held
    ///
    pub fn while_authorized<F>(&self, identity: Identity, work: F) -> impl Future<Item=(),Error=Error>
            where F: Future<Item=()> + 'static, Error: From<F::Error> {
        let acquire = self.authorize(identity);
        let work = acquire.and_then(move |hold| {
            info!("got auth for {}",identity);
            hold.select(work.map_err(|e|Error::from(e)))
                .map_err(|(e,_)|e).map(|_|())
        });
        work
    }
}

