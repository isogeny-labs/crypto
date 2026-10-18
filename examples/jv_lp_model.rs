//! **Large-prime cost model for the cover attack (`jv_cover`), evaluated and checked.**
//!
//! Study: `research/jv_cover_large_prime_20261018/` (lead L6 of #2216).
//! Native Rust only; no result is computed anywhere else.
//!
//! ```text
//! jv_lp_model yields --out yields.json LOG...        # measured vs modelled yields on logged decompositions
//! jv_lp_model curves --out curves.json               # Monte-Carlo X(f)/N for plain / 1-LP / 2-LP / generic
//! jv_lp_model cost --curves curves.json --dlp REC.json... --out cost.json
//! ```
//!
//! Model (derived in `COSTMODEL.md`): a decomposition test solves the six
//! quadrics over all of `F_p`, so it returns a decomposition over the *full*
//! base `F` at a cost `C_cov` that does not depend on the sub-base `F'`.  A
//! solution is a uniform 6-subset of `F`.  With `|F'| = f|F|` the solutions
//! needed grow as the relations over `F'` are fewer; the model counts how many.

use std::collections::HashMap;
use std::env;
use std::fs;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const M: usize = 6;

// ── log parsing ───────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct Dec {
    p: u64,
    seed: u64,
    base: usize,
    idx: [usize; M],
}

