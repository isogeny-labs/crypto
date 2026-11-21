//! Paired analysis of the census rows: ratios vs the plain arm with bootstrap
//! intervals, the A/A noise floor and minimum detectable effect, controls and
//! the frozen decision rule (PROTOCOL.md section 7). No arm labels are
//! interpreted here beyond the names in the frozen comparison list.

use crate::curve::Rng;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Row {
    pub cell: String,
    pub n: u32,
    pub m: usize,
    pub l: u32,
    pub curve_a: u32,
    pub round: u32,
    pub tidx: usize,
    pub xr: u64,
    pub kind: String,
    pub truth: String,
    pub arm: String,
    pub slot: u32,
    pub result: String,
    pub verdict_ok: bool,
    pub cert_ok: String,
    pub conflicts: f64,
    pub total_ops: f64,
    pub search_ops: f64,
    pub encode_ops: f64,
    pub wall_ns: f64,
    pub rss_kb: f64,
}

pub fn parse_rows(text: &str) -> Vec<Row> {
    let mut lines = text.lines();
    let head: Vec<&str> = match lines.next() {
        Some(h) => h.split('\t').collect(),
        None => return vec![],
    };
    let ix = |name: &str| {
        head.iter()
            .position(|h| *h == name)
            .unwrap_or_else(|| panic!("column {name}"))
    };
    let (
        c_cell,
        c_n,
        c_m,
        c_l,
        c_a,
        c_r,
        c_t,
        c_xr,
        c_k,
        c_truth,
        c_arm,
        c_slot,
        c_res,
        c_vok,
        c_cok,
        c_conf,
        c_tot,
        c_so,
        c_eo,
        c_wall,
        c_rss,
    ) = (
        ix("cell"),
        ix("n"),
        ix("m"),
        ix("l"),
        ix("curve_a"),
        ix("round"),
        ix("tidx"),
        ix("xr"),
        ix("kind"),
        ix("truth"),
        ix("arm"),
        ix("slot"),
        ix("result"),
        ix("verdict_ok"),
        ix("cert_ok"),
        ix("conflicts"),
        ix("total_ops"),
        ix("search_ops"),
        ix("encode_ops"),
        ix("wall_ns"),
        ix("rss_kb"),
    );
    lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            Row {
                cell: f[c_cell].to_string(),
                n: f[c_n].parse().unwrap(),
                m: f[c_m].parse().unwrap(),
                l: f[c_l].parse().unwrap(),
                curve_a: f[c_a].parse().unwrap(),
                round: f[c_r].parse().unwrap(),
                tidx: f[c_t].parse().unwrap(),
                xr: f[c_xr].parse().unwrap(),
                kind: f[c_k].to_string(),
                truth: f[c_truth].to_string(),
                arm: f[c_arm].to_string(),
                slot: f[c_slot].parse().unwrap(),
                result: f[c_res].to_string(),
                verdict_ok: f[c_vok] == "1",
                cert_ok: f[c_cok].to_string(),
                conflicts: f[c_conf].parse().unwrap(),
                total_ops: f[c_tot].parse().unwrap(),
                search_ops: f[c_so].parse().unwrap(),
                encode_ops: f[c_eo].parse().unwrap(),
                wall_ns: f[c_wall].parse().unwrap(),
                rss_kb: f[c_rss].parse().unwrap(),
            }
        })
        .collect()
}

pub fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

/// Percentile bootstrap (B resamples) of the median; returns (lo, hi) at 95 %.
pub fn boot_median_ci(vals: &[f64], b: usize, seed: u64) -> (f64, f64) {
    let n = vals.len();
    if n == 0 {
        return (f64::NAN, f64::NAN);
    }
    let mut rng = Rng(seed);
    let mut meds = Vec::with_capacity(b);
    let mut buf = vec![0.0; n];
    for _ in 0..b {
        for x in buf.iter_mut() {
            *x = vals[rng.below(n as u64) as usize];
        }
        let mut c = buf.clone();
        meds.push(median(&mut c));
    }
    meds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let lo = meds[((0.025 * b as f64) as usize).min(b - 1)];
    let hi = meds[((0.975 * b as f64) as usize).min(b - 1)];
    (lo, hi)
}

#[derive(Clone, Debug)]
pub struct Cmp {
    pub tag: &'static str, // primary | replicate | control | aa
    pub num: String,
    pub den: String,
    pub num_arm: String,
    pub den_arm: String,
    pub metric: &'static str,
    pub n_targets: usize,
    pub n_pairs: usize,
    pub median_ratio: f64,
    pub lo: f64,
    pub hi: f64,
    pub gm_ratio: f64,
    pub frac_below_095: f64,
    pub sd_log: f64,
}

// A target is identified by (cell, curve, round, tidx, xr): chunked runs (--seed-offset) restart
// tidx at 0 with different targets, replicate-slot files (--tag) repeat the same targets (same xr).
type Key = (String, u32, u32, usize, u64);

fn metric_value(r: &Row, metric: &str) -> f64 {
    match metric {
        "conflicts" => r.conflicts,
        "ops" => r.search_ops + r.encode_ops, // w = 1: encoder work priced at 1 word-op per literal
        "sops" => r.search_ops,               // w = 0: encoder work unpriced
        "ops01" => r.search_ops + 0.1 * r.encode_ops, // w = 0.1
        "wall" => r.wall_ns,
        _ => panic!("metric"),
    }
}

