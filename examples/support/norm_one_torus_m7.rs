//! Shared input and certificate contract for the exact single-target m=7
//! norm-one-torus experiment and its independent verifier.

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
    pub targets: usize,
    pub arity: usize,
    pub rounds: usize,
    pub query_block_size: usize,
    pub prefix_bits: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PointRecord {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationRecord {
    pub multiplications: u64,
    pub squarings: u64,
    pub inversions: u64,
    pub additions: u64,
    pub doublings: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArmCertificate {
    pub arm: String,
    pub retained_arity: usize,
    pub retained_states: usize,
    pub distinct_keys: usize,
    pub candidates: u64,
    pub table_probes: u64,
    pub key_comparisons: u64,
    pub setup_ops: OperationRecord,
    pub query_ops: OperationRecord,
    pub payload_bytes: usize,
    pub index_bytes: usize,
    pub container_capacity: usize,
    pub witness: Option<Vec<usize>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CertificateBundle {
    pub schema: String,
    pub manifest_sha256: String,
    pub result_digest: String,
    pub factor_base_points: usize,
    pub factor_base_abscissae: usize,
    pub target: PointRecord,
    pub planted_indices: Vec<usize>,
    pub arms: Vec<ArmCertificate>,
}

#[derive(Clone, Debug)]
pub struct Fixture {
    pub instance: PrimeInstance,
    pub base: Vec<PrimePoint>,
    pub neg_index: Vec<usize>,
    pub abscissae: usize,
    pub target: PrimePoint,
    pub planted_indices: Vec<usize>,
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

fn planted_indices(master: &[u8], domain: &str, arity: usize, neg_index: &[usize]) -> Vec<usize> {
    let mut selected = Vec::with_capacity(arity);
    for term in 0..arity {
        let mut candidate = stream_mod(
            master,
            domain,
            "planted-indices",
            term,
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
    assert_eq!(manifest.configuration.targets, 1, "single-target lane");
    assert_eq!(
        manifest.configuration.arity, 7,
        "this binary is the m=7 arm"
    );
    assert_eq!(manifest.configuration.prefix_bits, 16);
    assert!(manifest.configuration.query_block_size > 0);

    let instance = fp_p31_subexp_mersenne_instance();
    assert_eq!(manifest.configuration.curve, instance.name);
    let factor_base = dickson_factor_base(
        &instance,
        manifest.configuration.dickson_height,
        manifest.configuration.dickson_lo,
        manifest.configuration.dickson_c_top,
    )
    .expect("production Dickson factor base");
    assert_eq!(factor_base.abscissae, 34);
    assert_eq!(factor_base.points.len(), 68);
    assert!(factor_base
        .neg_index
        .iter()
        .enumerate()
        .all(|(index, &negated)| index != negated && factor_base.neg_index[negated] == index));

    let master = decode_hex(&manifest.master_seed_hex);
    assert_eq!(master.len(), 32, "the public master seed is 256 bits");
    let indices = planted_indices(
        &master,
        &manifest.stream_derivation.domain,
        manifest.configuration.arity,
        &factor_base.neg_index,
    );
    let target = indices
        .iter()
        .fold(instance.curve.identity(), |sum, &index| {
            instance
                .curve
                .add(&mut GroupOps::default(), sum, factor_base.points[index])
        });

    Fixture {
        instance,
        base: factor_base.points,
        neg_index: factor_base.neg_index,
        abscissae: factor_base.abscissae,
        target,
        planted_indices: indices,
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

pub fn verify_witness(fixture: &Fixture, witness: Option<&[usize]>) -> bool {
    let Some(witness) = witness else {
        return false;
    };
    if witness.len() != 7 || witness.iter().any(|&index| index >= fixture.base.len()) {
        return false;
    }
    witness
        .iter()
        .fold(fixture.instance.curve.identity(), |sum, &index| {
            fixture
                .instance
                .curve
                .add(&mut GroupOps::default(), sum, fixture.base[index])
        })
        == fixture.target
}

pub fn result_digest(arms: &[ArmCertificate]) -> String {
    let bytes = serde_json::to_vec(arms).expect("serialize deterministic arm certificates");
    sha256_hex(&bytes)
}

pub fn certificate_bundle(
    fixture: &Fixture,
    manifest_bytes: &[u8],
    arms: Vec<ArmCertificate>,
) -> CertificateBundle {
    assert_eq!(arms.len(), 4);
    assert!(arms
        .iter()
        .all(|arm| verify_witness(fixture, arm.witness.as_deref())));
    let result_digest = result_digest(&arms);
    CertificateBundle {
        schema: "isogeny-labs/norm-one-torus-m7-certificates/v1".into(),
        manifest_sha256: sha256_hex(manifest_bytes),
        result_digest,
        factor_base_points: fixture.base.len(),
        factor_base_abscissae: fixture.abscissae,
        target: point_record(fixture.target),
        planted_indices: fixture.planted_indices.clone(),
        arms,
    }
}
