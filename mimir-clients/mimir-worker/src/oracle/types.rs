use mimir_node::node::SimpleRpcFuture;
use mimir_proto::message::Message;
use mimir_proto::seal::Sealer;
use futures::{Future,Async,Poll};
use oracle::util::MessageBuilder;
use oracle::OracleError;
use common::ArcSealer;
use serde_json::Value;
use web3::Error as Web3Error;




pub struct SimpleOracleFuture<T> {
    builder: Option<MessageBuilder>,
    sealer: ArcSealer,
    inner: SimpleRpcFuture<T>,
}


impl<T> SimpleOracleFuture<T> {

    pub fn new(builder: MessageBuilder, sealer: ArcSealer, inner: SimpleRpcFuture<T>) -> Self {
        let builder = Some(builder);
        SimpleOracleFuture { builder, sealer, inner }
    }
}


impl<T> Future for SimpleOracleFuture<T> where T: Future<Item=Value,Error=Web3Error> {

    type Item = Message;

    type Error = OracleError;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        let record = try_ready!(self.inner.poll());
        let builder = self.builder.take()
            .expect("no polling past completion");
        let mut message = builder.finish(record)?;
        let cert = self.sealer.seal_oracle(&message);
        message.verify.push(cert);
        Ok(Async::Ready(message))
    }
}



