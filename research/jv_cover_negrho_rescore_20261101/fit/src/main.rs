//! Analysis for research/jv_cover_negrho_rescore_20261101 (frozen in PROTOCOL.md).
//! Usage:  negrho_rescore <results dir> <out dir>      full analysis, writes fit.json, fit.txt, table.md, scaling.svg
//!         negrho_rescore replay <old.json> <new.json>  field-by-field equality of two run records (wall_ms ignored)
//! Every number is computed here, in Rust.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const NB: usize = 20000; // bootstrap resamples (frozen)
const NPERM: usize = 20000; // permutation resamples for the homogeneity tests (frozen)
const SLOPE: f64 = -2.10; // crossover-study slope (frozen, model)
const R_WALK: f64 = 256.0;
const WS: [f64; 3] = [0.0, 0.1, 1.0];
const PRIMES: [u64; 3] = [1301, 1451, 1601];
const SEEDS: [u64; 3] = [1, 2, 3];

// ---------- RNGs ----------
/// the predecessor study's generator, kept only to reproduce its numbers exactly
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn idx(&mut self, n: usize) -> usize { (self.next() as usize) % n }
}
/// this study's generator (splitmix64)
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn idx(&mut self, n: usize) -> usize { (((self.next() as u128) * (n as u128)) >> 64) as usize }
}

// ---------- statistics ----------
fn mean(v: &[f64]) -> f64 { v.iter().sum::<f64>() / v.len() as f64 }
fn sd(v: &[f64]) -> f64 {
    if v.len() < 2 { return f64::NAN; }
    let m = mean(v);
    (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
}
fn gmean(v: &[f64]) -> f64 { (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp() }
fn pct(v: &[f64], q: f64) -> f64 {
    if v.is_empty() { return f64::NAN; }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    s[((s.len() - 1) as f64 * q).round() as usize]
}
fn verdict(lo: f64, hi: f64) -> &'static str {
    if hi < 1.0 { "below" } else if lo > 1.0 { "above" } else { "not resolved" }
}
fn anova_f(g: &[Vec<f64>]) -> f64 {
    let all: Vec<f64> = g.iter().flatten().cloned().collect();
    let (n, k) = (all.len() as f64, g.len() as f64);
    let gm = mean(&all);
    let ssb: f64 = g.iter().map(|x| x.len() as f64 * (mean(x) - gm).powi(2)).sum();
    let ssw: f64 = g.iter().map(|x| { let m = mean(x); x.iter().map(|v| (v - m).powi(2)).sum::<f64>() }).sum();
    (ssb / (k - 1.0)) / (ssw / (n - k))
}
fn kw_h(g: &[Vec<f64>]) -> f64 {
    let mut all: Vec<(f64, usize)> = vec![];
    for (i, x) in g.iter().enumerate() { for v in x { all.push((*v, i)); } }
    all.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let n = all.len();
    let mut rank = vec![0.0; n];
    let mut i = 0;
    let mut tie = 0.0;
    while i < n {
        let mut j = i;
        while j + 1 < n && all[j + 1].0 == all[i].0 { j += 1; }
        let r = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j { rank[k] = r; }
        let t = (j - i + 1) as f64;
        tie += t * t * t - t;
        i = j + 1;
    }
    let mut rs = vec![0.0; g.len()];
    for (k, a) in all.iter().enumerate() { rs[a.1] += rank[k]; }
    let nf = n as f64;
    let h: f64 = 12.0 / (nf * (nf + 1.0)) * rs.iter().zip(g).map(|(r, x)| r * r / x.len() as f64).sum::<f64>() - 3.0 * (nf + 1.0);
    h / (1.0 - tie / (nf * nf * nf - nf))
}
/// returns (F, p_perm(F), H, p_perm(H), p_chi2(H) if df == 2)
fn homogeneity(g: &[Vec<f64>], rng: &mut Rng) -> (f64, f64, f64, f64, Option<f64>) {
    let (f0, h0) = (anova_f(g), kw_h(g));
    let sizes: Vec<usize> = g.iter().map(|x| x.len()).collect();
    let mut pool: Vec<f64> = g.iter().flatten().cloned().collect();
    let (mut cf, mut ch) = (0usize, 0usize);
    for _ in 0..NPERM {
        for i in (1..pool.len()).rev() { let j = rng.idx(i + 1); pool.swap(i, j); }
        let mut off = 0;
        let pg: Vec<Vec<f64>> = sizes.iter().map(|&s| { let v = pool[off..off + s].to_vec(); off += s; v }).collect();
        if anova_f(&pg) >= f0 { cf += 1; }
        if kw_h(&pg) >= h0 { ch += 1; }
    }
    let pf = (1 + cf) as f64 / (1 + NPERM) as f64;
    let ph = (1 + ch) as f64 / (1 + NPERM) as f64;
    let chi = if g.len() == 3 { Some((-h0 / 2.0).exp()) } else { None };
    (f0, pf, h0, ph, chi)
}

// ---------- IO ----------
fn rows(path: &Path) -> Vec<Value> {
    let s = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str::<Value>(&s).unwrap().as_array().cloned().unwrap_or_default()
}
fn files_with(dir: &Path, prefix: &str) -> Vec<PathBuf> {
    let mut f: Vec<PathBuf> = fs::read_dir(dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| { let n = p.file_name().unwrap().to_string_lossy().to_string(); n.starts_with(prefix) && n.ends_with(".json") })
        .collect();
    f.sort();
    f
}
fn fl(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap_or_else(|| panic!("missing {k}")) }
fn nat_of(r: &Value) -> f64 {
    ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| fl(&r["ops"], k)).sum()
}

