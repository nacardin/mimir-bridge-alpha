/// websocket based transport helpers.
///
pub mod client;
pub mod server;

pub use websocket::WebSocketError as Error;

/// websocket protocol header.
///
const PROTOCOL: &'static str = "mimir-bridge-0";

