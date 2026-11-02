//! Analysis for research/jv_sieve_negrho_20261102 (frozen in PROTOCOL.md). Every number is computed here, in Rust.
//! Usage:  sieve_negrho <study dir> <#3278 results dir>   full analysis (writes fit.txt, fit.json, table.md, graph.svg in the study dir)
//!         sieve_negrho selftest <#3278 results dir>       pooled statistics of the 126 reference runs only
use serde_json::{json, Value};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const NB: usize = 20000; // bootstrap resamples (frozen)
const NPERM: usize = 20000; // permutation resamples (frozen)
const GAE: f64 = 331.0;
const WS: [f64; 3] = [0.0, 0.1, 1.0];

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
fn mean(v: &[f64]) -> f64 { v.iter().sum::<f64>() / v.len() as f64 }
fn sd(v: &[f64]) -> f64 {
    if v.len() < 2 { return f64::NAN; }
    let m = mean(v);
    (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
}
fn pct(v: &[f64], q: f64) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    s[((s.len() - 1) as f64 * q).round() as usize]
}
fn verdict(lo: f64, hi: f64) -> &'static str {
    if hi < 1.0 { "below" } else if lo > 1.0 { "above" } else { "not resolved" }
}
fn theory() -> f64 { (std::f64::consts::PI / 4.0).sqrt() / (1.0 - 1.0 / 256.0f64).sqrt() }

