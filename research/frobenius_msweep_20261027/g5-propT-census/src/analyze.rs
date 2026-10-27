//! Fixed decision rule (PROTOCOL.md sections 7 and 8) applied to the TSV rows of one run directory.
//! Writes summary.md, summary.tsv and fig_overlap_vs_m.svg. No judgement is left to the analyst: every class, gate,
//! abort condition and per-m statement named in the protocol is computed here. Nothing in this file reads wall time.

use crate::rng::{hash_str, Rng};
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Equivalence margin of the decision rule, declared in advance (PROTOCOL 8): E-OVERLAP means D inside
/// [1/(1+DELTA), 1+DELTA] with the whole 90 percent interval (two one-sided 5 percent tests, TOST).
pub const DELTA: f64 = 0.10;
/// An A/A, null-vs-null or plant gate that misses is expected by chance in about 5 percent of cell-arms; the run is
/// declared untrustworthy when more than this fraction miss (mean 5 percent, binomial sd about 1.5 percent at 216).
pub const MISS_ALARM: f64 = 0.12;

struct Table {
    cols: Vec<String>,
    rows: Vec<Vec<String>>,
    ragged: usize,
}

impl Table {
    fn read(path: &str) -> Result<Table, String> {
        let txt = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let mut lines = txt.lines();
        let cols: Vec<String> = lines
            .next()
            .ok_or("empty file")?
            .split('\t')
            .map(|s| s.to_string())
            .collect();
        let mut rows: Vec<Vec<String>> = vec![];
        let mut ragged = 0;
        for l in lines.filter(|l| !l.is_empty()) {
            let v: Vec<String> = l.split('\t').map(|s| s.to_string()).collect();
            if v.len() != cols.len() {
                ragged += 1; // a short last line of a killed job: counted, never read
                continue;
            }
            rows.push(v);
        }
        Ok(Table { cols, rows, ragged })
    }
    fn s(&self, r: usize, c: &str) -> &str {
        match self.cols.iter().position(|x| x == c) {
            Some(i) => &self.rows[r][i],
            None => "",
        }
    }
    fn f(&self, r: usize, c: &str) -> f64 {
        self.s(r, c).parse::<f64>().unwrap_or(f64::NAN)
    }
}

fn files(dir: &str, prefix: &str) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|n| n.starts_with(prefix) && n.ends_with(".tsv"))
                .collect()
        })
        .unwrap_or_default();
    v.sort();
    v.into_iter().map(|n| format!("{dir}/{n}")).collect()
}

fn pct(v: &mut Vec<f64>, q: f64) -> f64 {
    v.retain(|x| x.is_finite());
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// Point estimate with 95 and 90 percent percentile-bootstrap intervals.
#[derive(Clone, Copy, Debug)]
pub struct Ci {
    pub pt: f64,
    pub lo95: f64,
    pub hi95: f64,
    pub lo90: f64,
    pub hi90: f64,
}

impl Ci {
    fn nan(pt: f64) -> Ci {
        Ci {
            pt,
            lo95: f64::NAN,
            hi95: f64::NAN,
            lo90: f64::NAN,
            hi90: f64::NAN,
        }
    }
}

/// Percentile bootstrap (2000 resamples over bases) of a ratio of sums num/den.
pub fn boot_ratio(num: &[f64], den: &[f64], seed: u64) -> Ci {
    let n = num.len();
    let point = num.iter().sum::<f64>() / den.iter().sum::<f64>();
    if n < 2 {
        return Ci::nan(point);
    }
    let mut rng = Rng::new(seed);
    let mut vals = Vec::with_capacity(2000);
    for _ in 0..2000 {
        let (mut a, mut b) = (0.0, 0.0);
        for _ in 0..n {
            let i = rng.below(n as u64) as usize;
            a += num[i];
            b += den[i];
        }
        vals.push(a / b);
    }
    let mut v2 = vals.clone();
    let mut v3 = vals.clone();
    let mut v4 = vals.clone();
    Ci {
        pt: point,
        lo95: pct(&mut vals, 0.025),
        hi95: pct(&mut v2, 0.975),
        lo90: pct(&mut v3, 0.05),
        hi90: pct(&mut v4, 0.95),
    }
}

fn boot_mean(x: &[f64], seed: u64) -> (f64, f64, f64) {
    let x: Vec<f64> = x.iter().cloned().filter(|v| v.is_finite()).collect();
    let n = x.len();
    if n == 0 {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let point = x.iter().sum::<f64>() / n as f64;
    if n < 2 {
        return (point, f64::NAN, f64::NAN);
    }
    let mut rng = Rng::new(seed);
    let mut vals = Vec::with_capacity(2000);
    for _ in 0..2000 {
        let mut a = 0.0;
        for _ in 0..n {
            a += x[rng.below(n as u64) as usize];
        }
        vals.push(a / n as f64);
    }
    (point, pct(&mut vals.clone(), 0.025), pct(&mut vals, 0.975))
}

fn fmt3(t: (f64, f64, f64)) -> String {
    format!("{:.3} [{:.3}, {:.3}]", t.0, t.1, t.2)
}
fn fmt_ci95(c: &Ci) -> String {
    format!("{:.3} [{:.3}, {:.3}]", c.pt, c.lo95, c.hi95)
}
fn fmt_ci90(c: &Ci) -> String {
    format!("{:.3} [{:.3}, {:.3}]", c.pt, c.lo90, c.hi90)
}

/// Classes of the decision rule (PROTOCOL 8.4). `delta` is the declared margin DELTA.
/// E-EXTRA / E-DEFICIT: the 95 percent interval lies wholly beyond the margin on one side.
/// E-OVERLAP: the 90 percent interval lies wholly inside the margin (TOST). Otherwise UNRESOLVED.
pub fn classify(c: &Ci, delta: f64) -> &'static str {
    if !c.lo95.is_finite() || !c.hi95.is_finite() || !c.lo90.is_finite() || !c.hi90.is_finite() {
        return "UNRESOLVED";
    }
    if c.lo95 > 1.0 + delta {
        "E-EXTRA"
    } else if c.hi95 < 1.0 / (1.0 + delta) {
        "E-DEFICIT"
    } else if c.lo90 >= 1.0 / (1.0 + delta) && c.hi90 <= 1.0 + delta {
        "E-OVERLAP"
    } else {
        "UNRESOLVED"
    }
}

/// Least-squares slope of y on x.
fn slope(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
    sxy / sxx
}

/// Number of curves whose class sequence equals the most common one, ignoring sequences with a missing-cell placeholder "-".
pub fn trend_agreement(sq: &[String]) -> usize {
    let mut best = 0;
    for s in sq {
        best = best.max(
            sq.iter()
                .filter(|x| *x == s && !s.is_empty() && !s.split(',').any(|c| c == "-"))
                .count(),
        );
    }
    best
}

/// A/A gate: the 95 percent bootstrap interval of the ratio D_main / D_aa must contain 1 (the two halves are the same
/// arm under disjoint seeds). Returns (lo, hi, contains_one).
pub fn aa_gate(mu: &[f64], mn: &[f64], au: &[f64], an: &[f64], seed: u64) -> (f64, f64, bool) {
    let (m_n, a_n) = (mu.len(), au.len());
    if m_n < 2 || a_n < 2 {
        return (f64::NAN, f64::NAN, true);
    }
    let mut rng = Rng::new(seed);
    let mut rs: Vec<f64> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let (mut s1, mut s2, mut s3, mut s4) = (0.0, 0.0, 0.0, 0.0);
        for _ in 0..m_n {
            let i = rng.below(m_n as u64) as usize;
            s1 += mu[i];
            s2 += mn[i];
        }
        for _ in 0..a_n {
            let i = rng.below(a_n as u64) as usize;
            s3 += au[i];
            s4 += an[i];
        }
        rs.push((s1 / s2) / (s3 / s4));
    }
    let (lo, hi) = (pct(&mut rs.clone(), 0.025), pct(&mut rs, 0.975));
    (
        lo,
        hi,
        lo.is_finite() && hi.is_finite() && lo <= 1.0 && 1.0 <= hi,
    )
}

