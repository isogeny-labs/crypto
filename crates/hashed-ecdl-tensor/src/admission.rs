//! Host-independent audit of the numeric Appendix D.4 recursion guards.
//!
//! The live contractor uses `usize` because an executable graph must be held in
//! memory.  That representation cannot even name the first source-compatible
//! recursive instance on a 64-bit host.  This module mirrors the numeric guard
//! calculation with [`BigUint`] so the theorem boundary can be audited without
//! pretending that such a graph was allocated or timed.
//!
//! Passing this audit is necessary, not sufficient, for the contraction
//! theorem.  In particular it does not certify the source hypotheses about a
//! polynomial-size predicate description, predicate-entry evaluation cost,
//! construction cost, or aggregate polynomial space.

use crate::{invalid, BigUint, Result};
use num_traits::{One, ToPrimitive};

const EPSILON_DENOMINATOR: u64 = 128 * 1024;

/// Exact numeric parameters and guard decisions for one recursion node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumericRecursionAudit {
    /// Source size parameter (the number of edges after bipartite subdivision).
    pub size: BigUint,
    /// `ceil(log2(size))`.
    pub lambda: u64,
    /// Source leaf threshold `floor(size/floor(sqrt(lambda)))`.
    pub leaf_size: BigUint,
    /// Source degree cap `ceil(size^(9/10))`.
    pub delta: BigUint,
    /// `floor(epsilon*lambda/2^depth)` for epsilon `1/(128*1024)`.
    pub u: u64,
    /// Twice the sum of the scales through this depth.
    pub a: u64,
    /// Packet length `2^a`.
    pub length: BigUint,
    /// Pair-label array size `64^(2a)`.
    pub array: BigUint,
    /// Whether the supplied maximum degree is within `delta`.
    pub degree_hypothesis: bool,
    /// Whether the node is strictly above the leaf threshold.
    pub above_leaf: bool,
    /// Whether `u > 0`, `u <= a`, and `u <= size`.
    pub scale_guard: bool,
    /// Whether `size >= 2*array^2*delta`.
    pub variance_guard: bool,
    /// Whether `size >= length*array`.
    pub packing_guard: bool,
    /// Whether `2*length <= delta`.
    pub length_guard: bool,
    /// Whether all numeric source guards select the recursive branch.
    pub source_numeric_active: bool,
    /// Whether every value also fits the live host's `usize` representation.
    pub live_host_representable: bool,
}

