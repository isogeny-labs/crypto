//! Analysis for research/jv_sieve_line_vs_rho_20261103 (frozen in PROTOCOL.md). Every number is computed here, in Rust.
//! Usage:  line_vs_rho <study dir> <#3278 results dir> <study A results dir>   full analysis
//!         line_vs_rho selftest <#3278 results dir>                            pooled statistics of the 126 #3278 runs
use serde_json::{json, Value};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const NB: usize = 20000; // bootstrap resamples (frozen)
const NPERM: usize = 20000; // permutation resamples (frozen)
const GAE: f64 = 331.0;
const WS: [f64; 3] = [0.0, 0.1, 1.0];
const PRIMES: [u64; 3] = [1009, 1511, 1823];
const ARMS: [&str; 3] = ["P", "H", "U"];
const NSEED: usize = 8; // cover seeds 17..=24
const NCELL: usize = 18;

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
fn verdict(lo: f64, hi: f64) -> &'static str { if hi < 1.0 { "below" } else if lo > 1.0 { "above" } else { "not resolved" } }
fn theory() -> f64 { (std::f64::consts::PI / 4.0).sqrt() / (1.0 - 1.0 / 256.0f64).sqrt() }

// ---------- k-sample statistics (groups as slices) ----------
fn anova_stat(g: &[Vec<f64>]) -> f64 {
    let all: Vec<f64> = g.iter().flatten().cloned().collect();
    let m = mean(&all);
    g.iter().map(|x| x.len() as f64 * (mean(x) - m).powi(2)).sum()
}
fn absdiff(g: &[Vec<f64>]) -> f64 { (mean(&g[0]) - mean(&g[1])).abs() }
fn ks(g: &[Vec<f64>]) -> f64 {
    let (mut x, mut y) = (g[0].clone(), g[1].clone());
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
fn kw_h(g: &[Vec<f64>]) -> f64 {
    let mut all: Vec<(f64, usize)> = vec![];
    for (k, x) in g.iter().enumerate() { for v in x { all.push((*v, k)); } }
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
    let mut rs = vec![0.0; g.len()];
    for (k, x) in all.iter().enumerate() { rs[x.1] += rank[k]; }
    let nf = n as f64;
    let mut h = -3.0 * (nf + 1.0);
    for (k, x) in g.iter().enumerate() { h += 12.0 / (nf * (nf + 1.0)) * rs[k] * rs[k] / x.len() as f64; }
    h / (1.0 - tie / (nf * nf * nf - nf))
}
/// (observed, permutation p); group sizes kept
fn perm(g: &[Vec<f64>], f: fn(&[Vec<f64>]) -> f64, rng: &mut Rng) -> (f64, f64) {
    let t0 = f(g);
    let sizes: Vec<usize> = g.iter().map(|x| x.len()).collect();
    let mut pool: Vec<f64> = g.iter().flatten().cloned().collect();
    let mut c = 0usize;
    for _ in 0..NPERM {
        for i in (1..pool.len()).rev() { let j = rng.idx(i + 1); pool.swap(i, j); }
        let mut off = 0;
        let gg: Vec<Vec<f64>> = sizes.iter().map(|&s| { let v = pool[off..off + s].to_vec(); off += s; v }).collect();
        if f(&gg) >= t0 - 1e-12 { c += 1; }
    }
    (t0, (1 + c) as f64 / (1 + NPERM) as f64)
}

// ---------- IO ----------
fn rows(path: &Path) -> Vec<Value> {
    let s = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str::<Value>(&s).unwrap().as_array().cloned().unwrap_or_default()
}
fn files_with(dir: &Path, prefix: &str) -> Vec<PathBuf> {
    let mut f: Vec<PathBuf> = match fs::read_dir(dir) { Ok(d) => d.filter_map(|e| e.ok()).map(|e| e.path()).collect(), Err(_) => vec![] }
        .into_iter().filter(|p| { let n = p.file_name().unwrap().to_string_lossy().to_string(); n.starts_with(prefix) && n.ends_with(".json") }).collect();
    f.sort();
    f
}
fn fl(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap_or_else(|| panic!("missing {k}")) }
fn nat_of(r: &Value) -> f64 { ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| fl(&r["ops"], k)).sum() }
fn strip(v: &Value) -> Value { let mut c = v.clone(); c.as_object_mut().unwrap().remove("wall_ms"); c }

#[derive(Clone, Debug)]
struct Run { p: u64, seed: u64, run: u64, l: f64, c_e: f64, steps_n: f64, u1: f64, u2: f64, nat: f64, fpa: f64 }
impl Run { fn v(&self) -> [f64; 4] { [self.u1, self.u2, self.nat, self.fpa] } }
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
            out.push(Run { p, seed, run, l, c_e: fl(&r, "c_add_e"), steps_n: fl(&r, "steps") / sl, u1: fl(&r, "s_harness"), u2: fl(&r, "s_tuned"),
                nat: nat_of(&r) / sl, fpa: fl(&r["ops"], "fp_adds") / sl });
        }
    }
    out
}
fn pooled_3278(dir: &Path, fails: &mut Vec<String>) -> Vec<Run> {
    let mut v = load_runs(&dir.join("inputs/old_rhostrong"), fails);
    v.extend(load_runs(&dir.join("new"), fails));
    v
}

