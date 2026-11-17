//! Which of the ten guesses decide degree-3 refutation at m = 4
//! (`research/dreg_m4_guess_bits_20261106/`).
//!
//! Phase 1 reruns the m = 4 per-link study's 2 × 8 × 1024 branches
//! (`runs/I.d*.jsonl`, `runs/R.d*.jsonl`; line `b` is branch `b`, bit `i` of
//! `b` the value of guess `i`). For each arm and draw, refutation is a Boolean
//! function `f` of the ten guess bits. This tool reads it exactly and reports:
//!
//! - the essential set `E`: the guesses `i` with `f(b) ≠ f(b ^ 2^i)` for some
//!   `b`;
//! - for each guess, the refuting branches with that bit 1 and with it 0.
//!
//! Phase 2 guesses only `E` (the other guesses are dropped, so their variables
//! stay free) on all `2^|E|` assignments (`runs/P2.<arm>.d<draw>.jsonl`, line
//! `e`: bit `j` of `e` is the value of the `j`-th member of `E`, ascending),
//! and compares each assignment's refutation with `f` on the same values.
//!
//! ```sh
//! dreg_m4_guess_bits analyse research    # E per arm and draw, JSON lines (phase 2 input)
//! dreg_m4_guess_bits score research      # phase-1 and phase-2 readings
//! dreg_m4_guess_bits --plot-data score research
//! ```
//!
//! Fail-closed: a missing or mislabelled record makes the study invalid, and an
//! invalid study exits 1. Phase-1 guesses are validated in full by
//! `dreg_m4_link_score --study`.

use serde_json::{json, Value};
use std::path::Path;

const STUDY: &str = "dreg_m4_guess_bits_20261106";
const DRAWS_DIR: &str = "dreg_closure_driver_20261016/draws";
const DRAWS: [(u64, &str); 8] = [
    (24, "m4-12-5.jsonl"),
    (26, "m4-12-5.jsonl"),
    (47, "m4-12-5.jsonl"),
    (52, "m4-12-5.jsonl"),
    (8, "m4-11-5.jsonl"),
    (67, "m4-11-5.jsonl"),
    (107, "m4-11-5.jsonl"),
    (143, "m4-11-5.jsonl"),
];
const BRANCHES: usize = 1024;
/// Arm R's guessed variable for guess `i`.
const RAW_VARS: [u64; 10] = [9, 8, 7, 6, 14, 13, 12, 19, 18, 17];
/// Arm I's pair sums for guesses 0..3: `x_j := x_s + c`.
const SUMS: [(u64, u64); 4] = [(9, 4), (8, 3), (7, 2), (6, 1)];
/// Arm I's link guesses 4..9: (link, topI).
const LINKS: [(u64, usize); 6] = [(2, 1), (2, 2), (2, 3), (3, 1), (3, 2), (3, 3)];

fn draw_record(root: &Path, file: &str, draw: u64) -> Option<(u64, u64, Vec<u64>)> {
    let text = std::fs::read_to_string(root.join(DRAWS_DIR).join(file)).ok()?;
    let v = text
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .find(|v| v["draw"].as_u64() == Some(draw) && v["outcome"]["kind"] != "satisfiable")?;
    let basis = v["v_basis"]
        .as_array()?
        .iter()
        .map(|w| w.as_u64())
        .collect::<Option<Vec<_>>>()?;
    Some((v["n"].as_u64()?, v["ell"].as_u64()?, basis))
}

