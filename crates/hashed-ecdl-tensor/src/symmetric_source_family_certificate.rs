//! Standalone exponent audit for a symmetric root source family.
//!
//! The represented graph has `N` vertices on each side, `L` parallel edges
//! between every pair, and cyclic labels in an alphabet of size `q=64^a`.
//! Only exponents are serialized; no astronomical graph is allocated.

use serde::{Deserialize, Serialize};

/// Current symmetric-source-family wire schema.
pub const SCHEMA: u32 = 1;
const EPSILON_DENOMINATOR: u64 = 128 * 1024;
const MINIMUM_LAMBDA: u64 = 1 << 26;

/// Exact power-of-two exponents for the root construction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    /// Wire schema version.
    pub schema: u32,
    /// Root edge-count exponent, so `M=2^lambda`.
    pub lambda: u64,
    /// Root source scale `floor(lambda/2^17)`.
    pub u: u64,
    /// Packet exponent `a=2u` at depth zero.
    pub a: u64,
    /// Vertex-count exponent per side, so `N=2^vertex_exponent`.
    pub vertex_exponent: u64,
    /// Label-alphabet exponent, so `q=2^label_exponent=64^a`.
    pub label_exponent: u64,
    /// Pair-label-array exponent, so `D=2^array_exponent=q^2`.
    pub array_exponent: u64,
    /// Uniform label-pair class-size exponent.
    pub class_size_exponent: u64,
    /// Packet-count exponent for exact `KL=M`.
    pub packet_count_exponent: u64,
    /// Maximum source-degree exponent.
    pub degree_exponent: u64,
    /// Power-of-two lower exponent for the source degree cap.
    pub delta_lower_exponent: u64,
    /// Power-of-two upper exponent for the source degree cap.
    pub delta_upper_exponent: u64,
    /// Edge-count exponent of every active group child.
    pub child_edge_exponent: u64,
}

/// Checked construction facts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Root edge-count exponent.
    pub lambda: u64,
    /// Packet exponent selected by the root schedule.
    pub a: u64,
    /// Vertex-count exponent on each bipartite side.
    pub vertex_exponent: u64,
    /// Exact packet-count exponent.
    pub packet_count_exponent: u64,
    /// Exact child edge shrink factor.
    pub child_edge_shrink_factor: u64,
    /// Whether all depth-zero source parameter guards hold by exponent bounds.
    pub source_guards_certified: bool,
    /// Whether cyclic labels give uniform classes and exact `KL=M` packing.
    pub symmetric_packing_certified: bool,
    /// Whether every active-coordinate group child has exactly `M/32` edges.
    pub strict_child_shrink_certified: bool,
}

fn checked_add(left: u64, right: u64, label: &str) -> Result<u64, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("{label} overflow"))
}

fn checked_mul(left: u64, right: u64, label: &str) -> Result<u64, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("{label} overflow"))
}

/// Construct the unique exponent transcript for one root logarithmic size.
pub fn produce(lambda: u64) -> Result<Certificate, String> {
    if lambda < MINIMUM_LAMBDA {
        return Err("symmetric source lambda below threshold".into());
    }
    let u = lambda / EPSILON_DENOMINATOR;
    let a = checked_mul(2, u, "symmetric source packet exponent")?;
    let remainder = lambda
        .checked_sub(a)
        .ok_or_else(|| "symmetric source vertex exponent".to_string())?;
    if remainder % 2 != 0 {
        return Err("symmetric source vertex parity".into());
    }
    let vertex_exponent = remainder / 2;
    let label_exponent = checked_mul(6, a, "symmetric source label exponent")?;
    let array_exponent = checked_mul(2, label_exponent, "symmetric source array exponent")?;
    let class_size_exponent = checked_add(
        checked_mul(
            2,
            vertex_exponent
                .checked_sub(label_exponent)
                .ok_or_else(|| "symmetric source label multiplicity".to_string())?,
            "symmetric source class exponent",
        )?,
        a,
        "symmetric source class exponent",
    )?;
    let packet_count_exponent = lambda
        .checked_sub(a)
        .ok_or_else(|| "symmetric source packet count".to_string())?;
    let degree_exponent = checked_add(vertex_exponent, a, "symmetric source degree")?;
    let nine_lambda = checked_mul(9, lambda, "symmetric source delta")?;
    let delta_lower_exponent = nine_lambda / 10;
    let delta_upper_exponent = delta_lower_exponent + u64::from(nine_lambda % 10 != 0);
    let child_edge_exponent = lambda
        .checked_sub(5)
        .ok_or_else(|| "symmetric source child edges".to_string())?;
    Ok(Certificate {
        schema: SCHEMA,
        lambda,
        u,
        a,
        vertex_exponent,
        label_exponent,
        array_exponent,
        class_size_exponent,
        packet_count_exponent,
        degree_exponent,
        delta_lower_exponent,
        delta_upper_exponent,
        child_edge_exponent,
    })
}

