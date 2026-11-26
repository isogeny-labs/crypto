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
    /// Whether all fixed radix-64 strict child guards hold.
    pub strict_source_refinement_admitted: bool,
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
    Ok(VerificationSummary {
        edges: source.edges,
        packets: source.packets,
        packet_length: source.packet_length,
        tuples: children.tuples,
        groups: children.groups,
        children: children.children,
        maximum_child_edges: children.maximum_child_edges,
        maximum_child_degree: children.maximum_degree,
        normalized_precision_admitted: source.normalized_precision_admitted,
        strict_source_refinement_admitted: children.strict_source_refinement_admitted,
    })
}

/// Require the complete fixed radix-64 strict refinement, not only a report.
pub fn verify_strict(
    certificate: &Certificate,
    meter: &mut Meter,
) -> Result<VerificationSummary, String> {
    let summary = verify(certificate, meter)?;
    if !summary.strict_source_refinement_admitted {
        return Err("node refinement strict source guards".into());
    }
    Ok(summary)
}