// ---------- cover rows ----------
#[derive(Clone)]
struct Cov { p: u64, seed: u64, l: f64, c_e: f64, tm: f64, s: f64, sp: f64, nat_a: f64, nat_b: f64, ok: bool, shares: [f64; 6], s_adds: f64 }
fn cov_row(r: &Value, arm: &str) -> Cov {
    let l = fl(r, "l");
    let sl = l.sqrt();
    let tm = fl(r, "total_muls");
    let fb = |k: &str| r["fb_line"].get(k).and_then(|x| x.as_f64()).unwrap_or(0.0);
    let unemitted = tm - fl(r, "sieve_muls") - fl(r, "enum_muls");
    let fbm = fb("muls");
    let (em, sm, dm, lm) = (fl(r, "enum_muls"), fl(r, "sieve_muls"), fl(r, "descent_muls"), fl(r, "la_muls"));
    let fbok = match arm {
        "P" => r.get("fb_line").is_none(),
        "H" => r["fb_line"]["mode"].as_str() == Some("horner") && r["fb_line"]["candidates"].as_u64() == Some(4096),
        _ => r["fb_line"]["mode"].as_str() == Some("table") && r["fb_line"]["candidates"].as_u64() == Some(32768),
    };
    Cov { p: r["p"].as_u64().unwrap(), seed: r["seed"].as_u64().unwrap(), l, c_e: fl(r, "c_add_e"), tm, s: tm / (GAE * sl), sp: fl(r, "total_plus") / (GAE * sl),
        nat_a: (fl(r, "sieve_adds") + fl(r, "sieve_lookups") + fl(r, "enum_adds") + fb("adds") + fb("lookups") + unemitted) / sl,
        nat_b: (fl(r, "sieve_lookups") + fb("lookups")) / sl,
        ok: r["solved"].as_bool().unwrap() && r["correct"].as_bool().unwrap() && r["rels_failed_verify"].as_u64() == Some(0)
            && !r["exhausted"].as_bool().unwrap() && r["lin_mode"].as_str() == Some("sparse") && fbok,
        shares: [em, sm, dm, lm, fbm, tm - em - sm - dm - lm - fbm], s_adds: (fb("adds") + fb("lookups") + fl(r, "sieve_adds") + fl(r, "sieve_lookups") + fl(r, "enum_adds")) / sl }
}

