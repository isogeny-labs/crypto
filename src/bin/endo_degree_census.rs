//! `endo_degree_census`: how often does an ordinary `E/F_p` have a low-degree
//! endomorphism? For each curve: trace `t`, `Δ = t² − 4p = d_K f²`, the
//! smallest `|d_K|` (tested without factoring: is `Δ/d` a perfect square for
//! each fundamental `d`?), the class-wide minimum endomorphism degree, and
//! whether `E` is on the crater when that is cheaply decidable.
//!
//! Populations: (a) the registry's prime-field rows, (b) named curves,
//! (c) a null of uniformly random `(a, b)` over random primes, (d) CM controls.
//! Protocol (frozen): `research/endo_degree_census_20261112/PROTOCOL.md`.
//!
//! ```text
//! endo_degree_census --registry docs/curves/registry.json \
//!     --out-dir research/endo_degree_census_20261112
//! ```
//!
//! Every number printed here is a `[pcc:unverified]` assertion until signed.

use std::collections::BTreeMap;
use std::io::Write;

use clap::Parser;
use num_bigint::BigInt;
use num_traits::{Num, ToPrimitive};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crypto_lib::cryptanalysis::chained_transfer::endo_census::{
    analyse, class_number, count_trace, curve_with_j, expected_by_d, fundamental_discriminants,
    kronecker, min_degree, on_crater, Analysis, CLASS_NUMBER_ONE,
};
use crypto_lib::cryptanalysis::chained_transfer::field::is_prime;

#[derive(Parser, Debug)]
#[command(about = "Census of the class-wide minimal endomorphism degree")]
struct Args {
    /// ICV1 registry (population a and b).
    #[arg(long, default_value = "docs/curves/registry.json")]
    registry: String,
    #[arg(long, default_value = "research/endo_degree_census_20261112")]
    out_dir: String,
    #[arg(long, default_value_t = 20261112)]
    seed: u64,
    /// `bits:curves:primes,...` for the null population.
    #[arg(
        long,
        default_value = "12:30000:300,16:100000:500,20:100000:500,24:100000:500,28:50000:250,32:20000:100,40:2000:40"
    )]
    sizes: String,
    /// Control curves per (D, bit size).
    #[arg(long, default_value_t = 200)]
    controls_per: usize,
    #[arg(long, default_value = "16,24,32,40")]
    control_bits: String,
}

const BUCKETS: [&str; 8] = ["1", "2", "3", "4-5", "6-8", "9-11", "12-1100", ">11"];

fn bucket(deg: Option<u64>) -> &'static str {
    match deg {
        Some(1) => BUCKETS[0],
        Some(2) => BUCKETS[1],
        Some(3) => BUCKETS[2],
        Some(4..=5) => BUCKETS[3],
        Some(6..=8) => BUCKETS[4],
        Some(9..=11) => BUCKETS[5],
        Some(_) => BUCKETS[6],
        None => BUCKETS[7],
    }
}

#[derive(Default, Clone, Serialize)]
struct Pop {
    name: String,
    /// Curves analysed (ordinary + supersingular).
    n: u64,
    ordinary: u64,
    supersingular: u64,
    /// Order counter gave up (synthetic only).
    count_failed: u64,
    /// `|d_K| ≤ 44`.
    hit44: u64,
    hit44_by_d: BTreeMap<i64, u64>,
    /// Class-wide minimum degree buckets over ordinary curves.
    buckets: BTreeMap<String, u64>,
    crater_true: u64,
    crater_false: u64,
    crater_class_bound_only: u64,
    /// Exact `Σ P(|d_K| ≤ 44)` over curves whose prime was evaluated; 0 if none.
    expected_hits: f64,
    expected_var: f64,
    expected_n: u64,
    expected_by_d: BTreeMap<i64, f64>,
    observed_in_expected_n: u64,
}

