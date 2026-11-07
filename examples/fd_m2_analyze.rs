//! Frozen analysis of the fall-degree data request
//! (`research/fd_paper_data_request_20261029/`): reads the stage records
//! `fd_m2_degrees` wrote and applies the protocols' decision rules, with no
//! judgment beyond them.
//!
//! ```sh
//! cargo run --release --example fd_m2_analyze -- research/fd_paper_data_request_20261029
//! ```
//!
//! Writes one JSON document to stdout:
//!
//! * `c0`: every `runs/calibrate/<name>.jsonl` against the recorded
//!   outcome in `inputs/instances_m2_recorded.json` (last fall degree, and
//!   the degree-4 closure rank where the record has `W_dims` at 4);
//!   `pass` only if every recorded instance has a row and every row agrees.
//! * `aa`: the `aa-*` reruns against their 16-thread rows (closure ranks).
//! * `negative`: planted rows (`"planted":true`); any refutation fails.
//! * `closure_d4` / `closure_d5`: per instance, the last fall degree and
//!   the verdict (`exact`, `closure_only` for a `>=` bound, `censored`).
//! * `p1`: every random-subspace instance at n = 41..51 has `1 ∉ W_4`.
//! * `p2`: some instance has `1 ∉ W_5`.
//! * `f4`: per instance `D_reg`, `D_reg - d_F` where `d_F` is known, and
//!   per `n` the polynomial-subspace `D_reg` set against Kousidis–Wiemers.
//!
//! A verdict is only computed from rows that exist; a missing stage reads
//! `"not_run"`.  Unless C0 passes, the A/A reruns agree and no planted
//! control refutes, every new-row verdict reads `invalid_measurement`; the
//! F4 comparison needs C0 and C1.  P1 holds only on the full planned panel
//! (one exact `d_F <= 4` refutes it) and P2 fails only when all 8 panel
//! instances are exactly 5; otherwise they read `undecided`.  Replaced
//! (satisfiable) draws are diagnostics and enter no verdict.

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::path::Path;

/// Kousidis and Wiemers 2019, Table 1, m = 2: D_reg for n = 34..48.
const KW_DREG: [(u32, u32); 15] = [
    (34, 4),
    (35, 4),
    (36, 4),
    (37, 5),
    (38, 5),
    (39, 5),
    (40, 5),
    (41, 5),
    (42, 5),
    (43, 5),
    (44, 5),
    (45, 5),
    (46, 5),
    (47, 5),
    (48, 5),
];

fn rows(dir: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    let mut paths: Vec<_> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
        .filter(|p| p.file_stem().is_some_and(|x| x != "executions"))
        .collect();
    paths.sort();
    for p in paths {
        let file = p.file_stem().unwrap().to_string_lossy().to_string();
        for line in std::fs::read_to_string(&p).unwrap_or_default().lines() {
            if let Ok(mut v) = serde_json::from_str::<Value>(line) {
                v["_file"] = json!(file);
                out.push(v);
            }
        }
    }
    out
}

/// A drawn or recorded instance with a closure measurement: not a planted
/// control and not a replaced (satisfiable) draw's diagnostic line.
fn measured(v: &Value) -> bool {
    v["planted"] != true && v["replaced"] != true && v.get("closure").is_some()
}

/// The 8 instances of claim C2 (d_F >= 5 certified at n = 33..37).
const D5_PANEL: [&str; 8] = [
    "T33a", "T33b", "R35a", "R35b", "U35a", "U35b", "R37a", "R37b",
];
/// Planned random-subspace instances of P1: 3 per n = 41, 43, ..., 51.
const P1_PLANNED: usize = 18;

/// (exact degree, lower bound) of a row's closure.
fn lfd(v: &Value) -> (Option<u64>, Option<u64>) {
    let l = &v["closure"]["last_fall_degree"];
    (l["exact"].as_u64(), l["at_least"].as_u64())
}

fn rank_at(v: &Value, d: u64) -> Option<u64> {
    v["closure"]["scan"]
        .as_array()?
        .iter()
        .find(|s| s["degree"].as_u64() == Some(d))
        .and_then(|s| s["closure_rank"].as_u64())
}

fn verdict(v: &Value) -> &'static str {
    match lfd(v) {
        (Some(_), _) => "exact",
        (None, Some(_)) => "closure_only",
        _ => "censored",
    }
}

