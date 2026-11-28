//! Standalone composition of one finite recursive-node refinement witness.
//!
//! This module imports only serialized certificate verifiers. It does not
//! import graph, packing, deformation, packet-tensor, or contraction code.

use crate::child_provenance_certificate::{
    verify_family as verify_child_family, Certificate as ChildCertificate,
};
use crate::factorization_certificate::{
    verify as verify_factorization, verify_source_dimensions,
    Certificate as FactorizationCertificate, SourceDimensionWitness,
};
use crate::packing_provenance_certificate::{
    verify as verify_packing, Certificate as PackingCertificate,
};
use crate::{BigUint, Meter};
use serde::{Deserialize, Serialize};

/// Current composed node-refinement wire schema.
pub const SCHEMA: u32 = 1;

/// Fixed natural-number projection consumed by the formal history-shrink
/// importer. The eight fields are schema, radix, parent edges, packet length,
/// label classes, maximum class, padded height, and maximum child edges.
pub const HISTORY_SHRINK_PROJECTION_SCHEMA: u32 = 1;
/// Exact field count of the compact history-shrink projection.
pub const HISTORY_SHRINK_PROJECTION_FIELDS: usize = 8;

/// Complete finite evidence for one recursion node.
#[derive(Clone, Serialize, Deserialize)]
pub struct Certificate {
    /// Wire schema version.
    pub schema: u32,
    /// Source coordinates refining the factorization object.
    pub source: SourceDimensionWitness,
    /// Independently checked factorization algebra and coefficient tables.
    pub factorization: FactorizationCertificate,
    /// Independently replayed labeled source packing.
    pub packing: PackingCertificate,
    /// Exact Cartesian family of tuple/group children.
    pub children: Vec<ChildCertificate>,
}

/// Checked dimensions and admission facts for one composed finite node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Parent edge count `M`.
    pub edges: BigUint,
    /// Packet count `K`.
    pub packets: BigUint,
    /// Packet length `L=2^a`.
    pub packet_length: BigUint,
    /// Ordered label-class count `D`.
    pub label_classes: usize,
    /// Largest recomputed endpoint-label class.
    pub maximum_class: usize,
    /// Exact native `ceil(maximum_class/L)*L` class height.
    pub padded_class_height: usize,
    /// Number of retained mode tuples.
    pub tuples: usize,
    /// Number of radix groups.
    pub groups: usize,
    /// Number of verified tuple/group children.
    pub children: usize,
    /// Maximum edge count of any one child.
    pub maximum_child_edges: usize,
    /// Maximum child degree.
    pub maximum_child_degree: usize,
    /// Whether normalized precision `p u<=34 M D` was admitted.
    pub normalized_precision_admitted: bool,
    /// Whether source guards imply uniform packing and group-load closure.
    pub source_guard_closure_admitted: bool,
    /// Whether the explicit source length-mass premise `L D<=M` holds.
    pub source_length_mass_admitted: bool,
    /// Whether exact child replay proves the rounded-height two-sided bound.
    pub rounded_height_child_bound_verified: bool,
    /// Whether the packing facts derive eightfold shrink for radix 64.
    pub history_shrink_derived_from_packing: bool,
    /// Whether all fixed radix-64 strict child guards hold.
    pub strict_source_refinement_admitted: bool,
}

/// Canonical natural-number projection of the exact history-shrink inputs.
///
/// This is deliberately derived from a successful verification summary, not
/// from caller-supplied serialized fields. Formal source-guard acceptance is
/// still checked separately because this compact projection does not contain
/// the complete packing-label trace.
pub fn history_shrink_projection(summary: &VerificationSummary) -> [BigUint; 8] {
    [
        BigUint::from(HISTORY_SHRINK_PROJECTION_SCHEMA),
        BigUint::from(summary.groups),
        summary.edges.clone(),
        summary.packet_length.clone(),
        BigUint::from(summary.label_classes),
        BigUint::from(summary.maximum_class),
        BigUint::from(summary.padded_class_height),
        BigUint::from(summary.maximum_child_edges),
    ]
}