impl Pop {
    fn new(name: &str) -> Pop {
        Pop {
            name: name.to_string(),
            ..Default::default()
        }
    }
    fn add(&mut self, an: &Analysis) {
        self.n += 1;
        if an.supersingular {
            self.supersingular += 1;
            return;
        }
        self.ordinary += 1;
        *self
            .buckets
            .entry(bucket(an.class_min_degree()).to_string())
            .or_default() += 1;
        if an.within_44() {
            self.hit44 += 1;
            *self.hit44_by_d.entry(an.d_k.unwrap()).or_default() += 1;
        }
    }
    fn merge(&mut self, o: &Pop) {
        self.n += o.n;
        self.ordinary += o.ordinary;
        self.supersingular += o.supersingular;
        self.count_failed += o.count_failed;
        self.hit44 += o.hit44;
        for (k, v) in &o.hit44_by_d {
            *self.hit44_by_d.entry(*k).or_default() += v;
        }
        for (k, v) in &o.buckets {
            *self.buckets.entry(k.clone()).or_default() += v;
        }
        self.crater_true += o.crater_true;
        self.crater_false += o.crater_false;
        self.crater_class_bound_only += o.crater_class_bound_only;
        self.expected_hits += o.expected_hits;
        self.expected_var += o.expected_var;
        self.expected_n += o.expected_n;
        self.observed_in_expected_n += o.observed_in_expected_n;
        for (k, v) in &o.expected_by_d {
            *self.expected_by_d.entry(*k).or_default() += v;
        }
    }
}

#[derive(Serialize, Clone)]
struct Row {
    pop: String,
    id: String,
    bits: u64,
    p: String,
    t: String,
    supersingular: bool,
    d_k: Option<i64>,
    f: Option<String>,
    class_min_degree: Option<u64>,
    /// `Some(true/false)` exact (class number one); `None`: class bound only.
    on_crater: Option<bool>,
    /// Registry sub-populations the row belongs to (empty outside the registry).
    groups: Vec<String>,
}

fn parse_big(s: &str) -> Option<BigInt> {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x") {
        BigInt::from_str_radix(h, 16).ok()
    } else {
        BigInt::from_str_radix(s, 10).ok()
    }
}

fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn tier(bits: u64) -> &'static str {
    match bits {
        0..=16 => "bits<=16",
        17..=32 => "bits17-32",
        33..=64 => "bits33-64",
        65..=128 => "bits65-128",
        129..=256 => "bits129-256",
        _ => "bits>256",
    }
}

/// Analyse one `(p, t)` and, when `a, b` are known and `d_K` has class number one, the crater.
fn study(
    pop: &mut Pop,
    p: &BigInt,
    t: &BigInt,
    ab: Option<(&BigInt, &BigInt)>,
    discs: &[i64],
) -> (Analysis, Option<bool>) {
    let an = analyse(p, t, discs);
    pop.add(&an);
    let mut crater = None;
    if an.within_44() {
        if let (Some((a, b)), Some(d)) = (ab, an.d_k) {
            crater = on_crater(p, a, b, d);
        }
        match crater {
            Some(true) => pop.crater_true += 1,
            Some(false) => pop.crater_false += 1,
            None => pop.crater_class_bound_only += 1,
        }
    }
    (an, crater)
}

fn row_of(pop: &str, id: &str, p: &BigInt, t: &BigInt, an: &Analysis, crater: Option<bool>) -> Row {
    Row {
        pop: pop.to_string(),
        id: id.to_string(),
        bits: p.bits(),
        p: p.to_string(),
        t: t.to_string(),
        supersingular: an.supersingular,
        d_k: an.d_k,
        f: an.f.as_ref().map(|f| f.to_string()),
        class_min_degree: an.class_min_degree(),
        on_crater: crater,
        groups: Vec::new(),
    }
}

struct RegistryOut {
    pops: Vec<Pop>,
    rows: Vec<Row>,
    skipped: BTreeMap<String, u64>,
    hasse_checks: Vec<String>,
    unique_pt: u64,
}

