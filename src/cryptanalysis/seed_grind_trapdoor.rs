//! Reproducible honest-prior sampling for the N=51 GHS audit.
//!
//! This module measures the distribution of the existing auditor's output.
//! It does not perform a seed search or assert that a low-genus cover gives
//! an ECDLP speedup. Each index maps to a uniform `a` and a uniform nonzero
//! `b` in the polynomial basis via domain-separated BLAKE3 XOF blocks.

use crate::binary_ecc::{F2mElement, IrreduciblePoly};
use crate::cryptanalysis::ec_trapdoor::audit_curve;
use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use num_bigint::BigUint;
use serde::Serialize;
use std::collections::BTreeMap;

pub const DEGREE: u32 = 51;
pub const GENERATOR_VERSION: &str = "ghs-honest-prior-v1-blake3-xof";

/// The least sparse irreducible modulus selected by the repository's field
/// constructor. Its exact low terms are written in each run manifest.
pub fn modulus() -> IrreduciblePoly {
    find_irreducible_sparse(DEGREE).expect("degree 51 must have a sparse irreducible modulus")
}

fn coefficient(seed: &[u8; 32], index: u64, label: &[u8], nonzero: bool) -> F2mElement {
    let mut retry = 0u64;
    loop {
        let mut h = blake3::Hasher::new_derive_key("crypto/ghs-honest-prior/v1");
        h.update(seed);
        h.update(&index.to_le_bytes());
        h.update(label);
        h.update(&retry.to_le_bytes());
        let mut bytes = [0u8; 8];
        h.finalize_xof().fill(&mut bytes);
        let bits = u64::from_le_bytes(bytes) & ((1u64 << DEGREE) - 1);
        if !nonzero || bits != 0 {
            return F2mElement::from_biguint(&BigUint::from(bits), DEGREE);
        }
        retry += 1;
    }
}

/// A deterministic map from `(seed, index)` to an ordinary binary curve.
pub fn coefficients(seed: &[u8; 32], index: u64) -> (F2mElement, F2mElement) {
    (
        coefficient(seed, index, b"a", false),
        coefficient(seed, index, b"b", true),
    )
}

#[derive(Clone, Debug, Serialize)]
pub struct AuditObservation {
    pub n: u32,
    pub l: u32,
    pub magic_m: u32,
    pub genus: String,
    pub type_i: bool,
    pub descends: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct PriorObservation {
    pub index: u64,
    pub min_applicable_genus: String,
    pub min_applicable_l: u32,
    pub rows: Vec<AuditObservation>,
}

/// Audit one member of the honest prior. The minimum is over proper
/// intermediate subfields (`l=3,17`); the GF(2) row (`l=1`) remains in the
/// raw audit for transparency, but is outside this trapdoor class.
pub fn observe(seed: &[u8; 32], index: u64, irr: &IrreduciblePoly) -> PriorObservation {
    let (a, b) = coefficients(seed, index);
    let rows = audit_curve(DEGREE, irr, &a, &b);
    let best = rows
        .iter()
        .filter(|r| r.descends && r.l > 1)
        .min_by(|x, y| x.genus.cmp(&y.genus).then(y.l.cmp(&x.l)))
        .expect("N=51 odd: every row descends");
    PriorObservation {
        index,
        min_applicable_genus: best.genus.to_string(),
        min_applicable_l: best.l,
        rows: rows
            .into_iter()
            .map(|r| AuditObservation {
                n: r.n,
                l: r.l,
                magic_m: r.magic_m,
                genus: r.genus.to_string(),
                type_i: r.type_i,
                descends: r.descends,
            })
            .collect(),
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PriorSummary {
    pub schema: &'static str,
    pub generator: &'static str,
    pub degree: u32,
    pub modulus_low_terms: Vec<u32>,
    pub seed_hex: String,
    pub samples: u64,
    pub histogram: BTreeMap<String, u64>,
    pub min_l_histogram: BTreeMap<u32, u64>,
    pub thresholds: BTreeMap<String, ThresholdCount>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ThresholdCount {
    pub hits: u64,
    pub empirical_p: f64,
    pub empirical_grind_bits: Option<f64>,
    pub zero_hit_upper_95: Option<f64>,
}

/// Aggregate a complete, contiguous sequence of observations. The raw rows
/// are kept by the caller so every count can be recomputed independently.
pub fn summarize(
    seed: &[u8; 32],
    irr: &IrreduciblePoly,
    observations: &[PriorObservation],
) -> PriorSummary {
    let mut histogram = BTreeMap::new();
    let mut min_l_histogram = BTreeMap::new();
    for (i, o) in observations.iter().enumerate() {
        assert_eq!(o.index, i as u64, "observation index gap");
        *histogram.entry(o.min_applicable_genus.clone()).or_insert(0) += 1;
        *min_l_histogram.entry(o.min_applicable_l).or_insert(0) += 1;
    }
    let n = observations.len() as u64;
    let mut thresholds = BTreeMap::new();
    for bound in [0u32, 1, 2, 3, 4, 8] {
        let hits = observations
            .iter()
            .filter(|o| {
                o.min_applicable_genus
                    .parse::<BigUint>()
                    .expect("genus integer")
                    <= BigUint::from(bound)
            })
            .count() as u64;
        let p = if n == 0 { 0.0 } else { hits as f64 / n as f64 };
        thresholds.insert(
            bound.to_string(),
            ThresholdCount {
                hits,
                empirical_p: p,
                empirical_grind_bits: if hits == 0 { None } else { Some(-p.log2()) },
                zero_hit_upper_95: if hits == 0 && n > 0 {
                    Some(1.0 - 0.05f64.powf(1.0 / n as f64))
                } else {
                    None
                },
            },
        );
    }
    PriorSummary {
        schema: "ghs-honest-prior-summary-v1",
        generator: GENERATOR_VERSION,
        degree: DEGREE,
        modulus_low_terms: irr.low_terms.clone(),
        seed_hex: hex::encode(seed),
        samples: n,
        histogram,
        min_l_histogram,
        thresholds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::koblitz_index_calculus::is_irreducible_f2;

    #[test]
    fn modulus_and_seed_map_are_valid() {
        let irr = modulus();
        let bits = (1u64 << DEGREE) | irr.low_terms.iter().fold(0u64, |v, &k| v | (1u64 << k));
        assert!(is_irreducible_f2(bits));
        let seed = [0x42u8; 32];
        let (a, b) = coefficients(&seed, 0);
        assert!(!b.is_zero());
        assert_eq!(a, coefficients(&seed, 0).0);
        assert_ne!(b, coefficients(&seed, 1).1);
        let o = observe(&seed, 0, &irr);
        assert_eq!(o.rows.len(), 3);
        assert!(o.rows.iter().all(|r| r.descends));
        assert_eq!(
            o.rows
                .iter()
                .map(|r| r.l)
                .collect::<std::collections::BTreeSet<_>>(),
            [1, 3, 17].into_iter().collect()
        );
    }

    #[test]
    fn summary_counts_all_observations() {
        let seed = [7u8; 32];
        let irr = modulus();
        let observations: Vec<_> = (0..8).map(|i| observe(&seed, i, &irr)).collect();
        let s = summarize(&seed, &irr, &observations);
        assert_eq!(s.histogram.values().sum::<u64>(), 8);
        assert_eq!(s.min_l_histogram.values().sum::<u64>(), 8);
        assert!(s.thresholds["4"].hits >= s.thresholds["3"].hits);
    }
}
