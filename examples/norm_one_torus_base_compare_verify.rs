//! Independent affine verifier for the matched norm-one-torus/ordinary
//! streamed-two-sum comparison.
//!
//! This file does not call the candidate Jacobian arithmetic or streaming
//! solver. It rebuilds both production bases and public target streams,
//! enumerates the complete affine three-sum set, streams every affine
//! two-sum, checks all hit/miss decisions, and replays every witness.

#[path = "support/norm_one_torus_base_compare.rs"]
mod common;

use std::collections::HashSet;
use std::fs;

use common::{
    base_digest, point_from_record, result_digest, sha256_hex, verify_witness, BaseKind,
    CertificateBundle,
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

fn independent_mask(fixture: &common::Fixture) -> Vec<bool> {
    let curve = &fixture.instance.curve;
    let mut triples: HashSet<PointKey> = HashSet::new();
    for first in 0..fixture.base.len() {
        for second in first..fixture.base.len() {
            let pair = curve.add(
                &mut GroupOps::default(),
                fixture.base[first],
                fixture.base[second],
            );
            for third in second..fixture.base.len() {
                triples.insert(
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
                    if triples.contains(&complement.into()) {
                        return true;
                    }
                }
            }
            false
        })
        .collect()
}

#[derive(Serialize)]
struct ArmVerification {
    base_kind: BaseKind,
    factor_base_sha256: String,
    targets: usize,
    random_targets: usize,
    random_successes: usize,
    planted_successes: usize,
    result_digest: String,
    witness_replay: &'static str,
    exhaustive_mask: &'static str,
}

#[derive(Serialize)]
struct VerificationReport {
    schema: &'static str,
    manifest_sha256: String,
    certificate_sha256: String,
    common_random_targets: &'static str,
    arms: Vec<ArmVerification>,
}

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    assert_eq!(
        arguments.len(),
        5,
        "usage: norm_one_torus_base_compare_verify --manifest PATH --certificates PATH"
    );
    assert_eq!(arguments[1], "--manifest");
    assert_eq!(arguments[3], "--certificates");
    let (manifest, manifest_bytes) = common::read_manifest(&arguments[2]);
    let certificate_bytes = fs::read(&arguments[4]).expect("read certificates");
    let certificates: CertificateBundle =
        serde_json::from_slice(&certificate_bytes).expect("parse certificates");
    assert_eq!(
        certificates.schema,
        "isogeny-labs/norm-one-torus-base-compare-certificates/v1"
    );
    assert_eq!(certificates.manifest_sha256, sha256_hex(&manifest_bytes));

    let fixtures = common::load_fixtures(&manifest);
    assert_eq!(fixtures.len(), certificates.arms.len());
    let random_targets = manifest.configuration.random_targets;
    assert_eq!(
        &fixtures[0].targets[..random_targets],
        &fixtures[1].targets[..random_targets]
    );

    let mut arms = Vec::new();
    for fixture in &fixtures {
        let certificate = certificates
            .arms
            .iter()
            .find(|certificate| certificate.base_kind == fixture.kind)
            .expect("certificate for both fixed arms");
        assert_eq!(certificate.factor_base_sha256, base_digest(fixture));
        assert_eq!(certificate.factor_base_points, fixture.base.len());
        assert_eq!(certificate.factor_base_abscissae, fixture.abscissae);
        assert_eq!(certificate.random_targets, random_targets);
        assert_eq!(certificate.targets.len(), fixture.targets.len());

        let recorded_results: Vec<_> = certificate
            .targets
            .iter()
            .map(|target| target.witness.clone())
            .collect();
        for ((record, &target), witness) in certificate
            .targets
            .iter()
            .zip(&fixture.targets)
            .zip(&recorded_results)
        {
            assert_eq!(point_from_record(record.target), target);
            assert!(verify_witness(fixture, target, witness.as_deref()));
        }
        assert_eq!(certificate.result_digest, result_digest(&recorded_results));

        let independent = independent_mask(fixture);
        let recorded_mask: Vec<_> = recorded_results.iter().map(Option::is_some).collect();
        assert_eq!(independent, recorded_mask);
        let random_successes = recorded_mask[..random_targets]
            .iter()
            .filter(|&&hit| hit)
            .count();
        let planted_successes = recorded_mask[random_targets..]
            .iter()
            .filter(|&&hit| hit)
            .count();
        assert_eq!(
            planted_successes,
            manifest.configuration.planted_targets_per_base
        );
        arms.push(ArmVerification {
            base_kind: fixture.kind,
            factor_base_sha256: certificate.factor_base_sha256.clone(),
            targets: recorded_mask.len(),
            random_targets,
            random_successes,
            planted_successes,
            result_digest: certificate.result_digest.clone(),
            witness_replay: "pass",
            exhaustive_mask: "pass",
        });
    }

    let report = VerificationReport {
        schema: "isogeny-labs/norm-one-torus-base-compare-verification/v1",
        manifest_sha256: sha256_hex(&manifest_bytes),
        certificate_sha256: sha256_hex(&certificate_bytes),
        common_random_targets: "pass",
        arms,
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
