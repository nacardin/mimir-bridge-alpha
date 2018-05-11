/// notary worker
///

mod options;
mod types;


pub use self::options::Options;
pub use self::types::NotaryFuture;

use mimir_proto::message::Message;
use mimir_proto::visit::BlockState;
use crossbeam::sync::ArcCell;
use common::ArcSealer;
use std::sync::Arc;


pub struct Notary {
    sealer: ArcSealer,
    block: ArcCell<BlockState>,
}


impl Notary {

    /// instantiate new notary
    pub fn new(sealer: ArcSealer) -> Self {
        let block = ArcCell::new(Default::default());
        Self { sealer, block }
    }

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

    /// attempt to notarize specified message
    pub fn notarize(&self, message: Message) -> NotaryFuture {
        NotaryFuture::new(self.sealer.clone(),message,self.get_block())
    }
}

