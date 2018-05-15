use futures::future::{self,Future,Either};
use futures::stream::{self,Stream};
use mimir_node::contracts::WorkerSet;
use mimir_node::Error;
use mimir_transport::common::Role;
use mimir_types::Address;
use std::collections::HashMap;
use web3::Transport;


/// basic configuration of seeding source(s)
///
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SeedConfig {
    /// address of `WorkerSet` contract
    #[serde(default,skip_serializing_if = "Option::is_none")]
    contract: Option<Address>,
    
    /// list of pre-authorized entities
    #[serde(rename = "pre-auth",default,skip_serializing_if = "Vec::is_empty")]
    pre_auth: Vec<Address>,
}


/// configuration of seeding source(s) by role
///
pub type SeedSources = HashMap<Role,SeedConfig>;


/// collection of seeding values for a specific role
///
#[derive(Debug,Clone)]
pub struct SeedState {
    pub role: Role,
    pub conn: Vec<Address>,
    pub auth: Vec<Address>,
}


/// handle for loading seed state(s)
///
pub struct SeedLoader<T> {
    inner: HashMap<Role,BaseLoader<T>>,
}


impl<T: Clone> SeedLoader<T> {

    /// build new `SeedLoader` instance from specified sources
    ///
    pub fn new(transport: T, sources: SeedSources) -> Self {
        let inner = sources.into_iter().map(|(role,config)| {
            let SeedConfig { contract, pre_auth } = config;
            let contract = contract.map(|address| {
                WorkerSet::new(transport.clone(),address)
            });
            let loader = BaseLoader { contract, pre_auth };
            (role,loader)
        }).collect();
        Self { inner }
    }
}



impl<T> SeedLoader<T> where T: Transport {

    /// get a stream which will yield one `SeedState` for each role for which
    /// at least one source is known.
    ///
    pub fn get_seed_states(&self) -> impl Stream<Item=SeedState,Error=Error> {
        let all_work = (&self.inner).into_iter().map(|(&role,loader)| {
            let work = loader.get_conn_set().join(loader.get_auth_set())
                .map(move |(conn,auth)| {
                    let seed_state = SeedState { role, conn, auth };
                    debug!("generated {:?}",seed_state);
                    seed_state
                });
            work
        });
        stream::futures_unordered(all_work)
    }
}


struct BaseLoader<T> {
    contract: Option<WorkerSet<T>>,
    pre_auth: Vec<Address>,
}

impl<T> BaseLoader<T> where T: Transport {

    fn get_conn_set(&self) -> impl Future<Item=Vec<Address>,Error=Error> {
        let pre_auths = self.pre_auth.to_owned();
        if let Some(ref worker_set) = self.contract {
            let get_conn_future = worker_set.get_bound()
                .map(move |mut workers| {
                    // extend from pre-existing auths & remove duplicates
                    workers.extend_from_slice(&pre_auths);
                    workers.sort_unstable();
                    workers.dedup();
                    workers
                });
            Either::A(get_conn_future)
        } else {
            Either::B(future::ok(pre_auths))
        }
    }

    fn get_auth_set(&self) -> impl Future<Item=Vec<Address>,Error=Error> {
        // TODO: implement `active_set` calculation & chain updates... for alpha/debugging purposes,
        // we simply return the same values as a call to `get_conn_set`, since all operations
        // around the `auth` set are identical except for the method of initial calculation...
        self.get_conn_set()
    }
}