/// The expected (`fixed`, `sums`, `link`) of a record that guesses the
/// members of `set` (ascending guess indices) with the given values.
fn expected(
    arm: &str,
    set: &[usize],
    vals: &[u64],
    rec: &(u64, u64, Vec<u64>),
) -> (Value, Value, Option<Value>) {
    let (n, ell, basis) = rec;
    if arm == "R" {
        let fixed: Vec<Value> = set
            .iter()
            .zip(vals)
            .map(|(&i, &c)| json!([RAW_VARS[i], c]))
            .collect();
        return (json!(fixed), json!([]), None);
    }
    let t_of = |k: u64| -> Vec<u64> {
        (0..*ell)
            .filter(|&t| basis[t as usize] >> k & 1 == 1)
            .collect()
    };
    let eligible: Vec<u64> = (0..*n).rev().filter(|&k| t_of(k).len() >= 2).collect();
    let mut sums = Vec::new();
    let mut link = Vec::new();
    for (&i, &c) in set.iter().zip(vals) {
        if i < 4 {
            let (j, s) = SUMS[i];
            sums.push(json!([j, s, c]));
        }
    }
    for (&i, &c) in set.iter().zip(vals) {
        if i >= 4 {
            let (j, top) = LINKS[i - 4];
            let Some(&k) = eligible.get(top - 1) else {
                return (json!(null), json!(null), None);
            };
            let target = 4 * ell + (j - 2) * n + k;
            let sources: Vec<u64> = t_of(k).iter().map(|t| j * ell + t).collect();
            sums.push(json!([target, sources, c]));
            link.push(json!([j, k, c]));
        }
    }
    (
        json!([]),
        json!(sums),
        (!link.is_empty()).then(|| json!(link)),
    )
}

/// Refutes at degree 3 (`Some(true)`), saturates there without refuting
/// (`Some(false)`), or carries no degree-3 reading (`None`).
fn reading(v: &Value) -> Option<bool> {
    let scan3 = v["scan"]
        .as_array()
        .and_then(|s| s.iter().find(|e| e["degree"] == 3));
    match (v["closure_degree"].as_u64(), scan3) {
        (Some(3), _) => Some(true),
        (None, Some(e)) if e["closure_refuted"] == false => Some(false),
        _ => None,
    }
}

