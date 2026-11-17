//! Bounded shifted-reciprocal balanced divisor component, protocol12.
//! Existing polynomial/extension/core code is reused, not rederived.
// Preserve exact primary-source bytes. The context's from_fp needs &self.
#[rustfmt::skip]
#[allow(clippy::wrong_self_convention)]
#[path = "../vendor/fp2.rs"]
pub mod fp2;
#[rustfmt::skip]
#[path = "../vendor/fp_poly.rs"]
pub mod fp_poly;
pub mod balanced;
