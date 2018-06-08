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


