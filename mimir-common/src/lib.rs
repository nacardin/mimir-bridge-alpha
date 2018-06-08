//! common `mimir-bridge` helper utilities.
//!
#![warn(missing_docs)]
extern crate hex_core;
#[macro_use]
extern crate serde_derive;
pub extern crate serde; // used by macros
extern crate toml;
#[macro_use]
extern crate rand_derive;
extern crate rand;

// used in serialize/deserialize tests
#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[macro_use]
pub(crate) mod macros;
pub mod types;
pub mod util;


pub use types::Error;