fn num_after(s: &str, key: &str) -> u64 {
    let k = format!("\"{key}\":");
    let i = s.find(&k).expect(key) + k.len();
    s[i..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn list_after(s: &str, key: &str) -> Vec<u64> {
    let k = format!("\"{key}\":[");
    let i = s.find(&k).expect(key) + k.len();
    let j = s[i..].find(']').unwrap() + i;
    s[i..j].split(',').map(|v| v.trim().parse().unwrap()).collect()
}

fn read_logs(paths: &[String]) -> Vec<Dec> {
    let mut out = Vec::new();
    for path in paths {
        for line in fs::read_to_string(path).expect("log").lines() {
            let Some(i) = line.find("LPLOG ") else { continue };
            let s = &line[i + 6..];
            let v = list_after(s, "idx");
            assert_eq!(v.len(), M);
            let mut idx = [0usize; M];
            for (k, &x) in v.iter().enumerate() {
                idx[k] = x as usize;
            }
            out.push(Dec {
                p: num_after(s, "p"),
                seed: num_after(s, "seed"),
                base: num_after(s, "base") as usize,
                idx,
            });
        }
    }
    out
}

// ── combinatorics ─────────────────────────────────────────────────────────

fn ln_choose(n: usize, k: usize) -> f64 {
    if k > n {
        return f64::NEG_INFINITY;
    }
    (0..k).map(|i| ((n - i) as f64).ln() - ((i + 1) as f64).ln()).sum()
}

/// P(exactly `k` of the six distinct points lie outside F'), `|F| = n`, `|F'| = nf`.
fn hyper(n: usize, nf: usize, k: usize) -> f64 {
    let out = n - nf;
    if k > out || M - k > nf {
        return 0.0;
    }
    (ln_choose(out, k) + ln_choose(nf, M - k) - ln_choose(n, M)).exp()
}

fn binom(f: f64, k: usize) -> f64 {
    (ln_choose(M, k)).exp() * (1.0 - f).powi(k as i32) * f.powi((M - k) as i32)
}

fn wilson(k: f64, n: f64) -> (f64, f64) {
    let z = 1.959964;
    let ph = k / n;
    let d = 1.0 + z * z / n;
    let c = ph + z * z / (2.0 * n);
    let h = z * (ph * (1.0 - ph) / n + z * z / (4.0 * n * n)).sqrt();
    ((c - h) / d, (c + h) / d)
}

// ── union-find, matching ──────────────────────────────────────────────────

struct Uf(Vec<usize>);
impl Uf {
    fn new(n: usize) -> Uf {
        Uf((0..n).collect())
    }
    fn find(&mut self, mut a: usize) -> usize {
        while self.0[a] != a {
            self.0[a] = self.0[self.0[a]];
            a = self.0[a];
        }
        a
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        self.0[a] = b;
        true
    }
}

/// Incremental bipartite matching rows -> columns (generic rank of a sparse matrix).
struct Matching {
    adj: Vec<Vec<usize>>,
    col_row: Vec<usize>,
    stamp: Vec<u32>,
    cur: u32,
    size: usize,
}
const NONE: usize = usize::MAX;
impl Matching {
    fn new(cols: usize) -> Matching {
        Matching { adj: vec![], col_row: vec![NONE; cols], stamp: vec![0; cols], cur: 0, size: 0 }
    }
    fn dfs(&mut self, r: usize) -> bool {
        for k in 0..self.adj[r].len() {
            let c = self.adj[r][k];
            if self.stamp[c] == self.cur {
                continue;
            }
            self.stamp[c] = self.cur;
            if self.col_row[c] == NONE || self.dfs(self.col_row[c]) {
                self.col_row[c] = r;
                return true;
            }
        }
        false
    }
    fn add_row(&mut self, cols: Vec<usize>) {
        self.adj.push(cols);
        self.cur += 1;
        let r = self.adj.len() - 1;
        if self.dfs(r) {
            self.size += 1;
        }
    }
}

// ── the relation-count schemes ────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
enum Scheme {
    /// every column solved (the method as run, with a core filter)
    Plain,
    /// single large prime: a partial relation has exactly one point outside F'
    Single,
    /// double large prime: up to two points outside F' (graph cycles)
    Double,
    /// information bound: any elimination of the points outside F' (generic coefficients)
    Generic,
}

/// Incremental state for one Monte-Carlo trial, F' = indices `< nf`.
struct State {
    n: usize,
    nf: usize,
    scheme: Scheme,
    touched: Vec<bool>,
    touched_f: usize,
    touched_all: usize,
    rel: usize,
    deg1: Vec<u32>,
    uf: Uf,
    mat: Matching,
    x: usize,
}

impl State {
    fn new(n: usize, nf: usize, scheme: Scheme) -> State {
        State {
            n,
            nf,
            scheme,
            touched: vec![false; n],
            touched_f: 0,
            touched_all: 0,
            rel: 0,
            deg1: vec![0; n],
            uf: Uf::new(n + 1),
            // column n is the target's unknown d (every relation has it)
            mat: Matching::new(n + 1),
            x: 0,
        }
    }

    fn add(&mut self, pts: &[usize; M]) {
        self.x += 1;
        for &i in pts {
            if !self.touched[i] {
                self.touched[i] = true;
                self.touched_all += 1;
                if i < self.nf {
                    self.touched_f += 1;
                }
            }
        }
        let lps: Vec<usize> = pts.iter().copied().filter(|&i| i >= self.nf).collect();
        match self.scheme {
            Scheme::Plain => {
                let mut cols = pts.to_vec();
                cols.push(self.n);
                self.mat.add_row(cols);
            }
            Scheme::Single => match lps.len() {
                0 => self.rel += 1,
                1 => {
                    if self.deg1[lps[0]] >= 1 {
                        self.rel += 1;
                    }
                    self.deg1[lps[0]] += 1;
                }
                _ => {}
            },
            Scheme::Double => match lps.len() {
                0 => self.rel += 1,
                1 => {
                    if !self.uf.union(lps[0], self.n) {
                        self.rel += 1;
                    }
                }
                2 => {
                    if !self.uf.union(lps[0], lps[1]) {
                        self.rel += 1;
                    }
                }
                _ => {}
            },
            Scheme::Generic => {
                self.mat.add_row(lps);
            }
        }
    }

    /// Enough independent relations over F' (plus the target unknown)?
    fn solved(&self) -> bool {
        match self.scheme {
            Scheme::Plain => self.mat.size == self.touched_all + 1,
            Scheme::Generic => self.x - self.mat.size > self.touched_f,
            _ => self.rel > self.touched_f,
        }
    }
}

fn six(rng: &mut StdRng, n: usize) -> [usize; M] {
    let mut a = [0usize; M];
    let mut k = 0;
    while k < M {
        let v = rng.gen_range(0..n);
        if !a[..k].contains(&v) {
            a[k] = v;
            k += 1;
        }
    }
    a
}

/// Solutions needed: the first X at which the scheme can solve (cap `cap_x`; `None` if not reached).
fn trial(n: usize, f: f64, scheme: Scheme, rng: &mut StdRng, cap_x: usize) -> Option<usize> {
    let nf = ((f * n as f64).round() as usize).clamp(1, n);
    let mut st = State::new(n, nf, scheme);
    while st.x < cap_x {
        let pts = six(rng, n);
        st.add(&pts);
        if st.x > nf && st.solved() {
            return Some(st.x);
        }
    }
    None
}

/// Relations over F' that `scheme` extracts from a fixed list of decompositions.
fn relations_from(decs: &[[usize; M]], n: usize, nf: usize, scheme: Scheme) -> (usize, usize) {
    let mut st = State::new(n, nf, scheme);
    for d in decs {
        st.add(d);
    }
    let r = match scheme {
        Scheme::Generic => st.x - st.mat.size,
        Scheme::Plain => st.x,
        _ => st.rel,
    };
    (r, st.touched_f)
}

// ── subcommand: yields ────────────────────────────────────────────────────

fn cmd_yields(args: &[String]) {
    let mut out = String::from("yields.json");
    let mut draws = 200usize;
    let mut logs = vec![];
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out = args[i].clone();
            }
            "--draws" => {
                i += 1;
                draws = args[i].parse().unwrap();
            }
            a => logs.push(a.to_string()),
        }
        i += 1;
    }
    let decs = read_logs(&logs);
    let mut by_p: HashMap<u64, Vec<&Dec>> = HashMap::new();
    for d in &decs {
        by_p.entry(d.p).or_default().push(d);
    }
    let fs_grid = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let mut rng = StdRng::seed_from_u64(0x6C70);
    let mut report = vec![];
    let mut ps: Vec<_> = by_p.keys().copied().collect();
    ps.sort();
    for p in ps {
        let group = &by_p[&p];
        // per seed: base size differs
        let mut seeds: HashMap<u64, Vec<&Dec>> = HashMap::new();
        for d in group {
            seeds.entry(d.seed).or_default().push(d);
        }
        // uniformity of the six abscissae over the base, per seed
        let mut unif = vec![];
        for (&seed, ds) in &seeds {
            let n = ds[0].base;
            let mut cnt = vec![0u64; n];
            let mut distinct = 0usize;
            for d in ds {
                let mut s = d.idx.to_vec();
                s.sort();
                s.dedup();
                if s.len() == M {
                    distinct += 1;
                }
                for &i in &d.idx {
                    cnt[i] += 1;
                }
            }
            let e = (ds.len() * M) as f64 / n as f64;
            let chi: f64 = cnt.iter().map(|&c| (c as f64 - e).powi(2) / e).sum();
            let dof = (n - 1) as f64;
            unif.push(json!({"seed": seed, "decompositions": ds.len(), "base": n, "all_six_distinct": distinct,
                "expected_per_index": e, "chi2": chi, "dof": dof, "z": (chi - dof) / (2.0 * dof).sqrt()}));
        }
        // yields at each f (pooled over seeds; each seed uses its own base size)
        let mut rows = vec![];
        for &f in &fs_grid {
            let mut obs_prefix = [0f64; 4];
            let mut obs_rand = [0f64; 4];
            let mut model_h = [0f64; 4];
            let mut model_b = [0f64; 4];
            let mut n_tot = 0f64;
            for ds in seeds.values() {
                let n = ds[0].base;
                let nf = ((f * n as f64).round() as usize).clamp(1, n);
                for d in ds {
                    n_tot += 1.0;
                    let k = d.idx.iter().filter(|&&i| i >= nf).count().min(3);
                    obs_prefix[k] += 1.0;
                    let mut acc = [0f64; 4];
                    let mut perm: Vec<usize> = (0..n).collect();
                    for _ in 0..draws {
                        perm.shuffle(&mut rng);
                        // position of each base element in the random order; F' = first nf of perm
                        let mut inf = vec![false; n];
                        for &q in &perm[..nf] {
                            inf[q] = true;
                        }
                        let k = d.idx.iter().filter(|&&i| !inf[i]).count().min(3);
                        acc[k] += 1.0 / draws as f64;
                    }
                    for k in 0..4 {
                        obs_rand[k] += acc[k];
                    }
                    for k in 0..4 {
                        let (h, b) = if k < 3 {
                            (hyper(n, nf, k), binom(nf as f64 / n as f64, k))
                        } else {
                            (
                                1.0 - (0..3).map(|j| hyper(n, nf, j)).sum::<f64>(),
                                1.0 - (0..3).map(|j| binom(nf as f64 / n as f64, j)).sum::<f64>(),
                            )
                        };
                        model_h[k] += h;
                        model_b[k] += b;
                    }
                }
            }
            let cells: Vec<Value> = (0..4)
                .map(|k| {
                    let (lo, hi) = wilson(obs_prefix[k], n_tot);
                    let mh = model_h[k] / n_tot;
                    let z = (obs_prefix[k] / n_tot - mh) / (mh * (1.0 - mh) / n_tot).sqrt().max(1e-12);
                    json!({"k_outside": k, "measured_prefix": obs_prefix[k] / n_tot, "wilson_lo": lo, "wilson_hi": hi,
                        "measured_random_subsets": obs_rand[k] / n_tot, "model_hypergeometric": mh,
                        "model_binomial": model_b[k] / n_tot, "z_vs_hypergeometric": z})
                })
                .collect();
            rows.push(json!({"f": f, "n_decompositions": n_tot, "cells": cells}));
        }
        // relation counts the real decompositions give each scheme, against the uniform model
        let mut graph = vec![];
        for &f in &[0.5, 0.7, 0.8, 0.9, 0.95] {
            for scheme in [Scheme::Single, Scheme::Double, Scheme::Generic] {
                let (mut real, mut model, mut real_t, mut model_t, mut cnt) = (0f64, 0f64, 0f64, 0f64, 0f64);
                for ds in seeds.values() {
                    let n = ds[0].base;
                    let nf = ((f * n as f64).round() as usize).clamp(1, n);
                    // random F' per draw; the real decompositions in logged order
                    for _ in 0..draws.min(100) {
                        let mut perm: Vec<usize> = (0..n).collect();
                        perm.shuffle(&mut rng);
                        let mut rank = vec![0usize; n];
                        for (pos, &q) in perm.iter().enumerate() {
                            rank[q] = pos;
                        }
                        let list: Vec<[usize; M]> = ds
                            .iter()
                            .map(|d| {
                                let mut a = [0usize; M];
                                for k in 0..M {
                                    a[k] = rank[d.idx[k]];
                                }
                                a
                            })
                            .collect();
                        let (r, t) = relations_from(&list, n, nf, scheme);
                        real += r as f64;
                        real_t += t as f64;
                        let sim: Vec<[usize; M]> = (0..list.len()).map(|_| six(&mut rng, n)).collect();
                        let (r, t) = relations_from(&sim, n, nf, scheme);
                        model += r as f64;
                        model_t += t as f64;
                        cnt += 1.0;
                    }
                }
                graph.push(json!({"f": f, "scheme": format!("{scheme:?}"),
                    "mean_relations_real": real / cnt, "mean_relations_uniform_model": model / cnt,
                    "mean_touched_Fprime_real": real_t / cnt, "mean_touched_Fprime_model": model_t / cnt}));
            }
        }
        report.push(json!({"p": p, "seeds": seeds.len(), "decompositions": group.len(),
            "uniformity": unif, "yields": rows, "relation_counts": graph}));
    }
    fs::write(&out, serde_json::to_string_pretty(&json!(report)).unwrap()).unwrap();
    println!("wrote {out}");
}

