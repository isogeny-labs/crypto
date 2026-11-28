//! Extraction of the PR #5229 fixture contract, for a crate that builds
//! without the root library (whose front end needs more than 7 GB to type-check).
//!
//! Sources reproduced here, behaviour for behaviour:
//! * `fp_p31_subexp_mersenne_instance` (src/cryptanalysis/ic_boundary.rs)
//! * `dickson_abscissae`, `dickson_factor_base`
//!   (src/cryptanalysis/ic_framework/pkm_semaev.rs)
//! * `read_manifest`, `load_fixture`, `verify_results`, `result_digest`
//!   (examples/support/norm_one_torus_mitm.rs)
//!
//! Square roots are canonicalised with `min(r, p - r)` in both originals, so the
//! choice of square-root algorithm does not affect point order or values. The
//! `fidelity` mode checks the extraction against the library-produced
//! `public-precommit/results.csv` counters, result digest and certificates.

use std::fs;

use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimePoint {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

impl PrimePoint {
    pub const INFINITY: Self = Self {
        x: 0,
        y: 0,
        infinity: true,
    };
    pub fn affine(x: u64, y: u64) -> Self {
        Self {
            x,
            y,
            infinity: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrimeCurve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

fn mulm(l: u64, r: u64, p: u64) -> u64 {
    ((u128::from(l) * u128::from(r)) % u128::from(p)) as u64
}
fn addm(l: u64, r: u64, p: u64) -> u64 {
    ((u128::from(l) + u128::from(r)) % u128::from(p)) as u64
}
fn subm(l: u64, r: u64, p: u64) -> u64 {
    addm(l, p - r % p, p)
}
fn powm(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut acc = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            acc = mulm(acc, b, p);
        }
        b = mulm(b, b, p);
        e >>= 1;
    }
    acc
}
fn invm(v: u64, p: u64) -> u64 {
    assert_ne!(v % p, 0);
    powm(v, p - 2, p)
}

impl PrimeCurve {
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addm(addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p), self.b, p)
    }

    /// A square root for `p ≡ 3 (mod 4)` (the only case this fixture needs).
    pub fn sqrt(&self, n: u64) -> Option<u64> {
        let p = self.p;
        assert_eq!(p % 4, 3, "fixture extraction supports p = 3 mod 4 only");
        let n = n % p;
        if n == 0 {
            return Some(0);
        }
        let r = powm(n, (p + 1) / 4, p);
        (mulm(r, r, p) == n).then_some(r)
    }

    pub fn identity(&self) -> PrimePoint {
        PrimePoint::INFINITY
    }

    pub fn neg(&self, pt: PrimePoint) -> PrimePoint {
        if pt.infinity || pt.y == 0 {
            pt
        } else {
            PrimePoint::affine(pt.x, self.p - pt.y)
        }
    }

    /// Affine group law (independent of the Jacobian solver code).
    pub fn add(&self, a: PrimePoint, b: PrimePoint) -> PrimePoint {
        let p = self.p;
        if a.infinity {
            return b;
        }
        if b.infinity {
            return a;
        }
        let lambda = if a.x == b.x {
            if addm(a.y, b.y, p) == 0 {
                return PrimePoint::INFINITY;
            }
            let num = addm(mulm(3, mulm(a.x, a.x, p), p), self.a, p);
            mulm(num, invm(mulm(2, a.y, p), p), p)
        } else {
            mulm(subm(b.y, a.y, p), invm(subm(b.x, a.x, p), p), p)
        };
        let x3 = subm(subm(mulm(lambda, lambda, p), a.x, p), b.x, p);
        let y3 = subm(mulm(lambda, subm(a.x, x3, p), p), a.y, p);
        PrimePoint::affine(x3, y3)
    }

    pub fn mul(&self, pt: PrimePoint, mut k: u64) -> PrimePoint {
        let (mut acc, mut base) = (PrimePoint::INFINITY, pt);
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }
}

#[derive(Clone, Debug)]
pub struct PrimeInstance {
    pub name: &'static str,
    pub curve: PrimeCurve,
    pub r: u64,
    pub generator: (u64, u64),
}

impl PrimeInstance {
    pub fn generator_point(&self) -> PrimePoint {
        PrimePoint::affine(self.generator.0, self.generator.1)
    }
}

pub fn fp_p31_subexp_mersenne_instance() -> PrimeInstance {
    PrimeInstance {
        name: "icv1-fp31-tm3989-ae9db972",
        curve: PrimeCurve {
            p: 2_147_483_647,
            a: 2_147_483_644,
            b: 21,
        },
        r: 2_147_487_637,
        generator: (1_391_443_431, 177_979_044),
    }
}

pub struct FactorBase {
    pub points: Vec<PrimePoint>,
    pub neg_index: Vec<usize>,
    pub abscissae: usize,
}

fn dickson_abscissae(p: u64, t: u32, lo: u64, c_top: u64) -> Vec<u64> {
    let helper = PrimeCurve { p, a: 0, b: 1 };
    let mut layer = vec![c_top % p];
    for _ in 0..t {
        let mut next = Vec::with_capacity(layer.len() * 2);
        for &u in &layer {
            let rhs = addm(u, 2 % p, p);
            assert_ne!(rhs, 0, "dickson tower ramifies");
            let r = helper.sqrt(rhs).expect("dickson tower splits");
            let r0 = r.min(p - r);
            let r1 = p - r0;
            next.push(r0);
            if r1 != r0 {
                next.push(r1);
            }
        }
        layer = next;
    }
    let xs: Vec<u64> = layer.into_iter().map(|y0| addm(y0, lo % p, p)).collect();
    let distinct: std::collections::HashSet<_> = xs.iter().collect();
    assert_eq!(distinct.len(), 1 << t);
    xs
}

