//! Analysis for `research/scalar_mul_ct_timing_20261022` (PROTOCOL.md):
//! reads `<raw>/timing.jsonl` and `<raw>/isolation.jsonl`, writes
//! `<summary>` into the study directory and prints a Markdown summary.
//! `<raw>` defaults to `raw` (run 1) and `<summary>` to `summary.json`;
//! run 2 (PROTOCOL.md, amendment 1) is `raw-run2 summary-run2.json`.
//!
//! ```sh
//! cargo run --release --example scalar_mul_timing_analyze -- research/scalar_mul_ct_timing_20261022 [raw] [summary.json]
//! ```

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};

const ARMS: [&str; 3] = ["A", "A2", "B"];

fn read_jsonl(path: &Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("json line"))
        .collect()
}

fn median(mut v: Vec<f64>) -> f64 {
    assert!(!v.is_empty());
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        (v[m - 1] + v[m]) / 2.0
    }
}

/// Per leading-zero class, the median nanoseconds over the class's scalars
/// and repetitions.
fn class_medians(curve: &Value) -> BTreeMap<u64, f64> {
    let mut by: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for r in curve["rows"].as_array().unwrap() {
        let z = r["leading_zeros"].as_u64().unwrap();
        for ns in r["ns"].as_array().unwrap() {
            by.entry(z).or_default().push(ns.as_f64().unwrap());
        }
    }
    by.into_iter().map(|(z, v)| (z, median(v))).collect()
}

