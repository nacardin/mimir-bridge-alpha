//! abi helpers for the `WorkerSet` contract
//!
use mimir_types::{
    Address,
    Bytes
};


/// function selector corresponding to `is_bound(address)`
const IS_BOUND: [u8;4] = [0xd1,0x06,0x83,0x9f];

/// encode calldata for `is_bound(address)`
pub fn is_bound(worker: &Address) -> Bytes {
    let mut calldata = Bytes::from(vec![0u8;36]);
    calldata[0..4].copy_from_slice(&IS_BOUND);
    calldata[16..].copy_from_slice(&worker);
    calldata
}


/// function selector corresponding to `set_bound(address)`
const SET_BOUND: [u8;4] = [0xd8,0xd9,0x8d,0xa1];

/// encode calldata for `set_bound(address)`
pub fn set_bound(worker: &Address) -> Bytes {
    let mut calldata = Bytes::from(vec![0u8;36]);
    calldata[0..4].copy_from_slice(&SET_BOUND);
    calldata[16..].copy_from_slice(&worker);
    calldata
}


/// function selector corresponding to `get_bound()`
const GET_BOUND: [u8;4] = [0xf1,0xa2,0xc4,0xbb];

/// encode calldata for `get_bound()`
pub fn get_bound() -> Bytes {
    let mut calldata = Bytes::from(vec![0u8;4]);
    calldata.copy_from_slice(&GET_BOUND);
    calldata
}


#[cfg(test)]
mod tests {
    use mimir_crypto::Keccak256;
    use abi::workerset;

    #[test]
    fn is_bound() {
        let hash = Keccak256::hash(b"is_bound(address)");
        let addr = [0u8;20].into();
        let data = workerset::is_bound(&addr);
        assert_eq!(&data[0..4],&hash[0..4]);
        assert_eq!(&data[16..],&addr[..]);
    }

    #[test]
    fn set_bound() {
        let hash = Keccak256::hash(b"set_bound(address)");
        let addr = [0u8;20].into();
        let data = workerset::set_bound(&addr);
        assert_eq!(&data[0..4],&hash[0..4]);
        assert_eq!(&data[16..],&addr[..]);
    }

    #[test]
    fn get_bound() {
        let hash = Keccak256::hash(b"get_bound()");
        let data = workerset::get_bound();
        assert_eq!(&data[0..4],&hash[0..4]);
    }
}