/// Paired ratios num/den over UNSAT-truth, UNSAT-result, non-censored pairs.
/// `pairs` lists (numerator slot, denominator slot); one ratio per target and pair.
/// The bootstrap resamples TARGETS (clusters), keeping a target's slots together.
pub fn compare_pairs(
    rows: &[Row],
    num: &str,
    den: &str,
    pairs: &[(u32, u32)],
    metric: &'static str,
    tag: &'static str,
    seed: u64,
) -> Cmp {
    let mut by: BTreeMap<(Key, String, u32), f64> = BTreeMap::new();
    for r in rows
        .iter()
        .filter(|r| r.kind == "unsat" && r.truth == "UNSAT" && r.result == "UNSAT")
    {
        by.insert(
            (
                (r.cell.clone(), r.curve_a, r.round, r.tidx, r.xr),
                r.arm.clone(),
                r.slot,
            ),
            metric_value(r, metric),
        );
    }
    let mut clusters: BTreeMap<Key, Vec<f64>> = BTreeMap::new();
    for ((k, arm, slot), x) in &by {
        if arm != num {
            continue;
        }
        for &(sn, sd) in pairs {
            if *slot == sn {
                if let Some(y) = by.get(&(k.clone(), den.to_string(), sd)) {
                    if *x > 0.0 && *y > 0.0 {
                        clusters.entry(k.clone()).or_default().push(x / y);
                    }
                }
            }
        }
    }
    let cl: Vec<Vec<f64>> = clusters.into_values().collect();
    let all: Vec<f64> = cl.iter().flatten().copied().collect();
    let n = all.len();
    let logs: Vec<f64> = all.iter().map(|r| r.ln()).collect();
    let mean = if n > 0 {
        logs.iter().sum::<f64>() / n as f64
    } else {
        f64::NAN
    };
    let var = if n > 1 {
        logs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n as f64 - 1.0)
    } else {
        f64::NAN
    };
    let mut rv = all.clone();
    // cluster bootstrap of the median
    let (lo, hi) = if cl.is_empty() {
        (f64::NAN, f64::NAN)
    } else {
        let mut rng = Rng(seed);
        let mut meds = Vec::with_capacity(10_000);
        for _ in 0..10_000 {
            let mut buf = vec![];
            for _ in 0..cl.len() {
                buf.extend_from_slice(&cl[rng.below(cl.len() as u64) as usize]);
            }
            meds.push(median(&mut buf));
        }
        meds.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (meds[250], meds[9750])
    };
    let lab = |arm: &str, side: usize| {
        format!(
            "{arm}@{}",
            pairs
                .iter()
                .map(|p| (if side == 0 { p.0 } else { p.1 }).to_string())
                .collect::<Vec<_>>()
                .join("+")
        )
    };
    Cmp {
        tag,
        num: lab(num, 0),
        den: lab(den, 1),
        num_arm: num.into(),
        den_arm: den.into(),
        metric,
        n_targets: cl.len(),
        n_pairs: n,
        median_ratio: median(&mut rv),
        lo,
        hi,
        gm_ratio: mean.exp(),
        frac_below_095: if n > 0 {
            all.iter().filter(|r| **r < 0.95).count() as f64 / n as f64
        } else {
            f64::NAN
        },
        sd_log: var.sqrt(),
    }
}

/// Minimum detectable ratio (80 % power, 5 % two-sided) for a median of n
/// paired log-ratios with sd s: exp(-2.8 * 1.2533 * s / sqrt(n)).
pub fn mde(sd_log: f64, n: usize) -> f64 {
    (-(2.8 * 1.2533 * sd_log) / (n as f64).sqrt()).exp()
}

#[derive(Clone, Debug, PartialEq)]
pub enum Label {
    Relabel,
    Structure(String),
    /// m = 3 B-only cell: an interval statement, no structure claim.
    Bounded(String),
    Artifact(String),
    Impediment(String),
    Inconclusive(String),
}

impl Label {
    pub fn text(&self) -> String {
        match self {
            Label::Relabel => "O-RELABEL".into(),
            Label::Structure(s) => format!("O-STRUCTURE[{s}]"),
            Label::Bounded(s) => format!("O-BOUNDED[{s}]"),
            Label::Artifact(s) => format!("O-ARTIFACT[{s}]"),
            Label::Impediment(s) => format!("O-IMPEDIMENT[{s}]"),
            Label::Inconclusive(s) => format!("O-INCONCLUSIVE[{s}]"),
        }
    }
}

pub const BAND: f64 = 0.95;
/// Minimum paired UNSAT targets: full (A and B) cells, and B-only m = 3 cells (PROTOCOL 7, 8).
pub const MIN_N_FULL: usize = 10;
pub const MIN_N_BONLY: usize = 6;
/// Censoring tolerance per arm-slot: dropping censored pairs removes the hardest targets and biases the ratio.
pub const CENSOR_TOL: f64 = 0.01;

fn has(rows: &[Row], arm: &str, slot: u32) -> bool {
    rows.iter()
        .any(|r| r.arm == arm && r.slot == slot && r.kind == "unsat")
}

/// Slots used for an inference: (1,2) when every named arm ran both, else (1)
/// for the primary; (3,4) / (3) for the replicate, empty if not run.
fn slots_for(rows: &[Row], arms: &[&str], first: u32) -> Vec<(u32, u32)> {
    if arms
        .iter()
        .all(|a| has(rows, a, first) && has(rows, a, first + 1))
    {
        vec![(first, first), (first + 1, first + 1)]
    } else if arms.iter().all(|a| has(rows, a, first)) {
        vec![(first, first)]
    } else {
        vec![]
    }
}

