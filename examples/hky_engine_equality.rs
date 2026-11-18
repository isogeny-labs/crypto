//! Equality gate and timing table for the two HKY closure engines
//! (`research/hky_engine_speed_20261117/README.md`).
//!
//! Both engines are run by `pkm_tower_pilot` on the same cells from the same
//! seeds (`--engine hky` is the reference, `--engine hky2` the candidate);
//! this tool reads the rows they wrote.
//!
//! ```text
//! hky_engine_equality compare REF.jsonl CAND.jsonl
//!     every cell of REF must be in CAND and conversely, with the same
//!     system (curve, tower, x_r, generator degrees) and the same answer:
//!     for every degree c the same columns, dims, `one` and `censored`, and
//!     the same falls, first and last fall degree and censored degree.
//!     Exit 0 if all agree, 1 with one line per difference otherwise.
//! hky_engine_equality compare --expect-mismatch REF.jsonl CAND.jsonl
//!     the negative control: exit 0 only if the comparison fails.
//! hky_engine_equality times LABEL=a.jsonl,b.jsonl,c.jsonl [LABEL=...]
//!     per cell and label: repeats, median / min / max wall milliseconds of
//!     the whole cell, and the counters of the last closure.
//! ```

use std::collections::BTreeMap;
use std::process::ExitCode;

use serde_json::Value;

/// What identifies a cell and its system.
fn cell_key(r: &Value) -> String {
    let f = |k: &str| {
        r.get(k)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".into())
    };
    format!(
        "p={} kind={} m={} control={} t={} target={} k={} g={} null_width={}",
        f("p"),
        f("kind"),
        f("m"),
        f("control"),
        f("t"),
        f("target"),
        f("target_index"),
        f("g"),
        f("null_width"),
    )
}

fn rows(path: &str) -> BTreeMap<String, Value> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut out = BTreeMap::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).unwrap_or_else(|e| panic!("{path}: {e}"));
        if v.get("closures").is_none() {
            continue; // the summary line
        }
        let k = cell_key(&v);
        assert!(
            out.insert(k.clone(), v).is_none(),
            "{path}: cell twice: {k}"
        );
    }
    out
}

/// The answer of a closure: everything the study reads, nothing an
/// instrument is free to change (work counters, time).
fn closure_answer(c: &Value) -> Value {
    serde_json::json!({
        "c": c["c"], "cols": c["cols"], "dims": c["dims"], "one": c["one"],
        "censored": c["censored"],
    })
}

