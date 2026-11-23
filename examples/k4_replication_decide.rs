//! The frozen analysis of K4-CHAR2-REPLICATION-20261029
//! (`research/fd_paper_data_request_20261029/protocols/k4-char2-replication/`):
//! the decision rule of its `protocol.json`, computed from session records.
//!
//! ```text
//! cargo run --release --example k4_replication_decide -- \
//!     [--candidate ic-wied] [--reference rho-strong] [--baseline rho-neg] \
//!     [--aa rho-neg-aa] [--resamples 10000] [--seed 20261030] SESSION_DIR...
//! ```
//!
//! Arm names can be replaced by the masked labels of a blinded copy.  For
//! every `m` and weight `w ∈ {0, 0.1, 1}` (AGENTS.md §5a) the cost of a
//! record is its charged GAE plus `w` GAE per unpriced operation
//! (`band::unpriced_total`).  `band::priced_gae` would also strip and
//! reprice the units an IC run pinned, but on these curves `solve_ic_f2m4`
//! pins them in the run in counted `F_q` multiplications, not from the
//! host calibration table that function reads, so they stay as charged.  Per `m` it pairs the targets (one `workload_id`)
//! where both arms verified, and reports
//!
//! * `ratio(w) = Σ cost(candidate) / Σ cost(reference)` with a 95%
//!   paired-bootstrap percentile interval over targets;
//! * a one-sided bootstrap p-value `p(w) = (1 + #{ratio* ≥ 1}) / (B + 1)`,
//!   and `p_m = max_w p(w)` (supported only if below 1 at every weight);
//! * Holm's correction over the decisive `m = 16, 17` at α = 0.05;
//! * the verdict: `below_rho_across_band` when Holm rejects at `m`,
//!   `above_rho_across_band` when the interval's lower end is above 1 at
//!   every weight, `trades_within_band` otherwise, `invalid_measurement`
//!   with fewer than two paired targets;
//! * the controls: baseline / A/A (`within_control_distribution` when its
//!   interval holds 1 at every weight) and reference / baseline (C3).
//!
//! Rows that are warm-ups or not `verified` are left out of every ratio
//! and counted in the output.  The output is JSON on stdout.  Deterministic
//! in its inputs and `--seed`.

use std::collections::BTreeMap;
use std::path::Path;

use crypto_lib::cryptanalysis::ecbench::band::unpriced_total;
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::runner::read_records;
use crypto_lib::cryptanalysis::ecbench::workload::CurveSpec;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const DECISIVE: [u32; 2] = [16, 17];
const ALPHA: f64 = 0.05;

fn m_of(r: &Record) -> Option<u32> {
    match r.workload.curve_spec {
        CurveSpec::F2m4Torsion { m, .. } => Some(m),
        _ => None,
    }
}

/// Paired costs `(a, b)` per target at weight `w`.
fn pairs(
    rows: &[&Record],
    a: &str,
    b: &str,
    m: u32,
    w: f64,
) -> Result<Vec<(String, f64, f64)>, String> {
    let mut by: BTreeMap<String, (Option<f64>, Option<f64>)> = BTreeMap::new();
    for r in rows.iter().filter(|r| m_of(r) == Some(m)) {
        let slot = by.entry(r.workload.workload_id.clone()).or_default();
        let (u, missing) = unpriced_total(r);
        let cost = match r.cost.total_gae {
            Some(g) if !(missing && w > 0.0) => Some(g + w * u as f64),
            _ => None,
        };
        if r.arm == a {
            slot.0 = cost;
        } else if r.arm == b {
            slot.1 = cost;
        }
    }
    Ok(by
        .into_iter()
        .filter_map(|(k, (x, y))| Some((k, x?, y?)))
        .collect())
}

fn ratio(p: &[(String, f64, f64)], idx: &[usize]) -> f64 {
    let (n, d) = idx
        .iter()
        .fold((0.0, 0.0), |(n, d), &i| (n + p[i].1, d + p[i].2));
    n / d
}

struct Boot {
    point: f64,
    lo: f64,
    hi: f64,
    p_ge_1: f64,
    n: usize,
}

fn boot(p: &[(String, f64, f64)], resamples: usize, seed: u64) -> Option<Boot> {
    let n = p.len();
    if n < 2 {
        return None;
    }
    let all: Vec<usize> = (0..n).collect();
    let point = ratio(p, &all);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rs: Vec<f64> = (0..resamples)
        .map(|_| {
            let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
            ratio(p, &idx)
        })
        .collect();
    rs.sort_by(|x, y| x.total_cmp(y));
    let q = |f: f64| rs[((f * (resamples - 1) as f64).round() as usize).min(resamples - 1)];
    let ge = rs.iter().filter(|&&x| x >= 1.0).count();
    Some(Boot {
        point,
        lo: q(0.025),
        hi: q(0.975),
        p_ge_1: (1 + ge) as f64 / (resamples + 1) as f64,
        n,
    })
}

