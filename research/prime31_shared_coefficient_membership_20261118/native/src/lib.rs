//! Exact, typed experiment infrastructure for PR #4692.
//!
//! This crate deliberately keeps two predicates separate:
//! selector SAT accepts eight slot-specific point domains, while the frozen
//! RR9 remainder tower accepts only one shared x-coordinate support.  Nothing
//! in this crate constitutes a speedup claim.

pub mod algebra;
pub mod circuit;
pub mod ir;
pub mod panel;
pub mod rr9;

pub use algebra::{AffinePoint, Curve};
pub use circuit::{BooleanEncoding, EncodingStatistics, SelectorWitness};
pub use ir::{LoweringError, RelationSpec, SlotDomain};

pub const PRIME31_P: u64 = 2_147_483_647;
pub const PRIME31_A: u64 = 847_514_846;
pub const PRIME31_B: u64 = 2_087_444_724;
pub const PRIME31_N: u64 = 2_147_446_831;
pub const PRIME31_GENERATOR: AffinePoint = AffinePoint {
    x: 625_037_715,
    y: 2_100_627_251,
};
pub const W8_ODD_WEIGHTS: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];

pub fn prime31_curve() -> Curve {
    Curve {
        p: PRIME31_P,
        a: PRIME31_A,
        b: PRIME31_B,
        order: Some(PRIME31_N),
    }
}
