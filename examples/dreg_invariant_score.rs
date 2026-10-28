//! Scores the pair-invariant study (`research/dreg_pair_invariant_20261028/`)
//! against its pre-registration: validity V1–V3, then H_lower / H_none /
//! mixed per cell and, where it applies, the mechanism reading.
//!
//! ```sh
//! cargo run --release --example dreg_invariant_score -- \
//!     research/dreg_pair_invariant_20261028/runs            # the verdicts
//! cargo run --release --example dreg_invariant_score -- --plot-data \
//!     research/dreg_pair_invariant_20261028/runs            # JSON for the plot
//! ```
//!
//! Fail-closed: a registered run that is missing, empty or unparsable makes
//! the study invalid rather than being skipped.

use serde_json::Value;
use std::path::Path;

const CELLS: [(&str, [u64; 4], &[u64]); 2] = [
    ("22-6", [0, 1, 2, 3], &[]),
    ("21-7", [0, 2, 3, 6], &[1, 4, 5]),
];

fn load(dir: &Path, name: &str) -> Option<Value> {
    let text = std::fs::read_to_string(dir.join(format!("{name}.jsonl"))).ok()?;
    serde_json::from_str(text.lines().find(|l| !l.trim().is_empty())?).ok()
}

/// The dense scan entry at `degree`, if built.
fn entry(v: &Value, degree: u64) -> Option<&Value> {
    v["scan"]
        .as_array()?
        .iter()
        .find(|e| e["degree"].as_u64() == Some(degree))
}

/// A degree reading: `Some(d)` refutes first at `d`, `None` refutes at no
/// measured degree; `measured` is the highest degree with a reading.
struct Reading {
    degree: Option<u64>,
    measured: u64,
}

fn dense(v: &Value) -> Reading {
    Reading {
        degree: v["closure_degree"].as_u64(),
        measured: v["measured_to"].as_u64().unwrap_or(2),
    }
}

/// Q or R on one draw: the dense degree-3 run, extended by its sparse
/// degree-4 run where one exists and was not capped.
fn arm_reading(dir: &Path, arm: &str, cell: &str, draw: &str) -> Option<Reading> {
    let mut r = dense(&load(dir, &format!("{arm}.{cell}.{draw}"))?);
    if r.degree.is_none() {
        if let Some(s) = load(dir, &format!("{arm}.{cell}.{draw}.sparse4")) {
            if s["capped"] == false {
                r.measured = 4;
                if s["closure_refuted"] == true {
                    r.degree = Some(4);
                }
            }
        }
    }
    Some(r)
}

/// `C(n, ≤ d)`: the columns of the degree-`d` Macaulay matrix in `n` unknowns.
fn monomials_up_to(n: u64, d: u64) -> u64 {
    let (mut total, mut binom) = (0u64, 1u64);
    for i in 0..=d.min(n) {
        total += binom;
        binom = binom * (n - i) / (i + 1);
    }
    total
}

