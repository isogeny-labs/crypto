//! The instruction audit (`docs/units/BITOPS.md`, "What the unit leaves
//! out") of `research/ic_acct1_cover_bitop_20261021`'s two arms, which are
//! custom examples rather than ecbench sessions, so `ecbench audit-ir` cannot
//! replay them.
//!
//! A row is re-executed by `examples/jv_cover_ic_vs_rho_bitops.rs` under
//! `valgrind --tool=callgrind` with `--callgrind 1`: the arm is bracketed by
//! `jv_arm_begin` / `jv_arm_end` dumps and split at every phase change into
//! parts labelled `jv_phase_<phase>` (`jv_bitops::set_callgrind_phases`).
//! This program reads those parts (`ecbench::callgrind::read_parts`), sums
//! the arm's `Ir` by phase, divides by the row's `bitop/v1` total and per
//! phase, and checks the re-executed row's counts against the committed row
//! of the same instance (every primitive count of every phase and the
//! total): the reproduction BITOPS.md requires before a count is a bound.
//!
//! ```sh
//! target/release/examples/jv_cover_audit_ir --committed raw/ic_1009.jsonl \
//!     --row rerun.jsonl --callgrind cg/ic.callgrind [--native native.jsonl]
//! ```
//!
//! One JSON object on stdout (`ic-acct1.audit_ir/v1`).  `--row` without
//! `--callgrind` checks reproduction only (a native re-execution).

use std::collections::BTreeMap;
use std::path::PathBuf;

use crypto_lib::cryptanalysis::ecbench::callgrind;
use serde_json::{json, Value};

fn read_rows(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| panic!("{path}: {e}")))
        .collect()
}

/// The instance a row ran: arm, p, seed, moves and, for rho, variant and run.
fn key(r: &Value) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        r["arm"], r["p"], r["seed"], r["moves"], r["variant"], r["run"]
    )
}

/// Every `(phase, primitive) → count` of a row, plus its RNG words.
fn counts(r: &Value) -> BTreeMap<String, u64> {
    let mut out = BTreeMap::new();
    if let Some(ph) = r["phases"].as_object() {
        for (name, t) in ph {
            if let Some(c) = t["counts"].as_object() {
                for (prim, v) in c {
                    out.insert(format!("{name}.{prim}"), v.as_u64().unwrap_or(0));
                }
            }
            out.insert(
                format!("{name}.rng_words"),
                t["rng_words"].as_u64().unwrap_or(0),
            );
        }
    }
    out
}

fn bitops(v: &Value) -> u128 {
    v.as_str()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("bitops {v}"))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut committed: Vec<String> = Vec::new();
    let mut row_path = None;
    let mut prefix: Option<PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        let v = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--committed" => committed.push(v),
            "--row" => row_path = Some(v),
            "--callgrind" => prefix = Some(PathBuf::from(v)),
            other => panic!("unknown argument {other}"),
        }
        i += 2;
    }
    let rows = read_rows(&row_path.expect("--row"));
    assert_eq!(rows.len(), 1, "one re-executed row per audit");
    let row = &rows[0];
    let reference: Vec<Value> = committed.iter().flat_map(|p| read_rows(p)).collect();
    let k = key(row);
    let matches: Vec<&Value> = reference.iter().filter(|r| key(r) == k).collect();
    assert_eq!(matches.len(), 1, "exactly one committed row for {k}");
    let old = matches[0];
    let (a, b) = (counts(row), counts(old));
    let mismatches: Vec<Value> = a
        .keys()
        .chain(b.keys())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .filter(|name| a.get(*name) != b.get(*name))
        .map(|name| json!({"count": name, "rerun": a.get(name), "committed": b.get(name)}))
        .collect();
    let total = bitops(&row["total_bitops"]);
    let committed_total = bitops(&old["total_bitops"]);
    let mut out = json!({
        "schema": "ic-acct1.audit_ir/v1",
        "arm": row["arm"],
        "variant": row["variant"],
        "p": row["p"],
        "seed": row["seed"],
        "moves": row["moves"],
        "run": row["run"],
        "l": row["l"],
        "total_bitops": total.to_string(),
        "committed_total_bitops": committed_total.to_string(),
        "reproduces_total": total == committed_total,
        "reproduces_counts": mismatches.is_empty(),
        "count_mismatches": mismatches,
        "verified": row.get("verified_on_challenge").cloned().unwrap_or(Value::Null),
        "wall_ms": row["wall_ms"],
    });
    if let Some(prefix) = prefix {
        let parts = callgrind::read_parts(&prefix).expect("callgrind parts");
        let begin: Vec<usize> = parts
            .iter()
            .enumerate()
            .filter_map(|(i, p)| (p.trigger == "jv_arm_begin").then_some(i))
            .collect();
        let end: Vec<usize> = parts
            .iter()
            .enumerate()
            .filter_map(|(i, p)| (p.trigger == "jv_arm_end").then_some(i))
            .collect();
        assert!(
            begin.len() == 1 && end.len() == 1 && begin[0] < end[0],
            "expected one ordered jv_arm_begin/jv_arm_end pair, found {begin:?}/{end:?}"
        );
        // A part is the interval its dump closes: `jv_phase_<x>` closes
        // phase x; `jv_arm_end` closes the phase the arm started in.
        let outer = row["outer_phase"].as_str().unwrap_or("outer").to_string();
        let mut by_phase: BTreeMap<String, u64> = BTreeMap::new();
        for p in &parts[begin[0] + 1..=end[0]] {
            let name = match p.trigger.strip_prefix("jv_phase_") {
                Some(n) => n.to_string(),
                None if p.trigger == "jv_arm_end" => outer.clone(),
                None => format!("other:{}", p.trigger),
            };
            *by_phase.entry(name).or_default() += p.ir;
        }
        let arm_ir: u64 = by_phase.values().sum();
        let phase_bitops = |name: &str| -> Option<u128> {
            // The IC row's phase names; rho is one interval (its phases are
            // priced from native counts after the walk, not entered).
            row["phases"][name]["bitops"]
                .as_str()
                .and_then(|s| s.parse().ok())
        };
        let phases: Vec<Value> = by_phase
            .iter()
            .map(|(name, ir)| {
                let g = phase_bitops(name);
                json!({
                    "phase": name,
                    "ir": ir,
                    "bitops": g.map(|g| g.to_string()),
                    "ir_per_kilo_bitop": g.filter(|g| *g > 0).map(|g| *ir as f64 / g as f64 * 1e3),
                })
            })
            .collect();
        out["callgrind"] = json!({
            "unit": "callgrind.Ir",
            "prework_ir": parts[..=begin[0]].iter().map(|p| p.ir).sum::<u64>(),
            "arm_ir": arm_ir,
            "post_ir": parts[end[0] + 1..].iter().map(|p| p.ir).sum::<u64>(),
            "parts": end[0] - begin[0],
            "ir_per_bitop": arm_ir as f64 / total as f64,
            "ir_per_kilo_bitop": arm_ir as f64 / total as f64 * 1e3,
            "phases": phases,
        });
    }
    println!("{out}");
}