/// Independently check the arithmetic carried by the compact history-shrink
/// projection. Passing this check is not a source-guard proof: the formal
/// importer additionally aligns the projection with a strictly parsed packing
/// source before deriving eightfold shrink.
pub fn verify_history_shrink_projection(values: &[BigUint]) -> Result<(), String> {
    if values.len() != HISTORY_SHRINK_PROJECTION_FIELDS
        || values[0] != BigUint::from(HISTORY_SHRINK_PROJECTION_SCHEMA)
        || values[1] != BigUint::from(64u8)
        || values[3] == BigUint::from(0u8)
    {
        return Err("node refinement history projection context".into());
    }
    let rounded_height = ((&values[5] + &values[3] - BigUint::from(1u8)) / &values[3]) * &values[3];
    if values[6] != rounded_height
        || &values[3] * &values[4] > values[2]
        || &values[7] * BigUint::from(64u8) > BigUint::from(2u8) * &values[6] * &values[4]
    {
        return Err("node refinement history projection arithmetic".into());
    }
    Ok(())
}

/// Compose all finite node interfaces and report strict admission.
///
/// The factorization object is verified first. Subsequent verifiers may then
/// rely on it as an accepted interface. This function is generic over finite
/// certificate size; it does not assert that a uniform asymptotic family of
/// accepted certificates exists.
pub fn verify(certificate: &Certificate, meter: &mut Meter) -> Result<VerificationSummary, String> {
    if certificate.schema != SCHEMA || !certificate.source.require_normalized_precision {
        return Err("node refinement context".into());
    }
    verify_factorization(&certificate.factorization, meter)?;
    let source = verify_source_dimensions(&certificate.factorization, &certificate.source)?;
    let packing = verify_packing(&certificate.factorization, &certificate.packing, meter)?;
    let children = verify_child_family(
        &certificate.packing,
        &certificate.factorization,
        &certificate.children,
        meter,
    )?;
    if source.edges != BigUint::from(packing.edges)
        || source.packets != BigUint::from(packing.packets)
        || source.packet_length != BigUint::from(packing.packet_length)
        || certificate.source.a != certificate.packing.packet_exponent
        || certificate.source.coordinate != certificate.packing.coordinate
        || children.tuples != certificate.factorization.tuples.len()
        || children.groups != certificate.packing.radix
    {
        return Err("node refinement dimensions".into());
    }
    let history_shrink_derived_from_packing = certificate.packing.radix == 64
        && packing.source_guard_closure_admitted
        && packing.source_length_mass_admitted
        && children.rounded_height_child_bound_verified;
    if history_shrink_derived_from_packing && !children.eightfold_edge_shrink_admitted {
        return Err("node refinement derived child shrink".into());
    }
    let strict_source_refinement_admitted =
        history_shrink_derived_from_packing && children.strict_source_refinement_admitted;
    let summary = VerificationSummary {
        edges: source.edges,
        packets: source.packets,
        packet_length: source.packet_length,
        label_classes: packing.label_classes,
        maximum_class: packing.maximum_class,
        padded_class_height: packing.padded_class_height,
        tuples: children.tuples,
        groups: children.groups,
        children: children.children,
        maximum_child_edges: children.maximum_child_edges,
        maximum_child_degree: children.maximum_degree,
        normalized_precision_admitted: source.normalized_precision_admitted,
        source_guard_closure_admitted: packing.source_guard_closure_admitted,
        source_length_mass_admitted: packing.source_length_mass_admitted,
        rounded_height_child_bound_verified: children.rounded_height_child_bound_verified,
        history_shrink_derived_from_packing,
        strict_source_refinement_admitted,
    };
    if history_shrink_derived_from_packing
        && verify_history_shrink_projection(&history_shrink_projection(&summary)).is_err()
    {
        return Err("node refinement history projection".into());
    }
    Ok(summary)
}

/// Require the complete fixed radix-64 strict refinement, not only a report.
pub fn verify_strict(
    certificate: &Certificate,
    meter: &mut Meter,
) -> Result<VerificationSummary, String> {
    let summary = verify(certificate, meter)?;
    if !summary.source_guard_closure_admitted
        || !summary.history_shrink_derived_from_packing
        || !summary.strict_source_refinement_admitted
    {
        return Err("node refinement strict source guards".into());
    }
    Ok(summary)
}