fn run_registry(path: &str, discs: &[i64], discs44: &[i64]) -> RegistryOut {
    let text = std::fs::read_to_string(path).expect("registry");
    let v: serde_json::Value = serde_json::from_str(&text).expect("registry json");
    let curves = v["curves"].as_array().expect("curves");
    let mut skipped: BTreeMap<String, u64> = BTreeMap::new();
    struct In {
        id: String,
        p: BigInt,
        t: BigInt,
        ab: Option<(BigInt, BigInt)>,
        named: bool,
        pairing: bool,
        other_named: bool,
    }
    let mut ins: Vec<In> = Vec::new();
    for c in curves {
        let fam = c["family"].as_str().unwrap_or("?");
        if fam != "prime" {
            *skipped
                .entry(format!("family={fam} (not a prime field)"))
                .or_default() += 1;
            continue;
        }
        let ps = c["params"]["p"].as_str().unwrap_or("");
        let os = c["order"].as_str().unwrap_or("");
        let (Some(p), Some(o)) = (parse_big(ps), parse_big(os)) else {
            *skipped
                .entry("prime row without p or order".into())
                .or_default() += 1;
            continue;
        };
        if p < BigInt::from(5) {
            *skipped.entry("prime row with p < 5".into()).or_default() += 1;
            continue;
        }
        let t = &p + 1 - &o;
        if &t * &t > BigInt::from(4) * &p {
            *skipped
                .entry("prime row whose recorded order violates Hasse (not the group order)".into())
                .or_default() += 1;
            continue;
        }
        let a = c["params"]["a"].as_str().and_then(parse_big);
        let b = c["params"]["b"].as_str().and_then(parse_big);
        let ab = match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        };
        let aliases: Vec<&str> = c["aliases"]
            .as_array()
            .map(|a| a.iter().filter_map(|x| x.as_str()).collect())
            .unwrap_or_default();
        let std_names: Vec<&str> = c["standard_names"]
            .as_array()
            .map(|a| a.iter().filter_map(|x| x.as_str()).collect())
            .unwrap_or_default();
        let pairing = aliases.iter().any(|a| {
            a.starts_with("bn/")
                || a.starts_with("bls/")
                || (a.starts_with("other/") && a.contains("BN"))
        });
        let other_named = aliases.iter().any(|a| a.starts_with("other/")) && !pairing;
        let id = std_names
            .first()
            .map(|s| s.to_string())
            .or_else(|| aliases.first().map(|s| s.to_string()))
            .unwrap_or_else(|| c["slug"].as_str().unwrap_or("?").to_string());
        ins.push(In {
            id,
            p,
            t,
            ab,
            named: !std_names.is_empty(),
            pairing,
            other_named,
        });
    }
    let mut uniq = std::collections::BTreeSet::new();
    for i in &ins {
        uniq.insert((i.p.to_string(), i.t.to_string()));
    }
    // Per curve analysis in parallel; fold into populations afterwards.
    let results: Vec<(
        Vec<String>,
        Analysis,
        Option<bool>,
        Option<(f64, Vec<(i64, f64)>)>,
    )> = ins
        .par_iter()
        .map(|i| {
            let an = analyse(&i.p, &i.t, discs);
            let mut crater = None;
            if an.within_44() {
                if let (Some((a, b)), Some(d)) = (&i.ab, an.d_k) {
                    crater = on_crater(&i.p, a, b, d);
                }
            }
            let exp = i.p.to_u64().filter(|&p| p < (1 << 28)).map(|p| {
                let e = expected_by_d(p, discs44);
                (
                    e.iter().sum::<f64>(),
                    discs44.iter().cloned().zip(e).collect(),
                )
            });
            (
                names_for_static(i.p.bits(), i.named, i.pairing, i.other_named),
                an,
                crater,
                exp,
            )
        })
        .collect();
    let mut pops: BTreeMap<String, Pop> = BTreeMap::new();
    let mut rows = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (i, (mut names, an, crater, exp)) in ins.iter().zip(results) {
        if seen.insert((i.p.to_string(), i.t.to_string())) {
            names.push("a:registry_prime:distinct(p,t)".to_string());
        }
        for n in &names {
            let pop = pops.entry(n.clone()).or_insert_with(|| Pop::new(n));
            pop.add(&an);
            if an.within_44() {
                match crater {
                    Some(true) => pop.crater_true += 1,
                    Some(false) => pop.crater_false += 1,
                    None => pop.crater_class_bound_only += 1,
                }
            }
            if let Some((s, byd)) = &exp {
                if !an.supersingular {
                    pop.expected_hits += s;
                    pop.expected_var += s * (1.0 - s);
                    pop.expected_n += 1;
                    if an.within_44() {
                        pop.observed_in_expected_n += 1;
                    }
                    for (d, e) in byd {
                        *pop.expected_by_d.entry(*d).or_default() += e;
                    }
                }
            }
        }
        if an.within_44() || (i.named || i.pairing || i.other_named) {
            let mut r = row_of("registry", &i.id, &i.p, &i.t, &an, crater);
            r.groups = names
                .iter()
                .filter(|n| n.starts_with("b:"))
                .cloned()
                .collect();
            rows.push(r);
        }
    }
    // Hasse cross-check of P-256 and secp256k1 against the literals in src/ecc/curve.rs.
    let mut hasse_checks = Vec::new();
    let curve_rs = std::fs::read_to_string("src/ecc/curve.rs")
        .unwrap_or_default()
        .to_lowercase();
    for (name, want_p, want_n) in [
        (
            "P-256",
            "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
            "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551",
        ),
        (
            "secp256k1",
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        ),
    ] {
        let in_file = curve_rs.contains(want_p) && curve_rs.contains(want_n);
        let reg = ins
            .iter()
            .find(|i| i.id == name || i.id.to_lowercase() == name.to_lowercase());
        let reg_ok = reg
            .map(|i| {
                let n = BigInt::from_str_radix(want_n, 16).unwrap();
                let pp = BigInt::from_str_radix(want_p, 16).unwrap();
                i.p == pp && &i.p + 1 - &i.t == n
            })
            .unwrap_or(false);
        hasse_checks.push(format!(
            "{name}: p and n literals present in src/ecc/curve.rs = {in_file}; registry row has the same p and order (cofactor 1) = {reg_ok}"
        ));
    }
    RegistryOut {
        pops: pops.into_values().collect(),
        rows,
        skipped,
        hasse_checks,
        unique_pt: uniq.len() as u64,
    }
}

