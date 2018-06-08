
/// helper for building simple unit types.
///
/// This macro allows for quick construction of structs and
/// macros with no associated data.  Each struct/variant
/// requires an associated string value, which is used
/// to implement `FromStr`, `AsRef<str>`, `Display`, and
/// `Serialize`/`Deserialize`.
///
/// ```
/// # #[macro_use] extern crate mimir_common;
/// # fn main() {
/// 
/// // creates a pair of unit-structs.
/// simple_unit!(
///     UnitOne => "first unit",
///     UnitTwo => "second unit",
/// );
/// 
/// assert_eq!(UnitTwo.as_ref(),"second unit");
/// 
/// // crates an enum of unit variants.
/// simple_error!(
///     UnitKind,
///     VariantOne => "first unit kind",
///     VariantTwo => "second unit kind"
/// );
/// 
/// assert_eq!(format!("{}",UnitKind::VariantOne),"first unit kind");
/// # }
/// ```
///
#[macro_export]
macro_rules! simple_unit {
    // build one or more unit structs with specified name & message
    ( $($name:ident => $msg: expr),+ ) => {
        $(
            #[doc = $msg]
            #[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
            pub struct $name;

            impl ::std::str::FromStr for $name {

                type Err = $crate::types::Error;

                fn from_str(s: &str) -> Result<Self,Self::Err> {
                    if s == $msg {
                        Ok($name)
                    } else {
                        let message = concat!("did not match expected unit `",stringify!($name),"`");
                        Err($crate::types::Error::from(message))
                    }
                }
            }

            impl AsRef<str> for $name {

                fn as_ref(&self) -> &'static str { $msg }
            }
 
            impl ::std::fmt::Display for $name {

                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    f.write_str(self.as_ref())
                }
            }

            impl $crate::serde::Serialize for $name {

                fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where S: $crate::serde::Serializer {
                    serializer.serialize_str(self.as_ref())
                }
            }

            impl<'de> $crate::serde::Deserialize<'de> for $name {

                fn deserialize<D>(deserializer: D) -> Result<Self,D::Error> where D: $crate::serde::Deserializer<'de> {
                    let target: $crate::types::Either<&str,String> = $crate::serde::Deserialize::deserialize(deserializer)?;
                    let target_str: &str = target.as_ref();
                    target_str.parse().map_err(<D::Error as $crate::serde::de::Error>::custom)
                }
            }
        )+
    };

    ( $($name:ident => $msg: expr,)+ ) => {
        simple_error!( $( $name => $msg ),+ );
    };

    // builds an enum with one or more variants, each with a custom message
    ( $name:ident, $doc:expr, $( $var:ident => $msg:expr ),+ ) => {
        #[doc = $doc]
        #[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
        pub enum $name {
            $(
                #[doc = $msg]
                $var

            ),+
        }

        impl ::std::str::FromStr for $name {

            type Err = $crate::types::Error;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                match s {
                    $(
                        $msg => Ok($name::$var)
                    ,)+
                    _ => {
                        let message = concat!("did not match any variant of enum `",stringify!($name),"`");
                        Err($crate::types::Error::from(message))
                    }
                }
            }
        }

        impl AsRef<str> for $name {

            fn as_ref(&self) -> &'static str {
                match *self {
                    $(
                        $name::$var => $msg
                    ),+
                }
            }
        }

        impl ::std::fmt::Display for $name {

            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(self.as_ref())
            }
        }

        impl $crate::serde::Serialize for $name {

            fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where S: $crate::serde::Serializer {
                serializer.serialize_str(self.as_ref())
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $name {

            fn deserialize<D>(deserializer: D) -> Result<Self,D::Error> where D: $crate::serde::Deserializer<'de> {
                let target: $crate::types::Either<&str,String> = $crate::serde::Deserialize::deserialize(deserializer)?;
                let target_str: &str = target.as_ref();
                target_str.parse().map_err(<D::Error as $crate::serde::de::Error>::custom)
            }
        }
    };

    ( $name: ident, $doc: expr, $($var:ident => $msg:expr,)+ ) => {
        simple_error!( $name, $doc, $( $var => $msg ),+ );
    };

    ( $name:ident, $( $var:ident => $msg:expr ),+ ) => {
        simple_error!($name, "simple unit enum", $( $var => $msg ),+);
    };
    
    ( $name: ident, $($var:ident => $msg:expr,)+ ) => {
        simple_error!( $name, $( $var => $msg ),+ );
    };
}


