//! hexadecimal conversion helpers.
//!
//! *NOTE*: this module currently just reexports functionality of `hex-core`, but
//! will likely implement additional higher-level functionality in the future.
//!

pub use hex_core::{
    ParseHexError,
    from,
    into,
    into_upper,
    as_str,
    as_str_upper,
};


