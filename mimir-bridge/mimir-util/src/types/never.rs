//! no you neva gonna get it.
//!
use std::{fmt,error};



/// something that never happens.
///
/// the `Never` enum has no variants and as such cannot be instantiated.
/// this enum makes it easy to clearly represent states which cannot occur.
/// 
/// ```
/// # 
/// extern crate mimir_util;
/// # fn main() {
/// use mimir_util::types::Never;
///
/// fn always_ok() -> Result<u32,Never> { Ok(123) }
///
/// let _  = always_ok().unwrap(); // always safe to unwrap because `Err` variant cannot exist.
/// 
/// let _: Result<u32,&str> = always_ok().map_err(|e| e.into()); // can be converted to any type. 
/// # }
/// ```
///
#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub enum Never { }


impl Never {

    /// convert into any type.
    ///
    /// this function cannot ever actually be executed, but is
    /// useful for satisfying arbitrary type requirements.
    ///
    pub fn into<T>(self) -> T { match self { } }
}


impl<T: ?Sized> AsRef<T> for Never {

    fn as_ref(&self) -> &T { match *self { } }
}


impl<T: ?Sized> AsMut<T> for Never {

    fn as_mut(&mut self) -> &mut T { match *self { } }
}


impl fmt::Display for Never {

    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result { match *self { } }
}


impl error::Error for Never {

    fn description(&self) -> &str { match *self { } }
}

