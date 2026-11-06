//! Frozen analysis of `research/tm1514_recheck_20261106`
//! (`PROTOCOL.json`, protocol `XP-20261106-tm1514-recheck`).
//!
//! Usage:
//!
//! ```text
//! tm1514_analyze --a SESSION_A --b SESSION_B --out DIR
//!                [--wall-a CONTROL_COMPARISON_A.json] [--wall-b CONTROL_COMPARISON_B.json]
//!                [--devcheck]
//! ```
//!
//! `SESSION_*` are ecbench session directories (`session.json`,
//! `records.jsonl`) of replicates A and B.  `--wall-*` are the sessions'
//! `control-comparison.json` (an `ecbench.comparison` document); wall time
//! of a replicate is used for claim C3 only when its wall A/A is admitted
//! there.  `--devcheck` marks a development check on reduced specs: the
//! frozen spec ids are not enforced, wall time below L2 is used, and every
//! output is labelled as not data.
//!
//! What it computes is the protocol's `decision_rule`, nothing else:
//!
//! * C0: every measured row verified (`[d]G = Q`), twins identical counts;
//! * C1: `D(w) = ln R_tm1514(w) − ln R_sib(w)` at `w ∈ {0, 0.1, 1}` with a
//!   paired, curve-stratified target bootstrap, 98.33 % percentile interval
//!   (which alone decides); the seed-independent A/A half-width of `ln R` is
//!   reported beside it;
//! * C1b: the 95 % interval of `R_tm1514(0)` against the earlier 1.52;
//! * C2: pooled rho-plain / rho-neg at `w = 0`, 95 % interval, and every
//!   per-curve point estimate;
//! * C3: `w_meas` per native unit from L2 wall time (and hardware cycles
//!   where every row has them), regression with intercept on rho-plain;
//! * C4: `R_tm1514` at the measured prices, each arm at its own price and
//!   both at the larger one, the prices re-estimated in every resample.
//!
//! `R(w)` is the ratio of summed priced `S` (`band::priced_s`, AGENTS.md
//! §5a) over (workload, measured round) pairs in which both arms verified.
//! Arm names are replaced by key-derived codes as soon as the records are
//! validated; the statistics see only codes, and the names come back in the
//! output after every verdict has been computed.  Output is deterministic
//! for fixed inputs: one resampler, fixed curve and target order, no clocks.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crypto_lib::cryptanalysis::ecbench::band;
use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use crypto_lib::cryptanalysis::ecbench::compare::Comparison;
use crypto_lib::cryptanalysis::ecbench::record::{check_line_seal, Record};
use crypto_lib::cryptanalysis::ecbench::runner::{read_record_lines, Session};
use crypto_lib::cryptanalysis::ecbench::spec::Level;
use crypto_lib::cryptanalysis::ecbench::stats::{quantile, Resampler};
use serde_json::{json, Value};

const PROTOCOL_ID: &str = "XP-20261106-tm1514-recheck";
const SCHEMA: &str = "tm1514.analysis/v1";
/// PLAN.md §2: `int(sha256(D)[0:16 hex]) mod 2^53`, suffix `analysis.bootstrap_seed`.
const BOOTSTRAP_SEED: u64 = 244_596_609_222_896;
const RESAMPLES: usize = 10_000;
/// `ecbench plan` of the frozen specs (PLAN.md §1).
const SPEC_ID_A: &str = "ECS1h298a06a2f06d";
const SPEC_ID_B: &str = "ECS1h864cbf25c1f2";
const SUBJECT: &str = "icv1-fp24-tm1514-dce9313c";
const SIBLINGS: [&str; 4] = [
    "icv1-fp24-tm1578-e83ed1b2",
    "icv1-fp24-tm2338-191c1070",
    "icv1-fp24-tm3530-7cdfa218",
    "icv1-fp24-tm7826-eab5872b",
];
const EARLIER_R0: f64 = 1.52;
const C2_POOLED_LOWER: f64 = 1.20;
const C2_PER_CURVE: f64 = 1.10;
const C3_UPPER: f64 = 0.5;
/// Bonferroni over the three weights of C1.
const ALPHA_C1: f64 = 0.05 / 3.0;
const ALPHA: f64 = 0.05;
const MIN_AA_PAIRS: u64 = 5;

const NEG: &str = "rho-neg";
const NEG_AA: &str = "rho-neg-aa";
const INC: &str = "inc";
const INC_AA: &str = "inc-aa";
const PLAIN: &str = "rho-plain";
/// Arm name, method id, and the method parameters the protocol fixes.
const ARMS: [(&str, &str, &[(&str, &str)]); 5] = [
    (NEG, "rho.negation", &[]),
    (NEG_AA, "rho.negation", &[]),
    (INC, "ic.pipeline", INC_PARAMS),
    (INC_AA, "ic.pipeline", INC_PARAMS),
    (PLAIN, "rho.plain", &[]),
];
/// Method configuration ids (`ECM1h` + 12 hex of id and resolved params) of
/// the frozen specs, from `ecbench plan` (PLAN.md section 1).
const METHOD_IDS: [(&str, &str); 3] = [
    ("rho.negation", "ECM1hefea4e0ebe79"),
    ("ic.pipeline", "ECM1h3b0ad3332667"),
    ("rho.plain", "ECM1hb367476fce6c"),
];
const INC_PARAMS: &[(&str, &str)] = &[
    ("factor_base", "prime-projected-abscissa:size=2,scan_cap=32"),
    ("oracle", "wwalk:x=0.9,g=1,grow=2,seed=run"),
    ("targets", "stride-generator"),
];

// ── Additive per-cluster sums ──────────────────────────────────────────

// Counts (S priced by the band), pairs with both arms verified.
const IR_N: usize = 0;
const INC_S0: usize = 1;
const INC_SL: usize = 2;
const RHO_S0: usize = 3;
const RHO_SL: usize = 4;
const PN_N: usize = 5;
const PLAIN_S0: usize = 6;
const NEG_S0: usize = 7;
// Time blocks: block 0 is solve wall ns, block 1 hardware cycles.
const TB: usize = 8;
const TB_LEN: usize = 17;
const P_N: usize = 0;
const P_G: usize = 1;
const P_T: usize = 2;
const P_GG: usize = 3;
const P_GT: usize = 4;
const N_N: usize = 5;
const N_G: usize = 6;
const N_T: usize = 7;
const N_CANON: usize = 8;
const L_N: usize = 9;
const L_T: usize = 10;
const L_G: usize = 11;
const L_NAT: usize = 12;
const I_N: usize = 13;
const I_T: usize = 14;
const I_G: usize = 15;
const I_INS: usize = 16;
const NF: usize = TB + 2 * TB_LEN;

