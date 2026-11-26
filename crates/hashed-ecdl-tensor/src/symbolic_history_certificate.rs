//! Standalone verification of an implicit strict-recursion history schedule.
//!
//! This module consumes serialized scalar data and exact integers. It does not
//! import graph contraction, packing, factorization, or the producer-side
//! dimension audit.

use crate::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

pub(crate) const SCHEMA: u32 = 5;
const EPSILON_DENOMINATOR: u64 = 128 * 1024;
const MINIMUM_LAMBDA: u64 = 1 << 26;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DepthCertificate {
    pub(crate) depth: u32,
    pub(crate) shrink_factor: BigUint,
    pub(crate) u: u64,
    pub(crate) prior_a: u64,
    pub(crate) current_a: u64,
    pub(crate) first_tile_exponent: u64,
    pub(crate) last_tile_exponent: u64,
    pub(crate) first_distance_exponent: u64,
    pub(crate) last_distance_exponent: u64,
    pub(crate) normalized_edges: BigUint,
    pub(crate) normalized_packets: BigUint,
    pub(crate) normalized_first_precision: BigUint,
    pub(crate) normalized_last_precision: BigUint,
    pub(crate) description_overhead_exponent: BigUint,
    pub(crate) description_absorption_lhs: BigUint,
    pub(crate) description_absorption_rhs_exponent: u64,
    pub(crate) description_absorbed: bool,
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
    pub(crate) packet_capacity_constant: u64,
    pub(crate) mode_distance_constant: u64,
    pub(crate) mode_precision_product_constant: u64,
    pub(crate) tuple_square_root_ceiling: u64,
    pub(crate) tuple_exponent_constant: u64,
    pub(crate) description_overhead_constant: u64,
    pub(crate) packing_radix: u64,
    pub(crate) group_port_balance_constant: u64,
    pub(crate) endpoint_sides: u64,
    pub(crate) strict_edge_shrink_factor: u64,
    pub(crate) child_edge_bound_numerator: u64,
    pub(crate) strict_edge_bound_numerator: u64,
    pub(crate) packet_degree_factor: u64,
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
    /// Number of endpoint dimension witnesses checked across all depths.
    pub normalized_coordinate_witnesses: usize,
    /// Local exponent unit reserved for factorization-certificate descriptions.
    pub description_overhead_constant: u64,
    /// Whether source packing bounds imply strict edge and packet-degree shrink.
    pub strict_packing_shrink_certified: bool,
    /// Constant in `log2(T) <= C*M/sqrt(u)` from the source mode bounds.
    pub tuple_exponent_constant: u64,
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

fn power_of_two(exponent: u64, label: &str) -> Result<BigUint, String> {
    let shift = usize::try_from(exponent).map_err(|_| format!("{label} width"))?;
    Ok(BigUint::one() << shift)
}

fn verify_normalized_coordinate(
    edges: &BigUint,
    packets: &BigUint,
    packet_length: &BigUint,
    u: u64,
    tile_exponent: u64,
    distance_exponent: u64,
    precision: &BigUint,
) -> Result<(), String> {
    let tile_period = power_of_two(tile_exponent, "normalized tile")?;
    let distance = power_of_two(distance_exponent, "normalized distance")?;
    let modes_per_weight = packets * &tile_period;
    let expected_precision = BigUint::from(34u8) * packet_length * &distance;
    if precision != &expected_precision
        || packets * packet_length != BigUint::from(3u8) * edges
        || &tile_period * &distance != BigUint::from(2u8) * packet_length
        || &modes_per_weight * &distance != BigUint::from(6u8) * edges
        || precision * BigUint::from(u) != BigUint::from(34u8) * edges * &distance
        || modes_per_weight * precision * BigUint::from(u) != BigUint::from(204u16) * edges * edges
    {
        return Err("symbolic normalized dimension mismatch".into());
    }
    Ok(())
}

/// Reconstruct and verify every field in an implicit strict-history transcript.
///
/// Acceptance checks the complete depth schedule implied by eight-fold child
/// shrinkage and the fixed leaf cutoff, both precision-absorption inequalities,
/// the geometric path comparison, normalized endpoint dimension identities,
/// packet-entry absorption, and all reported exponent constants. It does not
/// establish that any graph realizes the stated worst-case premises or prove
/// the external contraction theorem.
pub fn verify(certificate: &Certificate) -> Result<VerificationSummary, String> {
    if certificate.schema != SCHEMA || certificate.lambda < MINIMUM_LAMBDA {
        return Err("symbolic history context".into());
    }
    let leaf_divisor = floor_sqrt(certificate.lambda);
    let root_u = certificate.lambda / EPSILON_DENOMINATOR;
    let packet_capacity_constant = 3u64;
    let mode_distance_constant = checked_mul(2, packet_capacity_constant, "mode distance")?;
    let mode_precision_product_constant =
        checked_mul(mode_distance_constant, 34, "mode precision product")?;
    let tuple_square_root_ceiling = 15u64;
    let tuple_exponent_constant = checked_add(
        checked_mul(5, tuple_square_root_ceiling, "tuple exponent")?,
        5,
        "tuple exponent",
    )?;
    let description_overhead_constant = 1u64;
    let packing_radix = 64u64;
    let group_port_balance_constant = 3u64;
    let endpoint_sides = 2u64;
    let strict_edge_shrink_factor = 8u64;
    let child_edge_bound_numerator = checked_mul(
        group_port_balance_constant,
        endpoint_sides,
        "packing child edge bound",
    )?;
    let strict_edge_bound_numerator = checked_mul(
        child_edge_bound_numerator,
        strict_edge_shrink_factor,
        "packing strict edge bound",
    )?;
    let packet_degree_factor = 2u64;
    if leaf_divisor <= 1
        || root_u == 0
        || certificate.leaf_divisor != leaf_divisor
        || certificate.root_u != root_u
        || certificate.scale_floor != 32
        || certificate.inherited_precision_constant != 81
        || certificate.normalized_precision_constant != 34
        || certificate.packet_capacity_constant != packet_capacity_constant
        || certificate.mode_distance_constant != mode_distance_constant
        || certificate.mode_precision_product_constant != mode_precision_product_constant
        || certificate.tuple_square_root_ceiling != tuple_square_root_ceiling
        || certificate.tuple_exponent_constant != tuple_exponent_constant
        || certificate.description_overhead_constant != description_overhead_constant
        || certificate.packing_radix != packing_radix
        || certificate.group_port_balance_constant != group_port_balance_constant
        || certificate.endpoint_sides != endpoint_sides
        || certificate.strict_edge_shrink_factor != strict_edge_shrink_factor
        || certificate.child_edge_bound_numerator != child_edge_bound_numerator
        || certificate.strict_edge_bound_numerator != strict_edge_bound_numerator
        || certificate.packet_degree_factor != packet_degree_factor
        || strict_edge_bound_numerator > packing_radix
        || (tuple_square_root_ceiling - 1).pow(2) >= mode_precision_product_constant
        || tuple_square_root_ceiling.pow(2) < mode_precision_product_constant
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
        let first_tile_exponent = 1u64;
        let last_tile_exponent = u;
        let first_distance_exponent = current_a;
        let last_distance_exponent = current_a
            .checked_sub(u)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| "symbolic tile-distance range".to_string())?;
        let tile_distance_exponent = checked_add(current_a, 1, "tile-distance exponent")?;
        let packet_length = power_of_two(current_a, "normalized packet length")?;
        let normalized_edges = BigUint::from(u) * &packet_length;
        let normalized_packets = BigUint::from(3u8) * BigUint::from(u);
        let description_polynomial = checked_mul(
            16,
            checked_add(
                checked_add(
                    checked_add(certificate.lambda, current_a, "description exponent")?,
                    u,
                    "description exponent",
                )?,
                1,
                "description exponent",
            )?,
            "description exponent",
        )?;
        let description_overhead_exponent =
            BigUint::from(2u8) * &packet_length + BigUint::from(description_polynomial);
        let floor_root = floor_sqrt(u);
        let ceiling_root = floor_root + u64::from(floor_root * floor_root != u);
        let description_absorption_lhs = &description_overhead_exponent
            * BigUint::from(leaf_divisor)
            * BigUint::from(ceiling_root);
        let description_absorption_rhs_exponent = certificate
            .lambda
            .checked_sub(1)
            .ok_or_else(|| "description absorption exponent".to_string())?;
        let description_absorbed = at_most_power_of_two(
            &description_absorption_lhs,
            description_absorption_rhs_exponent,
        );
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
            || case.first_tile_exponent != first_tile_exponent
            || case.last_tile_exponent != last_tile_exponent
            || case.first_distance_exponent != first_distance_exponent
            || case.last_distance_exponent != last_distance_exponent
            || case.normalized_edges != normalized_edges
            || case.normalized_packets != normalized_packets
            || case.description_overhead_exponent != description_overhead_exponent
            || case.description_absorption_lhs != description_absorption_lhs
            || case.description_absorption_rhs_exponent != description_absorption_rhs_exponent
            || case.description_absorbed != description_absorbed
            || case.inherited_lhs != inherited_lhs
            || case.inherited_rhs_exponent != inherited_rhs_exponent
            || case.evaluation_lhs != evaluation_lhs
            || case.evaluation_rhs_exponent != evaluation_rhs_exponent
            || case.geometric_scale_comparison_holds != geometric_holds
        {
            return Err("symbolic history depth mismatch".into());
        }
        verify_normalized_coordinate(
            &case.normalized_edges,
            &case.normalized_packets,
            &packet_length,
            u,
            first_tile_exponent,
            first_distance_exponent,
            &case.normalized_first_precision,
        )?;
        verify_normalized_coordinate(
            &case.normalized_edges,
            &case.normalized_packets,
            &packet_length,
            u,
            last_tile_exponent,
            last_distance_exponent,
            &case.normalized_last_precision,
        )?;
        if u < certificate.scale_floor
            || checked_add(
                first_tile_exponent,
                first_distance_exponent,
                "tile-distance exponent",
            )? != tile_distance_exponent
            || checked_add(
                last_tile_exponent,
                last_distance_exponent,
                "tile-distance exponent",
            )? != tile_distance_exponent
            || !at_most_power_of_two(&inherited_lhs, inherited_rhs_exponent)
            || !at_most_power_of_two(&evaluation_lhs, evaluation_rhs_exponent)
            || !geometric_holds
            || !description_absorbed
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
    let accounted_local_constant =
        tuple_exponent_constant + description_overhead_constant + 4 + 5 + 6;
    if local_child_exponent_constant < accounted_local_constant {
        return Err("symbolic history local exponent".into());
    }
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
        normalized_coordinate_witnesses: 2 * index,
        description_overhead_constant,
        strict_packing_shrink_certified: true,
        tuple_exponent_constant,
        total_root_u_exponent_constant,
        total_root_log_exponent_constant,
    })
}
