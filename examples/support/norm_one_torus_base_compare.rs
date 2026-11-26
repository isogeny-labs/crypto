//! Shared input and transparent-certificate contract for the matched
//! norm-one-torus/ordinary factor-base comparison.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;

use crypto_lib::cryptanalysis::ic_boundary::{
    fp_p31_subexp_mersenne_instance, prime_factor_base, CountedGroup, GroupOps, PrimeInstance,
    PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::dickson_factor_base;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize)]
pub struct Manifest {
    pub schema: String,
    pub study_id: String,
    pub master_seed_hex: String,
    pub stream_derivation: StreamDerivation,
    pub configuration: Configuration,
    #[serde(default)]
    pub source_sha256: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamDerivation {
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub curve: String,
    pub dickson_height: u32,
    pub dickson_lo: u64,
    pub dickson_c_top: u64,
    pub random_targets: usize,
    pub planted_targets_per_base: usize,
    pub arity: usize,
    pub rounds: usize,
    pub query_block_size: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BaseKind {
    DicksonTorus,
    OrdinarySmallX,
}

impl BaseKind {
    pub const ALL: [Self; 2] = [Self::DicksonTorus, Self::OrdinarySmallX];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DicksonTorus => "dickson-torus",
            Self::OrdinarySmallX => "ordinary-small-x",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BaseBuildCounts {
    pub abscissae_scanned: u64,
    pub legendre_symbols: u64,
    pub selector_sqrt_solves: u64,
    pub curve_sqrt_solves: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PointRecord {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TargetCertificate {
    pub target: PointRecord,
    pub witness: Option<Vec<usize>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArmCertificate {
    pub base_kind: BaseKind,
    pub factor_base_sha256: String,
    pub result_digest: String,
    pub factor_base_points: usize,
    pub factor_base_abscissae: usize,
    pub random_targets: usize,
    pub targets: Vec<TargetCertificate>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CertificateBundle {
    pub schema: String,
    pub manifest_sha256: String,
    pub arms: Vec<ArmCertificate>,
}

#[derive(Clone, Debug)]
pub struct Fixture {
    pub instance: PrimeInstance,
    pub kind: BaseKind,
    pub base: Vec<PrimePoint>,
    pub neg_index: Vec<usize>,
    pub abscissae: usize,
    pub build_counts: BaseBuildCounts,
    pub random_targets: usize,
    pub targets: Vec<PrimePoint>,
}

pub fn read_manifest(path: &str) -> (Manifest, Vec<u8>) {
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("read manifest {path}: {error}"));
    let manifest = serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("parse manifest {path}: {error}"));
    (manifest, bytes)
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn decode_hex(value: &str) -> Vec<u8> {
    assert!(
        value.len().is_multiple_of(2),
        "hex input must have even length"
    );
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("hex is ASCII");
            u8::from_str_radix(text, 16).expect("valid hex")
        })
        .collect()
}

fn stream_bytes(master: &[u8], domain: &str, stream: &str, index: usize) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    hasher.update([0]);
    hasher.update(master);
    hasher.update([0]);
    hasher.update(stream.as_bytes());
    hasher.update([0]);
    hasher.update((index as u64).to_be_bytes());
    hasher.finalize().into()
}

fn stream_mod(master: &[u8], domain: &str, stream: &str, index: usize, modulus: u64) -> u64 {
    assert!(modulus > 0);
    stream_bytes(master, domain, stream, index)
        .into_iter()
        .fold(0u64, |accumulator, byte| {
            ((u128::from(accumulator) * 256 + u128::from(byte)) % u128::from(modulus)) as u64
        })
}

fn planted_indices(
    master: &[u8],
    domain: &str,
    kind: BaseKind,
    target_index: usize,
    arity: usize,
    neg_index: &[usize],
) -> Vec<usize> {
    let stream = format!("planted-indices/{}", kind.as_str());
    let mut selected = Vec::with_capacity(arity);
    for term in 0..arity {
        let stream_index = target_index * arity + term;
        let mut candidate = stream_mod(
            master,
            domain,
            &stream,
            stream_index,
            neg_index.len() as u64,
        ) as usize;
        while selected.contains(&candidate) || selected.contains(&neg_index[candidate]) {
            candidate = (candidate + 1) % neg_index.len();
        }
        selected.push(candidate);
    }
    selected.sort_unstable();
    selected
}

fn validate_signed_layout(base: &[PrimePoint], neg_index: &[usize], p: u64) {
    assert_eq!(base.len(), neg_index.len());
    for (index, (&point, &negative)) in base.iter().zip(neg_index).enumerate() {
        assert!(negative < base.len());
        assert_eq!(neg_index[negative], index);
        let expected = if point.infinity || point.y == 0 {
            point
        } else {
            PrimePoint::affine(point.x, p - point.y)
        };
        assert_eq!(base[negative], expected);
    }
}

fn factor_base_sha256(kind: BaseKind, base: &[PrimePoint], neg_index: &[usize]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"isogeny-labs/norm-one-torus-base/v1\0");
    hasher.update(kind.as_str().as_bytes());
    hasher.update([0]);
    for (&point, &negative) in base.iter().zip(neg_index) {
        hasher.update(point.x.to_be_bytes());
        hasher.update(point.y.to_be_bytes());
        hasher.update([u8::from(point.infinity)]);
        hasher.update((negative as u64).to_be_bytes());
    }
    format!("{:x}", hasher.finalize())
}

pub fn base_digest(fixture: &Fixture) -> String {
    factor_base_sha256(fixture.kind, &fixture.base, &fixture.neg_index)
}

pub fn load_fixtures(manifest: &Manifest) -> Vec<Fixture> {
    assert_eq!(manifest.schema, "isogeny-labs/experiment-seeds/v1");
    assert_eq!(manifest.configuration.arity, 5, "this is the m=5 arm");
    let instance = fp_p31_subexp_mersenne_instance();
    assert_eq!(manifest.configuration.curve, instance.name);
    let dickson = dickson_factor_base(
        &instance,
        manifest.configuration.dickson_height,
        manifest.configuration.dickson_lo,
        manifest.configuration.dickson_c_top,
    )
    .expect("production Dickson factor base");
    let ordinary = prime_factor_base(&instance, dickson.abscissae);
    assert_eq!(dickson.abscissae, ordinary.abscissae);
    assert_eq!(dickson.points.len(), ordinary.points.len());
    assert_eq!(dickson.points.len(), dickson.abscissae * 2);
    let matched_abscissae = dickson.abscissae;

    let master = decode_hex(&manifest.master_seed_hex);
    assert_eq!(master.len(), 32, "the public master seed is 256 bits");
    let domain = &manifest.stream_derivation.domain;
    let generator = instance.generator_point();
    let random_targets: Vec<_> = (0..manifest.configuration.random_targets)
        .map(|index| {
            let scalar = 1 + stream_mod(
                &master,
                domain,
                "common-random-targets",
                index,
                instance.r - 1,
            );
            instance
                .curve
                .mul(&mut GroupOps::default(), generator, scalar)
        })
        .collect();

    let selector_nodes = (1u64 << manifest.configuration.dickson_height) - 1;
    let dickson_roots = 1u64 << manifest.configuration.dickson_height;
    let ordinary_counts = BaseBuildCounts {
        abscissae_scanned: ordinary.cost.get("abscissae_scanned"),
        legendre_symbols: ordinary.cost.get("legendre_symbols"),
        selector_sqrt_solves: 0,
        curve_sqrt_solves: ordinary.cost.get("sqrt_solves"),
    };
    let arms = [
        (
            BaseKind::DicksonTorus,
            dickson.points,
            dickson.neg_index,
            BaseBuildCounts {
                abscissae_scanned: dickson_roots,
                legendre_symbols: 0,
                selector_sqrt_solves: selector_nodes,
                curve_sqrt_solves: dickson_roots,
            },
        ),
        (
            BaseKind::OrdinarySmallX,
            ordinary.points,
            ordinary.neg_index,
            ordinary_counts,
        ),
    ];

    arms.into_iter()
        .map(|(kind, base, neg_index, build_counts)| {
            validate_signed_layout(&base, &neg_index, instance.curve.p);
            let mut targets = random_targets.clone();
            for target_index in 0..manifest.configuration.planted_targets_per_base {
                let witness = planted_indices(
                    &master,
                    domain,
                    kind,
                    target_index,
                    manifest.configuration.arity,
                    &neg_index,
                );
                let target = witness
                    .iter()
                    .fold(instance.curve.identity(), |sum, &index| {
                        instance
                            .curve
                            .add(&mut GroupOps::default(), sum, base[index])
                    });
                targets.push(target);
            }
            Fixture {
                instance: instance.clone(),
                kind,
                base,
                neg_index,
                abscissae: matched_abscissae,
                build_counts,
                random_targets: manifest.configuration.random_targets,
                targets,
            }
        })
        .collect()
}

pub fn point_record(point: PrimePoint) -> PointRecord {
    PointRecord {
        x: point.x,
        y: point.y,
        infinity: point.infinity,
    }
}

pub fn point_from_record(point: PointRecord) -> PrimePoint {
    PrimePoint {
        x: point.x,
        y: point.y,
        infinity: point.infinity,
    }
}

pub fn verify_witness(fixture: &Fixture, target: PrimePoint, witness: Option<&[usize]>) -> bool {
    let Some(witness) = witness else {
        return true;
    };
    if witness.len() != 5 || witness.iter().any(|&index| index >= fixture.base.len()) {
        return false;
    }
    let sum = witness
        .iter()
        .fold(fixture.instance.curve.identity(), |sum, &index| {
            fixture
                .instance
                .curve
                .add(&mut GroupOps::default(), sum, fixture.base[index])
        });
    sum == target
}

pub fn verify_results(fixture: &Fixture, results: &[Option<Vec<usize>>]) -> bool {
    results.len() == fixture.targets.len()
        && results
            .iter()
            .zip(&fixture.targets)
            .all(|(witness, &target)| verify_witness(fixture, target, witness.as_deref()))
}

pub fn result_digest(results: &[Option<Vec<usize>>]) -> String {
    let mut hasher = Sha256::new();
    for result in results {
        match result {
            None => hasher.update([0]),
            Some(witness) => {
                hasher.update([1]);
                hasher.update((witness.len() as u64).to_be_bytes());
                for &index in witness {
                    hasher.update((index as u64).to_be_bytes());
                }
            }
        }
    }
    format!("{:x}", hasher.finalize())
}

pub fn certificate_bundle(
    fixtures: &[Fixture],
    manifest_bytes: &[u8],
    results: &[Vec<Option<Vec<usize>>>],
) -> CertificateBundle {
    assert_eq!(fixtures.len(), results.len());
    let arms = fixtures
        .iter()
        .zip(results)
        .map(|(fixture, results)| {
            assert!(verify_results(fixture, results));
            ArmCertificate {
                base_kind: fixture.kind,
                factor_base_sha256: base_digest(fixture),
                result_digest: result_digest(results),
                factor_base_points: fixture.base.len(),
                factor_base_abscissae: fixture.abscissae,
                random_targets: fixture.random_targets,
                targets: fixture
                    .targets
                    .iter()
                    .copied()
                    .zip(results.iter().cloned())
                    .map(|(target, witness)| TargetCertificate {
                        target: point_record(target),
                        witness,
                    })
                    .collect(),
            }
        })
        .collect();
    CertificateBundle {
        schema: "isogeny-labs/norm-one-torus-base-compare-certificates/v1".into(),
        manifest_sha256: sha256_hex(manifest_bytes),
        arms,
    }
}