/// Frozen decision rule for one cell (PROTOCOL.md section 7).
pub fn decide(rows: &[Row], seed: u64) -> (Label, Vec<Cmp>) {
    let sound = ["C", "Cpoly", "Cnobreak", "A", "B", "Bnogauge", "G", "Chalf"]; // Chalf only runs on UNSAT targets
                                                                                // 1. artifact: any sound arm disagrees with the exact oracle, or a SAT certificate fails
    let bad_verdict = rows
        .iter()
        .filter(|r| sound.contains(&r.arm.as_str()) && r.result != "UNKNOWN" && !r.verdict_ok)
        .count();
    let bad_cert = rows
        .iter()
        .filter(|r| sound.contains(&r.arm.as_str()) && r.result == "SAT" && r.cert_ok == "0")
        .count();
    let mut cmps = vec![];
    for arm in ["A", "B", "Cpoly", "G", "U"] {
        let p = slots_for(rows, &[arm, "C"], 1);
        if !p.is_empty() {
            cmps.push(compare_pairs(
                rows,
                arm,
                "C",
                &p,
                "conflicts",
                if arm == "A" || arm == "B" {
                    "primary"
                } else {
                    "control"
                },
                seed,
            ));
        }
    }
    for arm in ["A", "B"] {
        let p = slots_for(rows, &[arm, "C"], 1);
        if !p.is_empty() {
            for metric in ["sops", "ops01", "ops"] {
                cmps.push(compare_pairs(rows, arm, "C", &p, metric, "primary", seed));
            }
        }
        let r = slots_for(rows, &[arm, "C"], 3);
        if !r.is_empty() {
            cmps.push(compare_pairs(
                rows,
                arm,
                "C",
                &r,
                "conflicts",
                "replicate",
                seed,
            ));
        }
    }
    cmps.push(compare_pairs(
        rows,
        "C",
        "C",
        &[(2, 1)],
        "conflicts",
        "aa",
        seed,
    )); // A/A noise floor
        // P1/P2: model-based expectations, informative only (PROTOCOL 4). P3 is the blindness control.
    cmps.push(compare_pairs(
        rows,
        "C",
        "Cnobreak",
        &[(1, 1)],
        "conflicts",
        "control",
        seed,
    )); // P1, expect in [0.5, 1.0] (m=2)
    cmps.push(compare_pairs(
        rows,
        "B",
        "Bnogauge",
        &[(1, 1)],
        "conflicts",
        "control",
        seed,
    )); // P2, expect <= 1 (about 1/n if the solver uses the gauge)
    cmps.push(compare_pairs(
        rows,
        "Chalf",
        "C",
        &[(1, 1)],
        "conflicts",
        "control",
        seed,
    )); // P3, all-slot pin: expected 0.125 to 0.5
    let find = |tag: &str, arm: &str, metric: &str| {
        cmps.iter()
            .find(|c| c.tag == tag && c.num_arm == arm && c.den_arm == "C" && c.metric == metric)
            .cloned()
    };
    if bad_verdict + bad_cert > 0 {
        return (
            Label::Artifact(format!(
                "{bad_verdict} verdict mismatches, {bad_cert} certificate failures"
            )),
            cmps,
        );
    }
    // P3 (top u-bit of every slot pinned to 0; at most half the search space retained) is the only voiding control: a detector that cannot
    // see a 2x search-space reduction cannot support any 0.95 statement. P1/P2 misses are reported
    // as "control uninformative" (control_notes), never as a void.
    if let Some(p) = cmps
        .iter()
        .find(|c| c.tag == "control" && c.num_arm == "Chalf" && c.den_arm == "C")
    {
        if p.n_targets >= MIN_N_BONLY && !(p.hi < BAND) {
            return (Label::Artifact(format!("positive control P3 (Chalf/C) not detected (hi {:.3}): the detector is blind in this cell", p.hi)), cmps);
        }
    }
    let mut by_arm: BTreeMap<(String, u32), (usize, usize)> = BTreeMap::new();
    for r in rows.iter().filter(|r| r.kind == "unsat") {
        let e = by_arm.entry((r.arm.clone(), r.slot)).or_insert((0, 0));
        e.1 += 1;
        if r.result == "UNKNOWN" {
            e.0 += 1;
        }
    }
    if let Some(((arm, slot), (c, t))) = by_arm
        .iter()
        .find(|(_, (c, t))| *t > 0 && *c as f64 / *t as f64 > CENSOR_TOL)
    {
        return (
            Label::Impediment(format!("{arm}@{slot}: {c}/{t} censored")),
            cmps,
        );
    }
    // m = 3 spec cells run B and C only (PROTOCOL 7 rule 2b): an interval statement for B, nothing more.
    if find("primary", "A", "conflicts").is_none() {
        let Some(b) = find("primary", "B", "conflicts") else {
            return (Label::Impediment("B or C arm missing".into()), cmps);
        };
        if b.n_targets < MIN_N_BONLY {
            return (
                Label::Impediment(format!(
                    "too few paired UNSAT targets for a B-only cell: {} < {MIN_N_BONLY}",
                    b.n_targets
                )),
                cmps,
            );
        }
        return (
            Label::Bounded(format!("B/C median {:.3}, 95% CI [{:.3}, {:.3}] over {} targets; excludes B/C below {:.3}; no structure claim without A, G and replicate arms", b.median_ratio, b.lo, b.hi, b.n_targets, b.lo)),
            cmps,
        );
    }
    let (Some(a), Some(b)) = (
        find("primary", "A", "conflicts"),
        find("primary", "B", "conflicts"),
    ) else {
        return (Label::Impediment("A or B or C arm missing".into()), cmps);
    };
    if a.n_targets < MIN_N_FULL || b.n_targets < MIN_N_FULL {
        return (
            Label::Impediment(format!(
                "too few paired UNSAT targets: A {} B {}",
                a.n_targets, b.n_targets
            )),
            cmps,
        );
    }
    let g = find("control", "G", "conflicts");
    let cp = find("control", "Cpoly", "conflicts");
    let aa = cmps.iter().find(|c| c.tag == "aa").cloned();
    let mut hits = vec![];
    for (nm, c) in [("A", &a), ("B", &b)] {
        if c.hi < BAND {
            let rep = find("replicate", nm, "conflicts");
            let replicated = rep
                .as_ref()
                .map(|r| r.n_targets >= 10 && r.hi < BAND)
                .unwrap_or(false);
            // PROTOCOL 7 rule 3: G, Cpoly and A/A are REQUIRED controls. A missing control, or one
            // with fewer than 10 paired targets, leaves the candidate "not confirmed" (fail closed).
            let g_present = g
                .as_ref()
                .map(|g| g.n_targets >= MIN_N_FULL)
                .unwrap_or(false);
            let cp_present = cp
                .as_ref()
                .map(|p| p.n_targets >= MIN_N_FULL)
                .unwrap_or(false);
            let aa_present = aa
                .as_ref()
                .map(|x| x.n_targets >= MIN_N_FULL)
                .unwrap_or(false);
            let g_reproduces = g
                .as_ref()
                .map(|g| g.n_targets >= MIN_N_FULL && g.hi < BAND)
                .unwrap_or(false);
            // basis: the plain arm in the polynomial basis must not be the cheaper reference
            let basis_explains = cp
                .as_ref()
                .map(|p| p.n_targets >= MIN_N_FULL && p.hi < BAND)
                .unwrap_or(false);
            // noise: the A/A interval must not already reach the effect
            let noise = aa
                .as_ref()
                .map(|x| x.n_targets >= MIN_N_FULL && x.lo <= c.hi)
                .unwrap_or(false);
            let controls_present = g_present && cp_present && aa_present;
            if replicated && controls_present && !g_reproduces && !basis_explains && !noise {
                hits.push(nm.to_string());
            } else {
                hits.push(format!("{nm}:sub-0.95 but not confirmed (replicated={replicated}, controls_present[G,Cpoly,AA]=[{g_present},{cp_present},{aa_present}], generic_control_reproduces={g_reproduces}, basis_explains={basis_explains}, aa_noise={noise})"));
            }
        }
    }
    if !hits.is_empty() {
        if hits.iter().any(|h| !h.contains("not confirmed")) {
            return (Label::Structure(hits.join("; ")), cmps);
        }
        return (Label::Inconclusive(hits.join("; ")), cmps);
    }
    if a.lo >= BAND && b.lo >= BAND {
        // PROTOCOL 7 rule 4: a null label needs its positive control. P3 must be present with >= MIN_N_BONLY
        // paired targets and detected (hi < BAND); otherwise the detector was never shown able to see a 2x change.
        let p3_ok = cmps.iter().any(|c| {
            c.tag == "control"
                && c.num_arm == "Chalf"
                && c.den_arm == "C"
                && c.n_targets >= MIN_N_BONLY
                && c.hi < BAND
        });
        if !p3_ok {
            return (
                Label::Inconclusive(
                    "positive control P3 (Chalf/C) absent or undersized: no O-RELABEL without it"
                        .into(),
                ),
                cmps,
            );
        }
        return (Label::Relabel, cmps);
    }
    (
        Label::Inconclusive(format!(
            "A lo {:.3} B lo {:.3}: interval straddles {BAND}",
            a.lo, b.lo
        )),
        cmps,
    )
}

