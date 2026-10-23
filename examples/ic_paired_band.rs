//! Paired ratios of ecbench arms at the AGENTS.md 5a pricing weights.
//!
//! `ecbench compare` reports the recorded `S` (weight 0 for an arm whose
//! unpriced counters are all emitted); `ecbench band` reports per-arm means at
//! each weight. Neither gives a paired interval at a weight above 0, and
//! `ecbench compare --weight` is not on every branch. This tool computes the
//! same statistic as `ecbench compare` (ratio of sums over (workload, round)
//! pairs, two-stage cluster bootstrap over workloads) on the band's priced `S`
//! at each weight, from the records of one session or of a set of sessions.
//!
//! Usage:
//!   ic_paired_band --out OUT.json --weights 0,0.1,1 --pair BASE:CAND [--pair ...]
//!                  [--resamples 4000] [--seed 20261002] [--select ARM[,ARM...]]
//!                  DIR [DIR ...]
//!
//! `--pair A:B` reports `sum S_B / sum S_A` (B over A) per curve, plus the
//! number of pairs, the verified count and whether the algorithm seeds agree.
//! `--objective-base ARM` adds to every arm `minimax_vs_base`, the worst ratio over
//! the weights of its mean priced S to ARM's on the same curve (null unless all runs
//! verified): the frozen train-panel selection objective.
//! `--select` prints, per curve, each listed arm's mean priced S at each
//! weight (used for the frozen train-panel selection).

use std::collections::BTreeMap;
use std::path::PathBuf;

use crypto_lib::cryptanalysis::ecbench::band::priced_s;
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::runner::read_records;
use crypto_lib::cryptanalysis::ecbench::stats::cluster_bootstrap_ci;
use serde_json::{json, Value};

fn ratio_of_sums(strata: &[Vec<(f64, f64)>]) -> Option<f64> {
    let (mut sa, mut sb) = (0.0, 0.0);
    for p in strata.iter().flatten() {
        sa += p.0;
        sb += p.1;
    }
    (sa > 0.0).then(|| sb / sa)
}

