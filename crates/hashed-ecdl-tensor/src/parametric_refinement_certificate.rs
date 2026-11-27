//! Parametric one-step implication for the strict recursion schedule.
//!
//! This certificate treats graph-generic source and normalized-precision
//! bounds as explicit premises. It independently checks the algebra that turns
//! those premises into capacity, mode dimension, group-load, strict-shrink,
//! degree, and cost-ledger conclusions across the symbolic depth schedule. It
//! does not prove that a graph realizes the premises or prove the external
//! contraction theorem.

use crate::composition_boundary_certificate::{
    verify as verify_composition, Certificate as CompositionCertificate,
};
use serde::{Deserialize, Serialize};

/// Current parametric-refinement wire schema.
pub const SCHEMA: u32 = 2;

/// Explicit premises to the parametric implication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Premises {
    /// The conditional-label variance bound is available.
    pub variance_bound_available: bool,
    /// The active source has `M>=2D^2 delta`.
    pub strong_source_guard_available: bool,
    /// The active source has `M>=LD`.
    pub length_array_guard_available: bool,
    /// The active source has `2L<=delta`.
    pub packet_degree_guard_available: bool,
    /// Parent old-vertex degree is at most `delta`.
    pub parent_degree_bound_available: bool,
    /// The local precision analysis supplies `pu<=34MD`.
    pub normalized_precision_bound_available: bool,
    /// The next call either rechecks all source guards or uses exact fallback.
    pub recheck_or_fallback_available: bool,
}

/// One retained depth in the symbolic schedule.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Depth {
    /// Zero-based recursion depth.
    pub depth: u32,
    /// Eightfold shrink accumulated before this depth.
    pub shrink_factor: u64,
    /// Source scale at this depth.
    pub u: u64,
    /// Packet exponent before this depth's increment.
    pub prior_a: u64,
    /// Packet exponent after adding `2u`.
    pub current_a: u64,
}

/// Constants in the checked algebraic implication and cost ledger.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constants {
    /// Label radix.
    pub radix: u64,
    /// Variance/source-guard coefficient.
    pub variance_factor: u64,
    /// Derived per-class coefficient in `cD<=2M`.
    pub class_cap_factor: u64,
    /// Contribution from `LD<=M` to padded capacity.
    pub length_array_factor: u64,
    /// Derived packet-capacity coefficient in `KL<=3M`.
    pub packet_capacity_factor: u64,
    /// Derived one-side group-load coefficient.
    pub derived_group_load_factor: u64,
    /// Coarser group-load coefficient used by the symbolic history.
    pub history_group_load_factor: u64,
    /// Number of bipartite endpoint sides in one child.
    pub endpoint_sides: u64,
    /// Derived child-edge numerator over the radix.
    pub derived_child_edge_numerator: u64,
    /// History child-edge numerator over the radix.
    pub history_child_edge_numerator: u64,
    /// Required strict shrink factor.
    pub strict_shrink_factor: u64,
    /// Packet-vertex degree coefficient in `2L<=delta`.
    pub packet_degree_factor: u64,
    /// Coefficient in `pu<=34MD`.
    pub normalized_precision_factor: u64,
    /// Coefficient in `nD<=6M`.
    pub mode_dimension_factor: u64,
    /// Product coefficient in `npu<=204M^2`.
    pub mode_precision_product_factor: u64,
    /// Integer ceiling for `sqrt(204)`.
    pub tuple_square_root_ceiling: u64,
    /// Tuple-count exponent constant.
    pub tuple_exponent_constant: u64,
    /// Description overhead in the local exponent ledger.
    pub description_overhead_constant: u64,
    /// Active-node overhead in the local exponent ledger.
    pub active_node_constant: u64,
    /// Interpolation-sample overhead in the local exponent ledger.
    pub interpolation_sample_constant: u64,
    /// Radix overhead in the local exponent ledger.
    pub radix_overhead_constant: u64,
    /// Complete local child exponent constant.
    pub local_child_exponent_constant: u64,
    /// Geometric path-sum constant.
    pub path_sum_constant: u64,
    /// Leaf exponent constant.
    pub leaf_exponent_constant: u64,
    /// Packet-entry exponent constant.
    pub packet_entry_exponent_constant: u64,
    /// Complete root-`u` exponent constant.
    pub total_root_u_exponent_constant: u64,
    /// Square-root conversion factor from root `u` to root log.
    pub root_log_conversion_factor: u64,
    /// Complete root-log exponent constant.
    pub total_root_log_exponent_constant: u64,
}

