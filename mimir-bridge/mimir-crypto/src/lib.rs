//! collection of simple cryptographic utilities.
//!
//!```
//!#
//! extern crate mimir_crypto;
//! extern crate rand;
//!# fn main() {
//! use mimir_crypto::secp256k1::{ecrecover,Signer};
//! use mimir_crypto::keccak256::Keccak256;
//! 
//! // get a signer instance
//! let signer: Signer = rand::random();
//! 
//! // hash a message for signing
//! let msg_hash = Keccak256::hash(b"hello world");
//!
//! // generate signature
//! let signature = signer.sign(&msg_hash);
//!
//! // recover the ethereum-style address of signer
//! let address = ecrecover(&msg_hash,&signature).unwrap();
//!
//! assert_eq!(address,signer.address());
//!# }
//!```
//!
#![warn(missing_docs)]

#[macro_use]
extern crate mimir_util;
#[macro_use]
extern crate lazy_static;
extern crate tiny_keccak;
extern crate secp256k1 as _secp256k1;
extern crate rand;


pub mod keccak256;
pub mod secp256k1;
pub mod traits;
pub mod util;


pub use keccak256::Keccak256;

pub use secp256k1::{
    Address,
    Signature,
    Secret,
};

pub use traits::{
    Hasher,
    Hashable,
    Signer,
};

pub use util::HashVoyeur;

