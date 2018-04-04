use serde_json::Value;
use rpc::RpcMethod;


/// simple query type.
pub type SimpleQuery = Query<RpcMethod,Value>;

/// simple record type.
pub type SimpleRecord = Record<RpcMethod,Value,Value,Value>;


/// generic rpc query.
///
#[derive(Default,Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct Query<M,P> {
    /// rpc method being called
    pub method: M,
    
    /// parameters supplied to method
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<P>,
}


impl<M,P> Query<M,P> {

    /// convert to success record.
    ///
    pub fn to_success<R,E>(self, result: R) -> Record<M,P,R,E> {
        let Query { method, params } = self;
        Record::success(method,params,result)
    }

    /// convert to failure record.
    ///
    pub fn to_failure<R,E>(self, error: E) -> Record<M,P,R,E> {
        let Query { method, params } = self;
        Record::failure(method,params,error)
    }
}


impl SimpleQuery {

    /// lazily seed the 
    pub fn seed_block<F>(&mut self, seed: F) where F: FnOnce() -> String {
        if let Some(index) = self.method.default_block_index() {
            match self.params.len() {
                len if len == index => {
                    let param = Value::String(seed());
                    self.params.push(param);
                },
                len if len == index + 1 => {
                    if let Some(tail) = self.params.last_mut() {
                        if tail.as_str().map(|s| s == "latest").unwrap_or(false) {
                            *tail = Value::String(seed());
                        }
                    }
                },
                _ => {
                    // params of unexpected size... do nothing.
                },
            }
        }
    }
}


/// generic record of rpc execution.
///
#[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[serde(untagged)]
pub enum Record<M,P,R,E> {
    Success(SuccessRecord<M,P,R>),
    Failure(FailureRecord<M,P,E>),
}


impl<M,P,R,E> Record<M,P,R,E> {

    pub fn success(method: M, params: Vec<P>, result: R) -> Self {
        let inner = SuccessRecord { method, params, result };
        Record::Success(inner)
    }

    pub fn failure(method: M, params: Vec<P>, error: E) -> Self {
        let inner = FailureRecord { method, params, error };
        Record::Failure(inner)
    }
}


/// generic record of successful rpc execution.
///
#[derive(Default,Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct SuccessRecord<M,P,R> {
    /// rpc method that was called
    pub method: M,

    /// parameters that were supplied
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<P>,
 
    /// successful outcome of rpc execution
    pub result: R,
}


/// generic record of failed rpc execution.
///
#[derive(Default,Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct FailureRecord<M,P,E> {
    /// rpc method that was called
    pub method: M,

    /// parameters that were supplied
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<P>,
 
    /// failed outcome of rpc execution
    pub error: E
}

