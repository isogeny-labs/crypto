//! Decision rule (PROTOCOL.md section 8), implemented exactly. Reads rows_*.tsv / meta_*.txt, writes verdicts.tsv and summary.md.

use std::collections::BTreeMap;
use std::fmt::Write as _;

pub const DELTA: f64 = 0.095_310_179_804_324_86; // ln(1.10): practical-equivalence margin
pub const DUP_MIN_LOG_RATIO: f64 = 0.405_465_108_108_164_4; // ln(1.5): the positive control must show at least this on disp
pub const TAMPER_MIN_FRACTION: f64 = 0.5; // fano_tamper must reach this fraction of n
/// 80 percent power quantile, rounded up (normal 0.842; t at df 7 is 0.896). The minimum detectable ratio of the DIFFERENT rule is
/// exp(max(DELTA, (t_diff(df) + MDE_POWER_Z) se)): the effect that the rule (Bonferroni threshold AND margin) detects with power 0.8.
pub const MDE_POWER_Z: f64 = 0.9;
pub const CHAIN_REL_TOL: f64 = 1e-7; // digit-model cross-check: |measured - predicted| <= tol * predicted (rows print 9 digits)

/// One-sided 95 percent t critical value, taken at the largest tabulated df not above the actual df (conservative).
pub fn t_tost(df: usize) -> f64 {
    let tab = [
        (7, 1.895),
        (8, 1.860),
        (10, 1.812),
        (15, 1.753),
        (20, 1.725),
        (30, 1.697),
        (60, 1.671),
    ];
    let mut v = 2.365; // df < 7
    for &(d, t) in tab.iter() {
        if df >= d {
            v = t;
        }
    }
    v
}

