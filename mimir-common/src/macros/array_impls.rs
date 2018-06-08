//! `array_impls` macro submodule.
//!


/// implement common traits for array newtypes.
///
/// this macro is mostly useful for newtypes which contain
/// oversized arrays (preventing the use of `#[derive(..)]`):
///
/// ```
/// # #[macro_use] extern crate mimir_common;
/// # fn main() {
/// struct Foo([u8;66]);
///
/// newtype!(Foo,[u8;66],[u8]);
/// 
/// array_impls!(Foo => PartialEq, Eq, PartialOrd, Ord);
///
/// let (a,b) = (Foo([0xaa;66]),Foo([0xbb;66]));
/// 
/// assert!(a == a);
/// assert!(a != b);
/// assert!(a < b);
/// assert!(b >= a);
/// # }
/// ```
///
/// but it does work fine for smaller arrays as well:
///
/// ```
/// # #[macro_use] extern crate mimir_common;
/// # fn main() {
/// struct Bar([u8;3]);
///
/// newtype!(Bar,[u8;3],[u8]);
///
/// array_impls!(Bar => Debug);
/// 
/// assert_eq!("bar: Bar([1, 2, 3])",format!("bar: {:?}",Bar([1,2,3])));
/// # }
/// ```
///
#[macro_export]
macro_rules! array_impls {
    
    ( $name:ident => $( $trait:tt ),+ ) => {
        $(
            array_impls!($trait $name);
        )+
    };
    
    ( $name:ident => $( $trait:tt ,)+ ) => { array_impls!($name => $( $trait ),+); };
    
    (PartialEq $name:ident) => {
        impl PartialEq for $name {

            fn eq(&self, other: &Self) -> bool {
                let slice: &[_] = self.as_ref();
                PartialEq::eq(slice,other.as_ref())
            }

            fn ne(&self, other: &Self) -> bool {
                let slice: &[_] = self.as_ref();
                PartialEq::ne(slice,other.as_ref())
            }
        }
    };

    (Eq $name:ident) => { impl Eq for $name { } };

    (PartialOrd $name:ident) => {
        impl PartialOrd for $name {

            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                let slice: &[_] = self.as_ref();
                PartialOrd::partial_cmp(slice,other.as_ref())
            }
        }
    };

    (Ord $name:ident) => {
        impl Ord for $name {

            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                let slice: &[_] = self.as_ref();
                Ord::cmp(slice,other.as_ref())
            }
        }
    };

    (Debug $name:ident) => {
        impl ::std::fmt::Debug for $name {

            fn fmt(&self,f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let slice: &[_] = self.as_ref();
                write!(f,"{}({:?})",stringify!($name),slice)
            }
        }
    };

    (Hash $name:ident) => {
        impl ::std::hash::Hash for $name {

            fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
                let slice: &[_] = self.as_ref();
                slice.hash(state);
            }
        }
    };

    (Clone $name:ident) => {
        impl Clone for $name {
            
            fn clone(&self) -> Self {
                $name(Clone::clone(&self.0))
            }
        }
    };

    (Copy $name:ident) => {
        impl Copy for $name { }
    };
}

