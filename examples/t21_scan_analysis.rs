//! T21 analysis: paired scan A/B (ns per probe, rounds paired), the process-start
//! A/B and the callgrind instruction counts, from the records in `<runs_dir>`.
//! Usage: t21_scan_analysis <runs_dir> <out_dir>.  Computation only.

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const BOOTSTRAP: usize = 10_000;
const ARMS: [&str; 5] = ["ctl", "sparse", "algebra", "all", "ctl2"];

struct Lcg(u64);
impl Lcg {
    fn below(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) % n as u64) as usize
    }
}

fn prof(path: &Path) -> Option<(f64, u64, u64)> {
    let t = fs::read_to_string(path).ok()?;
    let l = t.lines().find(|l| l.starts_with("PROF"))?;
    let get = |k: &str| -> Option<f64> {
        l.split_whitespace()
            .find_map(|w| w.strip_prefix(k))
            .and_then(|v| v.trim_start_matches('=').parse().ok())
    };
    Some((
        get("ns_per_probe")?,
        get("probes")? as u64,
        get("hits")? as u64,
    ))
}

fn real(path: &Path) -> Option<f64> {
    let t = fs::read_to_string(path).ok()?;
    t.split_whitespace()
        .find_map(|w| w.strip_prefix("real="))
        .and_then(|v| v.parse().ok())
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    }
}

/// Paired ratio of medians cand/base over rounds, bootstrap over rounds.
fn paired(base: &[f64], cand: &[f64], seed: u64) -> (f64, f64, f64, f64) {
    let n = base.len();
    let point = median(cand) / median(base);
    let mut rng = Lcg(seed);
    let mut rs: Vec<f64> = (0..BOOTSTRAP)
        .map(|_| {
            let idx: Vec<usize> = (0..n).map(|_| rng.below(n)).collect();
            let b: Vec<f64> = idx.iter().map(|&i| base[i]).collect();
            let c: Vec<f64> = idx.iter().map(|&i| cand[i]).collect();
            median(&c) / median(&b)
        })
        .collect();
    rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| rs[((rs.len() - 1) as f64 * p).round() as usize];
    // mean of per-round ratios as a secondary point
    let per: f64 = base.iter().zip(cand).map(|(b, c)| c / b).sum::<f64>() / n as f64;
    (point, q(0.025), q(0.975), per)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let runs = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    fs::create_dir_all(out).unwrap();

    // ---- stage A ----------------------------------------------------------
    let mut ns: Vec<Vec<f64>> = vec![Vec::new(); 5];
    let mut ident = true;
    let mut ref_counts: Option<(u64, u64)> = None;
    for r in 1..=7 {
        for (a, arm) in ARMS.iter().enumerate() {
            if let Some((v, probes, hits)) = prof(&runs.join(format!("scan_{arm}_{r}.txt"))) {
                ns[a].push(v);
                match ref_counts {
                    None => ref_counts = Some((probes, hits)),
                    Some(c) => ident &= c == (probes, hits),
                }
            }
        }
    }
    let complete = ns.iter().all(|v| v.len() == 7);
    let mut arms_json = serde_json::Map::new();
    let mut kill = json!({});
    if complete {
        for (a, arm) in ARMS.iter().enumerate().skip(1) {
            let (p, lo, hi, per) = paired(&ns[0], &ns[a], 100 + a as u64);
            arms_json.insert(
                arm.to_string(),
                json!({"ns_per_probe_median": median(&ns[a]), "ns_per_probe_all": ns[a],
                    "ratio_to_ctl": {"point": p, "lo": lo, "hi": hi, "mean_of_round_ratios": per},
                    "speedup_vs_ctl": 1.0 / p, "speedup_ci": [1.0 / hi, 1.0 / lo]}),
            );
        }
        let gate = |arm: &str| {
            let r = &arms_json[arm]["ratio_to_ctl"];
            r["point"].as_f64().unwrap() <= 1.0 / 1.2 && r["hi"].as_f64().unwrap() <= 0.90
        };
        kill = json!({
            "rule": "a cut clears only if the median ratio to ctl <= 1/1.2 (0.8333) and the bootstrap upper bound <= 0.90",
            "sparse_clears": gate("sparse"), "algebra_clears": gate("algebra"), "all_clears": gate("all"),
            "end_to_end_stage_runs": gate("all") || gate("sparse") || gate("algebra"),
        });
    }
    let ctl_median = if complete { median(&ns[0]) } else { f64::NAN };

    // ---- stage P ----------------------------------------------------------
    let (mut old, mut new) = (Vec::new(), Vec::new());
    for r in 1..=5 {
        if let (Some(o), Some(n)) = (
            real(&runs.join(format!("proc_old_{r}.time"))),
            real(&runs.join(format!("proc_new_{r}.time"))),
        ) {
            old.push(o);
            new.push(n);
        }
    }
    let proc_json = if old.len() == 5 {
        let (p, lo, hi, _) = paired(&old, &new, 7);
        json!({"old_real_s": old, "new_real_s": new, "new_over_old": {"point": p, "lo": lo, "hi": hi},
            "saved_s_median": median(&old) - median(&new)})
    } else {
        json!(null)
    };

    // ---- stage C ----------------------------------------------------------
    let mut cg = serde_json::Map::new();
    for arm in &ARMS[..4] {
        if let Ok(t) = fs::read_to_string(runs.join(format!("cg_{arm}.txt"))) {
            let total = t
                .lines()
                .find(|l| l.contains("Collected"))
                .and_then(|l| l.rsplit(':').next())
                .and_then(|v| v.trim().parse::<u64>().ok());
            cg.insert(arm.to_string(), json!({"collected_ir": total, "lines": t.lines().filter(|l| !l.starts_with("PROF") && !l.contains("Collected")).map(|l| l.trim().to_string()).collect::<Vec<_>>()}));
        }
    }

    let result = json!({
        "stage_A_scan": {"complete": complete, "outputs_identical_probes_hits_across_all_runs": ident,
            "ctl_ns_per_probe_median": ctl_median, "arms": Value::Object(arms_json),
            "A_A_ctl2_over_ctl_is_in_arms_as": "ctl2 entry", "gate": kill},
        "stage_P_process_start": proc_json,
        "stage_C_callgrind": Value::Object(cg),
    });
    fs::write(
        out.join("analysis.json"),
        serde_json::to_string_pretty(&result).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&result["stage_A_scan"]["gate"]).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&result["stage_P_process_start"]).unwrap()
    );
}