#[derive(Clone, Copy)]
struct Acc([f64; NF]);

impl Default for Acc {
    fn default() -> Self {
        Acc([0.0; NF])
    }
}

impl Acc {
    fn add(&mut self, o: &Acc) {
        for (a, b) in self.0.iter_mut().zip(o.0.iter()) {
            *a += b;
        }
    }
    fn t(&self, block: usize, f: usize) -> f64 {
        self.0[TB + block * TB_LEN + f]
    }
}

/// What one record contributes.
#[derive(Clone)]
struct Obs {
    status: String,
    verified: bool,
    /// `priced_s(0)` and `priced_s(1) − priced_s(0)`.
    s0: Option<f64>,
    s_slope: Option<f64>,
    gae: Option<f64>,
    l2: bool,
    wall: Option<f64>,
    cycles: Option<f64>,
    hw_error: Option<String>,
    unpriced_ops: f64,
    rel_wall: Option<f64>,
    rel_gae: f64,
    rel_native: f64,
    os_wall: Option<f64>,
    os_gae: f64,
    os_insert: f64,
    /// Everything the twin check compares.
    fingerprint: String,
}

fn obs(r: &Record) -> Result<Obs, String> {
    let s0 = band::priced_s(r, 0.0)?;
    let s1 = band::priced_s(r, 1.0)?;
    let s01 = band::priced_s(r, 0.1)?;
    let verified = r.outcome.status == "verified" && r.outcome.matches_target == Some(true);
    if verified && (s0.is_none() || s1.is_none()) {
        return Err(format!(
            "{}: verified row with no priced S at w = 1 (an unpriced counter has no count)",
            r.record_id
        ));
    }
    let slope = match (s0, s1) {
        (Some(a), Some(b)) => Some(b - a),
        _ => None,
    };
    if let (Some(a), Some(sl), Some(m)) = (s0, slope, s01) {
        if ((a + 0.1 * sl) - m).abs() > 1e-9 * m.abs().max(1.0) {
            return Err(format!("{}: band price is not linear in w", r.record_id));
        }
    }
    let unpriced: BTreeSet<&str> = r.cost.unpriced.iter().map(String::as_str).collect();
    let priced_here = |name: &str| {
        !band::is_non_operation(name)
            && (unpriced.contains(name) || unpriced.contains(format!("{name}_uncharged").as_str()))
    };
    let phase = |n: &str| r.phases.iter().find(|p| p.name == n);
    let (rel_wall, rel_gae, rel_native) = match phase("relations") {
        Some(p) => (
            p.wall_ns.map(|x| x as f64),
            p.gae,
            p.native
                .iter()
                .filter(|(k, _)| priced_here(k))
                .map(|(_, v)| *v as f64)
                .sum(),
        ),
        None => (None, 0.0, 0.0),
    };
    let (os_wall, os_gae, os_insert) = match phase("oracle_setup") {
        Some(p) => (
            p.wall_ns.map(|x| x as f64),
            p.gae,
            p.work_native
                .get("table_insert_attempts")
                .copied()
                .unwrap_or(0) as f64,
        ),
        None => (None, 0.0, 0.0),
    };
    let (u, _) = band::unpriced_total(r);
    let hw = r.time.hw_solve.as_ref();
    let mut phases = r.phases.clone();
    for p in &mut phases {
        p.wall_ns = None;
    }
    let fingerprint = serde_json::to_string(&json!({
        "algorithm_seed": r.algorithm_seed,
        "status": r.outcome.status,
        "recovered": r.outcome.recovered,
        "total_gae": r.cost.total_gae,
        "unpriced": r.cost.unpriced,
        "counters": r.counters,
        "phases": phases,
    }))
    .map_err(|e| e.to_string())?;
    Ok(Obs {
        status: r.outcome.status.clone(),
        verified,
        s0,
        s_slope: slope,
        gae: r.cost.total_gae,
        l2: r.isolation.level >= Level::L2,
        wall: r.time.solve_wall_ns.map(|x| x as f64),
        cycles: hw.and_then(|h| h.cycles).map(|x| x as f64),
        hw_error: hw.and_then(|h| h.error.clone()),
        unpriced_ops: u as f64,
        rel_wall,
        rel_gae,
        rel_native,
        os_wall,
        os_gae,
        os_insert,
        fingerprint,
    })
}

// ── Loading and validation (arm names visible only here) ──────────────

struct Replicate {
    label: &'static str,
    session_id: String,
    spec_id: String,
    records_sha256: String,
    status_counts: BTreeMap<String, u64>,
    /// (slug, target_index, workload_id, round) → arm → obs.
    cells: BTreeMap<(String, u64, String, u32), BTreeMap<String, Obs>>,
    /// (workload_id, round) → algorithm seed of the rho-neg row.
    seeds: BTreeMap<(String, u32), String>,
    r_of: BTreeMap<String, u128>,
}

