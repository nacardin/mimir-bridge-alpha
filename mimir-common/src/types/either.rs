//! either one will do.
//!


/// two-state generic helper enum.
///
/// The `Either` enum provides a simple mechanism for representing
/// a value which may be in two possible states.
/// 
/// ```
/// # extern crate mimir_common;
/// # fn main() {
/// use mimir_common::types::Either;
/// 
/// let either: Either<String,Vec<u8>> = Either::A("hello".to_string());
/// 
/// let bytes: &[u8] = either.as_ref();
/// 
/// assert_eq!(bytes,&[104, 101, 108, 108, 111]);
/// # }
/// ```
///
#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[serde(untagged)]
pub enum Either<A,B> {
    /// variant a
    A(A),
    /// variant b
    B(B)
}


impl<A,B,T> AsRef<T> for Either<A,B> where A: AsRef<T>, B: AsRef<T>, T: ?Sized {

    fn as_ref(&self) -> &T {
        match *self {
            Either::A(ref a) => a.as_ref(),
            Either::B(ref b) => b.as_ref(),
        }
    }
}


impl<A,B,T> AsMut<T> for Either<A,B> where A: AsMut<T>, B: AsMut<T>, T: ?Sized {

    fn as_mut(&mut self) -> &mut T {
        match *self {
            Either::A(ref mut a) => a.as_mut(),
            Either::B(ref mut b) => b.as_mut(),
        }
    }
}