fn show(r: &Reading) -> String {
    r.degree
        .map_or(format!(">{}", r.measured), |d| d.to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let plot = args.iter().any(|a| a == "--plot-data");
    let dir = Path::new(
        args.iter()
            .find(|a| !a.starts_with("--"))
            .expect("runs directory"),
    );
    let mut valid = true;
    let mut fail = |why: String| {
        eprintln!("INVALID: {why}");
        valid = false;
    };
    let mut rows = Vec::new();
    let mut verdicts = Vec::new();
    for (cell, unsat, sat) in CELLS {
        let (mut lower, mut q_lt_r, mut r_le_q) = (0, 0, 0);
        for d in unsat {
            let draw = format!("d{d}");
            let Some(b) = load(dir, &format!("B.{cell}.{draw}")) else {
                fail(format!("B.{cell}.{draw} missing"));
                continue;
            };
            let br = dense(&b);
            // V2: the committed B outcome.
            let b3 = entry(&b, 3).map(|e| e["closure_refuted"] == true);
            let b4 = entry(&b, 4);
            let v2 = match cell {
                "22-6" => b3 == Some(false) && br.degree == Some(4),
                _ => b4.is_some_and(|e| e["closure_refuted"] == false && e["saturated"] == true),
            };
            if !v2 {
                fail(format!(
                    "V2: B.{cell}.{draw} does not reproduce its committed record"
                ));
            }
            let mut readings = Vec::new();
            for arm in ["Q", "R"] {
                let name = format!("{arm}.{cell}.{draw}");
                let Some(v) = load(dir, &name) else {
                    fail(format!("{name} missing"));
                    continue;
                };
                if v["identity_check"] != true {
                    fail(format!("V1: {name} identity check not true"));
                }
                let r = arm_reading(dir, arm, cell, &draw).expect("loaded");
                let e3 = entry(&v, 3);
                rows.push(format!(
                    r#"{{"cell":"{cell}","draw":{d},"arm":"{arm}","n_vars":{},"degree":"{}","rank3":{},"cols3":{},"cols4_needed":{}}}"#,
                    v["n_vars"],
                    show(&r),
                    e3.map_or(Value::Null, |e| e["closure_rank"].clone()),
                    e3.map_or(Value::Null, |e| e["columns"].clone()),
                    monomials_up_to(v["n_vars"].as_u64().unwrap_or(0), 4),
                ));
                readings.push(r);
            }
            let b3e = entry(&b, 3);
            rows.push(format!(
                r#"{{"cell":"{cell}","draw":{d},"arm":"B","n_vars":{},"degree":"{}","rank3":{},"cols3":{},"rank4":{},"cols4":{}}}"#,
                b["n_vars"],
                show(&br),
                b3e.map_or(Value::Null, |e| e["closure_rank"].clone()),
                b3e.map_or(Value::Null, |e| e["columns"].clone()),
                b4.map_or(Value::Null, |e| e["closure_rank"].clone()),
                b4.map_or(Value::Null, |e| e["columns"].clone()),
            ));
            if readings.len() == 2 {
                let (q, r) = (&readings[0], &readings[1]);
                // D_Q < D_B: Q refutes, and B refutes later or not at all.
                if q.degree
                    .is_some_and(|dq| br.degree.is_none_or(|db| dq < db))
                {
                    lower += 1;
                }
                let key = |x: &Reading| x.degree.unwrap_or(u64::MAX);
                if q.degree.is_some() && key(q) < key(r) {
                    q_lt_r += 1;
                }
                if r.degree.is_some() && key(r) <= key(q) {
                    r_le_q += 1;
                }
                eprintln!(
                    "{cell} {draw}: B {} | Q {} | R {}",
                    show(&br),
                    show(q),
                    show(r)
                );
            }
        }
        // V3: no refutation of a satisfiable draw at any measured degree.
        for d in sat {
            for arm in ["Q", "R"] {
                let name = format!("{arm}.{cell}.sat.d{d}");
                match load(dir, &name) {
                    None => fail(format!("{name} missing")),
                    Some(v) => {
                        let refuted = v["scan"].as_array().is_some_and(|s| {
                            s.iter()
                                .any(|e| e["closure_refuted"] == true || e["xl_refuted"] == true)
                        });
                        let sparse = load(dir, &format!("{name}.sparse4"))
                            .is_some_and(|s| s["closure_refuted"] == true);
                        if refuted || sparse {
                            fail(format!("V3: {name} refutes a satisfiable draw"));
                        }
                        if v["identity_check"] != true {
                            fail(format!("V1: {name} identity check not true"));
                        }
                    }
                }
            }
        }
        let verdict = match lower {
            4 => "H_lower",
            0 => "H_none",
            _ => "mixed",
        };
        let mechanism = if lower == 0 {
            "not read (H_none)".to_string()
        } else if q_lt_r >= 3 {
            "invariant-specific".to_string()
        } else if r_le_q >= 3 {
            "quadratisation".to_string()
        } else {
            "unresolved".to_string()
        };
        verdicts.push(format!(
            r#"{{"cell":"{cell}","q_lower_than_b":{lower},"verdict":"{verdict}","mechanism":"{mechanism}"}}"#
        ));
        if !plot {
            println!(
                "cell ({}): D_Q < D_B on {lower} of 4 -> {verdict}; mechanism: {mechanism}",
                cell.replace('-', ", ")
            );
        }
    }
    if plot {
        println!(
            r#"{{"valid":{valid},"verdicts":[{}],"rows":[{}]}}"#,
            verdicts.join(","),
            rows.join(",")
        );
    } else {
        println!(
            "validity (V1 identity, V2 positive control, V3 soundness): {}",
            if valid { "valid" } else { "INVALID" }
        );
    }
}