fn load(dir: &Path, label: &'static str, devcheck: bool) -> Result<Replicate, String> {
    let session: Session = serde_json::from_slice(
        &std::fs::read(dir.join("session.json"))
            .map_err(|e| format!("{}: {e}", dir.join("session.json").display()))?,
    )
    .map_err(|e| format!("{}/session.json: {e}", dir.display()))?;
    let body = std::fs::read(dir.join("records.jsonl"))
        .map_err(|e| format!("{}: {e}", dir.join("records.jsonl").display()))?;
    let records_sha256 = sha256_hex(&body);
    if session.records_sha256.as_deref() != Some(records_sha256.as_str()) {
        return Err(format!(
            "replicate {label}: records.jsonl sha256 {records_sha256} is not session.json's {:?}",
            session.records_sha256
        ));
    }
    if session.status != "complete" || session.records_written != session.executions_planned {
        return Err(format!(
            "replicate {label}: session {} is {} with {} of {} records",
            session.session_id, session.status, session.records_written, session.executions_planned
        ));
    }
    let expected_spec = if label == "A" { SPEC_ID_A } else { SPEC_ID_B };
    if session.spec_id != expected_spec && !devcheck {
        return Err(format!(
            "replicate {label}: spec {} is not the frozen {expected_spec}",
            session.spec_id
        ));
    }
    let lines = read_record_lines(dir)?;
    if lines.len() as u64 != session.records_written {
        return Err(format!(
            "replicate {label}: record count differs from session.json"
        ));
    }
    let mut rep = Replicate {
        label,
        session_id: session.session_id.clone(),
        spec_id: session.spec_id.clone(),
        records_sha256,
        status_counts: BTreeMap::new(),
        cells: BTreeMap::new(),
        seeds: BTreeMap::new(),
        r_of: BTreeMap::new(),
    };
    let known: BTreeMap<&str, (&str, &[(&str, &str)])> =
        ARMS.iter().map(|(n, m, p)| (*n, (*m, *p))).collect();
    for (line, r) in &lines {
        if !check_line_seal(line, &r.record_id) {
            return Err(format!(
                "replicate {label}: record {} fails its seal",
                r.record_id
            ));
        }
        if r.session_id != session.session_id {
            return Err(format!(
                "replicate {label}: record {} is from another session",
                r.record_id
            ));
        }
        if r.warmup {
            continue;
        }
        let Some((method, params)) = known.get(r.arm.as_str()) else {
            return Err(format!("replicate {label}: unexpected arm {:?}", r.arm));
        };
        if r.method.id != *method {
            return Err(format!(
                "replicate {label}: arm {} runs {} not {method}",
                r.arm, r.method.id
            ));
        }
        let want: BTreeMap<String, String> = params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        if want.iter().any(|(k, v)| r.method.params.get(k) != Some(v)) {
            return Err(format!(
                "replicate {label}: arm {} params {:?} differ from the protocol's",
                r.arm, r.method.params
            ));
        }
        let frozen_id = METHOD_IDS
            .iter()
            .find(|(m, _)| m == method)
            .map(|(_, id)| *id)
            .expect("every protocol method has a frozen id");
        if r.method.method_id != frozen_id {
            return Err(format!(
                "replicate {label}: arm {} method id {} is not the frozen {frozen_id}",
                r.arm, r.method.method_id
            ));
        }
        let slug = r.workload.curve.slug.clone();
        if slug != SUBJECT && !SIBLINGS.contains(&slug.as_str()) {
            return Err(format!(
                "replicate {label}: curve {slug} is not one of the five"
            ));
        }
        rep.r_of.insert(slug.clone(), r.workload.curve.r);
        *rep.status_counts
            .entry(format!("{}:{}", r.arm, r.outcome.status))
            .or_default() += 1;
        if r.arm == NEG {
            rep.seeds.insert(
                (r.workload.workload_id.clone(), r.round),
                r.algorithm_seed.clone(),
            );
        }
        let cell = rep
            .cells
            .entry((
                slug,
                r.workload.target_index,
                r.workload.workload_id.clone(),
                r.round,
            ))
            .or_default();
        if cell.insert(r.arm.clone(), obs(r)?).is_some() {
            return Err(format!(
                "replicate {label}: two {} rows for workload {} round {}",
                r.arm, r.workload.workload_id, r.round
            ));
        }
    }
    for slug in std::iter::once(SUBJECT).chain(SIBLINGS) {
        if !rep.r_of.contains_key(slug) {
            return Err(format!(
                "replicate {label}: no measured rows on curve {slug}"
            ));
        }
    }
    for ((slug, _, wid, round), arms) in &rep.cells {
        for (name, _, _) in ARMS {
            if !arms.contains_key(name) {
                return Err(format!(
                    "replicate {label}: arm {name} missing on {slug} workload {wid} round {round}"
                ));
            }
        }
    }
    Ok(rep)
}

// ── Masking ────────────────────────────────────────────────────────────

fn mask(key: &str, arm: &str) -> String {
    format!(
        "arm-{}",
        &sha256_hex(format!("{key}\n{arm}").as_bytes())[..10]
    )
}

/// The roles the decision rule needs, as codes only.
struct Roles {
    neg: String,
    neg_aa: String,
    inc: String,
    inc_aa: String,
    plain: String,
}

/// One target of one curve, with every (replicate, round) cell, arms coded.
struct Cluster {
    workload_id: String,
    cells: [Vec<(u32, BTreeMap<String, Obs>)>; 2],
}

// ── Statistics ────────────────────────────────────────────────────────

struct Prep {
    /// Curves in fixed order: the subject first, then the siblings.
    curves: Vec<String>,
    /// curve → clusters → per-replicate sums.
    sums: Vec<Vec<[Acc; 2]>>,
    /// Which replicates' time may enter C3.
    wall_ok: [bool; 2],
    /// Per (curve, replicate): every time-eligible row carried cycles.
    hw_ok: Vec<[bool; 2]>,
}

fn cluster_sums(
    cl: &Cluster,
    roles: &Roles,
    wall_ok: [bool; 2],
    devcheck: bool,
) -> ([Acc; 2], [bool; 2]) {
    let mut out = [Acc::default(), Acc::default()];
    let mut hw_ok = [true, true];
    for rep in 0..2 {
        let a = &mut out[rep].0;
        for (_, arms) in &cl.cells[rep] {
            let (neg, inc, plain) = (&arms[&roles.neg], &arms[&roles.inc], &arms[&roles.plain]);
            if neg.verified && inc.verified {
                a[IR_N] += 1.0;
                a[INC_S0] += inc.s0.unwrap();
                a[INC_SL] += inc.s_slope.unwrap();
                a[RHO_S0] += neg.s0.unwrap();
                a[RHO_SL] += neg.s_slope.unwrap();
            }
            if neg.verified && plain.verified {
                a[PN_N] += 1.0;
                a[PLAIN_S0] += plain.s0.unwrap();
                a[NEG_S0] += neg.s0.unwrap();
            }
            if !wall_ok[rep] {
                continue;
            }
            let eligible = |o: &Obs| o.verified && (o.l2 || devcheck);
            for block in 0..2 {
                let time = |o: &Obs| if block == 0 { o.wall } else { o.cycles };
                let b = TB + block * TB_LEN;
                if eligible(plain) {
                    match (time(plain), plain.gae) {
                        (Some(t), Some(g)) => {
                            a[b + P_N] += 1.0;
                            a[b + P_G] += g;
                            a[b + P_T] += t;
                            a[b + P_GG] += g * g;
                            a[b + P_GT] += g * t;
                        }
                        _ if block == 1 => hw_ok[rep] = false,
                        _ => {}
                    }
                }
                if eligible(neg) {
                    match (time(neg), neg.gae) {
                        (Some(t), Some(g)) => {
                            a[b + N_N] += 1.0;
                            a[b + N_G] += g;
                            a[b + N_T] += t;
                            a[b + N_CANON] += neg.unpriced_ops;
                        }
                        _ if block == 1 => hw_ok[rep] = false,
                        _ => {}
                    }
                }
                // Phase wall exists only as ns: the hardware block has no
                // per-phase counterpart, so C3's lookup and insert prices are
                // wall-only.
                if block == 0 && eligible(inc) {
                    if let Some(t) = inc.rel_wall {
                        a[b + L_N] += 1.0;
                        a[b + L_T] += t;
                        a[b + L_G] += inc.rel_gae;
                        a[b + L_NAT] += inc.rel_native;
                    }
                    if let Some(t) = inc.os_wall {
                        a[b + I_N] += 1.0;
                        a[b + I_T] += t;
                        a[b + I_G] += inc.os_gae;
                        a[b + I_INS] += inc.os_insert;
                    }
                }
            }
        }
    }
    (out, hw_ok)
}

