//! Fixed analysis program (decision rule frozen in PROTOCOL.md section 7). Reads rows_*.tsv,
//! writes summary.tsv, summary.md and fig_cost_ratio.svg. No analyst choices: roles are hard-coded.

use crate::bases::Rng;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

type Row = HashMap<String, String>;
type CellKey = (u32, u32, usize, u32);

pub fn load(dir: &str) -> Result<BTreeMap<CellKey, Vec<Row>>, String> {
    let mut out: BTreeMap<CellKey, Vec<Row>> = BTreeMap::new();
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let p = e.map_err(|e| e.to_string())?.path();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if !(name.starts_with("rows_") && name.ends_with(".tsv")) {
            continue;
        }
        let txt = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
        let mut lines = txt.lines();
        let hdr: Vec<&str> = match lines.next() {
            Some(h) => h.split('\t').collect(),
            None => continue,
        };
        for ln in lines {
            let r: Row = hdr
                .iter()
                .zip(ln.split('\t'))
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            let key = (
                r["a"].parse().unwrap(),
                r["n"].parse().unwrap(),
                r["m"].parse().unwrap(),
                r["l"].parse().unwrap(),
            );
            out.entry(key).or_default().push(r);
        }
    }
    Ok(out)
}

/// plan.tsv: one line per planned cell, "a n m l reps" (written by `g4iota cells`).
pub fn load_plan(dir: &str) -> Result<Vec<(CellKey, usize)>, String> {
    let txt = std::fs::read_to_string(format!("{dir}/plan.tsv"))
        .map_err(|e| format!("plan.tsv is required to detect missing or partial cells: {e}"))?;
    let mut v = vec![];
    for ln in txt.lines().filter(|l| !l.trim().is_empty()) {
        let f: Vec<&str> = ln.split_whitespace().collect();
        if f.len() != 5 {
            return Err(format!("bad plan line `{ln}` (want: a n m l reps)"));
        }
        let p = |i: usize| f[i].parse::<u32>().map_err(|e| e.to_string());
        v.push(((p(0)?, p(1)?, p(2)? as usize, p(3)?), p(4)? as usize));
    }
    Ok(v)
}

