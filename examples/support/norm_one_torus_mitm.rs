//! Shared, non-timed input and certificate code for the norm-one-torus MITM
//! benchmark and its separately implemented verifier.

// Each of the two examples uses a different half of this shared contract.
#![allow(dead_code)]

use std::fs;

use crypto_lib::cryptanalysis::ic_boundary::{
    fp_p31_subexp_mersenne_instance, CountedGroup, GroupOps, PrimeInstance, PrimePoint,
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
    pub planted_targets: usize,
    pub arity: usize,
    pub rounds: usize,
    pub query_block_size: usize,
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
pub struct CertificateBundle {
    pub schema: String,
    pub manifest_sha256: String,
    pub result_digest: String,
    pub factor_base_points: usize,
    pub factor_base_abscissae: usize,
    pub targets: Vec<TargetCertificate>,
}

#[derive(Clone, Debug)]
pub struct Fixture {
    pub instance: PrimeInstance,
    pub base: Vec<PrimePoint>,
    pub neg_index: Vec<usize>,
    pub abscissae: usize,
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
    assert!(value.len() % 2 == 0, "hex input must have even length");
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
    target_index: usize,
    arity: usize,
    neg_index: &[usize],
) -> Vec<usize> {
    let mut selected = Vec::with_capacity(arity);
    for term in 0..arity {
        let stream_index = target_index * arity + term;
        let mut candidate = stream_mod(
            master,
            domain,
            "planted-indices",
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

pub fn load_fixture(manifest: &Manifest) -> Fixture {
    assert_eq!(manifest.schema, "isogeny-labs/experiment-seeds/v1");
    assert_eq!(
        manifest.configuration.arity, 5,
        "this binary is the m=5 arm"
    );
    let instance = fp_p31_subexp_mersenne_instance();
    assert_eq!(manifest.configuration.curve, instance.name);
    let factor_base = dickson_factor_base(
        &instance,
        manifest.configuration.dickson_height,
        manifest.configuration.dickson_lo,
        manifest.configuration.dickson_c_top,
    )
    .expect("production Dickson factor base");
    let master = decode_hex(&manifest.master_seed_hex);
    assert_eq!(master.len(), 32, "the public master seed is 256 bits");
    let domain = &manifest.stream_derivation.domain;
    let mut targets = Vec::with_capacity(
        manifest.configuration.random_targets + manifest.configuration.planted_targets,
    );
    let generator = instance.generator_point();
    for index in 0..manifest.configuration.random_targets {
        let scalar = stream_mod(&master, domain, "random-targets", index, instance.r);
        targets.push(
            instance
                .curve
                .mul(&mut GroupOps::default(), generator, scalar),
        );
    }
    for target_index in 0..manifest.configuration.planted_targets {
        let witness = planted_indices(
            &master,
            domain,
            target_index,
            manifest.configuration.arity,
            &factor_base.neg_index,
        );
        let target = witness
            .iter()
            .fold(instance.curve.identity(), |sum, &index| {
                instance
                    .curve
                    .add(&mut GroupOps::default(), sum, factor_base.points[index])
            });
        targets.push(target);
    }
    Fixture {
        instance,
        base: factor_base.points,
        neg_index: factor_base.neg_index,
        abscissae: factor_base.abscissae,
        targets,
    }
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
    fixture: &Fixture,
    manifest_bytes: &[u8],
    results: &[Option<Vec<usize>>],
) -> CertificateBundle {
    assert!(verify_results(fixture, results));
    CertificateBundle {
        schema: "isogeny-labs/norm-one-torus-mitm-certificates/v1".into(),
        manifest_sha256: sha256_hex(manifest_bytes),
        result_digest: result_digest(results),
        factor_base_points: fixture.base.len(),
        factor_base_abscissae: fixture.abscissae,
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
}
