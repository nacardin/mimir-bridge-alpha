//! `newtype` macro submodule
//!


/// implement basic newtype conversion traits.
///
/// takes the name of the newtype, the inner type, and the
/// desired dereference target.  An ethereum address, for example,
/// might be implemented like so:
///
/// ```
/// # #[macro_use] extern crate mimir_util;
/// # fn main() {
/// pub struct Address(pub [u8;20]);
/// 
/// newtype!(Address,[u8;20],[u8]);
///
/// // `From` and `Into` are implemented for inner type.
/// let mut addr: Address = [0u8;20].into();
/// 
/// // slice functionality available via deref coercions.
/// addr[..5].copy_from_slice(b"hello");
/// 
/// assert_eq!(&addr[..5],&[104, 101, 108, 108, 111]);
/// # }
/// ```
///
#[macro_export]
macro_rules! newtype {
    ( $name:ident, $inner:ty, $target:ty ) => {

        impl $name {

            /// consume `self`, returning inner value
            pub fn into_inner(self) -> $inner { self.0 }

            /// consume `self`, returning converted inner value
            pub fn into_other<T: From<$inner>>(self) -> T { self.0.into() }
        }


        impl From<$inner> for $name {

            fn from(inner: $inner) -> Self { $name(inner) }
        }


        impl Into<$inner> for $name {

            fn into(self) -> $inner { self.0 }
        }


        impl ::std::ops::Deref for $name {

            type Target = $target;

            fn deref(&self) -> &Self::Target { &self.0 }
        }


        impl ::std::ops::DerefMut for $name {

            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    }
}