/// Claims serialized so every inference edge is mutation-testable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assertions {
    /// Variance plus the strong guard imply `cD<=2M`.
    pub class_cap_derived: bool,
    /// Class cap plus `LD<=M` imply `KL<=3M`.
    pub packet_capacity_derived: bool,
    /// Packet capacity plus the tile/distance identity imply `nD<=6M`.
    pub mode_dimension_bound_derived: bool,
    /// The derived group cap implies the history group bound.
    pub history_group_bound_derived: bool,
    /// The history group bound implies eightfold child shrink.
    pub strict_child_shrink_derived: bool,
    /// Old and packet vertices preserve the degree cap.
    pub child_degree_bound_derived: bool,
    /// Precision and mode premises imply the tuple exponent constant.
    pub tuple_cost_derived: bool,
    /// Local and geometric ledgers reproduce the history totals.
    pub history_cost_ledger_composed: bool,
    /// A complete strict recursion graph is realized.
    pub strict_recursion_graph_realized: bool,
    /// The external contraction theorem is independently verified.
    pub external_contraction_theorem_verified: bool,
    /// A subexponential total-work theorem is certified by this object.
    pub subexponential_total_work_certified: bool,
}

/// Parametric implication paired with the checked schedule boundary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    /// Wire schema version.
    pub schema: u32,
    /// Independently checked strict-root/symbolic-history boundary.
    pub composition: CompositionCertificate,
    /// Explicit assumptions to the generic one-step implication.
    pub premises: Premises,
    /// Every retained symbolic depth.
    pub depths: Vec<Depth>,
    /// Constants reconstructed by the verifier.
    pub constants: Constants,
    /// Claimed conclusions checked against the implication.
    pub assertions: Assertions,
}

/// Checked parametric conclusions and their remaining boundary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Shared root logarithmic size.
    pub lambda: u64,
    /// Shared root source scale.
    pub root_u: u64,
    /// Number of retained depths checked.
    pub active_depths: usize,
    /// Derived packet-capacity coefficient.
    pub packet_capacity_factor: u64,
    /// Whether the mode-dimension bound was derived from packet capacity.
    pub mode_dimension_bound_derived: bool,
    /// Derived child-edge numerator over radix 64.
    pub derived_child_edge_numerator: u64,
    /// History child-edge numerator over radix 64.
    pub history_child_edge_numerator: u64,
    /// Complete local exponent constant.
    pub local_child_exponent_constant: u64,
    /// Complete root-`u` exponent constant.
    pub total_root_u_exponent_constant: u64,
    /// Complete root-log exponent constant.
    pub total_root_log_exponent_constant: u64,
    /// Whether the generic algebraic implication was checked.
    pub parametric_implication_certified: bool,
    /// Whether generic graph/source premises remain assumptions.
    pub generic_premises_remain_assumed: bool,
    /// Whether a complete strict recursion graph was realized.
    pub strict_recursion_graph_realized: bool,
    /// Whether the external contraction theorem was independently verified.
    pub external_contraction_theorem_verified: bool,
    /// Whether this object certifies the total subexponential conclusion.
    pub subexponential_total_work_certified: bool,
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

fn checked_add(left: u64, right: u64, label: &str) -> Result<u64, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("{label} overflow"))
}

fn checked_mul(left: u64, right: u64, label: &str) -> Result<u64, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("{label} overflow"))
}

fn expected_depths(lambda: u64, root_u: u64) -> Result<Vec<Depth>, String> {
    let leaf_divisor = floor_sqrt(lambda);
    let mut depths = Vec::new();
    let mut depth = 0u32;
    let mut shrink_factor = 1u64;
    let mut prior_a = 0u64;
    while shrink_factor < leaf_divisor {
        let u = root_u.checked_shr(depth).unwrap_or(0);
        if u == 0 {
            break;
        }
        let current_a = checked_add(
            prior_a,
            checked_mul(2, u, "parametric packet exponent")?,
            "parametric packet exponent",
        )?;
        depths.push(Depth {
            depth,
            shrink_factor,
            u,
            prior_a,
            current_a,
        });
        prior_a = current_a;
        depth = depth
            .checked_add(1)
            .ok_or_else(|| "parametric depth overflow".to_string())?;
        shrink_factor = checked_mul(shrink_factor, 8, "parametric shrink")?;
    }
    Ok(depths)
}

