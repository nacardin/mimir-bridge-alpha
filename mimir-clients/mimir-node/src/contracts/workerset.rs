//! `WorkerSet` contact object.
//!
use mimir_types::{Address,Signature};
use mimir_crypto::Signer;
use futures::Future;
use web3::types::{Bytes,BlockNumber,CallRequest};
use web3::helpers::CallResult;
use web3::error::{Error,ErrorKind};
use web3::api::{Eth,Namespace};
use web3::Transport;
use transact::TransactFuture;
use abi::workerset;


/// a contract which describes a set of related workers
#[derive(Debug,Clone)]
pub struct WorkerSet<T> {
    transport: T,
    address: Address,
}


impl<T> WorkerSet<T> {

    pub fn new(transport: T, address: Address) -> Self {
        Self { transport, address }
    }

    pub fn transport(&self) -> &T { &self.transport }

    pub fn address(&self) -> &Address { &self.address }
}


impl<T> WorkerSet<T> where T: Transport {

    /// check if specified worker is in bound set
    pub fn is_bound(&self, worker: &Address) -> impl Future<Item=bool,Error=Error> {
        let calldata = workerset::is_bound(worker);
        let work = self.eth_call(calldata.into_inner())
            .map(|rslt| rslt.0.iter().any(|x| *x > 0));
        work
    }


    /// add specified worker to bound set
    pub fn set_bound<S>(&self, worker: &Address, sealer: S) -> TransactFuture<T,S> 
            where S: Signer<Msg=[u8;32],Sig=Signature,Pub=Address> {
        let calldata = workerset::set_bound(worker);
        TransactFuture::new(
            self.transport.clone(),
            sealer,
            self.address.clone(),
            None,
            Some(calldata)
            )
    }


    /// get all currently bound workers
    pub fn get_bound(&self) -> impl Future<Item=Vec<Address>,Error=Error> {
        let calldata = workerset::get_bound();
        let work = self.eth_call(calldata.into_inner())
            //.and_then(|rslt| parse_addresses(&rslt.0));
            .and_then(|rslt|parse_addresses(&rslt.0));
        work
    }


    /// helper for contract-call construction
    fn eth_call(&self, calldata: Vec<u8>) -> CallResult<Bytes,T::Out> {
        let request = CallRequest {
            from: None,
            to: self.address.into_inner().into(),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(calldata.into())
        };
        let eth = Eth::new(self.transport.clone());
        let block = BlockNumber::Latest;
        eth.call(request,Some(block))
    }
}


/// parse returndata as vector of addresses
fn parse_addresses(bytes: &[u8]) -> Result<Vec<Address>,Error> {
    let mut addresses = Vec::new();
    if bytes.len() > 64 {
        for chunk in bytes[64..].chunks(32) {
            if chunk.len() == 32 {
                let mut addr = Address::default();
                addr.copy_from_slice(&chunk[12..]);
                addresses.push(addr);
            } else if chunk.iter().all(|x| *x == 0) {
                // allow trailing zeroes
                break;
            } else {
                let message = "trailing nonzero bytes in address array".to_string();
                return Err(ErrorKind::Msg(message).into());
            }
        }
    }
    Ok(addresses)
}
