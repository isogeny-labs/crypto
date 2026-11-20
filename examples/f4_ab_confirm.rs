//! Same-binary interleaved A/B of the F4 engines' defaults against their
//! old paths, with an A/A control, for one built pair of benchmarks.
//!
//! ```text
//! cargo run --release --example f4_ab_confirm -- OUT_DIR BLOCKS F4_FP_BENCH KOBLITZ_F4_KERNEL_BENCH
//! ```
//!
//! Arms: `new` runs the binaries with every F4 toggle unset; `old` sets
//! `F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1 KIC_M4RI_OLD=1`, which restores the code
//! paths before the 2026-11-16 speed leads (research/f4_groebner_speed_leads_20261116).
//! Cells: `f4_fp_bench 3 large` (pair DEFAULT and pair AA) and
//! `koblitz_f4_kernel_bench 5 x` (pair M4RI and pair AA), each at
//! RAYON_NUM_THREADS 1 and 4.  Every block visits every cell once, the cell
//! order rotated by the block, with the two arms in ABBA order on even
//! (block + cell) and BAAB on odd, so each block gives two (old, new) pairs
//! per cell.  Inherited `F4_FP_*`, `KIC_*` and `RAYON_NUM_THREADS` variables
//! are removed from every child.
//!
//! Per (cell, case): medians of old and new, the ratio median(old) /
//! median(new) (>1: new faster), and a percentile bootstrap interval of that
//! ratio over the (old, new) pairs ([`bootstrap_ci`], 4,000 resamples).  A
//! row is significant when its interval excludes 1 and |ln ratio| exceeds the
//! A/A spread of the same case and thread count (the larger |ln| endpoint of
//! the A/A interval).  Every output fingerprint (basis, solve, field
//! operations; rank and word operations) must be the same in every run of a
//! case across all arms, or the program exits 1.
//!
//! Overrides (unset, the panel above): `F4_AB_NEW` and `F4_AB_OLD` set the two
//! arms' settings as `K=V,K=V` (`F4_AB_OLD` replaces the old paths above);
//! `F4_AB_TIER` picks the `f4_fp_bench` tier (`large`, `xl`); `F4_AB_THREADS`
//! the thread counts (`1,4`); `F4_AB_KOBLITZ=0` drops the GF(2) cells.
//!
//! Writes `OUT_DIR/runs.jsonl` (one line per child run) and
//! `OUT_DIR/summary.json`, and prints the table.

use crypto_lib::cryptanalysis::ecbench::stats::{bootstrap_ci, median};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::io::Write;
use std::process::Command;
use std::time::Instant;

const OLD_ENV: &[(&str, &str)] = &[
    ("F4_FP_PAR_OLD", "1"),
    ("F4_FP_MEM_OLD", "1"),
    ("KIC_M4RI_OLD", "1"),
];
const RESAMPLES: usize = 4000;

/// `K=V,K=V` pairs from `var`, or `default` when it is unset.
fn env_pairs(var: &str, default: &[(&str, &str)]) -> Vec<(String, String)> {
    match std::env::var(var) {
        Ok(s) => s
            .split(',')
            .filter(|kv| !kv.is_empty())
            .map(|kv| {
                let (k, v) = kv.split_once('=').expect("K=V");
                (k.to_string(), v.to_string())
            })
            .collect(),
        Err(_) => default.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
    }
}

/// The old arm's settings: `F4_AB_OLD`, else [`OLD_ENV`].
fn old_env() -> &'static [(String, String)] {
    static E: std::sync::OnceLock<Vec<(String, String)>> = std::sync::OnceLock::new();
    E.get_or_init(|| env_pairs("F4_AB_OLD", OLD_ENV))
}

/// The new arm's settings: `F4_AB_NEW`, else none.
fn new_env() -> &'static [(String, String)] {
    static E: std::sync::OnceLock<Vec<(String, String)>> = std::sync::OnceLock::new();
    E.get_or_init(|| env_pairs("F4_AB_NEW", &[]))
}

