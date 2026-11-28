// Amendment-1 analysis for GB-SOLVER-SHOOTOUT-20261116, local diagnostic run.
//
// POST-DATA: this script was written on 2026-11-17 AFTER run-local/ existed.
// Amendment-1 (M3, PA-5) required an analysis script pinned by amendment-2
// before any data; none was. This file implements the frozen decision_rule
// text of protocol.amend1.json literally; it is not a preregistered script.
// analyze.py v1 is not used (protocol.amend1.json "inadmissible").
//
// Std-only Rust (AGENTS.md "Implementation language: no Python").
// Build: rustc -O analyze_amend1.rs -o <bin>
// Phase 1 (blind):  <bin> blind  <rows.tsv> <params.tsv> <outdir>
// Phase 2 (unmask): <bin> unmask <rows.tsv> <params.tsv> <key.tsv> <outdir>
//   key.tsv: "<arm name>\t<masked label>" per line (from blinding_key.json).
// Outputs are write-once (refuses to overwrite).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::Path;

const BOOT: usize = 20000;
const ALPHA: f64 = 0.05;
const FAMILY: f64 = 3.0; // registered pipeline pairs (decision_rule)
const BAND: [f64; 3] = [0.0, 0.1, 1.0];
const MARGIN: f64 = 10.0; // projected relations = fb_abscissae + 10

#[derive(Clone, Debug)]
struct Row {
    arm: String,
    inst: String,
    stratum: String,
    pass: String,
    position: i64,
    excluded: bool,
    outcome: String,
    charged: f64,
    internal: f64,
    projection: f64,
    n_verified: f64,
    oracle_count: f64,
    matches_oracle: bool,
    spurious: f64,
    native_ops: f64,
    degree: Option<i64>,
    mm: Option<(i64, i64)>,
    rss: f64,
    watchdog: bool,
    planted_found: Option<bool>,
    m1: bool,
    zero_abs: f64,
}

fn die(msg: &str) -> ! {
    eprintln!("analyze_amend1: {msg}");
    std::process::exit(2)
}

fn num(s: &str, what: &str) -> f64 {
    s.parse::<f64>().unwrap_or_else(|_| die(&format!("field {what} not numeric: '{s}' (fail closed)")))
}

fn load_rows(p: &str) -> Vec<Row> {
    let text = fs::read_to_string(p).unwrap_or_else(|e| die(&format!("{p}: {e}")));
    let mut lines = text.lines();
    let head: Vec<&str> = lines.next().unwrap().split('\t').collect();
    let idx = |n: &str| head.iter().position(|h| *h == n).unwrap_or_else(|| die(&format!("missing column {n}")));
    let mut out = vec![];
    for l in lines {
        if l.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = l.split('\t').collect();
        let g = |n: &str| f[idx(n)];
        let opt_i = |n: &str| if g(n) == "NA" { None } else { Some(num(g(n), n) as i64) };
        out.push(Row {
            arm: g("arm").into(),
            inst: g("instance").into(),
            stratum: g("stratum").into(),
            pass: g("pass").into(),
            position: num(g("position"), "position") as i64,
            excluded: g("excluded") == "true",
            outcome: g("outcome").into(),
            charged: num(g("charged_cpu_s"), "charged_cpu_s"),
            internal: num(g("internal_cpu_s"), "internal_cpu_s"),
            projection: num(g("projection_cpu_s"), "projection_cpu_s"),
            n_verified: num(g("n_verified"), "n_verified"),
            oracle_count: num(g("oracle_count"), "oracle_count"),
            matches_oracle: g("matches_oracle") == "true",
            spurious: num(g("spurious_roots"), "spurious_roots"),
            native_ops: num(g("native_ops"), "native_ops"),
            degree: opt_i("degree"),
            mm: match (opt_i("mm_rows"), opt_i("mm_cols")) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            },
            rss: num(g("peak_rss_bytes"), "peak_rss_bytes"),
            watchdog: g("watchdog_killed") == "true",
            planted_found: match g("planted_found") {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            },
            m1: g("m1_violation") == "true",
            zero_abs: num(g("zero_abscissa_roots"), "zero_abscissa_roots"),
        });
    }
    out
}

