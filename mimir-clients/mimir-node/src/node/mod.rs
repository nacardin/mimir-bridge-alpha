//! top-level node handle(s) to be used by worker clients.
//!


pub(crate) mod simple;
pub(crate) mod types;
pub(crate) mod util;


pub use self::simple::SimpleNode;
pub use self::types::SimpleRpcFuture;
pub use self::util::ipc;