pub fn dickson_factor_base(inst: &PrimeInstance, t: u32, lo: u64, c_top: u64) -> FactorBase {
    let curve = &inst.curve;
    let mut fb = FactorBase {
        points: Vec::new(),
        neg_index: Vec::new(),
        abscissae: 0,
    };
    for x in dickson_abscissae(curve.p, t, lo, c_top) {
        let Some(y) = curve.sqrt(curve.rhs(x)) else {
            continue;
        };
        let y = if y == 0 { 0 } else { y.min(curve.p - y) };
        let pt = PrimePoint::affine(x, y);
        let qn = curve.neg(pt);
        let i = fb.points.len();
        fb.points.push(pt);
        if qn != pt {
            fb.points.push(qn);
            fb.neg_index.push(i + 1);
            fb.neg_index.push(i);
        } else {
            fb.neg_index.push(i);
        }
        fb.abscissae += 1;
    }
    fb
}

// ---------------------------------------------------------------- manifest contract
#[derive(Clone, Debug, Deserialize)]
pub struct Manifest {
    pub schema: String,
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
    pub query_block_size: usize,
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
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("read manifest {path}: {e}"));
    let manifest = serde_json::from_slice(&bytes).unwrap_or_else(|e| panic!("parse {path}: {e}"));
    (manifest, bytes)
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn decode_hex(value: &str) -> Vec<u8> {
    assert!(value.len() % 2 == 0);
    (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16).expect("hex"))
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
    stream_bytes(master, domain, stream, index)
        .into_iter()
        .fold(0u64, |acc, byte| {
            ((u128::from(acc) * 256 + u128::from(byte)) % u128::from(modulus)) as u64
        })
}

fn planted_indices(master: &[u8], domain: &str, target: usize, arity: usize, neg: &[usize]) -> Vec<usize> {
    let mut selected = Vec::with_capacity(arity);
    for term in 0..arity {
        let mut candidate =
            stream_mod(master, domain, "planted-indices", target * arity + term, neg.len() as u64) as usize;
        while selected.contains(&candidate) || selected.contains(&neg[candidate]) {
            candidate = (candidate + 1) % neg.len();
        }
        selected.push(candidate);
    }
    selected.sort_unstable();
    selected
}

pub fn load_fixture(manifest: &Manifest) -> Fixture {
    assert_eq!(manifest.schema, "isogeny-labs/experiment-seeds/v1");
    assert_eq!(manifest.configuration.arity, 5);
    let instance = fp_p31_subexp_mersenne_instance();
    assert_eq!(manifest.configuration.curve, instance.name);
    let fb = dickson_factor_base(
        &instance,
        manifest.configuration.dickson_height,
        manifest.configuration.dickson_lo,
        manifest.configuration.dickson_c_top,
    );
    let master = decode_hex(&manifest.master_seed_hex);
    assert_eq!(master.len(), 32);
    let domain = &manifest.stream_derivation.domain;
    let generator = instance.generator_point();
    let mut targets = Vec::new();
    for index in 0..manifest.configuration.random_targets {
        let scalar = stream_mod(&master, domain, "random-targets", index, instance.r);
        targets.push(instance.curve.mul(generator, scalar));
    }
    for target in 0..manifest.configuration.planted_targets {
        let witness = planted_indices(&master, domain, target, 5, &fb.neg_index);
        targets.push(
            witness
                .iter()
                .fold(instance.curve.identity(), |s, &i| instance.curve.add(s, fb.points[i])),
        );
    }
    Fixture {
        instance,
        base: fb.points,
        neg_index: fb.neg_index,
        abscissae: fb.abscissae,
        targets,
    }
}

pub fn verify_results(fixture: &Fixture, results: &[Option<Vec<usize>>]) -> bool {
    let curve = &fixture.instance.curve;
    results.len() == fixture.targets.len()
        && results.iter().zip(&fixture.targets).all(|(w, &t)| match w {
            None => true,
            Some(w) => {
                w.len() == 5
                    && w.iter().all(|&i| i < fixture.base.len())
                    && w.iter()
                        .fold(curve.identity(), |s, &i| curve.add(s, fixture.base[i]))
                        == t
            }
        })
}

pub fn result_digest(results: &[Option<Vec<usize>>]) -> String {
    let mut hasher = Sha256::new();
    for result in results {
        match result {
            None => hasher.update([0]),
            Some(w) => {
                hasher.update([1]);
                hasher.update((w.len() as u64).to_be_bytes());
                for &i in w {
                    hasher.update((i as u64).to_be_bytes());
                }
            }
        }
    }
    format!("{:x}", hasher.finalize())
}

// ---------------------------------------------------------------- retained certificates
#[derive(Debug, Deserialize)]
pub struct CertificateBundle {
    pub manifest_sha256: String,
    pub result_digest: String,
    pub factor_base_points: usize,
    pub factor_base_abscissae: usize,
    pub targets: Vec<TargetCertificate>,
}

#[derive(Debug, Deserialize)]
pub struct TargetCertificate {
    pub target: PointRecord,
    pub witness: Option<Vec<usize>>,
}

#[derive(Debug, Deserialize)]
pub struct PointRecord {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

pub fn read_certificates(path: &str) -> CertificateBundle {
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("read certificates {path}: {e}"));
    serde_json::from_slice(&bytes).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}
