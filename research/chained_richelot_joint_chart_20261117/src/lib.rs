//! Frozen finite CT49 joint-chart normalization component, protocol13.
#[rustfmt::skip]
#[allow(clippy::wrong_self_convention)]
#[path = "../vendor/fp2.rs"]
pub mod fp2;
#[rustfmt::skip]
#[path = "../vendor/fp_poly.rs"]
pub mod fp_poly;

// The unchanged reused field has methods outside this component's interface.
#[allow(dead_code)]
mod joint;
pub use joint::run_unit;
