//! web3 utilitiy namespace.
//!
use futures::future::Future;
use tokio_timer::Interval;
use util::{TxReportFuture,SyncReportFuture,AwaitSync};
use web3::types::{Address,CallRequest,BlockNumber};
use web3::api::{Eth,Net,Namespace};
use web3::Transport;


/// web3 utility namespace.
///
/// this type implements the web3 `Namespace` trait and exposes a set
/// of useful utility functions.
///
#[derive(Debug,Clone)]
pub struct Util<T> {
    transport: T
}


impl<T> Util<T> {

    pub fn new(transport: T) -> Self {
        Util { transport }
    }

    pub fn transport(&self) -> &T {
        &self.transport
    }
}


impl<T> Util<T> where T: Transport {

    /// get a report on the necessary info for tx compilation.
    ///
    pub fn tx_report(&self, from: Address, call: CallRequest) -> TxReportFuture<T> {
        let eth = Eth::new(self.transport.clone());
        let nonce_future = eth.transaction_count(from,BlockNumber::Latest.into());
        let price_future = eth.gas_price();
        let limit_future = eth.estimate_gas(call,BlockNumber::Latest.into());
        let inner = nonce_future.join3(price_future,limit_future);
        TxReportFuture { inner }
    }

    /// get a report on current synchronization indicators.
    ///
    pub fn sync_report(&self) -> SyncReportFuture<T> {
        let eth = Eth::new(self.transport.clone());
        let net = Net::new(self.transport.clone());
        let sync_future = eth.syncing();
        let block_future = eth.block(BlockNumber::Latest.into());
        let peer_future = net.peer_count();
        let inner = sync_future.join3(block_future,peer_future);
        SyncReportFuture { inner }
    }

    /// get a future which resolves when node appears synced.
    ///
    pub fn await_sync(&self, interval: Interval) -> AwaitSync<T> {
        let util = Clone::clone(self);
        AwaitSync::new(interval,util)
    }
}


impl<T> Namespace<T> for Util<T> where T: Transport {

    fn new(transport: T) -> Self {
        Util::new(transport)
    }

    fn transport(&self) -> &T {
        &self.transport()
    }
}