/// Informative controls P1 and P2 (not voiding). One line per control, with whether the
/// pre-registered model expectation (PROTOCOL 4) was met.
pub fn control_notes(cmps: &[Cmp]) -> Vec<String> {
    let mut v = vec![];
    for (num, den, name, exp) in [
        ("C", "Cnobreak", "P1 S_m break", "expected in [0.5, 1.0]"),
        ("B", "Bnogauge", "P2 gauge", "expected <= 1"),
    ] {
        if let Some(p) = cmps
            .iter()
            .find(|c| c.tag == "control" && c.num_arm == num && c.den_arm == den && c.n_targets > 0)
        {
            let seen = p.hi < BAND;
            v.push(format!(
                "{name}: {num}/{den} median {:.3} CI [{:.3}, {:.3}] ({exp}); effect {}",
                p.median_ratio,
                p.lo,
                p.hi,
                if seen {
                    "detected"
                } else {
                    "not detected: control uninformative (not a void)"
                }
            ));
        }
    }
    if let Some(p) = cmps
        .iter()
        .find(|c| c.tag == "control" && c.num_arm == "Chalf" && c.n_targets > 0)
    {
        v.push(format!("P3 Chalf/C median {:.3} CI [{:.3}, {:.3}] (all-slot pin; expected 0.125-0.5, see PROTOCOL 4; a miss voids)", p.median_ratio, p.lo, p.hi));
    }
    v
}

/// Section 5a: an arm is ranked against C only if the median ratio is on the
/// same side of 1 at every pricing weight w in {0, 0.1, 1}; otherwise unresolved.
pub fn band_rank(cmps: &[Cmp], arm: &str) -> String {
    let rs: Vec<f64> = ["sops", "ops01", "ops"]
        .iter()
        .filter_map(|m| {
            cmps.iter().find(|c| {
                c.tag == "primary" && c.num_arm == arm && c.den_arm == "C" && c.metric == *m
            })
        })
        .map(|c| c.median_ratio)
        .collect();
    if rs.len() < 3 || rs.iter().any(|x| x.is_nan()) {
        return format!("{arm}: no data");
    }
    if rs.iter().all(|x| *x < 1.0) {
        format!(
            "{arm} cheaper than C at w=0,0.1,1 ({:.3},{:.3},{:.3})",
            rs[0], rs[1], rs[2]
        )
    } else if rs.iter().all(|x| *x > 1.0) {
        format!(
            "{arm} dearer than C at w=0,0.1,1 ({:.3},{:.3},{:.3})",
            rs[0], rs[1], rs[2]
        )
    } else {
        format!(
            "{arm} vs C UNRESOLVED inside the pricing band ({:.3},{:.3},{:.3})",
            rs[0], rs[1], rs[2]
        )
    }
}

/// Tripwire: literal per-instance clauses (arm U) must flip at least one
/// planted SAT target to UNSAT in the cell.
pub fn tripwire_flips(rows: &[Row]) -> (usize, usize) {
    let planted: Vec<&Row> = rows
        .iter()
        .filter(|r| r.kind == "planted" && r.arm == "U" && r.result != "UNKNOWN")
        .collect();
    (
        planted.iter().filter(|r| r.result == "UNSAT").count(),
        planted.len(),
    )
}

fn fmt(x: f64) -> String {
    if x.is_nan() {
        "null".into()
    } else {
        format!("{x:.4}")
    }
}

/// Pilot read-out for the gates in RUN_CARD.md: per (cell, arm, slot) mean
/// conflicts, mean ns per conflict and mean build ns over UNSAT-result rows.
pub fn cmd_gate(args: &[String]) -> i32 {
    let mut rows = vec![];
    for d in args {
        let Ok(rd) = std::fs::read_dir(d) else {
            eprintln!("cannot read {d}");
            return 2;
        };
        let mut files: Vec<_> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("rows_")
            })
            .collect();
        files.sort();
        for f in files {
            rows.extend(parse_rows(&std::fs::read_to_string(&f).unwrap()));
        }
    }
    // per (cell, arm): solved rows, censored rows, mean conflicts and ns per conflict over solved rows,
    // the largest VmHWM seen, the largest conflict count, and the smallest conflict count among censored
    // rows (a value below the cap means a DEADLINE censor, which is load dependent).
    #[derive(Default)]
    struct G {
        n: usize,
        unk: usize,
        conf: f64,
        ns: f64,
        rss: f64,
        maxc: f64,
        unk_min: f64,
    }
    let mut g: BTreeMap<(String, String), G> = BTreeMap::new();
    for r in rows.iter() {
        let e = g.entry((r.cell.clone(), r.arm.clone())).or_default();
        e.rss = e.rss.max(r.rss_kb);
        e.maxc = e.maxc.max(r.conflicts);
        if r.result == "UNKNOWN" {
            e.unk += 1;
            e.unk_min = if e.unk == 1 {
                r.conflicts
            } else {
                e.unk_min.min(r.conflicts)
            };
        } else if r.result == "UNSAT" && r.conflicts > 0.0 {
            e.n += 1;
            e.conf += r.conflicts;
            e.ns += r.wall_ns;
        }
    }
    println!("cell\tarm\tn_unsat_rows\tn_censored\tmean_conflicts\tns_per_conflict\tmax_rss_mb\tmax_conflicts\tmin_conflicts_of_censored");
    for ((c, a), e) in g {
        let mean = if e.n > 0 {
            e.conf / e.n as f64
        } else {
            f64::NAN
        };
        let nspc = if e.conf > 0.0 {
            e.ns / e.conf
        } else {
            f64::NAN
        };
        println!(
            "{c}\t{a}\t{}\t{}\t{:.1}\t{:.0}\t{:.0}\t{:.0}\t{}",
            e.n,
            e.unk,
            mean,
            nspc,
            e.rss / 1024.0,
            e.maxc,
            if e.unk > 0 {
                format!("{:.0}", e.unk_min)
            } else {
                "-".into()
            }
        );
    }
    0
}