/// one rhostrong run, every quantity per sqrt(l)
#[derive(Clone, Debug)]
struct Run {
    p: u64, seed: u64, run: u64, l: f64, c_e: f64,
    steps_n: f64, // steps / sqrt(l)
    u1: f64,      // s_harness = muls_harness / (c_add_e sqrt l)
    u2: f64,      // s_tuned   = muls_tuned   / (c_add_e sqrt l)
    nat: f64,     // native ops (canon_cmp+hash+dp_check+scalar_ops+table_ops+cycle_cmp) / sqrt l
    fpa: f64,     // F_p additions / sqrt l
    src: String,
}
impl Run { fn v(&self) -> [f64; 6] { [self.u1, self.u2, self.steps_n, self.nat, self.fpa, 0.0] } }
fn load_runs(dir: &Path, prefix: &str, fails: &mut Vec<String>) -> Vec<Run> {
    let mut out = vec![];
    for f in files_with(dir, prefix) {
        for r in rows(&f) {
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            let (p, seed, run) = (r["p"].as_u64().unwrap(), r["seed"].as_u64().unwrap(), r["run"].as_u64().unwrap());
            if !r["correct"].as_bool().unwrap() || r["capped"].as_bool().unwrap() || !r["negation"].as_bool().unwrap()
                || r["batch"].as_u64().unwrap() != 16 || r["r"].as_u64().unwrap() != 256 {
                fails.push(format!("{name}: p={p} seed={seed} run={run} excluded (incorrect, capped or wrong settings)"));
                continue;
            }
            let l = fl(&r, "l");
            let sl = l.sqrt();
            let c_e = fl(&r, "c_add_e");
            out.push(Run { p, seed, run, l, c_e, steps_n: fl(&r, "steps") / sl, u1: fl(&r, "s_harness"), u2: fl(&r, "s_tuned"),
                nat: nat_of(&r) / sl, fpa: fl(&r["ops"], "fp_adds") / sl, src: name });
        }
    }
    out
}

#[derive(Clone)]
struct Cover { p: u64, seed: u64, l: f64, c_e: f64, s: f64, nat: f64, total_muls: f64, ok: bool }
fn cover_row(r: &Value) -> Cover {
    let l = fl(r, "l");
    let c_e = fl(r, "c_add_e");
    let tm = r["total_muls"].as_u64().unwrap() as f64;
    Cover { p: r["p"].as_u64().unwrap(), seed: r["seed"].as_u64().unwrap(), l, c_e, s: tm / (c_e * l.sqrt()),
        nat: r["f4_native_ops"].as_u64().unwrap() as f64 / l.sqrt(), total_muls: tm,
        ok: r["solved"].as_bool().unwrap() && r["correct"].as_bool().unwrap() && r["mismatches"].as_u64().unwrap() == 0 }
}