// ── subcommand: curves ────────────────────────────────────────────────────

fn cmd_curves(args: &[String]) {
    let mut out = String::from("curves.json");
    let mut ns: Vec<usize> = vec![50, 129, 484, 2000, 6000];
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out = args[i].clone();
            }
            "--n" => {
                i += 1;
                ns = args[i].split(',').map(|v| v.parse().unwrap()).collect();
            }
            a => panic!("arg {a}"),
        }
        i += 1;
    }
    let grid: Vec<f64> = vec![0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95];
    let mut res = vec![];
    for &n in &ns {
        let reps = (24000 / n).clamp(6, 400);
        let mut rng = StdRng::seed_from_u64(0xC0DE ^ n as u64);
        // plain, f = 1
        let plain: Vec<f64> = (0..reps)
            .map(|_| trial(n, 1.0, Scheme::Plain, &mut rng, 4 * n).unwrap() as f64 / n as f64)
            .collect();
        let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
        let sd = |v: &[f64]| {
            let m = mean(v);
            (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0).max(1.0)).sqrt()
        };
        let mut rows = vec![];
        for &f in &grid {
            let mut cell = serde_json::Map::new();
            cell.insert("f".into(), json!(f));
            for (name, scheme) in [("single", Scheme::Single), ("double", Scheme::Double), ("generic", Scheme::Generic)] {
                // cap: 1-LP needs about f^-5 N; stop at 80 N and report as not reached
                let cap = 80 * n;
                let r = if matches!(scheme, Scheme::Generic) { (reps / 3).max(4) } else { reps };
                let v: Vec<f64> = (0..r)
                    .filter_map(|_| trial(n, f, scheme, &mut rng, cap).map(|x| x as f64 / n as f64))
                    .collect();
                if v.len() == r {
                    cell.insert(name.into(), json!({"mean": mean(&v), "sd": sd(&v), "reps": r}));
                } else {
                    cell.insert(name.into(), json!({"mean": null, "reps": r, "reached": v.len()}));
                }
            }
            rows.push(Value::Object(cell));
            eprintln!("N={n} f={f} done");
        }
        res.push(json!({"N": n, "reps": reps, "plain": {"mean": mean(&plain), "sd": sd(&plain)}, "rows": rows}));
    }
    fs::write(&out, serde_json::to_string_pretty(&json!(res)).unwrap()).unwrap();
    println!("wrote {out}");
}

