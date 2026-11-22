//! Prime31 run57: conditional rank-completion gate for public two-summand factor bases.
//!
//! Uses the ORIGINAL Rust prime31_algebraic_ic curve definition and check_curve.
//! The input is a span-preserving frozen basis of exact public homogeneous relations
//! independently verified in run56; the sampler is a synthetic idealized
//! successful-pair oracle, NOT a blind Semaev/F4 extractor or ECDLP solver.
//! Run from the slim crate: cargo run --release --example prime31_rank_completion_gate
use prime31_algebraic_ic::{check_curve, Arithmetic, PRIME31};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

const N: u64 = 2_147_446_831;
const TRIALS: usize = 128;
const MAX_HITS: usize = 8192;

#[derive(Deserialize)]
struct Fixture {
    base_points: [usize; 2],
    distinct_sums_including_identity: [u64; 2],
    rows: HashMap<String, Vec<Vec<i64>>>,
}

fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % N as u128) as u64
}
fn power(mut a: u64, mut e: u64) -> u64 {
    let mut out = 1;
    while e > 0 {
        if e & 1 != 0 { out = mul(out, a); }
        a = mul(a, a);
        e >>= 1;
    }
    out
}

struct SignedDsu {
    parent: Vec<usize>,
    sign: Vec<i8>,
    anchored: Vec<bool>,
    free: usize,
}
impl SignedDsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            sign: vec![1; n],
            anchored: vec![false; n],
            free: n,
        }
    }
    fn root(&mut self, i: usize) -> (usize, i8) {
        if self.parent[i] == i { return (i, 1); }
        let (root, parent_sign) = self.root(self.parent[i]);
        self.sign[i] *= parent_sign;
        self.parent[i] = root;
        (root, self.sign[i])
    }
    fn add(&mut self, i: usize, a: i8, j: usize, b: i8) {
        let (u, x) = self.root(i);
        let (v, y) = self.root(j);
        if u == v {
            if i16::from(a) * i16::from(x) + i16::from(b) * i16::from(y) != 0
                && !self.anchored[u]
            {
                self.anchored[u] = true;
                self.free -= 1;
            }
            return;
        }
        let before = usize::from(!self.anchored[u]) + usize::from(!self.anchored[v]);
        let any = self.anchored[u] || self.anchored[v];
        self.parent[u] = v;
        self.sign[u] = -a * x * b * y;
        self.anchored[v] = any;
        let after = usize::from(!any);
        self.free = self.free - before + after;
    }

    // Exact deficiency except the positive-only shortcut used for stop tests.
    fn deficiency(&mut self, rows: &[Vec<i64>], exact: bool) -> usize {
        if !exact && self.free > rows.len() {
            return self.free - rows.len();
        }
        let mut pivots: BTreeMap<usize, BTreeMap<usize, u64>> = BTreeMap::new();
        for encoded in rows {
            assert_eq!(encoded.len() % 2, 0);
            let mut row: BTreeMap<usize, u64> = BTreeMap::new();
            for chunk in encoded.chunks_exact(2) {
                let (root, sign) = self.root(chunk[0] as usize);
                if self.anchored[root] { continue; }
                let old = row.get(&root).copied().unwrap_or(0) as i64;
                let new = (old + i64::from(sign) * chunk[1]).rem_euclid(N as i64) as u64;
                if new == 0 { row.remove(&root); }
                else { row.insert(root, new); }
            }
            while let Some((&lead, &factor)) = row.first_key_value() {
                if let Some(pivot) = pivots.get(&lead) {
                    for (&col, &value) in pivot {
                        let old = row.get(&col).copied().unwrap_or(0);
                        let new = (old + N - mul(factor, value)) % N;
                        if new == 0 { row.remove(&col); }
                        else { row.insert(col, new); }
                    }
                } else {
                    let reciprocal = power(factor, N - 2);
                    for value in row.values_mut() { *value = mul(*value, reciprocal); }
                    pivots.insert(lead, row);
                    break;
                }
            }
        }
        self.free - pivots.len()
    }
}

struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}
type Pair = (usize, i8, usize, i8);
fn draw(r: &mut SplitMix64, n: usize) -> Pair {
    let i = (r.next() % n as u64) as usize;
    let j = (r.next() % n as u64) as usize;
    let a = if r.next() & 1 != 0 { 1 } else { -1 };
    let mut b = if r.next() & 1 != 0 { 1 } else { -1 };
    // The inverse signed-pair gives group identity and is not a useful query.
    if i == j && a == -b { b = a; }
    (i, a, j, b)
}
fn deficiency(n: usize, rows: &[Vec<i64>], pairs: &[Pair], k: usize, exact: bool) -> usize {
    let mut d = SignedDsu::new(n);
    for &(i, a, j, b) in &pairs[..k] { d.add(i, a, j, b); }
    d.deficiency(rows, exact)
}
fn trial(n: usize, rows: &[Vec<i64>], seed: u64) -> (usize, usize) {
    let mut rng = SplitMix64(seed);
    let pairs: Vec<Pair> = (0..MAX_HITS).map(|_| draw(&mut rng, n)).collect();
    assert_eq!(deficiency(n, rows, &pairs, MAX_HITS, false), 0);
    let mut lo = 0;
    let mut hi = MAX_HITS;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if deficiency(n, rows, &pairs, mid, false) == 0 { hi = mid; }
        else { lo = mid; }
    }
    assert!(deficiency(n, rows, &pairs, hi - 1, true) > 0);
    assert_eq!(deficiency(n, rows, &pairs, hi, true), 0);

    let mut naive_rng = SplitMix64(seed ^ 0x2468_ace0_57);
    let naive = n - rows.len();
    let naive_pairs: Vec<Pair> = (0..naive).map(|_| draw(&mut naive_rng, n)).collect();
    let missing = deficiency(n, rows, &naive_pairs, naive, true);
    (hi, missing)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arithmetic = Arithmetic::default();
    check_curve(PRIME31, &mut arithmetic)?;
    assert_eq!(PRIME31.n, N);
    let f: Fixture = serde_json::from_str(include_str!(
        "../../../prime31_f4_admission_20261121/run57_basis.json"
    ))?;
    let mut means = Vec::new();
    for (index, name) in ["dickson", "hybrid"].iter().enumerate() {
        let n = f.base_points[index];
        let rows = f.rows.get(*name).expect("missing frozen homogeneous row basis");
        assert_eq!((n, rows.len()), if index == 0 { (509, 13) } else { (677, 88) });
        assert_eq!(deficiency(n, rows, &[], 0, true), n - rows.len());
        let samples: Vec<(usize, usize)> = (0..TRIALS)
            .map(|k| trial(n, rows, 0x57_2026 + 7919 * k as u64))
            .collect();
        let mean_hits = samples.iter().map(|x| x.0 as f64).sum::<f64>() / TRIALS as f64;
        let mean_missing = samples.iter().map(|x| x.1 as f64).sum::<f64>() / TRIALS as f64;
        let mean_calls = mean_hits * N as f64
            / (f.distinct_sums_including_identity[index] - 1) as f64;
        println!(
            "{name}: {TRIALS} idealized positive-pair trials; mean hits={mean_hits:.3}, \
             mean residual after naive bound={mean_missing:.3}, \
             conditional uniform calls={mean_calls:.0}"
        );
        assert!(mean_missing > 20.0, "naive full-rank stopping rule unexpectedly passed");
        means.push(mean_calls);
    }
    println!(
        "hybrid/dickson conditional query ratio={:.6}; equal-call-cost threshold={:.6}",
        means[1] / means[0], means[0] / means[1]
    );
    Ok(())
}