/// Verify the root construction using only checked exponent arithmetic.
pub fn verify(certificate: &Certificate) -> Result<VerificationSummary, String> {
    let expected = produce(certificate.lambda)?;
    if certificate != &expected {
        return Err("symmetric source transcript mismatch".into());
    }
    let length_array_exponent = checked_add(
        certificate.a,
        certificate.array_exponent,
        "symmetric source length-array guard",
    )?;
    let degree_array_guard_exponent = checked_add(
        checked_add(
            1,
            checked_mul(
                2,
                certificate.array_exponent,
                "symmetric source degree-array guard",
            )?,
            "symmetric source degree-array guard",
        )?,
        certificate.delta_upper_exponent,
        "symmetric source degree-array guard",
    )?;
    if certificate.u == 0
        || certificate.u > certificate.a
        || certificate.vertex_exponent < certificate.label_exponent
        || certificate.class_size_exponent < certificate.a
        || certificate.degree_exponent > certificate.delta_lower_exponent
        || checked_add(1, certificate.a, "symmetric source packet degree")?
            > certificate.delta_lower_exponent
        || length_array_exponent > certificate.lambda
        || degree_array_guard_exponent > certificate.lambda
        || checked_add(
            checked_mul(
                2,
                certificate.vertex_exponent,
                "symmetric source edge identity",
            )?,
            certificate.a,
            "symmetric source edge identity",
        )? != certificate.lambda
        || checked_add(
            certificate.packet_count_exponent,
            certificate.a,
            "symmetric source packing identity",
        )? != certificate.lambda
    {
        return Err("symmetric source guard inequality".into());
    }
    Ok(VerificationSummary {
        lambda: certificate.lambda,
        a: certificate.a,
        vertex_exponent: certificate.vertex_exponent,
        packet_count_exponent: certificate.packet_count_exponent,
        child_edge_shrink_factor: 32,
        source_guards_certified: true,
        symmetric_packing_certified: true,
        strict_child_shrink_certified: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_root_family_satisfies_every_source_guard() {
        let produced = produce(MINIMUM_LAMBDA).unwrap();
        assert_eq!(produced.u, 512);
        assert_eq!(produced.a, 1_024);
        assert_eq!(produced.vertex_exponent, 33_553_920);
        assert_eq!(produced.label_exponent, 6_144);
        assert_eq!(produced.array_exponent, 12_288);
        assert_eq!(produced.class_size_exponent, 67_096_576);
        assert_eq!(produced.packet_count_exponent, 67_107_840);
        assert_eq!(produced.degree_exponent, 33_554_944);
        assert_eq!(produced.delta_lower_exponent, 60_397_977);
        assert_eq!(produced.delta_upper_exponent, 60_397_978);
        assert_eq!(produced.child_edge_exponent, 67_108_859);
        let serialized = serde_json::to_vec(&produced).unwrap();
        let parsed: Certificate = serde_json::from_slice(&serialized).unwrap();
        assert_eq!(parsed, produced);
        assert_eq!(serde_json::to_vec(&parsed).unwrap(), serialized);
        let summary = verify(&parsed).unwrap();
        assert_eq!(summary.child_edge_shrink_factor, 32);
        assert!(summary.source_guards_certified);
        assert!(summary.symmetric_packing_certified);
        assert!(summary.strict_child_shrink_certified);
        for lambda in [MINIMUM_LAMBDA + 2, 1 << 27, 1 << 30] {
            verify(&produce(lambda).unwrap()).unwrap();
        }

        let mut bad_degree = parsed;
        bad_degree.degree_exponent += 1;
        assert_eq!(
            verify(&bad_degree).unwrap_err(),
            "symmetric source transcript mismatch"
        );
        assert_eq!(
            produce(MINIMUM_LAMBDA + 1).unwrap_err(),
            "symmetric source vertex parity"
        );
    }
}