fn main() {
    let mut out: Option<PathBuf> = None;
    let mut weights = vec![0.0, 0.1, 1.0];
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut resamples = 4000usize;
    let mut seed = 20261002u64;
    let mut select: Vec<String> = Vec::new();
    let mut objective_base: Option<String> = None;
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" => out = it.next().map(PathBuf::from),
            "--weights" => {
                weights = it
                    .next()
                    .expect("--weights value")
                    .split(',')
                    .map(|x| x.parse().expect("weight"))
                    .collect()
            }
            "--pair" => {
                let v = it.next().expect("--pair A:B");
                let (x, y) = v.split_once(':').expect("A:B");
                pairs.push((x.into(), y.into()));
            }
            "--resamples" => resamples = it.next().unwrap().parse().unwrap(),
            "--seed" => seed = it.next().unwrap().parse().unwrap(),
            "--objective-base" => objective_base = it.next(),
            "--select" => select = it.next().unwrap().split(',').map(String::from).collect(),
            _ => dirs.push(PathBuf::from(a)),
        }
    }
    let mut records: Vec<Record> = Vec::new();
    for d in &dirs {
        records.extend(read_records(d).unwrap_or_else(|e| panic!("{}: {e}", d.display())));
    }
    let measured: Vec<&Record> = records.iter().filter(|r| !r.warmup).collect();
    let mut curves: Vec<String> = measured
        .iter()
        .map(|r| r.workload.curve.slug.clone())
        .collect();
    curves.sort();
    curves.dedup();

    let mut report = json!({
        "schema": "ic_paired_band/v1",
        "weights": weights,
        "resamples": resamples,
        "seed": seed,
        "dirs": dirs.iter().map(|d| d.display().to_string()).collect::<Vec<_>>(),
        "pairs": [],
        "arms": [],
    });
    let get = |curve: &str, arm: &str| -> Vec<&Record> {
        measured
            .iter()
            .copied()
            .filter(|r| r.workload.curve.slug == curve && r.arm == arm)
            .collect()
    };
    for curve in &curves {
        let mut arms: Vec<String> = measured
            .iter()
            .filter(|r| &r.workload.curve.slug == curve)
            .map(|r| r.arm.clone())
            .collect();
        arms.sort();
        arms.dedup();
        for arm in &arms {
            if !select.is_empty() && !select.contains(arm) {
                continue;
            }
            let rs = get(curve, arm);
            let ok: Vec<&&Record> = rs.iter().filter(|r| r.counts()).collect();
            let mut means: Vec<Value> = Vec::new();
            for &w in &weights {
                let mut v = Vec::new();
                let mut all = !ok.is_empty();
                for r in &ok {
                    match priced_s(r, w).expect("priced_s") {
                        Some(x) => v.push(x),
                        None => all = false,
                    }
                }
                means.push(if all {
                    json!(v.iter().sum::<f64>() / v.len() as f64)
                } else {
                    Value::Null
                });
            }
            let r0 = rs.first().map(|r| r.workload.curve.r).unwrap_or(0);
            report["arms"].as_array_mut().unwrap().push(json!({
                "curve": curve, "r": r0, "arm": arm, "runs": rs.len(),
                "verified": ok.len(), "mean_priced_s": means,
            }));
        }
        for (a, b) in &pairs {
            let key = |r: &&Record| (r.workload.workload_id.clone(), r.round);
            let ra: BTreeMap<_, _> = get(curve, a).iter().map(|r| (key(r), *r)).collect();
            let rb: BTreeMap<_, _> = get(curve, b).iter().map(|r| (key(r), *r)).collect();
            if ra.is_empty() || rb.is_empty() {
                continue;
            }
            let mut same_seeds = true;
            let mut verified = true;
            let mut n_pairs = 0usize;
            let mut per_w: Vec<BTreeMap<String, Vec<(f64, f64)>>> =
                weights.iter().map(|_| BTreeMap::new()).collect();
            let mut unrankable = vec![false; weights.len()];
            for (k, x) in &ra {
                let Some(y) = rb.get(k) else { continue };
                same_seeds &= x.algorithm_seed == y.algorithm_seed;
                verified &= x.counts() && y.counts();
                n_pairs += 1;
                for (i, &w) in weights.iter().enumerate() {
                    match (priced_s(x, w).unwrap(), priced_s(y, w).unwrap()) {
                        (Some(sa), Some(sb)) => per_w[i].entry(k.0.clone()).or_default().push((sa, sb)),
                        _ => unrankable[i] = true,
                    }
                }
            }
            let mut cells = Vec::new();
            for (i, &w) in weights.iter().enumerate() {
                let strata: Vec<Vec<(f64, f64)>> = per_w[i].values().cloned().collect();
                if unrankable[i] || strata.is_empty() {
                    cells.push(json!({"weight": w, "ratio": null, "ci95": null,
                        "note": "an unpriced counter has no count: lower bound, not rankable"}));
                    continue;
                }
                let ratio = ratio_of_sums(&strata);
                let ci = cluster_bootstrap_ci(&strata, resamples, seed, |s| ratio_of_sums(s));
                let (sa, sb): (f64, f64) = strata
                    .iter()
                    .flatten()
                    .fold((0.0, 0.0), |acc, p| (acc.0 + p.0, acc.1 + p.1));
                let n = strata.iter().map(|s| s.len()).sum::<usize>() as f64;
                cells.push(json!({"weight": w, "ratio_b_over_a": ratio,
                    "ci95": ci.map(|c| json!([c.0, c.1])),
                    "mean_s_a": sa / n, "mean_s_b": sb / n, "workloads": strata.len()}));
            }
            report["pairs"].as_array_mut().unwrap().push(json!({
                "curve": curve, "a": a, "b": b, "pairs": n_pairs, "same_seeds": same_seeds,
                "all_verified": verified, "weights": cells,
            }));
        }
    }
    // Selection objective: the worst ratio over the band of an arm's mean
    // priced S to the base arm's, on the same curve. `null` unless every arm
    // run verified and every weight is priced.
    if let Some(base) = &objective_base {
        let arms = report["arms"].as_array().unwrap().clone();
        let mut out_arms = Vec::new();
        for a in &arms {
            let b = arms.iter().find(|x| x["curve"] == a["curve"] && x["arm"] == json!(base));
            let obj = b.and_then(|b| {
                if a["verified"] != a["runs"] || b["verified"] != b["runs"] {
                    return None;
                }
                let mut worst: f64 = 0.0;
                for i in 0..weights.len() {
                    let (x, y) = (a["mean_priced_s"][i].as_f64()?, b["mean_priced_s"][i].as_f64()?);
                    worst = worst.max(x / y);
                }
                Some(worst)
            });
            let mut a = a.clone();
            a["minimax_vs_base"] = obj.map(|v| json!(v)).unwrap_or(Value::Null);
            out_arms.push(a);
        }
        report["arms"] = Value::Array(out_arms);
        report["objective_base"] = json!(base);
    }
    let text = serde_json::to_string_pretty(&report).unwrap();
    match out {
        Some(p) => std::fs::write(p, &text).unwrap(),
        None => println!("{text}"),
    }
    // A plain-text table on stdout.
    for p in report["pairs"].as_array().unwrap() {
        for c in p["weights"].as_array().unwrap() {
            println!(
                "{} {}->{} w={} ratio={} ci={} pairs={} same_seeds={} verified={}",
                p["curve"].as_str().unwrap().chars().take(24).collect::<String>(),
                p["a"].as_str().unwrap(),
                p["b"].as_str().unwrap(),
                c["weight"],
                c["ratio_b_over_a"],
                c["ci95"],
                p["pairs"],
                p["same_seeds"],
                p["all_verified"]
            );
        }
    }
}