pub fn cmd_summarize(args: &[String]) -> i32 {
    let out_i = args.iter().position(|a| a == "--out");
    let out = out_i
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| ".".into());
    let dirs: Vec<&String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| !a.starts_with("--") && Some(*i) != out_i.map(|j| j + 1))
        .map(|(_, a)| a)
        .collect();
    let mut rows = vec![];
    for d in dirs {
        let rd = match std::fs::read_dir(d) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{d}: {e}");
                return 2;
            }
        };
        let mut files: Vec<_> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().map(|x| x == "tsv").unwrap_or(false)
                    && p.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .starts_with("rows_")
            })
            .collect();
        files.sort();
        for f in files {
            rows.extend(parse_rows(&std::fs::read_to_string(&f).unwrap()));
        }
    }
    std::fs::create_dir_all(&out).unwrap();
    let mut cells: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    for r in rows {
        cells.entry(r.cell.clone()).or_default().push(r);
    }
    let mut md = String::from(
        "# Cell G3 census summary (generated; see PROTOCOL.md for the frozen rule)\n\n",
    );
    let mut json = String::from("{\n \"cells\": [\n");
    let mut svg_rows: Vec<(String, f64, f64, f64)> = vec![];
    let mut first = true;
    let mut tw_cells = (0usize, 0usize); // (cells with planted U rows, of which with >= 1 flip)
    let mut excess_tbl: Vec<(String, u32, usize, u32, f64, f64, f64)> = vec![];
    for (cell, rs) in &cells {
        if cell.contains("_b") {
            // ordinary-curve null: the detector must see B (and U) disagree with the plain truth; C and A must not
            let bad = |arm: &str| {
                rs.iter()
                    .filter(|r| {
                        r.arm == arm
                            && r.result != "UNKNOWN"
                            && (!r.verdict_ok || (r.result == "SAT" && r.cert_ok == "0"))
                    })
                    .count()
            };
            let (bb, cc, aa_) = (bad("B"), bad("C"), bad("A"));
            let ok = bb > 0 && cc == 0 && aa_ == 0;
            md.push_str(&format!("## ordinary-curve null {cell}\n\nB disagreements {bb}, C disagreements {cc}, A disagreements {aa_}: detector {}\n\n", if ok { "WORKS (unsoundness of the orbit system off F_2-curves is visible)" } else { "FAILED" }));
            if !first {
                json.push_str(",\n");
            }
            first = false;
            json.push_str(&format!("  {{\"cell\": \"{cell}\", \"ordinary_null_detector_ok\": {ok}, \"b_disagreements\": {bb}}}"));
            continue;
        }
        let (label, cmps) = decide(rs, 20261027);
        let (flips, planted_u) = tripwire_flips(rs);
        if planted_u > 0 {
            tw_cells.0 += 1;
            if flips >= 1 {
                tw_cells.1 += 1;
            }
        }
        md.push_str(&format!("## cell {cell}\n\nlabel: **{}**; tripwire: arm U flipped {flips}/{planted_u} planted SAT targets to UNSAT ({})\n\npricing band (AGENTS.md 5a): {}; {}\n\n", label.text(), if planted_u == 0 { "not run in this cell" } else if flips >= 1 { "at least one planted flip" } else { "NO planted flip: tripwire blind in this cell" }, band_rank(&cmps, "A"), band_rank(&cmps, "B")));
        for note in control_notes(&cmps) {
            md.push_str(&format!("- control {note}\n"));
        }
        md.push('\n');
        md.push_str("| role | numerator | denominator | metric | targets | pairs | median ratio | 95% CI (cluster bootstrap) | geo-mean | frac<0.95 | sd(log) | MDE |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n");
        if !first {
            json.push_str(",\n");
        }
        first = false;
        json.push_str(&format!("  {{\"cell\": \"{cell}\", \"label\": \"{}\", \"tripwire_flips\": {flips}, \"planted_u\": {planted_u}, \"comparisons\": [\n", label.text()));
        for (i, c) in cmps.iter().enumerate() {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | [{}, {}] | {} | {} | {} | {} |\n",
                c.tag,
                c.num,
                c.den,
                c.metric,
                c.n_targets,
                c.n_pairs,
                fmt(c.median_ratio),
                fmt(c.lo),
                fmt(c.hi),
                fmt(c.gm_ratio),
                fmt(c.frac_below_095),
                fmt(c.sd_log),
                fmt(mde(c.sd_log, c.n_targets))
            ));
            json.push_str(&format!("   {{\"tag\":\"{}\",\"num\":\"{}\",\"den\":\"{}\",\"metric\":\"{}\",\"targets\":{},\"pairs\":{},\"median\":{},\"lo\":{},\"hi\":{},\"gm\":{},\"sd_log\":{}}}{}\n", c.tag, c.num, c.den, c.metric, c.n_targets, c.n_pairs, fmt(c.median_ratio), fmt(c.lo), fmt(c.hi), fmt(c.gm_ratio), fmt(c.sd_log), if i + 1 < cmps.len() { "," } else { "" }));
            if c.metric == "conflicts" && c.n_pairs > 0 {
                svg_rows.push((
                    format!("{cell} {}/{}", c.num, c.den),
                    c.median_ratio,
                    c.lo,
                    c.hi,
                ));
            }
        }
        json.push_str("  ]}");
        md.push('\n');
        // mean conflicts per arm on UNSAT targets (for the additive-vs-multiplicative overhead diagnostic)
        let mean_of = |arm: &str| {
            let v: Vec<f64> = rs
                .iter()
                .filter(|r| r.kind == "unsat" && r.arm == arm && r.slot == 1 && r.result == "UNSAT")
                .map(|r| r.conflicts)
                .collect();
            if v.is_empty() {
                f64::NAN
            } else {
                v.iter().sum::<f64>() / v.len() as f64
            }
        };
        let r0 = &rs[0];
        excess_tbl.push((
            cell.clone(),
            r0.n,
            r0.m,
            r0.l,
            mean_of("C"),
            mean_of("A") - mean_of("C"),
            mean_of("B") - mean_of("C"),
        ));
    }
    md.push_str(&format!("## O-TRIPWIRE headline (H1: literal per-instance orbit clauses are unsound)\n\n{} of {} cells with planted U rows show at least one planted SAT target flipped to UNSAT (the instrument sees the unsoundness); this is a statement about arm U only, not about A or B.\n\n", tw_cells.1, tw_cells.0));
    md.push_str("## overhead diagnostic (mean conflicts; excess = arm - C)\n\n| cell | n | m | l | mean C | excess A | excess B |\n|---|---|---|---|---|---|---|\n");
    for (c, n, m, l, mc, ea, eb) in &excess_tbl {
        md.push_str(&format!(
            "| {c} | {n} | {m} | {l} | {} | {} | {} |\n",
            fmt(*mc),
            fmt(*ea),
            fmt(*eb)
        ));
    }
    for i in 0..excess_tbl.len() {
        for j in 0..excess_tbl.len() {
            let (a, b) = (&excess_tbl[i], &excess_tbl[j]);
            if a.1 == b.1 && a.2 == b.2 && b.3 == a.3 + 1 {
                let (rc, re, rea) = (b.4 / a.4, b.6 / a.6, b.5 / a.5);
                let vd = |r: f64| {
                    if r.is_nan() || rc.is_nan() {
                        "n/a"
                    } else if r < rc.sqrt() {
                        "additive-consistent (excess grows slower than sqrt of C)"
                    } else {
                        "multiplicative-consistent"
                    }
                };
                md.push_str(&format!("\nn={} m={} l {}->{}: C grows x{:.2}; excess_A grows x{:.2}: {}; excess_B grows x{:.2}: {}\n", a.1, a.2, a.3, b.3, rc, rea, vd(rea), re, vd(re)));
            }
        }
    }
    json.push_str("\n ]\n}\n");
    std::fs::write(format!("{out}/summary.json"), json).unwrap();
    std::fs::write(format!("{out}/SUMMARY.md"), md).unwrap();
    std::fs::write(format!("{out}/forest.svg"), forest_svg(&svg_rows)).unwrap();
    0
}

