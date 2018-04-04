//! misc helper types.
//!

mod either;
mod never;


pub use self::either::Either;
pub use self::never::Never;


simple_error!(
    ParseUnitError => "unable to parse string as unit type",
);