fn comparison(
    rows: &[&Record],
    a: &str,
    b: &str,
    m: u32,
    resamples: usize,
    seed: u64,
) -> Result<(Value, Vec<Option<Boot>>), String> {
    let mut per_w = Vec::new();
    let mut boots = Vec::new();
    for (wi, &w) in WEIGHTS.iter().enumerate() {
        let p = pairs(rows, a, b, m, w)?;
        let s = seed ^ (u64::from(m) << 8) ^ wi as u64;
        let bt = boot(&p, resamples, s);
        per_w.push(match &bt {
            Some(bt) => json!({
                "w": w, "paired_targets": bt.n, "ratio": bt.point,
                "ci95": [bt.lo, bt.hi], "p_ratio_ge_1": bt.p_ge_1,
            }),
            None => json!({"w": w, "paired_targets": p.len(), "ratio": null}),
        });
        boots.push(bt);
    }
    Ok((json!({"a": a, "b": b, "m": m, "band": per_w}), boots))
}

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut names = BTreeMap::from([
        ("candidate", "ic-wied".to_string()),
        ("reference", "rho-strong".to_string()),
        ("baseline", "rho-neg".to_string()),
        ("aa", "rho-neg-aa".to_string()),
    ]);
    let (mut resamples, mut seed) = (10_000usize, 20_261_030u64);
    let mut dirs = Vec::new();
    while let Some(a) = args.next() {
        let mut val = || args.next().ok_or(format!("{a} takes a value"));
        match a.as_str() {
            "--candidate" => drop(names.insert("candidate", val()?)),
            "--reference" => drop(names.insert("reference", val()?)),
            "--baseline" => drop(names.insert("baseline", val()?)),
            "--aa" => drop(names.insert("aa", val()?)),
            "--resamples" => resamples = val()?.parse().map_err(|e| format!("{e}"))?,
            "--seed" => seed = val()?.parse().map_err(|e| format!("{e}"))?,
            d => dirs.push(d.to_string()),
        }
    }
    if dirs.is_empty() || resamples < 100 {
        return Err("give at least one session directory and --resamples ≥ 100".into());
    }
    let mut all = Vec::new();
    for d in &dirs {
        all.extend(read_records(Path::new(d))?);
    }
    let excluded: Vec<Value> = all
        .iter()
        .filter(|r| r.warmup || r.outcome.status != "verified")
        .map(|r| json!({"arm": r.arm, "workload": r.workload.workload_id, "warmup": r.warmup, "status": r.outcome.status}))
        .collect();
    let rows: Vec<&Record> = all
        .iter()
        .filter(|r| !r.warmup && r.outcome.status == "verified")
        .collect();
    let mut ms: Vec<u32> = rows.iter().filter_map(|r| m_of(r)).collect();
    ms.sort_unstable();
    ms.dedup();
    let (cand, refr, base, aa) = (
        &names["candidate"],
        &names["reference"],
        &names["baseline"],
        &names["aa"],
    );
    let mut per_m = Vec::new();
    let mut decisive_p: Vec<(u32, f64)> = Vec::new();
    let mut verdicts: BTreeMap<u32, &str> = BTreeMap::new();
    for &m in &ms {
        let (c2, boots) = comparison(&rows, cand, refr, m, resamples, seed)?;
        let (aa_cmp, aa_boots) = comparison(&rows, base, aa, m, resamples, seed ^ 0xaa)?;
        let (c3, _) = comparison(&rows, refr, base, m, resamples, seed ^ 0xc3)?;
        let valid = boots.iter().all(Option::is_some);
        let verdict = if !valid {
            "invalid_measurement"
        } else if boots.iter().flatten().all(|b| b.lo > 1.0) {
            "above_rho_across_band"
        } else {
            "trades_within_band"
        };
        verdicts.insert(m, verdict);
        if valid && DECISIVE.contains(&m) {
            let p = boots.iter().flatten().map(|b| b.p_ge_1).fold(0.0, f64::max);
            decisive_p.push((m, p));
        }
        let aa_within = aa_boots
            .iter()
            .all(|b| b.as_ref().is_some_and(|b| b.lo <= 1.0 && 1.0 <= b.hi));
        per_m.push(json!({
            "m": m,
            "C2_candidate_over_reference": c2,
            "C_aa": aa_cmp,
            "C_aa_verdict": if aa_within { "within_control_distribution" } else { "outside_or_invalid" },
            "C3_reference_over_baseline": c3,
        }));
    }
    // Holm over the decisive m.
    decisive_p.sort_by(|a, b| a.1.total_cmp(&b.1));
    let k = decisive_p.len();
    let mut holm = Vec::new();
    let mut still = true;
    for (i, &(m, p)) in decisive_p.iter().enumerate() {
        let level = ALPHA / (k - i) as f64;
        let reject = still && p <= level;
        still = reject;
        if reject {
            verdicts.insert(m, "below_rho_across_band");
        }
        holm.push(
            json!({"m": m, "p_max_over_band": p, "holm_level": level, "reject_ratio_ge_1": reject}),
        );
    }
    let out = json!({
        "schema": "k4-char2-replication.decision/v1",
        "protocol_id": "K4-CHAR2-REPLICATION-20261029",
        "arms": names,
        "weights": WEIGHTS,
        "resamples": resamples,
        "seed": seed,
        "sessions": dirs,
        "excluded_rows": excluded,
        "per_m": per_m,
        "holm_decisive": holm,
        "verdict_by_m": verdicts,
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?
    );
    Ok(())
}