fn col(rows: &[Row], arm: &str, half: &str, k: &str) -> Vec<f64> {
    rows.iter()
        .filter(|r| r["arm"] == arm && r["half"] == half)
        .filter_map(|r| r.get(k).and_then(|v| v.parse::<f64>().ok()))
        .filter(|v| v.is_finite())
        .collect()
}
fn count(rows: &[Row], arm: &str, half: &str) -> usize {
    rows.iter()
        .filter(|r| r["arm"] == arm && r["half"] == half)
        .count()
}
fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        f64::NAN
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ci {
    pub est: f64,
    pub lo: f64,
    pub hi: f64,
}
const NAN_CI: Ci = Ci {
    est: f64::NAN,
    lo: f64::NAN,
    hi: f64::NAN,
};
fn pct(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    (
        v[(0.025 * n as f64) as usize],
        v[((0.975 * n as f64) as usize).min(n - 1)],
    )
}
const NB: usize = 2000;
pub fn boot_mean(v: &[f64], seed: u64) -> Ci {
    if v.is_empty() {
        return NAN_CI;
    }
    let mut rng = Rng(seed);
    let bs: Vec<f64> = (0..NB)
        .map(|_| {
            mean(
                &(0..v.len())
                    .map(|_| v[rng.below(v.len() as u64) as usize])
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let (lo, hi) = pct(bs);
    Ci {
        est: mean(v),
        lo,
        hi,
    }
}
/// Ratio of means (num/den), both resampled independently.
pub fn boot_ratio(num: &[f64], den: &[f64], seed: u64) -> Ci {
    if num.is_empty() || den.is_empty() {
        return NAN_CI;
    }
    let mut rng = Rng(seed);
    let rs = |v: &[f64], rng: &mut Rng| {
        mean(
            &(0..v.len())
                .map(|_| v[rng.below(v.len() as u64) as usize])
                .collect::<Vec<_>>(),
        )
    };
    let bs: Vec<f64> = (0..NB)
        .map(|_| rs(num, &mut rng) / rs(den, &mut rng))
        .collect();
    let (lo, hi) = pct(bs);
    Ci {
        est: mean(num) / mean(den),
        lo,
        hi,
    }
}
/// Paired version: both series are indexed by replicate and every replicate is exact-matched on the item
/// count, so the bootstrap resamples replicates jointly (the shared N cancels inside each pair). Only
/// replicates present in both series are used (a censored base drops its pair; the CENSORED gate bounds it).
pub fn boot_ratio_paired(num: &BTreeMap<usize, f64>, den: &BTreeMap<usize, f64>, seed: u64) -> Ci {
    let pairs: Vec<(f64, f64)> = num
        .iter()
        .filter_map(|(k, a)| den.get(k).map(|b| (*a, *b)))
        .collect();
    if pairs.is_empty() {
        return NAN_CI;
    }
    let mut rng = Rng(seed);
    let rs = |rng: &mut Rng| {
        let (mut sa, mut sb) = (0.0, 0.0);
        for _ in 0..pairs.len() {
            let (a, b) = pairs[rng.below(pairs.len() as u64) as usize];
            sa += a;
            sb += b;
        }
        sa / sb
    };
    let bs: Vec<f64> = (0..NB).map(|_| rs(&mut rng)).collect();
    let (lo, hi) = pct(bs);
    let (sa, sb) = pairs
        .iter()
        .fold((0.0, 0.0), |acc, p| (acc.0 + p.0, acc.1 + p.1));
    Ci {
        est: sa / sb,
        lo,
        hi,
    }
}
fn series(rows: &[Row], arm: &str, half: &str, k: &str) -> BTreeMap<usize, f64> {
    rows.iter()
        .filter(|r| r["arm"] == arm && r["half"] == half)
        .filter_map(|r| {
            Some((
                r["rep"].parse::<usize>().ok()?,
                r.get(k)?.parse::<f64>().ok()?,
            ))
        })
        .filter(|(_, v)| v.is_finite())
        .collect()
}
fn f3(c: Ci) -> String {
    format!("{:.3} [{:.3}, {:.3}]", c.est, c.lo, c.hi)
}
/// PARITY needs the whole interval inside this FIXED margin. It must not widen with the A/A noise floor:
/// a noisy cell would otherwise be called parity by default (a unit test plants exactly that).
pub const PARITY_MARGIN: f64 = 0.10;
/// GAIN / LOSS need the interval to clear 1 by delta = max(0.05, A/A half-width) (noise makes them harder).
pub fn classify(r: Ci, delta: f64) -> &'static str {
    if !r.est.is_finite() || !r.lo.is_finite() || !r.hi.is_finite() {
        "NO_DATA"
    } else if r.lo > 1.0 + delta {
        "GAIN"
    } else if r.hi < 1.0 / (1.0 + delta) {
        "LOSS"
    } else if r.lo >= 1.0 / (1.0 + PARITY_MARGIN) && r.hi <= 1.0 + PARITY_MARGIN {
        "PARITY"
    } else {
        "UNRESOLVED"
    }
}

/// Everything the decision rule says about one cell. Built only from the raw rows.
#[derive(Clone, Debug)]
pub struct Judge {
    pub completeness: String,
    pub instr: String,
    pub delta: f64,
    pub size_ratio: f64,
    pub col_ratio: f64,
    pub cens_v: f64,
    pub cens_g: f64,
    pub sat_v: f64,
    pub sat_g: f64,
    pub aa_flags: usize,
    pub r_best: Ci,
    pub r_blind: Ci,
    pub r_tbest: Ci,
    pub r_tblind: Ci,
    pub q_best: Ci,
    pub q_blind: Ci,
    pub null_rw: Ci,
    pub null_vw: Ci,
    pub null_vr: Ci,
    /// final verdict strings (after void conditions)
    pub v_best: String,
    pub v_blind: String,
    pub v_tbest: String,
    pub v_tblind: String,
    pub v_q_best: String,
    pub v_null: String,
    pub license: bool,
}

fn completeness(rows: &[Row], reps: usize) -> String {
    if rows.is_empty() {
        return "NOT_RUN".into();
    }
    let mut bad = vec![];
    for arm in crate::run::MATCHED_ARMS {
        for half in ["main", "aa"] {
            let seen: BTreeSet<usize> = rows
                .iter()
                .filter(|r| r["arm"] == arm && r["half"] == half)
                .filter_map(|r| r["rep"].parse().ok())
                .collect();
            let n = count(rows, arm, half);
            if n != reps || seen != (0..reps).collect::<BTreeSet<usize>>() {
                bad.push(format!("{arm}/{half}:{n}of{reps}"));
            }
        }
    }
    if bad.is_empty() {
        "COMPLETE".into()
    } else {
        format!("INCOMPLETE({})", bad.join(","))
    }
}

/// True iff every replicate has the same item count in every matched arm (exact |F| matching).
fn exactly_matched(rows: &[Row]) -> bool {
    let mut by: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
    for r in rows
        .iter()
        .filter(|r| crate::run::MATCHED_ARMS.contains(&r["arm"].as_str()))
    {
        by.entry((r["half"].clone(), r["rep"].clone()))
            .or_default()
            .insert(r["nitems"].clone());
    }
    by.values().all(|s| s.len() == 1)
}

fn hw(c: Ci) -> f64 {
    if c.est.is_finite() {
        (c.hi - 1.0).max(1.0 - c.lo)
    } else {
        0.0
    }
}
fn excludes_one(c: Ci) -> bool {
    c.est.is_finite() && (c.lo > 1.0 || c.hi < 1.0)
}

pub fn judge_cell(rows: &[Row], key: CellKey, reps: usize) -> Judge {
    let (a, n, m, _l) = key;
    let comp = completeness(rows, reps);
    let mut j = Judge {
        completeness: comp.clone(),
        instr: "n/a".into(),
        delta: f64::NAN,
        size_ratio: f64::NAN,
        col_ratio: f64::NAN,
        cens_v: f64::NAN,
        cens_g: f64::NAN,
        sat_v: f64::NAN,
        sat_g: f64::NAN,
        aa_flags: 0,
        r_best: NAN_CI,
        r_blind: NAN_CI,
        r_tbest: NAN_CI,
        r_tblind: NAN_CI,
        q_best: NAN_CI,
        q_blind: NAN_CI,
        null_rw: NAN_CI,
        null_vw: NAN_CI,
        null_vr: NAN_CI,
        v_best: comp.clone(),
        v_blind: comp.clone(),
        v_tbest: comp.clone(),
        v_tblind: comp.clone(),
        v_q_best: comp.clone(),
        v_null: comp.clone(),
        license: false,
    };
    if comp != "COMPLETE" {
        return j;
    }
    // instrument and control checks (exact)
    let (mut ident, mut quot, mut clos) = (true, true, true);
    for arm in ["g4poly", "g4rand", "riota"] {
        for r in rows.iter().filter(|r| r["arm"] == arm) {
            ident &= r["ident_ok"] == "1";
            quot &= r["quot_ok"] == "1";
            clos &= r["closure"].parse::<f64>().unwrap_or(0.0) == 1.0;
        }
    }
    let max_clos = ["vrand", "rset", "wrong"]
        .iter()
        .flat_map(|a| {
            ["main", "aa"]
                .into_iter()
                .flat_map(move |h| col(rows, a, h, "closure"))
        })
        .fold(0.0f64, f64::max);
    let cm = |arm: &str| mean(&col(rows, arm, "main", "collapse"));
    let want = (1u64 << (m - 1)) as f64;
    let ctl_ok = (cm("g4rand") - want).abs() < 1e-9
        && (cm("riota") - want).abs() < 1e-9
        && cm("vrand") <= 1.02
        && cm("rset") <= 1.02
        && cm("wrong") <= 1.10
        && max_clos < 0.5;
    j.instr = if !(ident && quot && clos) {
        "INSTRUMENT_FAIL".into()
    } else if !ctl_ok {
        "CONTROL_FAIL".into()
    } else {
        "ok".into()
    };
    // series
    let c = |arm: &str, half: &str, k: &str| col(rows, arm, half, k);
    let ps = |na: &str, ka: &str, nb: &str, kb: &str, seed: u64| {
        boot_ratio_paired(
            &series(rows, na, "main", ka),
            &series(rows, nb, "main", kb),
            seed,
        )
    };
    let (vb, gb, gbest) = (
        c("vrand", "main", "cpr_blind"),
        c("g4rand", "main", "cpr_blind"),
        c("g4rand", "main", "cpr_best"),
    );
    j.r_blind = ps("vrand", "cpr_blind", "g4rand", "cpr_blind", 21);
    j.r_best = ps("vrand", "cpr_blind", "g4rand", "cpr_best", 22);
    j.r_tblind = ps("vrand", "tot_blind", "g4rand", "tot_blind", 25);
    j.r_tbest = ps("vrand", "tot_blind", "g4rand", "tot_best", 26);
    j.q_blind = ps("wrong", "cpr_blind", "g4rand", "cpr_blind", 27);
    j.q_best = ps("wrong", "cpr_blind", "g4rand", "cpr_best", 28);
    j.null_rw = ps("rset", "cpr_blind", "wrong", "cpr_blind", 29);
    j.null_vw = ps("vrand", "cpr_blind", "wrong", "cpr_blind", 33);
    j.null_vr = ps("vrand", "cpr_blind", "rset", "cpr_blind", 34);
    // A/A noise floors: same arm, seed-disjoint halves, same estimand; they also feed the false-alarm gate
    let aa_v = boot_ratio(&vb, &c("vrand", "aa", "cpr_blind"), 23);
    let aa_g = boot_ratio(&gbest, &c("g4rand", "aa", "cpr_best"), 24);
    j.aa_flags = excludes_one(aa_v) as usize + excludes_one(aa_g) as usize;
    j.delta = 0.05f64.max(hw(aa_v)).max(hw(aa_g));
    // censored base counts (integers); the registered rule is "more than 5 percent", i.e. censored * 20 > n
    let cens_cnt = |arm: &str, me: &str| -> (usize, usize) {
        let n = count(rows, arm, "main");
        (n - col(rows, arm, "main", me).len().min(n), n)
    };
    let cens = |arm: &str, me: &str| {
        let (k, n) = cens_cnt(arm, me);
        if n == 0 {
            1.0
        } else {
            k as f64 / n as f64
        }
    };
    j.cens_v = cens("vrand", "cpr_blind");
    j.cens_g = cens("g4rand", "cpr_best").max(cens("g4rand", "cpr_blind"));
    // the null/attribution arms (wrong, rset) are gated too: censored bases would silently drop pairs from Q and the null
    let cens_ctl = cens("wrong", "cpr_blind").max(cens("rset", "cpr_blind"));
    let over = |arm: &str, me: &str| censored_over_5pct(cens_cnt(arm, me));
    let censored = over("vrand", "cpr_blind")
        || over("g4rand", "cpr_best")
        || over("g4rand", "cpr_blind")
        || over("wrong", "cpr_blind")
        || over("rset", "cpr_blind");
    j.size_ratio = mean(&c("g4rand", "main", "nitems")) / mean(&c("vrand", "main", "nitems"));
    j.col_ratio = mean(&c("g4rand", "main", "active")) / mean(&c("vrand", "main", "active"));
    j.sat_v = mean(&c("vrand", "main", "sat"));
    j.sat_g = mean(&c("g4rand", "main", "sat"));
    let unmatched = !exactly_matched(rows);
    // right-censoring sensitivity: replace never-90% bases by their lower-bound cost; a class that
    // changes is demoted to UNRESOLVED
    let lb = |me: &str, arm: &str| c(arm, "main", me);
    let _ = lb;
    let r_best_lb = ps("vrand", "cpr_lb_blind", "g4rand", "cpr_lb_best", 22);
    let r_blind_lb = ps("vrand", "cpr_lb_blind", "g4rand", "cpr_lb_blind", 21);
    let d = j.delta;
    let fin = |cl: &'static str, cl_lb: Option<&'static str>| -> String {
        if unmatched {
            "UNMATCHED".into()
        } else if j.instr != "ok" {
            "VOID".into()
        } else if censored {
            "CENSORED".into()
        } else if let Some(l) = cl_lb {
            if l != cl {
                "UNRESOLVED".into()
            } else {
                cl.into()
            }
        } else {
            cl.into()
        }
    };
    j.v_best = fin(classify(j.r_best, d), Some(classify(r_best_lb, d)));
    j.v_blind = fin(classify(j.r_blind, d), Some(classify(r_blind_lb, d)));
    j.v_tbest = fin(classify(j.r_tbest, d), None);
    j.v_tblind = fin(classify(j.r_tblind, d), None);
    j.v_q_best = fin(classify(j.q_best, d), None);
    j.v_null = fin(classify(j.null_rw, d), None);
    let primary_ok = a == 0 && n >= 13;
    let _ = primary_ok;
    j.license =
        j.v_best == "GAIN" && j.v_q_best == "GAIN" && j.v_tbest != "LOSS" && j.v_tbest != "VOID";
    j
}

/// The registered CENSORED rule as an integer comparison: more than 5 percent censored, censored * 20 > n
/// (n = 0 counts as censored). Exactly 5 percent (2 of 40) is not censored.
pub fn censored_over_5pct((k, n): (usize, usize)) -> bool {
    n == 0 || k * 20 > n
}

/// Smallest t with P(Bin(k, p) > t) < alpha.
pub fn binom_threshold(k: usize, p: f64, alpha: f64) -> usize {
    let mut pmf = (1.0 - p).powi(k as i32);
    let mut cdf = pmf;
    for t in 0..k {
        if 1.0 - cdf < alpha {
            return t;
        }
        pmf *= (k - t) as f64 / (t + 1) as f64 * p / (1.0 - p);
        cdf += pmf;
    }
    k
}

pub fn analyze(dir: &str) -> Result<(), String> {
    let plan = load_plan(dir)?;
    let cells = load(dir)?;
    let mut md = String::new();
    let mut tsv = String::from("a\tn\tm\tl\tarm\thalf\treps\tmetric\tmean\tlo\thi\n");
    let mut verdicts = String::new();
    let mut body = String::new();
    let mut fig: Vec<(String, usize, Ci, Ci)> = vec![];
    let metrics = [
        "nx",
        "nitems",
        "nsc",
        "ncls",
        "lam_pred",
        "mean_d",
        "fano_d",
        "mean_o",
        "fano_o",
        "hit_d",
        "hit_pois_o",
        "collapse",
        "active",
        "sat",
        "rank_final",
        "t90",
        "t90f",
        "cpr_blind",
        "cpr_best",
        "tot_blind",
        "tot_best",
        "deg",
        "param_deg",
        "anf_terms",
        "closure",
        "enum_ms",
    ];
    writeln!(md, "# G4 iota-stable bases: run summary\n\nDirectory: `{dir}`. Every number below is computed by the fixed analysis program from the raw rows; intervals are 95% percentile bootstrap over bases (2000 resamples, seeded).\nThe decision rule is PROTOCOL.md section 7. Nothing here is a solver result or a rho comparison.\n").unwrap();
    let mut judged: Vec<(CellKey, Judge)> = vec![];
    for (key, reps) in &plan {
        let empty: Vec<Row> = vec![];
        let rows = cells.get(key).unwrap_or(&empty);
        judged.push((*key, judge_cell(rows, *key, *reps)));
    }
    // design-level gates
    let complete: Vec<&(CellKey, Judge)> = judged
        .iter()
        .filter(|(_, j)| j.completeness == "COMPLETE")
        .collect();
    let n_notrun = judged
        .iter()
        .filter(|(_, j)| j.completeness == "NOT_RUN")
        .count();
    let n_incomplete = judged
        .iter()
        .filter(|(_, j)| j.completeness.starts_with("INCOMPLETE"))
        .count();
    let aa_total: usize = complete.iter().map(|(_, j)| j.aa_flags).sum();
    let aa_thr = binom_threshold(2 * complete.len(), 0.10, 0.01);
    let aa_fail = aa_total > aa_thr;
    let n_bad = judged
        .iter()
        .filter(|(_, j)| {
            j.completeness != "COMPLETE"
                || ["UNMATCHED", "CENSORED", "VOID"].contains(&j.v_best.as_str())
        })
        .count();
    let under_resolved = n_bad * 4 > judged.len();
    writeln!(md, "## 0. Design-level gates\n\n* planned cells {}; complete {}; NOT_RUN {n_notrun}; INCOMPLETE {n_incomplete}.\n* A/A false-alarm gate: {aa_total} of {} A/A intervals exclude 1 (allowed at most {aa_thr}, Bin(k, 0.10) 99th percentile, conservative for a percentile bootstrap of 16 to 100 bases): {}.\n* Cells void, incomplete, not run, censored or unmatched: {n_bad} of {} ({}).\n",
        judged.len(), complete.len(), 2 * complete.len(), if aa_fail { "DESIGN CONTROL_FAIL: the noise floor is miscalibrated; every class below is void" } else { "ok" },
        judged.len(), if under_resolved { "more than a quarter: the design is reported as under-resolved" } else { "within the quarter allowed" }).unwrap();
    writeln!(md, "## 1. Instrument, control and completeness checks\n\n| cell (a,n,m,l) | completeness | instrument / controls | items G/V | A/A flags | censored V / G | status |\n|---|---|---|---|---|---|---|").unwrap();
    for (key, j) in &judged {
        let (a, n, m, l) = *key;
        let lab = format!("a{a} n{n} m{m} l{l}");
        writeln!(
            md,
            "| {lab} | {} | {} | {:.2} | {} | {:.2} / {:.2} | {} |",
            j.completeness,
            j.instr,
            j.size_ratio,
            j.aa_flags,
            j.cens_v,
            j.cens_g,
            if j.completeness != "COMPLETE" {
                j.completeness.split('(').next().unwrap().to_string()
            } else if j.instr != "ok" {
                j.instr.clone()
            } else {
                "ok".into()
            }
        )
        .unwrap();
    }
    writeln!(md, "\nstatus ok requires: every stable-arm base satisfies D = 2^(m-1) O at every target, the quotient search equals the orbit count, closure = 1; collapse(g4rand) = collapse(riota) = 2^(m-1) exactly (positive control); collapse of vrand and rset <= 1.02, of the wrong-involution control <= 1.10, and closure < 0.5 on every vrand, rset and wrong base (negative controls). A cell with fewer or more rows than plan.tsv demands is INCOMPLETE and gets no class.\n").unwrap();
    for (key, j) in &judged {
        let (a, n, m, l) = *key;
        let lab = format!("a{a} n{n} m{m} l{l}");
        let rows_empty: Vec<Row> = vec![];
        let rows = cells.get(key).unwrap_or(&rows_empty);
        if j.completeness == "COMPLETE" {
            for arm in crate::run::ARMS {
                for half in ["main", "aa"] {
                    for me in metrics {
                        let v = col(rows, arm, half, me);
                        if v.is_empty() {
                            continue;
                        }
                        let c = boot_mean(&v, 7);
                        writeln!(
                            tsv,
                            "{a}\t{n}\t{m}\t{l}\t{arm}\t{half}\t{}\t{me}\t{}\t{}\t{}",
                            v.len(),
                            c.est,
                            c.lo,
                            c.hi
                        )
                        .unwrap();
                    }
                }
            }
            writeln!(body, "### {lab}  (|G| targets = {})\n\n| arm | reps | nitems | lam_pred | mean D | Fano D | mean orbits | Fano orbits | hit rate | Poisson hit (orbit mean) | active cols | orbits per target / 0.9 active | t90 targets (fractional) | cpr_blind | cpr_best | tot_blind | tot_best | ANF deg | censored |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|",
                rows.iter().find(|r| r["arm"] == "vrand").map(|r| r["ntargets_g"].clone()).unwrap_or_default()).unwrap();
            for arm in crate::run::ARMS {
                let nrep = count(rows, arm, "main");
                if nrep == 0 {
                    continue;
                }
                let g = |me: &str| {
                    let v = col(rows, arm, "main", me);
                    if v.is_empty() {
                        "NA".to_string()
                    } else {
                        let c = boot_mean(&v, 11);
                        format!("{:.4} [{:.4}, {:.4}]", c.est, c.lo, c.hi)
                    }
                };
                let cens = nrep - col(rows, arm, "main", "cpr_blind").len();
                writeln!(body, "| {arm} | {nrep} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {cens} |",
                    g("nitems"), g("lam_pred"), g("mean_d"), g("fano_d"), g("mean_o"), g("fano_o"), g("hit_d"), g("hit_pois_o"), g("active"), g("sat"), g("t90f"), g("cpr_blind"), g("cpr_best"), g("tot_blind"), g("tot_best"), g("deg")).unwrap();
            }
            writeln!(body).unwrap();
        }
        let primary = n >= 13 && a == 0;
        let fin = |v: &String| {
            if aa_fail && j.completeness == "COMPLETE" {
                "VOID_AA".to_string()
            } else {
                v.clone()
            }
        };
        let attrib = if j.v_best == "UNRESOLVED"
            || !["GAIN", "LOSS", "PARITY"].contains(&j.v_best.as_str())
        {
            "-".to_string()
        } else if j.v_best == j.v_q_best {
            "stabiliser".to_string()
        } else {
            "construction/size (Q disagrees)".to_string()
        };
        let lic = j.license && !aa_fail;
        writeln!(verdicts, "| {lab} | {} | {:.2} | {:.2} | {:.2} / {:.2} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} / {} / {} | {:.3} | {attrib} | {} |", if primary { "primary" } else { "secondary" },
            j.size_ratio, j.col_ratio, j.sat_v, j.sat_g, f3(j.r_best), fin(&j.v_best), f3(j.r_blind), fin(&j.v_blind), f3(j.r_tbest), fin(&j.v_tbest), f3(j.r_tblind), fin(&j.v_tblind),
            format!("{} ({})", f3(j.q_best), fin(&j.v_q_best)), format!("{} ({})", f3(j.null_rw), fin(&j.v_null)), f3(j.null_vw), f3(j.null_vr), f3(j.q_blind), j.delta, if lic { "LICENSE" } else { "-" }).unwrap();
        fig.push((lab, m, j.r_best, j.r_blind));
    }
    writeln!(md, "## 2. Decision table\n\nV = vrand (subspace), G = g4rand (iota-stable), W = wrong-involution control, S = rset; all matched on the exact item count within a replicate (column `items G/V` must be 1.00). cpr = model search operations per independent relation, fractional targets, to 90% of the arm's own active columns. tot = model search operations in total to 90% rank (whole targets). R = V/G (R > 1: G cheaper). R_best: V blind versus G in the quotient; R_blind: the same blind search on both. Q_best = cpr_blind(W)/cpr_best(G): the same construction without the stabiliser against G (isolates the stabiliser); null = cpr_blind(S)/cpr_blind(W): the construction null, expected 1. delta = max(0.05, A/A half-width). Classes GAIN / LOSS / PARITY / UNRESOLVED per PROTOCOL.md section 7; CENSORED if more than 5% of a main-half arm never reached 90% rank (a class that changes when censored bases are replaced by their lower-bound cost becomes UNRESOLVED); UNMATCHED if any replicate's item counts differ; VOID if an instrument or control check failed. `orbits/0.9 active` (V / G) is the saturation: above 1 one target alone supplies the rank that is needed. LICENSE = R_best GAIN and Q_best GAIN and tot_best not LOSS. Model costs are MITM probe counts, not solver conflicts.\n\n| cell | tier | items G/V | active cols G/V | saturation V / G | R_best (CI) | class | R_blind (CI) | class | R_tot_best (CI) | class | R_tot_blind (CI) | class | Q_best (class) | null S/W (class) | calibration V/W, V/S, Q_blind (not gated) | delta | attributed to | licence |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    md.push_str(&verdicts);
    // per-m counts over the primary cells (no analyst choices, no trend rule)
    writeln!(md, "\n## 2b. Per m, primary cells (a = 0, n >= 13): class counts\n\n| m | cells | R_best classes | R_tot_best classes | licensed | model-level closure |\n|---|---|---|---|---|---|").unwrap();
    for mm in 2..=6usize {
        let cs: Vec<&(CellKey, Judge)> = judged
            .iter()
            .filter(|(k, _)| k.0 == 0 && k.1 >= 13 && k.2 == mm)
            .collect();
        if cs.is_empty() {
            continue;
        }
        let tally = |f: &dyn Fn(&Judge) -> String| {
            let mut t: BTreeMap<String, usize> = BTreeMap::new();
            for (_, j) in &cs {
                *t.entry(if aa_fail && j.completeness == "COMPLETE" {
                    "VOID_AA".into()
                } else {
                    f(j)
                })
                .or_default() += 1;
            }
            t.iter()
                .map(|(k, v)| format!("{k}:{v}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let nlic = cs.iter().filter(|(_, j)| j.license).count();
        let closed = per_m_closed(&cs.iter().map(|(_, j)| j).collect::<Vec<_>>(), aa_fail);
        writeln!(
            md,
            "| {mm} | {} | {} | {} | {} (need >= 2) | {} |",
            cs.len(),
            tally(&|j| j.v_best.clone()),
            tally(&|j| j.v_tbest.clone()),
            nlic,
            if nlic >= 2 && !aa_fail {
                "no: licence"
            } else if closed {
                "yes: R_best and R_tot_best are LOSS or PARITY in every non-void primary cell"
            } else {
                "no: mixed, unresolved or void"
            }
        )
        .unwrap();
    }
    writeln!(
        md,
        "\n## 3. Per-cell arm tables (mean over bases, 95% CI)\n"
    )
    .unwrap();
    md.push_str(&body);
    writeln!(md, "## 4. What this does not establish\n\nNo SAT, Groebner or other solver was run: the membership-system price is reported as the exact ANF degree of the indicator only. The cost columns are meet-in-the-middle probe counts of a model search, not measured solver work, and no matched Pollard rho reference or section 5a pricing band is attached, so no row here may enter the IC leaderboard or benchmarks/README.md. Nothing here addresses m = 31 or the m = 83 gate or ECC2K-130. A GAIN class means only that the relation-collection economics on the toy field favour the iota-stable base before the membership price; it is not an improvement over the index-calculus baseline.").unwrap();
    std::fs::write(format!("{dir}/summary.md"), md).map_err(|e| e.to_string())?;
    std::fs::write(format!("{dir}/summary.tsv"), tsv).map_err(|e| e.to_string())?;
    std::fs::write(format!("{dir}/fig_cost_ratio.svg"), svg(&fig)).map_err(|e| e.to_string())?;
    Ok(())
}

fn svg(fig: &[(String, usize, Ci, Ci)]) -> String {
    let (w, h, ml, mb) = (900.0, 420.0, 60.0, 90.0);
    let n = fig.len().max(1) as f64;
    let vals: Vec<f64> = fig
        .iter()
        .flat_map(|f| [f.2.lo, f.2.hi, f.3.lo, f.3.hi])
        .filter(|v| v.is_finite() && *v > 0.0)
        .map(|v| v.log2())
        .collect();
    let (lo, hi) = (
        vals.iter().cloned().fold(-1.0, f64::min),
        vals.iter().cloned().fold(1.0, f64::max),
    );
    let y = |v: f64| 20.0 + (h - mb - 20.0) * (hi - v.log2()) / (hi - lo);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='10'><rect width='100%' height='100%' fill='white'/>");
    write!(s, "<text x='{ml}' y='12'>log2 of cost ratio cpr(V)/cpr(G); above 0 = iota-stable base cheaper per independent relation. Blue: quotient-aware search on G (R_best). Orange: same blind search (R_blind). Bars: 95% bootstrap CI over bases.</text>").unwrap();
    let y1 = y(1.0);
    write!(
        s,
        "<line x1='{ml}' x2='{}' y1='{y1}' y2='{y1}' stroke='#888'/>",
        w - 10.0
    )
    .unwrap();
    for (i, (lab, _m, rb, rl)) in fig.iter().enumerate() {
        let x = ml + (w - ml - 10.0) * (i as f64 + 0.5) / n;
        for (c, col, dx) in [(rb, "#1f5fa8", -3.0), (rl, "#d9822b", 3.0)] {
            if c.est.is_finite() && c.est > 0.0 && c.lo > 0.0 && c.hi > 0.0 {
                write!(s, "<line x1='{0}' x2='{0}' y1='{1}' y2='{2}' stroke='{col}'/><circle cx='{0}' cy='{3}' r='2.5' fill='{col}'/>", x + dx, y(c.lo), y(c.hi), y(c.est)).unwrap();
            }
        }
        write!(
            s,
            "<text transform='translate({x},{}) rotate(60)' font-size='8'>{lab}</text>",
            h - mb + 8.0
        )
        .unwrap();
    }
    s.push_str("</svg>");
    s
}

/// PROTOCOL 7 rule 8: "closed at the model level at m" needs at least two non-void primary cells at m and
/// R_best and R_tbest LOSS or PARITY in every non-void primary cell; void cells (NOT_RUN, INCOMPLETE,
/// UNMATCHED, CENSORED, VOID) neither count nor block.
pub fn per_m_closed(cs: &[&Judge], aa_fail: bool) -> bool {
    let ok = |s: &String| s == "LOSS" || s == "PARITY";
    let nonvoid = |j: &&&Judge| {
        j.completeness == "COMPLETE"
            && !["UNMATCHED", "CENSORED", "VOID"].contains(&j.v_best.as_str())
    };
    !aa_fail
        && cs.iter().filter(nonvoid).count() >= 2
        && cs
            .iter()
            .filter(nonvoid)
            .all(|j| ok(&j.v_best) && ok(&j.v_tbest))
}
