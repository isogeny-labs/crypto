//! The crypto library's binary-field modules, included unchanged.
#[path = "../../../../src/binary_ecc/f2m.rs"]
pub mod f2m;
#[path = "../../../../src/binary_ecc/operation_tally.rs"]
pub mod operation_tally;
pub use f2m::{F2mElement, IrreduciblePoly};
