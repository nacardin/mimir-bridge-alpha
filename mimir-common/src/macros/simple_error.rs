//! `simple_error` macro submodule.
//!


/// helper for building simple error types.
///
/// this macro assists in creating enums or unit structs to serve
/// as error types with static string descriptions.  The provided
/// strings will be used to generate both the `Display` format of
/// the erorr, and the output of the
/// [`std::error::Error::description`](https://doc.rust-lang.org/std/error/trait.Error.html#tymethod.description)
/// method:
///
/// ```
/// # #[macro_use] extern crate mimir_common;
/// # fn main() {
/// use std::error::Error;
/// 
/// // creates a pair of unit-struct style errors.
/// simple_error!(
///     ErrorOne => "first error",
///     ErrorTwo => "second error",
/// );
/// 
/// assert_eq!(ErrorTwo.description(),"second error");
/// 
/// // crates a single enum style error.
/// simple_error!(
///     ErrorKind,
///     VariantOne => "first error kind",
///     VariantTwo => "second error kind"
/// );
/// 
/// assert_eq!(format!("{}",ErrorKind::VariantOne),"first error kind");
/// # }
/// ```
///
#[macro_export]
macro_rules! simple_error {
    // build one or more unit structs with specified name & message
    ( $($name:ident => $msg: expr),+ ) => {
        $(
            simple_unit!($name => $msg);

            impl ::std::error::Error for $name {

                fn description(&self) -> &str { self.as_ref() }
            }
        )+
    };

    ( $($name:ident => $msg: expr,)+ ) => {
        simple_error!( $( $name => $msg ),+ );
    };
    // builds an enum with one or more variants, each with a custom message
    ( $name:ident, $doc:expr, $( $var:ident => $msg:expr ),+ ) => {

        simple_unit!($name, $doc, $( $var => $msg ),+ );

        impl ::std::error::Error for $name {

            fn description(&self) -> &str { self.as_ref() }
        }
    };

    ( $name: ident, $doc: expr, $($var:ident => $msg:expr,)+ ) => {
        simple_error!( $name, $doc, $( $var => $msg ),+ );
    };

    ( $name:ident, $( $var:ident => $msg:expr ),+ ) => {
        simple_error!($name, "simple error enum", $( $var => $msg ),+);
    };
    
    ( $name: ident, $($var:ident => $msg:expr,)+ ) => {
        simple_error!( $name, $( $var => $msg ),+ );
    };
}


#[cfg(test)]
mod tests {

    simple_error!(
        Error,
        VarOne => "the first disaster has struck",
        VarTwo => "the second calamity is upon us"
    );

    simple_error!(
        FooError => "foo went horribly wrong",
        BarError => "bar has seen better days",
    );
    
    #[test]
    fn variant_errors() {
        assert_eq!(
            format!("{}",Error::VarOne),
            "the first disaster has struck"
            );
        assert_eq!(
            ::std::error::Error::description(&Error::VarTwo),
            "the second calamity is upon us"
        )
    }

    #[test]
    fn unit_errors() {
        assert_eq!(
            format!("{}",FooError),
            "foo went horribly wrong"
            );
        assert_eq!(
            ::std::error::Error::description(&BarError),
            "bar has seen better days"
            );
    }
}