// ── subcommand: cost ──────────────────────────────────────────────────────

/// log-linear interpolation of mean X/N in f, with power-law extrapolation below the grid.
fn interp(rows: &[(f64, f64)], f: f64) -> f64 {
    // rows ascending in f, y = X/N
    if f >= rows.last().unwrap().0 {
        return rows.last().unwrap().1;
    }
    if f <= rows[0].0 {
        // extrapolate with the slope of the first two points in log-log
        let (f0, y0) = rows[0];
        let (f1, y1) = rows[1];
        let a = (y1.ln() - y0.ln()) / (f1.ln() - f0.ln());
        return (y0.ln() + a * (f.ln() - f0.ln())).exp();
    }
    for w in rows.windows(2) {
        if f >= w[0].0 && f <= w[1].0 {
            let t = (f.ln() - w[0].0.ln()) / (w[1].0.ln() - w[0].0.ln());
            return (w[0].1.ln() * (1.0 - t) + w[1].1.ln() * t).exp();
        }
    }
    unreachable!()
}

fn cmd_cost(args: &[String]) {
    let mut out = String::from("cost.json");
    let mut curves_path = String::new();
    let mut dlps: Vec<String> = vec![];
    let mut ccov_path = String::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out = args[i].clone();
            }
            "--curves" => {
                i += 1;
                curves_path = args[i].clone();
            }
            "--ccov-old" => {
                i += 1;
                ccov_path = args[i].clone();
            }
            "--dlp" => {
                i += 1;
                while i < args.len() && !args[i].starts_with("--") {
                    dlps.push(args[i].clone());
                    i += 1;
                }
                continue;
            }
            a => panic!("arg {a}"),
        }
        i += 1;
    }
    // constants from recorded dlp runs
    let (mut resid, mut decs) = (0f64, 0f64);
    let (mut c_test_sum, mut c_test_n, mut la_ops_sum, mut u2_sum) = (0f64, 0f64, 0f64, 0f64);
    for path in &dlps {
        let v: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        for r in v.as_array().unwrap() {
            if r["correct"] != json!(true) {
                continue;
            }
            resid += r["residuals"].as_f64().unwrap();
            decs += r["decompositions"].as_f64().unwrap();
        }
    }
    // the post-#1717 records carry the current per-test cost; they are listed first by name "dlp_new"
    for path in dlps.iter().filter(|p| p.contains("dlp_new")) {
        let v: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        for r in v.as_array().unwrap() {
            let total = r["total_muls"].as_f64().unwrap();
            let la = r["la_muls"].as_f64().unwrap();
            c_test_sum += (total - la) / r["residuals"].as_f64().unwrap();
            c_test_n += 1.0;
            la_ops_sum += r["la_ops"].as_f64().unwrap();
            u2_sum += (r["unknowns"].as_f64().unwrap()).powi(2);
        }
    }
    let t_hat = resid / decs;
    let t_ci = (t_hat / decs.sqrt() * 1.96, decs);
    let c_test_101 = c_test_sum / c_test_n;
    let la_const = 16.0 * la_ops_sum / u2_sum; // F_p mults per unknown^2
    // C_cov growth with p from the pre-#1717 ccov record (ratio), post-#1717 anchor at 101
    let old: Value = serde_json::from_str(&fs::read_to_string(&ccov_path).unwrap()).unwrap();
    let mut old_by_p: HashMap<u64, Vec<f64>> = HashMap::new();
    for r in old.as_array().unwrap() {
        old_by_p.entry(r["p"].as_u64().unwrap()).or_default().push(r["c_cov"]["mean"].as_f64().unwrap());
    }
    let om = |p: u64| old_by_p[&p].iter().sum::<f64>() / old_by_p[&p].len() as f64;
    let slope_per_log2 = (om(1511) - om(101)) / om(101) / (1511f64 / 101.0).log2();
    let c_of_p = |p: f64| -> f64 {
        let rel = if p <= 1511.0 {
            // interpolate the old ratio in log p
            let known = [101u64, 251, 503, 1009, 1511];
            let mut r = 1.0;
            for w in known.windows(2) {
                if p >= w[0] as f64 && p <= w[1] as f64 {
                    let t = (p.ln() - (w[0] as f64).ln()) / ((w[1] as f64).ln() - (w[0] as f64).ln());
                    r = (om(w[0]) * (1.0 - t) + om(w[1]) * t) / om(101);
                }
            }
            r
        } else {
            om(1511) / om(101) * (1.0 + slope_per_log2 * (p / 1511.0).log2())
        };
        c_test_101 * rel
    };
    let curves: Value = serde_json::from_str(&fs::read_to_string(&curves_path).unwrap()).unwrap();
    let curve_for = |n: usize, scheme: &str| -> (Vec<(f64, f64)>, f64, usize) {
        let arr = curves.as_array().unwrap();
        let c = arr
            .iter()
            .min_by_key(|c| ((c["N"].as_f64().unwrap().ln() - (n as f64).ln()).abs() * 1e6) as i64)
            .unwrap();
        let mut rows: Vec<(f64, f64)> = c["rows"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|r| r[scheme]["mean"].as_f64().map(|m| (r["f"].as_f64().unwrap(), m)))
            .collect();
        rows.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        (rows, c["plain"]["mean"].as_f64().unwrap(), c["N"].as_u64().unwrap() as usize)
    };

    let ps: Vec<f64> = vec![101.0, 251.0, 1009.0, 4096.0, 12000.0, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e12];
    let mut table = vec![];
    for &p in &ps {
        let n = p / 2.0;
        let c = c_of_p(p);
        let rel_unit = t_hat * c; // F_p mults per solution
        let s_full = (n + 1.0) * rel_unit + la_const * n * n;
        let la_share = la_const * n * n / s_full;
        let mut schemes = serde_json::Map::new();
        for sch in ["single", "double", "generic"] {
            let (rows, plain_x, nn) = curve_for(n as usize, sch);
            let mut best = (1.0f64, 1.0f64);
            let mut best_s = s_full;
            let mut best_x = 0.0;
            let mut sweep = vec![];
            // search f in (0.001, 1)
            let mut f = 0.001f64;
            while f < 1.0 {
                let x = interp(&rows, f) * n;
                let u = f * n;
                let s = x * rel_unit + la_const * u * u;
                if (f * 100.0).ln().rem_euclid(0.15) < 0.0101 {
                    sweep.push(json!({"f": f, "alpha": (u.max(1.0)).ln() / n.ln(), "X_over_N": x / n, "S_over_Sfull": s / s_full}));
                }
                if s < best_s {
                    best_s = s;
                    best = (f, s / s_full);
                    best_x = x / n;
                }
                f *= 1.01;
            }
            schemes.insert(
                sch.into(),
                json!({"curve_N": nn, "f_opt": best.0, "S_over_Sfull": best.1, "X_over_N_at_opt": best_x,
                       "ideal_plain_X_over_N": plain_x, "sweep": sweep}),
            );
        }
        table.push(json!({"p": p, "N": n, "C_test": c, "S_full": s_full, "LA_share_plain": la_share, "schemes": schemes}));
    }
    // p at which the LA share of the plain method reaches 1 %, 10 %, 50 % (bisection, N = p/2)
    let mut thresholds = serde_json::Map::new();
    for share in [0.01, 0.1, 0.5] {
        let (mut lo, mut hi) = (1e2f64, 1e12f64);
        for _ in 0..200 {
            let mid = (lo * hi).sqrt();
            let n = mid / 2.0;
            let rel = (n + 1.0) * t_hat * c_of_p(mid);
            let la = la_const * n * n;
            if la / (la + rel) < share {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        thresholds.insert(format!("LA_share_{share}"), json!(lo));
    }
    let rep = json!({
        "constants": {"T_tests_per_decomposition": t_hat, "T_ci95_halfwidth": t_ci.0, "decompositions_pooled": t_ci.1,
            "C_test_p101_mults": c_test_101, "LA_mults_per_unknown2": la_const,
            "C_cov_growth_per_log2p_relative": slope_per_log2},
        "table": table,
        "p_at_LA_share": thresholds,
    });
    fs::write(&out, serde_json::to_string_pretty(&rep).unwrap()).unwrap();
    println!("{}", serde_json::to_string_pretty(&rep["constants"]).unwrap());
    println!("{}", serde_json::to_string_pretty(&rep["p_at_LA_share"]).unwrap());
    println!("wrote {out}");
}

fn main() {
    let a: Vec<String> = env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("yields") => cmd_yields(&a[2..]),
        Some("curves") => cmd_curves(&a[2..]),
        Some("cost") => cmd_cost(&a[2..]),
        _ => eprintln!("usage: jv_lp_model yields|curves|cost ..."),
    }
}