/// Per-curve, per-replicate totals of one (re)sample.
type Totals = Vec<[Acc; 2]>;

fn ratio_curve(t: &[Acc; 2], w_num: f64, w_den: f64) -> f64 {
    let num: f64 = t.iter().map(|a| a.0[INC_S0] + w_num * a.0[INC_SL]).sum();
    let den: f64 = t.iter().map(|a| a.0[RHO_S0] + w_den * a.0[RHO_SL]).sum();
    num / den
}

fn ratio_rep(a: &Acc, w: f64) -> f64 {
    (a.0[INC_S0] + w * a.0[INC_SL]) / (a.0[RHO_S0] + w * a.0[RHO_SL])
}

fn ratio_pool(ts: &[[Acc; 2]], w: f64) -> f64 {
    let num: f64 = ts
        .iter()
        .flatten()
        .map(|a| a.0[INC_S0] + w * a.0[INC_SL])
        .sum();
    let den: f64 = ts
        .iter()
        .flatten()
        .map(|a| a.0[RHO_S0] + w * a.0[RHO_SL])
        .sum();
    num / den
}

/// `(t_add, c)` of `time = c + t_add · gae` over rho-plain rows.
fn fit(a: &Acc, block: usize) -> Option<(f64, f64)> {
    let n = a.t(block, P_N);
    let (sg, st, sgg, sgt) = (
        a.t(block, P_G),
        a.t(block, P_T),
        a.t(block, P_GG),
        a.t(block, P_GT),
    );
    let den = n * sgg - sg * sg;
    if n < 3.0 || den <= 0.0 {
        return None;
    }
    let slope = (n * sgt - sg * st) / den;
    (slope > 0.0).then(|| (slope, (st - slope * sg) / n))
}

/// Measured prices, pooled over the (curve, replicate) strata that may
/// enter: (canon, lookup, insert) in GAE per operation, and the per-curve
/// values of each.
struct Prices {
    canon: f64,
    lookup: f64,
    insert: f64,
    canon_hw: f64,
    per_curve: Vec<[f64; 3]>,
}

fn prices(t: &Totals, prep: &Prep) -> Prices {
    let (mut cn, mut cd, mut ln, mut ld, mut inn, mut ind, mut hn, mut hd) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let mut hw_all = true;
    let mut per_curve = Vec::new();
    for (ci, reps) in t.iter().enumerate() {
        let (mut pc, mut pcd, mut pl, mut pld, mut pi, mut pid) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        for rep in 0..2 {
            if !prep.wall_ok[rep] {
                continue;
            }
            let a = &reps[rep];
            let Some((tadd, c)) = fit(a, 0) else {
                pc = f64::NAN;
                continue;
            };
            let canon_res = (a.t(0, N_T) - a.t(0, N_N) * c - tadd * a.t(0, N_G)) / tadd;
            pc += canon_res;
            pcd += a.t(0, N_CANON);
            pl += (a.t(0, L_T) - tadd * a.t(0, L_G)) / tadd;
            pld += a.t(0, L_NAT);
            pi += (a.t(0, I_T) - tadd * a.t(0, I_G)) / tadd;
            pid += a.t(0, I_INS);
            if prep.hw_ok[ci][rep] {
                match fit(a, 1) {
                    Some((h, hc)) => {
                        hn += (a.t(1, N_T) - a.t(1, N_N) * hc - h * a.t(1, N_G)) / h;
                        hd += a.t(1, N_CANON);
                    }
                    None => hw_all = false,
                }
            } else {
                hw_all = false;
            }
        }
        cn += pc;
        cd += pcd;
        ln += pl;
        ld += pld;
        inn += pi;
        ind += pid;
        per_curve.push([pc / pcd, pl / pld, pi / pid]);
    }
    Prices {
        canon: cn / cd,
        lookup: ln / ld,
        insert: inn / ind,
        canon_hw: if hw_all { hn / hd } else { f64::NAN },
        per_curve,
    }
}

/// Every statistic the decision rule reads, by name, from one (re)sample.
fn stats_of(t: &Totals, prep: &Prep) -> BTreeMap<String, f64> {
    let mut s = BTreeMap::new();
    let w3 = band::DEFAULT_WEIGHTS;
    let sib: Vec<[Acc; 2]> = t[1..].to_vec();
    let pr = prices(t, prep);
    let w_max = pr.canon.max(pr.lookup);
    for (ci, curve) in prep.curves.iter().enumerate() {
        for w in w3 {
            s.insert(format!("R/{curve}/w={w}"), ratio_curve(&t[ci], w, w));
            let (a, b) = (&t[ci][0], &t[ci][1]);
            s.insert(
                format!("AA_lnR/{curve}/w={w}"),
                ratio_rep(a, w).ln() - ratio_rep(b, w).ln(),
            );
            s.insert(
                format!("AA_ln_inc/{curve}/w={w}"),
                ((a.0[INC_S0] + w * a.0[INC_SL]) / (b.0[INC_S0] + w * b.0[INC_SL])).ln(),
            );
            s.insert(
                format!("AA_ln_rho/{curve}/w={w}"),
                ((a.0[RHO_S0] + w * a.0[RHO_SL]) / (b.0[RHO_S0] + w * b.0[RHO_SL])).ln(),
            );
        }
        s.insert(
            format!("R/{curve}/w=meas_own"),
            ratio_curve(&t[ci], pr.lookup, pr.canon),
        );
        s.insert(
            format!("R/{curve}/w=meas_max"),
            ratio_curve(&t[ci], w_max, w_max),
        );
        let plain: f64 = t[ci].iter().map(|a| a.0[PLAIN_S0]).sum();
        let neg: f64 = t[ci].iter().map(|a| a.0[NEG_S0]).sum();
        s.insert(format!("pos/{curve}"), plain / neg);
        let [c, l, i] = pr.per_curve[ci];
        s.insert(format!("w_canon/{curve}"), c);
        s.insert(format!("w_lookup/{curve}"), l);
        s.insert(format!("w_insert/{curve}"), i);
    }
    for w in w3 {
        let rt = ratio_curve(&t[0], w, w);
        let rs = ratio_pool(&sib, w);
        s.insert(format!("R_sib/w={w}"), rs);
        s.insert(format!("D/w={w}"), rt.ln() - rs.ln());
        let aa: Vec<Acc> = t.iter().map(|r| r[0]).collect();
        let bb: Vec<Acc> = t.iter().map(|r| r[1]).collect();
        let pool_rep = |v: &[Acc]| {
            let num: f64 = v.iter().map(|a| a.0[INC_S0] + w * a.0[INC_SL]).sum();
            let den: f64 = v.iter().map(|a| a.0[RHO_S0] + w * a.0[RHO_SL]).sum();
            (num / den).ln()
        };
        s.insert(
            format!("AA_lnR/pooled/w={w}"),
            pool_rep(&aa) - pool_rep(&bb),
        );
    }
    let plain: f64 = t.iter().flatten().map(|a| a.0[PLAIN_S0]).sum();
    let neg: f64 = t.iter().flatten().map(|a| a.0[NEG_S0]).sum();
    s.insert("pos/pooled".into(), plain / neg);
    s.insert("w_canon".into(), pr.canon);
    s.insert("w_lookup".into(), pr.lookup);
    s.insert("w_insert".into(), pr.insert);
    s.insert("w_canon_hw".into(), pr.canon_hw);
    s
}

