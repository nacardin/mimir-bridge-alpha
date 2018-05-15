use mimir_transport::common::{Auth,Role};
use mimir_node::transact::TransactFuture;
use mimir_proto::visit::{BlockVisitor,BlockState};
use mimir_proto::message::Request;
use mimir_node::node::SimpleNode;
use mimir_node::rpc::SimpleQuery;
use mimir_node::abi::workerset;
use mimir_types::Address;
use crossbeam::sync::ArcCell;
use web3::types::CallRequest;
use web3::{self,Transport};
use futures::Future;
use std::sync::Arc;


use oracle::util::MessageBuilder;
use oracle::types::SimpleOracleFuture;
use common::ArcSealer;


pub type SimpleRequest = Request<SimpleQuery>;
 

#[derive(Debug)]
pub struct SimpleOracle<T> {
    sealer: ArcSealer,
    block: ArcCell<BlockState>,
    node: SimpleNode<T>,
}

impl<T> SimpleOracle<T> {

    /// instantiate new oracle client
    pub fn new(sealer: ArcSealer, node: SimpleNode<T>) -> Self {
        let block = ArcCell::new(Default::default());
        SimpleOracle { sealer, block, node }
    }

    /// get reference to inner node handle
    pub fn node(&self) -> &SimpleNode<T> { &self.node }

    /// get reference to inner sealer handle
    pub fn sealer(&self) -> &ArcSealer { &self.sealer }

    /// get handle to current block state
    pub fn get_block(&self) -> Arc<BlockState> { self.block.get() }
   
    /// set current block state
    pub fn set_block<B>(&self, block: B) where B: Into<Arc<BlockState>> {
        let block_state = block.into();
        debug!("setting new block state {:?}",block_state);
        let _ = self.block.set(block_state);
    }

    /// generate auth cert.
    pub fn gen_auth(&self) -> Auth {
        // NOTE: the `Validator` role has been depreciated in favor
        // of separate `Oracle` and `Verifier` roles.  The call below
        // will need to be changed as soon as the admin server & solidity
        // assets are updated to reflect this.
        Auth::new(&self.sealer,Role::Oracle)
    }
}


impl<T: Transport> SimpleOracle<T> {

    /// serve a simple request
    pub fn serve_request(&self, request: SimpleRequest) -> SimpleOracleFuture<T::Out> {
        debug!("serving request {:?}",request);
        let Request { address, blind, mut query } = request;
        let block = self.get_block();
        let builder = match (block.get_number(),block.get_hash()) {
            (Some(number),Some(hash)) => {
                query.seed_block(|| number.to_string());
                MessageBuilder::new(address,blind)
                    .number(number.to_owned())
                    .hash(hash.to_owned()) // NOTE: not guaranteed until atomic hash assertions implemented.
            },
            _ => {
                warn!("serving request without block seeding...");
                MessageBuilder::new(address,blind)
            }
        };
        let sealer = self.sealer.clone();
        let work = self.node.execute_query(query); 
        SimpleOracleFuture::new(builder,sealer,work)
    }


    /// check if worker is 'bound'
    pub fn check_bound_state(&self, api_contract: Address) -> Box<Future<Item=bool,Error=web3::Error>> where T::Out: 'static {
        let address = self.sealer().address();
        let calldata = workerset::is_bound(&address);
        let request = CallRequest {
            from: Some(address.into_other()),
            to: api_contract.into_other(),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(calldata.into_other())
        };
        let work = self.node.eth().call(request,None)
            .and_then(|mut bytes| {
                match bytes.0.pop() {
                    Some(0) => { Ok(false) },
                    Some(1) => { Ok(true)  },
                    _ => {
                        let message = "expected boolian value (0 or 1)";
                        let kind = web3::ErrorKind::InvalidResponse(message.to_string());
                        Err(web3::Error::from(kind))
                    }
                }
            });
        Box::new(work)
    }


    /// build a transaction future for stake lock.
    pub fn lock_stake(&self, api_contract: Address) -> TransactFuture<T,ArcSealer> {
        let address = self.sealer().address();
        let calldata = workerset::set_bound(&address);
        TransactFuture::new(
            self.node.transport().clone(),
            self.sealer.clone(),
            api_contract,
            Default::default(),
            Some(calldata)
            ) 
    }
}