#[derive(Clone)]
struct Cell {
    bench: &'static str,
    bin: String,
    args: Vec<String>,
    pair: &'static str,
    threads: usize,
}

/// (case → measured value in ms, case → fingerprint) of one child run.
type Outcome = (BTreeMap<String, f64>, BTreeMap<String, String>);

fn run_child(cell: &Cell, old: bool) -> (f64, i32, Outcome) {
    let mut cmd = Command::new(&cell.bin);
    cmd.args(&cell.args);
    for (k, _) in std::env::vars() {
        if k.starts_with("F4_FP_") || k.starts_with("KIC_") || k == "RAYON_NUM_THREADS" {
            cmd.env_remove(&k);
        }
    }
    cmd.env("RAYON_NUM_THREADS", cell.threads.to_string());
    for (k, v) in if old { old_env() } else { new_env() } {
        cmd.env(k, v);
    }
    let t0 = Instant::now();
    let out = cmd.output().expect("spawn benchmark");
    let wall = t0.elapsed().as_secs_f64() * 1e3;
    let code = out.status.code().unwrap_or(-1);
    let mut vals = BTreeMap::new();
    let mut fps = BTreeMap::new();
    let mut sum_f4 = 0.0;
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if let Some(case) = v.get("case").and_then(Value::as_str) {
            // f4_fp_bench: one line per system
            if let Some(ms) = v.get("f4_ms").and_then(Value::as_f64) {
                vals.insert(case.to_string(), ms);
                sum_f4 += ms;
            }
            if let Some(ms) = v.get("solve_ms").and_then(Value::as_f64) {
                vals.insert(format!("{case}:solve"), ms);
            }
            let fp = ["basis_fp", "solve_fp", "field_ops", "solve_field_ops", "steps", "max_rows", "max_cols"]
                .iter()
                .map(|k| v.get(*k).map_or("-".to_string(), |x| x.to_string()))
                .collect::<Vec<_>>()
                .join("/");
            fps.insert(case.to_string(), fp);
        } else if let Some(sys) = v.get("system").and_then(Value::as_str) {
            // koblitz_f4_kernel_bench: one summary line per system
            let case = format!("kernel_{sys}");
            if let Some(ms) = v.get("median_ms").and_then(Value::as_f64) {
                vals.insert(case.clone(), ms);
            }
            let fp = ["rows", "cols", "rank", "word_ops"]
                .iter()
                .map(|k| v.get(*k).map_or("-".to_string(), |x| x.to_string()))
                .collect::<Vec<_>>()
                .join("/");
            fps.insert(case, fp);
        }
    }
    if sum_f4 > 0.0 {
        vals.insert("SUM_f4_ms".to_string(), sum_f4);
    }
    vals.insert("process_wall".to_string(), wall);
    (wall, code, (vals, fps))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 5 {
        eprintln!("usage: f4_ab_confirm OUT_DIR BLOCKS F4_FP_BENCH KOBLITZ_F4_KERNEL_BENCH");
        std::process::exit(2);
    }
    let out_dir = std::path::PathBuf::from(&a[1]);
    let blocks: usize = a[2].parse().expect("BLOCKS");
    std::fs::create_dir_all(&out_dir).expect("out dir");
    let mut cells = Vec::new();
    // `F4_AB_TIER` (f4_fp_bench tier, default large), `F4_AB_THREADS` (default
    // 1,4) and `F4_AB_KOBLITZ=0` (skip the GF(2) cells) widen or narrow the panel.
    let tier = std::env::var("F4_AB_TIER").unwrap_or_else(|_| "large".to_string());
    let thread_list: Vec<usize> = std::env::var("F4_AB_THREADS")
        .unwrap_or_else(|_| "1,4".to_string())
        .split(',')
        .map(|t| t.parse().expect("F4_AB_THREADS"))
        .collect();
    let koblitz = std::env::var("F4_AB_KOBLITZ").as_deref() != Ok("0");
    for &threads in &thread_list {
        for pair in ["DEFAULT", "AA"] {
            cells.push(Cell { bench: "f4_fp_bench", bin: a[3].clone(), args: vec!["3".into(), tier.clone()], pair, threads });
        }
        if koblitz {
            for pair in ["M4RI", "AA"] {
                cells.push(Cell { bench: "koblitz_f4_kernel_bench", bin: a[4].clone(), args: vec!["5".into(), "x".into()], pair, threads });
            }
        }
    }
    let mut runs = std::fs::File::create(out_dir.join("runs.jsonl")).expect("runs.jsonl");
    // (cell index) → list of (old, new) outcome pairs, in block order
    let mut paired: Vec<Vec<(Outcome, Outcome)>> = vec![Vec::new(); cells.len()];
    let mut fingerprints: BTreeMap<(String, String), String> = BTreeMap::new();
    let mut mismatches = Vec::new();
    let mut seq = 0usize;
    let mut failures = 0usize;
    for b in 0..blocks {
        for k in 0..cells.len() {
            let ci = (k + b) % cells.len();
            let cell = &cells[ci];
            // arm A is new; arm B is old except in the A/A pair
            let b_old = cell.pair != "AA";
            let order: [bool; 4] = if (b + ci) % 2 == 0 {
                [false, true, true, false] // A B B A
            } else {
                [true, false, false, true] // B A A B
            };
            let mut slot_a = Vec::new();
            let mut slot_b = Vec::new();
            for is_b in order {
                let old = is_b && b_old;
                let (wall, code, outcome) = run_child(cell, old);
                seq += 1;
                if code != 0 {
                    failures += 1;
                }
                for (case, fp) in &outcome.1 {
                    let key = (cell.bench.to_string(), case.clone());
                    match fingerprints.get(&key) {
                        None => {
                            fingerprints.insert(key, fp.clone());
                        }
                        Some(f) if f != fp => mismatches.push(format!(
                            "{} {} {} block {b}: {fp} != {f}",
                            cell.bench, case, if old { "old" } else { "new" }
                        )),
                        _ => {}
                    }
                }
                let rec = json!({
                    "seq": seq, "block": b, "bench": cell.bench, "args": cell.args,
                    "pair": cell.pair, "arm": if is_b { "B" } else { "A" }, "old_env": old,
                    "threads": cell.threads, "exit": code, "wall_ms": wall,
                    "values_ms": outcome.0, "fingerprints": outcome.1,
                });
                writeln!(runs, "{rec}").expect("write run");
                if is_b {
                    slot_b.push(outcome);
                } else {
                    slot_a.push(outcome);
                }
            }
            for (na, ob) in slot_a.into_iter().zip(slot_b) {
                paired[ci].push((ob, na));
            }
        }
        eprintln!("block {}/{} done ({seq} runs)", b + 1, blocks);
    }

    // Per cell and case: ratio of medians with its bootstrap interval.
    let mut rows: Vec<Map<String, Value>> = Vec::new();
    let mut aa_spread: BTreeMap<(String, usize, String), f64> = BTreeMap::new();
    for pass in ["AA", "rest"] {
        for (ci, cell) in cells.iter().enumerate() {
            if (pass == "AA") != (cell.pair == "AA") {
                continue;
            }
            let cases: Vec<String> = paired[ci]
                .first()
                .map(|(o, _)| o.0.keys().cloned().collect())
                .unwrap_or_default();
            for case in cases {
                let pairs: Vec<(f64, f64)> = paired[ci]
                    .iter()
                    .filter_map(|(o, n)| Some((*o.0.get(&case)?, *n.0.get(&case)?)))
                    .collect();
                if pairs.len() < 4 {
                    continue;
                }
                let stat = |s: &[Vec<(f64, f64)>]| {
                    let o: Vec<f64> = s[0].iter().map(|p| p.0).collect();
                    let n: Vec<f64> = s[0].iter().map(|p| p.1).collect();
                    let (mo, mn) = (median(&o)?, median(&n)?);
                    (mn > 0.0).then(|| mo / mn)
                };
                let strata = vec![pairs.clone()];
                let ratio = stat(&strata).unwrap_or(f64::NAN);
                let seed = 0x5eed_f4ab_u64 ^ ((ci as u64) << 32) ^ case.len() as u64;
                let ci95 = bootstrap_ci(&strata, RESAMPLES, seed, stat);
                let key = (cell.bench.to_string(), cell.threads, case.clone());
                let mut significant = false;
                if let Some((lo, hi)) = ci95 {
                    if cell.pair == "AA" {
                        aa_spread.insert(key.clone(), lo.ln().abs().max(hi.ln().abs()));
                    } else {
                        let tau = aa_spread.get(&key).copied().unwrap_or(0.0);
                        significant = (lo > 1.0 || hi < 1.0) && ratio.ln().abs() > tau;
                    }
                }
                let o: Vec<f64> = pairs.iter().map(|p| p.0).collect();
                let n: Vec<f64> = pairs.iter().map(|p| p.1).collect();
                let mut row = Map::new();
                row.insert("bench".into(), json!(cell.bench));
                row.insert("pair".into(), json!(cell.pair));
                row.insert("threads".into(), json!(cell.threads));
                row.insert("case".into(), json!(case));
                row.insert("n_pairs".into(), json!(pairs.len()));
                row.insert("old_median_ms".into(), json!(median(&o)));
                row.insert("new_median_ms".into(), json!(median(&n)));
                row.insert("ratio_old_over_new".into(), json!(ratio));
                row.insert("ci_low".into(), json!(ci95.map(|c| c.0)));
                row.insert("ci_high".into(), json!(ci95.map(|c| c.1)));
                row.insert("aa_spread_ln".into(), json!(aa_spread.get(&key)));
                row.insert("significant".into(), json!(significant));
                rows.push(row);
            }
        }
    }
    let summary = json!({
        "schema": "f4-ab-confirm/v1",
        "blocks": blocks, "runs": seq, "failed_runs": failures,
        "resamples": RESAMPLES, "old_env": old_env(), "new_env": new_env(),
        "tier": tier, "threads": thread_list,
        "fingerprint_mismatches": mismatches, "rows": rows,
    });
    std::fs::write(out_dir.join("summary.json"), serde_json::to_string_pretty(&summary).unwrap())
        .expect("summary.json");
    println!("{:<26} {:<8} {:>3} {:>9} {:>9} {:>7}  {:<18} sig", "case", "pair", "thr", "old_ms", "new_ms", "ratio", "95% CI");
    for r in &rows {
        if r["pair"] == "AA" && r["case"] != "SUM_f4_ms" && r["case"] != "kernel_x" {
            continue;
        }
        println!(
            "{:<26} {:<8} {:>3} {:>9.3} {:>9.3} {:>7.3}  [{:.3}, {:.3}]   {}",
            r["case"].as_str().unwrap_or(""),
            r["pair"].as_str().unwrap_or(""),
            r["threads"],
            r["old_median_ms"].as_f64().unwrap_or(f64::NAN),
            r["new_median_ms"].as_f64().unwrap_or(f64::NAN),
            r["ratio_old_over_new"].as_f64().unwrap_or(f64::NAN),
            r["ci_low"].as_f64().unwrap_or(f64::NAN),
            r["ci_high"].as_f64().unwrap_or(f64::NAN),
            if r["significant"] == true { "*" } else { "" },
        );
    }
    println!("runs {seq}, failed {failures}, fingerprint mismatches {}", mismatches.len());
    if failures > 0 || !mismatches.is_empty() {
        for m in mismatches.iter().take(20) {
            eprintln!("MISMATCH {m}");
        }
        std::process::exit(1);
    }
}