// ---------- cells ----------
fn cell_labels() -> Vec<String> {
    let mut l: Vec<String> = ["S/rho U1", "S/rho U2", "S+/rho U1 (rho unpriced)", "S+/rho U2 (rho unpriced)", "S+/rhoS+ U1", "S+/rhoS+ U2"].iter().map(|s| s.to_string()).collect();
    for ly in ["A", "B"] { for u in ["U1", "U2"] { for w in WS { l.push(format!("band {ly} {u} w={w}")); } } }
    l
}
/// ln of the 18 per-instance ratios cover/reference. r = [u1, u2, native/sqrt(l), fp_adds/sqrt(l)] of the reference.
fn ln_cells(c: &Cov, r: &[f64; 4]) -> [f64; NCELL] {
    let rsp = (r[2] + r[3]) / GAE;
    let mut o = [0.0; NCELL];
    let base = [c.s / r[0], c.s / r[1], c.sp / r[0], c.sp / r[1], c.sp / (r[0] + rsp), c.sp / (r[1] + rsp)];
    for k in 0..6 { o[k] = base[k].ln(); }
    let mut k = 6;
    for ly in 0..2 { for u in 0..2 { for w in WS {
        let nc = if ly == 0 { c.nat_a } else { c.nat_b };
        let nr = if ly == 0 { r[2] + r[3] } else { r[2] };
        o[k] = ((c.s + w * nc) / (r[u] + w * nr)).ln();
        k += 1;
    } } }
    o
}
type Groups = Vec<Vec<[f64; 4]>>;
fn groups_of(runs: &[Run]) -> Groups {
    let mut keys: Vec<(u64, u64)> = runs.iter().map(|r| (r.p, r.seed)).collect();
    keys.sort();
    keys.dedup();
    keys.iter().map(|k| runs.iter().filter(|r| (r.p, r.seed) == *k).map(|r| r.v()).collect()).collect()
}
fn gmean4(g: &Groups) -> [f64; 4] {
    let (mut s, mut n) = ([0.0; 4], 0.0);
    for x in g { for v in x { for k in 0..4 { s[k] += v[k]; } n += 1.0; } }
    for k in 0..4 { s[k] /= n; }
    s
}
fn resample4(g: &Groups, rng: &mut Rng) -> [f64; 4] {
    let (mut s, mut n) = ([0.0; 4], 0.0);
    for x in g { for _ in 0..x.len() { let v = &x[rng.idx(x.len())]; for k in 0..4 { s[k] += v[k]; } n += 1.0; } }
    for k in 0..4 { s[k] /= n; }
    s
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() >= 3 && a[1] == "selftest" {
        let mut f = vec![];
        let pr = pooled_3278(Path::new(&a[2]), &mut f);
        let sn: Vec<f64> = pr.iter().map(|r| r.steps_n).collect();
        println!("pooled #3278 runs n={} (excluded {}): steps/sqrt(l) mean {:.4} sd {:.4}; s_harness mean {:.4}; s_tuned mean {:.4}; theory {:.5}; z {:.3}",
            pr.len(), f.len(), mean(&sn), sd(&sn), mean(&pr.iter().map(|r| r.u1).collect::<Vec<_>>()), mean(&pr.iter().map(|r| r.u2).collect::<Vec<_>>()),
            theory(), (mean(&sn) - theory()) / (sd(&sn) / (sn.len() as f64).sqrt()));
        return;
    }
    let study = PathBuf::from(&a[1]);
    let dir3278 = PathBuf::from(&a[2]);
    let dir_a = PathBuf::from(&a[3]);
    let mut out = String::new();
    let mut fails: Vec<String> = vec![];
    let mut rng = Rng(0x11AE_5EA2_0C0D_2026);

    // ---- cover rows [arm][prime][seed 17..24]
    let mut cov: Vec<Vec<Vec<Cov>>> = vec![];
    let _ = writeln!(out, "# Cover records and control C1 (arms re-run = study A's records on seeds 17-24, every field but wall_ms)");
    let mut c1_total = 0;
    let mut c1_eq = 0;
    for arm in ARMS {
        let mut per_p = vec![];
        for &p in &PRIMES {
            let new = rows(&study.join(format!("results/cover/{arm}_{p}.json")));
            let old: Vec<Value> = rows(&dir_a.join(format!("{arm}_{p}.json"))).into_iter().filter(|r| (17..=24).contains(&r["seed"].as_u64().unwrap())).collect();
            if new.len() != NSEED { fails.push(format!("cover {arm}_{p}: {} records, expected {NSEED}", new.len())); }
            let mut v = vec![];
            for (i, r) in new.iter().enumerate() {
                let seed = r["seed"].as_u64().unwrap();
                if seed != 17 + i as u64 { fails.push(format!("cover {arm}_{p}: record {i} has seed {seed}")); }
                c1_total += 1;
                match old.iter().find(|o| o["seed"] == r["seed"]) {
                    Some(o) if strip(o) == strip(r) => c1_eq += 1,
                    Some(_) => fails.push(format!("C1: {arm} p={p} seed {seed} differs from study A")),
                    None => fails.push(format!("C1: {arm} p={p} seed {seed} not in study A")),
                }
                let c = cov_row(r, arm);
                if !c.ok { fails.push(format!("C2: {arm} p={p} seed {seed} not solved/correct/sparse/arm-settings")); }
                if c.c_e != GAE || c.p != p { fails.push(format!("{arm} p={p} seed {seed}: c_add_e {} or p {}", c.c_e, c.p)); }
                v.push(c);
            }
            per_p.push(v);
        }
        cov.push(per_p);
    }
    let c1_pass = c1_eq == c1_total && c1_total == 3 * 3 * NSEED;
    let _ = writeln!(out, "C1: {c1_eq}/{c1_total} records equal study A's on every field but wall_ms: {}", if c1_pass { "PASS" } else { "FAIL" });
    let c2_pass = cov.iter().flatten().flatten().all(|c| c.ok);
    let _ = writeln!(out, "C2: all {} cover runs solved, correct, 0 failed verifications, not exhausted, sparse, arm settings as declared: {}", 3 * 3 * NSEED, if c2_pass { "PASS" } else { "FAIL" });
    if !c1_pass { fails.push("C1 failed: all cover claims withheld".into()); }
    if !c2_pass { fails.push("C2 failed".into()); }

    // ---- new rho runs
    let new_all = load_runs(&study.join("results/rho"), &mut fails);
    let mut by_p: Vec<Vec<Run>> = PRIMES.iter().map(|&p| new_all.iter().filter(|r| r.p == p).cloned().collect()).collect();
    for v in by_p.iter_mut() { v.sort_by_key(|r| (r.seed, r.run)); }
    let _ = writeln!(out, "\n# Matched rho runs (new)");
    let want = [16usize, 16, 8];
    let mut cut_note = String::new();
    for (pi, &p) in PRIMES.iter().enumerate() {
        let seeds: Vec<u64> = { let mut s: Vec<u64> = by_p[pi].iter().map(|r| r.seed).collect(); s.dedup(); s };
        let _ = writeln!(out, "p={p}: {} runs (expected {}), seeds {:?}", by_p[pi].len(), want[pi], seeds);
        if by_p[pi].len() != want[pi] { cut_note += &format!("p={p} has {} of {} runs; ", by_p[pi].len(), want[pi]); fails.push(format!("CUT: p={p} has {} of {} planned rho runs", by_p[pi].len(), want[pi])); }
    }
    if by_p.iter().any(|v| v.is_empty()) { println!("{out}\nrho runs missing for a prime"); fs::write(study.join("fit.txt"), &out).unwrap(); return; }
    // identity
    let mut ident = true;
    for (pi, v) in by_p.iter().enumerate() {
        for r in v {
            match cov[0][pi].iter().find(|c| c.seed == r.seed) {
                Some(c) if c.l == r.l && c.c_e == r.c_e => {}
                _ => { ident = false; fails.push(format!("identity: p={} seed {} run {}: l/c_add_e differ from the cover record", r.p, r.seed, r.run)); }
            }
        }
    }
    let _ = writeln!(out, "identity (l and c_add_e of every rho run equal the cover record of the same prime and seed): {ident}");

    // ---- replay controls
    let mut replay_ok = true;
    for tag in ["B_p1511_s17_run0", "O_p1451_s1_run0"] {
        let (po, pn) = (study.join(format!("results/replay/{tag}.old.json")), study.join(format!("results/replay/{tag}.new.json")));
        let ok = po.exists() && pn.exists() && { let (x, y) = (rows(&po), rows(&pn)); !x.is_empty() && x.len() == y.len() && x.iter().zip(&y).all(|(p, q)| strip(p) == strip(q)) };
        let _ = writeln!(out, "replay control {tag} (frozen binary vs earlier record, every field but wall_ms): bit-identical = {ok}");
        if !ok { fails.push(format!("replay {tag} failed or missing")); }
        replay_ok &= ok;
    }

    // ---- #3278 reference
    let ref126 = pooled_3278(&dir3278, &mut fails);
    let _ = writeln!(out, "#3278 runs: {}", ref126.len());

    // ---- positive control per prime
    let th = theory();
    let _ = writeln!(out, "\n# Positive control per prime: mean steps/sqrt(l) within 3 se of theory {th:.5}");
    let mut pc = vec![];
    for (pi, &p) in PRIMES.iter().enumerate() {
        let x: Vec<f64> = by_p[pi].iter().map(|r| r.steps_n).collect();
        let se = sd(&x) / (x.len() as f64).sqrt();
        let z = (mean(&x) - th) / se;
        let pass = z.abs() <= 3.0;
        let _ = writeln!(out, "p={p}: n={} mean {:.4} sd {:.4} se {:.4} z {:.3} PASS={pass}", x.len(), mean(&x), sd(&x), se, z);
        if !pass { fails.push(format!("positive control failed at p={p}")); }
        pc.push(json!({"p": p, "n": x.len(), "mean": mean(&x), "sd": sd(&x), "se": se, "z": z, "pass": pass}));
    }
    let sn126: Vec<f64> = ref126.iter().map(|r| r.steps_n).collect();
    let _ = writeln!(out, "#3278 126 runs: mean {:.4} sd {:.4} z {:.3} (information)", mean(&sn126), sd(&sn126), (mean(&sn126) - th) / (sd(&sn126) / (sn126.len() as f64).sqrt()));

    // ---- homogeneity (frozen)
    let _ = writeln!(out, "\n# Homogeneity (frozen decision metric: steps/sqrt(l))");
    let g3: Vec<Vec<f64>> = by_p.iter().map(|v| v.iter().map(|r| r.steps_n).collect()).collect();
    let (_, p_an) = perm(&g3, anova_stat, &mut rng);
    let (_, p_kw3) = perm(&g3, kw_h, &mut rng);
    let _ = writeln!(out, "H-a across the three primes' new runs: permutation ANOVA p {p_an:.4}; Kruskal-Wallis permutation p {p_kw3:.4} (means {:.4} / {:.4} / {:.4})", mean(&g3[0]), mean(&g3[1]), mean(&g3[2]));
    let allnew: Vec<f64> = g3.iter().flatten().cloned().collect();
    let (d, p_d) = perm(&[allnew.clone(), sn126.clone()], absdiff, &mut rng);
    let (k, p_k) = perm(&[allnew.clone(), sn126.clone()], ks, &mut rng);
    let _ = writeln!(out, "H-b all {} new runs vs #3278's {}: means {:.4} vs {:.4}; |diff| {:.4} perm p {:.4}; KS D {:.4} perm p {:.4}", allnew.len(), sn126.len(), mean(&allnew), mean(&sn126), d, p_d, k, p_k);
    let mut hom = json!({"anova_p": p_an, "kw_p": p_kw3, "new_vs_3278_absdiff_p": p_d, "new_vs_3278_ks_p": p_k});
    for (pi, &p) in PRIMES.iter().enumerate() {
        let (d1, pd1) = perm(&[g3[pi].clone(), sn126.clone()], absdiff, &mut rng);
        let (k1, pk1) = perm(&[g3[pi].clone(), sn126.clone()], ks, &mut rng);
        let _ = writeln!(out, "  (information) p={p} vs #3278's 126: means {:.4} vs {:.4}; |diff| {:.4} p {:.4}; KS {:.4} p {:.4}", mean(&g3[pi]), mean(&sn126), d1, pd1, k1, pk1);
        hom[format!("p{p}_vs_3278")] = json!({"absdiff_p": pd1, "ks_p": pk1});
    }
    for (nm, fm) in [("s_harness", 0usize), ("s_tuned", 1usize)] {
        let g: Vec<Vec<f64>> = by_p.iter().map(|v| v.iter().map(|r| r.v()[fm]).collect()).collect();
        let (_, pa) = perm(&g, anova_stat, &mut rng);
        let an: Vec<f64> = g.iter().flatten().cloned().collect();
        let ref_: Vec<f64> = ref126.iter().map(|r| r.v()[fm]).collect();
        let (_, pd2) = perm(&[an.clone(), ref_.clone()], absdiff, &mut rng);
        let (_, pk2) = perm(&[an, ref_], ks, &mut rng);
        let _ = writeln!(out, "  (information) {nm}: ANOVA p {pa:.4}; new vs 126 |diff| p {pd2:.4}; KS p {pk2:.4}");
    }
    let homogeneous = p_an >= 0.05 && p_kw3 >= 0.05 && p_d >= 0.05 && p_k >= 0.05;
    let pooled_primary = homogeneous && replay_ok;
    let _ = writeln!(out, "HOMOGENEOUS (all four permutation p >= 0.05): {homogeneous}; replays ok: {replay_ok} -> PRIMARY REFERENCE = {}", if pooled_primary { "pooled (#3278 126 + all new)" } else { "per-prime matched runs" });

    // ---- references
    let mut pooled_runs = ref126.clone();
    if replay_ok { pooled_runs.extend(new_all.iter().cloned()); } else { pooled_runs = new_all.clone(); }
    let g_pool = groups_of(&pooled_runs);
    let g_prime: Vec<Groups> = by_p.iter().map(|v| groups_of(v)).collect();
    let ref_pool = gmean4(&g_pool);
    let ref_prime: Vec<[f64; 4]> = g_prime.iter().map(gmean4).collect();
    let _ = writeln!(out, "\nreference means [s_harness, s_tuned, native/sqrt(l), fp_adds/sqrt(l)]\n  pooled ({} runs): {:?}", pooled_runs.len(), ref_pool);
    for (pi, &p) in PRIMES.iter().enumerate() { let _ = writeln!(out, "  p={p} matched ({} runs): {:?}", by_p[pi].len(), ref_prime[pi]); }
    let refs_pt: [Vec<[f64; 4]>; 2] = [vec![ref_prime[0], ref_prime[1], ref_prime[2]], vec![ref_pool; 3]]; // mode 0 per-prime, 1 pooled
    let mode_name = ["per-prime", "pooled"];
    let prim = if pooled_primary { 1 } else { 0 };

    // ---- point estimates and bootstrap: series[arm][mode][scope 0=pooled,1..3=primes][cell]
    let labs = cell_labels();
    let point = |arm: usize, mode: usize, idx: &[Vec<usize>]| -> Vec<Vec<f64>> {
        let mut sums = vec![[0.0f64; NCELL]; 3];
        for pi in 0..3 { for &i in &idx[pi] { let l = ln_cells(&cov[arm][pi][i], &refs_pt[mode][pi]); for k in 0..NCELL { sums[pi][k] += l[k]; } } }
        let mut res = vec![vec![0.0; NCELL]; 4];
        for k in 0..NCELL {
            let mut tot = 0.0;
            for pi in 0..3 { res[pi + 1][k] = (sums[pi][k] / idx[pi].len() as f64).exp(); tot += sums[pi][k]; }
            res[0][k] = (tot / idx.iter().map(|x| x.len()).sum::<usize>() as f64).exp();
        }
        res
    };
    let ident_idx: Vec<Vec<usize>> = (0..3).map(|_| (0..NSEED).collect()).collect();
    let mut series: Vec<Vec<Vec<Vec<Vec<f64>>>>> = vec![vec![vec![vec![Vec::with_capacity(NB); NCELL]; 4]; 2]; 3];
    for _ in 0..NB {
        let rm_prime: Vec<[f64; 4]> = (0..3).map(|pi| resample4(&g_prime[pi], &mut rng)).collect();
        let rm_pool = resample4(&g_pool, &mut rng);
        let refs_b: [Vec<[f64; 4]>; 2] = [rm_prime, vec![rm_pool; 3]];
        let idx: Vec<Vec<usize>> = (0..3).map(|_| (0..NSEED).map(|_| rng.idx(NSEED)).collect()).collect();
        for arm in 0..3 { for mode in 0..2 {
            let mut sums = vec![[0.0f64; NCELL]; 3];
            for pi in 0..3 { for &i in &idx[pi] { let l = ln_cells(&cov[arm][pi][i], &refs_b[mode][pi]); for k in 0..NCELL { sums[pi][k] += l[k]; } } }
            for k in 0..NCELL {
                let mut tot = 0.0;
                for pi in 0..3 { series[arm][mode][pi + 1][k].push((sums[pi][k] / NSEED as f64).exp()); tot += sums[pi][k]; }
                series[arm][mode][0][k].push((tot / (3 * NSEED) as f64).exp());
            }
        } }
    }
    let scope_name = ["pooled (24 instances)", "p=1009", "p=1511", "p=1823"];
    let mut j = json!({});
    let mut table = String::new();
    let _ = writeln!(table, "# Tables (derived; Rust `fit/`)\n\nPrimary reference: **{}**. Ratios are cover / matched negation rho per instance, geometric mean over instances; 95% percentile bootstrap, {NB} resamples, jointly over cover seeds (within prime) and rho runs (within instance).", mode_name[prim]);
    let mut pts_all = vec![]; // [arm][mode] -> Vec<Vec<f64>>
    for arm in 0..3 { let mut m = vec![]; for mode in 0..2 { m.push(point(arm, mode, &ident_idx)); } pts_all.push(m); }
    let _ = writeln!(out);
    for mode in [prim, 1 - prim] {
        let _ = writeln!(out, "\n## Reference {} {}", mode_name[mode], if mode == prim { "[PRIMARY]" } else { "[secondary]" });
        for arm in 0..3 { for sc in 0..4 {
            let _ = writeln!(out, "-- arm {} / {}", ARMS[arm], scope_name[sc]);
            for k in 0..NCELL {
                let (lo, hi) = (pct(&series[arm][mode][sc][k], 0.025), pct(&series[arm][mode][sc][k], 0.975));
                let pt = pts_all[arm][mode][sc][k];
                let _ = writeln!(out, "  {:30} {:.5}  [{:.5}, {:.5}]  {}  (x{:.1} below [{:.1}, {:.1}])", labs[k], pt, lo, hi, verdict(lo, hi), 1.0 / pt, 1.0 / hi, 1.0 / lo);
                j[format!("{}|{}|{}|{}", mode_name[mode], ARMS[arm], scope_name[sc], labs[k])] = json!({"point": pt, "lo": lo, "hi": hi, "verdict": verdict(lo, hi)});
            }
        } }
    }
    // tables (both modes, primary first)
    for mode in [prim, 1 - prim] {
        let _ = writeln!(table, "\n## Reference: {} {}\n", mode_name[mode], if mode == prim { "(PRIMARY)" } else { "(secondary)" });
        for (title, ks_) in [("S/rho (muls only, search charged)", [0usize, 1]), ("S+/rho (adds and lookups charged at one mult; rho unpriced)", [2, 3])] {
            let _ = writeln!(table, "### {title}\n\n| arm | scope | U1 | U2 |\n|:--|:--|--:|--:|");
            for arm in 0..3 { for sc in 0..4 {
                let c = |k: usize| { let (lo, hi) = (pct(&series[arm][mode][sc][k], 0.025), pct(&series[arm][mode][sc][k], 0.975)); format!("{:.5} [{:.5}, {:.5}] ({})", pts_all[arm][mode][sc][k], lo, hi, verdict(lo, hi)) };
                let _ = writeln!(table, "| {} | {} | {} | {} |", ARMS[arm], scope_name[sc], c(ks_[0]), c(ks_[1]));
            } }
            let _ = writeln!(table);
        }
        let _ = writeln!(table, "### S+/(rho S+) like-for-like (rho native counters + fp_adds priced one per mult)\n\n| arm | scope | U1 | U2 |\n|:--|:--|--:|--:|");
        for arm in 0..3 { for sc in 0..4 {
            let c = |k: usize| { let (lo, hi) = (pct(&series[arm][mode][sc][k], 0.025), pct(&series[arm][mode][sc][k], 0.975)); format!("{:.5} [{:.5}, {:.5}]", pts_all[arm][mode][sc][k], lo, hi) };
            let _ = writeln!(table, "| {} | {} | {} | {} |", ARMS[arm], scope_name[sc], c(4), c(5));
        } }
        let _ = writeln!(table, "\n### Section 5a band (cover/rho; native operations priced at w GAE each on both arms)\n\nLayer A = V2's upper bound (cover: sieve adds + lookups + enumeration adds + search adds + lookups + all unemitted multiplications repriced as native; rho: six counters + fp_adds). Layer B = as emitted (cover: sieve + search lookups; rho: the six #3278 counters).\n\n| layer | unit | arm | scope | w=0 | w=0.1 | w=1 | order across band |\n|:--|:--|:--|:--|--:|--:|--:|:--|");
        for ly in 0..2 { for u in 0..2 { for arm in 0..3 { for sc in 0..4 {
            let mut cs = vec![]; let mut vs = vec![];
            for wi in 0..3 {
                let k = 6 + ly * 6 + u * 3 + wi;
                let (lo, hi) = (pct(&series[arm][mode][sc][k], 0.025), pct(&series[arm][mode][sc][k], 0.975));
                cs.push(format!("{:.4} [{:.4}, {:.4}]", pts_all[arm][mode][sc][k], lo, hi)); vs.push(verdict(lo, hi));
            }
            let _ = writeln!(table, "| {} | U{} | {} | {} | {} | {} | {} | {} ({}) |", if ly == 0 { "A" } else { "B" }, u + 1, ARMS[arm], scope_name[sc], cs[0], cs[1], cs[2], if vs[0] == vs[1] && vs[1] == vs[2] { "holds" } else { "not resolved" }, vs[0]);
        } } } }
    }

    // ---- per-instance table (primary reference)
    let _ = writeln!(table, "\n## Per instance (measured S, S+; ratio to the primary reference, {})\n\n| p | seed | arm | S | S+ | S/rho U1 | S/rho U2 | S+/rho U1 | S+/rho U2 |\n|--:|--:|:--|--:|--:|--:|--:|--:|--:|", mode_name[prim]);
    for pi in 0..3 { for i in 0..NSEED { for arm in 0..3 {
        let c = &cov[arm][pi][i];
        let r = &refs_pt[prim][pi];
        let _ = writeln!(table, "| {} | {} | {} | {:.5} | {:.5} | {:.5} | {:.5} | {:.5} | {:.5} |", PRIMES[pi], c.seed, ARMS[arm], c.s, c.sp, c.s / r[0], c.s / r[1], c.sp / r[0], c.sp / r[1]);
    } } }
    // own-run secondary
    let _ = writeln!(out, "\n# Secondary: ratio to each instance's own rho runs (noisy, 1-2 runs), geometric mean over instances");
    for arm in 0..3 {
        let mut o = format!("arm {}:", ARMS[arm]);
        for (u, un) in [(0usize, "S/rho U1"), (1, "S/rho U2")] {
            let mut ls = vec![];
            for pi in 0..3 { for c in &cov[arm][pi] {
                let rr: Vec<f64> = by_p[pi].iter().filter(|r| r.seed == c.seed).map(|r| r.v()[u]).collect();
                if !rr.is_empty() { ls.push((c.s / mean(&rr)).ln()); }
            } }
            o += &format!("  {un} {:.5} (n={})", mean(&ls).exp(), ls.len());
        }
        let _ = writeln!(out, "{o}");
    }
    // ---- paired arm/P (cover only, point)
    let _ = writeln!(out, "\n# Cover-only paired ratios arm/P (point, geometric mean, 24 instances) for consistency with study A");
    for arm in 1..3 {
        let g = |f: &dyn Fn(&Cov) -> f64| { let mut l = vec![]; for pi in 0..3 { for i in 0..NSEED { l.push((f(&cov[arm][pi][i]) / f(&cov[0][pi][i])).ln()); } } mean(&l).exp() };
        let _ = writeln!(out, "arm {}: S {:.4}  S+ {:.4}  (per prime S: {})", ARMS[arm], g(&|c| c.s), g(&|c| c.sp),
            (0..3).map(|pi| { let mut l = vec![]; for i in 0..NSEED { l.push((cov[arm][pi][i].s / cov[0][pi][i].s).ln()); } format!("{:.4}", mean(&l).exp()) }).collect::<Vec<_>>().join(" / "));
    }
    // ---- phase shares
    let _ = writeln!(out, "\n# Phase shares of total_muls (ratio of means over the 8 instances): enum / sieve / descent / linear algebra / search / other");
    for arm in 0..3 { for pi in 0..3 {
        let mut s = [0.0; 6];
        for c in &cov[arm][pi] { for k in 0..6 { s[k] += c.shares[k]; } }
        let t: f64 = s.iter().sum();
        let _ = writeln!(out, "arm {} p={}: {}", ARMS[arm], PRIMES[pi], s.iter().map(|x| format!("{:.3}", x / t)).collect::<Vec<_>>().join(" / "));
    } }
    // ---- product-estimate comparison at p = 1511
    let _ = writeln!(out, "\n# Comparison with LEADS.md's product estimate at p = 1511 (primary reference; seeds 17-24)");
    for (arm, k, nm, est) in [(1usize, 0usize, "H, S/rho U1", 0.0042f64), (1, 1, "H, S/rho U2", 0.010), (2, 3, "U, S+/rho U2", 0.034), (2, 2, "U, S+/rho U1", 0.014)] {
        let (lo, hi) = (pct(&series[arm][prim][2][k], 0.025), pct(&series[arm][prim][2][k], 0.975));
        let pt = pts_all[arm][prim][2][k];
        let _ = writeln!(out, "{nm}: measured {pt:.5} [{lo:.5}, {hi:.5}] vs product {est}: ratio measured/product {:.3}; product inside interval: {}", pt / est, est >= lo && est <= hi);
        j[format!("product|{nm}")] = json!({"measured": pt, "lo": lo, "hi": hi, "product": est, "inside": est >= lo && est <= hi});
    }
    let _ = writeln!(out, "\n# Failures / flags\n{}", if fails.is_empty() { "none".to_string() } else { fails.join("\n") });
    j["homogeneity"] = hom;
    j["controls"] = json!({"c1_pass": c1_pass, "c2_pass": c2_pass, "identity": ident, "replays_ok": replay_ok, "positive_control": pc, "theory": th, "homogeneous": homogeneous, "primary": mode_name[prim], "cut": cut_note, "flags": fails});
    j["reference_means"] = json!({"pooled": ref_pool, "per_prime": ref_prime, "fields": ["s_harness", "s_tuned", "native_per_sqrt_l", "fp_adds_per_sqrt_l"]});
    fs::write(study.join("fit.txt"), &out).unwrap();
    fs::write(study.join("fit.json"), serde_json::to_string_pretty(&j).unwrap()).unwrap();
    fs::write(study.join("table.md"), &table).unwrap();
    let svg = graph(&series, &pts_all, prim, &by_p, &ref126, th, mode_name[prim]);
    fs::write(study.join("graph.svg"), svg).unwrap();
    print!("{out}");
}