fn compare(reference: &str, candidate: &str) -> Vec<String> {
    let (a, b) = (rows(reference), rows(candidate));
    let mut diffs = Vec::new();
    if a.is_empty() {
        diffs.push(format!("{reference}: no rows"));
    }
    for k in a.keys() {
        if !b.contains_key(k) {
            diffs.push(format!("only in reference: {k}"));
        }
    }
    for k in b.keys() {
        if !a.contains_key(k) {
            diffs.push(format!("only in candidate: {k}"));
        }
    }
    for (k, x) in &a {
        let Some(y) = b.get(k) else { continue };
        // the same system
        for field in [
            "x_r",
            "curve",
            "tower",
            "nf_degrees",
            "n_vars",
            "hky_cmin",
            "hky_cmax",
            "planted_ok",
        ] {
            if x.get(field) != y.get(field) {
                diffs.push(format!(
                    "{k}: system field `{field}` differs: {:?} vs {:?}",
                    x.get(field),
                    y.get(field)
                ));
            }
        }
        // the same answer
        for field in [
            "falls",
            "first_fall_degree",
            "last_fall_degree",
            "censored_at",
            "timed_out",
        ] {
            if x.get(field) != y.get(field) {
                diffs.push(format!(
                    "{k}: `{field}` differs: {:?} vs {:?}",
                    x.get(field),
                    y.get(field)
                ));
            }
        }
        let (ca, cb) = (
            x["closures"].as_array().unwrap(),
            y["closures"].as_array().unwrap(),
        );
        if ca.len() != cb.len() {
            diffs.push(format!("{k}: {} closures vs {}", ca.len(), cb.len()));
        }
        for (p, q) in ca.iter().zip(cb) {
            let (p, q) = (closure_answer(p), closure_answer(q));
            if p != q {
                diffs.push(format!("{k}: closure differs: {p} vs {q}"));
            }
        }
    }
    diffs
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn times(specs: &[String]) {
    println!("| cell | engine | repeats | wall ms median | min | max | last c | cols | dim | one | rows reduced | muladds | stored entries |");
    println!("|:--|:--|--:|--:|--:|--:|--:|--:|--:|:--|--:|--:|--:|");
    // cell -> label -> (ms list, last row)
    let mut table: BTreeMap<(u64, u64, String), BTreeMap<String, (Vec<f64>, Value)>> =
        BTreeMap::new();
    let mut labels = Vec::new();
    for spec in specs {
        let (label, files) = spec.split_once('=').expect("LABEL=file[,file…]");
        labels.push(label.to_string());
        for file in files.split(',') {
            for (k, r) in rows(file) {
                let t = r["t"].as_u64().unwrap_or(0);
                let m = r["m"].as_u64().unwrap_or(0);
                let e = table.entry((m, t, k)).or_default();
                let slot = e
                    .entry(label.to_string())
                    .or_insert_with(|| (Vec::new(), Value::Null));
                slot.0.push(
                    r["ms"]
                        .as_f64()
                        .or_else(|| r["wall_s"].as_f64().map(|s| s * 1e3))
                        .unwrap_or(f64::NAN),
                );
                slot.1 = r;
            }
        }
    }
    for ((m, t, k), per) in &table {
        for label in &labels {
            let Some((ms, r)) = per.get(label) else {
                continue;
            };
            let mut ms = ms.clone();
            let med = median(&mut ms);
            let last = r["closures"]
                .as_array()
                .and_then(|c| c.last())
                .cloned()
                .unwrap_or(Value::Null);
            let short = k
                .split(' ')
                .filter(|s| {
                    s.starts_with("control=") || s.starts_with("k=") || s.starts_with("target=")
                })
                .collect::<Vec<_>>()
                .join(" ")
                .replace('"', "");
            println!(
                "| m={m} t={t} {short} | {label} | {} | {:.0} | {:.0} | {:.0} | {} | {} | {} | {} | {} | {} | {} |",
                ms.len(),
                med,
                ms[0],
                ms[ms.len() - 1],
                last["c"],
                last["cols"],
                last["dims"].as_array().and_then(|d| d.last()).cloned().unwrap_or(Value::Null),
                last["one"],
                last["rows_reduced"],
                last["muladds"],
                last["entries"],
            );
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("compare") => {
            let expect_mismatch = args.iter().any(|a| a == "--expect-mismatch");
            let files: Vec<&String> = args[1..].iter().filter(|a| !a.starts_with("--")).collect();
            assert_eq!(files.len(), 2, "compare REF.jsonl CAND.jsonl");
            let diffs = compare(files[0], files[1]);
            let cells = rows(files[0]).len();
            for d in diffs.iter().take(40) {
                println!("DIFF {d}");
            }
            if expect_mismatch {
                if diffs.is_empty() {
                    println!("NEGATIVE CONTROL FAILED: {cells} cells compared equal, a difference was expected");
                    return ExitCode::from(1);
                }
                println!(
                    "negative control ok: {} differences over {cells} cells (expected)",
                    diffs.len()
                );
                ExitCode::SUCCESS
            } else if diffs.is_empty() {
                println!("EQUAL: {cells} cells, every closure, fall degree and censoring field identical");
                ExitCode::SUCCESS
            } else {
                println!("NOT EQUAL: {} differences over {cells} cells", diffs.len());
                ExitCode::from(1)
            }
        }
        Some("times") => {
            times(&args[1..]);
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: hky_engine_equality compare [--expect-mismatch] REF CAND | times LABEL=f1,f2 …");
            ExitCode::from(2)
        }
    }
}
