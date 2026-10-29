//! Scores the invariant-guess study (`research/dreg_invariant_guess_20261029/`)
//! against its pre-registration: validity V1–V3, then each arm's verdict per
//! draw and the registered hypothesis.
//!
//! ```sh
//! cargo run --release --example dreg_linear_guess_score -- \
//!     research/dreg_invariant_guess_20261029            # the verdicts
//! cargo run --release --example dreg_linear_guess_score -- --plot-data \
//!     research/dreg_invariant_guess_20261029            # JSON for the plot
//! ```
//!
//! Fail-closed: a registered run that is missing, unparsable, mislabelled
//! (its embedded draw or guesses differ from its name) or fails its exactness
//! check makes the study invalid, and an invalid study exits 1.

use serde_json::Value;
use std::path::Path;

const DRAWS: [u64; 4] = [0, 3, 5, 6];
const ARMS: [&str; 2] = ["S3", "X3"];

fn load(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(text.lines().find(|l| !l.trim().is_empty())?).ok()
}

/// The guesses branch `b` of `arm` registers, as the tool records them:
/// (`fixed` as `[v, b]`, `sums` as `[j, i, c]`).
fn registered(arm: &str, b: u64) -> (Value, Value) {
    let (c, f, g) = (b & 1, (b >> 1) & 1, (b >> 2) & 1);
    match arm {
        "S3" => (
            serde_json::json!([[23, f], [24, g]]),
            serde_json::json!([[15, 7, c]]),
        ),
        "X3" => (
            serde_json::json!([[7, c], [23, f], [24, g]]),
            serde_json::json!([]),
        ),
        _ => unreachable!(),
    }
}

/// The record matches its registered identity and passed its exactness check.
fn admissible(v: &Value, draw: u64, fixed: &Value, sums: &Value, sat: bool) -> bool {
    v["draw"].as_u64() == Some(draw)
        && v["n"] == 22
        && v["ell"] == 8
        && &v["fixed"] == fixed
        && &v["sums"] == sums
        && (v["committed_outcome"]["kind"] == "satisfiable") == sat
        && v["identity_check"] == true
}

/// The degree-4 entry, if built.
fn degree4(v: &Value) -> Option<&Value> {
    v["scan"]
        .as_array()?
        .iter()
        .find(|e| e["degree"].as_u64() == Some(4))
}

/// A branch outcome: refutes at ≤ 4, or saturates degree 4 without refuting.
/// Anything else (not built, unsaturated) is no reading.
#[derive(Clone, Copy, PartialEq)]
enum Branch {
    Refutes,
    Fails,
    NoReading,
}

fn branch(v: &Value) -> Branch {
    if v["closure_degree"].as_u64().is_some_and(|d| d <= 4) {
        Branch::Refutes
    } else if degree4(v).is_some_and(|e| e["saturated"] == true && e["closure_refuted"] == false) {
        Branch::Fails
    } else {
        Branch::NoReading
    }
}