type Groups = Vec<Vec<[f64; 6]>>; // reference runs, grouped by instance
fn gsum(g: &Groups) -> ([f64; 6], f64) {
    let mut s = [0.0; 6];
    let mut n = 0.0;
    for x in g { for v in x { for k in 0..6 { s[k] += v[k]; } n += 1.0; } }
    (s, n)
}
fn gmeanvec(g: &Groups) -> [f64; 6] { let (s, n) = gsum(g); let mut m = [0.0; 6]; for k in 0..6 { m[k] = s[k] / n; } m }
fn resample_sum(g: &[Vec<[f64; 6]>], rng: &mut Rng) -> ([f64; 6], f64) {
    let mut s = [0.0; 6];
    let mut n = 0.0;
    for x in g { for _ in 0..x.len() { let v = &x[rng.idx(x.len())]; for k in 0..6 { s[k] += v[k]; } n += 1.0; } }
    (s, n)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() >= 4 && a[1] == "replay" { replay(&a[2], &a[3]); return; }
    let (rdir, out) = (Path::new(&a[1]), Path::new(&a[2]));
    let inp = rdir.join("inputs");
    let mut txt = String::new();
    macro_rules! say { ($($t:tt)*) => {{ let s = format!($($t)*); println!("{s}"); txt += &s; txt += "\n"; }} }
    let mut fails: Vec<String> = vec![];
    let theory = (std::f64::consts::PI / 4.0).sqrt() / (1.0 - 1.0 / R_WALK).sqrt();
    say!("negrho_rescore; frozen constants: NB {NB}, NPERM {NPERM}, slope {SLOPE}, theory steps/sqrt(l) = sqrt(pi/4)/sqrt(1-1/{R_WALK}) = {theory:.5}");

    // ---------------- cover arm: 9 existing records ----------------
    let mut cover: BTreeMap<u64, Vec<Cover>> = BTreeMap::new();
    for f in files_with(&inp.join("cover"), "dlp_p") { for r in rows(&f) {
        let c = cover_row(&r);
        if !c.ok { fails.push(format!("cover p={} seed={} not solved/correct", c.p, c.seed)); continue; }
        cover.entry(c.p).or_default().push(c);
    } }
    for p in PRIMES { assert_eq!(cover[&p].len(), 3, "cover rows at {p}"); }

    // ---------------- rhostrong runs ----------------
    let old = load_runs(&inp.join("old_rhostrong"), "rhostrong_p", &mut fails);
    let newr_all = load_runs(&rdir.join("new"), "rhostrong_p", &mut fails);
    let rt = load_runs(&inp.join("redteam_p251"), "rhostrong_neg_p251", &mut fails);
    // l must match the cover records
    let mut l_ok = true;
    for r in old.iter().chain(newr_all.iter()) {
        let c = cover[&r.p].iter().find(|c| c.seed == r.seed).expect("cover seed");
        if c.l != r.l || c.c_e != r.c_e { l_ok = false; fails.push(format!("l or c_add_e mismatch p={} seed={} run={}", r.p, r.seed, r.run)); }
    }
    say!("l and c_add_e of all {} rhostrong runs match the cover records: {l_ok}", old.len() + newr_all.len());
    // complete rounds: a run index counts only if all 9 instances have it (frozen cut rule)
    let mut have: BTreeMap<u64, usize> = BTreeMap::new();
    for r in &newr_all { *have.entry(r.run).or_default() += 1; }
    let mut excluded_rounds: Vec<u64> = vec![];
    let mut complete: Vec<u64> = vec![];
    for (k, n) in &have { if *n == 9 { complete.push(*k) } else { excluded_rounds.push(*k) } }
    let newr: Vec<Run> = newr_all.iter().filter(|r| complete.contains(&r.run)).cloned().collect();
    say!("new run indices complete on all 9 instances: {:?}; incomplete (excluded from analysis): {:?}", complete, excluded_rounds);
    for r in &newr { assert!(r.run >= 2, "new run index collides with run0/run1"); }

    // reference sets by instance (p, seed)
    let mk = |rs: &[&Run]| -> BTreeMap<(u64, u64), Vec<Run>> {
        let mut m: BTreeMap<(u64, u64), Vec<Run>> = BTreeMap::new();
        for r in rs { m.entry((r.p, r.seed)).or_default().push((*r).clone()); }
        for v in m.values_mut() { v.sort_by_key(|r| r.run); }
        m
    };
    let all_new_primes: Vec<&Run> = old.iter().chain(newr.iter()).collect();
    let inst = mk(&all_new_primes);
    let inst251 = mk(&rt.iter().collect::<Vec<_>>());
    let g_of = |m: &BTreeMap<(u64, u64), Vec<Run>>, p: Option<u64>| -> Groups {
        m.iter().filter(|((q, _), _)| p.map_or(true, |p| *q == p)).map(|(_, v)| v.iter().map(|r| r.v()).collect()).collect()
    };
    let n_new_per_inst = complete.len();
    say!("reference runs per instance: {} old + {} new = {}; per prime {}; new primes {}; red-team p=251 {}",
        2, n_new_per_inst, 2 + n_new_per_inst, 3 * (2 + n_new_per_inst), 9 * (2 + n_new_per_inst), rt.len());

    for ((p, seed), v) in &inst {
        let st: Vec<f64> = v.iter().map(|r| r.steps_n).collect();
        say!("instance p={p} seed={seed}: n {} steps/sqrt(l) mean {:.4} sd {:.3} (l = {:.6e})", st.len(), mean(&st), sd(&st), v[0].l);
    }
    // ---------------- instrument identity: s_harness vs steps/sqrt(l) ----------------
    {
        let dev: Vec<f64> = old.iter().chain(newr.iter()).map(|r| r.u1 / r.steps_n - 1.0).collect();
        let (mn, mx) = (dev.iter().cloned().fold(f64::INFINITY, f64::min), dev.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        let dev2: Vec<f64> = old.iter().chain(newr.iter()).map(|r| r.u2 / r.u1).collect();
        say!("s_harness / (steps/sqrt l) - 1 over the {} new-prime runs: mean {:+.5}, min {:+.5}, max {:+.5}; s_tuned/s_harness: min {:.5}, max {:.5}",
            dev.len(), mean(&dev), mn, mx, dev2.iter().cloned().fold(f64::INFINITY, f64::min), dev2.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    }

    // ---------------- homogeneity ----------------
    let mut rng = Rng(0x5EED_2026_1101);
    let per_prime_vals = |f: &dyn Fn(&Run) -> f64, with251: bool| -> Vec<Vec<f64>> {
        let mut g: Vec<Vec<f64>> = vec![];
        if with251 { g.push(rt.iter().map(|r| f(r)).collect()); }
        for p in PRIMES { g.push(inst.iter().filter(|((q, _), _)| *q == p).flat_map(|(_, v)| v.iter().map(|r| f(r))).collect()); }
        g
    };
    let mut hom_json = json!({});
    let mut homogeneous = true;
    for (name, f) in [("steps/sqrt(l)", (&|r: &Run| r.steps_n) as &dyn Fn(&Run) -> f64), ("s_harness", &|r: &Run| r.u1), ("s_tuned", &|r: &Run| r.u2)] {
        let g3 = per_prime_vals(f, false);
        let (fa, pf, h, ph, pc) = homogeneity(&g3, &mut rng);
        say!("HOMOGENEITY across p in {{1301,1451,1601}}, {name}: group means {:?} (n {:?}); one-way ANOVA F {fa:.4} perm-p {pf:.4}; Kruskal-Wallis H {h:.4} perm-p {ph:.4} chi2(2)-p {:.4}",
            g3.iter().map(|x| (mean(x) * 1e4).round() / 1e4).collect::<Vec<_>>(), g3.iter().map(|x| x.len()).collect::<Vec<_>>(), pc.unwrap());
        if name == "steps/sqrt(l)" { homogeneous = pf >= 0.05 && ph >= 0.05; }
        let g4 = per_prime_vals(f, true);
        let (fa4, pf4, h4, ph4, _) = homogeneity(&g4, &mut rng);
        say!("  incl. p=251 red-team runs (4 groups, information): means {:?}; ANOVA F {fa4:.4} perm-p {pf4:.4}; KW H {h4:.4} perm-p {ph4:.4}",
            g4.iter().map(|x| (mean(x) * 1e4).round() / 1e4).collect::<Vec<_>>());
        hom_json[name] = json!({"new_primes": {"anova_F": fa, "anova_perm_p": pf, "kw_H": h, "kw_perm_p": ph, "kw_chi2_p": pc},
            "with_p251": {"anova_F": fa4, "anova_perm_p": pf4, "kw_H": h4, "kw_perm_p": ph4}});
    }
    {
        // across the 9 instances (diagnostic)
        let g9: Vec<Vec<f64>> = inst.values().map(|v| v.iter().map(|r| r.steps_n).collect()).collect();
        let (fa, pf, h, ph, _) = homogeneity(&g9, &mut rng);
        say!("  across the 9 instances (diagnostic), steps/sqrt(l): ANOVA F {fa:.4} perm-p {pf:.4}; KW H {h:.4} perm-p {ph:.4}");
        hom_json["across_9_instances_steps"] = json!({"anova_F": fa, "anova_perm_p": pf, "kw_H": h, "kw_perm_p": ph});
    }
    let primary = if homogeneous { "b" } else { "a" };
    say!("FROZEN CHOICE: homogeneity {} (both permutation p >= 0.05 on steps/sqrt(l)) -> primary reference estimator ({}) {}", if homogeneous { "holds" } else { "FAILS" }, primary,
        if homogeneous { "pooled across the three new primes" } else { "per-prime mean" });

    // ---------------- reference means and positive control ----------------
    let g_prime: BTreeMap<u64, Groups> = PRIMES.iter().map(|&p| (p, g_of(&inst, Some(p)))).collect();
    let g_b = g_of(&inst, None);
    let mut g_c = g_b.clone();
    g_c.extend(g_of(&inst251, None));
    let mean_a: BTreeMap<u64, [f64; 6]> = PRIMES.iter().map(|&p| (p, gmeanvec(&g_prime[&p]))).collect();
    let mean_b = gmeanvec(&g_b);
    let mean_c = gmeanvec(&g_c);
    let steps_of = |g: &Groups| -> Vec<f64> { g.iter().flatten().map(|v| v[2]).collect() };
    let mut pos_json = json!({});
    let mut pos_pass = true;
    for (name, g) in [("a1301", &g_prime[&1301]), ("a1451", &g_prime[&1451]), ("a1601", &g_prime[&1601]), ("b", &g_b), ("c", &g_c)] {
        let v = steps_of(g);
        let (m, se) = (mean(&v), sd(&v) / (v.len() as f64).sqrt());
        let z = (m - theory) / se;
        say!("POSITIVE CONTROL ({name}): n {} steps/sqrt(l) mean {m:.4} sd {:.3} se {se:.4}; theory {theory:.4}; z {z:+.2}; within 3 se: {}", v.len(), sd(&v), z.abs() <= 3.0);
        if name == "b" || name == "c" { if z.abs() > 3.0 { pos_pass = false; } }
        pos_json[name] = json!({"n": v.len(), "mean": m, "sd": sd(&v), "se": se, "theory": theory, "z": z, "within_3se": z.abs() <= 3.0});
    }
    say!("positive control on the pooled references (b) and (c): {}", if pos_pass { "PASSED" } else { "FAILED: reference flagged" });
    for p in PRIMES {
        let m = &mean_a[&p];
        say!("ref (a) p={p}: n {} steps/sqrt(l) {:.4} s_harness {:.4} s_tuned {:.4} native/sqrt(l) {:.2} fp_adds/sqrt(l) {:.1}", g_prime[&p].iter().map(|x| x.len()).sum::<usize>(), m[2], m[0], m[1], m[3], m[4]);
    }
    for (n, m, g) in [("b", &mean_b, &g_b), ("c", &mean_c, &g_c)] {
        say!("ref ({n}): n {} steps/sqrt(l) {:.4} s_harness {:.4} s_tuned {:.4} native/sqrt(l) {:.2} fp_adds/sqrt(l) {:.1}", g.iter().map(|x| x.len()).sum::<usize>(), m[2], m[0], m[1], m[3], m[4]);
    }

    // ---------------- main bootstrap ----------------
    // est index 0=a,1=b,2=c ; unit 0=U1,1=U2 ; w index 0..3 ; prime index 0..3
    let cs: Vec<Vec<Cover>> = PRIMES.iter().map(|p| { let mut v = cover[p].clone(); v.sort_by_key(|c| c.seed); v }).collect();
    let ref_mean = |est: usize, pi: usize| -> [f64; 6] { match est { 0 => mean_a[&PRIMES[pi]], 1 => mean_b, _ => mean_c } };
    let ratio = |gm_w: f64, m: &[f64; 6], unit: usize, w: f64| -> f64 { gm_w / (m[unit] + w * m[3]) };
    // point estimates
    let mut pt = [[[[0.0f64; 3]; 3]; 3]; 2]; // [unit][est][w][prime]
    for unit in 0..2 { for est in 0..3 { for (wi, w) in WS.iter().enumerate() { for pi in 0..3 {
        let gm = gmean(&cs[pi].iter().map(|c| c.s + w * c.nat).collect::<Vec<_>>());
        pt[unit][est][wi][pi] = ratio(gm, &ref_mean(est, pi), unit, *w);
    } } } }
    let mut bs: Vec<Vec<f64>> = vec![Vec::with_capacity(NB); 2 * 3 * 3 * 3]; // index ((unit*3+est)*3+w)*3+prime
    let bidx = |unit: usize, est: usize, wi: usize, pi: usize| ((unit * 3 + est) * 3 + wi) * 3 + pi;
    let mut pstar_bs: Vec<Vec<f64>> = vec![Vec::with_capacity(NB); 2 * 3]; // unit, est (w = 0)
    let mut refmean_bs: Vec<Vec<f64>> = vec![Vec::with_capacity(NB); 3 * 2]; // est, unit -> bootstrap of the pooled ref (b/c) or prime-1451 (a)
    let g_251: Groups = g_of(&inst251, None);
    let group_keys: Vec<(u64, u64)> = inst.keys().cloned().collect();
    let g_inst: Vec<Vec<[f64; 6]>> = g_of(&inst, None);
    for _ in 0..NB {
        // resample every instance group once (paired across estimators)
        let rs: Vec<([f64; 6], f64)> = g_inst.iter().map(|g| resample_sum(std::slice::from_ref(g), &mut rng)).collect();
        let r251: Vec<([f64; 6], f64)> = g_251.iter().map(|g| resample_sum(std::slice::from_ref(g), &mut rng)).collect();
        let comb = |idx: &[usize], extra: bool| -> [f64; 6] {
            let mut s = [0.0; 6];
            let mut n = 0.0;
            for &i in idx { for k in 0..6 { s[k] += rs[i].0[k]; } n += rs[i].1; }
            if extra { for r in &r251 { for k in 0..6 { s[k] += r.0[k]; } n += r.1; } }
            for k in 0..6 { s[k] /= n; }
            s
        };
        let all_idx: Vec<usize> = (0..group_keys.len()).collect();
        let m_b = comb(&all_idx, false);
        let m_c = comb(&all_idx, true);
        let mut gms: Vec<Vec<f64>> = vec![vec![0.0; 3]; 3]; // [w][prime]
        let mut ma: Vec<[f64; 6]> = vec![];
        for pi in 0..3 {
            let idx: Vec<usize> = group_keys.iter().enumerate().filter(|(_, k)| k.0 == PRIMES[pi]).map(|(i, _)| i).collect();
            ma.push(comb(&idx, false));
            let pick: Vec<usize> = (0..3).map(|_| rng.idx(3)).collect();
            for (wi, w) in WS.iter().enumerate() {
                gms[wi][pi] = gmean(&pick.iter().map(|&k| cs[pi][k].s + w * cs[pi][k].nat).collect::<Vec<_>>());
            }
        }
        let mut gsl = [[0.0f64; 3]; 2]; // for p*: [unit][prime] ratio at w = 0 per est computed below
        let _ = &mut gsl;
        for unit in 0..2 { for est in 0..3 { for wi in 0..3 { for pi in 0..3 {
            let m = match est { 0 => &ma[pi], 1 => &m_b, _ => &m_c };
            bs[bidx(unit, est, wi, pi)].push(gms[wi][pi] / (m[unit] + WS[wi] * m[3]));
        } } } }
        for unit in 0..2 { for est in 0..3 {
            let mut acc = 0.0;
            for pi in 0..3 {
                let m = match est { 0 => &ma[pi], 1 => &m_b, _ => &m_c };
                let g = gms[0][pi] / m[unit];
                acc += g.ln() - SLOPE * (PRIMES[pi] as f64).ln();
            }
            pstar_bs[unit * 3 + est].push((acc / 3.0 / (-SLOPE)).exp());
        } }
        refmean_bs[0 * 2 + 0].push(m_b[2]);
        refmean_bs[1 * 2 + 0].push(m_c[2]);
    }
    let ci = |unit: usize, est: usize, wi: usize, pi: usize| -> (f64, f64) {
        let v = &bs[bidx(unit, est, wi, pi)];
        (pct(v, 0.025), pct(v, 0.975))
    };
    let est_name = ["(a) per-prime", "(b) pooled new primes", "(c) pooled + p=251"];
    let unit_name = ["U1 group additions", "U2 textbook F_p mults"];
    let pri = if primary == "b" { 1 } else { 0 };
    let mut tables = String::new();
    let mut res_json = json!({});
    for unit in 0..2 {
        for est in 0..3 {
            for pi in 0..3 {
                let m = ref_mean(est, pi);
                let (lo, hi) = ci(unit, est, 0, pi);
                let gm = gmean(&cs[pi].iter().map(|c| c.s).collect::<Vec<_>>());
                say!("{} | est {} | p={} | cover geo-mean S {:.4} | ref {:.4} | S/rho {:.3} [{:.3}, {:.3}] -> {}", unit_name[unit], est_name[est], PRIMES[pi], gm, m[unit], pt[unit][est][0][pi], lo, hi, verdict(lo, hi));
            }
        }
    }
    // distance remaining table (primary estimator)
    tables += &format!("### Result per prime, primary estimator {} (frozen choice: homogeneity {})\n\n", est_name[pri], if homogeneous { "holds" } else { "fails" });
    tables += "S/rho = geometric mean over the 3 cover seeds of S, divided by the reference mean; 95% percentile bootstrap, 20000 resamples of seeds and reference runs jointly (derived). Distance remaining = the factor by which the cover must improve to reach the matched rho (S/rho itself when above 1; the cover is ahead by 1/(S/rho) otherwise).\n\n";
    tables += "| unit | p | cover geo-mean S (measured) | reference mean (derived) | S/rho [95% boot] | verdict | distance remaining |\n|---|---|---|---|---|---|---|\n";
    for unit in 0..2 { for pi in 0..3 {
        let m = ref_mean(pri, pi);
        let (lo, hi) = ci(unit, pri, 0, pi);
        let gm = gmean(&cs[pi].iter().map(|c| c.s).collect::<Vec<_>>());
        let pt0 = pt[unit][pri][0][pi];
        let dist = if pt0 >= 1.0 { format!("x{:.2} [{:.2}, {:.2}]", pt0, lo, hi) } else { format!("none: ahead by x{:.2} [{:.2}, {:.2}]", 1.0 / pt0, 1.0 / hi, 1.0 / lo) };
        tables += &format!("| {} | {} | {:.3} | {:.4} | **{:.3}** [{:.3}, {:.3}] | {} | {} |\n", unit_name[unit], PRIMES[pi], gm, m[unit], pt0, lo, hi, verdict(lo, hi), dist);
    } }
    tables += "\n### All three reference estimators (w = 0)\n\n| unit | estimator | n | ref mean | p=1301 | p=1451 | p=1601 |\n|---|---|---|---|---|---|---|\n";
    for unit in 0..2 { for est in 0..3 {
        let n: String = match est { 0 => g_prime.values().map(|g| g.iter().map(|x| x.len()).sum::<usize>().to_string()).collect::<Vec<_>>().join("/"),
            1 => g_b.iter().map(|x| x.len()).sum::<usize>().to_string(), _ => g_c.iter().map(|x| x.len()).sum::<usize>().to_string() };
        let mm: String = match est { 0 => PRIMES.iter().map(|p| format!("{:.4}", mean_a[p][unit])).collect::<Vec<_>>().join("/"), 1 => format!("{:.4}", mean_b[unit]), _ => format!("{:.4}", mean_c[unit]) };
        let cells: Vec<String> = (0..3).map(|pi| { let (lo, hi) = ci(unit, est, 0, pi); format!("{:.3} [{:.3}, {:.3}] {}", pt[unit][est][0][pi], lo, hi, verdict(lo, hi)) }).collect();
        tables += &format!("| {} | {} | {} | {} | {} | {} | {} |\n", unit_name[unit], est_name[est], n, mm, cells[0], cells[1], cells[2]);
    } }
    // 5a band
    tables += "\n### Section 5a band (primary estimator; native operations priced at w group-addition equivalents each, on both arms)\n\n";
    tables += "Cover native count = `f4_native_ops` (emitted, a LOWER BOUND: F_p additions, pair selection, memory traffic not emitted). Reference native count = canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp (emitted by the walk; `fp_adds` listed separately below, not priced in the band).\n\n";
    tables += "| unit | p | w=0 | w=0.1 | w=1 | same verdict at all w | cover native/sqrt(l) (lower bound) | rho native/sqrt(l) |\n|---|---|---|---|---|---|---|---|\n";
    let mut band_json = vec![];
    for unit in 0..2 { for pi in 0..3 {
        let cells: Vec<String> = (0..3).map(|wi| { let (lo, hi) = ci(unit, pri, wi, pi); format!("{:.3} [{:.3}, {:.3}] {}", pt[unit][pri][wi][pi], lo, hi, verdict(lo, hi)) }).collect();
        let vs: Vec<&str> = (0..3).map(|wi| { let (lo, hi) = ci(unit, pri, wi, pi); verdict(lo, hi) }).collect();
        let same = vs.iter().all(|v| *v == vs[0]);
        let cn = mean(&cs[pi].iter().map(|c| c.nat).collect::<Vec<_>>());
        let m = ref_mean(pri, pi);
        tables += &format!("| {} | {} | {} | {} | {} | {} | {:.2} | {:.2} |\n", unit_name[unit], PRIMES[pi], cells[0], cells[1], cells[2], if same { format!("yes ({})", vs[0]) } else { "NO".into() }, cn, m[3]);
        band_json.push(json!({"unit": unit_name[unit], "p": PRIMES[pi], "verdicts": vs, "same_at_all_w": same,
            "ratio": (0..3).map(|wi| pt[unit][pri][wi][pi]).collect::<Vec<_>>(),
            "ci": (0..3).map(|wi| { let (a, b) = ci(unit, pri, wi, pi); json!([a, b]) }).collect::<Vec<_>>()}));
    } }
    tables += &format!("\nUnpriced counters listed (section 5a): reference `fp_adds` {:.1} per sqrt(l) (about {:.1} per step), emitted; cover F_p additions: NOT emitted (cover is a lower bound); cover residual-table and factor-base bytes: not operations, not charged (5a, 2026-11-01). Because the cover count is a lower bound, an \"above\" verdict at w = 0 can only widen under complete counts; a \"below\" or \"not resolved\" verdict could flip.\n", mean_b[4], mean_b[4] / mean_b[2]);
    say!("{}", tables);

    // crossover model
    tables += "\n### Extrapolated fair crossover p* (model)\n\nModel: S/rho(p) = G (p/p_i)^-2.10 through the three measured points (slope from the crossover study, not re-fitted here), p* where S/rho = 1. Interval: 2.5/97.5 percentile of the same bootstrap draws (no slope uncertainty). Slope sensitivity: -1.99 / -2.21 (the crossover study's 95% t interval).\n\n| unit | estimator | p* (model) [95% boot] | p* at slope -1.99 | p* at slope -2.21 | below the u64 limit p <= 2039? |\n|---|---|---|---|---|---|\n";
    let mut pstar_json = json!({});
    for unit in 0..2 { for est in 0..3 {
        let pts: Vec<(f64, f64)> = (0..3).map(|pi| (PRIMES[pi] as f64, pt[unit][est][0][pi])).collect();
        let ps = |slope: f64| -> f64 { let acc: f64 = pts.iter().map(|(p, g)| g.ln() - slope * p.ln()).sum::<f64>() / 3.0; (acc / (-slope)).exp() };
        let v = &pstar_bs[unit * 3 + est];
        let (lo, hi) = (pct(v, 0.025), pct(v, 0.975));
        tables += &format!("| {} | {} | {:.0} [{:.0}, {:.0}] | {:.0} | {:.0} | {} |\n", unit_name[unit], est_name[est], ps(SLOPE), lo, hi, ps(-1.99), ps(-2.21),
            if hi <= 2039.0 { "yes (whole interval)" } else if ps(SLOPE) <= 2039.0 { "point yes, interval reaches beyond" } else { "no: beyond the limit" });
        say!("p* {} {}: {:.0} [{:.0}, {:.0}] slope -1.99: {:.0}, -2.21: {:.0}", unit_name[unit], est_name[est], ps(SLOPE), lo, hi, ps(-1.99), ps(-2.21));
        pstar_json[format!("{}|{}", unit_name[unit], est_name[est])] = json!({"p_star": ps(SLOPE), "ci95": [lo, hi], "slope_-1.99": ps(-1.99), "slope_-2.21": ps(-2.21)});
    } }

    // ---------------- consistency controls ----------------
    let mut ctrl_json = json!({});
    let mut ctrl_pass = true;
    // (i) the predecessor study's rhostrong n = 6 numbers, reproduced with its own bootstrap procedure and RNG stream
    {
        let mut old_rho_dp: BTreeMap<u64, BTreeMap<u64, Vec<f64>>> = BTreeMap::new();
        for f in files_with(&inp.join("old_rhodp"), "rhodp_p") { for r in rows(&f) {
            if r["correct"].as_bool().unwrap() { old_rho_dp.entry(r["p"].as_u64().unwrap()).or_default().entry(r["seed"].as_u64().unwrap()).or_default().push(fl(&r, "s")); }
        } }
        let mut old_strong: BTreeMap<u64, BTreeMap<u64, Vec<f64>>> = BTreeMap::new();
        for r in &old { old_strong.entry(r.p).or_default().entry(r.seed).or_default().push(r.u1); }
        let boot_mean = |arm: &BTreeMap<u64, Vec<f64>>, rng: &mut Lcg| -> f64 {
            let (mut tot, mut n) = (0.0, 0.0);
            for v in arm.values() { for _ in 0..v.len() { tot += v[rng.idx(v.len())]; n += 1.0; } }
            tot / n
        };
        let mut lrng = Lcg(20261101);
        let expected: BTreeMap<u64, ([f64; 3], [f64; 3])> = [(1301, ([0.793, 0.587, 1.119], [1.349, 1.085, 1.699])), (1451, ([0.775, 0.645, 0.970], [1.063, 0.819, 1.477])), (1601, ([0.587, 0.448, 0.811], [0.910, 0.658, 1.339]))].into_iter().collect();
        for pi in 0..3 {
            let p = PRIMES[pi];
            let ss: Vec<f64> = cs[pi].iter().map(|c| c.s).collect();
            let gm = gmean(&ss);
            for (name, arm) in [("rhodp", &old_rho_dp[&p]), ("rhostrong", &old_strong[&p])] {
                let all: Vec<f64> = arm.values().flatten().cloned().collect();
                let point = gm / mean(&all);
                let mut b = vec![];
                for _ in 0..NB {
                    let g: Vec<f64> = (0..ss.len()).map(|_| ss[lrng.idx(ss.len())]).collect();
                    b.push(gmean(&g) / boot_mean(arm, &mut lrng));
                }
                let (lo, hi) = (pct(&b, 0.025), pct(&b, 0.975));
                let e = &expected[&p];
                let exp = if name == "rhodp" { e.0 } else { e.1 };
                let ok = (point - exp[0]).abs() < 0.0005 && (lo - exp[1]).abs() < 0.0005 && (hi - exp[2]).abs() < 0.0005;
                if name == "rhostrong" && !ok { ctrl_pass = false; }
                say!("CONTROL (i) original study S/{name} n=6/9 p={p}: tool {point:.3} [{lo:.3}, {hi:.3}] vs published {:.3} [{:.3}, {:.3}]: match to 3 decimals: {ok}", exp[0], exp[1], exp[2]);
                ctrl_json[format!("study_{name}_{p}")] = json!({"tool": [point, lo, hi], "published": exp, "match": ok});
            }
        }
    }
    // (ii) red team R4 / R5 / R6 / R7 (their post-hoc numbers)
    {
        let rt_exp: BTreeMap<&str, [[f64; 3]; 3]> = [
            ("R4 per-prime n=6 steps", [[1.349, 1.089, 1.699], [1.063, 0.809, 1.476], [0.909, 0.649, 1.339]]),
            ("R5 pooled 82 steps", [[1.480, 1.244, 1.759], [1.221, 1.042, 1.437], [0.916, 0.718, 1.239]]),
            ("R6 theory 0.8880", [[1.475, 1.267, 1.682], [1.217, 1.106, 1.398], [0.913, 0.749, 1.260]]),
            ("R7 pooled 82 tuned", [[3.540, 2.971, 4.193], [2.922, 2.518, 3.453], [2.192, 1.710, 2.977]]),
        ].into_iter().collect();
        let mut grp82: Groups = g_of(&mk(&old.iter().collect::<Vec<_>>()), None);
        grp82.extend(g_of(&inst251, None));
        let nsteps = |g: &Groups, k: usize| -> Vec<Vec<f64>> { g.iter().map(|x| x.iter().map(|v| v[k]).collect()).collect() };
        let mut crng = Rng(0xC0DE_0001);
        for (name, exp) in &rt_exp {
            for pi in 0..3 {
                let ss: Vec<f64> = cs[pi].iter().map(|c| c.s).collect();
                let gm = gmean(&ss);
                // reference groups for this control
                let (grp, fixed): (Vec<Vec<f64>>, Option<f64>) = match &name[..2] {
                    "R4" => (nsteps(&g_of(&mk(&old.iter().collect::<Vec<_>>()), Some(PRIMES[pi])), 2), None),
                    "R5" => (nsteps(&grp82, 2), None),
                    "R6" => (vec![], Some(theory)),
                    _ => (nsteps(&grp82, 1), None),
                };
                let rmean = fixed.unwrap_or_else(|| mean(&grp.iter().flatten().cloned().collect::<Vec<_>>()));
                let point = gm / rmean;
                let mut b = vec![];
                for _ in 0..NB {
                    let g: Vec<f64> = (0..3).map(|_| ss[crng.idx(3)]).collect();
                    let d = match fixed { Some(c) => c, None => { let (mut t, mut n) = (0.0, 0.0); for x in &grp { for _ in 0..x.len() { t += x[crng.idx(x.len())]; n += 1.0; } } t / n } };
                    b.push(gmean(&g) / d);
                }
                let (lo, hi) = (pct(&b, 0.025), pct(&b, 0.975));
                let e = exp[pi];
                let okp = (point - e[0]).abs() < 0.0005;
                let okc = (lo - e[1]).abs() < 0.01 && (hi - e[2]).abs() < 0.01;
                if !okp { ctrl_pass = false; }
                say!("CONTROL (ii) red team {name} p={}: tool {point:.3} [{lo:.3}, {hi:.3}] vs report {:.3} [{:.3}, {:.3}]: point match to 3 decimals {okp}; interval within 0.01 (own RNG stream) {okc}", PRIMES[pi], e[0], e[1], e[2]);
                ctrl_json[format!("{name}_{}", PRIMES[pi])] = json!({"tool": [point, lo, hi], "report": e, "point_match": okp, "interval_within_0.01": okc});
            }
        }
        let st: Vec<f64> = grp82.iter().flatten().map(|v| v[2]).collect();
        say!("CONTROL (ii) pooled 82 runs: steps/sqrt(l) {:.4} +- {:.4} (report: 0.885 +- 0.054)", mean(&st), sd(&st) / (st.len() as f64).sqrt());
    }
    say!("consistency controls (point estimates to 3 decimals): {}", if ctrl_pass { "PASSED" } else { "FAILED" });

    // ---------------- outputs ----------------
    let timings: Vec<String> = vec![];
    let _ = timings;
    let fit = json!({
        "frozen": {"nb": NB, "nperm": NPERM, "slope": SLOPE, "theory_steps_per_sqrt_l": theory},
        "runs": {"old": old.len(), "new_complete": newr.len(), "new_runs_per_instance": n_new_per_inst, "complete_run_indices": complete,
                 "incomplete_run_indices_excluded": excluded_rounds, "red_team_p251": rt.len(), "l_matches_cover": l_ok},
        "homogeneity": hom_json, "homogeneous": homogeneous, "primary_estimator": primary, "positive_control": pos_json, "positive_control_pass": pos_pass,
        "ratios": (0..2).map(|unit| (unit_name[unit].to_string(), (0..3).map(|est| (est_name[est].to_string(), (0..3).map(|pi| {
            let (lo, hi) = ci(unit, est, 0, pi);
            (PRIMES[pi].to_string(), json!({"ratio": pt[unit][est][0][pi], "ci95": [lo, hi], "verdict": verdict(lo, hi)}))
        }).collect::<BTreeMap<_, _>>())).collect::<BTreeMap<_, _>>())).collect::<BTreeMap<_, _>>(),
        "band_5a": band_json, "p_star": pstar_json, "controls": ctrl_json, "controls_pass": ctrl_pass, "failures": fails,
    });
    res_json["_"] = Value::Null;
    fs::write(out.join("fit.json"), serde_json::to_string_pretty(&fit).unwrap()).unwrap();
    fs::write(out.join("fit.txt"), &txt).unwrap();
    fs::write(out.join("table.md"), &tables).unwrap();
    // graph
    let pstar_primary = |unit: usize| -> (f64, f64, f64) {
        let pts: Vec<(f64, f64)> = (0..3).map(|pi| (PRIMES[pi] as f64, pt[unit][pri][0][pi])).collect();
        let acc: f64 = pts.iter().map(|(p, g)| g.ln() - SLOPE * p.ln()).sum::<f64>() / 3.0;
        let v = &pstar_bs[unit * 3 + pri];
        ((acc / (-SLOPE)).exp(), pct(v, 0.025), pct(v, 0.975))
    };
    let svg = scaling_svg(&pt, &|u, pi| ci(u, pri, 0, pi), pri, &pstar_primary, &est_name[pri]);
    fs::write(out.join("scaling.svg"), svg).unwrap();
    if !fails.is_empty() { println!("FAILURES / EXCLUSIONS: {fails:?}"); }
}

fn replay(oldp: &str, newp: &str) {
    let (a, b) = (rows(Path::new(oldp)), rows(Path::new(newp)));
    let mut ok = true;
    for rn in &b {
        let ro = a.iter().find(|r| r["p"] == rn["p"] && r["seed"] == rn["seed"] && r["run"] == rn["run"]);
        match ro {
            None => { println!("no old record for p={} seed={} run={}", rn["p"], rn["seed"], rn["run"]); ok = false; }
            Some(ro) => {
                let (mut x, mut y) = (ro.clone(), rn.clone());
                x.as_object_mut().unwrap().remove("wall_ms");
                y.as_object_mut().unwrap().remove("wall_ms");
                let same = x == y;
                println!("REPLAY p={} seed={} run={}: bit-identical (all fields except wall_ms): {same}; steps old {} new {}; wall old {:.0} s new {:.0} s",
                    rn["p"], rn["seed"], rn["run"], ro["steps"], rn["steps"], fl(ro, "wall_ms") / 1e3, fl(rn, "wall_ms") / 1e3);
                if !same {
                    ok = false;
                    for (k, v) in x.as_object().unwrap() { if y[k] != *v { println!("   differs: {k}: old {v} new {}", y[k]); } }
                }
            }
        }
    }
    println!("REPLAY RESULT: {}", if ok { "IDENTICAL" } else { "NOT IDENTICAL" });
}

fn scaling_svg(pt: &[[[[f64; 3]; 3]; 3]; 2], ci: &dyn Fn(usize, usize) -> (f64, f64), pri: usize, ps: &dyn Fn(usize) -> (f64, f64, f64), est: &str) -> String {
    let (w, h) = (980.0, 620.0);
    let (ml, mr, mt, mb) = (80.0, 30.0, 80.0, 70.0);
    let (x0, x1) = (1200f64.ln(), 2800f64.ln());
    let (y0, y1) = (0.4f64.ln(), 8f64.ln());
    let px = |p: f64| ml + (p.ln() - x0) / (x1 - x0) * (w - ml - mr);
    let py = |v: f64| h - mb - (v.ln() - y0) / (y1 - y0) * (h - mt - mb);
    let cols = ["#1f6fb5", "#d9730d"];
    let names = ["U1: group additions (cover S / rho steps per sqrt l)", "U2: textbook F_p multiplications (cover / rho tuned count)"];
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"Helvetica, Arial, sans-serif\" font-size=\"13\">\n<rect width=\"{w}\" height=\"{h}\" fill=\"#ffffff\"/>\n");
    s += &format!("<text x=\"{ml}\" y=\"26\" font-size=\"17\" font-weight=\"bold\" fill=\"#222\">JV cover vs negation-map rho, same instances</text>\n<text x=\"{ml}\" y=\"46\" fill=\"#444\">S/rho (&gt;1: cover costs more than rho). Points and 95% bootstrap bars: derived from measured runs, reference {}.</text>\n<text x=\"{ml}\" y=\"62\" fill=\"#444\">Dashed lines and p*: model, slope -2.10 from the crossover study.</text>\n", est);
    for v in [0.5, 1.0, 2.0, 4.0, 8.0] {
        s += &format!("<line x1=\"{ml}\" x2=\"{}\" y1=\"{y}\" y2=\"{y}\" stroke=\"#e3e3e3\"/><text x=\"{}\" y=\"{}\" text-anchor=\"end\" fill=\"#444\">{v}</text>\n", w - mr, ml - 8.0, py(v) + 4.0, y = py(v));
    }
    for p in [1301.0, 1451.0, 1601.0, 2039.0, 2400.0] {
        s += &format!("<line x1=\"{x}\" x2=\"{x}\" y1=\"{}\" y2=\"{}\" stroke=\"#eee\"/><text x=\"{x}\" y=\"{}\" text-anchor=\"middle\" fill=\"#444\">{p}</text>\n", mt, h - mb, h - mb + 20.0, x = px(p));
    }
    s += &format!("<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#222\">prime p (log scale)</text>\n", (ml + w - mr) / 2.0, h - 28.0);
    s += &format!("<text transform=\"translate(22,{}) rotate(-90)\" text-anchor=\"middle\" fill=\"#222\">S / rho (log scale)</text>\n", (mt + h - mb) / 2.0);
    s += &format!("<line x1=\"{ml}\" x2=\"{}\" y1=\"{y}\" y2=\"{y}\" stroke=\"#000\" stroke-width=\"1.6\"/><text x=\"{}\" y=\"{}\" fill=\"#000\" text-anchor=\"end\">S/rho = 1 (parity)</text>\n", w - mr, w - mr - 4.0, py(1.0) - 6.0, y = py(1.0));
    s += &format!("<line x1=\"{x}\" x2=\"{x}\" y1=\"{}\" y2=\"{}\" stroke=\"#8a1c1c\" stroke-dasharray=\"2 4\" stroke-width=\"1.6\"/><text x=\"{}\" y=\"{}\" fill=\"#8a1c1c\">u64 limit p &lt;= 2039</text>\n", mt, h - mb, px(2039.0) + 5.0, mt + 14.0, x = px(2039.0));
    for unit in 0..2 {
        let pts: Vec<(f64, f64)> = (0..3).map(|pi| ([1301.0, 1451.0, 1601.0][pi], pt[unit][pri][0][pi])).collect();
        let (pstar, plo, phi) = ps(unit);
        let acc: f64 = pts.iter().map(|(p, g)| g.ln() + 2.10 * p.ln()).sum::<f64>() / 3.0;
        let f = |p: f64| (acc - 2.10 * p.ln()).exp();
        s += &format!("<polyline fill=\"none\" stroke=\"{}\" stroke-width=\"1.6\" stroke-dasharray=\"7 5\" points=\"{},{} {},{}\"/>\n", cols[unit], px(1250.0), py(f(1250.0)), px(2750.0), py(f(2750.0)));
        s += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"6\" fill=\"#fff\" stroke=\"{}\" stroke-width=\"2.2\"/>\n", px(pstar), py(1.0), cols[unit]);
        s += &format!("<line x1=\"{}\" x2=\"{}\" y1=\"{y}\" y2=\"{y}\" stroke=\"{}\" stroke-width=\"3\" opacity=\"0.55\"/>\n", px(plo.min(2790.0).max(1210.0)), px(phi.min(2790.0).max(1210.0)), cols[unit], y = py(1.0));
        s += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" text-anchor=\"middle\">p* ~ {:.0} (model)</text>\n", px(pstar), if unit == 0 { py(1.0) - 12.0 } else { py(1.0) + 22.0 }, cols[unit], pstar);
        for pi in 0..3 {
            let (lo, hi) = ci(unit, pi);
            let x = px([1301.0, 1451.0, 1601.0][pi]) + (unit as f64 - 0.5) * 8.0;
            s += &format!("<line x1=\"{x}\" x2=\"{x}\" y1=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\"/><line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\"/><line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\"/>\n",
                py(lo), py(hi), cols[unit], x - 4.0, x + 4.0, py(lo), py(lo), cols[unit], x - 4.0, x + 4.0, py(hi), py(hi), cols[unit]);
            s += &format!("<circle cx=\"{x}\" cy=\"{}\" r=\"5\" fill=\"{}\"/>\n", py(pt[unit][pri][0][pi]), cols[unit]);
        }
        let ly = mt + 36.0 + 20.0 * unit as f64;
        s += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{}\"/><text x=\"{}\" y=\"{}\" fill=\"#222\">{}</text>\n", ml + 14.0, ly, cols[unit], ml + 28.0, ly + 4.0, names[unit]);
    }
    s += &format!("<text x=\"{}\" y=\"{}\" fill=\"#444\">open circle on the parity line: model crossover p*, bar = 95% bootstrap of p* (no slope uncertainty)</text>\n", ml + 14.0, mt + 90.0);
    s += "</svg>\n";
    s
}