fn main() {
    let root = std::env::args().nth(1).expect("study directory");
    let root = Path::new(&root);
    let recorded: Vec<Value> = serde_json::from_str(
        &std::fs::read_to_string(root.join("inputs/instances_m2_recorded.json"))
            .expect("recorded instances"),
    )
    .expect("json");
    let rec: BTreeMap<String, &Value> = recorded
        .iter()
        .filter(|e| e.get("t").is_none())
        .map(|e| (e["name"].as_str().unwrap().to_string(), e))
        .collect();
    let mut out = Map::new();

    // C0 and the A/A.
    let calib = rows(&root.join("runs/calibrate"));
    let (mut c0_rows, mut c0_ok, mut seen) = (Vec::new(), true, BTreeMap::new());
    let (mut aa_rows, mut aa_ok) = (Vec::new(), true);
    let by_name: BTreeMap<String, &Value> = calib
        .iter()
        .filter(|v| !v["_file"].as_str().unwrap().starts_with("aa-"))
        .map(|v| (v["name"].as_str().unwrap().to_string(), v))
        .collect();
    for v in &calib {
        let name = v["name"].as_str().unwrap().to_string();
        if v["_file"].as_str().unwrap().starts_with("aa-") {
            let base = by_name.get(&name);
            let same = base.is_some_and(|b| {
                (3..=4).all(|d| rank_at(b, d) == rank_at(v, d)) && lfd(b) == lfd(v)
            });
            aa_ok &= same;
            aa_rows.push(json!({"name": name, "agrees": same}));
            continue;
        }
        let Some(r) = rec.get(&name) else { continue };
        seen.insert(name.clone(), ());
        let (exact, at_least) = lfd(v);
        let (r_exact, r_lb) = (r["d_ref_exact"].as_u64(), r["d_ref_lower_bound"].as_u64());
        let degree_ok = match (r_exact, r_lb) {
            (Some(e), _) => exact == Some(e),
            (None, Some(lb)) => exact.is_none() && at_least.is_some_and(|a| a >= lb),
            _ => false,
        };
        let rec_rank4 = r["closures"]["4"]["W_dims"]
            .as_array()
            .and_then(|a| a.last())
            .and_then(Value::as_u64);
        let rank_ok = match (rec_rank4, rank_at(v, 4)) {
            (Some(a), Some(b)) => r["closures"]["4"]["W_one"] == 1 || a == b,
            _ => true,
        };
        let ok = degree_ok && rank_ok && v["solutions"].as_u64() == Some(0);
        c0_ok &= ok;
        c0_rows.push(json!({
            "name": name, "native": v["closure"]["last_fall_degree"],
            "recorded_exact": r_exact, "recorded_at_least": r_lb,
            "native_rank4": rank_at(v, 4), "recorded_rank4": rec_rank4, "agrees": ok
        }));
    }
    let missing: Vec<&String> = rec.keys().filter(|k| !seen.contains_key(*k)).collect();
    let c0_pass = !calib.is_empty() && c0_ok && missing.is_empty();
    out.insert(
        "c0".into(),
        json!({"status": if calib.is_empty() { "not_run" } else if c0_pass { "pass" } else { "fail" },
               "rows": c0_rows, "missing": missing}),
    );
    out.insert(
        "aa".into(),
        json!({"status": if aa_rows.is_empty() { "not_run" } else if aa_ok { "pass" } else { "fail" }, "rows": aa_rows}),
    );
    // Closure stages and the negative control.
    let d4 = rows(&root.join("runs/closure-d4"));
    let neg: Vec<&Value> = d4.iter().filter(|v| v["planted"] == true).collect();
    let neg_fail = neg.iter().any(|v| lfd(v).0.is_some());
    // Every substantive verdict needs the whole instrument check: C0, the
    // A/A reruns (run in the calibrate stage) and no refuted planted control.
    let instrument_ok = c0_pass && !aa_rows.is_empty() && aa_ok && !neg_fail;
    let gate = |s: &str| {
        if instrument_ok {
            s.to_string()
        } else {
            "invalid_measurement".to_string()
        }
    };
    out.insert(
        "instrument".into(),
        json!({"c0_pass": c0_pass, "aa_pass": !aa_rows.is_empty() && aa_ok, "negative_refuted": neg_fail, "admitted": instrument_ok}),
    );
    out.insert(
        "negative".into(),
        json!({"status": if neg.is_empty() { "not_run".into() } else if neg_fail { "fail".to_string() } else { gate("pass") },
               "rows": neg.iter().map(|v| json!({"name": v["name"], "n": v["n"], "solutions": v["solutions"], "lfd": v["closure"]["last_fall_degree"]})).collect::<Vec<_>>()}),
    );
    let instance_rows = |vs: &[Value]| -> Vec<Value> {
        vs.iter()
            .filter(|v| measured(v))
            .map(|v| json!({"name": v["name"], "n": v["n"], "k": v["k"], "family": v["family"],
                             "lfd": v["closure"]["last_fall_degree"], "verdict": gate(verdict(v)),
                             "columns": v["closure"]["scan"].as_array().and_then(|s| s.last()).map(|s| s["columns"].clone()),
                             "secs": v["closure"]["scan"].as_array().map(|s| s.iter().map(|x| x["secs"].as_f64().unwrap_or(0.0)).sum::<f64>())}))
            .collect()
    };
    out.insert("closure_d4".into(), json!(instance_rows(&d4)));
    let p1_rows: Vec<&Value> = d4
        .iter()
        .filter(|v| measured(v) && v["family"] == "R")
        .filter(|v| (41..=51).contains(&v["n"].as_u64().unwrap_or(0)))
        .collect();
    // One exact d_F <= 4 refutes P1; holding needs the whole planned panel.
    let p1 = if p1_rows.is_empty() {
        "not_run".to_string()
    } else if p1_rows.iter().any(|v| lfd(v).0.is_some_and(|e| e <= 4)) {
        gate("fails")
    } else if p1_rows.len() == P1_PLANNED
        && p1_rows.iter().all(|v| lfd(v).1.is_some_and(|a| a >= 5))
    {
        gate("holds")
    } else {
        gate("undecided")
    };
    out.insert(
        "p1".into(),
        json!({"status": p1, "instances": p1_rows.len()}),
    );
    let d5 = rows(&root.join("runs/closure-d5"));
    out.insert("closure_d5".into(), json!(instance_rows(&d5)));
    // One d_F >= 6 establishes P2; failing needs every panel instance at exactly 5.
    let d5_of = |name: &str| d5.iter().find(|v| measured(v) && v["name"] == name);
    let p2 = if d5.is_empty() {
        "not_run".to_string()
    } else if d5
        .iter()
        .any(|v| measured(v) && lfd(v).1.is_some_and(|a| a >= 6))
    {
        gate("holds")
    } else if D5_PANEL
        .iter()
        .all(|n| d5_of(n).is_some_and(|v| lfd(v).0 == Some(5)))
    {
        gate("fails")
    } else {
        gate("undecided")
    };
    out.insert(
        "p2".into(),
        json!({"status": p2, "instances": d5.iter().filter(|v| measured(v)).count(), "panel": D5_PANEL}),
    );

    // F4.
    let f4 = rows(&root.join("runs/f4"));
    let mut f4_rows = Vec::new();
    let mut by_n: BTreeMap<u64, Vec<(u64, bool)>> = BTreeMap::new();
    let mut c1_ok = true;
    for v in &f4 {
        let Some(f) = v.get("f4") else { continue };
        let censored = f["timed_out"] == true || f["oversize"] == true;
        let dreg = f["degree_reached"].as_u64();
        let name = v["name"].as_str().unwrap_or("").to_string();
        let df = rec.get(&name).and_then(|r| r["d_ref_exact"].as_u64());
        if let (Some(d), Some(e), false) = (dreg, df, censored) {
            c1_ok &= d >= e;
        }
        if v["family"] == "P" || v["family"] == "poly" {
            if let Some(d) = dreg {
                by_n.entry(v["n"].as_u64().unwrap())
                    .or_default()
                    .push((d, censored));
            }
        }
        f4_rows.push(json!({"name": name, "n": v["n"], "family": v["family"], "d_reg": dreg,
                            "censored": censored, "d_f": df, "overshoot": dreg.zip(df).map(|(a, b)| a as i64 - b as i64)}));
    }
    // The F4 comparison shares the build C0 checks, and C1 (D_reg >= d_F)
    // checks the engine against the closure; either failing voids it.
    let f4_ok = c0_pass && c1_ok;
    let kw: Vec<Value> = KW_DREG
        .iter()
        .map(|&(n, k)| {
            let ours = by_n.get(&(n as u64)).cloned().unwrap_or_default();
            let status = if ours.is_empty() {
                "not_run".to_string()
            } else if ours.iter().any(|&(_, c)| c) {
                "censored".to_string()
            } else if !f4_ok {
                "invalid_measurement".to_string()
            } else if ours.iter().all(|&(d, _)| d == k as u64) {
                "reproduced".to_string()
            } else {
                "differs".to_string()
            };
            json!({"n": n, "kw_d_reg": k, "ours": ours.iter().map(|&(d, c)| json!({"d_reg": d, "censored": c})).collect::<Vec<_>>(), "status": status})
        })
        .collect();
    out.insert(
        "f4".into(),
        json!({"c1": if f4.is_empty() { "not_run" } else if c1_ok { "pass" } else { "fail" },
               "rows": f4_rows, "kousidis_wiemers": kw}),
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&Value::Object(out)).unwrap()
    );
}