fn floor_sqrt_u64(value: u64) -> u64 {
    if value < 2 {
        return value;
    }
    let mut lo = 1u64;
    let mut hi = 1u64 << value.ilog2().div_ceil(2);
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if mid <= value / mid {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    if hi <= value / hi {
        hi
    } else {
        lo
    }
}

fn floor_nth_root(value: &BigUint, exponent: u32) -> Result<BigUint> {
    if exponent == 0 {
        return Err(invalid("zeroth symbolic root"));
    }
    if exponent == 1 || value <= &BigUint::one() {
        return Ok(value.clone());
    }
    let width = value.bits();
    let shift = width.div_ceil(u64::from(exponent));
    let shift = usize::try_from(shift).map_err(|_| invalid("symbolic root width"))?;
    let mut estimate = BigUint::one() << shift;
    loop {
        let denominator = estimate.pow(exponent - 1);
        let next = ((&estimate * BigUint::from(exponent - 1)) + value / denominator)
            / BigUint::from(exponent);
        if next >= estimate {
            break;
        }
        estimate = next;
    }
    while (&estimate + BigUint::one()).pow(exponent) <= *value {
        estimate += BigUint::one();
    }
    while estimate.pow(exponent) > *value {
        estimate -= BigUint::one();
    }
    Ok(estimate)
}

fn ceil_nine_tenths(size: &BigUint) -> Result<BigUint> {
    let ninth_power = size.pow(9);
    let floor = floor_nth_root(&ninth_power, 10)?;
    if floor.pow(10) == ninth_power {
        Ok(floor)
    } else {
        Ok(floor + BigUint::one())
    }
}

/// Mirror the source D.4 numeric admission calculation using arbitrary-size
/// edge and degree integers.
///
/// This deliberately does not accept a Boolean saying that the predicate-cost
/// hypotheses hold: an opaque closure cannot make those hypotheses true by
/// assertion.  They require a separate typed-description/cost certificate.
pub fn audit_numeric_recursion(
    size: &BigUint,
    maximum_degree: &BigUint,
    depth: u32,
) -> Result<NumericRecursionAudit> {
    if size < &BigUint::from(2u8) {
        return Err(invalid("symbolic contraction size"));
    }
    let lambda = (size - BigUint::one()).bits();
    let sqrt = floor_sqrt_u64(lambda).max(1);
    let leaf_size = size / BigUint::from(sqrt);
    let delta = ceil_nine_tenths(size)?;
    let root_u = lambda / EPSILON_DENOMINATOR;
    let u = root_u.checked_shr(depth).unwrap_or(0);
    let mut sum = 0u64;
    for level in 0..=depth {
        sum = sum
            .checked_add(root_u.checked_shr(level).unwrap_or(0))
            .ok_or_else(|| invalid("symbolic recursive scale"))?;
    }
    let a = sum
        .checked_mul(2)
        .ok_or_else(|| invalid("symbolic recursive scale"))?;
    let a_index = usize::try_from(a).map_err(|_| invalid("symbolic packet exponent"))?;
    let array_exponent = a_index
        .checked_mul(12)
        .ok_or_else(|| invalid("symbolic packet array exponent"))?;
    let length = BigUint::one() << a_index;
    let array = BigUint::one() << array_exponent;

    let degree_hypothesis = maximum_degree <= &delta;
    let above_leaf = size > &leaf_size;
    let scale_guard = u > 0 && u <= a && &BigUint::from(u) <= size;
    let variance_guard = size >= &(BigUint::from(2u8) * &array * &array * &delta);
    let packing_guard = size >= &(&length * &array);
    let length_guard = BigUint::from(2u8) * &length <= delta.clone();
    let source_numeric_active = degree_hypothesis
        && above_leaf
        && scale_guard
        && variance_guard
        && packing_guard
        && length_guard;
    let live_host_representable = size.to_usize().is_some()
        && maximum_degree.to_usize().is_some()
        && leaf_size.to_usize().is_some()
        && delta.to_usize().is_some()
        && length.to_usize().is_some()
        && array.to_usize().is_some();

    Ok(NumericRecursionAudit {
        size: size.clone(),
        lambda,
        leaf_size,
        delta,
        u,
        a,
        length,
        array,
        degree_hypothesis,
        above_leaf,
        scale_guard,
        variance_guard,
        packing_guard,
        length_guard,
        source_numeric_active,
        live_host_representable,
    })
}

/// First even subdivided edge count at which the root scale `u` is nonzero.
///
/// Public Boolean networks are subdivided, so their source size is even.
pub fn first_public_nonzero_scale_size() -> BigUint {
    (BigUint::one() << 131_071usize) + BigUint::from(2u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_root_is_exact_on_and_between_powers() {
        for base in 0u32..40 {
            let power = BigUint::from(base).pow(10);
            assert_eq!(floor_nth_root(&power, 10).unwrap(), BigUint::from(base));
            if base != 0 {
                assert_eq!(
                    floor_nth_root(&(power + BigUint::from(base)), 10).unwrap(),
                    BigUint::from(base)
                );
            }
        }
    }

    #[test]
    fn first_scale_boundary_is_exact_and_not_host_representable() {
        let below = BigUint::one() << 131_071usize;
        let inactive = audit_numeric_recursion(&below, &BigUint::from(2u8), 0).unwrap();
        assert_eq!(inactive.lambda, 131_071);
        assert_eq!(inactive.u, 0);
        assert!(!inactive.source_numeric_active);

        let first = first_public_nonzero_scale_size();
        let active = audit_numeric_recursion(&first, &BigUint::from(2u8), 0).unwrap();
        assert_eq!(active.lambda, 131_072);
        assert_eq!(active.u, 1);
        assert_eq!(active.a, 2);
        assert_eq!(active.length, BigUint::from(4u8));
        assert_eq!(active.array, BigUint::one() << 24usize);
        assert!(active.source_numeric_active);
        assert!(!active.live_host_representable);
    }

    #[test]
    fn every_64_bit_public_size_has_zero_root_scale() {
        let audit =
            audit_numeric_recursion(&BigUint::from(u64::MAX - 1), &BigUint::from(2u8), 0).unwrap();
        assert!(audit.lambda <= 64);
        assert_eq!(audit.u, 0);
        assert!(!audit.source_numeric_active);
    }

    #[test]
    fn degree_hypothesis_fails_closed() {
        let first = first_public_nonzero_scale_size();
        let degree = first.clone();
        let audit = audit_numeric_recursion(&first, &degree, 0).unwrap();
        assert!(!audit.degree_hypothesis);
        assert!(!audit.source_numeric_active);
    }
}
