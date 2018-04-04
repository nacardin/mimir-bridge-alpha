use futures::future::{self,Either,FutureResult};
use futures::{Future,Async,Poll};
use rpc::{SimpleQuery,SimpleRecord};
use serde_json::Value;
use web3::Error;
use node::util;


/// future which drives a simple transparent rpc request.
///
pub struct SimpleRpcFuture<T> {
    query: Option<SimpleQuery>,
    inner: Either<T,FutureResult<Value,Error>>,
}


impl<T> SimpleRpcFuture<T> {

    pub fn new(query: SimpleQuery, inner: T) -> Self { 
        let inner = Either::A(inner);
        let query = Some(query);
        SimpleRpcFuture { query, inner } 
    }

    pub fn succeed(query: SimpleQuery, inner: Value) -> Self {
        let inner = Either::B(future::ok(inner));
        let query = Some(query);
        SimpleRpcFuture { query, inner }
    }
}


impl<T> Future for SimpleRpcFuture<T> where T: Future<Item=Value,Error=Error> {

    type Item = SimpleRecord;

    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        match self.inner.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(value)) => {
                let query = self.query.take()
                    .expect("no polling past completion");
                let record = query.to_success(value);
                Ok(Async::Ready(record))
            },
            Err(error) => {
                let value = util::map_rpc_error(error)?;
                let query = self.query.take()
                    .expect("no polling past completion");
                let record = query.to_failure(value);
                Ok(Async::Ready(record))
            }
        }
    }
}