fn load_kv(p: &str) -> BTreeMap<String, String> {
    let text = fs::read_to_string(p).unwrap_or_else(|e| die(&format!("{p}: {e}")));
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.splitn(2, '\t');
            (it.next().unwrap().to_string(), it.next().unwrap_or_else(|| die("bad kv line")).to_string())
        })
        .collect()
}

struct Params {
    p_sat: f64,
    fb: f64,
    a_rho: f64,
    u_rho: f64,
    e_steps: f64,
    rho_cpu: f64,
    cap: f64,
    seed_str: String,
}

fn params(p: &str) -> Params {
    let kv = load_kv(p);
    let g = |k: &str| num(kv.get(k).unwrap_or_else(|| die(&format!("param {k} missing (fail closed)"))), k);
    if g("rho_verified") != g("rho_dlps") {
        die("rho reference not fully verified");
    }
    Params {
        p_sat: g("p_sat_num") / g("p_sat_den"),
        fb: g("fb_abscissae"),
        a_rho: g("rho_walk_ga"),
        u_rho: g("rho_partition_hashes") + g("rho_canonicalizations"),
        e_steps: g("rho_expected_steps"),
        rho_cpu: g("rho_cpu_s_per_dlp"),
        cap: g("cap_cpu_s"),
        seed_str: kv.get("protocol_canonical_sha256").cloned().unwrap_or_else(|| die("no protocol sha")),
    }
}

