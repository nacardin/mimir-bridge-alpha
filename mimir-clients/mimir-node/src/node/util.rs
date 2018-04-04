use tokio_core::reactor::Handle;
use web3::error::{Error,ErrorKind};
use web3::transports::Ipc;
use serde_json::Value;
use node::SimpleNode;
use std::path::Path;


/// instantiate an ipc based node interface.
///
pub fn ipc<P: AsRef<Path>>(path: P, handle: &Handle) -> Result<SimpleNode<Ipc>,Error> {
    let ipc = Ipc::with_event_loop(path,handle)?;
    let node = SimpleNode::new(ipc);
    Ok(node)
}


/// remap rpc error variants.
///
/// converts the rpc variant of `web3::error::Error` to an instance
/// of `serde_json::Value`.
/// 
pub fn map_rpc_error(error: Error) -> Result<Value,Error> {
    match error {
        Error(ErrorKind::Rpc(rpc_error),..) => {
            if let Some(data) = rpc_error.data {
                let value = json!({
                    "code": rpc_error.code.code(),
                    "message": rpc_error.message,
                    "data": data
                });
                Ok(value)
            } else {
                let value = json!({
                    "code": rpc_error.code.code(),
                    "message": rpc_error.message
                });
                Ok(value)
            }
        },
        other => Err(other),
    }
}

