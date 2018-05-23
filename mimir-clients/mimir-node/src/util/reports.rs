//! helper futures which collect a set of related data.
//!
use futures::future::{Future,Join3};
use futures::{Poll,Async};
use web3::helpers::CallResult;
use web3::{Error,Transport};
use web3::types::{
    U256,
    H256,
    SyncState,
    Block,
};


/// Detailed report of info needed for tx compilation.
#[derive(Debug,Clone,PartialEq)]
pub struct TxReport {
    /// nonce of sender account
    pub acc_nonce: U256,

    /// gas price of latest block
    pub gas_price: U256,

    /// minimum estimated gas limit
    pub gas_limit: U256
}


/// future which will resolve to a `TxReport`.
pub struct TxReportFuture<T> where T: Transport {
    pub(crate) inner: Join3<CallResult<U256,T::Out>,CallResult<U256,T::Out>,CallResult<U256,T::Out>>
}


impl<T> Future for TxReportFuture<T> where T: Transport {

    type Item = TxReport;

    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        let (acc_nonce,gas_price,gas_limit) = try_ready!(self.inner.poll());
        let report = TxReport { acc_nonce, gas_price, gas_limit };
        Ok(Async::Ready(report))
    }
}


/// Detailed report of blockchain sync related info.
///
/// This report is useful for determining whether a node is properly
/// synchronized with the blockchain.
#[derive(Debug,Clone,PartialEq)]
pub struct SyncReport {
    /// state of current sync ops if any
    pub sync_state: SyncState,

    /// description of last block
    pub last_block: Option<Block<H256>>,

    /// number of currently connected peers
    pub peer_count: U256
}


/// future which will resolve to a `SyncReport`.
pub struct SyncReportFuture<T> where T: Transport {
    pub(crate) inner: Join3<CallResult<SyncState,T::Out>,CallResult<Option<Block<H256>>,T::Out>,CallResult<U256,T::Out>>
}



impl<T> Future for SyncReportFuture<T> where T: Transport {
    
    type Item = SyncReport;

    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        let (sync_state,last_block,peer_count) = try_ready!(self.inner.poll());
        let sync_report = SyncReport { sync_state, last_block, peer_count };
        Ok(Async::Ready(sync_report))
    }
}