fn totals_of(prep: &Prep, pick: &[Vec<usize>]) -> Totals {
    prep.sums
        .iter()
        .zip(pick)
        .map(|(clusters, idx)| {
            let mut t = [Acc::default(), Acc::default()];
            for &i in idx {
                t[0].add(&clusters[i][0]);
                t[1].add(&clusters[i][1]);
            }
            t
        })
        .collect()
}

struct Boot {
    point: BTreeMap<String, f64>,
    draws: BTreeMap<String, Vec<f64>>,
}

/// Paired, curve-stratified bootstrap: each resample draws, within every
/// curve, as many targets as it has, with replacement; a drawn target
/// brings every (replicate, round) cell of every arm with it.
fn bootstrap(prep: &Prep) -> Boot {
    let all: Vec<Vec<usize>> = prep.sums.iter().map(|c| (0..c.len()).collect()).collect();
    let point = stats_of(&totals_of(prep, &all), prep);
    let mut draws: BTreeMap<String, Vec<f64>> = point
        .keys()
        .map(|k| (k.clone(), Vec::with_capacity(RESAMPLES)))
        .collect();
    let mut rng = Resampler::new(BOOTSTRAP_SEED);
    for _ in 0..RESAMPLES {
        let pick: Vec<Vec<usize>> = prep
            .sums
            .iter()
            .map(|c| (0..c.len()).map(|_| rng.below(c.len())).collect())
            .collect();
        for (k, v) in stats_of(&totals_of(prep, &pick), prep) {
            draws.get_mut(&k).expect("same keys").push(v);
        }
    }
    Boot { point, draws }
}

fn interval(b: &Boot, key: &str, alpha: f64) -> Option<(f64, f64)> {
    let finite: Vec<f64> = b.draws[key]
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .collect();
    if finite.len() < RESAMPLES * 9 / 10 {
        return None;
    }
    Some((
        quantile(&finite, alpha / 2.0)?,
        quantile(&finite, 1.0 - alpha / 2.0)?,
    ))
}

fn num(x: f64) -> Value {
    if x.is_finite() {
        json!(x)
    } else {
        Value::Null
    }
}

fn est(b: &Boot, key: &str, alpha: f64) -> Value {
    let ci = interval(b, key, alpha);
    json!({
        "point": num(b.point[key]),
        "level": 1.0 - alpha,
        "interval": ci.map(|(l, h)| json!([l, h])).unwrap_or(Value::Null),
        "resamples_finite": b.draws[key].iter().filter(|v| v.is_finite()).count(),
    })
}

// ── Main ───────────────────────────────────────────────────────────────

struct Args {
    a: PathBuf,
    b: PathBuf,
    out: PathBuf,
    wall_a: Option<PathBuf>,
    wall_b: Option<PathBuf>,
    devcheck: bool,
}

fn args() -> Result<Args, String> {
    let mut it = std::env::args().skip(1);
    let (mut a, mut b, mut out, mut wall_a, mut wall_b, mut devcheck) =
        (None, None, None, None, None, false);
    while let Some(k) = it.next() {
        let mut val = || {
            it.next()
                .map(PathBuf::from)
                .ok_or(format!("{k} needs a value"))
        };
        match k.as_str() {
            "--a" => a = Some(val()?),
            "--b" => b = Some(val()?),
            "--out" => out = Some(val()?),
            "--wall-a" => wall_a = Some(val()?),
            "--wall-b" => wall_b = Some(val()?),
            "--devcheck" => devcheck = true,
            _ => return Err(format!("unknown argument {k}")),
        }
    }
    let need = |x: Option<PathBuf>, n: &str| x.ok_or(format!("--{n} is required"));
    Ok(Args {
        a: need(a, "a")?,
        b: need(b, "b")?,
        out: need(out, "out")?,
        wall_a,
        wall_b,
        devcheck,
    })
}

/// `(admitted, reason)` of a replicate's wall A/A.
fn wall_admission(path: Option<&Path>, devcheck: bool) -> Result<(bool, String), String> {
    let Some(p) = path else {
        return Ok(if devcheck {
            (
                true,
                "devcheck: no control comparison; wall used below L2, not data".into(),
            )
        } else {
            (false, "no control-comparison.json given".into())
        });
    };
    let c: Comparison =
        serde_json::from_slice(&std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?)
            .map_err(|e| format!("{}: {e}", p.display()))?;
    let w = &c.wall;
    let ok = w.status == "admitted" && w.aa_pairs >= MIN_AA_PAIRS && w.pairs_below_level == 0;
    Ok((
        ok || devcheck,
        format!(
            "{}: wall.status {}, aa_pairs {}, pairs_below_level {}{}",
            p.display(),
            w.status,
            w.aa_pairs,
            w.pairs_below_level,
            if !ok && devcheck {
                " (devcheck: used anyway)"
            } else {
                ""
            }
        ),
    ))
}