/// Two-sided critical value for a family-wise 0.05 over 338 tests (26 cells x 13 classified contrasts per cell: primary 3, A/A 3,
/// DUP 1, NC 3, NR 3; per-test alpha 0.05/338 = 1.48e-4). Values are the exact Student-t quantiles (7.412, 5.916, 5.035, 4.668,
/// 4.342, 4.053 at df 7, 10, 15, 20, 30, 60, recomputed with a continued-fraction incomplete beta) rounded UP to one decimal; the
/// code does not compute quantiles. Contrasts need >= 8 pairs, so df >= 7 always.
pub fn t_diff(df: usize) -> f64 {
    let tab = [
        (7, 7.5),
        (10, 6.0),
        (15, 5.1),
        (20, 4.7),
        (30, 4.4),
        (60, 4.1),
    ];
    let mut v = 8.0; // df < 7
    for &(d, t) in tab.iter() {
        if df >= d {
            v = t;
        }
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    Equivalent,
    Different,
    Unresolved,
    TooFew,
}

impl Verdict {
    pub fn name(&self) -> &'static str {
        match self {
            Verdict::Equivalent => "EQUIVALENT",
            Verdict::Different => "DIFFERENT",
            Verdict::Unresolved => "UNRESOLVED",
            Verdict::TooFew => "TOO_FEW",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Contrast {
    pub n: usize,
    pub d: f64,
    pub se: f64,
    pub df: usize,
    pub verdict: Verdict,
}

impl Contrast {
    pub fn ratio(&self) -> f64 {
        self.d.exp()
    }
    pub fn ci90(&self) -> (f64, f64) {
        let t = t_tost(self.df);
        ((self.d - t * self.se).exp(), (self.d + t * self.se).exp())
    }
    pub fn mde_ratio(&self) -> f64 {
        (((t_diff(self.df) + MDE_POWER_Z) * self.se).max(DELTA)).exp()
    }
}

fn classify(d: f64, se: f64, df: usize) -> Verdict {
    // Degenerate case (PROTOCOL section 8): zero standard error (all pair differences identical) is decided by the margin alone.
    if se == 0.0 {
        return if d.abs() < DELTA {
            Verdict::Equivalent
        } else {
            Verdict::Different
        };
    }
    let (tt, td) = (t_tost(df), t_diff(df));
    if d + tt * se < DELTA && d - tt * se > -DELTA {
        Verdict::Equivalent
    } else if d.abs() > td * se && d.abs() > DELTA {
        Verdict::Different
    } else {
        Verdict::Unresolved
    }
}

fn mean_var(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let var = if v.len() > 1 {
        v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n - 1.0)
    } else {
        0.0
    };
    (m, var)
}

/// Paired contrast of ln-values (x_i, u_i are pairs); fewer than 8 pairs gives TooFew.
pub fn contrast_paired(x: &[f64], u: &[f64]) -> Contrast {
    assert_eq!(x.len(), u.len());
    let d: Vec<f64> = x.iter().zip(u).map(|(a, b)| a - b).collect();
    if d.len() < 8 {
        return Contrast {
            n: d.len(),
            d: f64::NAN,
            se: f64::NAN,
            df: 0,
            verdict: Verdict::TooFew,
        };
    }
    let (m, var) = mean_var(&d);
    let se = (var / d.len() as f64).sqrt();
    let df = d.len() - 1;
    Contrast {
        n: d.len(),
        d: m,
        se,
        df,
        verdict: classify(m, se, df),
    }
}

/// Welch contrast of ln-values (independent samples); df is the smaller sample size minus one (conservative).
pub fn contrast_welch(x: &[f64], u: &[f64]) -> Contrast {
    if x.len() < 8 || u.len() < 8 {
        return Contrast {
            n: x.len().min(u.len()),
            d: f64::NAN,
            se: f64::NAN,
            df: 0,
            verdict: Verdict::TooFew,
        };
    }
    let (mx, vx) = mean_var(x);
    let (mu, vu) = mean_var(u);
    let se = (vx / x.len() as f64 + vu / u.len() as f64).sqrt();
    let df = x.len().min(u.len()) - 1;
    Contrast {
        n: x.len().min(u.len()),
        d: mx - mu,
        se,
        df,
        verdict: classify(mx - mu, se, df),
    }
}

#[derive(Clone, Debug)]
pub struct R {
    pub arm: String,
    pub idx: u64,
    pub l2mu: f64,
    pub p: f64,
    pub eff: f64,
    pub disp: f64,
    pub fano: f64,
    pub fano_tamper: f64,
    pub planted: usize,
    pub conserved: bool,
    pub a_len: usize,
    pub b_len: usize,
    pub adds_build: u64,
    pub log2t: f64,
    /// digit-model prediction (NaN unless the instance is a Frobenius chain, PROTOCOL section 4.4)
    pub pred_p: f64,
    pub pred_disp: f64,
}

impl R {
    pub fn is_chain(&self) -> bool {
        self.pred_p.is_finite()
    }
}

pub fn parse_rows(text: &str) -> Vec<R> {
    let mut out = vec![];
    for (i, line) in text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        assert!(c.len() >= 14, "bad row: {line}");
        let f = |k: usize| c[k].parse::<f64>().unwrap();
        out.push(R {
            arm: c[0].to_string(),
            idx: c[1].parse().unwrap(),
            l2mu: f(2),
            p: f(4),
            eff: f(5),
            disp: f(6),
            fano: f(7),
            fano_tamper: f(8),
            planted: c[9].parse().unwrap(),
            conserved: c[10] == "1",
            a_len: c[11].parse().unwrap(),
            b_len: c[12].parse().unwrap(),
            adds_build: c[13].parse().unwrap(),
            log2t: f(3),
            pred_p: c.get(14).map_or(f64::NAN, |x| x.parse::<f64>().unwrap()),
            pred_disp: c.get(15).map_or(f64::NAN, |x| x.parse::<f64>().unwrap()),
        });
    }
    out
}

pub fn stat_of(r: &R, s: &str) -> f64 {
    match s {
        "eff" => r.eff,
        "disp" => r.disp,
        "fano" => r.fano,
        _ => panic!("stat"),
    }
}

fn ln_ok(v: f64) -> Option<f64> {
    if v.is_finite() && v > 0.0 {
        Some(v.ln())
    } else {
        None
    }
}

/// Paired ln-values of `stat` for arms (a, b) matched on idx; pairs with a non-finite or non-positive value are dropped.
pub fn paired_ln(rows: &[R], a: &str, b: &str, stat: &str) -> (Vec<f64>, Vec<f64>, usize) {
    paired_ln_by(rows, a, b, stat, &|_| true)
}

/// As `paired_ln`, keeping only pairs whose arm-`a` row passes `keep`.
pub fn paired_ln_by(
    rows: &[R],
    a: &str,
    b: &str,
    stat: &str,
    keep: &dyn Fn(&R) -> bool,
) -> (Vec<f64>, Vec<f64>, usize) {
    let (mut xa, mut xb, mut dropped) = (vec![], vec![], 0);
    for ra in rows.iter().filter(|r| r.arm == a && keep(r)) {
        if let Some(rb) = rows.iter().find(|r| r.arm == b && r.idx == ra.idx) {
            match (ln_ok(stat_of(ra, stat)), ln_ok(stat_of(rb, stat))) {
                (Some(x), Some(y)) => {
                    xa.push(x);
                    xb.push(y);
                }
                _ => dropped += 1,
            }
        }
    }
    (xa, xb, dropped)
}

pub fn meta_get(meta: &str, key: &str) -> Option<String> {
    meta.lines().find_map(|l| {
        let mut it = l.splitn(2, '\t');
        if it.next() == Some(key) {
            it.next().map(|s| s.to_string())
        } else {
            None
        }
    })
}

pub struct CellResult {
    pub id: String,
    pub n: u32,
    pub a: u32,
    pub m: usize,
    pub status: String,
    pub valid: bool,
    pub void_reasons: Vec<String>,
    pub yield_verdict: String,
    pub attempt_verdict: String,
    /// primary contrasts on the NON-chain pairs only (the statistical claim, PROTOCOL section 8)
    pub primary: Vec<(String, Contrast)>,
    /// NI/UP pairs whose NI instance is an exactly predictable Frobenius chain (derived, not tested)
    pub chain_pairs: usize,
    pub stat_pairs: usize,
    /// median over the NI rows of log2(tuples)/m: the mean log2 slot size, the covariate that m is confounded with
    pub slot_s: f64,
    pub lines: Vec<String>,
}

/// Accounting only (PROTOCOL section 9): group-addition equivalents per COVERED TARGET (a target with H >= 1, i.e. a distinct
/// relation), native lookups/inserts at kappa each. Amortised: q (1 + kappa) / p. Single target: (adds_build + q + kappa (q + stored)) / p.
/// NI and UP have the same operation counts, so the two arms differ here only through p; this is not an independent cost result.
/// Structurally duplicated tuples are not priced as relations.
pub fn cost_per_covered(r: &R, kappa: f64) -> (f64, f64) {
    let q = r.a_len.min(r.b_len) as f64;
    let stored = r.a_len.max(r.b_len) as f64;
    (
        q * (1.0 + kappa) / r.p,
        (r.adds_build as f64 + q + kappa * (q + stored)) / r.p,
    )
}

pub fn analyze_cell(id: &str, meta: &str, rows: &[R]) -> CellResult {
    let n: u32 = meta_get(meta, "n").unwrap().parse().unwrap();
    let a: u32 = meta_get(meta, "a").unwrap().parse().unwrap();
    let m: usize = meta_get(meta, "m").unwrap().parse().unwrap();
    let status = meta_get(meta, "status").unwrap_or_else(|| "MISSING".into());
    let mut cr = CellResult {
        id: id.into(),
        n,
        a,
        m,
        status: status.clone(),
        valid: false,
        void_reasons: vec![],
        yield_verdict: "NOT_EVALUATED".into(),
        attempt_verdict: "NOT_EVALUATED".into(),
        primary: vec![],
        chain_pairs: 0,
        stat_pairs: 0,
        slot_s: f64::NAN,
        lines: vec![],
    };
    if status != "OK" {
        cr.yield_verdict = status.clone();
        cr.attempt_verdict = status;
        return cr;
    }
    // instrument checks
    if rows.iter().any(|r| !r.conserved) {
        cr.void_reasons.push("CONSERVATION_FAILED".into());
    }
    if rows.iter().any(|r| r.planted != crate::cell::PLANTS) {
        cr.void_reasons.push("PLANTED_TUPLE_MISSED".into());
    }
    let tmin = TAMPER_MIN_FRACTION * n as f64;
    if rows
        .iter()
        .any(|r| r.fano_tamper.is_finite() && r.fano_tamper < tmin)
    {
        cr.void_reasons.push("ORBIT_DETECTOR_MISSED_TAMPER".into());
    }
    // digit-model cross-check (PROTOCOL section 4.4, claim S6): wherever the instance is an exactly predictable Frobenius chain,
    // the measured p and disp must equal the algebraic prediction. A mismatch is an instrument (or model) failure, never a finding.
    for r in rows.iter().filter(|r| r.is_chain()) {
        let ok_p = (r.p - r.pred_p).abs() <= CHAIN_REL_TOL * r.pred_p;
        let ok_d = (r.disp - r.pred_disp).abs() <= CHAIN_REL_TOL * r.pred_disp.abs().max(1e-12);
        if !(ok_p && ok_d) {
            let why = format!("DIGIT_MODEL_DISAGREES({} idx {})", r.arm, r.idx);
            if !cr.void_reasons.contains(&why) {
                cr.void_reasons.push(why);
            }
        }
    }
    {
        let mut v: Vec<f64> = rows
            .iter()
            .filter(|r| r.arm == "NI")
            .map(|r| r.log2t / m as f64)
            .collect();
        if !v.is_empty() {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            cr.slot_s = v[v.len() / 2];
        }
    }
    // positive control (DUP vs UP on disp), A/A (UP2 vs UP on every stat)
    let (xd, ud, _) = paired_ln(rows, "DUP", "UP", "disp");
    let dup = contrast_paired(&xd, &ud);
    cr.lines.push(format!(
        "{id}\tCONTROL_DUP_vs_UP\tdisp\t{}\t{:.4}\t{:.4}\t{}",
        dup.n,
        dup.d,
        dup.se,
        dup.verdict.name()
    ));
    if dup.verdict == Verdict::TooFew {
        cr.void_reasons.push("POSITIVE_CONTROL_TOO_FEW".into());
    } else if !(dup.verdict == Verdict::Different && dup.d > DUP_MIN_LOG_RATIO) {
        cr.void_reasons.push("POSITIVE_CONTROL_NOT_DETECTED".into());
    }
    for s in ["eff", "disp", "fano"] {
        let (x, u, _) = paired_ln(rows, "UP2", "UP", s);
        let aa = contrast_paired(&x, &u);
        cr.lines.push(format!(
            "{id}\tAA_UP2_vs_UP\t{s}\t{}\t{:.4}\t{:.4}\t{}",
            aa.n,
            aa.d,
            aa.se,
            aa.verdict.name()
        ));
        if aa.verdict == Verdict::Different {
            cr.void_reasons.push(format!("AA_DIFFERENT_{s}"));
        }
    }
    // primary: NI vs profile-matched generic, paired. Pairs whose NI instance is an exactly predictable chain are a derived
    // stratum (exact computation, reported, not tested); the statistical claim rests on the non-chain pairs only.
    let is_chain: std::collections::HashSet<u64> = rows
        .iter()
        .filter(|r| r.arm == "NI" && r.is_chain())
        .map(|r| r.idx)
        .collect();
    let not_chain = |r: &R| !is_chain.contains(&r.idx);
    for s in ["eff", "disp", "fano"] {
        let (x, u, dropped) = paired_ln_by(rows, "NI", "UP", s, &not_chain);
        let c = contrast_paired(&x, &u);
        let (lo, hi) = c.ci90();
        if s == "eff" {
            cr.stat_pairs = c.n;
        }
        cr.lines.push(format!(
            "{id}\tPRIMARY_NI_vs_UP_nonchain\t{s}\t{}\t{:.4}\t{:.4}\t{:.4}\t[{:.4},{:.4}]\tMDE x{:.4}\tdropped {dropped}\t{}",
            c.n, c.d, c.se, c.ratio(), lo, hi, c.mde_ratio(), c.verdict.name()
        ));
        cr.primary.push((s.to_string(), c));
        let (xc, uc, dc) = paired_ln_by(rows, "NI", "UP", s, &|r: &R| is_chain.contains(&r.idx));
        if !xc.is_empty() {
            cr.chain_pairs = xc.len();
            let md = xc.iter().zip(&uc).map(|(a, b)| a - b).sum::<f64>() / xc.len() as f64;
            cr.lines.push(format!("{id}\tDERIVED_CHAIN_NI_vs_UP\t{s}\t{}\t{:.4}\t-\tratio {:.4}\tdropped {dc}\tnot_a_test", xc.len(), md, md.exp()));
        }
    }
    // secondary, descriptive: NC and NR against the UP rows inside the same mu window, Welch
    let med: f64 = meta_get(meta, "window_center")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    for arm in ["NC", "NR"] {
        for s in ["eff", "disp", "fano"] {
            let x: Vec<f64> = rows
                .iter()
                .filter(|r| r.arm == arm)
                .filter_map(|r| ln_ok(stat_of(r, s)))
                .collect();
            let u: Vec<f64> = rows
                .iter()
                .filter(|r| r.arm == "UP" && (r.l2mu - med).abs() <= crate::cell::WINDOW_HALF)
                .filter_map(|r| ln_ok(stat_of(r, s)))
                .collect();
            let c = contrast_welch(&x, &u);
            cr.lines.push(format!(
                "{id}\tSECONDARY_{arm}_vs_UPwin\t{s}\t{}\t{:.4}\t{:.4}\t{}",
                c.n,
                c.d,
                c.se,
                c.verdict.name()
            ));
        }
    }
    // single-instance ONB: descriptive
    if let (Some(o), Some(u)) = (
        rows.iter().find(|r| r.arm == "ONB"),
        rows.iter().find(|r| r.arm == "ONBUP"),
    ) {
        for s in ["eff", "disp", "fano"] {
            if let (Some(a), Some(b)) = (ln_ok(stat_of(o, s)), ln_ok(stat_of(u, s))) {
                cr.lines.push(format!(
                    "{id}\tDESCRIPTIVE_ONB_vs_ONBUP\t{s}\t1\t{:.4}\t-\t-",
                    a - b
                ));
            }
        }
    }
    cr.valid = cr.void_reasons.is_empty();
    if !cr.valid {
        cr.yield_verdict = format!("VOID({})", cr.void_reasons.join(","));
        cr.attempt_verdict = cr.yield_verdict.clone();
        return cr;
    }
    let get = |s: &str| cr.primary.iter().find(|(k, _)| k == s).unwrap().1.clone();
    let (e, d, f) = (get("eff"), get("disp"), get("fano"));
    let chain_note = if cr.chain_pairs > 0 {
        let md = |st: &str| {
            let (x, u, _) = paired_ln_by(rows, "NI", "UP", st, &|r: &R| is_chain.contains(&r.idx));
            (x.iter().zip(&u).map(|(a, b)| a - b).sum::<f64>() / x.len().max(1) as f64).exp()
        };
        format!(
            "; CHAIN_DERIVED(pairs {}, eff x{:.3}, disp x{:.3}: exact algebra, not a finding)",
            cr.chain_pairs,
            md("eff"),
            md("disp")
        )
    } else {
        String::new()
    };
    let mut diff = vec![];
    if e.verdict == Verdict::Different {
        diff.push("eff");
    }
    if d.verdict == Verdict::Different {
        diff.push("disp");
    }
    let stat = if cr.stat_pairs < 8 {
        format!("NO_STATISTICAL_PAIRS({} non-chain)", cr.stat_pairs)
    } else if !diff.is_empty() {
        format!("YIELD_DIFFERS({})", diff.join("+"))
    } else if e.verdict == Verdict::Equivalent && d.verdict == Verdict::Equivalent {
        "NO_CHANGE_WITHIN_DELTA".into()
    } else {
        format!(
            "UNRESOLVED(MDE eff x{:.3}, disp x{:.3})",
            e.mde_ratio(),
            d.mde_ratio()
        )
    };
    cr.yield_verdict = format!("{stat}{chain_note}");
    cr.attempt_verdict = if cr.stat_pairs < 8 {
        format!("NO_STATISTICAL_PAIRS({} non-chain)", cr.stat_pairs)
    } else {
        match f.verdict {
            Verdict::Different => format!("ATTEMPTS_DEPENDENCE_DIFFERS(ratio {:.3})", f.ratio()),
            Verdict::Equivalent => "ATTEMPT_DEPENDENCE_SAME_AS_GENERIC".into(),
            Verdict::Unresolved => format!("UNRESOLVED(MDE x{:.3})", f.mde_ratio()),
            Verdict::TooFew => "TOO_FEW".into(),
        }
    };
    cr
}

fn corr(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < 3 {
        return None;
    }
    let (mx, my) = (
        x.iter().sum::<f64>() / n as f64,
        y.iter().sum::<f64>() / n as f64,
    );
    let (sxx, syy): (f64, f64) = (
        x.iter().map(|a| (a - mx) * (a - mx)).sum(),
        y.iter().map(|a| (a - my) * (a - my)).sum(),
    );
    if sxx == 0.0 || syy == 0.0 {
        return None;
    }
    Some(
        x.iter()
            .zip(y)
            .map(|(a, b)| (a - mx) * (b - my))
            .sum::<f64>()
            / (sxx * syy).sqrt(),
    )
}

fn ols(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    let n = x.len();
    if n < 4 {
        return None;
    }
    let (mx, my) = (
        x.iter().sum::<f64>() / n as f64,
        y.iter().sum::<f64>() / n as f64,
    );
    let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
    if sxx == 0.0 {
        return None;
    }
    let b = x
        .iter()
        .zip(y)
        .map(|(a, c)| (a - mx) * (c - my))
        .sum::<f64>()
        / sxx;
    let res: f64 = x
        .iter()
        .zip(y)
        .map(|(a, c)| {
            let r = c - my - b * (a - mx);
            r * r
        })
        .sum();
    Some((b, (res / (n as f64 - 2.0) / sxx).sqrt()))
}

pub fn analyze_dir(dir: &str) -> Result<String, String> {
    let mut ids: Vec<String> = vec![];
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let name = e
            .map_err(|e| e.to_string())?
            .file_name()
            .to_string_lossy()
            .to_string();
        if let Some(id) = name
            .strip_prefix("meta_")
            .and_then(|s| s.strip_suffix(".txt"))
        {
            ids.push(id.to_string());
        }
    }
    ids.sort();
    let expected = crate::cell::cell_list();
    let mut results = vec![];
    for id in &ids {
        let meta =
            std::fs::read_to_string(format!("{dir}/meta_{id}.txt")).map_err(|e| e.to_string())?;
        let rows = parse_rows(
            &std::fs::read_to_string(format!("{dir}/rows_{id}.tsv")).map_err(|e| e.to_string())?,
        );
        results.push(analyze_cell(id, &meta, &rows));
    }
    let mut tsv = String::from("cell\tcontrast\tstat\tN\td_ln\tse\t(PRIMARY rows: ratio, ci90, mde, dropped)\tverdict (last column)\n");
    let mut md = String::new();
    writeln!(md, "# G6 summary (generated by `g6census analyze`; the decision rule of PROTOCOL.md section 8)\n").unwrap();
    writeln!(md, "## 0. Planned versus present\n").unwrap();
    let mut missing = vec![];
    for (n, a, m) in &expected {
        let id = format!("n{n}a{a}m{m}");
        if !ids.contains(&id) {
            missing.push(id);
        }
    }
    writeln!(
        md,
        "cells present: {} of {}; missing (CELL_NOT_RUN): {}\n",
        ids.len(),
        expected.len(),
        if missing.is_empty() {
            "none".to_string()
        } else {
            missing.join(", ")
        }
    )
    .unwrap();
    writeln!(md, "## 1. Cell verdicts\n\n| cell | status | valid | yield (eff, disp) | attempts (fano) |\n|:--|:--|:--|:--|:--|").unwrap();
    for r in &results {
        writeln!(
            md,
            "| {} | {} | {} | {} | {} |",
            r.id, r.status, r.valid, r.yield_verdict, r.attempt_verdict
        )
        .unwrap();
        for l in &r.lines {
            writeln!(tsv, "{}", l.replace("\t", "\t")).unwrap();
        }
    }
    writeln!(md, "\n## 1b. Size census (exact slot sizes only, no group sums): can the design reach the target window?\n").unwrap();
    writeln!(md, "| cell | draws | feasible NI | feasible generic U | selected pairs | median log2 mu of selected | sensitivity flag |\n|:--|--:|--:|--:|--:|--:|:--|").unwrap();
    for id in &ids {
        let meta =
            std::fs::read_to_string(format!("{dir}/meta_{id}.txt")).map_err(|e| e.to_string())?;
        let g = |k: &str| meta_get(&meta, k).unwrap_or_else(|| "-".into());
        let wc = g("window_center");
        let flag = match wc.parse::<f64>() {
            Ok(v) if v < -3.0 => "LOW_MU: coincidence statistics have little to see (mu < 1/8); EQUIVALENT is weak evidence",
            Ok(v) if v > 2.0 => "HIGH_MU: eff ~ 1 and fano is NaN when p = 1 (mu > 4); EQUIVALENT is weak evidence",
            Ok(_) => "ok",
            Err(_) => "no enumeration",
        };
        writeln!(
            md,
            "| {id} | {} | {} | {} | {} | {wc} | {flag} |",
            g("draws"),
            g("feasible_NI"),
            g("feasible_U"),
            g("selected_pairs")
        )
        .unwrap();
    }
    // trend of the primary log-ratio over m, per curve and stat; m is confounded with the mean slot size (PROTOCOL section 4.5)
    writeln!(md, "\n## 2. Trend of ln(NI/UP) over m (valid cells, non-chain pairs only; DESCRIPTIVE, m is confounded with slot size)\n").unwrap();
    writeln!(md, "At fixed n the mean log2 slot size s = log2(T)/m falls as m grows, so a slope over m is a slope over s as well and is not attributable to m. The correlation r(m, s) is printed; where |r| > 0.9 the two are not separable here. Section 2b varies s at fixed m.\n").unwrap();
    for (n, a) in [(23u32, 0u32), (19, 0), (19, 1)] {
        let cells: Vec<&CellResult> = results
            .iter()
            .filter(|r| r.valid && r.n == n && r.a == a && r.slot_s.is_finite())
            .collect();
        let (mx, sx): (Vec<f64>, Vec<f64>) = cells.iter().map(|r| (r.m as f64, r.slot_s)).unzip();
        match corr(&mx, &sx) {
            Some(r) => writeln!(
                md,
                "- n={n} a={a}: r(m, s) = {r:+.3} over {} cells",
                cells.len()
            )
            .unwrap(),
            None => writeln!(md, "- n={n} a={a}: fewer than 3 valid cells, no r(m, s)").unwrap(),
        }
        for s in ["eff", "disp", "fano"] {
            let (mut x, mut y, mut sv) = (vec![], vec![], vec![]);
            for r in cells.iter() {
                if let Some((_, c)) = r.primary.iter().find(|(k, _)| k == s) {
                    if c.d.is_finite() && c.n >= 8 {
                        x.push(r.m as f64);
                        y.push(c.d);
                        sv.push(r.slot_s);
                    }
                }
            }
            match (ols(&x, &y), ols(&sv, &y)) {
                (Some((b, se)), Some((bs, ses))) => writeln!(md, "  - {s}: slope per unit m {b:+.4} (se {se:.4}); slope per bit of s {bs:+.4} (se {ses:.4}); {} cells", x.len()).unwrap(),
                _ => writeln!(md, "  - {s}: fewer than 4 cells with non-chain pairs, no slope").unwrap(),
            }
        }
    }
    writeln!(
        md,
        "\n## 2b. Slot-size control: fixed m, n varying (a = 0; valid cells; non-chain pairs)\n"
    )
    .unwrap();
    writeln!(md, "| m | n | s = mean log2 slot size | non-chain pairs | chain pairs | ln ratio eff | ln ratio disp | ln ratio fano |\n|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for m in [9usize, 10] {
        let mut cs: Vec<&CellResult> = results
            .iter()
            .filter(|r| r.valid && r.a == 0 && r.m == m)
            .collect();
        cs.sort_by_key(|r| r.n);
        for r in cs {
            let g = |st: &str| {
                r.primary
                    .iter()
                    .find(|(k, _)| k == st)
                    .map_or("-".to_string(), |(_, c)| {
                        if c.d.is_finite() {
                            format!("{:+.3}", c.d)
                        } else {
                            "-".into()
                        }
                    })
            };
            writeln!(
                md,
                "| {} | {} | {:.2} | {} | {} | {} | {} | {} |",
                r.m,
                r.n,
                r.slot_s,
                r.stat_pairs,
                r.chain_pairs,
                g("eff"),
                g("disp"),
                g("fano")
            )
            .unwrap();
        }
    }
    writeln!(md, "\nIf ln(NI/UP) tracks s at fixed m and not m at fixed s, the effect is a slot-size effect (the Frobenius collision mechanism needs equal digits in consecutive slots, probability about 1/|S|^2 per window), not an m effect; nothing here transfers to n = 131 where |S| is 2^8 to 2^9.").unwrap();
    // pricing (accounting only)
    writeln!(md, "\n## 3. Accounting per COVERED target (a target with H >= 1), group additions + kappa x native ops, mean over instances\n").unwrap();
    writeln!(md, "Not a cost result: NI and UP have identical operation counts (same slot profile), so they differ here only through p. Structurally duplicated tuples are not priced as relations. The cost half of the original question was dropped (PROTOCOL section 9).\n").unwrap();
    writeln!(md, "| cell | arm | rows | amortised kappa=0 | 0.1 | 1 | single-target kappa=0 | 0.1 | 1 |\n|:--|:--|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for id in &ids {
        let rows = parse_rows(
            &std::fs::read_to_string(format!("{dir}/rows_{id}.tsv")).map_err(|e| e.to_string())?,
        );
        let mut by: BTreeMap<String, Vec<&R>> = BTreeMap::new();
        for r in &rows {
            by.entry(r.arm.clone()).or_default().push(r);
        }
        for arm in ["NI", "UP"] {
            if let Some(v) = by.get(arm) {
                let mean = |k: f64, one: bool| {
                    v.iter()
                        .map(|r| {
                            let c = cost_per_covered(r, k);
                            if one {
                                c.1
                            } else {
                                c.0
                            }
                        })
                        .sum::<f64>()
                        / v.len() as f64
                };
                writeln!(
                    md,
                    "| {id} | {arm} | {} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} |",
                    v.len(),
                    mean(0.0, false),
                    mean(0.1, false),
                    mean(1.0, false),
                    mean(0.0, true),
                    mean(0.1, true),
                    mean(1.0, true)
                )
                .unwrap();
            }
        }
    }
    writeln!(md, "\nBecause the operation counts are equal, the NI/UP order is the order of 1/p and is the same at kappa = 0, 0.1 and 1 by construction; no rho comparison is made (PROTOCOL section 9).").unwrap();
    std::fs::write(format!("{dir}/verdicts.tsv"), &tsv).map_err(|e| e.to_string())?;
    std::fs::write(format!("{dir}/summary.md"), &md).map_err(|e| e.to_string())?;
    Ok(md)
}