/// Log-scale forest plot of median ratios with 95 % intervals; the 0.95 band and 1.0 are marked.
pub fn forest_svg(rows: &[(String, f64, f64, f64)]) -> String {
    let h = 24 + rows.len() * 18;
    let (x0, x1) = (260.0, 780.0);
    let (lmin, lmax) = (-4.0f64, 4.0f64); // log2 range 1/16 .. 16
    let px =
        |r: f64| x0 + (r.max(1e-9).log2().clamp(lmin, lmax) - lmin) / (lmax - lmin) * (x1 - x0);
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"820\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"11\">\n<rect width=\"820\" height=\"{h}\" fill=\"white\"/>\n");
    for (r, name) in [(0.95, "0.95"), (1.0, "1")] {
        s.push_str(&format!("<line x1=\"{0:.1}\" y1=\"10\" x2=\"{0:.1}\" y2=\"{1}\" stroke=\"{2}\" stroke-dasharray=\"4 3\"/><text x=\"{0:.1}\" y=\"9\" text-anchor=\"middle\">{3}</text>\n", px(r), h - 4, if r < 1.0 { "#c0392b" } else { "#555" }, name));
    }
    for (i, (name, med, lo, hi)) in rows.iter().enumerate() {
        let y = 24 + i * 18;
        s.push_str(&format!("<text x=\"4\" y=\"{}\">{}</text>\n", y + 4, name));
        if lo.is_finite() && hi.is_finite() {
            s.push_str(&format!("<line x1=\"{:.1}\" y1=\"{y}\" x2=\"{:.1}\" y2=\"{y}\" stroke=\"#1f4e79\" stroke-width=\"2\"/>\n", px(*lo), px(*hi)));
        }
        if med.is_finite() {
            s.push_str(&format!(
                "<circle cx=\"{:.1}\" cy=\"{y}\" r=\"3.5\" fill=\"#1f4e79\"/>\n",
                px(*med)
            ));
        }
    }
    s.push_str("</svg>\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(
        cell: &str,
        tidx: usize,
        arm: &str,
        slot: u32,
        conflicts: f64,
        result: &str,
        truth: &str,
        kind: &str,
    ) -> Row {
        Row {
            cell: cell.into(),
            n: 17,
            m: 2,
            l: 8,
            curve_a: 0,
            round: 1,
            tidx,
            xr: 1000 + tidx as u64,
            kind: kind.into(),
            truth: truth.into(),
            arm: arm.into(),
            slot,
            result: result.into(),
            verdict_ok: result == truth || result == "UNKNOWN",
            cert_ok: "NA".into(),
            conflicts,
            total_ops: conflicts * 100.0,
            search_ops: conflicts * 100.0,
            encode_ops: 0.0,
            wall_ns: conflicts * 1000.0,
            rss_kb: 1000.0,
        }
    }

    #[test]
    fn median_and_bootstrap_known_values() {
        let mut v = vec![3.0, 1.0, 2.0];
        assert_eq!(median(&mut v), 2.0);
        let mut v = vec![4.0, 1.0, 2.0, 3.0];
        assert_eq!(median(&mut v), 2.5);
        let vals: Vec<f64> = (0..101).map(|i| 1.0 + i as f64 / 1000.0).collect();
        let (lo, hi) = boot_median_ci(&vals, 2000, 7);
        assert!(lo < 1.05 && hi > 1.05 && lo > 1.0 && hi < 1.1);
        // constant data -> degenerate interval
        let (lo, hi) = boot_median_ci(&[1.5; 20], 500, 1);
        assert_eq!((lo, hi), (1.5, 1.5));
        assert!(mde(0.1, 50) < 1.0 && mde(0.1, 50) > 0.9);
    }

    fn synth(a_factor: f64, g_factor: f64, bad_verdict: bool) -> Vec<Row> {
        let mut rows = vec![];
        for t in 0..30 {
            let base = 1000.0 + (t % 7) as f64 * 13.0;
            let jitter = 1.0 + (((t * 7919) % 11) as f64 - 5.0) * 0.002;
            for (arm, slot, f) in [
                ("C", 1, 1.0),
                ("C", 2, jitter),
                ("C", 3, 1.0),
                ("A", 1, a_factor * jitter),
                ("A", 3, a_factor),
                ("B", 1, 1.2),
                ("B", 3, 1.2),
                ("G", 1, g_factor),
                ("Cpoly", 1, 1.01),
                ("Cnobreak", 1, 2.0),
                ("Bnogauge", 1, 1.2 * 17.0),
                ("Chalf", 1, 0.5),
            ] {
                rows.push(row(
                    "17_8_2_0",
                    t,
                    arm,
                    slot,
                    base * f,
                    "UNSAT",
                    "UNSAT",
                    "unsat",
                ));
            }
        }
        if bad_verdict {
            let mut r = row("17_8_2_0", 0, "A", 1, 5.0, "SAT", "UNSAT", "unsat");
            r.verdict_ok = false;
            rows.push(r);
        }
        rows
    }

    #[test]
    fn decision_rule_labels_on_synthetic_data() {
        // relabelling: A costs 3x, B 1.2x, G neutral
        let (l, cmps) = decide(&synth(3.0, 1.0, false), 1);
        assert_eq!(l, Label::Relabel, "{l:?}");
        // P1 informative: C vs Cnobreak ratio is 0.5; P3 derivable control: Chalf/C is 0.5, detected
        let p1 = cmps
            .iter()
            .find(|c| c.num_arm == "C" && c.den_arm == "Cnobreak")
            .unwrap();
        assert!(p1.hi < BAND && (p1.median_ratio - 0.5).abs() < 1e-9);
        let p3 = cmps
            .iter()
            .find(|c| c.num_arm == "Chalf" && c.den_arm == "C")
            .unwrap();
        assert!(p3.hi < BAND && (p3.median_ratio - 0.5).abs() < 1e-9);
        // A/A noise is small and straddles 1
        let aa = cmps.iter().find(|c| c.tag == "aa").unwrap();
        assert!(aa.lo <= 1.02 && aa.hi >= 0.98 && aa.sd_log < 0.03, "{aa:?}");
        // a genuine 0.5x arm that replicates, G neutral: structure
        let (l, _) = decide(&synth(0.5, 1.0, false), 1);
        assert!(matches!(l, Label::Structure(_)), "{l:?}");
        // same 0.5x but the generic control reproduces it: not structure
        let (l, _) = decide(&synth(0.5, 0.5, false), 1);
        assert!(matches!(l, Label::Inconclusive(_)), "{l:?}");
        // a verdict mismatch voids the cell
        let (l, _) = decide(&synth(3.0, 1.0, true), 1);
        assert!(matches!(l, Label::Artifact(_)), "{l:?}");
    }

    /// Round-2 defect 2: O-RELABEL needs P3 present with >= 6 paired targets.
    #[test]
    fn relabel_requires_the_positive_control() {
        let (l, _) = decide(&synth(3.0, 1.0, false), 1);
        assert_eq!(l, Label::Relabel, "{l:?}");
        let no_p3: Vec<Row> = synth(3.0, 1.0, false)
            .into_iter()
            .filter(|r| r.arm != "Chalf")
            .collect();
        let (l, _) = decide(&no_p3, 1);
        assert!(
            matches!(l, Label::Inconclusive(ref s) if s.contains("P3")),
            "{l:?}"
        );
        let five: Vec<Row> = synth(3.0, 1.0, false)
            .into_iter()
            .filter(|r| r.arm != "Chalf" || r.tidx < 5)
            .collect();
        let (l, _) = decide(&five, 1);
        assert!(
            matches!(l, Label::Inconclusive(ref s) if s.contains("P3")),
            "{l:?}"
        );
    }

    #[test]
    fn censored_cells_are_impediments_never_evidence() {
        let mut rows = synth(3.0, 1.0, false);
        for r in rows.iter_mut().filter(|r| r.tidx < 5 && r.arm == "A") {
            r.result = "UNKNOWN".into();
        }
        let (l, _) = decide(&rows, 1);
        assert!(matches!(l, Label::Impediment(_)), "{l:?}");
    }

    #[test]
    fn tsv_roundtrip_and_svg() {
        let hdr = crate::run::HEADER;
        let cols = hdr.split('\t').count();
        let mut f: Vec<String> = vec!["0".into(); cols];
        let set = |f: &mut Vec<String>, k: &str, v: &str| {
            let i = hdr.split('\t').position(|h| h == k).unwrap();
            f[i] = v.to_string();
        };
        set(&mut f, "cell", "17_8_2_0");
        set(&mut f, "kind", "unsat");
        set(&mut f, "truth", "UNSAT");
        set(&mut f, "arm", "C");
        set(&mut f, "result", "UNSAT");
        set(&mut f, "verdict_ok", "1");
        set(&mut f, "cert_ok", "NA");
        set(&mut f, "conflicts", "123");
        let text = format!("{hdr}\n{}\n", f.join("\t"));
        let rows = parse_rows(&text);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].conflicts, 123.0);
        let svg = forest_svg(&[("x".into(), 1.0, 0.9, 1.1)]);
        assert!(svg.starts_with("<svg") && svg.contains("circle"));
    }

    #[test]
    fn summarize_end_to_end_on_synthetic_rows() {
        let hdr = crate::run::HEADER;
        let cols: Vec<&str> = hdr.split('\t').collect();
        let mut text = format!("{hdr}\n");
        for r in synth(3.0, 1.0, false) {
            let mut f = vec!["0".to_string(); cols.len()];
            let mut set = |k: &str, v: String| {
                let i = cols.iter().position(|h| *h == k).unwrap();
                f[i] = v;
            };
            set("cell", r.cell.clone());
            set("n", "17".into());
            set("m", "2".into());
            set("l", "8".into());
            set("curve_a", "0".into());
            set("round", "1".into());
            set("tidx", r.tidx.to_string());
            set("kind", r.kind.clone());
            set("truth", r.truth.clone());
            set("arm", r.arm.clone());
            set("slot", r.slot.to_string());
            set("result", r.result.clone());
            set("verdict_ok", "1".into());
            set("cert_ok", "NA".into());
            set("conflicts", r.conflicts.to_string());
            set("total_ops", r.total_ops.to_string());
            set("search_ops", r.search_ops.to_string());
            set("encode_ops", "0".into());
            set("wall_ns", r.wall_ns.to_string());
            text.push_str(&f.join("\t"));
            text.push('\n');
        }
        let dir = format!(
            "{}/g3census-test-summarize-{}",
            std::env::temp_dir().display(),
            std::process::id()
        );
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(format!("{dir}/rows_synthetic.tsv"), text).unwrap();
        let out = format!("{dir}/out");
        assert_eq!(
            cmd_summarize(&[dir.clone(), "--out".into(), out.clone()]),
            0
        );
        let md = std::fs::read_to_string(format!("{out}/SUMMARY.md")).unwrap();
        assert!(
            md.contains("O-RELABEL") && md.contains("overhead diagnostic"),
            "{md}"
        );
        assert!(std::fs::read_to_string(format!("{out}/summary.json"))
            .unwrap()
            .contains("\"comparisons\""));
        assert!(std::fs::read_to_string(format!("{out}/forest.svg"))
            .unwrap()
            .starts_with("<svg"));
    }

    #[test]
    fn cluster_bootstrap_keeps_slots_with_their_target() {
        let mut rows = vec![];
        for t in 0..12 {
            for slot in [1u32, 2] {
                rows.push(row(
                    "c",
                    t,
                    "C",
                    slot,
                    100.0 * (1.0 + t as f64),
                    "UNSAT",
                    "UNSAT",
                    "unsat",
                ));
                rows.push(row(
                    "c",
                    t,
                    "A",
                    slot,
                    200.0 * (1.0 + t as f64),
                    "UNSAT",
                    "UNSAT",
                    "unsat",
                ));
            }
        }
        let c = compare_pairs(
            &rows,
            "A",
            "C",
            &[(1, 1), (2, 2)],
            "conflicts",
            "primary",
            5,
        );
        assert_eq!((c.n_targets, c.n_pairs), (12, 24));
        assert!((c.median_ratio - 2.0).abs() < 1e-12 && c.lo == 2.0 && c.hi == 2.0);
        assert_eq!(c.num, "A@1+2");
        let aa = compare_pairs(&rows, "C", "C", &[(2, 1)], "conflicts", "aa", 5);
        assert_eq!((aa.n_targets, aa.n_pairs), (12, 12));
        assert!((aa.median_ratio - 1.0).abs() < 1e-12);
    }

    #[test]
    fn blind_detector_voids_the_cell() {
        let mut rows = synth(3.0, 1.0, false);
        let base: std::collections::HashMap<usize, f64> = rows
            .iter()
            .filter(|r| r.arm == "C" && r.slot == 1)
            .map(|r| (r.tidx, r.conflicts))
            .collect();
        for r in rows.iter_mut().filter(|r| r.arm == "Chalf") {
            r.conflicts = base[&r.tidx]; // pinning every slot's top bit "has no effect": the detector is blind
        }
        let (l, _) = decide(&rows, 1);
        assert!(
            matches!(l, Label::Artifact(ref s) if s.contains("P3")),
            "{l:?}"
        );
    }

    #[test]
    fn structure_is_not_confirmed_when_required_controls_are_missing() {
        // A at 0.5x C, replicated (slot 3), so with G, Cpoly and A/A present this is O-STRUCTURE
        let full = synth(0.5, 1.0, false);
        let (l, _) = decide(&full, 1);
        assert!(matches!(l, Label::Structure(_)), "{l:?}");
        // dropping any one required control (G, Cpoly, or the C@2 A/A slot) must NOT confirm
        for drop in ["G", "Cpoly", "C2"] {
            let rows: Vec<Row> = full
                .iter()
                .filter(|r| !(r.arm == drop || (drop == "C2" && r.arm == "C" && r.slot == 2)))
                .cloned()
                .collect();
            let (l, _) = decide(&rows, 1);
            assert!(!matches!(l, Label::Structure(_)), "dropped {drop}: {l:?}");
        }
        // a control with fewer than 10 paired targets does not confirm either
        let rows: Vec<Row> = full
            .iter()
            .filter(|r| !(r.arm == "G" && r.tidx >= 6))
            .cloned()
            .collect();
        let (l, _) = decide(&rows, 1);
        assert!(!matches!(l, Label::Structure(_)), "G with 6 targets: {l:?}");
    }

    #[test]
    fn p1_miss_is_uninformative_not_a_void() {
        let mut rows = synth(3.0, 1.0, false);
        let base: std::collections::HashMap<usize, f64> = rows
            .iter()
            .filter(|r| r.arm == "C" && r.slot == 1)
            .map(|r| (r.tidx, r.conflicts))
            .collect();
        for r in rows
            .iter_mut()
            .filter(|r| r.arm == "Cnobreak" || r.arm == "Bnogauge")
        {
            r.conflicts = base[&r.tidx]; // the S_m break and the gauge do nothing
        }
        let (l, cmps) = decide(&rows, 1);
        assert_eq!(l, Label::Relabel, "{l:?}");
        let notes = control_notes(&cmps).join("\n");
        assert!(
            notes.contains("P1") && notes.contains("uninformative"),
            "{notes}"
        );
    }

    /// Review defect 2: chunked runs restart tidx at 0 with different targets (different xr).
    #[test]
    fn chunked_runs_with_restarting_tidx_are_not_collapsed() {
        let mut rows = vec![];
        for chunk in 0..4u64 {
            for t in 0..3usize {
                for (arm, f) in [("C", 1.0), ("B", 1.3)] {
                    let mut r = row(
                        "31_3_10_0",
                        t,
                        arm,
                        1,
                        1000.0 * f * (1.0 + (chunk * 3 + t as u64) as f64 * 0.01),
                        "UNSAT",
                        "UNSAT",
                        "unsat",
                    );
                    r.xr = 7000 + chunk * 100 + t as u64;
                    rows.push(r);
                }
            }
        }
        let c = compare_pairs(&rows, "B", "C", &[(1, 1)], "conflicts", "primary", 3);
        assert_eq!(c.n_targets, 12);
        // a replicate-slot file repeats the same targets (same xr) and must merge, not add targets
        let mut rep = vec![];
        for r in rows.iter().filter(|r| r.arm == "C") {
            let mut q = r.clone();
            q.slot = 3;
            rep.push(q);
        }
        rows.extend(rep);
        let c = compare_pairs(&rows, "C", "C", &[(3, 1)], "conflicts", "aa", 3);
        assert_eq!(c.n_targets, 12);
    }

    /// Review defect 3: a B and C only m = 3 cell gets a label from the rule, with N from 6.
    #[test]
    fn b_only_cells_get_a_bounded_label_from_six_targets() {
        let mk = |nt: usize| {
            let mut rows = vec![];
            for t in 0..nt {
                for (arm, slot, f) in [
                    ("C", 1, 1.0),
                    ("C", 2, 1.0 + 0.01 * t as f64),
                    ("B", 1, 1.25),
                ] {
                    rows.push(row(
                        "31_3_10_0",
                        t,
                        arm,
                        slot,
                        5000.0 * f * (1.0 + 0.02 * t as f64),
                        "UNSAT",
                        "UNSAT",
                        "unsat",
                    ));
                }
            }
            rows
        };
        let (l, _) = decide(&mk(6), 1);
        assert!(
            matches!(l, Label::Bounded(ref s) if s.contains("excludes B/C below")),
            "{l:?}"
        );
        let (l, _) = decide(&mk(5), 1);
        assert!(matches!(l, Label::Impediment(_)), "{l:?}");
        // one censored target in 6 exceeds the 1 % tolerance
        let mut rows = mk(8);
        rows.iter_mut()
            .find(|r| r.arm == "B" && r.tidx == 0)
            .unwrap()
            .result = "UNKNOWN".into();
        let (l, _) = decide(&rows, 1);
        assert!(
            matches!(l, Label::Impediment(ref s) if s.contains("censored")),
            "{l:?}"
        );
    }
}