fn strip_secs(scan: &Value) -> Value {
    let mut s = scan.clone();
    if let Some(a) = s.as_array_mut() {
        for e in a {
            if let Some(o) = e.as_object_mut() {
                o.remove("secs");
            }
        }
    }
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let plot = args.iter().any(|a| a == "--plot-data");
    let dir = Path::new(
        args.iter()
            .find(|a| !a.starts_with("--"))
            .expect("study directory"),
    );
    let runs = dir.join("runs");
    let mut valid = true;
    let mut fail = |why: String| {
        eprintln!("INVALID: {why}");
        valid = false;
    };

    // V2a: the instrument identity control.
    match (
        load(&runs.join("N.d0.jsonl")),
        load(&dir.join("reference/m3-22-8.d0.G3b0.jsonl")),
    ) {
        (Some(n), Some(r)) => {
            let fixed = serde_json::json!([[7, 0], [15, 0], [23, 0]]);
            if !admissible(&n, 0, &fixed, &serde_json::json!([]), false) {
                fail("N.d0: identity or exactness check fails".into());
            }
            if strip_secs(&n["scan"]) != strip_secs(&r["scan"]) || n["fixed"] != r["fixed"] {
                fail("V2: N.d0 differs from the committed G3 branch-0 record".into());
            }
        }
        _ => fail("N.d0 or its reference missing".into()),
    }
    // V2b: the positive control.
    let mut p_refutes = 0;
    for d in DRAWS {
        let fixed = serde_json::json!([[7, 0], [15, 0], [23, 0], [24, 0]]);
        match load(&runs.join(format!("P.d{d}.jsonl"))) {
            Some(v) => {
                if !admissible(&v, d, &fixed, &serde_json::json!([]), false) {
                    fail(format!("P.d{d}: identity or exactness check fails"));
                }
                if branch(&v) == Branch::Refutes {
                    p_refutes += 1;
                }
            }
            None => fail(format!("P.d{d} missing")),
        }
    }
    if p_refutes < 3 {
        fail(format!("V2: positive control refutes on {p_refutes} of 4"));
    }

    // The arms.
    let mut cells = Vec::new();
    let mut suffices = [0usize; 2];
    for (ai, arm) in ARMS.iter().enumerate() {
        for d in DRAWS {
            let (mut refuted, mut failed, mut unrun) = (0, 0, 0);
            let mut first_fail: Option<(u64, Value)> = None;
            for b in 0..8u64 {
                let name = format!("{arm}.d{d}.b{b}");
                let Some(v) = load(&runs.join(format!("{name}.jsonl"))) else {
                    unrun += 1;
                    continue;
                };
                let (fixed, sums) = registered(arm, b);
                if !admissible(&v, d, &fixed, &sums, false) {
                    fail(format!("{name}: identity or exactness check fails"));
                }
                match branch(&v) {
                    Branch::Refutes => refuted += 1,
                    Branch::Fails => {
                        failed += 1;
                        first_fail.get_or_insert((b, degree4(&v).unwrap()["closure_rank"].clone()));
                    }
                    Branch::NoReading => fail(format!("{name}: no degree-4 reading")),
                }
            }
            // The early-stop rule: unrun branches are allowed only after a
            // failing branch.
            let verdict = if refuted == 8 {
                suffices[ai] += 1;
                "suffices"
            } else if failed > 0 {
                "does not suffice"
            } else {
                fail(format!(
                    "{arm}.d{d}: incomplete ({unrun} branches unrun, none failing)"
                ));
                "incomplete"
            };
            if !plot {
                println!(
                    "{arm} draw {d}: {refuted} refute, {failed} fail, {unrun} unrun -> {verdict}{}",
                    first_fail.as_ref().map_or(String::new(), |(b, r)| format!(
                        " (first failing branch b{b}, degree-4 closure rank {r})"
                    ))
                );
            }
            cells.push(format!(
                r#"{{"arm":"{arm}","draw":{d},"refuted":{refuted},"failed":{failed},"unrun":{unrun},"verdict":"{verdict}"}}"#
            ));
        }
    }

    // V3: soundness on the satisfiable draw.
    let mut sat_not_refuting = 0;
    for b in 0..8u64 {
        let name = format!("S3.sat.d1.b{b}");
        match load(&runs.join(format!("{name}.jsonl"))) {
            Some(v) => {
                let (fixed, sums) = registered("S3", b);
                if !admissible(&v, 1, &fixed, &sums, true) {
                    fail(format!("{name}: identity or exactness check fails"));
                }
                if branch(&v) != Branch::Refutes {
                    sat_not_refuting += 1;
                }
            }
            None => fail(format!("{name} missing")),
        }
    }
    if sat_not_refuting == 0 {
        fail("V3: every S3 branch refutes the satisfiable draw 1".into());
    }

    let label = |i: usize| {
        if suffices[i] >= 3 {
            "suffices"
        } else if suffices[i] <= 1 {
            "fails"
        } else {
            "split"
        }
    };
    let (s, x) = (label(0), label(1));
    let hypothesis = match (s, x) {
        ("suffices", "fails") => "H_inv",
        ("fails", "fails") => "H_dim",
        ("suffices", "suffices") => "H_both",
        ("fails", "suffices") => "H_rev",
        _ => "mixed",
    };
    if plot {
        println!(
            r#"{{"valid":{valid},"hypothesis":"{hypothesis}","positive_refutes":{p_refutes},"sat_not_refuting":{sat_not_refuting},"cells":[{}]}}"#,
            cells.join(",")
        );
    } else {
        println!(
            "S3 suffices on {} of 4 ({s}); X3 suffices on {} of 4 ({x}) -> {hypothesis}",
            suffices[0], suffices[1]
        );
        println!(
            "controls: positive refutes on {p_refutes} of 4; satisfiable draw 1: {sat_not_refuting} of 8 S3 branches do not refute"
        );
        println!(
            "validity (V1 exactness, V2 identity and positive control, V3 soundness, completeness): {}",
            if valid { "valid" } else { "INVALID" }
        );
    }
    if !valid {
        std::process::exit(1);
    }
}
