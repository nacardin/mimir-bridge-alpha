//! core oracle definition.
//!

pub(crate) mod options;
pub(crate) mod config;
pub(crate) mod simple;
pub(crate) mod types;
pub(crate) mod error;
pub(crate) mod util;

pub use self::options::Options;
pub use self::config::Config;
pub use self::simple::SimpleOracle;
pub use self::error::OracleError;
pub use self::types::SimpleOracleFuture;
pub use self::util::OracleOp;