fn lines(path: &Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

/// Phase 1: f over 1024 branches, or errors.
fn phase1(runs: &Path, arm: &str, draw: u64, errors: &mut Vec<String>) -> Vec<bool> {
    let name = format!("{arm}.d{draw}");
    let ls = lines(&runs.join(format!("{name}.jsonl")));
    if ls.len() != BRANCHES {
        errors.push(format!("{name}: {} of {BRANCHES} branches", ls.len()));
    }
    ls.iter()
        .take(BRANCHES)
        .enumerate()
        .map(|(b, l)| {
            let v: Value = serde_json::from_str(l).unwrap_or(Value::Null);
            if v["draw"].as_u64() != Some(draw) || v["identity_check"] != true {
                errors.push(format!("{name}.b{b}: identity or exactness check fails"));
            }
            reading(&v).unwrap_or_else(|| {
                errors.push(format!("{name}.b{b}: no degree-3 reading"));
                false
            })
        })
        .collect()
}

fn essential(f: &[bool]) -> Vec<usize> {
    (0..10)
        .filter(|&i| (0..f.len()).any(|b| f.get(b ^ (1 << i)).is_some_and(|&g| g != f[b])))
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let plot = args.iter().any(|a| a == "--plot-data");
    let mode = args
        .iter()
        .find(|a| *a == "analyse" || *a == "score")
        .expect("analyse | score")
        .clone();
    let root = Path::new(
        args.iter()
            .rev()
            .find(|a| !a.starts_with("--") && **a != mode)
            .expect("research directory"),
    );
    let runs = root.join(STUDY).join("runs");
    let mut errors = Vec::new();
    let mut out = Vec::new();
    let mut table = Vec::new();
    let (mut kept_all, mut tested, mut sum_up, mut sum_down) = (0u32, 0u32, 0u32, 0u32);
    // Q2: essential guesses by link (1: guesses 0-3, 2: 4-6, 3: 7-9), per arm I, R.
    let mut by_link = [[0u32; 3]; 2];
    for (draw, file) in DRAWS {
        let Some(rec) = draw_record(root, file, draw) else {
            errors.push(format!("draw {draw}: not found"));
            continue;
        };
        for arm in ["I", "R"] {
            let f = phase1(&runs, arm, draw, &mut errors);
            if f.len() != BRANCHES {
                continue;
            }
            let e = essential(&f);
            for &i in &e {
                by_link[usize::from(arm == "R")][match i {
                    0..=3 => 0,
                    4..=6 => 1,
                    _ => 2,
                }] += 1;
            }
            let refuting = f.iter().filter(|&&x| x).count();
            // Refuting branches with guess i set to 1 and to 0.
            let per_bit: Vec<(usize, usize)> = (0..10)
                .map(|i| {
                    let ones = (0..BRANCHES).filter(|b| b >> i & 1 == 1 && f[*b]).count();
                    (ones, refuting - ones)
                })
                .collect();
            if arm == "I" {
                for &i in e.iter().filter(|&&i| i < 4) {
                    let (one, zero) = per_bit[i];
                    sum_up += u32::from(one > zero);
                    sum_down += u32::from(one < zero);
                }
            }
            if mode == "analyse" {
                out.push(json!({"arm": arm, "draw": draw, "essential": e}).to_string());
                continue;
            }
            // Phase 2.
            let mut p2 = Value::Null;
            if !e.is_empty() && e.len() < 10 {
                tested += 1;
                let name = format!("P2.{arm}.d{draw}");
                let ls = lines(&runs.join(format!("{name}.jsonl")));
                let want = 1usize << e.len();
                if ls.len() != want {
                    errors.push(format!("{name}: {} of {want} assignments", ls.len()));
                }
                let (mut agree, mut kept, mut lost, mut gained, mut red_ref) = (0, 0, 0, 0, 0);
                for (x, l) in ls.iter().take(want).enumerate() {
                    let v: Value = serde_json::from_str(l).unwrap_or(Value::Null);
                    let vals: Vec<u64> = (0..e.len()).map(|j| (x >> j & 1) as u64).collect();
                    let (fixed, sums, link) = expected(arm, &e, &vals, &rec);
                    if v["draw"].as_u64() != Some(draw)
                        || v["identity_check"] != true
                        || v["fixed"] != fixed
                        || v["sums"] != sums
                        || v.get("link") != link.as_ref()
                    {
                        errors.push(format!("{name}.e{x}: identity or exactness check fails"));
                    }
                    let b: usize = e.iter().enumerate().map(|(j, &i)| (x >> j & 1) << i).sum();
                    let Some(r) = reading(&v) else {
                        errors.push(format!("{name}.e{x}: no degree-3 reading"));
                        continue;
                    };
                    red_ref += usize::from(r);
                    match (f[b], r) {
                        (true, true) => {
                            agree += 1;
                            kept += 1
                        }
                        (false, false) => agree += 1,
                        (true, false) => lost += 1,
                        (false, true) => gained += 1,
                    }
                }
                let all_kept = lost == 0 && kept > 0;
                kept_all += u32::from(all_kept);
                p2 = json!({"assignments": want, "reduced_refuting": red_ref, "agree": agree, "kept": kept, "lost": lost, "gained": gained, "all_kept": all_kept});
            }
            table.push(format!(
                "{arm} draw {draw}: refuting {refuting}/1024, |E| = {}, E = {e:?}{}",
                e.len(),
                if p2.is_null() {
                    String::new()
                } else {
                    format!(
                        "; E only: {}/{} refute, kept {} lost {} gained {}",
                        p2["reduced_refuting"],
                        p2["assignments"],
                        p2["kept"],
                        p2["lost"],
                        p2["gained"]
                    )
                }
            ));
            out.push(
                json!({"arm": arm, "draw": draw, "n": rec.0, "refuting": refuting, "essential": e,
                       "per_bit": per_bit, "phase2": p2})
                .to_string(),
            );
        }
    }
    let valid = errors.is_empty();
    for e in errors.iter().take(40) {
        eprintln!("INVALID: {e}");
    }
    if mode == "analyse" {
        for l in &out {
            println!("{l}");
        }
    } else if plot {
        println!(
            r#"{{"valid":{valid},"phase2_tested":{tested},"phase2_all_kept":{kept_all},"pair_sum_essential_up":{sum_up},"pair_sum_essential_down":{sum_down},"by_link_i":{:?},"by_link_r":{:?},"cells":[{}]}}"#,
            by_link[0],
            by_link[1],
            out.join(",")
        );
    } else {
        for l in &table {
            println!("{l}");
        }
        println!(
            "Q3: arm I's essential pair-sum guesses refute more at value 1 on {sum_up}, at value 0 on {sum_down}"
        );
        println!(
            "Q2: essential guesses on links 1, 2, 3: arm I {:?}, arm R {:?}",
            by_link[0], by_link[1]
        );
        println!(
            "Q4: guessing only E keeps every refutation on {kept_all} of {tested} arm-draws tested"
        );
        println!("validity: {}", if valid { "valid" } else { "INVALID" });
    }
    if !valid {
        std::process::exit(1);
    }
}