/// Verdict of one Proposition T (n, m, arm) aggregate. e = column sums in the order used below. Returns (ok, text).
/// koblitz / stable: no violation of any identity. ordinary: the identity fails in EVERY assignment. wrongdir: the
/// identity fails in at least 99 percent of trials. Nothing else is thresholded (PROTOCOL 8.1).
pub fn propt_verdict(arm: &str, e: &[f64]) -> (bool, String) {
    let (trials, assigns, eqf, bf, sing, planted, pbad, cmf, domf, negd, scf, aeq) = (
        e[0], e[1], e[2], e[3], e[4], e[5], e[6], e[8], e[10], e[12], e[14], e[15],
    );
    match arm {
        "koblitz" | "stable" => {
            if eqf == 0.0
                && aeq == 0.0
                && bf == 0.0
                && sing == 0.0
                && pbad == 0.0
                && cmf == 0.0
                && domf == 0.0
                && negd == 0.0
                && scf == 0.0
                && planted > 0.0
            {
                (
                    true,
                    format!(
                        "HOLDS (violation rate < {:.1e}, rule of three)",
                        3.0 / trials
                    ),
                )
            } else {
                (
                    false,
                    "VIOLATION: PROP_T_FALSIFIED_OR_INSTRUMENT_FAIL".to_string(),
                )
            }
        }
        "ordinary" => {
            let ok = aeq == assigns && assigns > 0.0;
            (
                ok,
                format!(
                    "{} (assignments failing {}/{})",
                    if ok {
                        "CONTROL DETECTS"
                    } else {
                        "CONTROL_FAIL"
                    },
                    aeq,
                    assigns
                ),
            )
        }
        _ => {
            let det = eqf / trials;
            let ok = det >= 0.99;
            (
                ok,
                format!(
                    "{} (trial detection {:.4})",
                    if ok {
                        "CONTROL DETECTS"
                    } else {
                        "CONTROL_FAIL"
                    },
                    det
                ),
            )
        }
    }
}

const PROPT_SUM_COLS: [&str; 16] = [
    "trials",
    "assigns",
    "trials_eq_fail",
    "trials_bool_fail",
    "rec_singular",
    "planted",
    "plant_bad",
    "conj_member_n",
    "conj_member_fail",
    "dom_n",
    "dom_fail",
    "neg_n",
    "neg_disagree",
    "stable_n",
    "stable_cob_fail",
    "assigns_eq_fail",
];

fn propt_aggregate(dir: &str) -> Result<BTreeMap<(u32, usize, String), Vec<f64>>, String> {
    let mut agg: BTreeMap<(u32, usize, String), Vec<f64>> = BTreeMap::new();
    for p in &files(dir, "propt_") {
        let t = Table::read(p)?;
        for r in 0..t.rows.len() {
            let key = (
                t.f(r, "n") as u32,
                t.f(r, "m") as usize,
                t.s(r, "arm").to_string(),
            );
            let e = agg.entry(key).or_insert_with(|| vec![0.0; 16]);
            for (i, c) in PROPT_SUM_COLS.iter().enumerate() {
                e[i] += t.f(r, c);
            }
        }
    }
    Ok(agg)
}

/// Fail-fast gate used between the phases of run_cells.sh: Ok(()) only if every Proposition T cell holds and every
/// control detects. Err lists the failures.
pub fn propt_gate(dir: &str) -> Result<usize, String> {
    let agg = propt_aggregate(dir)?;
    if agg.is_empty() {
        return Err("no propt rows found".into());
    }
    let bad: Vec<String> = agg
        .iter()
        .filter_map(|((n, m, arm), e)| {
            let (ok, text) = propt_verdict(arm, e);
            if ok {
                None
            } else {
                Some(format!("propt n={n} m={m} {arm}: {text}"))
            }
        })
        .collect();
    if bad.is_empty() {
        Ok(agg.len())
    } else {
        Err(bad.join("\n"))
    }
}

struct OvRow {
    a: u32,
    n: u32,
    m: usize,
    l: u32,
    r: f64,
    arm: String,
    half: String,
    nx: f64,
    nsol: f64,
    nu: f64,
    uf: f64,
    un: f64,
    up: f64,
    cf: f64,
    cn: f64,
    ceq: f64,
    hb: f64,
    cpb: f64,
    cnb: f64,
    lbb: f64,
    clos: f64,
    closp: f64,
    u2: f64,
    upl: f64,
    plok: f64,
    s1l_pairs: f64,
    s1l_cc1: f64,
    s1l_uf: f64,
    s1l_un: f64,
    status: String,
    jn: f64,
}

/// One evaluated (cell, arm), kept for the per-m statements and the abort conditions.
struct CellArm {
    cell: (u32, u32, usize, u32),
    arm: String,
    class: String,
    weight: f64,
}

fn parse_jobs(dir: &str) -> Vec<(String, u32, u32, usize, u32, usize)> {
    let mut v = vec![];
    if let Ok(txt) = std::fs::read_to_string(format!("{dir}/jobs.txt")) {
        for l in txt.lines() {
            let w: Vec<&str> = l.split_whitespace().collect();
            if w.len() == 6 {
                if let (Ok(a), Ok(n), Ok(m), Ok(ll), Ok(reps)) = (
                    w[1].parse(),
                    w[2].parse(),
                    w[3].parse(),
                    w[4].parse(),
                    w[5].parse(),
                ) {
                    v.push((w[0].to_string(), a, n, m, ll, reps));
                }
            }
        }
    }
    v
}

/// Planned versus present: every job of jobs.txt must have its exit code, its file and its expected number of rows.
/// Returns (markdown table, incomplete-job descriptions).
fn reconcile(dir: &str, md: &mut String) -> Vec<String> {
    let jobs = parse_jobs(dir);
    let mut incomplete: Vec<String> = vec![];
    if jobs.is_empty() {
        writeln!(md, "## 0. Planned versus present\n\nNo jobs.txt in the run directory: reconciliation not possible (a driver smoke test, not a run).\n").unwrap();
        return incomplete;
    }
    writeln!(md, "## 0. Planned versus present (jobs.txt)\n").unwrap();
    writeln!(
        md,
        "| job | exit | file | rows (expected) | ragged | status |"
    )
    .unwrap();
    writeln!(md, "|---|---|---|---|---|---|").unwrap();
    let (mut ok_n, mut notrun_n) = (0, 0);
    for (kind, a, n, m, l, reps) in &jobs {
        let stem = format!("{kind}_a{a}_n{n}_m{m}_l{l}");
        let exit = std::fs::read_to_string(format!("{dir}/exit_{stem}.txt"))
            .ok()
            .map(|s| s.trim().to_string());
        let (file, expected) = match kind.as_str() {
            "overlap" => {
                let stable = if *n == 17 && *l == 8 { 2 } else { 0 };
                (
                    format!("overlap_a{a}_n{n}_m{m}_l{l}.tsv"),
                    1 + stable + 8 * reps,
                )
            }
            "rank" => (format!("rank_a{a}_n{n}_m{m}_l{l}.tsv"), 4 * reps),
            _ => {
                let arms = if *n == 17 { 4 } else { 3 };
                (
                    format!("propt_n{n}_m{m}.tsv"),
                    ((*reps + 499) / 500) * 2 * arms,
                )
            }
        };
        let (rows, ragged) = match Table::read(&format!("{dir}/{file}")) {
            Ok(t) => (Some(t.rows.len()), t.ragged),
            Err(_) => (None, 0),
        };
        let status = match (exit.as_deref(), rows) {
            (Some("0"), Some(r)) if r == expected && ragged == 0 => {
                ok_n += 1;
                "OK".to_string()
            }
            (Some("2"), _) => {
                notrun_n += 1;
                "CELL_NOT_RUN (instrument refused the cell; recorded, not rerun)".to_string()
            }
            (None, _) => "NOT_FINISHED (no exit code)".to_string(),
            (Some(e), None) => format!("MISSING_FILE (exit {e})"),
            (Some(e), Some(r)) => format!("SHORT_OR_RAGGED (exit {e}, rows {r}, ragged {ragged})"),
        };
        if !(status == "OK" || status.starts_with("CELL_NOT_RUN")) {
            incomplete.push(format!("{stem}: {status}"));
        }
        writeln!(
            md,
            "| {stem} | {} | {file} | {} ({expected}) | {ragged} | {status} |",
            exit.unwrap_or_else(|| "-".into()),
            rows.map(|r| r.to_string()).unwrap_or_else(|| "-".into())
        )
        .unwrap();
    }
    writeln!(md, "\nJobs planned {}, complete {ok_n}, refused by the instrument (CELL_NOT_RUN) {notrun_n}, incomplete {}.\n", jobs.len(), incomplete.len()).unwrap();
    incomplete
}

