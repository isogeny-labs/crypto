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

/// The record's embedded identity matches the file it was loaded as: arm,
/// cell `n-ℓ`, draw, and whether the committed draw is satisfiable.
fn identity_ok(v: &Value, arm: &str, cell: &str, draw: u64, sat: bool) -> bool {
    v["arm"].as_str().unwrap_or("B") == arm
        && format!("{}-{}", v["n"], v["ell"]) == cell
        && v["draw"].as_u64() == Some(draw)
        && (v["committed_outcome"]["kind"] == "satisfiable") == sat
}

/// Q or R on one draw: the dense degree-3 run, extended by its sparse
/// degree-4 run where one exists and was not capped. A sparse record whose
/// identity or construction check fails is reported in `errors`, not used.
fn arm_reading(
    dir: &Path,
    arm: &str,
    cell: &str,
    draw: u64,
    errors: &mut Vec<String>,
) -> Option<Reading> {
    let mut r = dense(&load(dir, &format!("{arm}.{cell}.d{draw}"))?);
    if r.degree.is_none() {
        let name = format!("{arm}.{cell}.d{draw}.sparse4");
        if let Some(s) = load(dir, &name) {
            if !identity_ok(&s, arm, cell, draw, false) || s["identity_check"] != true {
                errors.push(format!("{name}: identity or construction check fails"));
            } else if s["capped"] == false {
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

/// The registered dense degree-3 measurement: the entry exists and either
/// refuted or saturated (an unbuilt or unsaturated degree is no reading).
fn measured_at_3(v: &Value) -> bool {
    entry(v, 3).is_some_and(|e| e["closure_refuted"] == true || e["saturated"] == true)
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
        let (mut lower, mut q_lt_r, mut r_le_q, mut unresolved) = (0, 0, 0, 0);
        // Draws where one arm refutes and the other has no reading there.
        let mut mechanism_open = 0;
        for d in unsat {
            let draw = format!("d{d}");
            let Some(b) = load(dir, &format!("B.{cell}.{draw}")) else {
                fail(format!("B.{cell}.{draw} missing"));
                continue;
            };
            if !identity_ok(&b, "B", cell, d, false) {
                fail(format!("B.{cell}.{draw}: embedded identity does not match"));
            }
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
                if !identity_ok(&v, arm, cell, d, false) {
                    fail(format!("{name}: embedded identity does not match"));
                }
                if v["identity_check"] != true {
                    fail(format!("V1: {name} identity check not true"));
                }
                if !measured_at_3(&v) {
                    fail(format!("{name}: no saturated degree-3 measurement"));
                }
                let mut errors = Vec::new();
                let r = arm_reading(dir, arm, cell, d, &mut errors).expect("loaded");
                for e in errors {
                    fail(format!("V1: {e}"));
                }
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
                // Not lower, yet not excluded: Q has no reading at some
                // degree below B's (B refutes above Q's measured degree + 1,
                // or not at all), so Q could still refute below B there.
                if q.degree.is_none() && br.degree.is_none_or(|db| db > q.measured + 1) {
                    unresolved += 1;
                }
                // Compare on measured bounds: an arm with no refutation is
                // known only to refute above its measured degree.
                if let Some(dq) = q.degree {
                    match r.degree {
                        Some(dr) if dq < dr => q_lt_r += 1,
                        None if r.measured >= dq => q_lt_r += 1,
                        None => mechanism_open += 1,
                        _ => {}
                    }
                }
                if let Some(dr) = r.degree {
                    match q.degree {
                        Some(dq) if dr <= dq => r_le_q += 1,
                        None if q.measured >= dr => r_le_q += 1,
                        None => mechanism_open += 1,
                        _ => {}
                    }
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
                        if !identity_ok(&v, arm, cell, *d, true) {
                            fail(format!("{name}: embedded identity does not match"));
                        }
                        let refuted = v["scan"].as_array().is_some_and(|s| {
                            s.iter()
                                .any(|e| e["closure_refuted"] == true || e["xl_refuted"] == true)
                        });
                        let sparse_name = format!("{name}.sparse4");
                        let sparse_record = load(dir, &sparse_name);
                        if sparse_record.as_ref().is_some_and(|s| {
                            !identity_ok(s, arm, cell, *d, true) || s["identity_check"] != true
                        }) {
                            fail(format!(
                                "V1: {sparse_name}: identity or construction check fails"
                            ));
                        }
                        let sparse = sparse_record.is_some_and(|s| s["closure_refuted"] == true);
                        if refuted || sparse {
                            fail(format!("V3: {name} refutes a satisfiable draw"));
                        }
                        if v["identity_check"] != true {
                            fail(format!("V1: {name} identity check not true"));
                        }
                        if !measured_at_3(&v) {
                            fail(format!("{name}: no saturated degree-3 measurement"));
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
        } else if mechanism_open > 0 {
            "unresolved".to_string()
        } else if q_lt_r >= 3 {
            "invariant-specific".to_string()
        } else if r_le_q >= 3 {
            "quadratisation".to_string()
        } else {
            "unresolved".to_string()
        };
        verdicts.push(format!(
            r#"{{"cell":"{cell}","q_lower_than_b":{lower},"verdict":"{verdict}","unresolved_below_b":{unresolved},"mechanism":"{mechanism}"}}"#
        ));
        if !plot {
            println!(
                "cell ({}): D_Q < D_B on {lower} of 4 -> {verdict}; Q unmeasured below D_B on {unresolved} of 4 (not excluded); mechanism: {mechanism}",
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
            "validity (V1 identity, V2 positive control, V3 soundness, degree-3 measured): {}",
            if valid { "valid" } else { "INVALID" }
        );
    }
    // Fail closed for automation too: an invalid study is a failed run.
    if !valid {
        std::process::exit(1);
    }
}