fn run() -> Result<(), String> {
    let args = args()?;
    let reps = [
        load(&args.a, "A", args.devcheck)?,
        load(&args.b, "B", args.devcheck)?,
    ];
    if reps[0].session_id == reps[1].session_id {
        return Err("replicates A and B are the same session".into());
    }
    let keys = |r: &Replicate| -> BTreeSet<(String, u64, String, u32)> {
        r.cells.keys().cloned().collect()
    };
    if keys(&reps[0]) != keys(&reps[1]) {
        return Err("replicates A and B do not cover the same (workload, round) cells".into());
    }
    for (k, seed) in &reps[0].seeds {
        if reps[1].seeds.get(k) == Some(seed) {
            return Err(format!(
                "workload {} round {}: A and B share an algorithm seed; the seed A/A is not independent",
                k.0, k.1
            ));
        }
    }
    let wall = [
        wall_admission(args.wall_a.as_deref(), args.devcheck)?,
        wall_admission(args.wall_b.as_deref(), args.devcheck)?,
    ];

    // C0 before masking needs only the twin pairing, which the codes keep.
    let key = sha256_hex(
        format!(
            "{PROTOCOL_ID}/mask\n{}\n{}",
            reps[0].records_sha256, reps[1].records_sha256
        )
        .as_bytes(),
    );
    let roles = Roles {
        neg: mask(&key, NEG),
        neg_aa: mask(&key, NEG_AA),
        inc: mask(&key, INC),
        inc_aa: mask(&key, INC_AA),
        plain: mask(&key, PLAIN),
    };
    let curves: Vec<String> = std::iter::once(SUBJECT)
        .chain(SIBLINGS)
        .map(str::to_string)
        .collect();
    // Mask: from here the statistics see codes only.
    let mut clusters: Vec<BTreeMap<u64, Cluster>> =
        curves.iter().map(|_| BTreeMap::new()).collect();
    for (ri, rep) in reps.iter().enumerate() {
        for ((slug, tidx, wid, round), arms) in &rep.cells {
            let ci = curves
                .iter()
                .position(|c| c == slug)
                .expect("validated curve");
            let cl = clusters[ci].entry(*tidx).or_insert_with(|| Cluster {
                workload_id: wid.clone(),
                cells: [Vec::new(), Vec::new()],
            });
            if cl.workload_id != *wid {
                return Err(format!("target {tidx} on {slug} has two workload ids"));
            }
            let coded: BTreeMap<String, Obs> = arms
                .iter()
                .map(|(name, o)| (mask(&key, name), o.clone()))
                .collect();
            cl.cells[ri].push((*round, coded));
        }
    }

    // C0: every measured row verified or censored; twins identical.
    let mut c0_failures: Vec<String> = Vec::new();
    let mut censored: BTreeMap<String, u64> = BTreeMap::new();
    for (ci, cs) in clusters.iter().enumerate() {
        for cl in cs.values() {
            for (ri, cells) in cl.cells.iter().enumerate() {
                for (round, arms) in cells {
                    for (code, o) in arms {
                        match o.status.as_str() {
                            _ if o.verified => {}
                            "exhausted" | "timeout" => {
                                *censored
                                    .entry(format!("{}|{code}", curves[ci]))
                                    .or_default() += 1
                            }
                            other => c0_failures.push(format!(
                                "{} {} round {round} {code}: status {other}",
                                reps[ri].label, cl.workload_id
                            )),
                        }
                    }
                    for (x, y) in [(&roles.neg, &roles.neg_aa), (&roles.inc, &roles.inc_aa)] {
                        if arms[x].fingerprint != arms[y].fingerprint {
                            c0_failures.push(format!(
                                "{} {} round {round}: twin {x}/{y} counts differ",
                                reps[ri].label, cl.workload_id
                            ));
                        }
                    }
                }
            }
        }
    }

    let wall_ok = [wall[0].0, wall[1].0];
    let mut sums = Vec::new();
    let mut hw_ok = Vec::new();
    for cs in &clusters {
        let mut v = Vec::new();
        let mut h = [true, true];
        for cl in cs.values() {
            let (s, ok) = cluster_sums(cl, &roles, wall_ok, args.devcheck);
            h[0] &= ok[0];
            h[1] &= ok[1];
            v.push(s);
        }
        sums.push(v);
        hw_ok.push(h);
    }
    let prep = Prep {
        curves: curves.clone(),
        sums,
        wall_ok,
        hw_ok,
    };
    let boot = bootstrap(&prep);

    // ── Decisions (still on codes) ──
    let c0_ok = c0_failures.is_empty();
    let pos = interval(&boot, "pos/pooled", ALPHA);
    let per_curve_pos_ok = curves
        .iter()
        .all(|c| boot.point[&format!("pos/{c}")] > C2_PER_CURVE);
    let c2_ok = pos.is_some_and(|(lo, _)| lo >= C2_POOLED_LOWER) && per_curve_pos_ok;
    let valid = c0_ok && c2_ok;
    let invalid = "invalid_measurement";

    let mut c1 = Vec::new();
    for w in band::DEFAULT_WEIGHTS {
        let ci = interval(&boot, &format!("D/w={w}"), ALPHA_C1);
        let aa = interval(&boot, &format!("AA_lnR/{SUBJECT}/w={w}"), ALPHA);
        let half = aa.map(|(l, h)| (h - l) / 2.0);
        // Frozen rule: the 98.33 % interval of D alone decides; the A/A
        // half-width is reported beside it, never used as a second bar.
        let verdict = match (valid, ci) {
            (false, _) | (_, None) => invalid,
            (_, Some((_, hi))) if hi < 0.0 => "tm1514 lower than siblings",
            (_, Some((lo, _))) if lo > 0.0 => "tm1514 higher than siblings",
            _ => "within the sibling ratio (draw not excluded)",
        };
        let rt = interval(&boot, &format!("R/{SUBJECT}/w={w}"), ALPHA_C1);
        let rs = interval(&boot, &format!("R_sib/w={w}"), ALPHA_C1);
        let secondary = match (rt, rs) {
            (Some((_, th)), Some((sl, _))) if th < sl => "tm1514 lower than siblings",
            (Some((tl, _)), Some((_, sh))) if tl > sh => "tm1514 higher than siblings",
            (Some(_), Some(_)) => "within the sibling ratio (draw not excluded)",
            _ => invalid,
        };
        c1.push(json!({
            "w": w,
            "D": est(&boot, &format!("D/w={w}"), ALPHA_C1),
            "aa_lnR_subject_95": est(&boot, &format!("AA_lnR/{SUBJECT}/w={w}"), ALPHA),
            "aa_half_width": half.map(num).unwrap_or(Value::Null),
            "aa_lnR_pooled_95": est(&boot, &format!("AA_lnR/pooled/w={w}"), ALPHA),
            "R_subject": est(&boot, &format!("R/{SUBJECT}/w={w}"), ALPHA_C1),
            "R_siblings": est(&boot, &format!("R_sib/w={w}"), ALPHA_C1),
            "verdict": if valid { verdict } else { invalid },
            "secondary_non_overlap": if valid { secondary } else { invalid },
        }));
    }
    let r0 = interval(&boot, &format!("R/{SUBJECT}/w=0"), ALPHA);
    let c1b = match (valid, r0) {
        (true, Some((lo, hi))) if EARLIER_R0 < lo || EARLIER_R0 > hi => {
            "the earlier 1.52 was a draw"
        }
        (true, Some(_)) => "compatible with 1.52",
        _ => invalid,
    };
    let any_wall = wall_ok[0] || wall_ok[1];
    let c3_unit = |k: &str| -> (&'static str, Value) {
        let ci = interval(&boot, k, ALPHA);
        let v = match (valid && any_wall, ci) {
            (true, Some((_, hi))) if hi < C3_UPPER => "measured price below 0.5",
            (true, Some(_)) => "measured price not resolved below 0.5",
            _ => invalid,
        };
        (v, est(&boot, k, ALPHA))
    };
    let (c3_canon, c3_canon_est) = c3_unit("w_canon");
    let (c3_lookup, c3_lookup_est) = c3_unit("w_lookup");
    let own = interval(&boot, &format!("R/{SUBJECT}/w=meas_own"), ALPHA);
    let max = interval(&boot, &format!("R/{SUBJECT}/w=meas_max"), ALPHA);
    let c4 = match (valid && any_wall, own, max) {
        (true, Some((a, _)), Some((b, _))) if a > 1.0 && b > 1.0 => {
            "the w = 1 flip does not survive measured pricing"
        }
        (true, Some(_), Some(_)) => "unresolved at measured pricing",
        _ => invalid,
    };

    // ── Unmask: names come back only now ──
    let unmask: BTreeMap<String, String> = ARMS
        .iter()
        .map(|(n, _, _)| (mask(&key, n), n.to_string()))
        .collect();
    let named = |s: &str| {
        let mut out = s.to_string();
        for (code, name) in &unmask {
            out = out.replace(code, name);
        }
        out
    };
    let c0_failures: Vec<String> = c0_failures.iter().map(|s| named(s)).collect();
    let censored: BTreeMap<String, u64> =
        censored.into_iter().map(|(k, v)| (named(&k), v)).collect();
    let censored_on_decision_arms = censored
        .keys()
        .any(|k| k.ends_with(&format!("|{NEG}")) || k.ends_with(&format!("|{INC}")));

    let mut per_curve = Vec::new();
    for (ci, c) in curves.iter().enumerate() {
        let mut rs = serde_json::Map::new();
        for w in band::DEFAULT_WEIGHTS {
            rs.insert(
                format!("w={w}"),
                est(&boot, &format!("R/{c}/w={w}"), ALPHA_C1),
            );
        }
        rs.insert(
            "w=meas_own".into(),
            est(&boot, &format!("R/{c}/w=meas_own"), ALPHA),
        );
        rs.insert(
            "w=meas_max".into(),
            est(&boot, &format!("R/{c}/w=meas_max"), ALPHA),
        );
        let mut aa = serde_json::Map::new();
        for w in band::DEFAULT_WEIGHTS {
            aa.insert(
                format!("w={w}"),
                json!({
                    "lnR": est(&boot, &format!("AA_lnR/{c}/w={w}"), ALPHA),
                    "ln_inc": est(&boot, &format!("AA_ln_inc/{c}/w={w}"), ALPHA),
                    "ln_rho_neg": est(&boot, &format!("AA_ln_rho/{c}/w={w}"), ALPHA),
                }),
            );
        }
        let pairs: f64 = prep.sums[ci].iter().flatten().map(|a| a.0[IR_N]).sum();
        // With censored rows: every pair whose two rows carry a priced S,
        // a censored row at its recorded (floor) cost.  Descriptive only.
        let (mut cn, mut cd, mut cpairs) = (0.0, 0.0, 0u64);
        for cl in clusters[ci].values() {
            for (_, arms) in cl.cells.iter().flatten() {
                if let (Some(i), Some(r)) = (arms[&roles.inc].s0, arms[&roles.neg].s0) {
                    cn += i;
                    cd += r;
                    cpairs += 1;
                }
            }
        }
        let at = |w: &str| boot.point[&format!("R/{c}/w={w}")];
        let band_order = ["0", "0.1", "1"].iter().all(|w| at(w) > 1.0)
            || ["0", "0.1", "1"].iter().all(|w| at(w) < 1.0);
        per_curve.push(json!({
            "curve": c,
            "role": if ci == 0 { "subject" } else { "sibling" },
            "r": reps[0].r_of[c].to_string(),
            "targets": prep.sums[ci].len(),
            "inc_rho_neg_pairs": pairs,
            "with_censored_w0": {"pairs": cpairs, "ratio_point": num(cn / cd)},
            "ratio_inc_over_rho_neg": Value::Object(rs),
            "band_order_holds": band_order,
            "seed_aa": Value::Object(aa),
            "ratio_plain_over_neg_w0": num(boot.point[&format!("pos/{c}")]),
            "w_meas": {
                "canon": num(boot.point[&format!("w_canon/{c}")]),
                "lookup": num(boot.point[&format!("w_lookup/{c}")]),
                "insert": num(boot.point[&format!("w_insert/{c}")]),
            },
        }));
    }
    let hw_errors: BTreeSet<String> = clusters
        .iter()
        .flat_map(|cs| cs.values())
        .flat_map(|cl| cl.cells.iter().flatten())
        .flat_map(|(_, arms)| arms.values())
        .filter_map(|o| o.hw_error.clone())
        .collect();
    let any_l2 = clusters
        .iter()
        .flat_map(|cs| cs.values())
        .flat_map(|cl| cl.cells.iter().flatten())
        .flat_map(|(_, arms)| arms.values())
        .any(|o| o.l2);

    let verdicts = json!({
        "C0": if c0_ok { "holds" } else { invalid },
        "C1": c1.iter().map(|x| json!({"w": x["w"], "verdict": x["verdict"]})).collect::<Vec<_>>(),
        "C1b": c1b,
        "C2": if c2_ok { json!("positive control detected") } else { json!("positive control failed - invalid") },
        "C3_canon": c3_canon,
        "C3_lookup": c3_lookup,
        "C4": c4,
    });
    let result = json!({
        "schema": SCHEMA,
        "protocol_id": PROTOCOL_ID,
        "data_status": if args.devcheck { "devcheck: development check on reduced specs, not data" } else { "data" },
        "inputs": reps.iter().map(|r| json!({
            "replicate": r.label,
            "session_id": r.session_id,
            "spec_id": r.spec_id,
            "records_sha256": r.records_sha256,
            "status_counts": r.status_counts,
        })).collect::<Vec<_>>(),
        "bootstrap": {
            "seed": BOOTSTRAP_SEED,
            "resamples": RESAMPLES,
            "resampler": "ecbench::stats::Resampler (splitmix64)",
            "unit": "target (all rounds of both replicates together), stratified by curve",
            "interval": "percentile; 98.33% (Bonferroni over 3 weights) for C1 and per-curve band ratios, 95% otherwise",
        },
        "masking": {
            "key_sha256": sha256_hex(key.as_bytes()),
            "codes": unmask,
            "unmasked_after": "every verdict below was computed on codes",
        },
        "C0": {"holds": c0_ok, "failures": c0_failures, "censored_rows": censored,
               "censored_on_inc_or_rho_neg": censored_on_decision_arms,
               "note": "a curve with censored inc or rho-neg rows cannot decide C1 alone; ratios exclude pairs with a censored member"},
        "C1": c1,
        "C1b": {"R_subject_w0_95": est(&boot, &format!("R/{SUBJECT}/w=0"), ALPHA), "earlier": EARLIER_R0, "verdict": c1b},
        "C2": {
            "pooled_plain_over_neg_95": est(&boot, "pos/pooled", ALPHA),
            "required_lower": C2_POOLED_LOWER,
            "per_curve_required": C2_PER_CURVE,
            "per_curve_all_above": per_curve_pos_ok,
            "expected_detection_met": c2_ok,
        },
        "C3": {
            "wall_admission": [wall[0].1, wall[1].1],
            "records_at_L2": any_l2,
            "estimator": "per (curve, replicate): least squares solve_wall_ns = c + t_add*total_gae on rho-plain; canon = sum(wall - c - t_add*gae over rho-neg)/t_add / sum canonicalisations; lookup = sum(relations wall - t_add*relations gae)/t_add / sum priced relations natives; pooled over strata",
            "w_canon_95": c3_canon_est,
            "w_lookup_95": c3_lookup_est,
            "w_insert_95": est(&boot, "w_insert", ALPHA),
            "w_canon_hw_cycles_95": est(&boot, "w_canon_hw", ALPHA),
            "hw_errors": hw_errors,
            "threshold_upper": C3_UPPER,
            "verdict_canon": c3_canon,
            "verdict_lookup": c3_lookup,
        },
        "C4": {
            "R_subject_own_prices_95": est(&boot, &format!("R/{SUBJECT}/w=meas_own"), ALPHA),
            "R_subject_max_price_95": est(&boot, &format!("R/{SUBJECT}/w=meas_max"), ALPHA),
            "verdict": c4,
        },
        "per_curve": per_curve,
        "verdicts": verdicts,
    });
    std::fs::create_dir_all(&args.out).map_err(|e| format!("{}: {e}", args.out.display()))?;
    let text = serde_json::to_string_pretty(&result).map_err(|e| e.to_string())? + "\n";
    std::fs::write(args.out.join("result.json"), &text).map_err(|e| e.to_string())?;

    let f = |v: &Value| match v.as_f64() {
        Some(x) => format!("{x:.4}"),
        None => "n/a".into(),
    };
    let ci = |v: &Value| match v["interval"].as_array() {
        Some(a) => format!("[{}, {}]", f(&a[0]), f(&a[1])),
        None => "n/a".into(),
    };
    let mut s = String::new();
    if args.devcheck {
        s += "DEVELOPMENT CHECK ON REDUCED SPECS - NOT DATA\n";
    }
    s += &format!(
        "{PROTOCOL_ID} analysis ({SCHEMA}), result.json sha256 {}\n",
        sha256_hex(text.as_bytes())
    );
    s += &format!("C0: {}\n", if c0_ok { "holds" } else { invalid });
    for x in &c1 {
        s += &format!(
            "C1 w={}: D {} {} (98.33%), A/A half-width {}: {}; non-overlap: {}\n",
            x["w"],
            f(&x["D"]["point"]),
            ci(&x["D"]),
            f(&x["aa_half_width"]),
            x["verdict"].as_str().unwrap_or(""),
            x["secondary_non_overlap"].as_str().unwrap_or("")
        );
    }
    s += &format!(
        "C1b: R_tm1514(0) {} {} (95%): {c1b}\n",
        f(&result["C1b"]["R_subject_w0_95"]["point"]),
        ci(&result["C1b"]["R_subject_w0_95"])
    );
    s +=
        &format!(
        "C2: pooled plain/neg {} {} (95%), per-curve all > {C2_PER_CURVE}: {per_curve_pos_ok}{}\n",
        f(&result["C2"]["pooled_plain_over_neg_95"]["point"]),
        ci(&result["C2"]["pooled_plain_over_neg_95"]),
        if c2_ok { "" } else { ": positive control failed - invalid" }
    );
    s += &format!(
        "C3: w_canon {} {}: {c3_canon}; w_lookup {} {}: {c3_lookup}; w_insert {} {} (reported, not in the band)\n",
        f(&c3_canon_est["point"]), ci(&c3_canon_est),
        f(&c3_lookup_est["point"]), ci(&c3_lookup_est),
        f(&result["C3"]["w_insert_95"]["point"]), ci(&result["C3"]["w_insert_95"])
    );
    s += &format!(
        "C4: R_tm1514 own prices {} {}, max price {} {}: {c4}\n",
        f(&result["C4"]["R_subject_own_prices_95"]["point"]),
        ci(&result["C4"]["R_subject_own_prices_95"]),
        f(&result["C4"]["R_subject_max_price_95"]["point"]),
        ci(&result["C4"]["R_subject_max_price_95"])
    );
    for pc in &per_curve {
        s += &format!(
            "  {} R(0) {} R(0.1) {} R(1) {} plain/neg {}\n",
            pc["curve"].as_str().unwrap_or(""),
            f(&pc["ratio_inc_over_rho_neg"]["w=0"]["point"]),
            f(&pc["ratio_inc_over_rho_neg"]["w=0.1"]["point"]),
            f(&pc["ratio_inc_over_rho_neg"]["w=1"]["point"]),
            f(&pc["ratio_plain_over_neg_w0"]),
        );
    }
    std::fs::write(args.out.join("summary.txt"), &s).map_err(|e| e.to_string())?;
    print!("{s}");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("tm1514_analyze: {e}");
        std::process::exit(1);
    }
}