pub fn analyze(dir: &str) -> Result<(), String> {
    let mut md = String::new();
    let mut tsv = String::from("kind\tcell\tarm\tstat\tvalue\tlo\thi\tclass\n");
    writeln!(md, "# G5 summary (generated by `g5census analyze`; decision rule frozen in PROTOCOL.md sections 7 and 8)\n").unwrap();
    writeln!(md, "Run directory: `{dir}`. Notation: n = field degree (AGENTS.md 8a calls it m), m = number of legs, r = order of the prime subgroup G.\n").unwrap();
    let mut voids: Vec<String> = vec![];
    let mut aborts: Vec<String> = vec![];

    let incomplete = reconcile(dir, &mut md);
    for i in &incomplete {
        voids.push(format!("INCOMPLETE job {i}"));
    }

    // ---------------- Proposition T ----------------
    let agg = propt_aggregate(dir)?;
    if !agg.is_empty() {
        writeln!(md, "## A. Proposition T, instance by instance\n").unwrap();
        writeln!(md, "| n | m | arm | trials | assigns | trials failing identity | assignments failing identity | Boolean-recombination fails | singular M_j | planted (bad) | conj. solution fails / n | domain fails / n | neg. disagree | verdict |").unwrap();
        writeln!(
            md,
            "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
        )
        .unwrap();
        for ((n, m, arm), e) in agg.iter() {
            let (ok, verdict) = propt_verdict(arm, e);
            if !ok {
                voids.push(format!("propt n={n} m={m} {arm}: {verdict}"));
                aborts.push(format!("propt n={n} m={m} {arm}: {verdict}"));
            }
            writeln!(
                md,
                "| {n} | {m} | {arm} | {} | {} | {} | {} | {} | {} | {} ({}) | {}/{} | {}/{} | {} | {verdict} |",
                e[0], e[1], e[2], e[15], e[3], e[4], e[5], e[6], e[8], e[7], e[10], e[9], e[12]
            )
            .unwrap();
            writeln!(
                tsv,
                "propt\tn{n}_m{m}\t{arm}\ttrials_eq_fail\t{}\t\t\t{verdict}",
                e[2]
            )
            .unwrap();
        }
        writeln!(md).unwrap();
    }

    // ---------------- overlap census ----------------
    let ofiles = files(dir, "overlap_");
    let mut rows: Vec<OvRow> = vec![];
    for p in &ofiles {
        let t = Table::read(p)?;
        for r in 0..t.rows.len() {
            let st = t.s(r, "status").to_string();
            let g = |c: &str| if st == "OK" { t.f(r, c) } else { f64::NAN };
            rows.push(OvRow {
                a: t.f(r, "a") as u32,
                n: t.f(r, "n") as u32,
                m: t.f(r, "m") as usize,
                l: t.f(r, "l") as u32,
                r: t.f(r, "r"),
                arm: t.s(r, "arm").to_string(),
                half: t.s(r, "half").to_string(),
                nx: t.f(r, "nx"),
                nsol: g("nsol"),
                nu: g("nu"),
                uf: g("u_frob"),
                un: g("u_null"),
                up: g("u_pred"),
                cf: g("corr_frob"),
                cn: g("corr_null"),
                ceq: g("count_equal_frob"),
                hb: g("halving_bad"),
                cpb: g("chain_pos_bad"),
                cnb: g("chain_neg_bad"),
                lbb: g("lb_bad"),
                clos: g("clos_closed"),
                closp: g("clos_pairs"),
                u2: g("u_null2"),
                upl: g("u_plant"),
                plok: g("plant_lags_ok"),
                s1l_pairs: g("s1l_pairs"),
                s1l_cc1: g("s1l_cc1"),
                s1l_uf: g("s1l_uf"),
                s1l_un: g("s1l_un"),
                status: st,
                jn: ((t.f(r, "n") - 1.0) / 2.0).floor(),
            });
        }
    }
    let mut cells: BTreeMap<(u32, u32, usize, u32), Vec<usize>> = BTreeMap::new();
    for (i, r) in rows.iter().enumerate() {
        cells.entry((r.a, r.n, r.m, r.l)).or_default().push(i);
    }
    let mut results: Vec<CellArm> = vec![];
    let (
        mut n_eval,
        mut n_aa_fail,
        mut n_nn_miss,
        mut n_plant_miss,
        mut n_plant_inform,
        mut n_void,
        mut n_cens,
    ) = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    if !rows.is_empty() {
        writeln!(
            md,
            "## B. Same-instance overlap census (Lemma A2-prime), decision table\n"
        )
        .unwrap();
        writeln!(md, "U = shared (tuple, conjugate-pair) triples; model = independence prediction from the measured K(S) sizes; null = same bases and tuples, random multipliers. D = U_frob / U_null (paired, same enumeration), pooled over the main and aa halves after the A/A gate. Declared margin delta = {DELTA}: E-OVERLAP iff the 90% interval of D lies inside [1/(1+delta), 1+delta] (TOST); E-EXTRA / E-DEFICIT iff the 95% interval lies wholly beyond the margin; otherwise UNRESOLVED. 'D != 1 detectable' marks a 95% interval that excludes 1 (expected by chance in about 5% of cell-arms; never a claim by itself). Gates: A/A = ratio D_main/D_aa 95% interval contains 1; NN = independent second null, D_nn = U_null/U_null2 95% interval contains 1 (calibrates the whole pipeline at truth D = 1); PLANT = planted positive control, recovered excess (U_plant - U_null2)/(J bases) (truth 1 by construction, exact floor: every lag has a hit), shown only when the excess is at least 3 sd above the count noise.\n").unwrap();
        writeln!(md, "| cell (a,n,m,l) | arm | bases | mean nx | mean nu | U_frob/model [95%] | U_null/model [95%] | D = frob/null [90%] | class | D != 1 detectable | A/A ratio [95%] | NN D_nn [95%] | PLANT recovery | gates |").unwrap();
        writeln!(
            md,
            "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
        )
        .unwrap();
        for (cell, idxs) in cells.iter() {
            let mut arms: BTreeMap<String, Vec<usize>> = BTreeMap::new();
            for &i in idxs {
                arms.entry(rows[i].arm.clone()).or_default().push(i);
            }
            for (arm, ai) in arms.iter() {
                let ok: Vec<usize> = ai
                    .iter()
                    .cloned()
                    .filter(|&i| rows[i].status == "OK")
                    .collect();
                let skipped = ai.len() - ok.len();
                if ok.is_empty() {
                    writeln!(md, "| {:?} | {arm} | 0 ({skipped} skipped) | | | | | | NOT_MEASURED | | | | | |", cell).unwrap();
                    continue;
                }
                let main: Vec<usize> = ok
                    .iter()
                    .cloned()
                    .filter(|&i| rows[i].half == "main")
                    .collect();
                let aa: Vec<usize> = ok
                    .iter()
                    .cloned()
                    .filter(|&i| rows[i].half == "aa")
                    .collect();
                let single = aa.is_empty();
                let seed = hash_str(&format!("{:?}{arm}", cell));
                let v = |idx: &[usize], f: &dyn Fn(&OvRow) -> f64| -> Vec<f64> {
                    idx.iter().map(|&i| f(&rows[i])).collect()
                };
                // pooled estimate over both halves (the A/A gate below decides whether pooling is allowed)
                let uf = v(&ok, &|r| r.uf);
                let un = v(&ok, &|r| r.un);
                let up = v(&ok, &|r| r.up);
                let u2 = v(&ok, &|r| r.u2);
                let rf = boot_ratio(&uf, &up, seed);
                let rn = boot_ratio(&un, &up, seed ^ 1);
                let d = boot_ratio(&uf, &un, seed ^ 2);
                let nn = boot_ratio(&un, &u2, seed ^ 5);
                let (aa_lo, aa_hi, aa_ok) = aa_gate(
                    &v(&main, &|r| r.uf),
                    &v(&main, &|r| r.un),
                    &v(&aa, &|r| r.uf),
                    &v(&aa, &|r| r.un),
                    seed ^ 4,
                );
                let jn = rows[ok[0]].jn;
                let nb = ok.len() as f64;
                let plant_pairs: Vec<(f64, f64)> = ok
                    .iter()
                    .filter(|&&i| rows[i].upl.is_finite())
                    .map(|&i| (rows[i].upl, rows[i].u2))
                    .collect();
                let (plant_txt, plant_miss, plant_informative) = if plant_pairs.is_empty() {
                    ("NA".to_string(), false, false)
                } else {
                    let pb = plant_pairs.len() as f64;
                    let excess: f64 = plant_pairs.iter().map(|p| p.0 - p.1).sum();
                    let noise: f64 = plant_pairs.iter().map(|p| p.0 + p.1).sum::<f64>().sqrt();
                    let rec = excess / (jn * pb);
                    let inform = jn * pb >= 3.0 * noise;
                    (
                        format!(
                            "{:.2}{}",
                            rec,
                            if inform { "" } else { " (not informative)" }
                        ),
                        inform && rec < 0.5,
                        inform,
                    )
                };
                let plok_bad = ok
                    .iter()
                    .any(|&i| rows[i].plok.is_finite() && rows[i].plok < rows[i].jn);
                let gates_bad: f64 = ok
                    .iter()
                    .map(|&i| rows[i].hb + rows[i].cpb + rows[i].cnb + rows[i].lbb)
                    .sum();
                let mut gate = if gates_bad == 0.0 && !plok_bad {
                    "pass".to_string()
                } else {
                    "INSTRUMENT_FAIL".to_string()
                };
                if gates_bad != 0.0 {
                    voids.push(format!(
                        "overlap cell {:?} arm {arm}: gate failures {gates_bad}",
                        cell
                    ));
                }
                if plok_bad {
                    voids.push(format!("overlap cell {:?} arm {arm}: planted positive control missed a guaranteed hit (counter failure)", cell));
                }
                let mut control_fail = false;
                if arm == "stable" || arm == "orbunion" {
                    let all_eq = ok.iter().all(|&i| (rows[i].ceq - 1.0).abs() < 1e-12);
                    if !all_eq {
                        voids.push(format!(
                            "overlap cell {:?} arm {arm}: not exactly equivariant (CONTROL_FAIL)",
                            cell
                        ));
                        gate = format!("CONTROL_FAIL({arm} equivariance)");
                        control_fail = true;
                    } else {
                        gate.push_str(&format!("; {arm} |Sol| equivariance = 1.000 exactly"));
                    }
                }
                let cal = rn.lo95 <= 1.0 && 1.0 <= rn.hi95;
                gate.push_str(if cal {
                    "; null calibrates the model"
                } else {
                    "; NULL_MISCALIBRATED (model column not used)"
                });
                if !aa_ok {
                    gate.push_str("; AA_FAIL");
                }
                let nn_miss = nn.lo95.is_finite() && !(nn.lo95 <= 1.0 && 1.0 <= nn.hi95);
                if nn_miss {
                    gate.push_str("; NN_MISS");
                }
                if plant_miss {
                    gate.push_str("; PLANT_MISS");
                }
                let censored = skipped as f64 / ai.len() as f64 > 0.05;
                // B4 (PROTOCOL 8.7): a positive Frobenius count on provably coincidence-free pairs voids the cell-arm
                let b4_viol = ok
                    .iter()
                    .any(|&i| rows[i].s1l_pairs.is_finite() && rows[i].s1l_uf > 0.0);
                if b4_viol {
                    gate.push_str("; B4_VIOLATION");
                }
                let class = if gates_bad != 0.0 || plok_bad || control_fail || b4_viol {
                    "VOID".to_string()
                } else if !aa_ok {
                    "AA_FAIL".to_string()
                } else if censored {
                    "CENSORED".to_string()
                } else if single {
                    "SINGLE_INSTANCE (no interval)".to_string()
                } else {
                    classify(&d, DELTA).to_string()
                };
                let detectable = d.lo95.is_finite() && (d.lo95 > 1.0 || d.hi95 < 1.0);
                if !single {
                    n_eval += 1;
                    n_aa_fail += (!aa_ok) as usize;
                    n_nn_miss += nn_miss as usize;
                    n_plant_miss += plant_miss as usize;
                    n_plant_inform += plant_informative as usize;
                    n_void += (class == "VOID") as usize;
                    n_cens += censored as usize;
                }
                results.push(CellArm {
                    cell: *cell,
                    arm: arm.clone(),
                    class: class.clone(),
                    weight: up.iter().sum::<f64>(),
                });
                let mnx = ok.iter().map(|&i| rows[i].nx).sum::<f64>() / nb;
                let mnu = ok.iter().map(|&i| rows[i].nu).sum::<f64>() / nb;
                writeln!(
                    md,
                    "| {:?} | {arm} | {} (skipped {skipped}) | {:.1} | {:.2} | {} | {} | {} | {class} | {} | {} | {} | {plant_txt} | {gate} |",
                    cell,
                    ok.len(),
                    mnx,
                    mnu,
                    fmt_ci95(&rf),
                    fmt_ci95(&rn),
                    fmt_ci90(&d),
                    if detectable { "yes" } else { "no" },
                    if aa_lo.is_finite() { format!("[{:.3}, {:.3}]", aa_lo, aa_hi) } else { "NA".into() },
                    fmt_ci95(&nn)
                )
                .unwrap();
                writeln!(
                    tsv,
                    "overlap\t{:?}\t{arm}\tD\t{:.5}\t{:.5}\t{:.5}\t{class}",
                    cell, d.pt, d.lo90, d.hi90
                )
                .unwrap();
                writeln!(
                    tsv,
                    "overlap\t{:?}\t{arm}\tD_nn\t{:.5}\t{:.5}\t{:.5}\t",
                    cell, nn.pt, nn.lo95, nn.hi95
                )
                .unwrap();
                writeln!(
                    tsv,
                    "overlap\t{:?}\t{arm}\tU_frob_over_model\t{:.5}\t{:.5}\t{:.5}\t",
                    cell, rf.pt, rf.lo95, rf.hi95
                )
                .unwrap();
                writeln!(
                    tsv,
                    "overlap\t{:?}\t{arm}\tU_null_over_model\t{:.5}\t{:.5}\t{:.5}\t",
                    cell, rn.pt, rn.lo95, rn.hi95
                )
                .unwrap();
            }
        }
        writeln!(md).unwrap();

        // gate bookkeeping and abort conditions on the instrument
        writeln!(
            md,
            "### B0. Gate bookkeeping over the {n_eval} cell-arms with an A/A split\n"
        )
        .unwrap();
        let fr = |x: usize| x as f64 / n_eval.max(1) as f64;
        writeln!(
            md,
            "A/A misses {n_aa_fail} ({:.1}%), NN misses {n_nn_miss} ({:.1}%), PLANT misses {n_plant_miss} of {n_plant_inform} informative ({:.1}%), VOID {n_void} ({:.1}%), CENSORED {n_cens} ({:.1}%). Chance expectation of an A/A or NN miss is about 5%; alarm above {:.0}%.\n",
            100.0 * fr(n_aa_fail),
            100.0 * fr(n_nn_miss),
            100.0 * n_plant_miss as f64 / n_plant_inform.max(1) as f64,
            100.0 * fr(n_void),
            100.0 * fr(n_cens),
            100.0 * MISS_ALARM
        )
        .unwrap();
        if fr(n_aa_fail) > MISS_ALARM {
            aborts.push(format!("A/A misses {n_aa_fail}/{n_eval} above {:.0}%: the pooled-estimate instrument is untrustworthy", 100.0 * MISS_ALARM));
        }
        if fr(n_nn_miss) > MISS_ALARM {
            aborts.push(format!("null-vs-null misses {n_nn_miss}/{n_eval} above {:.0}%: the decision pipeline is miscalibrated at truth D = 1", 100.0 * MISS_ALARM));
        }
        if n_plant_inform > 0 && n_plant_miss as f64 / n_plant_inform as f64 > MISS_ALARM {
            aborts.push(format!("planted control missed {n_plant_miss}/{n_plant_inform} informative cell-arms: the detector does not see a known excess"));
        }
        if fr(n_void) > 0.25 {
            aborts.push(format!("more than a quarter of cell-arms VOID ({n_void}/{n_eval}): instrument untrustworthy"));
        }
        if fr(n_cens) > 0.25 {
            aborts.push(format!("more than a quarter of cell-arms CENSORED ({n_cens}/{n_eval}): design under-resolved"));
        }

        // per-m statements (rule 5): four primary curves, E_0 (a = 0) family separately
        writeln!(
            md,
            "### B1. Per-m statements over the four primary curves (rule 5)\n"
        )
        .unwrap();
        writeln!(md, "Primary curves: (1,11), (0,13), (1,17), (0,19); the primary cell of a curve and m is the planned l with the largest total independence-model weight sum(U_pred) for that arm (chosen without looking at Frobenius or null hits). A statement for (arm, m) is made only if at least 3 of the 4 curves have the same class; a trend over m only if the class sequences over m are identical in at least 3 of 4 curves. The ECC2K-130 family E_0 is a = 0 (two toy curves, n = 13 and 19, cofactor 4); a = 1 (n = 11 and 17, cofactor 2; the ledger's K17 is an a = 1 model) is reported separately and the two families are never pooled into one claim.\n").unwrap();
        let primary: [(u32, u32); 4] = [(1, 11), (0, 13), (1, 17), (0, 19)];
        writeln!(md, "| arm | m | (1,11) | (0,13) | (1,17) | (0,19) | statement over 4 curves | E_0 family (a=0) | a=1 family |").unwrap();
        writeln!(md, "|---|---|---|---|---|---|---|---|---|").unwrap();
        let mut seqs: BTreeMap<String, Vec<(usize, Vec<String>)>> = BTreeMap::new();
        for arm in ["rand", "rset", "orbunion", "tuned"] {
            for m in 2..=8usize {
                let mut cls: Vec<String> = vec![];
                for &(a, n) in primary.iter() {
                    // primary cell = the planned l with the largest total independence-model weight sum(u_pred) of the arm
                    // (a function of the K(S) sizes only, independent of the Frobenius and null hits)
                    let c = results
                        .iter()
                        .filter(|r| r.arm == arm && r.cell.0 == a && r.cell.1 == n && r.cell.2 == m)
                        .max_by(|x, y| {
                            x.weight
                                .partial_cmp(&y.weight)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .map(|r| r.class.clone())
                        .unwrap_or_else(|| "-".into());
                    cls.push(c);
                }
                let mut counts: BTreeMap<&String, usize> = BTreeMap::new();
                for c in &cls {
                    if c != "-" {
                        *counts.entry(c).or_default() += 1;
                    }
                }
                let best = counts
                    .iter()
                    .max_by_key(|(_, &v)| v)
                    .map(|(k, &v)| ((*k).clone(), v));
                let stmt = match &best {
                    Some((k, v)) if *v >= 3 => format!("{k} ({v}/4)"),
                    _ => "NO_MAJORITY".to_string(),
                };
                let fam = |i: usize, j: usize| {
                    if cls[i] == cls[j] && cls[i] != "-" {
                        format!("both {}", cls[i])
                    } else {
                        format!("{} / {}", cls[i], cls[j])
                    }
                };
                writeln!(
                    md,
                    "| {arm} | {m} | {} | {} | {} | {} | {stmt} | {} | {} |",
                    cls[0],
                    cls[1],
                    cls[2],
                    cls[3],
                    fam(1, 3),
                    fam(0, 2)
                )
                .unwrap();
                writeln!(tsv, "perm\tm{m}\t{arm}\tstatement\t0\t\t\t{stmt}").unwrap();
                for (ci, c) in cls.iter().enumerate() {
                    seqs.entry(format!("{arm}|{ci}"))
                        .or_default()
                        .push((m, vec![c.clone()]));
                }
            }
        }
        writeln!(md).unwrap();
        for arm in ["rand", "rset", "orbunion", "tuned"] {
            let mut sq: Vec<String> = vec![];
            for ci in 0..4 {
                let s = seqs
                    .get(&format!("{arm}|{ci}"))
                    .map(|v| {
                        v.iter()
                            .map(|(_, c)| c[0].clone())
                            .collect::<Vec<_>>()
                            .join(",")
                    })
                    .unwrap_or_default();
                sq.push(s);
            }
            let best = trend_agreement(&sq);
            writeln!(
                md,
                "- trend in m, arm {arm}: class sequence identical in {best} of 4 curves -> {}",
                if best >= 3 {
                    "TREND STATEMENT ALLOWED (sequence above)"
                } else {
                    "no trend statement"
                }
            )
            .unwrap();
        }
        writeln!(md).unwrap();

        // indicator correlation
        writeln!(md, "### B2. Indicator autocorrelation along the orbit (mean over lags), Frobenius minus null\n").unwrap();
        writeln!(
            md,
            "| cell | arm | mean corr_frob - corr_null [95%] | mean corr_frob |"
        )
        .unwrap();
        writeln!(md, "|---|---|---|---|").unwrap();
        for (cell, idxs) in cells.iter() {
            let mut arms: BTreeMap<String, Vec<f64>> = BTreeMap::new();
            let mut armf: BTreeMap<String, Vec<f64>> = BTreeMap::new();
            for &i in idxs {
                let r = &rows[i];
                if r.status == "OK" {
                    arms.entry(r.arm.clone()).or_default().push(r.cf - r.cn);
                    armf.entry(r.arm.clone()).or_default().push(r.cf);
                }
            }
            for (arm, x) in arms.iter() {
                let b = boot_mean(x, hash_str(&format!("{:?}{arm}corr", cell)));
                let mf = armf[arm].iter().filter(|v| v.is_finite()).sum::<f64>()
                    / armf[arm].iter().filter(|v| v.is_finite()).count().max(1) as f64;
                writeln!(md, "| {:?} | {arm} | {} | {:.4} |", cell, fmt3(b), mf).unwrap();
            }
        }
        writeln!(md).unwrap();

        // m trend on the yield-matched arm with a bootstrap interval of the slope
        writeln!(md, "### B3. Per-solution, per-lag overlap rate vs m (arm `tuned`, nu matched near 4), log2 slope\n").unwrap();
        writeln!(md, "Model: rate_pred = U_pred / (nsol * J). Observed: U_frob / (nsol * J), pooled over bases. The slope of log2(rate_obs) on m has a 95% bootstrap interval (bases resampled within each m, 400 resamples); the model slope is shown beside it. The ledger's prediction is about +1 per unit m until 2^m approaches r.\n").unwrap();
        writeln!(
            md,
            "| curve (a,n) | m | rate_obs | rate_null | rate_pred | obs/pred |"
        )
        .unwrap();
        writeln!(md, "|---|---|---|---|---|---|").unwrap();
        type PerM = BTreeMap<usize, Vec<(f64, f64, f64, f64)>>; // m -> bases (uf, un, up, den)
        let mut curves: BTreeMap<(u32, u32), PerM> = BTreeMap::new();
        for (cell, idxs) in cells.iter() {
            let tn: Vec<&OvRow> = idxs
                .iter()
                .map(|&i| &rows[i])
                .filter(|r| r.arm == "tuned" && r.status == "OK")
                .collect();
            if tn.is_empty() {
                continue;
            }
            // tuned does not depend on l: keep the first l per (curve, m)
            let e = curves
                .entry((cell.0, cell.1))
                .or_default()
                .entry(cell.2)
                .or_default();
            if e.is_empty() {
                for r in tn {
                    e.push((r.uf, r.un, r.up, r.nsol * r.jn));
                }
            }
        }
        let rate = |b: &[(f64, f64, f64, f64)], which: usize| -> f64 {
            let den: f64 = b.iter().map(|x| x.3).sum();
            b.iter().map(|x| [x.0, x.1, x.2][which]).sum::<f64>() / den
        };
        let mut svg_series: Vec<(String, Vec<(f64, f64)>, Vec<(f64, f64)>)> = vec![];
        for (cv, ms) in curves.iter() {
            let mut obs = vec![];
            let mut prd = vec![];
            for (m, b) in ms.iter() {
                let (o, nl, pr) = (rate(b, 0), rate(b, 1), rate(b, 2));
                writeln!(
                    md,
                    "| {:?} | {m} | {:.3e} | {:.3e} | {:.3e} | {:.3} |",
                    cv,
                    o,
                    nl,
                    pr,
                    o / pr
                )
                .unwrap();
                if o > 0.0 {
                    obs.push((*m as f64, o.log2()));
                }
                prd.push((*m as f64, pr.log2()));
            }
            if obs.len() >= 3 {
                let (x, y): (Vec<f64>, Vec<f64>) = (
                    obs.iter().map(|p| p.0).collect(),
                    obs.iter().map(|p| p.1).collect(),
                );
                let (xp, yp): (Vec<f64>, Vec<f64>) = (
                    prd.iter().map(|p| p.0).collect(),
                    prd.iter().map(|p| p.1).collect(),
                );
                let mut rng = Rng::new(hash_str(&format!("slope{:?}", cv)));
                let mut sl: Vec<f64> = vec![];
                for _ in 0..400 {
                    let (mut bx, mut by) = (vec![], vec![]);
                    for (m, b) in ms.iter() {
                        if b.len() < 2 {
                            continue;
                        }
                        let rs: Vec<(f64, f64, f64, f64)> = (0..b.len())
                            .map(|_| b[rng.below(b.len() as u64) as usize])
                            .collect();
                        let o = rate(&rs, 0);
                        if o > 0.0 {
                            bx.push(*m as f64);
                            by.push(o.log2());
                        }
                    }
                    if bx.len() >= 3 {
                        sl.push(slope(&bx, &by));
                    }
                }
                let (slo, shi) = (pct(&mut sl.clone(), 0.025), pct(&mut sl, 0.975));
                writeln!(md, "| {:?} | slope(log2 rate_obs on m) = {:.3} [{:.3}, {:.3}]; slope(log2 rate_pred on m) = {:.3} | | | | |", cv, slope(&x, &y), slo, shi, slope(&xp, &yp)).unwrap();
                writeln!(
                    tsv,
                    "overlap\t{:?}\ttuned\tslope_obs\t{:.4}\t{:.4}\t{:.4}\t",
                    cv,
                    slope(&x, &y),
                    slo,
                    shi
                )
                .unwrap();
                writeln!(
                    tsv,
                    "overlap\t{:?}\ttuned\tslope_pred\t{:.4}\t\t\t",
                    cv,
                    slope(&xp, &yp)
                )
                .unwrap();
            }
            svg_series.push((format!("a={} n={}", cv.0, cv.1), obs, prd));
        }
        writeln!(md).unwrap();
        let _ = std::fs::write(format!("{dir}/fig_overlap_vs_m.svg"), svg(&svg_series));

        // B4: the proved zero on low-height single-orbit pairs
        writeln!(md, "### B4. Low-height single-orbit pairs (stratum 1L): Frobenius coincidences are impossible\n").unwrap();
        writeln!(md, "Theorem (PROTOCOL 3.4, derived here, checked exhaustively by a unit test): if all legs of a tuple lie in one sigma-orbit with circular spread s and the lag j satisfies m^2 2^s (2^(j/2)+1)^2 < r, then U_frob = 0 on that (tuple, lag) pair. Observed U_frob must be 0 exactly; any positive count is a counterexample to the derivation or an instrument error (VOID). The test is informative only if the null is expected to hit at least 5 times on the same pairs (E_null = sum c(c-1) / ((r-3)/2)); otherwise the cell-arm is NOT_INFORMATIVE and supports nothing.\n").unwrap();
        writeln!(
            md,
            "| cell | arm | pairs | E_null | U_null observed | U_frob observed | verdict |"
        )
        .unwrap();
        writeln!(md, "|---|---|---|---|---|---|---|").unwrap();
        let (mut b4_inf, mut b4_bad) = (0usize, 0usize);
        for (cell, idxs) in cells.iter() {
            let mut arms: BTreeMap<String, Vec<&OvRow>> = BTreeMap::new();
            for &i in idxs {
                if rows[i].status == "OK" && rows[i].s1l_pairs.is_finite() {
                    arms.entry(rows[i].arm.clone()).or_default().push(&rows[i]);
                }
            }
            for (arm, rs) in arms.iter() {
                let pairs: f64 = rs.iter().map(|r| r.s1l_pairs).sum();
                if pairs == 0.0 {
                    continue;
                }
                let e_null: f64 = rs.iter().map(|r| r.s1l_cc1 / ((r.r - 3.0) / 2.0)).sum();
                let (un, uf): (f64, f64) = (
                    rs.iter().map(|r| r.s1l_un).sum(),
                    rs.iter().map(|r| r.s1l_uf).sum(),
                );
                let verdict = if uf > 0.0 {
                    b4_bad += 1;
                    voids.push(format!("B4 cell {:?} arm {arm}: {uf} Frobenius coincidences on provably coincidence-free pairs", cell));
                    "VIOLATION (derivation or instrument)".to_string()
                } else if e_null >= 5.0 {
                    b4_inf += 1;
                    format!(
                        "CONSISTENT (informative: P(zero null hits under the model) = {:.1e})",
                        (-e_null).exp()
                    )
                } else {
                    "NOT_INFORMATIVE (E_null < 5)".to_string()
                };
                writeln!(
                    md,
                    "| {:?} | {arm} | {pairs} | {e_null:.2} | {un} | {uf} | {verdict} |",
                    cell
                )
                .unwrap();
                writeln!(
                    tsv,
                    "overlap\t{:?}\t{arm}\tB4_1L_uf\t{uf}\t{e_null:.3}\t{un}\t{verdict}",
                    cell
                )
                .unwrap();
            }
        }
        writeln!(
            md,
            "\nInformative and consistent cell-arms: {b4_inf}; violations: {b4_bad}.\n"
        )
        .unwrap();

        // stratified by number of distinct sigma-orbits among the legs (descriptive)
        writeln!(md, "### B5. Overlap by number of distinct sigma-orbits among the legs (descriptive, pooled over bases; no decision rule)\n").unwrap();
        writeln!(md, "Strata 1, 2, >= 3 distinct orbits. Stratum 1 contains both low-height pairs (provably zero, B4) and high-height pairs where the theorem is silent, so D in stratum 1 is a diluted descriptive number; the claim-bearing statement is B4.\n").unwrap();
        writeln!(md, "| cell | arm | stratum | subsets | solution pairs | U_frob | U_null | D = frob/null [95%] |").unwrap();
        writeln!(md, "|---|---|---|---|---|---|---|---|").unwrap();
        for p in &ofiles {
            let t = Table::read(p)?;
            let mut by: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
            for r in 0..t.rows.len() {
                if t.s(r, "status") == "OK" {
                    by.entry((
                        format!(
                            "a{}_n{}_m{}_l{}",
                            t.s(r, "a"),
                            t.s(r, "n"),
                            t.s(r, "m"),
                            t.s(r, "l")
                        ),
                        t.s(r, "arm").to_string(),
                    ))
                    .or_default()
                    .push(r);
                }
            }
            for ((cell, arm), ri) in by.iter() {
                for sx in 1..=3 {
                    let g = |c: &str| -> Vec<f64> {
                        ri.iter().map(|&r| t.f(r, &format!("s{sx}_{c}"))).collect()
                    };
                    let (ns, nl, uf, un) = (g("nsub"), g("nsol"), g("uf"), g("un"));
                    let tot_n: f64 = ns.iter().sum();
                    if tot_n == 0.0 {
                        continue;
                    }
                    let d = boot_ratio(&uf, &un, hash_str(&format!("{cell}{arm}{sx}")));
                    writeln!(
                        md,
                        "| {cell} | {arm} | {sx} | {} | {} | {} | {} | {} |",
                        tot_n,
                        nl.iter().sum::<f64>(),
                        uf.iter().sum::<f64>(),
                        un.iter().sum::<f64>(),
                        fmt_ci95(&d)
                    )
                    .unwrap();
                    writeln!(
                        tsv,
                        "overlap\t{cell}\t{arm}\tD_stratum{sx}\t{:.5}\t{:.5}\t{:.5}\t",
                        d.pt, d.lo95, d.hi95
                    )
                    .unwrap();
                }
            }
        }
        writeln!(md).unwrap();

        // Proposition Sigma closure
        let (mut cp, mut cc) = (0.0, 0.0);
        for r in rows.iter().filter(|r| r.status == "OK") {
            if r.closp.is_finite() {
                cp += r.closp;
                cc += r.clos;
            }
        }
        writeln!(
            md,
            "### B6. Proposition Sigma (setwise closure of Sol(R,V) under sigma^j)\n"
        )
        .unwrap();
        writeln!(md, "(target, lag) pairs checked: {cp}; closed: {cc}. Proved for 2^(m-1) < n (a closed pair needs the whole lambda-orbit of n classes inside K(S), |K(S)| <= 2^(m-1)); for larger m the expected count is 0 by a probability argument, not a proof, so a nonzero count there is a headline, not an instrument error, and is replicated before being read.\n").unwrap();
        if cc != 0.0 {
            voids.push(format!("Proposition Sigma closure count {cc} != 0 (instrument error if 2^(m-1) < n, otherwise a finding to replicate)"));
        }
    }

    // ---------------- rank ----------------
    let rfiles = files(dir, "rank_");
    if !rfiles.is_empty() {
        writeln!(
            md,
            "## C. Rank of conjugate relations (plain vs orbit basis)\n"
        )
        .unwrap();
        writeln!(md, "The row cap is lowered per base to at most a third of the orbits the base touches, so rank_orbit(N) cannot saturate by pigeonhole; the independent-target control is row-matched ((n-1) N rows). A cell is INFORMATIVE only if the wrong-lambda control and the independent control both grew the orbit rank in at least one base while rank_orbit(1) < columns.\n").unwrap();
        writeln!(md, "| cell | arm | bases | mean rank_orbit(N) | mean cols | rank_orbit(nN) == rank_orbit(N) in all bases | non-proportional conjugate rows | rows failing identity | wrong-lambda control grows (frac) | independent-target control grows (frac) | indep rows matched (frac) | mean rank_plain(nN)/rank_plain(N) | informative |").unwrap();
        writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
        let (mut rk_cells, mut rk_inf) = (0usize, 0usize);
        for p in &rfiles {
            let t = Table::read(p)?;
            let mut by: BTreeMap<String, Vec<usize>> = BTreeMap::new();
            for r in 0..t.rows.len() {
                if t.s(r, "n_rel") != "NA" && t.f(r, "n_rel") > 0.0 {
                    by.entry(t.s(r, "arm").to_string()).or_default().push(r);
                }
            }
            for (arm, ri) in by.iter() {
                let eq = ri
                    .iter()
                    .all(|&r| t.f(r, "rank_orbit_n") == t.f(r, "rank_orbit_1"));
                let np: f64 = ri.iter().map(|&r| t.f(r, "nonproportional")).sum();
                let rb: f64 = ri
                    .iter()
                    .map(|&r| t.f(r, "rows_bad") + t.f(r, "incons"))
                    .sum();
                let nb = ri.len() as f64;
                let wg = ri
                    .iter()
                    .filter(|&&r| t.f(r, "rank_orbit_wrong") > t.f(r, "rank_orbit_n"))
                    .count() as f64
                    / nb;
                let ig = ri
                    .iter()
                    .filter(|&&r| t.f(r, "rank_orbit_indep") > t.f(r, "rank_orbit_1"))
                    .count() as f64
                    / nb;
                let mt = ri
                    .iter()
                    .filter(|&&r| t.f(r, "n_rel_indep") >= t.f(r, "indep_need"))
                    .count() as f64
                    / nb;
                let pl: f64 = ri
                    .iter()
                    .map(|&r| t.f(r, "rank_plain_n") / t.f(r, "rank_plain_1").max(1.0))
                    .sum::<f64>()
                    / nb;
                let mo: f64 = ri.iter().map(|&r| t.f(r, "rank_orbit_1")).sum::<f64>() / nb;
                let mc: f64 = ri.iter().map(|&r| t.f(r, "cols_orbit")).sum::<f64>() / nb;
                let cell = format!(
                    "a{}_n{}_m{}_l{}",
                    t.s(ri[0], "a"),
                    t.s(ri[0], "n"),
                    t.s(ri[0], "m"),
                    t.s(ri[0], "l")
                );
                let informative = wg > 0.0 && ig > 0.0;
                rk_cells += 1;
                rk_inf += informative as usize;
                let ok = eq && np == 0.0 && rb == 0.0;
                if !ok {
                    voids.push(format!(
                        "rank cell {cell} {arm}: orbit-basis equality failed (INSTRUMENT_FAIL)"
                    ));
                }
                writeln!(
                    md,
                    "| {cell} | {arm} | {} | {:.1} | {:.1} | {} | {np} | {rb} | {:.2} | {:.2} | {:.2} | {:.2} | {} |",
                    ri.len(), mo, mc, if ok { "yes" } else { "NO" }, wg, ig, mt, pl, if informative { "INFORMATIVE" } else { "NOT_INFORMATIVE" }
                )
                .unwrap();
            }
        }
        writeln!(md, "\nRank cell-arms: {rk_cells}; informative: {rk_inf}. NOT_INFORMATIVE cells confirm nothing beyond the regression check.\n").unwrap();
    }

    // ---------------- status, voids, limits ----------------
    writeln!(md, "## Abort conditions (RUN_CARD section 5)\n").unwrap();
    if aborts.is_empty() && incomplete.is_empty() {
        writeln!(md, "None triggered.\n").unwrap();
    } else {
        for a in &aborts {
            writeln!(md, "- ABORT: {a}").unwrap();
        }
        if !incomplete.is_empty() {
            writeln!(md, "- INCOMPLETE: {} job(s) did not finish with their expected rows (listed in section 0); the run is reported as incomplete.", incomplete.len()).unwrap();
        }
        writeln!(md).unwrap();
    }
    writeln!(md, "## Voids and failures\n").unwrap();
    if voids.is_empty() {
        writeln!(md, "None.\n").unwrap();
    } else {
        for v in &voids {
            writeln!(md, "- {v}").unwrap();
        }
        writeln!(md).unwrap();
    }
    writeln!(md, "## What this does not establish\n").unwrap();
    writeln!(md, "- No solver, no cost per relation, no wall-time comparison, no comparison with Pollard rho, no statement about ECC2K-130 beyond the arithmetic in PROTOCOL.md section 9. Nothing here is an improvement over the index-calculus baseline.").unwrap();
    writeln!(md, "- Toy sizes only (n in {{11, 13, 17, 19}}, r between 991 and 262543). **Nothing here discharges the m = 83 gate of AGENTS.md 8a** (E_0 over GF(2^83)), and any transfer to the 131-bit challenge field is extrapolation. The toy fields differ structurally from 83 and 131: ord_n(2) = 10, 12, 8, 18 for n = 11, 13, 17, 19, so only n = 17 has sigma-stable subspaces (the `stable` arm, which cannot transfer), while 83 and 131 have ord_83(2) = 82 and ord_131(2) = 130 (irreducible nontrivial block, no stable subspaces).").unwrap();
    writeln!(md, "- The overlap census is exact on toy groups where 2^m approaches or exceeds r; the ratio-to-model at m close to n is a saturation statement, not an asymptotic one.").unwrap();
    writeln!(md, "- The factor base is the set of x-coordinates of points of the prime-order subgroup G with x in V; legs outside G (cosets of the 2-torsion) and twist legs are excluded in every arm. Conditioning on at least m classes (bases below that are skipped) slightly biases the surviving bases.").unwrap();
    writeln!(md, "- Notation: this cell uses n for the field degree and m for the number of legs; AGENTS.md 8a/8b use m for the field degree.").unwrap();
    std::fs::write(format!("{dir}/summary.md"), md).map_err(|e| e.to_string())?;
    std::fs::write(format!("{dir}/summary.tsv"), tsv).map_err(|e| e.to_string())?;
    Ok(())
}

/// Poisson draw (Knuth for small means, normal approximation above 60). Used only by the design-stage power table.
fn poisson(rng: &mut Rng, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        return 0.0;
    }
    if lambda < 60.0 {
        let l = (-lambda).exp();
        let (mut k, mut p) = (0.0, 1.0);
        loop {
            p *= rng.f64();
            if p <= l {
                return k;
            }
            k += 1.0;
        }
    }
    let (u1, u2) = (rng.f64().max(1e-300), rng.f64());
    let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    (lambda + lambda.sqrt() * z).round().max(0.0)
}

/// Operating characteristic of the decision rule on synthetic Poisson counts (design stage, no experiment data):
/// `u_per_base` expected null hits per base, `bases_per_half` bases in each of the two halves, truth D = `d_true`.
/// Returns the fractions of simulations ending in [E-OVERLAP, E-EXTRA, E-DEFICIT, UNRESOLVED, AA_FAIL].
/// Counts are independent Poisson per base, so between-base overdispersion is NOT included: real intervals are wider.
pub fn oc_probs(
    u_per_base: f64,
    bases_per_half: usize,
    d_true: f64,
    sims: usize,
    seed: u64,
) -> [f64; 5] {
    let mut rng = Rng::new(seed);
    let mut tally = [0usize; 5];
    for _ in 0..sims {
        let mut gen = |k: usize| -> (Vec<f64>, Vec<f64>) {
            let uf: Vec<f64> = (0..k)
                .map(|_| poisson(&mut rng, d_true * u_per_base))
                .collect();
            let un: Vec<f64> = (0..k).map(|_| poisson(&mut rng, u_per_base)).collect();
            (uf, un)
        };
        let (mf, mn) = gen(bases_per_half);
        let (af, an) = gen(bases_per_half);
        let (_, _, ok) = aa_gate(&mf, &mn, &af, &an, rng.next_u64());
        if !ok {
            tally[4] += 1;
            continue;
        }
        let uf: Vec<f64> = mf.iter().chain(af.iter()).cloned().collect();
        let un: Vec<f64> = mn.iter().chain(an.iter()).cloned().collect();
        let c = boot_ratio(&uf, &un, rng.next_u64());
        match classify(&c, DELTA) {
            "E-OVERLAP" => tally[0] += 1,
            "E-EXTRA" => tally[1] += 1,
            "E-DEFICIT" => tally[2] += 1,
            _ => tally[3] += 1,
        }
    }
    let mut out = [0.0; 5];
    for i in 0..5 {
        out[i] = tally[i] as f64 / sims as f64;
    }
    out
}

fn svg(series: &[(String, Vec<(f64, f64)>, Vec<(f64, f64)>)]) -> String {
    let (w, h, ml, mb, mt, mr) = (720.0, 420.0, 70.0, 50.0, 30.0, 150.0);
    let mut xs: Vec<f64> = vec![];
    let mut ys: Vec<f64> = vec![];
    for (_, o, p) in series {
        for q in o.iter().chain(p.iter()) {
            xs.push(q.0);
            ys.push(q.1);
        }
    }
    if xs.is_empty() {
        return "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"/>".into();
    }
    let (x0, x1) = (
        xs.iter().cloned().fold(f64::MAX, f64::min),
        xs.iter().cloned().fold(f64::MIN, f64::max),
    );
    let (y0, y1) = (
        ys.iter().cloned().fold(f64::MAX, f64::min) - 0.5,
        ys.iter().cloned().fold(f64::MIN, f64::max) + 0.5,
    );
    let px = |x: f64| ml + (x - x0) / (x1 - x0).max(1e-9) * (w - ml - mr);
    let py = |y: f64| h - mb - (y - y0) / (y1 - y0).max(1e-9) * (h - mb - mt);
    let colors = ["#1f77b4", "#d95f02", "#1b9e77", "#7570b3", "#e7298a"];
    let mut s = String::new();
    write!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"12\">").unwrap();
    write!(s, "<rect width=\"{w}\" height=\"{h}\" fill=\"#ffffff\"/>").unwrap();
    write!(s, "<text x=\"{ml}\" y=\"18\" font-size=\"14\">log2 of overlap rate per solution per lag vs m (circles observed, dashed model)</text>").unwrap();
    write!(
        s,
        "<line x1=\"{ml}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#333\"/>",
        h - mb,
        w - mr,
        h - mb
    )
    .unwrap();
    write!(
        s,
        "<line x1=\"{ml}\" y1=\"{mt}\" x2=\"{ml}\" y2=\"{}\" stroke=\"#333\"/>",
        h - mb
    )
    .unwrap();
    let mut m = x0.ceil();
    while m <= x1 {
        write!(
            s,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
            px(m),
            h - mb + 16.0,
            m
        )
        .unwrap();
        m += 1.0;
    }
    write!(
        s,
        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">m</text>",
        (ml + w - mr) / 2.0,
        h - 10.0
    )
    .unwrap();
    let mut y = y0.ceil();
    while y <= y1 {
        write!(
            s,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{}</text>",
            ml - 6.0,
            py(y) + 4.0,
            y
        )
        .unwrap();
        write!(
            s,
            "<line x1=\"{ml}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#ddd\"/>",
            py(y),
            w - mr,
            py(y)
        )
        .unwrap();
        y += 2.0;
    }
    for (i, (name, obs, prd)) in series.iter().enumerate() {
        let c = colors[i % colors.len()];
        let pts: Vec<String> = prd
            .iter()
            .map(|p| format!("{},{}", px(p.0), py(p.1)))
            .collect();
        write!(
            s,
            "<polyline fill=\"none\" stroke=\"{c}\" stroke-dasharray=\"5 4\" points=\"{}\"/>",
            pts.join(" ")
        )
        .unwrap();
        for p in obs {
            write!(
                s,
                "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{c}\"/>",
                px(p.0),
                py(p.1)
            )
            .unwrap();
        }
        write!(s, "<rect x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" fill=\"{c}\"/><text x=\"{}\" y=\"{}\">{}</text>", w - mr + 10.0, mt + 16.0 * i as f64, w - mr + 26.0, mt + 9.0 + 16.0 * i as f64, name).unwrap();
    }
    s.push_str("</svg>");
    s
}
