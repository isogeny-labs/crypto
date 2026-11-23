//! Exhaustive independent Picard-class oracle for the direct F5 split-sextic cover.

const P: u8 = 5;
const CURVE_COEFFICIENTS: &[u8] = &[1, 0, 0, 0, 0, 0, 1];
const TARGET_A: u8 = 0;
const TARGET_B: u8 = 1;
const COVER_SHIFT: u8 = 0;
const EXTENSION_NON_SQUARE: u8 = 2;
const EXPECTED_SOURCE_FQ_POINTS: usize = 6;
const EXPECTED_SOURCE_FQ2_POINTS: usize = 46;
const EXPECTED_JACOBIAN_ORDER: usize = 36;
const EXPECTED_TARGET_ORDER: usize = 6;
const EXPECTED_PRINCIPAL_ZERO_DIVISORS: Option<usize> = Some(705);
const EXPECTED_SUPPORT_COUNTERS: Option<(usize, usize, usize, usize)> = Some((18, 2, 113, 113));
const MONTGOMERY_MODEL: Option<(u8, u8)> = None;

#[path = "common/prime_split_sextic_oracle.rs"]
mod prime_split_sextic_oracle;
