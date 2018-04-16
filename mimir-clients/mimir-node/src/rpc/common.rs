use std::fmt::{self,Display};
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

impl<M,P> Display for Query<M,P> where M: Display, P: Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{ ")?;
        display_query(f,&self.method,&self.params)?;
        f.write_str(" }")
    }
}


impl SimpleQuery {

    /// lazily seed the default block parameter if applicable
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

    /// instantiate success variant
    pub fn success(method: M, params: Vec<P>, result: R) -> Self {
        let inner = SuccessRecord { method, params, result };
        Record::Success(inner)
    }

    /// instantiate failure variant
    pub fn failure(method: M, params: Vec<P>, error: E) -> Self {
        let inner = FailureRecord { method, params, error };
        Record::Failure(inner)
    }

    /// get reference to `method` field of inner record
    pub fn method(&self) -> &M {
        match *self {
            Record::Success(SuccessRecord { ref method, .. }) => method,
            Record::Failure(FailureRecord { ref method, .. }) => method,
        }
    }

    /// get reference to `params` field of inner record
    pub fn params(&self) -> &[P] {
        match *self {
            Record::Success(SuccessRecord { ref params, .. }) => params,
            Record::Failure(FailureRecord { ref params, .. }) => params,
        }
    }

    /// get reference to payload of inner record as `Result` type
    pub fn result(&self) -> Result<&R,&E> {
        match *self {
            Record::Success(SuccessRecord { ref result, .. }) => Ok(result),
            Record::Failure(FailureRecord { ref error, .. }) => Err(error),
        }
    }
}



impl<M,P,R,E> Display for Record<M,P,R,E> where M: Display, P: Display, R: Display, E: Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{ ")?;
        display_query(f,self.method(),self.params())?;
        f.write_str(", ")?;
        display_result(f,self.result())?;
        f.write_str(" }")
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


fn display_query<M: Display, P: Display>(f: &mut fmt::Formatter, method: &M, params: &[P]) -> fmt::Result {
    f.write_str("method: ")?;
    method.fmt(f)?;
    if params.len() > 0 {
        f.write_str(", params: [")?;
        for (i,param) in params.iter().enumerate() {
            if i > 0 { f.write_str(", ")?; }
            param.fmt(f)?;
        }
        f.write_str("]")?;
    }
    Ok(())
}


fn display_result<R: Display, E: Display>(f: &mut fmt::Formatter, result: Result<&R,&E>) -> fmt::Result {
    match result {
        Ok(result) => {
            f.write_str("result: ")?;
            result.fmt(f)
        },
        Err(error) => {
            f.write_str("error: ")?;
            error.fmt(f)
        },
    }
}

