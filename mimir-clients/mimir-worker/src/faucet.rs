use mimir_node::transact::TransactFuture;
use mimir_node::{Transport,Error};
use mimir_node::node::SimpleNode;
use mimir_types::{Address,H256};
use futures::Future;
use lru_cache::LruCache;
use common::ArcSealer;
use std::time::{Duration,Instant};


#[derive(Debug,Clone)]
pub struct Faucet<T> {
    expiry: Duration,
    funded: LruCache<Address,Instant>, 
    sealer: ArcSealer,
    node: SimpleNode<T>, 
}


impl<T> Faucet<T> {

    pub fn new(sealer: ArcSealer, node: SimpleNode<T>) -> Self {
        let expiry = Duration::from_secs(1024);
        let funded = LruCache::new(256); 
        Self { expiry, funded, sealer, node }
    }
}


impl<T> Faucet<T> where T: Transport {

    /// attempt to build funding future for specified address
    ///
    /// returns `None` if entity is not eligible for funding.  care must be taken to only
    /// process one funding future at a time, else races around nonce values may emerge.
    /// the returned future does not begin until 
    ///
    pub fn get_fund_work(&mut self, address: Address) -> Option<impl Future<Item=H256,Error=Error>> {
        // get instant representing current time
        let now = Instant::now();
        // update funded mapping with current instant...
        // if previous attempt exists and is still within cooldown
        // return `None`.
        if let Some(previous) = self.funded.insert(address,now) {
            if previous + self.expiry > now { return None; }
        }
        let work = self.build_tx_future(address)
            .map(|rslt|H256::from(rslt.transaction_hash.0));
        Some(work)
    }

    /// construct a funding future... take care not to execute more than one funding
    /// future at a time, as this will produce a race-condition in nonce discovery.
    ///
    fn build_tx_future(&self, address: Address) -> TransactFuture<T,ArcSealer> {
        // 1 eth == 10 ^ 18 wei
        let value: u64 = 1000000000000000000;
        TransactFuture::new(
            self.node.transport().to_owned(),
            self.sealer.clone(),
            address,
            Some(value.into()),
            None
            )
    }
}


