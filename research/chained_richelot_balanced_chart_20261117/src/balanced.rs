//! Exact extracted original balanced arithmetic, followed by the new adapter.
use crate::{fp2::Fp2Ctx, fp_poly::FpPoly};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
type Result<T> = std::result::Result<T, String>;
type Key = (Vec<u64>, Vec<u64>, u8);

// This compiler input preserves the old method bodies byte for byte.
include!("../reused-core/balanced_core.rs.inc");

#[path = "shifted.rs"]
mod shifted;
pub use shifted::run_unit;
