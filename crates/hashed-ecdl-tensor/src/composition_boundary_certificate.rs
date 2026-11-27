//! Standalone boundary check for the strict-root and symbolic-history interfaces.
//!
//! The two component certificates share schedule parameters but use different
//! dimension witnesses. This module verifies the compatible facts and rejects
//! claims that schedule compatibility alone realizes the recursive graph or
//! proves the external contraction theorem.

use crate::symbolic_history_certificate::{
    verify as verify_history, Certificate as HistoryCertificate,
};
use crate::symmetric_source_family_certificate::{
    verify as verify_source, Certificate as SourceCertificate,
};
use serde::{Deserialize, Serialize};

/// Current composition-boundary wire schema.
pub const SCHEMA: u32 = 1;

/// Claims serialized so that unsupported promotions are mutation-testable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assertions {
    /// Both transcripts use the same root `lambda`, `u`, and `a` schedule.
    pub schedule_compatible: bool,
    /// Exact source shrink `M/32` satisfies the history bound `M/8`.
    pub source_bound_implies_history_bound: bool,
    /// Normalized history dimensions equal the strict root dimensions.
    pub normalized_dimensions_equal_strict_root: bool,
    /// A complete strict recursion graph is realized by the evidence.
    pub strict_recursion_graph_realized: bool,
    /// The external tensor-contraction theorem has been independently verified.
    pub external_contraction_theorem_verified: bool,
    /// The evidence alone proves the claimed subexponential total-work bound.
    pub subexponential_total_work_certified: bool,
}

/// Paired component transcripts and their explicit boundary assertions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    /// Wire schema version.
    pub schema: u32,
    /// Exponent-only strict symmetric root and source-guard transcript.
    pub source: SourceCertificate,
    /// Normalized implicit strict-history transcript.
    pub history: HistoryCertificate,
    /// Claims checked against the independently replayed components.
    pub assertions: Assertions,
}

/// Checked facts and deliberately unproved conclusions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Shared root logarithmic size.
    pub lambda: u64,
    /// Shared root source scale.
    pub u: u64,
    /// Shared root packet exponent.
    pub a: u64,
    /// Exponent in the strict-root edge count `M=2^lambda`.
    pub strict_root_edge_exponent: u64,
    /// Exponent in the strict-root packet count `K=2^(lambda-a)`.
    pub strict_root_packet_exponent: u64,
    /// Multiplier in the normalized identity `M_norm=u*2^a`.
    pub normalized_root_edge_factor: u64,
    /// Normalized packet count `K_norm=3u`.
    pub normalized_root_packets: u64,
    /// Denominator in the symmetric source's exact child size `M/32`.
    pub source_child_shrink_factor: u64,
    /// Denominator in the symbolic history's child upper bound `M/8`.
    pub history_child_shrink_factor: u64,
    /// Whether the independently replayed schedules agree.
    pub schedule_compatible: bool,
    /// Whether the stronger source shrink satisfies the history inequality.
    pub source_bound_implies_history_bound: bool,
    /// Whether the normalized and strict-root dimension witnesses are equal.
    pub normalized_dimensions_equal_strict_root: bool,
    /// Whether this certificate realizes the complete strict recursion graph.
    pub strict_recursion_graph_realized: bool,
    /// Whether an independent proof of the external theorem is included.
    pub external_contraction_theorem_verified: bool,
    /// Whether the evidence proves the subexponential total-work conclusion.
    pub subexponential_total_work_certified: bool,
}

/// Pair two producer transcripts with the only assertions supported here.
pub fn assemble(source: SourceCertificate, history: HistoryCertificate) -> Certificate {
    Certificate {
        schema: SCHEMA,
        source,
        history,
        assertions: Assertions {
            schedule_compatible: true,
            source_bound_implies_history_bound: true,
            normalized_dimensions_equal_strict_root: false,
            strict_recursion_graph_realized: false,
            external_contraction_theorem_verified: false,
            subexponential_total_work_certified: false,
        },
    }
}

fn equals_power_of_two(value: u64, exponent: u64) -> bool {
    value != 0 && value.is_power_of_two() && u64::from(value.trailing_zeros()) == exponent
}