fn expected_constants() -> Result<Constants, String> {
    let radix = 64;
    let variance_factor = 2;
    let class_cap_factor = variance_factor;
    let length_array_factor = 1;
    let packet_capacity_factor = checked_add(
        class_cap_factor,
        length_array_factor,
        "parametric packet capacity",
    )?;
    let derived_group_load_factor = class_cap_factor;
    let history_group_load_factor = packet_capacity_factor;
    let endpoint_sides = 2;
    let derived_child_edge_numerator = checked_mul(
        endpoint_sides,
        derived_group_load_factor,
        "parametric derived child",
    )?;
    let history_child_edge_numerator = checked_mul(
        endpoint_sides,
        history_group_load_factor,
        "parametric history child",
    )?;
    let strict_shrink_factor = 8;
    let packet_degree_factor = 2;
    let normalized_precision_factor = 34;
    let mode_dimension_factor = checked_mul(
        endpoint_sides,
        packet_capacity_factor,
        "parametric mode dimension",
    )?;
    let mode_precision_product_factor = checked_mul(
        normalized_precision_factor,
        mode_dimension_factor,
        "parametric mode precision",
    )?;
    let tuple_square_root_ceiling = 15;
    let tuple_exponent_constant = checked_add(
        checked_mul(5, tuple_square_root_ceiling, "parametric tuple exponent")?,
        5,
        "parametric tuple exponent",
    )?;
    let description_overhead_constant = 1;
    let active_node_constant = 4;
    let interpolation_sample_constant = 5;
    let radix_overhead_constant = 6;
    let local_child_exponent_constant = [
        tuple_exponent_constant,
        description_overhead_constant,
        active_node_constant,
        interpolation_sample_constant,
        radix_overhead_constant,
    ]
    .into_iter()
    .try_fold(0u64, |sum, value| {
        checked_add(sum, value, "parametric local ledger")
    })?;
    let path_sum_constant = 3;
    let leaf_exponent_constant = 1;
    let packet_entry_exponent_constant = 1;
    let total_root_u_exponent_constant = checked_add(
        checked_add(
            checked_mul(
                local_child_exponent_constant,
                path_sum_constant,
                "parametric root ledger",
            )?,
            leaf_exponent_constant,
            "parametric root ledger",
        )?,
        packet_entry_exponent_constant,
        "parametric root ledger",
    )?;
    let root_log_conversion_factor = 512;
    let total_root_log_exponent_constant = checked_mul(
        total_root_u_exponent_constant,
        root_log_conversion_factor,
        "parametric root-log ledger",
    )?;
    Ok(Constants {
        radix,
        variance_factor,
        class_cap_factor,
        length_array_factor,
        packet_capacity_factor,
        derived_group_load_factor,
        history_group_load_factor,
        endpoint_sides,
        derived_child_edge_numerator,
        history_child_edge_numerator,
        strict_shrink_factor,
        packet_degree_factor,
        normalized_precision_factor,
        mode_dimension_factor,
        mode_precision_product_factor,
        tuple_square_root_ceiling,
        tuple_exponent_constant,
        description_overhead_constant,
        active_node_constant,
        interpolation_sample_constant,
        radix_overhead_constant,
        local_child_exponent_constant,
        path_sum_constant,
        leaf_exponent_constant,
        packet_entry_exponent_constant,
        total_root_u_exponent_constant,
        root_log_conversion_factor,
        total_root_log_exponent_constant,
    })
}

