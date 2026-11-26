//! Standalone verification of an implicit strict-recursion history schedule.
//!
//! This module consumes serialized scalar data and exact integers. It does not
//! import graph contraction, packing, factorization, or the producer-side
//! dimension audit.

use crate::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

pub(crate) const SCHEMA: u32 = 1;
const EPSILON_DENOMINATOR: u64 = 128 * 1024;
const MINIMUM_LAMBDA: u64 = 1 << 26;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DepthCertificate {
    pub(crate) depth: u32,
    pub(crate) shrink_factor: BigUint,
    pub(crate) u: u64,
    pub(crate) prior_a: u64,
    pub(crate) current_a: u64,
    pub(crate) inherited_lhs: BigUint,
    pub(crate) inherited_rhs_exponent: u64,
    pub(crate) evaluation_lhs: BigUint,
    pub(crate) evaluation_rhs_exponent: u64,
    pub(crate) geometric_scale_comparison_holds: bool,
}

/// Schema-versioned exact transcript for the symbolic strict-history audit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    pub(crate) schema: u32,
    pub(crate) lambda: u64,
    pub(crate) leaf_divisor: u64,
    pub(crate) root_u: u64,
    pub(crate) scale_floor: u64,
    pub(crate) inherited_precision_constant: u64,
    pub(crate) normalized_precision_constant: u64,
    pub(crate) depths: Vec<DepthCertificate>,
    pub(crate) local_child_exponent_constant: u64,
    pub(crate) path_sum_constant: u64,
    pub(crate) active_tree_exponent_constant: u64,
    pub(crate) leaf_exponent_constant: u64,
    pub(crate) leaf_scale_absorbed: bool,
    pub(crate) packet_entry_exponent_constant: u64,
    pub(crate) packet_entry_polynomial_factor: BigUint,
    pub(crate) packet_entry_absorbed: bool,
    pub(crate) total_root_u_exponent_constant: u64,
    pub(crate) total_root_log_exponent_constant: u64,
}

/// Checked summary of one accepted symbolic-history certificate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Root logarithmic size parameter.
    pub lambda: u64,
    /// Root source scale `floor(lambda/2^17)`.
    pub root_u: u64,
    /// Number of possible active depths reconstructed from strict shrinkage.
    pub active_depths: usize,
    /// Root-`u` exponent constant for the complete recursion composition.
    pub total_root_u_exponent_constant: u64,
    /// Root-log exponent constant after the uniform square-root conversion.
    pub total_root_log_exponent_constant: u64,
}

fn floor_sqrt(value: u64) -> u64 {
    if value < 2 {
        return value;
    }
    let mut low = 1u64;
    let mut high = 1u64 << value.ilog2().div_ceil(2);
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if middle <= value / middle {
            low = middle;
        } else {
            high = middle;
        }
    }
    if high <= value / high {
        high
    } else {
        low
    }
}

fn at_most_power_of_two(value: &BigUint, exponent: u64) -> bool {
    if value.is_zero() {
        return true;
    }
    let bits = value.bits();
    if bits <= exponent {
        return true;
    }
    if bits != exponent.saturating_add(1) {
        return false;
    }
    let Ok(shift) = usize::try_from(exponent) else {
        return false;
    };
    value == &(BigUint::one() << shift)
}

fn checked_add(left: u64, right: u64, label: &str) -> Result<u64, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("{label} overflow"))
}

fn checked_mul(left: u64, right: u64, label: &str) -> Result<u64, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("{label} overflow"))
}