/// The result digests of a curve's scalars, in (class, index) order.
fn digests(curve: &Value) -> Vec<String> {
    let mut rows: Vec<_> = curve["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["leading_zeros"].as_u64().unwrap(),
                r["idx"].as_u64().unwrap(),
                r["result"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    rows.sort();
    rows.into_iter().map(|r| r.2).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = std::path::PathBuf::from(args.first().expect("study directory"));
    let raw = dir.join(args.get(1).map_or("raw", String::as_str));
    let out = dir.join(args.get(2).map_or("summary.json", String::as_str));
    let runs = read_jsonl(&raw.join("timing.jsonl"));
    let iso: BTreeMap<String, Value> = read_jsonl(&raw.join("isolation.jsonl"))
        .into_iter()
        .map(|r| (r["label"].as_str().unwrap().to_string(), r))
        .collect();
    let clean = |label: &str| {
        iso.get(label).is_some_and(|r| {
            r["run"]["contended"] == json!(false) && r["run"]["exit_status"] == json!(0)
        })
    };
    let left_out: Vec<&str> = runs
        .iter()
        .map(|v| v["label"].as_str().unwrap())
        .filter(|l| !clean(l))
        .collect();

    // (arm, round) -> curve name -> class medians; and the pinned digests.
    let mut cell: BTreeMap<(String, u64), BTreeMap<String, BTreeMap<u64, f64>>> = BTreeMap::new();
    let mut pinned: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut pinned_identical = true;
    let mut order_bits: BTreeMap<String, u64> = BTreeMap::new();
    for v in &runs {
        let label = v["label"].as_str().unwrap();
        let (arm, round) = label.split_once("-r").expect("label <arm>-r<round>");
        for c in v["curves"].as_array().unwrap() {
            let name = c["curve"].as_str().unwrap().to_string();
            order_bits.insert(name.clone(), c["order_bits"].as_u64().unwrap());
            let d = digests(c);
            match pinned.get(&name) {
                Some(p) => pinned_identical &= *p == d,
                None => {
                    pinned.insert(name.clone(), d);
                }
            }
            if clean(label) {
                cell.entry((arm.to_string(), round.parse().unwrap()))
                    .or_default()
                    .insert(name, class_medians(c));
            }
        }
    }

    let rounds: Vec<u64> = {
        let mut r: Vec<u64> = cell.keys().map(|k| k.1).collect();
        r.sort();
        r.dedup();
        r
    };
    let ratio = |m: &BTreeMap<u64, f64>, z: u64| m[&z] / m[&0];
    let flatness = |m: &BTreeMap<u64, f64>| {
        m.keys()
            .map(|z| (ratio(m, *z) - 1.0).abs())
            .fold(0.0, f64::max)
    };

    let mut delta = 0.0f64;
    let mut curves_out = Vec::new();
    let (mut leak_on_a, mut closed_on_b) = (true, true);
    println!("| curve | round | R = t(224)/t(0): A, A′, B | flatness F: A, A′, B | A/A′ band | cost B/A at full length |");
    println!("|:--|--:|:--|:--|--:|--:|");
    // The A/A band first, over every curve and round, since the verdict on B
    // compares against its maximum.
    let mut per_curve_rounds: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for name in order_bits.keys() {
        for &r in &rounds {
            let get = |arm: &str| cell.get(&(arm.to_string(), r)).and_then(|m| m.get(name));
            let (Some(a), Some(a2), Some(b)) = (get("A"), get("A2"), get("B")) else {
                continue;
            };
            let band = a
                .keys()
                .map(|z| (a2[z] / a[z] - 1.0).abs())
                .fold(0.0, f64::max);
            delta = delta.max(band);
            let cost = b[&0] / a[&0];
            per_curve_rounds
                .entry(name.clone())
                .or_default()
                .push(json!({
                    "round": r,
                    "R": {"A": ratio(a, 224), "A2": ratio(a2, 224), "B": ratio(b, 224)},
                    "F": {"A": flatness(a), "A2": flatness(a2), "B": flatness(b)},
                    "aa_band": band, "cost": cost,
                    "full_length_ms": {"A": a[&0] / 1e6, "B": b[&0] / 1e6},
                }));
        }
    }
    for (name, rows) in &per_curve_rounds {
        for row in rows {
            println!(
                "| {name} | {} | {:.3}, {:.3}, {:.3} | {:.3}, {:.3}, {:.3} | {:.3} | {:.3} |",
                row["round"],
                row["R"]["A"].as_f64().unwrap(),
                row["R"]["A2"].as_f64().unwrap(),
                row["R"]["B"].as_f64().unwrap(),
                row["F"]["A"].as_f64().unwrap(),
                row["F"]["A2"].as_f64().unwrap(),
                row["F"]["B"].as_f64().unwrap(),
                row["aa_band"].as_f64().unwrap(),
                row["cost"].as_f64().unwrap()
            );
            leak_on_a &= row["R"]["A"].as_f64().unwrap() < 0.9;
            closed_on_b &= row["F"]["B"].as_f64().unwrap() <= delta;
        }
        // Per arm and class: the median over rounds of the class median,
        // normalised to the full-length class, with the per-round range.
        let mut profile = BTreeMap::new();
        for arm in ARMS {
            let per_round: Vec<&BTreeMap<u64, f64>> = rounds
                .iter()
                .filter_map(|&r| cell.get(&(arm.to_string(), r)).and_then(|m| m.get(name)))
                .collect();
            let zs: Vec<u64> = per_round[0].keys().copied().collect();
            let classes: Vec<Value> = zs
                .iter()
                .map(|&z| {
                    let rel: Vec<f64> = per_round.iter().map(|m| ratio(m, z)).collect();
                    let ms: Vec<f64> = per_round.iter().map(|m| m[&z] / 1e6).collect();
                    json!({
                        "leading_zeros": z,
                        "relative_median": median(rel.clone()),
                        "relative_min": rel.iter().cloned().fold(f64::INFINITY, f64::min),
                        "relative_max": rel.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                        "ms_median": median(ms),
                    })
                })
                .collect();
            profile.insert(arm, json!({"rounds": per_round.len(), "classes": classes}));
        }
        let costs: Vec<f64> = rows.iter().map(|r| r["cost"].as_f64().unwrap()).collect();
        curves_out.push(json!({
            "curve": name, "order_bits": order_bits[name], "per_round": rows, "profile": profile,
            "cost_median": median(costs.clone()),
            "cost_min": costs.iter().cloned().fold(f64::INFINITY, f64::min),
            "cost_max": costs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            "R_A_max": rows.iter().map(|r| r["R"]["A"].as_f64().unwrap()).fold(f64::NEG_INFINITY, f64::max),
            "F_B_max": rows.iter().map(|r| r["F"]["B"].as_f64().unwrap()).fold(0.0, f64::max),
        }));
    }
    println!();
    println!(
        "A/A band δ = {delta:.4}; leak present on A (R_A < 0.9 every round, both curves): {leak_on_a}; \
         coarse leak closed on B (F_B ≤ δ every round, both curves): {closed_on_b}; \
         pinned digests identical across all runs: {pinned_identical}; runs {}, left out (contended) {:?}",
        runs.len(),
        left_out
    );
    let summary = json!({
        "schema": "scalar-mul-ct-timing-summary/v1",
        "runs": runs.len(), "left_out_contended": left_out, "rounds_complete": rounds.len(),
        "pinned_identical": pinned_identical, "delta": delta,
        "verdicts": {"leak_present_on_A": leak_on_a, "closed_on_B": closed_on_b},
        "curves": curves_out,
    });
    std::fs::write(&out, serde_json::to_string_pretty(&summary).unwrap() + "\n")
        .expect("write summary.json");
    eprintln!("wrote {}", out.display());
}