fn names_for_static(bits: u64, named: bool, pairing: bool, other_named: bool) -> Vec<String> {
    let mut v = vec![
        "a:registry_prime:all".to_string(),
        format!("a:registry_prime:{}", tier(bits)),
    ];
    if bits >= 31 {
        v.push("a:registry_prime:p>=2^30".to_string());
    }
    if named {
        v.push("b:named_standard".to_string());
    }
    if pairing {
        v.push("b:pairing_friendly".to_string());
    }
    if other_named {
        v.push("b:other_named".to_string());
    }
    v
}

fn random_prime(rng: &mut SmallRng, bits: u32, ok: &dyn Fn(u64) -> bool) -> u64 {
    loop {
        let p = rng.gen_range(1u64 << (bits - 1)..1u64 << bits) | 1;
        if is_prime(p) && p > 5 && ok(p) {
            return p;
        }
    }
}

struct SynthOut {
    pop: Pop,
    rows: Vec<Row>,
    digest: String,
}

fn run_synthetic(
    seed: u64,
    bits: u32,
    curves: usize,
    primes: usize,
    discs: &[i64],
    discs44: &[i64],
) -> SynthOut {
    let per = curves / primes;
    let outs: Vec<(Pop, Vec<Row>, [u8; 32])> = (0..primes)
        .into_par_iter()
        .map(|pi| {
            let mut rng =
                SmallRng::seed_from_u64(splitmix(seed ^ ((bits as u64) << 40) ^ pi as u64));
            let p = random_prime(&mut rng, bits, &|_| true);
            let e = expected_by_d(p, discs44);
            let pe: f64 = e.iter().sum();
            let mut pop = Pop::new(&format!("c:null:{bits}"));
            let mut rows = Vec::new();
            let mut h = Sha256::new();
            let pb = BigInt::from(p);
            let mut done = 0u64;
            while (done as usize) < per {
                let a = rng.gen_range(0..p);
                let b = rng.gen_range(0..p);
                let disc = (4 * (a as u128).pow(3) % p as u128
                    + 27 * (b as u128 * b as u128 % p as u128))
                    % p as u128;
                if disc == 0 {
                    continue;
                }
                done += 1;
                let Some(t) = count_trace(p, a, b) else {
                    pop.n += 1;
                    pop.count_failed += 1;
                    continue;
                };
                h.update(p.to_le_bytes());
                h.update(a.to_le_bytes());
                h.update(b.to_le_bytes());
                h.update(t.to_le_bytes());
                let tb = BigInt::from(t);
                let (an, crater) = study(
                    &mut pop,
                    &pb,
                    &tb,
                    Some((&BigInt::from(a), &BigInt::from(b))),
                    discs,
                );
                pop.expected_n += 1;
                pop.expected_hits += pe;
                pop.expected_var += pe * (1.0 - pe);
                if an.within_44() {
                    pop.observed_in_expected_n += 1;
                    rows.push(row_of(
                        &format!("null:{bits}"),
                        &format!("a={a},b={b}"),
                        &pb,
                        &tb,
                        &an,
                        crater,
                    ));
                }
            }
            for (d, x) in discs44.iter().zip(&e) {
                *pop.expected_by_d.entry(*d).or_default() += x * per as f64;
            }
            (pop, rows, h.finalize().into())
        })
        .collect();
    let mut pop = Pop::new(&format!("c:null:{bits}"));
    let mut rows = Vec::new();
    let mut h = Sha256::new();
    for (p, r, d) in outs {
        pop.merge(&p);
        rows.extend(r);
        h.update(d);
    }
    SynthOut {
        pop,
        rows,
        digest: format!("{:x}", h.finalize()),
    }
}

