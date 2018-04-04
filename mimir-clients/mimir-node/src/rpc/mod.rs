//! rpc types.
//!

mod method;
mod common;

pub use self::method::RpcMethod;
pub use self::common::{
    SimpleQuery,
    SimpleRecord,
    Query,
    Record,
};

