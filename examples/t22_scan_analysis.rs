//! T22 analysis: paired scan A/B of the old binary against the binary with the
//! inlined carry-less multiply (ns per probe, rounds paired, A/A arm), and the
//! callgrind instruction counts.  Usage: t22_scan_analysis <runs_dir> <out_dir>.
//! Computation only.

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const BOOTSTRAP: usize = 10_000;
const ARMS: [&str; 3] = ["old", "new", "old2"];

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

fn paired(base: &[f64], cand: &[f64], seed: u64) -> (f64, f64, f64) {
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
    (point, q(0.025), q(0.975))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let runs = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    fs::create_dir_all(out).unwrap();
    let mut ns: Vec<Vec<f64>> = vec![Vec::new(); 3];
    let mut ident = true;
    let mut reference: Option<(u64, u64)> = None;
    for r in 1..=7 {
        for (a, arm) in ARMS.iter().enumerate() {
            if let Some((v, probes, hits)) = prof(&runs.join(format!("scan_{arm}_{r}.txt"))) {
                ns[a].push(v);
                match reference {
                    None => reference = Some((probes, hits)),
                    Some(c) => ident &= c == (probes, hits),
                }
            }
        }
    }
    let complete = ns.iter().all(|v| v.len() == 7);
    let mut arms = serde_json::Map::new();
    let mut gate = json!(null);
    if complete {
        for (a, arm) in ARMS.iter().enumerate().skip(1) {
            let (p, lo, hi) = paired(&ns[0], &ns[a], 200 + a as u64);
            arms.insert(arm.to_string(), json!({"ns_per_probe_median": median(&ns[a]), "ns_per_probe_all": ns[a],
                "ratio_to_old": {"point": p, "lo": lo, "hi": hi}, "speedup_vs_old": 1.0 / p, "speedup_ci": [1.0 / hi, 1.0 / lo]}));
        }
        let r = &arms["new"]["ratio_to_old"];
        let clears = r["point"].as_f64().unwrap() <= 1.0 / 1.2 && r["hi"].as_f64().unwrap() <= 0.90;
        gate = json!({"rule": "median ratio new/old <= 0.8333 and bootstrap upper bound <= 0.90", "new_clears": clears,
            "end_to_end_stage_runs": clears});
    }
    let mut cg = serde_json::Map::new();
    for arm in ["old", "new"] {
        if let Ok(t) = fs::read_to_string(runs.join(format!("cg_{arm}.txt"))) {
            let total = t
                .lines()
                .find(|l| l.contains("Collected"))
                .and_then(|l| l.rsplit(':').next())
                .and_then(|v| v.trim().parse::<u64>().ok());
            cg.insert(arm.to_string(), json!({"collected_ir": total, "lines": t.lines().filter(|l| !l.starts_with("PROF") && !l.contains("Collected")).map(|l| l.trim().to_string()).collect::<Vec<_>>()}));
        }
    }
    let result = json!({"scan": {"complete": complete, "old_ns_per_probe_median": if complete { median(&ns[0]) } else { f64::NAN },
        "outputs_identical_probes_hits": ident, "arms": Value::Object(arms), "gate": gate},
        "callgrind_k40": Value::Object(cg)});
    fs::write(
        out.join("analysis.json"),
        serde_json::to_string_pretty(&result).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&result["scan"]["gate"]).unwrap()
    );
}
