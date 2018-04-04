use web3::api::{Eth,Namespace};
use web3::Transport;
use node::SimpleRpcFuture;
use rpc::SimpleQuery;
use util::Util;


/// simple node handle.
/// 
/// this simple handle is intended to execute instances of `SimpleQuery`.
///
#[derive(Debug,Clone)]
pub struct SimpleNode<T> {
    transport: T
}


impl<T> SimpleNode<T> {

    pub fn new(transport: T) -> Self { SimpleNode { transport } }

    pub fn transport(&self) -> &T { &self.transport }

}


impl<T: Transport> SimpleNode<T> {

    pub fn eth(&self) -> Eth<T> { Eth::new(self.transport.clone()) }

    pub fn util(&self) -> Util<T> { Util::new(self.transport.clone()) }

    pub fn execute_query(&self, query: SimpleQuery) -> SimpleRpcFuture<T::Out> {
        if let Some(result) = query.method.static_result() {
            SimpleRpcFuture::succeed(query,result)
        } else {
            let params = query.params.clone();
            let work = self.transport.execute(query.method.as_ref(),params);
            SimpleRpcFuture::new(query,work)
        }
    }
}


impl<T: Transport> Namespace<T> for SimpleNode<T> {

    fn new(transport: T) -> Self { SimpleNode::new(transport) }

    fn transport(&self) -> &T { self.transport() }
}

