//! Independent affine verifier for retained norm-one-torus MITM certificates.
//!
//! This deliberately does not call the benchmark's Jacobian arithmetic or
//! solver. It rebuilds the production Dickson factor base, reconstructs all
//! affine three-sums, scans every affine two-sum, checks the complete hit/miss
//! mask, and replays every returned five-index witness.

#[path = "support/norm_one_torus_mitm.rs"]
mod common;

use std::collections::HashSet;
use std::fs;

use common::{point_from_record, result_digest, sha256_hex, verify_witness, CertificateBundle};
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

fn independent_mask(fixture: &common::Fixture) -> Vec<bool> {
    let curve = &fixture.instance.curve;
    let mut right: HashSet<PointKey> = HashSet::new();
    for first in 0..fixture.base.len() {
        for second in first..fixture.base.len() {
            let pair = curve.add(
                &mut GroupOps::default(),
                fixture.base[first],
                fixture.base[second],
            );
            for third in second..fixture.base.len() {
                right.insert(
                    curve
                        .add(&mut GroupOps::default(), pair, fixture.base[third])
                        .into(),
                );
            }
        }
    }
    fixture
        .targets
        .iter()
        .copied()
        .map(|target| {
            for first in 0..fixture.base.len() {
                for second in first..fixture.base.len() {
                    let pair = curve.add(
                        &mut GroupOps::default(),
                        fixture.base[first],
                        fixture.base[second],
                    );
                    let complement = curve.add(&mut GroupOps::default(), target, curve.neg(pair));
                    if right.contains(&complement.into()) {
                        return true;
                    }
                }
            }
            false
        })
        .collect()
}

#[derive(Serialize)]
struct VerificationReport {
    schema: &'static str,
    manifest_sha256: String,
    certificate_sha256: String,
    targets: usize,
    successes: usize,
    result_digest: String,
    witness_replay: &'static str,
    exhaustive_mask: &'static str,
}

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    assert_eq!(
        arguments.len(),
        5,
        "usage: norm_one_torus_mitm_verify --manifest PATH --certificates PATH"
    );
    assert_eq!(arguments[1], "--manifest");
    assert_eq!(arguments[3], "--certificates");
    let (manifest, manifest_bytes) = common::read_manifest(&arguments[2]);
    let certificate_bytes = fs::read(&arguments[4]).expect("read certificates");
    let certificates: CertificateBundle =
        serde_json::from_slice(&certificate_bytes).expect("parse certificates");
    assert_eq!(
        certificates.schema,
        "isogeny-labs/norm-one-torus-mitm-certificates/v1"
    );
    assert_eq!(certificates.manifest_sha256, sha256_hex(&manifest_bytes));

    let fixture = common::load_fixture(&manifest);
    assert_eq!(certificates.factor_base_points, fixture.base.len());
    assert_eq!(certificates.factor_base_abscissae, fixture.abscissae);
    assert_eq!(certificates.targets.len(), fixture.targets.len());
    let recorded_results: Vec<_> = certificates
        .targets
        .iter()
        .map(|certificate| certificate.witness.clone())
        .collect();
    for ((certificate, &target), witness) in certificates
        .targets
        .iter()
        .zip(&fixture.targets)
        .zip(&recorded_results)
    {
        assert_eq!(point_from_record(certificate.target), target);
        assert!(verify_witness(&fixture, target, witness.as_deref()));
    }
    assert_eq!(certificates.result_digest, result_digest(&recorded_results));

    let independent = independent_mask(&fixture);
    let recorded_mask: Vec<_> = recorded_results.iter().map(Option::is_some).collect();
    assert_eq!(independent, recorded_mask);
    let successes = recorded_mask.iter().filter(|&&value| value).count();
    assert!(successes >= manifest.configuration.planted_targets);

    let report = VerificationReport {
        schema: "isogeny-labs/norm-one-torus-mitm-verification/v1",
        manifest_sha256: sha256_hex(&manifest_bytes),
        certificate_sha256: sha256_hex(&certificate_bytes),
        targets: recorded_mask.len(),
        successes,
        result_digest: certificates.result_digest,
        witness_replay: "pass",
        exhaustive_mask: "pass",
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