#[derive(Serialize, Clone)]
struct ControlOut {
    d: i64,
    expected_d_k: i64,
    expect_crater: bool,
    bits: u32,
    n: u64,
    detected: u64,
    crater_ok: u64,
    count_failed: u64,
}

fn run_controls(seed: u64, bits_list: &[u32], per: usize, discs: &[i64]) -> Vec<ControlOut> {
    // (control D, expected d_K, j, expect crater)
    let mut spec: Vec<(i64, i64, Option<i64>, bool)> =
        vec![(-3, -3, None, true), (-4, -4, None, true)];
    for &(d, j) in CLASS_NUMBER_ONE.iter().skip(2) {
        spec.push((d, d, Some(j), true));
    }
    for (d, dk, j) in [
        (-12, -3, 54_000),
        (-16, -4, 287_496),
        (-27, -3, -12_288_000),
        (-28, -7, 16_581_375),
    ] {
        spec.push((d, dk, Some(j), false));
    }
    let jobs: Vec<(i64, i64, Option<i64>, bool, u32)> = spec
        .iter()
        .flat_map(|&(d, dk, j, cr)| bits_list.iter().map(move |&b| (d, dk, j, cr, b)))
        .collect();
    jobs.par_iter()
        .map(|&(d, dk, j, expect_crater, bits)| {
            let mut rng = SmallRng::seed_from_u64(splitmix(
                seed ^ 0xC0DE ^ ((bits as u64) << 32) ^ (d.unsigned_abs() << 8),
            ));
            let mut out = ControlOut {
                d,
                expected_d_k: dk,
                expect_crater,
                bits,
                n: 0,
                detected: 0,
                crater_ok: 0,
                count_failed: 0,
            };
            for _ in 0..per {
                let p = random_prime(&mut rng, bits, &|p| kronecker(dk, p) == 1);
                let (a, b) = match (d, j) {
                    (-3, _) => (0, rng.gen_range(1..p)),
                    (-4, _) => (rng.gen_range(1..p), 0),
                    (_, Some(j)) => {
                        let Some((a0, b0)) = curve_with_j(p, j) else {
                            continue;
                        };
                        let u = rng.gen_range(1..p) as u128;
                        let pp = p as u128;
                        (
                            (a0 as u128 * (u * u % pp) % pp) as u64,
                            (b0 as u128 * (u * u % pp * u % pp) % pp) as u64,
                        )
                    }
                    _ => unreachable!(),
                };
                out.n += 1;
                let Some(t) = count_trace(p, a, b) else {
                    out.count_failed += 1;
                    continue;
                };
                let (pb, tb) = (BigInt::from(p), BigInt::from(t));
                let an = analyse(&pb, &tb, discs);
                if an.d_k == Some(dk) {
                    out.detected += 1;
                    if on_crater(&pb, &BigInt::from(a), &BigInt::from(b), dk) == Some(expect_crater)
                    {
                        out.crater_ok += 1;
                    }
                }
            }
            out
        })
        .collect()
}

