//! Round 315 analysis: amortized per-target units for the multi-target
//! transported-log candidate against the matched strong rho reference.
//!
//! Reads one or more ecbench session directories produced from the frozen
//! Round 315 specs and writes a deterministic ANALYSIS.json: the paired
//! total ratio, the amortized per-target ratio (total GAE divided by the
//! frozen target count), per-phase splits of the shared cold cost, and the
//! correctness and A/A evidence.  The analysis computes nothing the
//! sessions do not carry; every figure names its target count.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Record {
    arm: String,
    #[serde(default)]
    warmup: bool,
    outcome: Outcome,
    cost: Cost,
    workload: WorkloadRef,
    #[serde(default)]
    counters: BTreeMap<String, u64>,
    #[serde(default)]
    phases: Vec<Phase>,
}

#[derive(Deserialize)]
struct Outcome {
    status: String,
}

#[derive(Deserialize)]
struct Cost {
    total_gae: f64,
}

#[derive(Deserialize)]
struct WorkloadRef {
    workload_id: String,
}

#[derive(Deserialize)]
struct Phase {
    name: String,
    gae: f64,
}

struct Args {
    sessions: Vec<PathBuf>,
    out: PathBuf,
}

fn parse_args() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);
    let out = args
        .next_back()
        .ok_or("usage: transported_log_amortize_round315_analyze SESSION... OUT.json")?
        .into();
    let sessions: Vec<PathBuf> = args.map(Into::into).collect();
    if sessions.is_empty() {
        return Err("usage: transported_log_amortize_round315_analyze SESSION... OUT.json".into());
    }
    Ok(Args { sessions, out })
}