/// Assemble the unique transcript supported by one composition boundary.
pub fn assemble(composition: CompositionCertificate) -> Result<Certificate, String> {
    let summary = verify_composition(&composition)?;
    Ok(Certificate {
        schema: SCHEMA,
        depths: expected_depths(summary.lambda, summary.u)?,
        constants: expected_constants()?,
        premises: Premises {
            variance_bound_available: true,
            strong_source_guard_available: true,
            length_array_guard_available: true,
            packet_degree_guard_available: true,
            parent_degree_bound_available: true,
            normalized_precision_bound_available: true,
            recheck_or_fallback_available: true,
        },
        assertions: Assertions {
            class_cap_derived: true,
            packet_capacity_derived: true,
            mode_dimension_bound_derived: true,
            history_group_bound_derived: true,
            strict_child_shrink_derived: true,
            child_degree_bound_derived: true,
            tuple_cost_derived: true,
            history_cost_ledger_composed: true,
            strict_recursion_graph_realized: false,
            external_contraction_theorem_verified: false,
            subexponential_total_work_certified: false,
        },
        composition,
    })
}

/// Verify the parametric inference rules and retained-depth cost ledger.
///
/// Acceptance proves the implication from the serialized premises. It does
/// not prove those premises for a generic graph family.
pub fn verify(certificate: &Certificate) -> Result<VerificationSummary, String> {
    if certificate.schema != SCHEMA {
        return Err("parametric refinement context".into());
    }
    let composition = verify_composition(&certificate.composition)
        .map_err(|error| format!("parametric composition: {error}"))?;
    let expected_depths = expected_depths(composition.lambda, composition.u)?;
    if certificate.depths != expected_depths
        || certificate.depths.len() != composition.active_depths
    {
        return Err("parametric depth schedule".into());
    }
    let expected_constants = expected_constants()?;
    if certificate.constants != expected_constants {
        return Err("parametric constants".into());
    }
    let premises = &certificate.premises;
    if !premises.variance_bound_available
        || !premises.strong_source_guard_available
        || !premises.length_array_guard_available
        || !premises.packet_degree_guard_available
        || !premises.parent_degree_bound_available
        || !premises.normalized_precision_bound_available
        || !premises.recheck_or_fallback_available
    {
        return Err("parametric premise missing".into());
    }

    let constants = &certificate.constants;
    if constants.derived_group_load_factor > constants.history_group_load_factor
        || checked_mul(
            constants.strict_shrink_factor,
            constants.history_child_edge_numerator,
            "parametric strict child",
        )? > constants.radix
        || checked_mul(
            constants.strict_shrink_factor,
            constants.derived_child_edge_numerator,
            "parametric derived child",
        )? > constants.radix
        || (constants.tuple_square_root_ceiling - 1).pow(2)
            >= constants.mode_precision_product_factor
        || constants.tuple_square_root_ceiling.pow(2) < constants.mode_precision_product_factor
        || constants.tuple_exponent_constant != composition.tuple_exponent_constant
        || constants.total_root_u_exponent_constant != composition.total_root_u_exponent_constant
        || constants.total_root_log_exponent_constant
            != composition.total_root_log_exponent_constant
    {
        return Err("parametric implication mismatch".into());
    }

    let assertions = &certificate.assertions;
    if !assertions.class_cap_derived
        || !assertions.packet_capacity_derived
        || !assertions.mode_dimension_bound_derived
        || !assertions.history_group_bound_derived
        || !assertions.strict_child_shrink_derived
        || !assertions.child_degree_bound_derived
        || !assertions.tuple_cost_derived
        || !assertions.history_cost_ledger_composed
    {
        return Err("parametric conclusion missing".into());
    }
    if assertions.strict_recursion_graph_realized {
        return Err("parametric graph realization unsupported".into());
    }
    if assertions.external_contraction_theorem_verified {
        return Err("parametric external theorem unsupported".into());
    }
    if assertions.subexponential_total_work_certified {
        return Err("parametric total-work conclusion unsupported".into());
    }

    Ok(VerificationSummary {
        lambda: composition.lambda,
        root_u: composition.u,
        active_depths: certificate.depths.len(),
        packet_capacity_factor: constants.packet_capacity_factor,
        mode_dimension_bound_derived: true,
        derived_child_edge_numerator: constants.derived_child_edge_numerator,
        history_child_edge_numerator: constants.history_child_edge_numerator,
        local_child_exponent_constant: constants.local_child_exponent_constant,
        total_root_u_exponent_constant: constants.total_root_u_exponent_constant,
        total_root_log_exponent_constant: constants.total_root_log_exponent_constant,
        parametric_implication_certified: true,
        generic_premises_remain_assumed: true,
        strict_recursion_graph_realized: false,
        external_contraction_theorem_verified: false,
        subexponential_total_work_certified: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::admission::dimensions::symbolic_history_certificate;
    use crate::composition_boundary_certificate::assemble as assemble_composition;
    use crate::symmetric_source_family_certificate::produce as produce_source;

    const LAMBDA: u64 = 1 << 26;

    fn certificate() -> Certificate {
        let composition = assemble_composition(
            produce_source(LAMBDA).unwrap(),
            symbolic_history_certificate(LAMBDA).unwrap(),
        );
        assemble(composition).unwrap()
    }

    #[test]
    fn composes_the_parametric_step_across_every_retained_depth() {
        let summary = verify(&certificate()).unwrap();
        assert_eq!(summary.active_depths, 5);
        assert_eq!(summary.packet_capacity_factor, 3);
        assert!(summary.mode_dimension_bound_derived);
        assert_eq!(summary.derived_child_edge_numerator, 4);
        assert_eq!(summary.history_child_edge_numerator, 6);
        assert_eq!(summary.local_child_exponent_constant, 96);
        assert_eq!(summary.total_root_u_exponent_constant, 290);
        assert_eq!(summary.total_root_log_exponent_constant, 148_480);
        assert!(summary.parametric_implication_certified);
        assert!(summary.generic_premises_remain_assumed);
        assert!(!summary.strict_recursion_graph_realized);
        assert!(!summary.external_contraction_theorem_verified);
        assert!(!summary.subexponential_total_work_certified);
    }

    #[test]
    fn rejects_depth_schedule_mutations() {
        let mut wrong_u = certificate();
        wrong_u.depths[2].u += 1;
        assert_eq!(verify(&wrong_u).unwrap_err(), "parametric depth schedule");

        let mut missing = certificate();
        missing.depths.pop();
        assert_eq!(verify(&missing).unwrap_err(), "parametric depth schedule");
    }

    #[test]
    fn rejects_source_and_shrink_rule_mutations() {
        for mutate in [
            |constants: &mut Constants| constants.class_cap_factor += 1,
            |constants: &mut Constants| constants.packet_capacity_factor += 1,
            |constants: &mut Constants| constants.derived_group_load_factor += 1,
            |constants: &mut Constants| constants.history_child_edge_numerator += 1,
            |constants: &mut Constants| constants.strict_shrink_factor += 1,
        ] {
            let mut changed = certificate();
            mutate(&mut changed.constants);
            assert_eq!(verify(&changed).unwrap_err(), "parametric constants");
        }
    }

    #[test]
    fn rejects_cost_ledger_mutations() {
        for mutate in [
            |constants: &mut Constants| constants.normalized_precision_factor += 1,
            |constants: &mut Constants| constants.mode_dimension_factor += 1,
            |constants: &mut Constants| constants.tuple_exponent_constant += 1,
            |constants: &mut Constants| constants.local_child_exponent_constant += 1,
            |constants: &mut Constants| constants.total_root_log_exponent_constant += 1,
        ] {
            let mut changed = certificate();
            mutate(&mut changed.constants);
            assert_eq!(verify(&changed).unwrap_err(), "parametric constants");
        }
    }

    #[test]
    fn rejects_missing_premises_and_conclusions() {
        let mut premise = certificate();
        premise.premises.variance_bound_available = false;
        assert_eq!(verify(&premise).unwrap_err(), "parametric premise missing");

        let mut conclusion = certificate();
        conclusion.assertions.mode_dimension_bound_derived = false;
        assert_eq!(
            verify(&conclusion).unwrap_err(),
            "parametric conclusion missing"
        );
    }

    #[test]
    fn rejects_every_unsupported_promotion() {
        let cases: [(&str, fn(&mut Assertions)); 3] = [
            ("parametric graph realization unsupported", |assertions| {
                assertions.strict_recursion_graph_realized = true;
            }),
            ("parametric external theorem unsupported", |assertions| {
                assertions.external_contraction_theorem_verified = true;
            }),
            (
                "parametric total-work conclusion unsupported",
                |assertions| {
                    assertions.subexponential_total_work_certified = true;
                },
            ),
        ];
        for (expected, mutate) in cases {
            let mut promoted = certificate();
            mutate(&mut promoted.assertions);
            assert_eq!(verify(&promoted).unwrap_err(), expected);
        }
    }
}
