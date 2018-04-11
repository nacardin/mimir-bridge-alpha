/// calldata encoders for common contract calls.
///
use mimir_types::{
    Address,
    Bytes,
};


/// selector for the `lock_stake()` function.
///
const LOCK_STAKE: [u8;4] = [0xac, 0xed, 0x34, 0x24];

/// encode calldata for the `lock_stake` contract call.
///
// the `lock_stake` function encodes no arguments, so this function just
// returns a function selector...
pub fn lock_stake() -> Bytes {
    let mut calldata = Bytes::from(vec![0u8;4]);
    calldata.copy_from_slice(&LOCK_STAKE);
    calldata
}


const BOUND_STATE: [u8;4] = [0x59, 0x93, 0x3d, 0x63];

pub fn bound_state(address: &Address) -> Bytes {
    let mut calldata = Bytes::from(vec![0u8;36]);
    calldata[0..4].copy_from_slice(&BOUND_STATE);
    calldata[16..].copy_from_slice(&address);
    calldata
}


#[cfg(test)]
mod tests {
    use mimir_crypto::Keccak256;
    use abi;
    // 59933d630000000000000000000000000000000000000000000000000000000000000000

    #[test]
    fn lock_stake() {
        // get hash of function signature.
        let hash = Keccak256::hash(b"lock_stake()");
        let calldata = abi::lock_stake();
        assert_eq!(&calldata[..],&hash[0..4]);
    }

    #[test]
    fn bound_state() {
        let hash = Keccak256::hash(b"bound_state(address)");
        let addr = [0xff;20].into();
        let calldata = abi::bound_state(&addr);
        assert_eq!(&calldata[0..4],&hash[0..4]);
        assert_eq!(&calldata[16..],&addr[..]);
    }
}
