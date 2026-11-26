//! Independent affine verifier for the single-target norm-one-torus m=7 run.
//!
//! This binary does not call the candidate projective arithmetic, retained
//! table, prefix directory, or solver. It rebuilds the public input, replays all
//! witnesses, and independently performs the complete affine 3+4 decision.

#[path = "support/norm_one_torus_m7.rs"]
mod common;

use std::collections::HashSet;
use std::fs;

use common::{
    point_from_record, result_digest, sha256_hex, verify_witness, CertificateBundle, Fixture,
};
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimePoint};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PointKey {
    x: u64,
    y: u64,
    infinity: bool,
}

impl From<PrimePoint> for PointKey {
    fn from(point: PrimePoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
            infinity: point.infinity,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct XKey {
    x: u64,
    infinity: bool,
}

impl From<PrimePoint> for XKey {
    fn from(point: PrimePoint) -> Self {
        Self {
            x: point.x,
            infinity: point.infinity,
        }
    }
}

struct IndependentSearch {
    decomposable: bool,
    triple_candidates: usize,
    quadruple_candidates: usize,
    triple_x_keys: usize,
    quadruple_x_keys: usize,
    triple_infinity: bool,
    quadruple_infinity: bool,
}

fn independent_search(fixture: &Fixture) -> IndependentSearch {
    let curve = &fixture.instance.curve;
    let mut triple_points: HashSet<PointKey> = HashSet::new();
    let mut triple_x: HashSet<XKey> = HashSet::new();
    let mut triple_candidates = 0usize;
    for first in 0..fixture.base.len() {
        for second in first..fixture.base.len() {
            let pair = curve.add(
                &mut GroupOps::default(),
                fixture.base[first],
                fixture.base[second],
            );
            for third in second..fixture.base.len() {
                let triple = curve.add(&mut GroupOps::default(), pair, fixture.base[third]);
                triple_points.insert(triple.into());
                triple_x.insert(triple.into());
                triple_candidates += 1;
            }
        }
    }
    assert_eq!(triple_candidates, 54_740);

    let mut quadruple_x: HashSet<XKey> = HashSet::new();
    let mut quadruple_candidates = 0usize;
    let mut decomposable = false;
    for first in 0..fixture.base.len() {
        for second in first..fixture.base.len() {
            let pair = curve.add(
                &mut GroupOps::default(),
                fixture.base[first],
                fixture.base[second],
            );
            for third in second..fixture.base.len() {
                let triple = curve.add(&mut GroupOps::default(), pair, fixture.base[third]);
                for fourth in third..fixture.base.len() {
                    let quadruple =
                        curve.add(&mut GroupOps::default(), triple, fixture.base[fourth]);
                    quadruple_x.insert(quadruple.into());
                    let complement = curve.add(
                        &mut GroupOps::default(),
                        fixture.target,
                        curve.neg(quadruple),
                    );
                    decomposable |= triple_points.contains(&complement.into());
                    quadruple_candidates += 1;
                }
            }
        }
    }
    assert_eq!(quadruple_candidates, 971_635);

    IndependentSearch {
        decomposable,
        triple_candidates,
        quadruple_candidates,
        triple_x_keys: triple_x.len(),
        quadruple_x_keys: quadruple_x.len(),
        triple_infinity: triple_x.contains(&XKey {
            x: 0,
            infinity: true,
        }),
        quadruple_infinity: quadruple_x.contains(&XKey {
            x: 0,
            infinity: true,
        }),
    }
}

#[derive(Serialize)]
struct VerificationReport {
    schema: &'static str,
    manifest_sha256: String,
    certificate_sha256: String,
    result_digest: String,
    target: common::PointRecord,
    planted_indices: Vec<usize>,
    triple_candidates: usize,
    quadruple_candidates: usize,
    triple_x_keys: usize,
    quadruple_x_keys: usize,
    witness_replays: usize,
    seed_replay: &'static str,
    affine_full_scan: &'static str,
    proof_boundary: &'static str,
}

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    assert_eq!(
        arguments.len(),
        5,
        "usage: norm_one_torus_m7_verify --manifest PATH --certificates PATH"
    );
    assert_eq!(arguments[1], "--manifest");
    assert_eq!(arguments[3], "--certificates");

    let (manifest, manifest_bytes) = common::read_manifest(&arguments[2]);
    let fixture = common::load_fixture(&manifest);
    let certificate_bytes = fs::read(&arguments[4]).expect("read certificates");
    let certificates: CertificateBundle =
        serde_json::from_slice(&certificate_bytes).expect("parse certificates");

    assert_eq!(
        certificates.schema,
        "isogeny-labs/norm-one-torus-m7-certificates/v1"
    );
    assert_eq!(certificates.manifest_sha256, sha256_hex(&manifest_bytes));
    assert_eq!(certificates.factor_base_points, fixture.base.len());
    assert_eq!(certificates.factor_base_abscissae, fixture.abscissae);
    assert_eq!(point_from_record(certificates.target), fixture.target);
    assert_eq!(certificates.planted_indices, fixture.planted_indices);
    assert!((4..=6).contains(&certificates.arms.len()));
    assert_eq!(
        certificates.result_digest,
        result_digest(&certificates.arms)
    );
    for arm in &certificates.arms {
        assert!(verify_witness(&fixture, arm.witness.as_deref()));
    }

    let independent = independent_search(&fixture);
    assert!(independent.decomposable);
    for arm in &certificates.arms {
        let (states, candidates, additions, keys, infinity) = match arm.retained_arity {
            3 => (
                27_370,
                971_635,
                1_028_789,
                independent.triple_x_keys,
                independent.triple_infinity,
            ),
            4 if arm.arm == "retain4-hash-sum" => (
                486_115,
                54_740,
                111_826,
                independent.quadruple_x_keys,
                independent.quadruple_infinity,
            ),
            4 => (
                486_115,
                54_740,
                57_154,
                independent.quadruple_x_keys,
                independent.quadruple_infinity,
            ),
            _ => panic!("unexpected retained arity"),
        };
        assert_eq!(arm.retained_states, states);
        assert_eq!(arm.candidates, candidates);
        assert_eq!(arm.table_probes, candidates);
        assert_eq!(arm.query_ops.additions, additions);
        assert_eq!(arm.distinct_keys, keys);
        let finite_keys = keys - usize::from(infinity);
        assert_eq!(
            arm.payload_bytes,
            finite_keys * 12 + usize::from(infinity) * 8
        );
        if arm.arm.contains("prefix16") {
            assert_eq!(arm.index_bytes, 65_537 * 4);
        } else {
            assert_eq!(arm.index_bytes, 0);
            assert_eq!(arm.key_comparisons, 0);
        }
        let affine_branches =
            arm.affine_generic + arm.affine_inverse + arm.affine_doubling + arm.affine_identity;
        if matches!(
            arm.arm.as_str(),
            "retain3-prefix16-affine-leaf" | "retain3-prefix16-affine-x-first"
        ) {
            assert_eq!(affine_branches, arm.candidates);
            assert_eq!(arm.query_ops.inversions, 476);
        } else {
            assert_eq!(affine_branches, 0);
        }
        if arm.arm == "retain3-prefix16-affine-x-first" {
            assert!(arm.affine_y_materializations <= 97_163);
        } else {
            assert_eq!(arm.affine_y_materializations, 0);
        }
    }
    if certificates.arms.len() >= 5 {
        let centered = certificates
            .arms
            .iter()
            .find(|arm| arm.arm == "retain3-prefix16-centered")
            .expect("centered retained-3 arm");
        let affine = certificates
            .arms
            .iter()
            .find(|arm| arm.arm == "retain3-prefix16-affine-leaf")
            .expect("affine-leaf arm");
        assert!(affine.query_ops.multiplications < centered.query_ops.multiplications);
        assert!(affine.query_ops.squarings < centered.query_ops.squarings);
    }
    if certificates.arms.len() == 6 {
        let affine = certificates
            .arms
            .iter()
            .find(|arm| arm.arm == "retain3-prefix16-affine-leaf")
            .expect("affine-leaf arm");
        let x_first = certificates
            .arms
            .iter()
            .find(|arm| arm.arm == "retain3-prefix16-affine-x-first")
            .expect("exact-x-first arm");
        assert_eq!(x_first.retained_states, affine.retained_states);
        assert_eq!(x_first.distinct_keys, affine.distinct_keys);
        assert_eq!(x_first.candidates, affine.candidates);
        assert_eq!(x_first.table_probes, affine.table_probes);
        assert_eq!(x_first.key_comparisons, affine.key_comparisons);
        assert_eq!(x_first.query_ops.additions, affine.query_ops.additions);
        assert_eq!(x_first.query_ops.inversions, affine.query_ops.inversions);
        assert_eq!(x_first.query_ops.squarings, affine.query_ops.squarings);
        assert_eq!(
            x_first.query_ops.multiplications,
            affine.query_ops.multiplications - affine.candidates
                + x_first.affine_y_materializations
        );
    }

    let report = VerificationReport {
        schema: "isogeny-labs/norm-one-torus-m7-verification/v1",
        manifest_sha256: sha256_hex(&manifest_bytes),
        certificate_sha256: sha256_hex(&certificate_bytes),
        result_digest: certificates.result_digest,
        target: certificates.target,
        planted_indices: certificates.planted_indices,
        triple_candidates: independent.triple_candidates,
        quadruple_candidates: independent.quadruple_candidates,
        triple_x_keys: independent.triple_x_keys,
        quadruple_x_keys: independent.quadruple_x_keys,
        witness_replays: certificates.arms.len(),
        seed_replay: "pass",
        affine_full_scan: "pass",
        proof_boundary:
            "transparent public-seed and witness replay; not zero knowledge or host attestation",
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
