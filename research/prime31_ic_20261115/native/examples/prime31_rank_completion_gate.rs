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

    /// Run58 exact count of immediately solvable signed-pair target templates.
    /// Only the current affine relation graph is used; frozen homogeneous rows
    /// and target-to-target cancellations can yield additional solutions.
    fn single_target_count(&mut self) -> (u128, u128, usize) {
        let n = self.parent.len();
        let mut sizes = vec![0usize; n];
        let mut known = 0usize;
        for i in 0..n {
            let (r, _) = self.root(i);
            if self.anchored[r] { known += 1; }
            else { sizes[r] += 1; }
        }
        let a = known as u128;
        let good = 2 * a * a + sizes.iter().map(|&s| {
            let x = s as u128;
            x * x.saturating_sub(1)
        }).sum::<u128>();
        (good, 2 * (n as u128) * (n as u128), known)
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


// Run59: exact two-term target visibility after ALSO projecting the frozen
// homogeneous free-relation basis. Reuses run57/58 SignedDsu and field operations;
// no new EC arithmetic, solver or known-scalar extraction code.
impl SignedDsu {
    fn single_target_free_count(
        &mut self,
        free_rows: &[Vec<i64>],
    ) -> (u128, u128, usize, usize) {
        let n = self.parent.len();
        let roots: Vec<usize> = (0..n).map(|i| self.root(i).0).collect();
        let mut pivots: BTreeMap<usize, BTreeMap<usize, u64>> = BTreeMap::new();
        for encoded in free_rows {
            assert_eq!(encoded.len() % 2, 0);
            let mut row: BTreeMap<usize, u64> = BTreeMap::new();
            for pair in encoded.chunks_exact(2) {
                let (root, sign) = self.root(pair[0] as usize);
                if self.anchored[root] { continue; }
                let old = row.get(&root).copied().unwrap_or(0) as i64;
                let new = (old + i64::from(sign) * pair[1]).rem_euclid(N as i64) as u64;
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
                    let inverse = power(factor, N - 2);
                    for value in row.values_mut() { *value = mul(*value, inverse); }
                    pivots.insert(lead, row);
                    break;
                }
            }
        }
        // Normal form of each component-root coordinate in the quotient by the
        // free-relation row span. Two signed pair terms cancel iff their reduced
        // normal forms are equal up to sign, OR both normal forms are zero.
        let mut signatures: HashMap<usize, Vec<(usize, u64)>> = HashMap::new();
        for &root in &roots {
            if signatures.contains_key(&root) { continue; }
            if self.anchored[root] {
                signatures.insert(root, Vec::new());
                continue;
            }
            let mut form: BTreeMap<usize, u64> = BTreeMap::from([(root, 1)]);
            for (&lead, pivot) in &pivots {
                let factor = form.get(&lead).copied().unwrap_or(0);
                if factor == 0 { continue; }
                for (&col, &value) in pivot {
                    let old = form.get(&col).copied().unwrap_or(0);
                    let new = (old + N - mul(factor, value)) % N;
                    if new == 0 { form.remove(&col); }
                    else { form.insert(col, new); }
                }
            }
            let positive: Vec<(usize, u64)> = form.into_iter().collect();
            let negative: Vec<(usize, u64)> =
                positive.iter().map(|&(col, value)| (col, N - value)).collect();
            let key = std::cmp::min(positive, negative);
            signatures.insert(root, key);
        }
        let mut anchored = 0usize;
        let mut classes: HashMap<Vec<(usize, u64)>, usize> = HashMap::new();
        for root in roots {
            let key = signatures.get(&root).expect("missing reduced root");
            if key.is_empty() { anchored += 1; }
            else { *classes.entry(key.clone()).or_insert(0) += 1; }
        }
        let a = anchored as u128;
        let favorable = 2 * a * a + classes.values().map(|&s| {
            let v = s as u128; v * (v - 1)
        }).sum::<u128>();
        (favorable, 2 * (n as u128) * (n as u128), anchored, pivots.len())
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

/// Run58: synthetic successful-pair stream; full failed-probe scaling by exact
/// frozen Prime31 two-point support. This is not a root-extraction timing.
fn run58_one_target_gate(name: &str, n: usize, support: u64) {
    const GRID: [usize; 11] = [0,100,200,300,400,500,600,700,900,1200,1800];
    const TARGETS: [usize; 4] = [1,32,128,1024];
    const REPS: usize = 128;
    let mut reciprocal = [0f64; GRID.len()];
    let mut probability = [0f64; GRID.len()];
    let mut zero = [0usize; GRID.len()];
    for trial in 0..REPS {
        let mut d = SignedDsu::new(n);
        let mut rng = SplitMix64(0x58_2026 + 7919 * trial as u64
            + if name == "hybrid" { 104729 } else { 0 });
        let mut previous = 0;
        for (ix, &m) in GRID.iter().enumerate() {
            for _ in previous..m {
                let (i,a,j,b) = draw(&mut rng, n);
                d.add(i,a,j,b);
            }
            let (good,total,_) = d.single_target_count();
            let p = good as f64 / total as f64;
            probability[ix] += p;
            if good == 0 { zero[ix] += 1; }
            else { reciprocal[ix] += 1.0 / p; }
            previous = m;
        }
    }
    for &target_count in &TARGETS {
        let mut best: Option<(usize, f64, f64)> = None;
        for (ix, &m) in GRID.iter().enumerate() {
            if zero[ix] > 0 { continue; }
            let mean_wait = reciprocal[ix] / REPS as f64;
            let mean_p = probability[ix] / REPS as f64;
            let queries = (m as f64 + target_count as f64 * mean_wait)
                * N as f64 / support as f64;
            if best.is_none_or(|(_, old, _)| queries < old) {
                best = Some((m, queries, mean_p));
            }
        }
        let (m, expected_queries, p) = best.expect("no supported stopping budget");
        println!(
            "run58 {name}: targets={target_count}, base_positive_hits={m},              CONDITIONAL model_uniform_calls={expected_queries:.0},              p_one_target_hit={p:.5}"
        );
    }
}


/// Run59 conditional arithmetic only: same existing original-Rust DSU and
/// synthetic known-scalar positive pair generator, with exact frozen free rows.
fn run59_free_row_gate(name: &str, n: usize, support: u64, free_rows: &[Vec<i64>]) {
    const GRID: [usize; 12] = [0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1200];
    const TARGETS: [usize; 4] = [1, 32, 128, 1024];
    const REPS: usize = 128;
    let mut graph_recip = [0.0f64; GRID.len()];
    let mut free_recip = [0.0f64; GRID.len()];
    let mut p_graph = [0.0f64; GRID.len()];
    let mut p_free = [0.0f64; GRID.len()];
    let mut zeros_graph = [0usize; GRID.len()];
    let mut zeros_free = [0usize; GRID.len()];
    for trial in 0..REPS {
        let mut d = SignedDsu::new(n);
        let mut rng = SplitMix64(0x59_2026 + 7919 * trial as u64
            + if name == "hybrid" { 104729 } else { 0 });
        let mut previous = 0;
        for (ix, &m) in GRID.iter().enumerate() {
            for _ in previous..m {
                let (i, a, j, b) = draw(&mut rng, n);
                d.add(i, a, j, b);
            }
            let (graph_good, total, _) = d.single_target_count();
            let (free_good, free_total, _, _) = d.single_target_free_count(free_rows);
            assert_eq!(total, free_total);
            assert!(free_good >= graph_good, "free rows decreased target visibility");
            let pg = graph_good as f64 / total as f64;
            let pf = free_good as f64 / total as f64;
            p_graph[ix] += pg;
            p_free[ix] += pf;
            if graph_good == 0 { zeros_graph[ix] += 1; }
            else { graph_recip[ix] += 1.0 / pg; }
            if free_good == 0 { zeros_free[ix] += 1; }
            else { free_recip[ix] += 1.0 / pf; }
            previous = m;
        }
    }
    for &targets in &TARGETS {
        let mut best_graph: Option<(usize, f64)> = None;
        let mut best_free: Option<(usize, f64)> = None;
        for (ix, &m) in GRID.iter().enumerate() {
            if zeros_graph[ix] == 0 {
                let calls = (m as f64 + targets as f64 * graph_recip[ix] / REPS as f64)
                    * N as f64 / support as f64;
                if best_graph.is_none_or(|(_, old)| calls < old) { best_graph = Some((m, calls)); }
            }
            if zeros_free[ix] == 0 {
                let calls = (m as f64 + targets as f64 * free_recip[ix] / REPS as f64)
                    * N as f64 / support as f64;
                if best_free.is_none_or(|(_, old)| calls < old) { best_free = Some((m, calls)); }
            }
        }
        let (m_old, old) = best_graph.expect("no graph-only visible targets");
        let (m_new, new) = best_free.expect("no free-row visible targets");
        assert!(new <= old + 0.000_001);
        println!("run59 {name}: T={targets}, graph_m={m_old}, graph_calls={old:.0}, \
                free_m={m_new}, free_calls={new:.0}, model_reduction={:.2}%",
                100.0 * (1.0 - new / old));
    }
    if name == "hybrid" {
        let ix = GRID.iter().position(|&v| v == 400).unwrap();
        println!("run59 hybrid m400: mean_graph_p={:.6}, mean_free_p={:.6}, \
                 conditional_coverage_ratio={:.3}",
                 p_graph[ix] / REPS as f64, p_free[ix] / REPS as f64,
                 p_free[ix] / p_graph[ix]);
    }
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
    // Run58 adds conditional per-target early stopping WITHOUT requiring full base rank.
    run58_one_target_gate("dickson", 509, 518_084);
    run58_one_target_gate("hybrid", 677, 916_468);
    run59_free_row_gate("dickson", 509, 518_084,
        f.rows.get("dickson").expect("frozen Dickson rows"));
    run59_free_row_gate("hybrid", 677, 916_468,
        f.rows.get("hybrid").expect("frozen hybrid rows"));
    Ok(())
}
