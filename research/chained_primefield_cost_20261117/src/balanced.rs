use crate::{fp2::Fp2Ctx, fp_poly::FpPoly};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
type Result<T> = std::result::Result<T, String>;
type Key = (Vec<u64>, Vec<u64>, u8);
include!("../reused-core/balanced_core.rs.inc");
#[path = "shifted.rs"]
mod shifted;
pub(crate) use shifted::{witnesses14, Bridge14};