/// Reconstruct and verify every field in an implicit strict-history transcript.
///
/// Acceptance checks the complete depth schedule implied by eight-fold child
/// shrinkage and the fixed leaf cutoff, both precision-absorption inequalities,
/// the geometric path comparison, packet-entry absorption, and all reported
/// exponent constants. It does not establish that any graph realizes the
/// stated worst-case premises or prove the external contraction theorem.
pub fn verify(certificate: &Certificate) -> Result<VerificationSummary, String> {
    if certificate.schema != SCHEMA || certificate.lambda < MINIMUM_LAMBDA {
        return Err("symbolic history context".into());
    }
    let leaf_divisor = floor_sqrt(certificate.lambda);
    let root_u = certificate.lambda / EPSILON_DENOMINATOR;
    if leaf_divisor <= 1
        || root_u == 0
        || certificate.leaf_divisor != leaf_divisor
        || certificate.root_u != root_u
        || certificate.scale_floor != 32
        || certificate.inherited_precision_constant != 81
        || certificate.normalized_precision_constant != 34
    {
        return Err("symbolic history constants".into());
    }

    let mut expected_depth = 0u32;
    let mut shrink_factor = BigUint::one();
    let mut prior_a = 0u64;
    let mut index = 0usize;
    while shrink_factor < BigUint::from(leaf_divisor) {
        let u = root_u.checked_shr(expected_depth).unwrap_or(0);
        if u == 0 {
            break;
        }
        let current_a = checked_add(
            prior_a,
            checked_mul(2, u, "symbolic history scale")?,
            "symbolic history scale",
        )?;
        let inherited_lhs = BigUint::from(81u8) * BigUint::from(leaf_divisor) * BigUint::from(u);
        let inherited_rhs_exponent = checked_add(u, 1, "inherited exponent")?;
        let evaluation_lhs = BigUint::from(8u8)
            * BigUint::from(u)
            * BigUint::from(checked_add(current_a, 1, "evaluation width")?);
        let evaluation_rhs_exponent = checked_add(
            checked_add(prior_a, u, "evaluation exponent")?,
            1,
            "evaluation exponent",
        )?;
        let geometric_scale = BigUint::from(4u8).pow(expected_depth + 1) * BigUint::from(u);
        let geometric_holds = BigUint::from(root_u) <= geometric_scale;
        let Some(case) = certificate.depths.get(index) else {
            return Err("symbolic history depth missing".into());
        };
        if case.depth != expected_depth
            || case.shrink_factor != shrink_factor
            || case.u != u
            || case.prior_a != prior_a
            || case.current_a != current_a
            || case.inherited_lhs != inherited_lhs
            || case.inherited_rhs_exponent != inherited_rhs_exponent
            || case.evaluation_lhs != evaluation_lhs
            || case.evaluation_rhs_exponent != evaluation_rhs_exponent
            || case.geometric_scale_comparison_holds != geometric_holds
        {
            return Err("symbolic history depth mismatch".into());
        }
        if u < certificate.scale_floor
            || !at_most_power_of_two(&inherited_lhs, inherited_rhs_exponent)
            || !at_most_power_of_two(&evaluation_lhs, evaluation_rhs_exponent)
            || !geometric_holds
        {
            return Err("symbolic history depth inequality".into());
        }
        index += 1;
        expected_depth = expected_depth
            .checked_add(1)
            .ok_or_else(|| "symbolic history depth overflow".to_string())?;
        prior_a = current_a;
        shrink_factor *= BigUint::from(8u8);
    }
    if index != certificate.depths.len() || index == 0 {
        return Err("symbolic history depth coverage".into());
    }

    let local_child_exponent_constant = 96u64;
    let path_sum_constant = 3u64;
    let active_tree_exponent_constant = checked_mul(
        local_child_exponent_constant,
        path_sum_constant,
        "active tree exponent",
    )?;
    let leaf_exponent_constant = 1u64;
    let packet_entry_exponent_constant = 1u64;
    let total_root_u_exponent_constant = checked_add(
        checked_add(
            active_tree_exponent_constant,
            leaf_exponent_constant,
            "root exponent",
        )?,
        packet_entry_exponent_constant,
        "root exponent",
    )?;
    let total_root_log_exponent_constant =
        checked_mul(total_root_u_exponent_constant, 512, "root-log exponent")?;
    let leaf_scale_absorbed = BigUint::from(leaf_divisor).pow(2) >= BigUint::from(root_u);
    let packet_entry_polynomial_factor = BigUint::from(8u8)
        + BigUint::from(2u8) * BigUint::from(certificate.lambda)
        + BigUint::from(5u8) * BigUint::from(root_u);
    let entry_power = checked_mul(root_u, 4, "packet entry exponent")?;
    let entry_rhs_exponent = certificate
        .lambda
        .checked_sub(1)
        .and_then(|value| value.checked_sub(entry_power))
        .ok_or_else(|| "packet entry absorption width".to_string())?;
    let packet_entry_absorbed = at_most_power_of_two(
        &(&packet_entry_polynomial_factor * BigUint::from(root_u)),
        entry_rhs_exponent,
    );

    if certificate.local_child_exponent_constant != local_child_exponent_constant
        || certificate.path_sum_constant != path_sum_constant
        || certificate.active_tree_exponent_constant != active_tree_exponent_constant
        || certificate.leaf_exponent_constant != leaf_exponent_constant
        || certificate.leaf_scale_absorbed != leaf_scale_absorbed
        || certificate.packet_entry_exponent_constant != packet_entry_exponent_constant
        || certificate.packet_entry_polynomial_factor != packet_entry_polynomial_factor
        || certificate.packet_entry_absorbed != packet_entry_absorbed
        || certificate.total_root_u_exponent_constant != total_root_u_exponent_constant
        || certificate.total_root_log_exponent_constant != total_root_log_exponent_constant
    {
        return Err("symbolic history composition mismatch".into());
    }
    if !leaf_scale_absorbed || !packet_entry_absorbed {
        return Err("symbolic history composition inequality".into());
    }

    Ok(VerificationSummary {
        lambda: certificate.lambda,
        root_u,
        active_depths: index,
        total_root_u_exponent_constant,
        total_root_log_exponent_constant,
    })
}