// ---------- RNG (splitmix64 seeded by FNV-1a of a string) ----------
struct Rng(u64);
impl Rng {
    fn new(s: &str) -> Rng {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in s.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        Rng(h)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

// ---------- statistics ----------
fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}
fn median(xs: &[f64]) -> f64 {
    let mut s = xs.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = s.len();
    if k % 2 == 1 { s[k / 2] } else { (s[k / 2 - 1] + s[k / 2]) / 2.0 }
}
fn quantile(sorted: &[f64], q: f64) -> f64 {
    let pos = q * (sorted.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    sorted[lo] + (sorted[hi] - sorted[lo]) * (pos - lo as f64)
}

/// Two-sided Wilcoxon signed-rank; zeros dropped; exact without ties, normal
/// approximation with tie and continuity correction otherwise (as analyze.py v1).
fn wilcoxon(d: &[f64]) -> (f64, usize, bool) {
    let d: Vec<f64> = d.iter().cloned().filter(|x| *x != 0.0).collect();
    let n = d.len();
    if n == 0 {
        return (1.0, 0, false);
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| d[a].abs().partial_cmp(&d[b].abs()).unwrap());
    let mut ranks = vec![0.0; n];
    let mut ties = false;
    let mut i = 0;
    let mut tie_term = 0.0;
    while i < n {
        let mut j = i;
        while j + 1 < n && d[order[j + 1]].abs() == d[order[i]].abs() {
            j += 1;
        }
        if j > i {
            ties = true;
            let t = (j - i + 1) as f64;
            tie_term += t * t * t - t;
        }
        for k in i..=j {
            ranks[order[k]] = (i + j) as f64 / 2.0 + 1.0;
        }
        i = j + 1;
    }
    let w_plus: f64 = ranks.iter().zip(&d).filter(|(_, x)| **x > 0.0).map(|(r, _)| *r).sum();
    let total = (n * (n + 1) / 2) as f64;
    if !ties {
        let t = total as usize;
        let mut counts = vec![0f64; t + 1];
        counts[0] = 1.0;
        for r in 1..=n {
            for s in (r..=t).rev() {
                counts[s] += counts[s - r];
            }
        }
        let denom = 2f64.powi(n as i32);
        let w = w_plus.min(total - w_plus);
        let tail: f64 = counts[..=(w.floor() as usize)].iter().sum::<f64>() / denom;
        return ((2.0 * tail).min(1.0), n, false);
    }
    let m = total / 2.0;
    let var = (n * (n + 1) * (2 * n + 1)) as f64 / 24.0 - tie_term / 48.0;
    if var <= 0.0 {
        return (1.0, n, true);
    }
    let z = ((w_plus - m).abs() - 0.5) / var.sqrt();
    (erfc(z.max(0.0) / 2f64.sqrt()).min(1.0), n, true)
}

fn erfc(x: f64) -> f64 {
    // Numerical Recipes erfcc (fractional error < 1.2e-7).
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let r = t * (-z * z - 1.26551223
        + t * (1.00002368
            + t * (0.37409196
                + t * (0.09678418
                    + t * (-0.18628806
                        + t * (0.27886807 + t * (-1.13520398 + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))))
        .exp();
    if x >= 0.0 { r } else { 2.0 - r }
}

// ---------- per-target tables ----------
#[derive(Clone)]
struct T {
    stratum: String, // sat | unsat
    cpu: f64,
    rel: f64,
    ops: f64,
    internal: f64,
    proj: f64,
}

/// Stratified rate: p*mean_SAT(rel) / (p*mean_SAT(cpu) + (1-p)*mean_UNSAT(cpu)).
fn strat_rate(p: f64, sat: &[&T], uns: &[&T], cpu: fn(&T) -> f64) -> f64 {
    let rs = sat.iter().map(|t| t.rel).sum::<f64>() / sat.len() as f64;
    let cs = sat.iter().map(|t| cpu(t)).sum::<f64>() / sat.len() as f64;
    let cu = uns.iter().map(|t| cpu(t)).sum::<f64>() / uns.len() as f64;
    p * rs / (p * cs + (1.0 - p) * cu)
}
fn strat_mean(p: f64, sat: &[&T], uns: &[&T], f: fn(&T) -> f64) -> f64 {
    p * sat.iter().map(|t| f(t)).sum::<f64>() / sat.len() as f64
        + (1.0 - p) * uns.iter().map(|t| f(t)).sum::<f64>() / uns.len() as f64
}

fn split<'a>(m: &'a BTreeMap<String, T>, keys: &[String]) -> (Vec<&'a T>, Vec<&'a T>) {
    let s = keys.iter().map(|k| &m[k]).filter(|t| t.stratum == "sat").collect();
    let u = keys.iter().map(|k| &m[k]).filter(|t| t.stratum == "unsat").collect();
    (s, u)
}

struct Strata {
    sat: Vec<String>,
    unsat: Vec<String>,
}

fn resample(rng: &mut Rng, st: &Strata) -> Vec<String> {
    let mut v = Vec::with_capacity(st.sat.len() + st.unsat.len());
    for _ in 0..st.sat.len() {
        v.push(st.sat[rng.below(st.sat.len())].clone());
    }
    for _ in 0..st.unsat.len() {
        v.push(st.unsat[rng.below(st.unsat.len())].clone());
    }
    v
}

fn ci(mut xs: Vec<f64>, a: f64) -> (f64, f64) {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (quantile(&xs, a / 2.0), quantile(&xs, 1.0 - a / 2.0))
}

fn fmt(x: f64) -> String {
    if x.is_finite() { format!("{x:.6e}") } else { "null".into() }
}
fn fpair(p: (f64, f64)) -> String {
    format!("[{}, {}]", fmt(p.0), fmt(p.1))
}

// rate with bootstrap CI over targets
fn rate_block(p: f64, m: &BTreeMap<String, T>, st: &Strata, seed: &str) -> String {
    let all: Vec<String> = st.sat.iter().chain(st.unsat.iter()).cloned().collect();
    let (s, u) = split(m, &all);
    let r = strat_rate(p, &s, &u, |t| t.cpu);
    let ri = strat_rate(p, &s, &u, |t| t.internal);
    let rp = strat_rate(p, &s, &u, |t| t.cpu + t.proj);
    let mut rng = Rng::new(seed);
    let mut b = Vec::with_capacity(BOOT);
    for _ in 0..BOOT {
        let k = resample(&mut rng, st);
        let (s, u) = split(m, &k);
        b.push(strat_rate(p, &s, &u, |t| t.cpu));
    }
    let c95 = ci(b.clone(), ALPHA);
    let cb = ci(b, ALPHA / FAMILY);
    let cpu_sat = mean(&s.iter().map(|t| t.cpu).collect::<Vec<_>>());
    let cpu_uns = mean(&u.iter().map(|t| t.cpu).collect::<Vec<_>>());
    let rel_sat = mean(&s.iter().map(|t| t.rel).collect::<Vec<_>>());
    format!(
        "{{\"rel_per_cpu_s\": {}, \"ci95\": {}, \"ci_bonferroni_0.05/3\": {}, \"rel_per_cpu_s_internal_timer\": {}, \"rel_per_cpu_s_incl_projection\": {}, \"mean_cpu_s_sat\": {}, \"mean_cpu_s_unsat\": {}, \"mean_rel_sat\": {}, \"cpu_s_per_verified_relation\": {}, \"stratified_native_ops_per_target\": {}}}",
        fmt(r), fpair(c95), fpair(cb), fmt(ri), fmt(rp), fmt(cpu_sat), fmt(cpu_uns), fmt(rel_sat), fmt(1.0 / r),
        fmt(strat_mean(p, &s, &u, |t| t.ops))
    )
}

/// Paired comparison of two per-target tables: log2(cpu_a/cpu_b) per target.
fn pair_block(p: f64, a: &BTreeMap<String, T>, b: &BTreeMap<String, T>, st: &Strata, seed: &str) -> (String, f64, (f64, f64), f64, usize) {
    let all: Vec<String> = st.sat.iter().chain(st.unsat.iter()).cloned().collect();
    let d: Vec<f64> = all.iter().map(|k| (a[k].cpu.max(1e-6)).log2() - (b[k].cpu.max(1e-6)).log2()).collect();
    let (pv, n_eff, ties) = wilcoxon(&d);
    let med = median(&d);
    let dmap: BTreeMap<String, f64> = all.iter().cloned().zip(d.iter().cloned()).collect();
    let mut rng = Rng::new(seed);
    let mut bm = Vec::with_capacity(BOOT);
    let mut br = Vec::with_capacity(BOOT);
    for _ in 0..BOOT {
        let k = resample(&mut rng, st);
        let dd: Vec<f64> = k.iter().map(|x| dmap[x]).collect();
        bm.push(median(&dd));
        let (sa, ua) = split(a, &k);
        let (sb, ub) = split(b, &k);
        br.push(strat_rate(p, &sa, &ua, |t| t.cpu) / strat_rate(p, &sb, &ub, |t| t.cpu));
    }
    let c95 = ci(bm.clone(), ALPHA);
    let cb = ci(bm, ALPHA / FAMILY);
    let (sa, ua) = split(a, &all);
    let (sb, ub) = split(b, &all);
    let rr = strat_rate(p, &sa, &ua, |t| t.cpu) / strat_rate(p, &sb, &ub, |t| t.cpu);
    let rr95 = ci(br.clone(), ALPHA);
    let rrb = ci(br, ALPHA / FAMILY);
    let n_neg = d.iter().filter(|x| **x < 0.0).count();
    let n_pos = d.iter().filter(|x| **x > 0.0).count();
    let s = format!(
        "{{\"n\": {}, \"n_eff\": {}, \"ties_present\": {}, \"median_log2_cpu_ratio_a_over_b\": {}, \"median_ci95\": {}, \"median_ci_bonferroni\": {}, \"wilcoxon_p_two_sided\": {}, \"targets_a_cheaper\": {}, \"targets_b_cheaper\": {}, \"rate_ratio_a_over_b\": {}, \"rate_ratio_ci95\": {}, \"rate_ratio_ci_bonferroni\": {}}}",
        d.len(), n_eff, ties, fmt(med), fpair(c95), fpair(cb), fmt(pv), n_neg, n_pos, fmt(rr), fpair(rr95), fpair(rrb)
    );
    (s, med, c95, pv, n_eff)
}

fn per_label(rows: &[Row]) -> BTreeMap<String, BTreeMap<String, T>> {
    let mut m: BTreeMap<String, BTreeMap<String, T>> = BTreeMap::new();
    for r in rows.iter().filter(|r| !r.excluded && (r.stratum == "sat" || r.stratum == "unsat")) {
        let cpu = if r.outcome == "censored" { f64::NAN } else { r.charged };
        m.entry(r.arm.clone()).or_default().insert(
            r.inst.clone(),
            T { stratum: r.stratum.clone(), cpu, rel: r.n_verified, ops: r.native_ops, internal: r.internal, proj: r.projection },
        );
    }
    m
}

fn strata(m: &BTreeMap<String, T>) -> Strata {
    Strata {
        sat: m.iter().filter(|(_, t)| t.stratum == "sat").map(|(k, _)| k.clone()).collect(),
        unsat: m.iter().filter(|(_, t)| t.stratum == "unsat").map(|(k, _)| k.clone()).collect(),
    }
}

fn label_descriptives(rows: &[Row], label: &str, cap: f64) -> String {
    let rs: Vec<&Row> = rows.iter().filter(|r| r.arm == label && !r.excluded).collect();
    let nat: Vec<&Row> = rs.iter().cloned().filter(|r| r.stratum == "sat" || r.stratum == "unsat").collect();
    let planted: Vec<&Row> = rs.iter().cloned().filter(|r| r.stratum == "planted").collect();
    let censored = rs.iter().filter(|r| r.outcome == "censored").count();
    let at_cap = rs.iter().filter(|r| r.charged >= cap).count();
    let wd = rs.iter().filter(|r| r.watchdog).count();
    let maxdeg = rs.iter().filter_map(|r| r.degree).max();
    let maxmm = rs.iter().filter_map(|r| r.mm).max_by_key(|(a, b)| a * b);
    let maxrss = rs.iter().map(|r| r.rss).fold(0.0, f64::max);
    let spur: f64 = rs.iter().map(|r| r.spurious).sum();
    let mism = rs.iter().filter(|r| !r.matches_oracle).count();
    let m1 = rs.iter().filter(|r| r.m1).count();
    let pf = planted.iter().filter(|r| r.planted_found == Some(true)).count();
    let false_rel = nat.iter().filter(|r| r.oracle_count == 0.0 && r.n_verified > 0.0).count();
    let zero = rs.iter().map(|r| r.zero_abs).sum::<f64>();
    let maxcpu = rs.iter().map(|r| r.charged).fold(0.0, f64::max);
    let medcpu = median(&nat.iter().map(|r| r.charged).collect::<Vec<_>>());
    format!(
        "{{\"rows_not_excluded\": {}, \"natural\": {}, \"planted\": {}, \"planted_found\": {}, \"censored\": {}, \"at_or_over_cap\": {}, \"watchdog_killed\": {}, \"max_degree\": {}, \"max_matrix\": {}, \"max_peak_rss_bytes\": {}, \"spurious_roots_sum\": {}, \"rows_not_matching_oracle\": {}, \"m1_violations\": {}, \"false_relations_on_oracle_zero\": {}, \"zero_abscissa_roots_sum\": {}, \"max_charged_cpu_s\": {}, \"median_charged_cpu_s_natural\": {}}}",
        rs.len(), nat.len(), planted.len(), pf, censored, at_cap, wd,
        maxdeg.map(|x| x.to_string()).unwrap_or("null".into()),
        maxmm.map(|(a, b)| format!("\"{a}x{b}\"")).unwrap_or("null".into()),
        maxrss as u64, spur, mism, m1, false_rel, zero, fmt(maxcpu), fmt(medcpu)
    )
}

fn write_once(path: &Path, s: &str) {
    if path.exists() {
        die(&format!("refusing to overwrite {}", path.display()));
    }
    let mut f = fs::File::create(path).unwrap();
    f.write_all(s.as_bytes()).unwrap();
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 5 {
        die("usage: blind <rows.tsv> <params.tsv> <outdir> | unmask <rows.tsv> <params.tsv> <key.tsv> <outdir>");
    }
    let rows = load_rows(&a[2]);
    let pr = params(&a[3]);
    let censored_present = rows.iter().any(|r| r.outcome == "censored");
    if censored_present {
        // the frozen rule charges censored calls at the cap and drops both-censored pairs;
        // this run has none, so that branch is not implemented and the script fails closed.
        die("censored rows present: both-censored handling not implemented (fail closed)");
    }
    let labels = per_label(&rows);
    let first = labels.values().next().unwrap();
    let st = strata(first);
    for (l, m) in &labels {
        let s2 = strata(m);
        if s2.sat != st.sat || s2.unsat != st.unsat {
            die(&format!("label {l} target set differs"));
        }
    }
    match a[1].as_str() {
        "blind" => {
            let out = Path::new(&a[4]);
            let mut s = String::new();
            s += &format!("{{\"phase\": \"1 (blind, masked labels)\", \"protocol_canonical_sha256\": \"{}\", \"p_sat\": {}, \"n_sat\": {}, \"n_unsat\": {}, \"bootstrap\": {}, \"labels\": {{\n", pr.seed_str, fmt(pr.p_sat), st.sat.len(), st.unsat.len(), BOOT);
            let ls: Vec<&String> = labels.keys().collect();
            for (i, l) in ls.iter().enumerate() {
                s += &format!("  \"{}\": {{\"rate\": {}, \"descriptive\": {}}}{}\n", l,
                    rate_block(pr.p_sat, &labels[*l], &st, &format!("{}:rate:{}", pr.seed_str, l)),
                    label_descriptives(&rows, l, pr.cap), if i + 1 < ls.len() { "," } else { "" });
            }
            s += "}, \"label_pairs\": {\n";
            let mut items = vec![];
            for i in 0..ls.len() {
                for j in i + 1..ls.len() {
                    let (blk, ..) = pair_block(pr.p_sat, &labels[ls[i]], &labels[ls[j]], &st, &format!("{}:pair:{}:{}", pr.seed_str, ls[i], ls[j]));
                    items.push(format!("  \"{}|{}\": {}", ls[i], ls[j], blk));
                }
            }
            s += &items.join(",\n");
            s += "\n}}\n";
            write_once(&out.join("summary.json"), &s);
            println!("phase 1 (blind) written: {}", out.join("summary.json").display());
        }
        "unmask" => {
            let out = Path::new(&a[5]);
            if !out.join("summary.json").exists() {
                die("phase 2 needs the blind phase-1 summary.json first");
            }
            let key = load_kv(&a[4]);
            let lab = |name: &str| key.get(name).cloned().unwrap_or_else(|| die(&format!("key lacks {name}")));
            // Hard-coded pipelines (pass A arm, pass B = A/A arm) and registered comparisons (M3).
            let pipelines = ["msolve-f4", "cms-xor", "inherited-f4"];
            let registered = [("C-msolve-cms", "msolve-f4", "cms-xor"), ("C-inh-msolve", "inherited-f4", "msolve-f4"), ("C-inh-cms", "inherited-f4", "cms-xor")];
            let mut pipe: BTreeMap<&str, BTreeMap<String, T>> = BTreeMap::new();
            let mut aa_floor: BTreeMap<&str, f64> = BTreeMap::new();
            let mut s = format!("{{\"phase\": \"2 (unmasked)\", \"label\": \"LOCAL DIAGNOSTIC, non-admitted, [pcc:unverified]; diagnostic tier (local, non-admitted)\", \"post_data_script\": true, \"p_sat\": {}, \"pipelines\": {{\n", fmt(pr.p_sat));
            let mut pitems = vec![];
            let mut aa_items = vec![];
            for p in pipelines {
                let la = lab(p);
                let lb = lab(&format!("{p}-aa"));
                let (ta, tb) = (&labels[&la], &labels[&lb]);
                // A/A: pass A vs pass B, paired log2 CPU
                let (aa_blk, _, c95, _, _) = pair_block(pr.p_sat, ta, tb, &st, &format!("{}:aa:{}", pr.seed_str, p));
                let floor = c95.0.abs().max(c95.1.abs());
                aa_floor.insert(p, floor);
                let same_ops = ta.iter().filter(|(k, t)| tb[*k].ops == t.ops).count();
                aa_items.push(format!("  \"{p}\": {{\"pass_a_label\": \"{la}\", \"pass_b_label\": \"{lb}\", \"aa\": {aa_blk}, \"aa_floor_log2\": {}, \"native_ops_identical_targets\": {}, \"targets\": {}}}", fmt(floor), same_ops, ta.len()));
                // pipeline per target = mean of passes
                let mut m = BTreeMap::new();
                for (k, t) in ta {
                    let u = &tb[k];
                    if u.rel != t.rel {
                        die(&format!("{p} {k}: relation count differs between passes"));
                    }
                    m.insert(k.clone(), T { stratum: t.stratum.clone(), cpu: (t.cpu + u.cpu) / 2.0, rel: t.rel, ops: (t.ops + u.ops) / 2.0, internal: (t.internal + u.internal) / 2.0, proj: (t.proj + u.proj) / 2.0 });
                }
                // rho band (C3)
                let all: Vec<String> = st.sat.iter().chain(st.unsat.iter()).cloned().collect();
                let (ss, uu) = split(&m, &all);
                let rel_per_target = pr.p_sat * mean(&ss.iter().map(|t| t.rel).collect::<Vec<_>>());
                let n_arm = strat_mean(pr.p_sat, &ss, &uu, |t| t.ops);
                let rate = strat_rate(pr.p_sat, &ss, &uu, |t| t.cpu);
                let needed = pr.fb + MARGIN;
                let mut readings = vec![];
                let mut ratios = vec![];
                for w in BAND {
                    let c_arm = needed / rel_per_target * w * n_arm;
                    let c_rho = pr.a_rho + w * pr.u_rho;
                    let c_rho_e = pr.e_steps + w * pr.u_rho;
                    let r = c_arm / c_rho;
                    ratios.push(r);
                    readings.push(format!("\"w={w}\": {{\"C_arm\": {}, \"C_rho_measured\": {}, \"ratio\": {}, \"log2_distance\": {}, \"C_rho_expected_steps\": {}, \"ratio_expected\": {}}}", fmt(c_arm), fmt(c_rho), fmt(r), fmt(r.log2()), fmt(c_rho_e), fmt(c_arm / c_rho_e)));
                }
                let clock = needed / rate / pr.rho_cpu;
                ratios.push(clock);
                // clock ratio CI from rate bootstrap
                let mut rng = Rng::new(&format!("{}:clock:{}", pr.seed_str, p));
                let mut bc = Vec::with_capacity(BOOT);
                for _ in 0..BOOT {
                    let k = resample(&mut rng, &st);
                    let (s2, u2) = split(&m, &k);
                    bc.push(needed / strat_rate(pr.p_sat, &s2, &u2, |t| t.cpu) / pr.rho_cpu);
                }
                let cc = ci(bc, ALPHA);
                readings.push(format!("\"clock\": {{\"ratio\": {}, \"ci95\": {}, \"log2_distance\": {}, \"log2_ci95\": [{}, {}], \"note\": \"diagnostic, IC lower bound\"}}", fmt(clock), fpair(cc), fmt(clock.log2()), fmt(cc.0.log2()), fmt(cc.1.log2())));
                let verdict = if ratios.iter().all(|r| *r > 1.0) { "above matched rho at every reading" } else { "unresolved in band" };
                pitems.push(format!("  \"{p}\": {{\"rate\": {}, \"descriptive_pass_a\": {}, \"descriptive_pass_b\": {}, \"relations_per_target_stratified\": {}, \"native_ops_per_target_stratified\": {}, \"projected_relations\": {}, \"rho\": {{{}}}, \"rho_verdict\": \"{}; diagnostic tier (local, non-admitted)\", \"abandon_check_clock_log2_gt_20\": {}}}",
                    rate_block(pr.p_sat, &m, &st, &format!("{}:pipe:{}", pr.seed_str, p)),
                    label_descriptives(&rows, &la, pr.cap), label_descriptives(&rows, &lb, pr.cap),
                    fmt(rel_per_target), fmt(n_arm), needed, readings.join(", "), verdict, clock.log2() > 20.0));
                pipe.insert(p, m);
            }
            s += &pitems.join(",\n");
            s += "\n}, \"aa\": {\n";
            s += &aa_items.join(",\n");
            s += "\n}, \"rho_reference\": {";
            s += &format!("\"A_rho_mean_walk_ga\": {}, \"U_rho_hashes_plus_canon\": {}, \"E_steps\": {}, \"cpu_s_per_dlp\": {}}}", fmt(pr.a_rho), fmt(pr.u_rho), fmt(pr.e_steps), fmt(pr.rho_cpu));
            s += ", \"registered\": {\n";
            let mut ritems = vec![];
            let mut tsv = String::from("comparison\ta\tb\tn_eff\tmedian_log2_cpu_a_over_b\tci95_lo\tci95_hi\tp\taa_floor\trate_ratio_a_over_b\tverdict\n");
            for (id, x, y) in registered {
                let (blk, med, c95, pv, n_eff) = pair_block(pr.p_sat, &pipe[x], &pipe[y], &st, &format!("{}:reg:{}", pr.seed_str, id));
                let floor = aa_floor[x].max(aa_floor[y]);
                let v = if n_eff < 6 {
                    "unresolved".to_string()
                } else if pv < ALPHA / FAMILY && med.abs() > floor {
                    format!("{} uses less CPU per target", if med < 0.0 { x } else { y })
                } else {
                    "within A/A noise or not significant".to_string()
                };
                ritems.push(format!("  \"{id}\": {{\"a\": \"{x}\", \"b\": \"{y}\", \"stats\": {blk}, \"aa_floor_log2_max_of_both\": {}, \"verdict\": \"{v}; diagnostic tier (local, non-admitted)\"}}", fmt(floor)));
                let rr = strat_rate(pr.p_sat, &split(&pipe[x], &st.sat.iter().chain(st.unsat.iter()).cloned().collect::<Vec<_>>()).0, &split(&pipe[x], &st.sat.iter().chain(st.unsat.iter()).cloned().collect::<Vec<_>>()).1, |t| t.cpu)
                    / strat_rate(pr.p_sat, &split(&pipe[y], &st.sat.iter().chain(st.unsat.iter()).cloned().collect::<Vec<_>>()).0, &split(&pipe[y], &st.sat.iter().chain(st.unsat.iter()).cloned().collect::<Vec<_>>()).1, |t| t.cpu);
                tsv += &format!("{id}\t{x}\t{y}\t{n_eff}\t{}\t{}\t{}\t{}\t{}\t{}\t{v}\n", fmt(med), fmt(c95.0), fmt(c95.1), fmt(pv), fmt(floor), fmt(rr));
            }
            s += &ritems.join(",\n");
            s += "\n}}\n";
            write_once(&out.join("summary.unblinded.json"), &s);
            write_once(&out.join("registered.tsv"), &tsv);
            // per-target table for plotting (layout only downstream)
            let mut pt = String::from("pipeline\ttarget\tstratum\tcpu_pass_a\tcpu_pass_b\trelations\tnative_ops_a\tnative_ops_b\tposition_a\tposition_b\n");
            for p in pipelines {
                let (la, lb) = (lab(p), lab(&format!("{p}-aa")));
                for r in rows.iter().filter(|r| r.arm == la && !r.excluded && (r.stratum == "sat" || r.stratum == "unsat")) {
                    let rb = rows.iter().find(|q| q.arm == lb && q.inst == r.inst).unwrap();
                    pt += &format!("{p}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", r.inst, r.stratum, r.charged, rb.charged, r.n_verified, r.native_ops, rb.native_ops, r.position, rb.position);
                }
            }
            write_once(&out.join("per_target.tsv"), &pt);
            // order-position effect (M2): median charged CPU by position, per pipeline and pass
            let mut op = String::from("pipeline\tpass\tposition\tn\tmedian_charged_cpu_s\n");
            for p in pipelines {
                for (pass_lab, pass) in [(lab(p), "A"), (lab(&format!("{p}-aa")), "B")] {
                    let positions: BTreeSet<i64> = rows.iter().filter(|r| r.arm == pass_lab).map(|r| r.position).collect();
                    for pos in positions {
                        let xs: Vec<f64> = rows.iter().filter(|r| r.arm == pass_lab && !r.excluded && r.position == pos && (r.stratum == "sat" || r.stratum == "unsat")).map(|r| r.charged).collect();
                        if !xs.is_empty() {
                            op += &format!("{p}\t{pass}\t{pos}\t{}\t{}\n", xs.len(), fmt(median(&xs)));
                        }
                    }
                }
            }
            write_once(&out.join("order_position.tsv"), &op);
            println!("phase 2 written: {}", out.join("summary.unblinded.json").display());
        }
        _ => die("unknown phase"),
    }
    let _ = pr.cap;
}
