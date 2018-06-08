//! trustless routing helpers.
//!

mod proof;
mod tree;
mod node;


pub use self::proof::{
    ProofError,
    NodeHash,
    recover_root
};
pub use self::node::{
    Branch,
    Leaf
};


/// simple policy definitions
pub(crate) mod policy {
    use mimir_crypto::Keccak256;

    /// decide whether to turn left during tree traversal
    #[inline]
    pub fn turn_left(key: &[u8;32], val: &[u8;32]) -> bool { !turn_right(key,val) }

    /// decide whether to turn right during tree traversal
    #[inline]
    pub fn turn_right(key: &[u8;32], val: &[u8;32]) -> bool { key > val }

    /// hash a pair of values.
    #[inline]
    pub fn hash_pair(left: &[u8], right: &[u8]) -> [u8;32] {
        let mut hasher = Keccak256::default();
        hasher.absorb(left);
        hasher.absorb(right);
        hasher.finish()
    }
}

