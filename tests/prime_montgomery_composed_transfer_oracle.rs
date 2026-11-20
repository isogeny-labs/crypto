//! Exhaustive independent oracle for one composed Montgomery cover fixture.
//!
//! The independent arithmetic is completed before production objects are
//! constructed. This one-fixture test is not a catalog promotion, scaling
//! result, solver, or claim about the seven live proof-gated bindings.

const P: u8 = 7;
const CURVE_COEFFICIENTS: &[u8] = &[5, 0, 6, 0, 3, 0, 1];
const TARGET_A: u8 = 3;
const TARGET_B: u8 = 1;
const COVER_SHIFT: u8 = 1;
const EXTENSION_NON_SQUARE: u8 = 3;
const EXPECTED_SOURCE_FQ_POINTS: usize = 14;
const EXPECTED_SOURCE_FQ2_POINTS: usize = 58;
const EXPECTED_JACOBIAN_ORDER: usize = 120;
const EXPECTED_TARGET_ORDER: usize = 12;
const EXPECTED_PRINCIPAL_ZERO_DIVISORS: Option<usize> = Some(2608);
const EXPECTED_SUPPORT_COUNTERS: Option<(usize, usize, usize, usize)> =
    Some((21, 12, 1_989, 1_989));
const MONTGOMERY_MODEL: Option<(u8, u8)> = Some((3, 2));

#[path = "common/prime_split_sextic_oracle.rs"]
mod prime_split_sextic_oracle;
