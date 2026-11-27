//! Native M31 building blocks for the preclosed projective Semaev comparison.
//!
//! The quotient module implements exact local normal forms and finite row
//! spaces. The separate F4 prototype retains defining-ideal critical pairs
//! and distinguishes completed bases from resource-limited partial results.
//! Neither component infers a solving degree from generator degree.

pub const M31: u64 = 2_147_483_647;

pub mod quotient;
pub mod multiplicity;
pub mod systems;
pub mod f4;
#[cfg(test)]
mod system_certificate_tests;
#[cfg(test)]
mod projection_tests;
pub use quotient::{Error, Poly, Polynomial, Ring};

