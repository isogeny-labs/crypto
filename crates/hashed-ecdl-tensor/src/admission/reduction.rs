//! Exact finite ledgers for the hashed-factor-base ECDL reduction.
//!
//! These bounds price factor-base enumeration, relation attempts, contraction
//! self-reduction calls, and dense linear algebra.  The asymptotic exponent
//! surrogates deliberately omit the unknown constants in the external tensor
//! theorem and therefore are not runtime or security estimates.

use crate::arithmetic::floor_root;
use crate::{invalid, BigUint, Meter, Result};
use num_traits::{One, Zero};

/// Exact finite and exponent-scale bounds for one theorem-parameterized trial.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReductionCostAudit {
    /// Prime-field modulus bit length `b`.
    pub field_bits: u64,
    /// Prime subgroup order bit length.
    pub subgroup_bits: u64,
    /// `ceil(log2(b))`.
    pub log_field_bits: u64,
    /// `ceil((log2(b))^(1/4))`, with the source minimum two.
    pub slots: u64,
    /// `max(ceil(r^(1/m)),10m)`, the target membership mass.
    pub target_membership_mass: BigUint,
    /// Safe threshold bound using the endpoint `s<=4p`.
    pub threshold_upper_bound: BigUint,
    /// Safe column bound `N<=m*tau`.
    pub columns_upper_bound: BigUint,
    /// Factor-base decoding candidates in one trial.
    pub enumeration_candidates_upper_bound: BigUint,
    /// Target attempts in one trial.
    pub relation_attempts_upper_bound: BigUint,
    /// Interface bits exposed by one decomposition network.
    pub interface_bits_upper_bound: BigUint,
    /// Contraction calls made by deterministic prefix self-reduction in a trial.
    pub contraction_calls_per_trial_upper_bound: BigUint,
    /// Expected contraction calls using the theorem's at-most-six trial bound.
    pub expected_contraction_calls_upper_bound: BigUint,
    /// Dense field operations for one `(N+1)`-row elimination.
    pub dense_elimination_field_operations_bound: BigUint,
    /// Corresponding schoolbook bit-operation envelope.
    pub dense_elimination_bit_operations_bound: BigUint,
    /// Numerator of the per-trial success lower bound.
    pub trial_success_lower_numerator: BigUint,
    /// Denominator of the per-trial success lower bound.
    pub trial_success_lower_denominator: BigUint,
    /// Whether that exact lower bound is greater than one sixth.
    pub trial_success_exceeds_one_sixth: bool,
    /// Bit exponent of the finite expected contraction-call bound.
    pub expected_contraction_calls_bits: u64,
    /// Bit exponent of the finite dense-elimination bound.
    pub dense_elimination_work_bits: u64,
    /// Unit-constant surrogate for one contraction exponent.
    pub contraction_shape_exponent_surrogate: BigUint,
    /// Solver-call count plus contraction exponent surrogate.
    pub relation_phase_exponent_surrogate: BigUint,
    /// Theorem-form dense-linear-algebra exponent surrogate.
    pub linear_algebra_exponent_surrogate: BigUint,
}

fn ceil_div(numerator: &BigUint, denominator: &BigUint) -> BigUint {
    (numerator + denominator - BigUint::one()) / denominator
}

