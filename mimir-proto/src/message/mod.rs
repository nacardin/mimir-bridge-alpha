//! Logic and definitions related to the mimir-bridge messaging protocol.
//!
//! This module provides the core primitives necessary to generate and
//! verify messages passing through the mimir-bridge.  A set of constants
//! are also provided dictating the ordering and members of the
//! oracle and verify circuits.
pub mod types;
pub mod step;
pub mod cert;


pub use self::types::{
    Request,
    Payload,
    Message,
};
pub use self::step::STEP;


/// number of validator destinations specified by a router.
pub const DESTS: usize = 2;


/// number of steps in an oracle circuit.
pub(crate) const OSTEPS: usize = 4;


/// number of steps in a verify circuit.
pub(crate) const VSTEPS: usize = 6;


/// ordered array of steps expected in an oracle circuit.
pub(crate) const OCIRCUIT: [STEP;OSTEPS] = [
    STEP::BLIND,
    STEP::ORACLE,
    STEP::NOTARY,
    STEP::CLEAR
];


/// ordered array of steps expected in a verify circuit.
pub(crate) const VCIRCUIT: [STEP;VSTEPS] = [
    STEP::BLIND,
    STEP::ROUTE,
    STEP::VERIFY,
    STEP::VERIFY,
    STEP::NOTARY,
    STEP::CLEAR
];



#[cfg(test)]
mod tests {
    use message::{STEP,VCIRCUIT,DESTS};

    /// ensure that the `DESTS` constant corresponds
    /// to the actual expected destination count for
    /// a verify circuit.
    #[test]
    fn dest_count() {
        let count = VCIRCUIT.iter().fold(0,|sum,cert| {
            match *cert {
                STEP::VERIFY => { sum + 1 },
                _ => { sum }
            }
        });
        assert_eq!(DESTS,count);
    }
}



