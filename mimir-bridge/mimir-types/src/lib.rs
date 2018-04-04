//! common `mimir-bridge` types.
//!
#![warn(missing_docs)]
extern crate mimir_crypto;
#[macro_use]
extern crate mimir_util;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate rand_derive;
extern crate rand;

// use `serde_json` for testing serialize/deserialize
// implementations.
#[cfg(test)]
#[macro_use]
extern crate serde_json;


pub mod primitive;
pub mod eth;

// re-export primitives
pub use primitive::{
    Bytes,
    U256,
    H256,
};

// re-export common ethereum types
pub use eth::{
    Signature,
    Address,
    Secret,
};

