//! helpers for 'sealing' messages (adding certs).
//!

pub mod sealer;
pub mod util;

pub use self::sealer::Sealer;
pub use self::util::{
    oracle,
    notary,
    verify,
    route,
};