fn ceil_log2_u64(value: u64) -> u64 {
    if value <= 1 {
        return 0;
    }
    u64::from(u64::BITS - (value - 1).leading_zeros())
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

fn theorem_slots(field_bits: u64) -> u64 {
    let logarithm = ceil_log2_u64(field_bits.max(2));
    let mut slots = 2u64;
    while slots.pow(4) < logarithm {
        slots += 1;
    }
    slots
}

/// Audit the finite reduction schedule for a prime field and prime subgroup.
///
/// The coverage and trial constants are those of Lemmas 10--13 in the pinned
/// Bochum v0.2 source.  For a fixed cofactor, the same asymptotic reduction
/// applies after subgroup projection and filtering.  A bounded native solve
/// may still return a resource-limit error after its configured trial cap.
pub fn audit_reduction_cost(
    field_modulus: &BigUint,
    subgroup_order: &BigUint,
    meter: &mut Meter,
) -> Result<ReductionCostAudit> {
    if field_modulus <= &BigUint::from(3u8)
        || subgroup_order < &BigUint::from(100u8)
        || subgroup_order.is_zero()
    {
        return Err(invalid("reduction audit parameters"));
    }
    let field_bits = field_modulus.bits();
    let subgroup_bits = subgroup_order.bits();
    let log_field_bits = ceil_log2_u64(field_bits);
    let slots = theorem_slots(field_bits);
    let slots_usize =
        usize::try_from(slots).map_err(|_| invalid("reduction slot representation"))?;
    let root = floor_root(subgroup_order, slots_usize, meter)?;
    let root = if root.pow(slots as u32) < *subgroup_order {
        root + BigUint::one()
    } else {
        root
    };
    let target_membership_mass = root.max(BigUint::from(10u8) * BigUint::from(slots));
    let n = subgroup_order - BigUint::one();
    if target_membership_mass > n {
        return Err(invalid("reduction coverage range"));
    }
    let hash_prime_upper = BigUint::from(4u8) * field_modulus;
    let threshold_upper_bound = ceil_div(&(&hash_prime_upper * &target_membership_mass), &n);
    let columns_upper_bound = BigUint::from(slots) * &threshold_upper_bound;
    let enumeration_candidates_upper_bound = columns_upper_bound.clone();
    let required = &columns_upper_bound + BigUint::one();
    let relation_attempts_upper_bound = BigUint::from(20u8) * &required;
    let interface_bits_upper_bound = BigUint::from(slots) * BigUint::from(2 * field_bits + 1);
    let contraction_calls_per_trial_upper_bound =
        &relation_attempts_upper_bound * (&interface_bits_upper_bound + BigUint::one());
    let expected_contraction_calls_upper_bound =
        BigUint::from(6u8) * &contraction_calls_per_trial_upper_bound;
    let dense_elimination_field_operations_bound = required.pow(3);
    let dense_elimination_bit_operations_bound =
        &dense_elimination_field_operations_bound * BigUint::from(field_bits).pow(2);

    // Good seed >=1/4, enough rows conditional on a good seed >=3/4, and
    // nonzero extraction coefficient exactly 1-1/r.
    let trial_success_lower_numerator = BigUint::from(3u8) * &n;
    let trial_success_lower_denominator = BigUint::from(16u8) * subgroup_order;
    let trial_success_exceeds_one_sixth =
        BigUint::from(6u8) * &trial_success_lower_numerator > trial_success_lower_denominator;

    let sqrt_log = floor_sqrt_u64(log_field_bits).max(1);
    let network_scale = BigUint::from(slots) * BigUint::from(field_bits);
    let contraction_shape_exponent_surrogate = ceil_div(&network_scale, &BigUint::from(sqrt_log));
    let solver_call_exponent =
        BigUint::from(subgroup_bits.div_ceil(slots) + 2 * ceil_log2_u64(slots) + log_field_bits);
    let relation_phase_exponent_surrogate =
        solver_call_exponent + &contraction_shape_exponent_surrogate;
    let linear_algebra_exponent_surrogate = BigUint::from(
        (3 * subgroup_bits).div_ceil(slots) + 3 * ceil_log2_u64(slots) + 2 * log_field_bits,
    );

    Ok(ReductionCostAudit {
        field_bits,
        subgroup_bits,
        log_field_bits,
        slots,
        target_membership_mass,
        threshold_upper_bound,
        columns_upper_bound,
        enumeration_candidates_upper_bound,
        relation_attempts_upper_bound,
        interface_bits_upper_bound,
        expected_contraction_calls_bits: expected_contraction_calls_upper_bound.bits(),
        contraction_calls_per_trial_upper_bound,
        expected_contraction_calls_upper_bound,
        dense_elimination_work_bits: dense_elimination_bit_operations_bound.bits(),
        dense_elimination_field_operations_bound,
        dense_elimination_bit_operations_bound,
        trial_success_lower_numerator,
        trial_success_lower_denominator,
        trial_success_exceeds_one_sixth,
        contraction_shape_exponent_surrogate,
        relation_phase_exponent_surrogate,
        linear_algebra_exponent_surrogate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Limits;

    fn curve25519_order() -> BigUint {
        (BigUint::one() << 252usize)
            + BigUint::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap()
    }

    #[test]
    fn curve25519_finite_reduction_ledger_is_exact() {
        let field = (BigUint::one() << 255usize) - BigUint::from(19u8);
        let mut meter = Meter::new(Limits::default());
        let audit = audit_reduction_cost(&field, &curve25519_order(), &mut meter).unwrap();
        assert_eq!(audit.field_bits, 255);
        assert_eq!(audit.subgroup_bits, 253);
        assert_eq!(audit.log_field_bits, 8);
        assert_eq!(audit.slots, 2);
        assert_eq!(audit.expected_contraction_calls_bits, 149);
        assert_eq!(audit.dense_elimination_work_bits, 412);
        assert!(audit.trial_success_exceeds_one_sixth);
        assert_eq!(audit.contraction_shape_exponent_surrogate, 255u16.into());
        assert_eq!(audit.relation_phase_exponent_surrogate, 392u16.into());
        assert_eq!(audit.linear_algebra_exponent_surrogate, 399u16.into());
    }

    #[test]
    fn slot_boundary_and_invalid_inputs_are_distinguished() {
        assert_eq!(theorem_slots(255), 2);
        assert_eq!(theorem_slots(65_536), 2);
        assert_eq!(theorem_slots(65_537), 3);
        let mut meter = Meter::new(Limits::default());
        assert!(audit_reduction_cost(&3u8.into(), &101u8.into(), &mut meter).is_err());
        assert!(audit_reduction_cost(&101u8.into(), &99u8.into(), &mut meter).is_err());
    }
}
