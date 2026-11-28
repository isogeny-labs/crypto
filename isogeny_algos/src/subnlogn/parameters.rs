//! Literal integer parameter prescription from section 8.
//!
//! The exponents are the published values: epsilon=2^-75 and c=2^-56.
//! They are not enlarged to make native-sized examples enter the accelerated
//! branch. Consequently every input length representable by today's usize
//! derives d=1 and is explicitly a base case. Structural tests of the later
//! formulas force d only through a private helper, never through `select`.

use super::assembly::Parameters;
use super::tensor;
use crate::bigint::Big;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BaseCaseReason {
    TinyInput,
    DimensionBelowTwo { dimension: usize },
    EventualCondition(&'static str),
}

#[derive(Clone, Debug)]
pub enum Selection {
    BaseCase { reason: BaseCaseReason },
    Transform(Parameters),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    SizeOverflow,
    InvalidGeometry(&'static str),
    InsufficientPrimes { target: usize },
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "published parameter selection: {self:?}")
    }
}
impl std::error::Error for Error {}

fn big(value: usize) -> Big {
    Big::from_u64(value as u64)
}

fn ceil_log2(value: usize) -> usize {
    if value <= 1 {
        0
    } else {
        (usize::BITS - (value - 1).leading_zeros()) as usize
    }
}

/// Composition of integer square roots equals the integer 2^steps-th root.
/// Fixed points zero and one allow an exact early exit, not an approximation.
fn power_two_root(value: &Big, steps: usize) -> Big {
    let mut root = value.clone();
    for _ in 0..steps {
        if root <= Big::from_u64(1) {
            break;
        }
        root = root.isqrt();
    }
    root
}

fn ceil_fourth_root(value: &Big) -> Big {
    let root = value.isqrt().isqrt();
    let square = root.mul(&root);
    if square.mul(&square) < *value {
        root.add_small(1)
    } else {
        root
    }
}

fn as_usize(value: &Big) -> Result<usize, Error> {
    usize::try_from(value.to_u128().ok_or(Error::SizeOverflow)?).map_err(|_| Error::SizeOverflow)
}

#[derive(Debug)]
struct Geometry {
    b: usize,
    p: usize,
    d: usize,
    k: usize,
    total: usize,
    r: usize,
    ell: usize,
    targets: Vec<usize>,
    alpha: usize,
    gamma: usize,
}

// Separated from the literal dimension selection so unit tests can verify
// exact rounding and interval endpoints at manageable, explicitly forced d.
fn geometry(input_bits: usize, d: usize) -> Result<Geometry, Error> {
    let b = ceil_log2(input_bits);
    if b == 0 || d == 0 || d > b {
        return Err(Error::InvalidGeometry("require 1 <= d <= ceil(log2(n))"));
    }
    let p = b.checked_mul(6).ok_or(Error::SizeOverflow)?;
    let k = as_usize(&power_two_root(&big(d), 56))?;
    let lower_numerator = big(input_bits).mul_small(4);
    let (lower, remainder) = lower_numerator.divrem_small(b as u64);
    let lower = if remainder == 0 {
        lower
    } else {
        lower.add_small(1)
    };
    let lower = as_usize(&lower)?;
    let total = lower
        .checked_next_power_of_two()
        .ok_or(Error::SizeOverflow)?;
    let total_bits = total.trailing_zeros() as usize;
    let ell = total_bits.div_ceil(d);
    let r = 1usize
        .checked_shl(u32::try_from(ell).map_err(|_| Error::SizeOverflow)?)
        .ok_or(Error::SizeOverflow)?;
    let short = d
        .checked_mul(ell)
        .and_then(|v| v.checked_sub(total_bits))
        .ok_or(Error::SizeOverflow)?;
    if short >= d {
        return Err(Error::InvalidGeometry("at least one long axis is required"));
    }
    let mut targets = vec![r; d];
    for value in &mut targets[..short] {
        *value = r / 2;
    }
    let radicand = big(d).mul(&big(d)).mul(&big(b)).mul_small(12);
    let alpha = as_usize(&ceil_fourth_root(&radicand))?;
    let gamma = alpha
        .checked_mul(alpha)
        .and_then(|v| v.checked_mul(d))
        .and_then(|v| v.checked_mul(2))
        .ok_or(Error::SizeOverflow)?;
    // Exact endpoint checks include the strict upper end of [4n/b,8n/b).
    let scaled_total = big(total).mul_small(b as u64);
    if scaled_total < lower_numerator || scaled_total >= big(input_bits).mul_small(8) {
        return Err(Error::InvalidGeometry(
            "power-of-two volume left its prescribed interval",
        ));
    }
    Ok(Geometry {
        b,
        p,
        d,
        k,
        total,
        r,
        ell,
        targets,
        alpha,
        gamma,
    })
}