// ---------- two-sample statistics ----------
fn absdiff(a: &[f64], b: &[f64]) -> f64 { (mean(a) - mean(b)).abs() }
fn ks(a: &[f64], b: &[f64]) -> f64 {
    let mut x = a.to_vec();
    let mut y = b.to_vec();
    x.sort_by(|p, q| p.partial_cmp(q).unwrap());
    y.sort_by(|p, q| p.partial_cmp(q).unwrap());
    let (n, m) = (x.len() as f64, y.len() as f64);
    let (mut i, mut j, mut d) = (0usize, 0usize, 0.0f64);
    while i < x.len() && j < y.len() {
        let v = x[i].min(y[j]);
        while i < x.len() && x[i] <= v { i += 1; }
        while j < y.len() && y[j] <= v { j += 1; }
        d = d.max((i as f64 / n - j as f64 / m).abs());
    }
    d
}
fn kw_h(a: &[f64], b: &[f64]) -> f64 {
    let mut all: Vec<(f64, usize)> = a.iter().map(|v| (*v, 0)).chain(b.iter().map(|v| (*v, 1))).collect();
    all.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap());
    let n = all.len();
    let mut rank = vec![0.0; n];
    let (mut i, mut tie) = (0, 0.0);
    while i < n {
        let mut j = i;
        while j + 1 < n && all[j + 1].0 == all[i].0 { j += 1; }
        let r = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j { rank[k] = r; }
        let t = (j - i + 1) as f64;
        tie += t * t * t - t;
        i = j + 1;
    }
    let mut rs = [0.0; 2];
    for (k, x) in all.iter().enumerate() { rs[x.1] += rank[k]; }
    let nf = n as f64;
    let h = 12.0 / (nf * (nf + 1.0)) * (rs[0] * rs[0] / a.len() as f64 + rs[1] * rs[1] / b.len() as f64) - 3.0 * (nf + 1.0);
    h / (1.0 - tie / (nf * nf * nf - nf))
}
/// (observed, permutation p) for a two-sample statistic; group sizes kept
fn perm(a: &[f64], b: &[f64], f: fn(&[f64], &[f64]) -> f64, rng: &mut Rng) -> (f64, f64) {
    let t0 = f(a, b);
    let mut pool: Vec<f64> = a.iter().chain(b.iter()).cloned().collect();
    let mut c = 0usize;
    for _ in 0..NPERM {
        for i in (1..pool.len()).rev() { let j = rng.idx(i + 1); pool.swap(i, j); }
        if f(&pool[..a.len()], &pool[a.len()..]) >= t0 - 1e-12 { c += 1; }
    }
    (t0, (1 + c) as f64 / (1 + NPERM) as f64)
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

#[derive(Clone, Debug)]
struct Run { p: u64, seed: u64, run: u64, l: f64, c_e: f64, steps_n: f64, u1: f64, u2: f64, nat: f64, fpa: f64 }
impl Run { fn v(&self) -> [f64; 5] { [self.u1, self.u2, self.steps_n, self.nat, self.fpa] } }
fn load_runs(dir: &Path, fails: &mut Vec<String>) -> Vec<Run> {
    let mut out = vec![];
    for f in files_with(dir, "rhostrong") {
        for r in rows(&f) {
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            let (p, seed, run) = (r["p"].as_u64().unwrap(), r["seed"].as_u64().unwrap(), r["run"].as_u64().unwrap());
            if !r["correct"].as_bool().unwrap() || r["capped"].as_bool().unwrap() || !r["negation"].as_bool().unwrap()
                || r["batch"].as_u64().unwrap() != 16 || r["r"].as_u64().unwrap() != 256 || r["dp_bits"].as_u64().unwrap() != 16 {
                fails.push(format!("{name}: p={p} seed={seed} run={run} excluded (incorrect, capped or wrong settings)"));
                continue;
            }
            let l = fl(&r, "l");
            let sl = l.sqrt();
            out.push(Run { p, seed, run, l, c_e: fl(&r, "c_add_e"), steps_n: fl(&r, "steps") / sl, u1: fl(&r, "s_harness"),
                u2: fl(&r, "s_tuned"), nat: nat_of(&r) / sl, fpa: fl(&r["ops"], "fp_adds") / sl });
        }
    }
    out
}

#[derive(Clone)]
struct Cov { seed: u64, l: f64, c_e: f64, s: f64, sp: f64, nat_a: f64, nat_b: f64, ok: bool }
fn cov_row(r: &Value) -> Cov {
    let l = fl(r, "l");
    let sl = l.sqrt();
    let tm = fl(r, "total_muls");
    let unemitted = tm - fl(r, "sieve_muls") - fl(r, "enum_muls");
    Cov { seed: r["seed"].as_u64().unwrap(), l, c_e: fl(r, "c_add_e"), s: tm / (GAE * sl), sp: fl(r, "total_plus") / (GAE * sl),
        nat_a: (fl(r, "sieve_adds") + fl(r, "sieve_lookups") + fl(r, "enum_adds") + unemitted) / sl,
        nat_b: fl(r, "sieve_lookups") / sl,
        ok: r["solved"].as_bool().unwrap() && r["correct"].as_bool().unwrap() && r["p"].as_u64().unwrap() == 1511 && r["lin_mode"].as_str() == Some("sparse") }
}

// ---------- cells ----------
fn gm(idx: &[usize], f: &dyn Fn(usize) -> f64) -> f64 { (idx.iter().map(|&i| f(i).ln()).sum::<f64>() / idx.len() as f64).exp() }
fn am(idx: &[usize], f: &dyn Fn(usize) -> f64) -> f64 { idx.iter().map(|&i| f(i)).sum::<f64>() / idx.len() as f64 }
fn labels() -> Vec<String> {
    let mut l: Vec<String> = ["S/rho U1", "S/rho U2", "S+/rho U1 (rho unpriced)", "S+/rho U2 (rho unpriced)", "S+/rhoS+ U1", "S+/rhoS+ U2",
        "S/rho U1 (ratio of means)", "S/rho U2 (ratio of means)"].iter().map(|s| s.to_string()).collect();
    for ly in ["A", "B"] { for u in ["U1", "U2"] { for w in WS { l.push(format!("band {ly} {u} w={w}")); } } }
    l
}
fn cells(cov: &[Cov], idx: &[usize], r: &[f64; 5]) -> Vec<f64> {
    let s = |i: usize| cov[i].s;
    let sp = |i: usize| cov[i].sp;
    let rsp = (r[3] + r[4]) / GAE;
    let mut o = vec![gm(idx, &s) / r[0], gm(idx, &s) / r[1], gm(idx, &sp) / r[0], gm(idx, &sp) / r[1],
        gm(idx, &sp) / (r[0] + rsp), gm(idx, &sp) / (r[1] + rsp), am(idx, &s) / r[0], am(idx, &s) / r[1]];
    for ly in 0..2 {
        for u in 0..2 {
            for w in WS {
                let nc = |i: usize| if ly == 0 { cov[i].nat_a } else { cov[i].nat_b };
                let nr = if ly == 0 { r[3] + r[4] } else { r[3] };
                o.push(gm(idx, &|i| cov[i].s + w * nc(i)) / (r[u] + w * nr));
            }
        }
    }
    o
}
type Groups = Vec<Vec<[f64; 5]>>;
fn gmeanvec(g: &Groups) -> [f64; 5] {
    let (mut s, mut n) = ([0.0; 5], 0.0);
    for x in g { for v in x { for k in 0..5 { s[k] += v[k]; } n += 1.0; } }
    for k in 0..5 { s[k] /= n; }
    s
}
fn resample_mean(g: &Groups, rng: &mut Rng) -> [f64; 5] {
    let (mut s, mut n) = ([0.0; 5], 0.0);
    for x in g { for _ in 0..x.len() { let v = &x[rng.idx(x.len())]; for k in 0..5 { s[k] += v[k]; } n += 1.0; } }
    for k in 0..5 { s[k] /= n; }
    s
}
fn groups_of(runs: &[Run]) -> Groups {
    let mut keys: Vec<(u64, u64)> = runs.iter().map(|r| (r.p, r.seed)).collect();
    keys.sort();
    keys.dedup();
    keys.iter().map(|k| runs.iter().filter(|r| (r.p, r.seed) == *k).map(|r| r.v()).collect()).collect()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() >= 3 && a[1] == "selftest" {
        let mut f = vec![];
        let pr = pooled(Path::new(&a[2]), &mut f);
        let sn: Vec<f64> = pr.iter().map(|r| r.steps_n).collect();
        println!("pooled #3278 runs n={} (excluded {}): steps/sqrt(l) mean {:.4} sd {:.4}; s_harness mean {:.4}; s_tuned mean {:.4}; theory {:.5}; z {:.3}",
            pr.len(), f.len(), mean(&sn), sd(&sn), mean(&pr.iter().map(|r| r.u1).collect::<Vec<_>>()), mean(&pr.iter().map(|r| r.u2).collect::<Vec<_>>()),
            theory(), (mean(&sn) - theory()) / (sd(&sn) / (sn.len() as f64).sqrt()));
        return;
    }
    if a.len() >= 5 && a[1] == "replay" { replay(&a[2], &a[3]); return; }
    let study = PathBuf::from(&a[1]);
    let mut out = String::new();
    let mut fails: Vec<String> = vec![];
    let mut rng = Rng(0x5EED_1511_0211_2026);

    // ---- cover rows
    let all_cov: Vec<Cov> = rows(&study.join("results/inputs/ic_1511_sparse.json")).iter().map(cov_row).collect();
    let _ = writeln!(out, "# Inputs\nsieve rows {} (seeds {:?}), all solved+correct+sparse+p=1511: {}", all_cov.len(), all_cov.iter().map(|c| c.seed).collect::<Vec<_>>(), all_cov.iter().all(|c| c.ok));
    if !all_cov.iter().all(|c| c.ok) { fails.push("a sieve row is not solved/correct/sparse/p=1511".into()); }

    // ---- new runs, with the frozen cut rule
    let new_all = load_runs(&study.join("results/runs"), &mut fails);
    let have = |run: u64| (17..=24).all(|s| new_all.iter().any(|r| r.seed == s && r.run == run));
    let use_runs: Vec<u64> = if have(0) && have(1) { vec![0, 1] } else if have(0) { vec![0] } else { vec![] };
    let _ = writeln!(out, "new runs loaded {} ; complete rounds used: run indices {:?}", new_all.len(), use_runs);
    if use_runs.len() < 2 { fails.push(format!("CUT RULE applied: only run indices {:?} used", use_runs)); }
    let newr: Vec<Run> = new_all.iter().filter(|r| use_runs.contains(&r.run)).cloned().collect();
    if newr.is_empty() { println!("no complete round"); return; }
    // identity check
    let mut ident = true;
    for r in &newr {
        let c = all_cov.iter().find(|c| c.seed == r.seed).unwrap();
        if r.l != c.l || r.c_e != c.c_e { ident = false; fails.push(format!("identity: seed {} run {} l/c_add_e differ from V2", r.seed, r.run)); }
    }
    let _ = writeln!(out, "identity (l and c_add_e equal V2's sieve row, every new run): {ident}");

    // ---- replay control
    let rp_old = study.join("results/replay/old_p1451_s1_run0.json");
    let rp_new = study.join("results/replay/new_p1451_s1_run0.json");
    let mut replay_ok = false;
    if rp_old.exists() && rp_new.exists() {
        let (x, y) = (rows(&rp_old), rows(&rp_new));
        let strip = |v: &Value| { let mut c = v.clone(); c.as_object_mut().unwrap().remove("wall_ms"); c };
        replay_ok = x.len() == 1 && y.len() == 1 && strip(&x[0]) == strip(&y[0]);
        let _ = writeln!(out, "replay control p=1451 seed=1 run=0 (frozen binary vs #3278 record, all fields but wall_ms): bit-identical = {replay_ok}");
    } else { let _ = writeln!(out, "replay control: files missing"); }

    // ---- pooled reference
    let pooled_runs = pooled(Path::new(&a[2]), &mut fails);
    let _ = writeln!(out, "pooled #3278 runs: {}", pooled_runs.len());

    // ---- tests
    let m_sn: Vec<f64> = newr.iter().map(|r| r.steps_n).collect();
    let p_sn: Vec<f64> = pooled_runs.iter().map(|r| r.steps_n).collect();
    let th = theory();
    let _ = writeln!(out, "\n# Test 2 (positive control): matched mean vs theory {th:.5}");
    let z_m = (mean(&m_sn) - th) / (sd(&m_sn) / (m_sn.len() as f64).sqrt());
    let ctrl2 = z_m.abs() <= 3.0;
    let _ = writeln!(out, "matched n={} steps/sqrt(l) mean {:.4} sd {:.4} se {:.4}; z = {:.3}; PASS = {ctrl2}", m_sn.len(), mean(&m_sn), sd(&m_sn), sd(&m_sn) / (m_sn.len() as f64).sqrt(), z_m);
    let z_p = (mean(&p_sn) - th) / (sd(&p_sn) / (p_sn.len() as f64).sqrt());
    let _ = writeln!(out, "#3278 pooled n={} mean {:.4} sd {:.4}; z = {:.3} (information)", p_sn.len(), mean(&p_sn), sd(&p_sn), z_p);
    let _ = writeln!(out, "\n# Test 1 (homogeneity, matched vs #3278 pooled)");
    let mut hom = json!({});
    let mut homogeneous = true;
    for (nm, fm, fp) in [("steps/sqrt(l)", 2usize, true), ("s_harness", 0, false), ("s_tuned", 1, false)] {
        let x: Vec<f64> = newr.iter().map(|r| r.v()[fm]).collect();
        let y: Vec<f64> = pooled_runs.iter().map(|r| r.v()[fm]).collect();
        let (d, pd) = perm(&x, &y, absdiff, &mut rng);
        let (k, pk) = perm(&x, &y, ks, &mut rng);
        let (h, ph) = perm(&x, &y, kw_h, &mut rng);
        let _ = writeln!(out, "{nm}{}: means {:.4} vs {:.4}; |diff| {:.4} perm p {:.4}; KS D {:.4} perm p {:.4}; KW H {:.4} perm p {:.4}",
            if fp { " [FROZEN DECISION METRIC]" } else { " (information)" }, mean(&x), mean(&y), d, pd, k, pk, h, ph);
        hom[nm] = json!({"mean_new": mean(&x), "mean_pooled": mean(&y), "absdiff": d, "p_absdiff": pd, "ks": k, "p_ks": pk, "kw": h, "p_kw": ph});
        if fp { homogeneous = pd >= 0.05 && pk >= 0.05; }
    }
    let primary_pooled = homogeneous && replay_ok;
    let _ = writeln!(out, "HOMOGENEOUS (both permutation p >= 0.05): {homogeneous}; replay ok: {replay_ok} -> PRIMARY REFERENCE = {}", if primary_pooled { "pooled (#3278 126 + new)" } else { "the matched new runs" });

    // ---- reference groups
    let g_m = groups_of(&newr);
    let mut all_ref = pooled_runs.clone();
    all_ref.extend(newr.iter().cloned());
    let g_p = groups_of(&all_ref);
    let r_m = gmeanvec(&g_m);
    let r_p = gmeanvec(&g_p);
    let _ = writeln!(out, "\nreference means [s_harness, s_tuned, steps/sqrt(l), native/sqrt(l), fp_adds/sqrt(l)]\n  matched ({} runs): {:?}\n  pooled  ({} runs): {:?}", newr.len(), r_m, all_ref.len(), r_p);

    // ---- bootstrap
    let covsets: Vec<(&str, Vec<usize>)> = vec![("seeds 17-24", (0..8).collect()), ("all 16 seeds", (0..all_cov.len()).collect())];
    for (_, s) in &covsets { for &i in s { assert!(all_cov[i].seed == 17 + i as u64); } }
    let labs = labels();
    let nc = labs.len();
    // series[cset][rset][cell]
    let mut series: Vec<Vec<Vec<Vec<f64>>>> = vec![vec![vec![Vec::with_capacity(NB); nc]; 2]; 2];
    for _ in 0..NB {
        let rm = [resample_mean(&g_m, &mut rng), resample_mean(&g_p, &mut rng)];
        for (ci, (_, set)) in covsets.iter().enumerate() {
            let idx: Vec<usize> = (0..set.len()).map(|_| set[rng.idx(set.len())]).collect();
            for rs in 0..2 {
                let v = cells(&all_cov, &idx, &rm[rs]);
                for (k, x) in v.into_iter().enumerate() { series[ci][rs][k].push(x); }
            }
        }
    }
    let refs = [("matched", r_m), ("pooled", r_p)];
    let prim = if primary_pooled { 1 } else { 0 };
    let mut j = json!({});
    let mut table = String::new();
    let _ = writeln!(table, "# Tables (derived; Rust `fit/`)\n\nPrimary reference: **{}** ({} runs). ", refs[prim].0, if prim == 1 { all_ref.len() } else { newr.len() });
    for (ci, (cn, set)) in covsets.iter().enumerate() {
        for rs in 0..2 {
            let idx: Vec<usize> = set.clone();
            let pt = cells(&all_cov, &idx, &refs[rs].1);
            let _ = writeln!(out, "\n## cover {cn} (n={}), reference {} {}", set.len(), refs[rs].0, if rs == prim { "[PRIMARY]" } else { "" });
            for k in 0..nc {
                let lo = pct(&series[ci][rs][k], 0.025);
                let hi = pct(&series[ci][rs][k], 0.975);
                let _ = writeln!(out, "  {:34} {:.5}  [{:.5}, {:.5}]  {}  (distance below rho: x{:.1} [{:.1}, {:.1}])", labs[k], pt[k], lo, hi, verdict(lo, hi), 1.0 / pt[k], 1.0 / hi, 1.0 / lo);
                j[format!("{cn}|{}|{}", refs[rs].0, labs[k])] = json!({"point": pt[k], "lo": lo, "hi": hi, "verdict": verdict(lo, hi)});
            }
            if rs == prim {
                let _ = writeln!(table, "\n## Cover {cn} (n={}), primary reference ({})\n\n| quantity | point | 95% interval | verdict | how far below rho (x) |\n|:--|--:|--:|:--|--:|", set.len(), refs[rs].0);
                for k in 0..8 {
                    let lo = pct(&series[ci][rs][k], 0.025);
                    let hi = pct(&series[ci][rs][k], 0.975);
                    let _ = writeln!(table, "| {} | {:.5} | [{:.5}, {:.5}] | {} | {:.1} [{:.1}, {:.1}] |", labs[k], pt[k], lo, hi, verdict(lo, hi), 1.0 / pt[k], 1.0 / hi, 1.0 / lo);
                }
                let _ = writeln!(table, "\n### Section 5a band (sieve/rho; native priced at w GAE per operation on both arms)\n\n| layer | unit | w=0 | w=0.1 | w=1 | order holds |\n|:--|:--|--:|--:|--:|:--|");
                for ly in 0..2 { for u in 0..2 {
                    let mut cellsr = vec![]; let mut vs = vec![];
                    for (wi, _w) in WS.iter().enumerate() {
                        let k = 8 + ly * 6 + u * 3 + wi;
                        let lo = pct(&series[ci][rs][k], 0.025); let hi = pct(&series[ci][rs][k], 0.975);
                        cellsr.push(format!("{:.4} [{:.4}, {:.4}]", pt[k], lo, hi)); vs.push(verdict(lo, hi));
                    }
                    let _ = writeln!(table, "| {} | U{} | {} | {} | {} | {} ({}) |", if ly == 0 { "A: V2 upper bound (cover adds+lookups+enum adds+unemitted; rho native+fp_adds)" } else { "B: as emitted (cover lookups; rho 6 native counters)" }, u + 1, cellsr[0], cellsr[1], cellsr[2], if vs[0] == vs[1] && vs[1] == vs[2] { "yes" } else { "no" }, vs[0]);
                } }
            }
        }
    }
    // ---- run table
    let _ = writeln!(table, "\n## New reference runs (measured)\n\n| seed | run | steps/sqrt(l) | s_harness | s_tuned |\n|--:|--:|--:|--:|--:|");
    let mut nr = newr.clone();
    nr.sort_by_key(|r| (r.seed, r.run));
    for r in &nr { let _ = writeln!(table, "| {} | {} | {:.4} | {:.4} | {:.4} |", r.seed, r.run, r.steps_n, r.u1, r.u2); }
    let _ = writeln!(table, "\n## Sieve rows (V2 sparse, measured)\n\n| seed | S | S+ | band-A native/sqrt(l) | lookups/sqrt(l) |\n|--:|--:|--:|--:|--:|");
    for c in &all_cov { let _ = writeln!(table, "| {} | {:.5} | {:.5} | {:.4e} | {:.4e} |", c.seed, c.s, c.sp, c.nat_a, c.nat_b); }

    let _ = writeln!(out, "\n# Failures / flags\n{}", if fails.is_empty() { "none".to_string() } else { fails.join("\n") });
    j["homogeneity"] = hom;
    j["controls"] = json!({"replay_bit_identical": replay_ok, "identity": ident, "z_matched": z_m, "positive_control_pass": ctrl2, "theory": th, "homogeneous": homogeneous, "primary": refs[prim].0, "flags": fails});
    j["reference_means"] = json!({"matched": r_m, "pooled": r_p, "fields": ["s_harness", "s_tuned", "steps_per_sqrt_l", "native_per_sqrt_l", "fp_adds_per_sqrt_l"]});
    fs::write(study.join("fit.txt"), &out).unwrap();
    fs::write(study.join("fit.json"), serde_json::to_string_pretty(&j).unwrap()).unwrap();
    fs::write(study.join("table.md"), &table).unwrap();

    // ---- graph
    let cell_ci = |ci: usize, k: usize| (pct(&series[ci][prim][k], 0.025), pct(&series[ci][prim][k], 0.975));
    let pts: Vec<Vec<f64>> = covsets.iter().map(|(_, s)| cells(&all_cov, s, &refs[prim].1)).collect();
    let svg = graph(&pts, &cell_ci, &newr, &pooled_runs, th, refs[prim].0);
    fs::write(study.join("graph.svg"), svg).unwrap();
    print!("{out}");
}

fn pooled(dir: &Path, fails: &mut Vec<String>) -> Vec<Run> {
    let mut v = load_runs(&dir.join("inputs/old_rhostrong"), fails);
    v.extend(load_runs(&dir.join("new"), fails));
    v
}

fn replay(oldp: &str, newp: &str) {
    let (x, y) = (rows(Path::new(oldp)), rows(Path::new(newp)));
    let strip = |v: &Value| { let mut c = v.clone(); c.as_object_mut().unwrap().remove("wall_ms"); c };
    println!("bit-identical except wall_ms: {}", x.len() == y.len() && x.iter().zip(&y).all(|(p, q)| strip(p) == strip(q)));
}

fn graph(pts: &[Vec<f64>], ci: &dyn Fn(usize, usize) -> (f64, f64), newr: &[Run], pooled: &[Run], th: f64, refname: &str) -> String {
    let (w, h) = (1000.0, 760.0);
    let mut s = String::new();
    let _ = write!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"DejaVu Sans, Arial, sans-serif\" font-size=\"13\"><rect width=\"{w}\" height=\"{h}\" fill=\"#ffffff\"/>");
    let _ = write!(s, "<text x=\"20\" y=\"28\" font-size=\"17\" font-weight=\"bold\">Sieve route vs matched negation-map rho at p = 1511 (primary reference: {refname})</text>");
    let _ = write!(s, "<text x=\"20\" y=\"48\" fill=\"#444\">Panel A: sieve / rho (log scale, 1 = parity with rho; points are far below 1). Panel B: steps/sqrt(l) of the reference runs.</text>");
    // panel A
    let (x0, x1) = (250.0, 960.0);
    let (lo, hi) = (1e-3f64.ln(), 2.0f64.ln());
    let xp = |v: f64| x0 + (v.ln() - lo) / (hi - lo) * (x1 - x0);
    let rowsd: [(&str, usize); 4] = [("S, U1 (group additions)", 0), ("S, U2 (textbook F_p mults)", 1), ("S+, U1 (adds+lookups, rho unpriced)", 2), ("S+, U2 (adds+lookups, rho unpriced)", 3)];
    let ytop = 80.0;
    for t in [1e-3, 1e-2, 1e-1, 1.0] {
        let x = xp(t);
        let _ = write!(s, "<line x1=\"{x}\" y1=\"{ytop}\" x2=\"{x}\" y2=\"430\" stroke=\"{}\" stroke-width=\"{}\"/><text x=\"{}\" y=\"448\" text-anchor=\"middle\">{}</text>", if t == 1.0 { "#222" } else { "#ddd" }, if t == 1.0 { 2 } else { 1 }, x, if t == 1.0 { "1 (rho)".to_string() } else { format!("{t}") });
    }
    let _ = write!(s, "<text x=\"{}\" y=\"468\" text-anchor=\"middle\">S / rho (95% bootstrap over cover seeds and reference runs)</text>", (x0 + x1) / 2.0);
    let cols = ["#0072B2", "#D55E00"];
    for (ri, (nm, k)) in rowsd.iter().enumerate() {
        let yb = ytop + 20.0 + ri as f64 * 86.0;
        let _ = write!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{nm}</text>", x0 - 12.0, yb + 28.0);
        for (c, nmc) in [(1usize, "16 seeds"), (0usize, "seeds 17-24")] {
            let y = yb + if c == 1 { 14.0 } else { 44.0 };
            let (l, u) = ci(c, *k);
            let p = pts[c][*k];
            let _ = write!(s, "<line x1=\"{}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"{}\" stroke-width=\"3\"/><circle cx=\"{}\" cy=\"{y}\" r=\"6\" fill=\"{}\"/><text x=\"{}\" y=\"{}\" fill=\"{}\">{:.4} [{:.4}, {:.4}] ({nmc})</text>", xp(l), xp(u), cols[c], xp(p), cols[c], xp(p) + 4.0, y - 9.0, cols[c], p, l, u);
        }
    }
    // panel B
    let (bx0, bx1) = (250.0, 960.0);
    let (vlo, vhi) = (0.0f64, 3.2f64);
    let xv = |v: f64| bx0 + (v - vlo) / (vhi - vlo) * (bx1 - bx0);
    let _ = write!(s, "<text x=\"20\" y=\"500\" font-weight=\"bold\">Panel B: reference runs, steps/sqrt(l)</text>");
    for t in [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0] {
        let _ = write!(s, "<line x1=\"{x}\" y1=\"520\" x2=\"{x}\" y2=\"700\" stroke=\"#ddd\"/><text x=\"{x}\" y=\"718\" text-anchor=\"middle\">{t}</text>", x = xv(t));
    }
    let _ = write!(s, "<line x1=\"{x}\" y1=\"520\" x2=\"{x}\" y2=\"700\" stroke=\"#009E73\" stroke-width=\"2\" stroke-dasharray=\"6 4\"/><text x=\"{x}\" y=\"514\" text-anchor=\"middle\" fill=\"#009E73\">theory {th:.3}</text>", x = xv(th));
    let mut jit = Rng(7);
    for (gi, (nm, runs)) in [("#3278 pooled (p 1301-1601)", pooled), ("new matched (p 1511)", newr)].iter().enumerate() {
        let yc = 570.0 + gi as f64 * 90.0;
        let _ = write!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{nm} n={}</text>", bx0 - 12.0, yc + 4.0, runs.len());
        for r in runs.iter() {
            let y = yc - 25.0 + (jit.next() % 5000) as f64 / 5000.0 * 50.0;
            let _ = write!(s, "<circle cx=\"{}\" cy=\"{y}\" r=\"3.5\" fill=\"{}\" fill-opacity=\"0.6\"/>", xv(r.steps_n), cols[gi]);
        }
        let m = mean(&runs.iter().map(|r| r.steps_n).collect::<Vec<_>>());
        let _ = write!(s, "<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"#000\" stroke-width=\"3\"/><text x=\"{}\" y=\"{}\">mean {m:.3}</text>", yc - 32.0, yc + 32.0, xv(m) + 6.0, yc + 44.0, x = xv(m));
    }
    let _ = write!(s, "<text x=\"20\" y=\"745\" fill=\"#555\">Measured: run records. Derived: ratios and intervals (Rust tool fit/). Native work priced in the table.md band, not here.</text></svg>");
    s
}
