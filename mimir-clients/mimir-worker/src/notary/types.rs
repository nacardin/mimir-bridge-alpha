use futures::{Future,Async,Poll};
use mimir_proto::visit::{self,BlockState};
use mimir_proto::judge::JudgeVisitor;
use mimir_proto::seal::Sealer;
use mimir_proto::message::{
    Message,
    STEP,
};
use common::ArcSealer;
use std::sync::Arc;


pub struct NotaryFuture {
    sealer: Option<ArcSealer>,
    message: Option<Message>,
    block: Option<Arc<BlockState>>,
}


impl NotaryFuture {

    pub fn new(sealer: ArcSealer, message: Message, block: Arc<BlockState>) -> Self {
        let (sealer,message,block) = (Some(sealer),Some(message),Some(block));
        Self { sealer, message, block }
    }

}


impl Future for NotaryFuture {

    type Item = Message;

    type Error = &'static str;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        let sealer = self.sealer.take().expect("no polling past completion");
        let mut message = self.message.take().expect("no polling past completion");
        let block = self.block.take().expect("no polling past complection");
        let (next_step,visit_okay,accusations) = {
            let mut visitor = JudgeVisitor::new(block);
            let next_step = visit::apply(&mut visitor, &message);
            let visit_okay = visitor.is_ok();
            let (_,accusations) = visitor.finish();
            (next_step,visit_okay,accusations)
        };
        match (next_step == STEP::NOTARY,visit_okay) {
            (true, true) => {
                let cert = sealer.seal_notary(&message);
                message.notary.push(cert);
                Ok(Async::Ready(message))
            },
            (true, false) => {
                if accusations.len() > 0 {
                    warn!("unhandled accusations {:?}",accusations);
                }
                Err("invalid message value")
            },
            (false, true) => {
                Err("invalid message step")
            },
            (false, false) => {
                if accusations.len() > 0 {
                    warn!("unhandled accusations {:?}",accusations);
                }
                Err("literally everything about this message is is wrong")
            },
        }
    }
}