/// Replay both components and verify the exact composition boundary.
///
/// Acceptance certifies schedule compatibility and a one-step bound
/// implication only. It explicitly does not certify graph realization, the
/// external contraction theorem, or a Curve25519 attack.
pub fn verify(certificate: &Certificate) -> Result<VerificationSummary, String> {
    if certificate.schema != SCHEMA {
        return Err("composition boundary context".into());
    }
    let source = verify_source(&certificate.source)
        .map_err(|error| format!("composition source: {error}"))?;
    let history = verify_history(&certificate.history)
        .map_err(|error| format!("composition history: {error}"))?;

    let schedule_compatible =
        source.lambda == history.lambda && source.u == history.root_u && source.a == history.root_a;
    if !schedule_compatible {
        return Err("composition schedule mismatch".into());
    }
    let source_bound_implies_history_bound =
        source.child_edge_shrink_factor >= history.strict_edge_shrink_factor;
    if !source_bound_implies_history_bound {
        return Err("composition child bound mismatch".into());
    }

    let strict_root_packet_exponent = source
        .lambda
        .checked_sub(source.a)
        .ok_or_else(|| "composition root packet exponent".to_string())?;
    let normalized_edges_equal = equals_power_of_two(
        history.normalized_root_edge_factor,
        strict_root_packet_exponent,
    );
    let normalized_packets_equal =
        equals_power_of_two(history.normalized_root_packets, strict_root_packet_exponent);
    let normalized_dimensions_equal_strict_root =
        normalized_edges_equal && normalized_packets_equal;

    if certificate.assertions.schedule_compatible != schedule_compatible {
        return Err("composition schedule assertion".into());
    }
    if certificate.assertions.source_bound_implies_history_bound
        != source_bound_implies_history_bound
    {
        return Err("composition child bound assertion".into());
    }
    if certificate
        .assertions
        .normalized_dimensions_equal_strict_root
        != normalized_dimensions_equal_strict_root
    {
        return Err("composition normalized dimension assertion".into());
    }
    if certificate.assertions.strict_recursion_graph_realized {
        return Err("composition graph realization unsupported".into());
    }
    if certificate.assertions.external_contraction_theorem_verified {
        return Err("composition external theorem unsupported".into());
    }
    if certificate.assertions.subexponential_total_work_certified {
        return Err("composition total-work conclusion unsupported".into());
    }

    Ok(VerificationSummary {
        lambda: source.lambda,
        u: source.u,
        a: source.a,
        strict_root_edge_exponent: source.lambda,
        strict_root_packet_exponent,
        normalized_root_edge_factor: history.normalized_root_edge_factor,
        normalized_root_packets: history.normalized_root_packets,
        source_child_shrink_factor: source.child_edge_shrink_factor,
        history_child_shrink_factor: history.strict_edge_shrink_factor,
        schedule_compatible,
        source_bound_implies_history_bound,
        normalized_dimensions_equal_strict_root,
        strict_recursion_graph_realized: false,
        external_contraction_theorem_verified: false,
        subexponential_total_work_certified: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::admission::dimensions::symbolic_history_certificate;
    use crate::symmetric_source_family_certificate::produce as produce_source;
    use serde_json::json;

    const LAMBDA: u64 = 1 << 26;

    fn certificate(lambda: u64) -> Certificate {
        assemble(
            produce_source(lambda).unwrap(),
            symbolic_history_certificate(lambda).unwrap(),
        )
    }

    #[test]
    fn accepts_only_the_conditional_composition_boundary() {
        let summary = verify(&certificate(LAMBDA)).unwrap();
        assert_eq!(summary.u, 512);
        assert_eq!(summary.a, 1_024);
        assert_eq!(summary.strict_root_packet_exponent, LAMBDA - 1_024);
        assert_eq!(summary.normalized_root_edge_factor, 512);
        assert_eq!(summary.normalized_root_packets, 1_536);
        assert_eq!(summary.source_child_shrink_factor, 32);
        assert_eq!(summary.history_child_shrink_factor, 8);
        assert!(summary.schedule_compatible);
        assert!(summary.source_bound_implies_history_bound);
        assert!(!summary.normalized_dimensions_equal_strict_root);
        assert!(!summary.strict_recursion_graph_realized);
        assert!(!summary.external_contraction_theorem_verified);
        assert!(!summary.subexponential_total_work_certified);
    }

    #[test]
    fn rejects_individually_valid_but_mismatched_schedules() {
        let source = produce_source(1 << 27).unwrap();
        let history = symbolic_history_certificate(LAMBDA).unwrap();
        assert_eq!(
            verify(&assemble(source, history)).unwrap_err(),
            "composition schedule mismatch"
        );
    }

    #[test]
    fn rejects_mutated_source_schedule_and_shrink() {
        let mut wrong_a = certificate(LAMBDA);
        wrong_a.source.a += 1;
        assert_eq!(
            verify(&wrong_a).unwrap_err(),
            "composition source: symmetric source transcript mismatch"
        );

        let mut wrong_shrink = certificate(LAMBDA);
        wrong_shrink.source.child_edge_exponent += 1;
        assert_eq!(
            verify(&wrong_shrink).unwrap_err(),
            "composition source: symmetric source transcript mismatch"
        );
    }

    #[test]
    fn rejects_mutated_history_schedule_and_shrink() {
        let mut wrong_a = certificate(LAMBDA);
        let mut value = serde_json::to_value(&wrong_a.history).unwrap();
        value["depths"][0]["current_a"] = json!(1_025);
        wrong_a.history = serde_json::from_value(value).unwrap();
        assert_eq!(
            verify(&wrong_a).unwrap_err(),
            "composition history: symbolic history depth mismatch"
        );

        let mut wrong_shrink = certificate(LAMBDA);
        let mut value = serde_json::to_value(&wrong_shrink.history).unwrap();
        value["strict_edge_shrink_factor"] = json!(16);
        wrong_shrink.history = serde_json::from_value(value).unwrap();
        assert_eq!(
            verify(&wrong_shrink).unwrap_err(),
            "composition history: symbolic history constants"
        );
    }

    #[test]
    fn rejects_every_unsupported_promotion() {
        let cases: [(&str, fn(&mut Assertions)); 4] = [
            ("composition normalized dimension assertion", |assertions| {
                assertions.normalized_dimensions_equal_strict_root = true;
            }),
            ("composition graph realization unsupported", |assertions| {
                assertions.strict_recursion_graph_realized = true;
            }),
            ("composition external theorem unsupported", |assertions| {
                assertions.external_contraction_theorem_verified = true;
            }),
            (
                "composition total-work conclusion unsupported",
                |assertions| {
                    assertions.subexponential_total_work_certified = true;
                },
            ),
        ];
        for (expected, mutate) in cases {
            let mut promoted = certificate(LAMBDA);
            mutate(&mut promoted.assertions);
            assert_eq!(verify(&promoted).unwrap_err(), expected);
        }
    }
}