/// Inclusive integer endpoints of the strict-left prime interval.
fn prime_interval(target: usize, d: usize) -> Result<(usize, usize), Error> {
    if d < 2 {
        return Err(Error::InvalidGeometry("prime interval requires d >= 2"));
    }
    let two_d = big(d).mul_small(2);
    let four_d = big(d).mul_small(4);
    let lower = big(target)
        .mul(&two_d.sub_small(1))
        .divrem(&two_d)
        .0
        .add_small(1);
    let upper = big(target).mul(&four_d.sub_small(1)).divrem(&four_d).0;
    Ok((as_usize(&lower)?, as_usize(&upper)?))
}

// The manuscript's deterministic trial division, including the exact square
// boundary. This is setup work, not a probable-prime or randomized predicate.
fn trial_prime(value: usize) -> bool {
    if value < 2 {
        return false;
    }
    if value.is_multiple_of(2) {
        return value == 2;
    }
    let mut divisor = 3;
    while divisor <= value / divisor {
        if value.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }
    true
}

fn choose_primes(targets: &[usize]) -> Result<Vec<usize>, Error> {
    let d = targets.len();
    let mut primes = Vec::with_capacity(d);
    for &target in targets {
        let (lower, upper) = prime_interval(target, d)?;
        let mut candidate = lower.max(3);
        if candidate.is_multiple_of(2) {
            candidate = candidate.checked_add(1).ok_or(Error::SizeOverflow)?;
        }
        let mut chosen = None;
        while candidate <= upper {
            if !primes.contains(&candidate) && trial_prime(candidate) {
                chosen = Some(candidate);
                break;
            }
            let Some(next) = candidate.checked_add(2) else {
                break;
            };
            candidate = next;
        }
        primes.push(chosen.ok_or(Error::InsufficientPrimes { target })?);
    }
    Ok(primes)
}

fn eventual_condition(g: &Geometry) -> Option<&'static str> {
    if g.d < 2 || g.d > g.p {
        return Some("require 2 <= d <= p");
    }
    if g.ell < 2 || g.ell >= g.p || g.k < 6 || g.k >= g.ell {
        return Some("require 6 <= K <= ell-1 and 2 <= ell < p");
    }
    let d_ell = big(g.d).mul_small(g.ell as u64);
    if d_ell > big(g.p) || d_ell.mul_small(12) < big(g.p) || d_ell.mul_small(3) > big(g.p) {
        return Some("transform size lies outside the fixed comparison bands");
    }
    if tensor::published_guard(&big(g.d)) > big(g.p) {
        return Some("published exact layer guard Delta exceeds p");
    }
    if tensor::published_row_axes(&big(g.d)) >= big(g.d - 1) {
        return Some("reserved row axes consume the common transform axes");
    }
    let p_squared = big(g.p).mul(&big(g.p));
    // C_A=C_M+b_r+7=28/3<10. These sufficient integer inequalities
    // bound both additional packed-rectangle cost terms by a constant.
    if big(g.r) < p_squared.mul_small(1000) || Big::from_u64(1).shl(g.k) < p_squared.mul_small(1600)
    {
        return Some("packed-rectangle uniformity inequalities not yet satisfied");
    }
    let alpha2 = big(g.alpha).mul(&big(g.alpha));
    if g.alpha < 2 || alpha2 >= big(g.p) || big(g.gamma).mul_small(4) > big(g.b) {
        return Some("Gaussian precision and source-scale margins not yet satisfied");
    }
    // For x=r/2, log(x) >= (2/3)log2(x) and log(x)<log2(x).
    // These stronger, purely integral tests imply x>exp(8d) and
    // x/(8d log x)>d, the manuscript's prime-count hypotheses.
    let x_bits = g.ell - 1;
    if big(x_bits) <= big(g.d).mul_small(12)
        || big(g.r / 2)
            <= big(g.d)
                .mul(&big(g.d))
                .mul_small(8)
                .mul_small(x_bits as u64)
    {
        return Some("prime-interval supply inequalities not yet satisfied");
    }
    None
}

