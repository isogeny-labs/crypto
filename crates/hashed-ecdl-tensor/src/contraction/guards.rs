//! Symbolic Appendix D.4 parameter and guard certificates.
//!
//! The executable contraction stores edge counts in `usize`, while the first
//! nonzero source parameter needs `ceil(log2(s)) >= 2^17`.  This module works
//! with the logarithmic size parameter directly, using arbitrary-precision
//! integers and never materializing a graph with `s = 2^lambda` edges.

use crate::{invalid, BigUint, Result};
use num_traits::{One, Zero};
use serde::Serialize;

/// Denominator of epsilon in Appendix D.4 equation (D.4).
pub const EPSILON_DENOMINATOR: u32 = 128 * 1024;

/// One active-depth certificate for the model `s = 2^lambda`, `C = 1`.
#[derive(Clone, Debug, Serialize)]
pub struct GuardLevel {
    depth: usize,
    u: String,
    a: String,
    packet_length_exponent: String,
    packet_array_exponent: String,
    active_edge_lower_exponent: String,
    nonzero_u: bool,
    packing_load_guard: bool,
    packet_volume_guard: bool,
    packet_degree_guard: bool,
    u_at_least_lambda_four_fifths: bool,
    a_at_most_four_epsilon_lambda: bool,
    packet_length_at_most_s_one_thousandth: bool,
    packet_array_at_most_s_one_thousandth: bool,
    u_below_active_edge_lower_bound: bool,
    all_source_guards: bool,
    all_proof_invariants: bool,
}

/// Replayable symbolic transcript for all potentially active recursion depths.
#[derive(Clone, Debug, Serialize)]
pub struct GuardTranscript {
    schema: &'static str,
    model: &'static str,
    lambda: String,
    epsilon_denominator: u32,
    leaf_divisor: String,
    leaf_lower_exponent: String,
    root_u: String,
    native_usize_lambda_possible: bool,
    native_recursive_root_possible: bool,
    active_depth_count: usize,
    all_source_guards: bool,
    all_proof_invariants: bool,
    levels: Vec<GuardLevel>,
    interpretation: &'static str,
}

/// First passing power-of-two lambda found by a bounded exact scan.
#[derive(Clone, Debug, Serialize)]
pub struct GuardScan {
    schema: &'static str,
    searched_log2_lambda_min: usize,
    searched_log2_lambda_max: usize,
    first_passing_log2_lambda: Option<usize>,
    first_passing: Option<GuardTranscript>,
    claim_scope: &'static str,
}

fn decimal(value: &BigUint) -> String {
    value.to_str_radix(10)
}

fn ceil_log2(value: &BigUint) -> BigUint {
    if value <= &BigUint::one() {
        return BigUint::zero();
    }
    let floor = value.bits() - 1;
    let power = BigUint::one() << usize::try_from(floor).expect("resident integer bit width");
    BigUint::from(floor + u64::from(&power != value))
}

fn packing_load_guard(lambda: &BigUint, edge_exponent: &BigUint, a: &BigUint) -> bool {
    // Darr = 64^(2a) = 2^(12a).  The guard
    // 2^edge_exponent >= 2 * Darr^2 * ceil((2^lambda)^(9/10))
    // is equivalent to 9*lambda <= 10*(edge_exponent - 1 - 24*a)
    // whenever the exponent on the right is nonnegative.
    let overhead = BigUint::one() + BigUint::from(24u8) * a;
    if edge_exponent < &overhead {
        return false;
    }
    BigUint::from(9u8) * lambda <= BigUint::from(10u8) * (edge_exponent - overhead)
}

fn packet_degree_guard(lambda: &BigUint, a: &BigUint) -> bool {
    // This sufficient certificate is exact when true:
    // 2^(9*lambda/10) >= 2^(a+1), hence ceil(s^(9/10)) >= 2L.
    BigUint::from(9u8) * lambda >= BigUint::from(10u8) * (a + BigUint::one())
}

