//! Frozen operational joint-chart divisor map14; no new field/group engine.
#![forbid(unsafe_code)]
#[rustfmt::skip]
#[allow(clippy::wrong_self_convention)]
#[path = "../vendor/fp2.rs"]
pub mod fp2;
#[rustfmt::skip]
#[path = "../vendor/fp_poly.rs"]
pub mod fp_poly;
#[allow(dead_code)]
pub mod balanced;
pub mod reference {
    pub use crate::fp2::{Fp2, Fp2Ctx};
    pub use crate::fp_poly::FpPoly;
}
#[allow(dead_code)]
mod nonsplit;