#[derive(Serialize)]
struct PopReport {
    #[serde(flatten)]
    pop: Pop,
    frac44: Option<f64>,
    wilson95: Option<(f64, f64)>,
    expected_frac44: Option<f64>,
    z: Option<f64>,
    tested: bool,
    c_over_sqrt_p_note: Option<f64>,
}

fn wilson(k: u64, n: u64) -> (f64, f64) {
    let (k, n) = (k as f64, n as f64);
    let z = 1.959964;
    let ph = k / n;
    let den = 1.0 + z * z / n;
    let centre = (ph + z * z / (2.0 * n)) / den;
    let half = z * (ph * (1.0 - ph) / n + z * z / (4.0 * n * n)).sqrt() / den;
    ((centre - half).max(0.0), (centre + half).min(1.0))
}

fn report(pop: Pop) -> PopReport {
    let frac = if pop.ordinary > 0 {
        Some(pop.hit44 as f64 / pop.ordinary as f64)
    } else {
        None
    };
    let w = if pop.ordinary > 0 {
        Some(wilson(pop.hit44, pop.ordinary))
    } else {
        None
    };
    let (ef, z, tested) = if pop.expected_n > 0 {
        let ef = pop.expected_hits / pop.expected_n as f64;
        let z = if pop.expected_var > 0.0 {
            Some((pop.observed_in_expected_n as f64 - pop.expected_hits) / pop.expected_var.sqrt())
        } else {
            None
        };
        (Some(ef), z, pop.expected_hits >= 5.0)
    } else {
        (None, None, false)
    };
    PopReport {
        pop,
        frac44: frac,
        wilson95: w,
        expected_frac44: ef,
        z,
        tested,
        c_over_sqrt_p_note: None,
    }
}