/// Certify the source guards and displayed proof invariants for `s = 2^lambda`.
///
/// `C = 1` is used, and every potentially active call is conservatively given
/// only the common lower bound from the leaf cutoff.  A `true` field is an
/// exact integer certificate.  This evaluates parameter admissibility only;
/// it does not validate the contraction identities or establish their cost.
pub fn power_of_two_size_guards(lambda: &BigUint) -> Result<GuardTranscript> {
    if lambda.is_zero() {
        return Err(invalid("symbolic lambda must be positive"));
    }
    let leaf_divisor = lambda.sqrt().max(BigUint::one());
    let leaf_log = ceil_log2(&leaf_divisor);
    if lambda < &leaf_log {
        return Err(invalid("symbolic leaf exponent"));
    }
    let leaf_lower_exponent = lambda - &leaf_log;
    let root_u = lambda >> 17usize;
    let native_usize_lambda_possible = lambda <= &BigUint::from(usize::BITS);
    let native_recursive_root_possible = native_usize_lambda_possible && !root_u.is_zero();

    let lambda_fourth = lambda.pow(4);
    let mut levels = Vec::new();
    let mut scale_sum = BigUint::zero();
    let mut depth = 0usize;
    let mut shrink = BigUint::one();
    while shrink < leaf_divisor {
        let u = &root_u >> depth;
        scale_sum += &u;
        let a = BigUint::from(2u8) * &scale_sum;
        let packet_array_exponent = BigUint::from(12u8) * &a;

        let nonzero_u = !u.is_zero();
        let load = packing_load_guard(lambda, &leaf_lower_exponent, &a);
        let volume = leaf_lower_exponent >= BigUint::from(13u8) * &a;
        let degree = packet_degree_guard(lambda, &a);
        let u_lower = u.pow(5) >= lambda_fourth;
        let taper = BigUint::from(EPSILON_DENOMINATOR) * &a <= BigUint::from(4u8) * lambda;
        let length_bound = BigUint::from(1000u16) * &a <= *lambda;
        let array_bound = BigUint::from(12_000u16) * &a <= *lambda;
        let u_bits = BigUint::from(u.bits());
        let u_bound = u.is_zero() || u_bits <= leaf_lower_exponent;
        let all_source_guards = nonzero_u && load && volume && degree && u_bound;
        let all_proof_invariants =
            all_source_guards && u_lower && taper && length_bound && array_bound;

        levels.push(GuardLevel {
            depth,
            u: decimal(&u),
            a: decimal(&a),
            packet_length_exponent: decimal(&a),
            packet_array_exponent: decimal(&packet_array_exponent),
            active_edge_lower_exponent: decimal(&leaf_lower_exponent),
            nonzero_u,
            packing_load_guard: load,
            packet_volume_guard: volume,
            packet_degree_guard: degree,
            u_at_least_lambda_four_fifths: u_lower,
            a_at_most_four_epsilon_lambda: taper,
            packet_length_at_most_s_one_thousandth: length_bound,
            packet_array_at_most_s_one_thousandth: array_bound,
            u_below_active_edge_lower_bound: u_bound,
            all_source_guards,
            all_proof_invariants,
        });

        depth = depth
            .checked_add(1)
            .ok_or_else(|| invalid("symbolic recursion depth"))?;
        shrink <<= 3usize;
    }
    let all_source_guards =
        !levels.is_empty() && levels.iter().all(|level| level.all_source_guards);
    let all_proof_invariants =
        !levels.is_empty() && levels.iter().all(|level| level.all_proof_invariants);
    Ok(GuardTranscript {
        schema: "tensor-appendix-d4-guards/v1",
        model: "s=2^lambda,C=1; active calls use the leaf-cutoff lower edge bound",
        lambda: decimal(lambda),
        epsilon_denominator: EPSILON_DENOMINATOR,
        leaf_divisor: decimal(&leaf_divisor),
        leaf_lower_exponent: decimal(&leaf_lower_exponent),
        root_u: decimal(&root_u),
        native_usize_lambda_possible,
        native_recursive_root_possible,
        active_depth_count: levels.len(),
        all_source_guards,
        all_proof_invariants,
        levels,
        interpretation:
            "parameter certificate only; no graph, theorem proof, runtime, or ECDLP claim",
    })
}

/// Scan `lambda = 2^x` and return the first transcript passing every invariant.
pub fn scan_power_of_two_lambdas(minimum: usize, maximum: usize) -> Result<GuardScan> {
    if minimum > maximum {
        return Err(invalid("symbolic scan interval"));
    }
    let mut first = None;
    for exponent in minimum..=maximum {
        let lambda = BigUint::one() << exponent;
        let transcript = power_of_two_size_guards(&lambda)?;
        if transcript.all_proof_invariants {
            first = Some((exponent, transcript));
            break;
        }
    }
    let (first_passing_log2_lambda, first_passing) = match first {
        Some((exponent, transcript)) => (Some(exponent), Some(transcript)),
        None => (None, None),
    };
    Ok(GuardScan {
        schema: "tensor-appendix-d4-guard-scan/v1",
        searched_log2_lambda_min: minimum,
        searched_log2_lambda_max: maximum,
        first_passing_log2_lambda,
        first_passing,
        claim_scope: "first passing power-of-two lambda in the scanned interval, not the exact global threshold",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_usize_can_never_reach_nonzero_root_u() {
        for lambda in [1u32, 2, 32, usize::BITS] {
            let transcript = power_of_two_size_guards(&BigUint::from(lambda)).unwrap();
            assert!(transcript.native_usize_lambda_possible);
            assert!(!transcript.native_recursive_root_possible);
            assert_eq!(transcript.root_u, "0");
        }
    }

    #[test]
    fn first_nonzero_root_parameter_is_not_a_full_recurrence_certificate() {
        let transcript = power_of_two_size_guards(&(BigUint::one() << 17usize)).unwrap();
        assert_eq!(transcript.root_u, "1");
        assert!(!transcript.native_usize_lambda_possible);
        assert!(!transcript.all_source_guards);
        assert!(!transcript.all_proof_invariants);
        assert!(transcript.levels.iter().any(|level| !level.nonzero_u));
    }

    #[test]
    fn proof_invariant_scan_has_a_stable_power_of_two_boundary() {
        let scan = scan_power_of_two_lambdas(0, 520).unwrap();
        let exponent = scan
            .first_passing_log2_lambda
            .expect("a power-of-two lambda passes in the registered interval");
        assert_eq!(exponent, 480);
        assert!(scan.first_passing.unwrap().all_proof_invariants);

        let previous = power_of_two_size_guards(&(BigUint::one() << (exponent - 1))).unwrap();
        assert!(previous.all_source_guards);
        assert!(!previous.all_proof_invariants);
        let failed: Vec<_> = previous
            .levels
            .iter()
            .filter(|level| !level.all_proof_invariants)
            .collect();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].depth, 79);
        assert!(!failed[0].u_at_least_lambda_four_fifths);
    }
}
