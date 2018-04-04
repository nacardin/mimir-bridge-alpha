/// calldata encoders for common contract calls.
///
use mimir_types::Bytes;


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


#[cfg(test)]
mod tests {
    use mimir_crypto::Keccak256;
    use abi;

    #[test]
    fn lock_stake() {
        // get hash of function signature.
        let hash = Keccak256::hash(b"lock_stake()");
        let calldata = abi::lock_stake();
        assert_eq!(&calldata[..],&hash[0..4]);
    }
}
