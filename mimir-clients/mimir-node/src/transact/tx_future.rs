use mimir_types::{Signature,Address,U256,Bytes};
use mimir_crypto::Signer;
use transact::RawTxBuilder;
use futures::{Future,Poll,Async};
use web3::confirm::{self,SendTransactionWithConfirmation};
use web3::types::{CallRequest,TransactionReceipt};
use web3::{Transport,Error};
use util::{Util,TxReportFuture};
use std::time::Duration;


// TODO: This future has a number of hard-coded behaviors which need to be
// made configurable via config/builder pattern of some kind.  See comments.


/// future which manages raw transaction generation, signing, and confirmation.
///
pub struct TransactFuture<T,S> where T: Transport {
    transport: T,
    signer: S,
    inner: TransactInner,
    state: TransactState<T>,
}


impl<T,S> TransactFuture<T,S> where T: Transport, S: Signer<Msg=[u8;32],Sig=Signature,Pub=Address> {

    pub fn new(transport: T, signer: S, to: Address, value: Option<U256>, data: Option<Bytes>) -> Self {
        let from: Address = signer.identify();
        let inner = TransactInner {
            from: from.into(),
            to: to.into(),
            value: value.map(Into::into),
            data: data.map(Into::into),
        };
        let state = TransactState::AwaitPoll;
        TransactFuture { transport, signer, inner, state }
    }

}



impl<T,S> Future for TransactFuture<T,S> where T: Transport, S: Signer<Msg=[u8;32],Sig=Signature,Pub=Address> {

    type Item = TransactionReceipt;

    type Error = Error;


    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        loop {
            let next_state = match self.state {
                TransactState::AwaitPoll => {
                    let call: CallRequest = self.inner.clone().into();
                    let util = Util::new(self.transport.clone());
                    let work = util.tx_report(self.inner.from.clone().into(),call);
                    TransactState::GetReport { work }
                },
                TransactState::GetReport { ref mut work } => {
                    let report = try_ready!(work.poll());
                    let value: &[u8] = self.inner.value.as_ref()
                        .map(|val| val.as_ref())
                        .unwrap_or(&[]);
                    let data: &[u8] = self.inner.data.as_ref()
                        .map(|val| val.as_ref())
                        .unwrap_or(&[]);
                    let acc_nonce = {
                        let mut buff = [0u8;32];
                        report.acc_nonce.to_big_endian(&mut buff);
                        buff
                    };
                    let gas_price = {
                        let mut buff = [0u8;32];
                        report.gas_price.to_big_endian(&mut buff);
                        buff
                    };
                    let gas_limit = {
                        let mut buff = [0u8;32];
                        report.gas_limit.to_big_endian(&mut buff);
                        buff
                    };
                    let raw_tx: Vec<u8> = RawTxBuilder::new(&self.signer)
                        .nonce(&acc_nonce)
                        .gas_price(&gas_price) // TODO: add method of configuring gas price/limit modifiers
                        .gas_limit(&gas_limit) // (increment, decrement, etc...).
                        .to(&self.inner.to)
                        .value(value)
                        .data(data)
                        .finish();
                    let work = confirm::send_raw_transaction_with_confirmation(
                        self.transport.clone(),
                        raw_tx.into(),
                        Duration::from_secs(1), // TODO: make configurable
                        3, // TODO: make configurable
                        );
                    TransactState::ConfirmTx { work }
                },
                TransactState::ConfirmTx { ref mut work } => {
                    let receipt = try_ready!(work.poll());
                    return Ok(Async::Ready(receipt));
                },
            };
            self.state = next_state;
        }
    }
}



enum TransactState<T: Transport> {
    AwaitPoll,
    GetReport {
        work: TxReportFuture<T>
    },
    ConfirmTx {
        work: SendTransactionWithConfirmation<T>
    },
}


#[derive(Clone)]
struct TransactInner {
    from: [u8;20],
    to: [u8;20],
    value: Option<[u8;32]>,
    data: Option<Vec<u8>>,
}


impl Into<CallRequest> for TransactInner {

    fn into(self) -> CallRequest {
        CallRequest {
            from: Some(self.from.into()),
            to: self.to.into(),
            gas: None,
            gas_price: None,
            value: self.value.map(Into::into),
            data: self.data.map(Into::into),
        }
    }
}