fn main() {
    let args = Args::parse();
    let discs = fundamental_discriminants(4400);
    let discs44: Vec<i64> = discs.iter().cloned().filter(|d| d.abs() <= 44).collect();
    assert_eq!(discs44.len(), 15);
    let out_dir = std::path::PathBuf::from(&args.out_dir);
    std::fs::create_dir_all(&out_dir).unwrap();
    let mut rows: Vec<Row> = Vec::new();
    let t0 = std::time::Instant::now();
    println!(
        "endo_degree_census seed={} discriminants<=4400: {} (<=44: {})",
        args.seed,
        discs.len(),
        discs44.len()
    );
    println!(
        "min degree table (|d_K| <= 44): {}",
        discs44
            .iter()
            .map(|d| format!("{d}:h={},deg>={}", class_number(*d), min_degree(*d)))
            .collect::<Vec<_>>()
            .join(" ")
    );

    // (a)+(b)
    let reg = run_registry(&args.registry, &discs, &discs44);
    println!(
        "registry: skipped {:?}; distinct (p,t) among analysed rows: {}",
        reg.skipped, reg.unique_pt
    );
    for c in &reg.hasse_checks {
        println!("cross-check {c}");
    }
    let mut reports: Vec<PopReport> = Vec::new();
    for p in reg.pops {
        reports.push(report(p));
    }
    rows.extend(reg.rows);

    // (c)
    let mut digests = BTreeMap::new();
    for spec in args.sizes.split(',') {
        let f: Vec<usize> = spec.split(':').map(|x| x.parse().unwrap()).collect();
        let (bits, curves, primes) = (f[0] as u32, f[1], f[2]);
        let s = run_synthetic(args.seed, bits, curves, primes, &discs, &discs44);
        println!(
            "null {bits:>2} bits: curves={} ordinary={} count_failed={} hits44={} expected={:.3} elapsed={:.1}s",
            s.pop.n,
            s.pop.ordinary,
            s.pop.count_failed,
            s.pop.hit44,
            s.pop.expected_hits,
            t0.elapsed().as_secs_f64()
        );
        digests.insert(bits, s.digest.clone());
        rows.extend(s.rows);
        reports.push(report(s.pop));
    }

    // (d)
    let bits_list: Vec<u32> = args
        .control_bits
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let controls = run_controls(args.seed, &bits_list, args.controls_per, &discs);
    let mut control_fail = 0u64;
    for c in &controls {
        let ok = c.detected + c.count_failed == c.n && c.crater_ok == c.detected;
        if !ok {
            control_fail += 1;
        }
        println!(
            "control D={:>4} (d_K {:>3}, crater {:5}) {:>2} bits: n={} detected={} crater_ok={} count_failed={} {}",
            c.d, c.expected_d_k, c.expect_crater, c.bits, c.n, c.detected, c.crater_ok, c.count_failed,
            if ok { "ok" } else { "FAIL" }
        );
    }

    // Decision-rule inputs (PROTOCOL.md, "Decision rule").
    let max_z_tested = reports
        .iter()
        .filter(|r| r.pop.name.starts_with("c:null") && r.tested)
        .filter_map(|r| r.z)
        .fold(0.0f64, |m, z| m.max(z.abs()));
    let natural: Vec<&Row> = rows
        .iter()
        .filter(|r| {
            r.pop == "registry"
                && r.bits >= 31
                && matches!(r.d_k, Some(d) if d.abs() <= 44 && d != -3 && d != -4)
        })
        .filter(|r| r.on_crater != Some(false))
        .collect();
    let named_candidates: Vec<&Row> = natural
        .iter()
        .cloned()
        .filter(|r| !r.groups.is_empty())
        .collect();
    let invalid = control_fail > 0 || max_z_tested > 3.0;
    let verdict = if invalid {
        "INVALID"
    } else if !natural.is_empty() {
        "WORTH_RUNNING_ON_NATURAL_CURVES"
    } else {
        "WORTH_RUNNING_ON_CONSTRUCTED_CURVES_ONLY"
    };
    println!("control failures: {control_fail}; max |z| over tested null sizes: {max_z_tested:.3}");
    println!("natural candidates (registry, p >= 2^30, |d_K| <= 44, d_K not in {{-3,-4}}, not known non-crater): {}", natural.len());
    for r in &natural {
        println!(
            "  {} bits={} d_K={:?} f={:?} deg={:?} crater={:?}",
            r.id, r.bits, r.d_k, r.f, r.class_min_degree, r.on_crater
        );
    }
    println!(
        "  of which in population (b) (named curves): {}",
        named_candidates.len()
    );
    for r in &named_candidates {
        println!(
            "  (b) {} {:?} bits={} d_K={:?} deg={:?} crater={:?}",
            r.id, r.groups, r.bits, r.d_k, r.class_min_degree, r.on_crater
        );
    }
    println!("verdict against the frozen rule: {verdict}");

    // Output files.
    let json = serde_json::json!({
        "schema": "endo_degree_census/v1",
        "protocol": "research/endo_degree_census_20261112/PROTOCOL.md",
        "seed": args.seed,
        "sizes": args.sizes,
        "controls_per": args.controls_per,
        "control_bits": args.control_bits,
        "discriminants_44": discs44,
        "class_min_degree_44": discs44.iter().map(|d| (d.to_string(), min_degree(*d))).collect::<BTreeMap<_,_>>(),
        "registry_skipped": reg.skipped,
        "registry_distinct_pt": reg.unique_pt,
        "cross_checks": reg.hasse_checks,
        "populations": reports,
        "controls": controls,
        "null_transcript_sha256": digests,
        "control_failures": control_fail,
        "max_abs_z_tested_null": max_z_tested,
        "natural_candidates": natural,
        "natural_candidates_in_b": named_candidates,
        "verdict": verdict,
        "elapsed_s": t0.elapsed().as_secs_f64(),
    });
    std::fs::write(
        out_dir.join("census.json"),
        serde_json::to_string_pretty(&json).unwrap(),
    )
    .unwrap();
    let mut f = std::fs::File::create(out_dir.join("rows.jsonl")).unwrap();
    for r in &rows {
        writeln!(f, "{}", serde_json::to_string(r).unwrap()).unwrap();
    }
    println!(
        "wrote {}/census.json and rows.jsonl ({} rows) in {:.1}s",
        args.out_dir,
        rows.len(),
        t0.elapsed().as_secs_f64()
    );
}