/// Select the literal published parameter family by exact integer operations.
/// A returned transform still requires `assembly::Plan::new`, whose finite
/// precision certificate validates every numerical recovery margin. This
/// selector does not execute the selected path and no exponent is relaxed.
pub fn select(input_bits: usize) -> Result<Selection, Error> {
    let b = ceil_log2(input_bits);
    if b == 0 {
        return Ok(Selection::BaseCase {
            reason: BaseCaseReason::TinyInput,
        });
    }
    let d = as_usize(&power_two_root(&big(b), 75))?;
    if d < 2 {
        return Ok(Selection::BaseCase {
            reason: BaseCaseReason::DimensionBelowTwo { dimension: d },
        });
    }
    let g = geometry(input_bits, d)?;
    if let Some(condition) = eventual_condition(&g) {
        return Ok(Selection::BaseCase {
            reason: BaseCaseReason::EventualCondition(condition),
        });
    }
    let source = choose_primes(&g.targets)?;
    let source_volume = source
        .iter()
        .try_fold(1usize, |a, &b| a.checked_mul(b).ok_or(Error::SizeOverflow))?;
    if big(source_volume).mul_small(2) <= big(g.total) || source_volume >= g.total {
        return Err(Error::InvalidGeometry(
            "source box must occupy strictly between half and all of target box",
        ));
    }
    let alpha4 = big(g.alpha).mul(&big(g.alpha));
    let alpha4 = alpha4.mul(&alpha4);
    for (&s, &t) in source.iter().zip(&g.targets) {
        if alpha4.mul_small((t - s) as u64) <= big(g.p).mul_small(s as u64) {
            return Err(Error::InvalidGeometry(
                "Gaussian separation inequality failed",
            ));
        }
    }
    Ok(Selection::Transform(Parameters {
        radix_bits: g.b,
        precision: g.p,
        source_lengths: source,
        target_lengths: g.targets,
        alpha: g.alpha,
        chunk_bits: g.k,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_lengths_are_explicit_literal_base_cases() {
        for n in [0, 1, 2, 3, 128, 1 << 20, usize::MAX] {
            assert!(matches!(select(n).unwrap(), Selection::BaseCase { .. }));
        }
        for b in 1..=usize::BITS as usize {
            assert_eq!(power_two_root(&big(b), 75), Big::from_u64(1));
        }
    }

    #[test]
    fn roots_and_logarithms_use_exact_endpoints() {
        assert_eq!(ceil_log2(1), 0);
        assert_eq!(ceil_log2(16), 4);
        assert_eq!(ceil_log2(17), 5);
        assert_eq!(power_two_root(&big(80), 2), big(2));
        assert_eq!(power_two_root(&big(81), 2), big(3));
        assert_eq!(ceil_fourth_root(&big(80)), big(3));
        assert_eq!(ceil_fourth_root(&big(81)), big(3));
        assert_eq!(ceil_fourth_root(&big(82)), big(4));
    }

    #[test]
    fn forced_dimension_geometry_is_only_a_structural_fixture() {
        let g = geometry(1 << 20, 2).unwrap();
        assert_eq!((g.b, g.p, g.total, g.r, g.ell), (20, 120, 1 << 18, 512, 9));
        assert_eq!(g.targets, vec![512, 512]);
        assert_eq!((g.alpha, g.gamma), (6, 144));
        assert!(eventual_condition(&g).is_some());
        let source = choose_primes(&g.targets).unwrap();
        assert_eq!(source, vec![389, 397]);
        let g = geometry(1 << 24, 3).unwrap();
        assert_eq!(g.targets, vec![128, 128, 256]);
        assert_eq!(choose_primes(&g.targets).unwrap(), vec![107, 109, 223]);
        assert!(matches!(
            select(1 << 24).unwrap(),
            Selection::BaseCase {
                reason: BaseCaseReason::DimensionBelowTwo { dimension: 1 }
            }
        ));
    }

    #[test]
    fn prime_interval_left_is_strict_and_right_inclusive() {
        assert_eq!(prime_interval(8, 2).unwrap(), (7, 7));
        assert_eq!(prime_interval(16, 2).unwrap(), (13, 14));
        assert!(trial_prime(2));
        assert!(trial_prime(389));
        for composite in [0, 1, 9, 25, 49, 121, 169] {
            assert!(!trial_prime(composite));
        }
        assert!(choose_primes(&[8, 8]).is_err());
    }
}
