//! miscellaneous helpers
//!
pub mod toml;


use std::time::{SystemTime, UNIX_EPOCH};


/// get the current unix timestamp.  
///
/// # panics
///
/// will panic if called before jan 1 1970. try not to do that.
///
pub fn unix_time() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_secs()
}


/// hexadecimal conversion helpers.
///
/// *NOTE*: this module currently just reexports functionality of `hex-core`, but
/// will likely implement additional higher-level functionality in the future.
///
pub mod hex {
    pub use hex_core::{
        ParseHexError,
        from,
        into,
        into_upper,
        as_str,
        as_str_upper,
    };
}