fn graph(series: &Vec<Vec<Vec<Vec<Vec<f64>>>>>, pts: &Vec<Vec<Vec<Vec<f64>>>>, prim: usize, by_p: &[Vec<Run>], r126: &[Run], th: f64, refname: &str) -> String {
    let (w, h) = (1040.0, 1330.0);
    let mut s = String::new();
    let _ = write!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"DejaVu Sans, Arial, sans-serif\" font-size=\"12\"><rect width=\"{w}\" height=\"{h}\" fill=\"#ffffff\"/>");
    let _ = write!(s, "<text x=\"20\" y=\"26\" font-size=\"16\" font-weight=\"bold\">Line-search sieve route vs matched negation rho, same instances (reference: {refname})</text>");
    let _ = write!(s, "<text x=\"20\" y=\"44\" fill=\"#444\">Cover / rho per instance, geometric mean, 95% bootstrap over cover seeds and rho runs. Log scale; 1 = parity with rho. P plain, H Horner n=4096, U table n=32768.</text>");
    let (x0, x1) = (200.0, 1010.0);
    let (lo, hi) = (1e-3f64.ln(), 1.5f64.ln());
    let xp = |v: f64| x0 + (v.max(1e-3).ln() - lo) / (hi - lo) * (x1 - x0);
    let cols = ["#555555", "#0072B2", "#D55E00"];
    let blocks = [("S, U1 (group additions)", 0usize), ("S, U2 (textbook F_p mults)", 1), ("S+, U1 (rho unpriced)", 2), ("S+, U2 (rho unpriced)", 3)];
    let names = ["pooled", "p=1009", "p=1511", "p=1823"];
    let mut y = 70.0;
    for (bn, k) in blocks {
        let _ = write!(s, "<text x=\"20\" y=\"{}\" font-weight=\"bold\">{bn}</text>", y + 4.0);
        let top = y + 12.0;
        let hgt = 4.0 * 52.0;
        for t in [1e-3, 1e-2, 1e-1, 1.0] {
            let _ = write!(s, "<line x1=\"{x}\" y1=\"{top}\" x2=\"{x}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/><text x=\"{x}\" y=\"{}\" text-anchor=\"middle\" fill=\"#444\">{}</text>", top + hgt, if t == 1.0 { "#222" } else { "#ddd" }, if t == 1.0 { 2 } else { 1 }, top + hgt + 14.0, if t == 1.0 { "1 (rho)".to_string() } else { format!("{t}") }, x = xp(t));
        }
        for sc in 0..4 {
            let yb = top + sc as f64 * 52.0;
            let _ = write!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{}</text>", x0 - 10.0, yb + 30.0, names[sc]);
            for arm in 0..3 {
                let yy = yb + 12.0 + arm as f64 * 14.0;
                let (l, u) = (pct(&series[arm][prim][sc][k], 0.025), pct(&series[arm][prim][sc][k], 0.975));
                let p = pts[arm][prim][sc][k];
                let _ = write!(s, "<line x1=\"{}\" y1=\"{yy}\" x2=\"{}\" y2=\"{yy}\" stroke=\"{}\" stroke-width=\"3\"/><circle cx=\"{}\" cy=\"{yy}\" r=\"4.5\" fill=\"{}\"/>", xp(l), xp(u), cols[arm], xp(p), cols[arm]);
                if sc == 0 || sc == 2 { let _ = write!(s, "<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"10\">{} {:.4}</text>", xp(u) + 6.0, yy + 3.0, cols[arm], ["P", "H", "U"][arm], p); }
            }
        }
        y += 4.0 * 52.0 + 52.0;
    }
    y += 30.0;
    // legend
    for (i, nm) in ["P plain", "H Horner n=4096", "U table n=32768"].iter().enumerate() {
        let _ = write!(s, "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{}\"/><text x=\"{}\" y=\"{}\">{nm}</text>", 40.0 + i as f64 * 200.0, y - 20.0, cols[i], 52.0 + i as f64 * 200.0, y - 16.0);
    }
    // reference strip
    let _ = write!(s, "<text x=\"20\" y=\"{}\" font-weight=\"bold\">Rho positive control: steps/sqrt(l), mean +/- 3 se (theory {th:.3} dashed)</text>", y + 14.0);
    let (bx0, bx1) = (200.0, 1010.0);
    let (vlo, vhi) = (0.4f64, 1.6f64);
    let xv = |v: f64| bx0 + (v - vlo) / (vhi - vlo) * (bx1 - bx0);
    let _ = write!(s, "<line x1=\"{x}\" y1=\"{}\" x2=\"{x}\" y2=\"{}\" stroke=\"#009E73\" stroke-width=\"2\" stroke-dasharray=\"6 4\"/>", y + 22.0, y + 22.0 + 4.0 * 26.0, x = xv(th));
    let groups: Vec<(String, Vec<f64>)> = vec![("p=1009".into(), by_p[0].iter().map(|r| r.steps_n).collect()), ("p=1511".into(), by_p[1].iter().map(|r| r.steps_n).collect()),
        ("p=1823".into(), by_p[2].iter().map(|r| r.steps_n).collect()), ("#3278 (126)".into(), r126.iter().map(|r| r.steps_n).collect())];
    for (i, (nm, v)) in groups.iter().enumerate() {
        let yy = y + 36.0 + i as f64 * 26.0;
        let (m, se) = (mean(v), sd(v) / (v.len() as f64).sqrt());
        let _ = write!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{nm} n={}</text><line x1=\"{}\" y1=\"{yy}\" x2=\"{}\" y2=\"{yy}\" stroke=\"#444\" stroke-width=\"3\"/><circle cx=\"{}\" cy=\"{yy}\" r=\"5\" fill=\"#444\"/><text x=\"{}\" y=\"{}\" fill=\"#444\">{m:.3}</text>", bx0 - 10.0, yy + 4.0, v.len(), xv((m - 3.0 * se).max(vlo)), xv((m + 3.0 * se).min(vhi)), xv(m), xv(m) + 8.0, yy - 7.0);
    }
    let _ = write!(s, "<text x=\"20\" y=\"{}\" fill=\"#555\" font-size=\"11\">Measured: run records. Derived: ratios and intervals (Rust tool fit/). Native work priced in table.md (Section 5a band), not here.</text></svg>", h - 12.0);
    s
}