fn read_records(dir: &Path) -> Result<Vec<Record>, String> {
    let path = dir.join("records.jsonl");
    let text =
        std::fs::read_to_string(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let record: Record = serde_json::from_str(line)
            .map_err(|e| format!("parsing a record in {}: {e}", path.display()))?;
        out.push(record);
    }
    Ok(out)
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

/// A cluster bootstrap over workloads: resample the eight workload means
/// with replacement and recompute the paired ratio, the way `ecbench
/// compare` does for the total ratio.
fn cluster_interval(
    workload_ratios: &BTreeMap<String, (f64, f64)>,
    point: f64,
    draws: usize,
    seed: u64,
) -> (f64, f64) {
    let ids: Vec<&String> = workload_ratios.keys().collect();
    if ids.len() < 2 {
        return (f64::NAN, f64::NAN);
    }
    let mut s = seed;
    let mut samples = Vec::with_capacity(draws);
    for _ in 0..draws {
        let mut a_sum = 0.0;
        let mut b_sum = 0.0;
        for _ in 0..ids.len() {
            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let pick = ((s >> 33) as usize) % ids.len();
            let (a, b) = workload_ratios[ids[pick]];
            a_sum += a;
            b_sum += b;
        }
        if a_sum > 0.0 {
            samples.push(b_sum / a_sum);
        }
    }
    samples.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let low = samples[(0.025 * samples.len() as f64) as usize];
    let high = samples[(0.975 * samples.len() as f64).max(1.0) as usize - 1];
    let _ = point;
    (low, high)
}

fn run(args: Args) -> Result<(), String> {
    let mut rows = Vec::new();
    for dir in &args.sessions {
        let records = read_records(dir)?;
        // A/A check: the two identical candidate arms must agree exactly.
        let mut by_arm: BTreeMap<String, Vec<&Record>> = BTreeMap::new();
        for r in &records {
            by_arm.entry(r.arm.clone()).or_default().push(r);
        }
        let candidate_arms: Vec<&String> = by_arm
            .keys()
            .filter(|a| a.contains("transport-multi"))
            .collect();
        if candidate_arms.len() != 2 {
            return Err(format!(
                "{}: expected exactly two transported-multi arms (candidate and A/A), found {}",
                dir.display(),
                candidate_arms.len()
            ));
        }
        let rho_name = by_arm
            .keys()
            .find(|a| a.contains("rho"))
            .ok_or_else(|| format!("{}: no rho arm", dir.display()))?
            .clone();
        for key in [
            &rho_name,
            &candidate_arms[0].clone(),
            &candidate_arms[1].clone(),
        ] {
            let arms = &by_arm[key];
            if arms.iter().any(|r| r.warmup) {
                continue;
            }
        }
        // Measured (non-warmup) records only.
        let measured: Vec<&Record> = records
            .iter()
            .filter(|r| !r.warmup && r.outcome.status == "verified")
            .collect();
        let rho: Vec<&Record> = measured
            .iter()
            .filter(|r| r.arm == rho_name)
            .copied()
            .collect();
        let cand: Vec<&Record> = measured
            .iter()
            .filter(|r| r.arm == **candidate_arms[0])
            .copied()
            .collect();
        let aa: Vec<&Record> = measured
            .iter()
            .filter(|r| r.arm == **candidate_arms[1])
            .copied()
            .collect();
        if rho.len() != cand.len() || cand.len() != aa.len() || cand.is_empty() {
            return Err(format!(
                "{}: unbalanced measured arms {} / {} / {}",
                dir.display(),
                rho.len(),
                cand.len(),
                aa.len()
            ));
        }
        let targets: Vec<u64> = cand
            .iter()
            .map(|r| *r.counters.get("amortized_targets").unwrap_or(&0))
            .collect();
        if targets.contains(&0) || targets.iter().any(|&t| t != targets[0]) {
            return Err(format!(
                "{}: amortized_targets missing or not constant",
                dir.display()
            ));
        }
        let t = targets[0] as f64;
        let mut aa_exact = true;
        for (a, b) in cand.iter().zip(aa.iter()) {
            if (a.cost.total_gae - b.cost.total_gae).abs() > f64::EPSILON {
                aa_exact = false;
            }
        }
        // Pair by (workload, round): records arrive per workload in the
        // same order for every arm, so pair by workload in sequence.
        let mut rho_by_wl: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for r in &rho {
            rho_by_wl
                .entry(r.workload.workload_id.clone())
                .or_default()
                .push(r.cost.total_gae);
        }
        let mut cand_by_wl: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for r in &cand {
            cand_by_wl
                .entry(r.workload.workload_id.clone())
                .or_default()
                .push(r.cost.total_gae);
        }
        let mut workload_ratios = BTreeMap::new();
        for (wl, rho_gaes) in &rho_by_wl {
            let cand_gaes = cand_by_wl
                .get(wl)
                .ok_or_else(|| format!("{}: workload {wl} missing in candidate", dir.display()))?;
            workload_ratios.insert(wl.clone(), (mean(rho_gaes), mean(cand_gaes)));
        }
        let total_point = {
            let a: Vec<f64> = workload_ratios.values().map(|(a, _)| *a).collect();
            let b: Vec<f64> = workload_ratios.values().map(|(_, b)| *b).collect();
            b.iter().sum::<f64>() / a.iter().sum::<f64>()
        };
        let (total_lo, total_hi) = cluster_interval(&workload_ratios, total_point, 100_000, 315);
        // Amortized per-target: the candidate carries T targets per
        // execution, rho one.
        let amortized_point = total_point / t;
        let (amortized_lo, amortized_hi) = (total_lo / t, total_hi / t);
        // Phase split of the shared cold cost.
        let mut shared_gae = 0.0;
        let mut online_gae = 0.0;
        for r in &cand {
            for phase in &r.phases {
                if phase.name.starts_with("target") {
                    online_gae += phase.gae;
                } else {
                    shared_gae += phase.gae;
                }
            }
        }
        let executions = cand.len() as f64;
        rows.push(json!({
            "session": dir.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default(),
            "amortized_targets": targets[0],
            "measured_pairs": cand.len(),
            "verified": measured.len() + records.iter().filter(|r| r.warmup).count(),
            "total_records": records.len(),
            "aa_exact": aa_exact,
            "rho_mean_gae": mean(&rho.iter().map(|r| r.cost.total_gae).collect::<Vec<_>>()),
            "candidate_mean_total_gae": mean(&cand.iter().map(|r| r.cost.total_gae).collect::<Vec<_>>()),
            "candidate_mean_amortized_per_target_gae": mean(&cand.iter().map(|r| r.cost.total_gae).collect::<Vec<_>>()) / t,
            "total_ratio": total_point,
            "total_ratio_ci95": [total_lo, total_hi],
            "amortized_per_target_ratio": amortized_point,
            "amortized_per_target_ratio_ci95": [amortized_lo, amortized_hi],
            "shared_cold_gae_mean": shared_gae / executions,
            "online_gae_mean": online_gae / executions,
            "online_fraction_of_total": online_gae / (shared_gae + online_gae),
            "gate_amortized_ci_excludes_one": amortized_hi < 1.0,
        }));
    }
    let out = json!({
        "schema": "transport-log-amortize-round315/v1",
        "unit": "counted GAE; the amortized per-target unit divides the candidate's whole-execution cost by the named amortized_targets",
        "reference": "rho.signed_frobenius_strong, one cold target per execution, no reusable precomputation",
        "caveat": "a distinguished-point rho fleet sharing tables across targets can also amortize; this is a multi-target mechanism screen, not a one-target speedup",
        "rows": rows,
    });
    let text = serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?;
    std::fs::write(&args.out, text).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    if let Err(e) = run(parse_args().expect("args")) {
        eprintln!("round315 analyze: {e}");
        std::process::exit(1);
    }
}
