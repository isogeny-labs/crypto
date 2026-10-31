//! Scores the invariant-guess extension
//! (`research/dreg_invariant_guess_more_20261029/`) against its
//! pre-registration: V0 replay, V1 identity and exactness, V2 positive control,
//! then the per-draw verdicts and the one-sided exact sign test on discordant
//! draws. The secondary count adds the 4 draws of
//! `research/dreg_invariant_guess_20261029/`.
//!
//! ```sh
//! cargo run --release --example dreg_linear_guess_more_score -- \
//!     research/dreg_invariant_guess_more_20261029 research/dreg_invariant_guess_20261029
//! cargo run --release --example dreg_linear_guess_more_score -- --plot-data \
//!     research/dreg_invariant_guess_more_20261029 research/dreg_invariant_guess_20261029
//! ```
//!
//! Fail-closed: a missing, mislabelled or inexact registered run, or an
//! undecided (arm, draw), makes the study invalid, and an invalid study exits 1.

use serde_json::Value;
use std::path::Path;

fn lines(path: &Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

fn load(path: &Path) -> Option<Value> {
    lines(path).into_iter().next()
}

/// The guesses branch `b` of `arm` registers (`fixed`, `sums`).
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

fn admissible(v: &Value, draw: u64, fixed: &Value, sums: &Value) -> bool {
    v["draw"].as_u64() == Some(draw)
        && v["n"] == 22
        && v["ell"] == 8
        && &v["fixed"] == fixed
        && &v["sums"] == sums
        && v["committed_outcome"]["kind"] != "satisfiable"
        && v["identity_check"] == true
}

fn refutes(v: &Value) -> bool {
    v["closure_degree"].as_u64().is_some_and(|d| d <= 4)
}

fn fails(v: &Value) -> bool {
    v["scan"].as_array().is_some_and(|s| {
        s.iter()
            .any(|e| e["degree"] == 4 && e["saturated"] == true && e["closure_refuted"] == false)
    })
}

/// `Some(true)` suffices, `Some(false)` fails, `None` undecided; with the
/// counts of refuting, failing and unrun branches.
fn decide(runs: &Path, arm: &str, draw: u64, errors: &mut Vec<String>) -> (Option<bool>, [u32; 3]) {
    let mut counts = [0u32; 3];
    for b in 0..8u64 {
        let name = format!("{arm}.d{draw}.b{b}");
        let Some(v) = load(&runs.join(format!("{name}.jsonl"))) else {
            counts[2] += 1;
            continue;
        };
        let (fixed, sums) = registered(arm, b);
        if !admissible(&v, draw, &fixed, &sums) {
            errors.push(format!("{name}: identity or exactness check fails"));
        }
        if refutes(&v) {
            counts[0] += 1;
        } else if fails(&v) {
            counts[1] += 1;
        } else {
            errors.push(format!("{name}: no degree-4 reading"));
        }
    }
    let verdict = if counts[0] == 8 {
        Some(true)
    } else if counts[1] > 0 {
        Some(false)
    } else {
        None
    };
    (verdict, counts)
}

/// `P(Bin(d, 1/2) ≥ k)`, exactly.
fn sign_p(k: u32, d: u32) -> f64 {
    let mut total = 0f64;
    let mut binom = 1f64;
    for i in 0..=d {
        if i >= k {
            total += binom;
        }
        binom = binom * (d - i) as f64 / (i + 1) as f64;
    }
    total / 2f64.powi(d as i32)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let plot = args.iter().any(|a| a == "--plot-data");
    let dirs: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    let (dir, first) = (
        Path::new(dirs.first().expect("study directory")),
        Path::new(dirs.get(1).expect("first study directory")),
    );
    let runs = dir.join("runs");
    let mut errors: Vec<String> = Vec::new();

    // V0: the replay of draws 0–6 equals the committed draws.
    let replay = lines(&dir.join("draws/replay-0-6.jsonl"));
    let committed = lines(&first.join("draws/m3-22-8.jsonl"));
    let key = |v: &Value| {
        (
            v["draw"].clone(),
            v["v_basis"].clone(),
            v["x_r"].clone(),
            v["solutions"].clone(),
            v["outcome"].clone(),
        )
    };
    if replay.len() != 7
        || committed.len() != 7
        || replay.iter().zip(&committed).any(|(a, b)| key(a) != key(b))
    {
        errors.push("V0: replay of draws 0-6 differs from the committed draws".into());
    }

    // The registered set: the first 8 unsatisfiable draws from index 7.
    let draws_file = lines(&dir.join("draws/m3-22-8-more.jsonl"));
    let set: Vec<u64> = draws_file
        .iter()
        .filter(|v| v["draw"].as_u64().is_some_and(|d| d >= 7))
        .filter(|v| v["outcome"]["kind"] != "satisfiable" && v["solutions"] == 0)
        .filter_map(|v| v["draw"].as_u64())
        .take(8)
        .collect();
    if set.len() < 8 {
        errors.push(format!("only {} registered draws", set.len()));
    }

    // V2: the positive control.
    let mut p_refutes = 0;
    for &d in &set {
        let name = format!("P.d{d}");
        match load(&runs.join(format!("{name}.jsonl"))) {
            Some(v) => {
                let fixed = serde_json::json!([[7, 0], [15, 0], [23, 0], [24, 0]]);
                if !admissible(&v, d, &fixed, &serde_json::json!([])) {
                    errors.push(format!("{name}: identity or exactness check fails"));
                }
                if refutes(&v) {
                    p_refutes += 1;
                }
            }
            None => errors.push(format!("{name} missing")),
        }
    }
    if p_refutes < 6 {
        errors.push(format!("V2: positive control refutes on {p_refutes} of 8"));
    }

    // The arms on the new draws, then on the first study's four.
    let mut cells = Vec::new();
    let mut tally = |study: &str,
                     runs: &Path,
                     draws: &[u64],
                     errors: &mut Vec<String>|
     -> (u32, u32, u32, u32) {
        let (mut s_only, mut x_only, mut s_suff, mut x_suff) = (0, 0, 0, 0);
        for &d in draws {
            let (s, sc) = decide(runs, "S3", d, errors);
            let (x, xc) = decide(runs, "X3", d, errors);
            for (arm, verdict, c) in [("S3", s, sc), ("X3", x, xc)] {
                if verdict.is_none() {
                    errors.push(format!("{study} {arm}.d{d}: undecided"));
                }
                cells.push(format!(
                    r#"{{"study":"{study}","arm":"{arm}","draw":{d},"refuted":{},"failed":{},"unrun":{},"suffices":{}}}"#,
                    c[0],
                    c[1],
                    c[2],
                    verdict.map_or("null".to_string(), |v| v.to_string())
                ));
            }
            match (s, x) {
                (Some(true), Some(false)) => s_only += 1,
                (Some(false), Some(true)) => x_only += 1,
                _ => {}
            }
            s_suff += u32::from(s == Some(true));
            x_suff += u32::from(x == Some(true));
        }
        (s_only, x_only, s_suff, x_suff)
    };
    let (s_only, x_only, s_suff, x_suff) = tally("new", &runs, &set, &mut errors);
    let (s1, x1, s1s, x1s) = tally("first", &first.join("runs"), &[0, 3, 5, 6], &mut errors);

    let d = s_only + x_only;
    let (p_s, p_x) = (sign_p(s_only, d), sign_p(x_only, d));
    let verdict = if d > 0 && p_s <= 0.05 {
        "S3_better"
    } else if d > 0 && p_x <= 0.05 {
        "X3_better"
    } else {
        "no detected difference"
    };
    let (dc, sc) = (d + s1 + x1, s_only + s1);
    let p_comb = sign_p(sc, dc);
    let valid = errors.is_empty();
    for e in &errors {
        eprintln!("INVALID: {e}");
    }
    if plot {
        println!(
            r#"{{"valid":{valid},"verdict":"{verdict}","registered":{set:?},"s_only":{s_only},"x_only":{x_only},"p_s3":{p_s:.4},"p_x3":{p_x:.4},"combined_s_only":{sc},"combined_discordant":{dc},"combined_p_s3":{p_comb:.4},"cells":[{}]}}"#,
            cells.join(",")
        );
    } else {
        println!("registered draws: {set:?}");
        println!(
            "new draws: S3 suffices on {s_suff} of {}, X3 on {x_suff}; discordant {d} (S3-only {s_only}, X3-only {x_only}); one-sided sign test p(S3) = {p_s:.4}, p(X3) = {p_x:.4} -> {verdict}",
            set.len()
        );
        println!(
            "secondary, with the first study's 4 draws (S3 {s1s}, X3 {x1s} there): S3-only {sc} of {dc} discordant, p(S3) = {p_comb:.4}"
        );
        println!("positive control refutes on {p_refutes} of {}", set.len());
        println!(
            "validity (V0 replay, V1 identity and exactness, V2 positive control, every draw decided): {}",
            if valid { "valid" } else { "INVALID" }
        );
    }
    if !valid {
        std::process::exit(1);
    }
}
