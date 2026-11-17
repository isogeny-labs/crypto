//! Scores the second-link invariant-guess study
//! (`research/dreg_second_link_guess_20261105/`): arm L3 (`a₇+b₇`,
//! `(e₁+x₃)₂₀`, `e₁[0]`) against the committed S3 (`a₇+b₇`, `c₇`, `e₁[0]`) on
//! 12 draws, with the instrument identity control and a two-sided exact sign
//! test on discordant draws. Each L3 record's second-link sources are checked
//! against the draw's basis (`T₂₀ = {t : v_t has bit 20}`).
//!
//! ```sh
//! cargo run --release --example dreg_second_link_score -- research   # verdicts
//! cargo run --release --example dreg_second_link_score -- --plot-data research
//! ```
//!
//! The argument is the directory holding the three studies. Fail-closed: a
//! missing, mislabelled or inexact record, or an undecided draw, makes the
//! study invalid, and an invalid study exits 1.

use serde_json::{json, Value};
use std::path::Path;

const STUDY: &str = "dreg_second_link_guess_20261105";
const FIRST: &str = "dreg_invariant_guess_20261029";
const MORE: &str = "dreg_invariant_guess_more_20261029";
const DRAWS: [u64; 12] = [0, 3, 5, 6, 8, 11, 14, 15, 17, 24, 25, 27];

fn load(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(text.lines().find(|l| !l.trim().is_empty())?).ok()
}

/// The registered guesses of branch `b` (`fixed`, `sums`); `sources` are the
/// draw's second-link source variables `16 + t` for `t ∈ T₂₀`.
fn guesses(arm: &str, b: u64, sources: &[u64]) -> (Value, Value) {
    let (c, f, g) = (b & 1, (b >> 1) & 1, (b >> 2) & 1);
    match arm {
        "L3" => (json!([[24, g]]), json!([[15, 7, c], [44, sources, f]])),
        "S3" => (json!([[23, f], [24, g]]), json!([[15, 7, c]])),
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

/// The second-link source variables of `draw`: `16 + t` for each basis vector
/// `v_t` with bit 20 set, read from the committed draw file.
fn second_link_sources(root: &Path, draw: u64) -> Vec<u64> {
    let file = if draw <= 6 {
        root.join(FIRST).join("draws/m3-22-8.jsonl")
    } else {
        root.join(MORE).join("draws/m3-22-8-more.jsonl")
    };
    let text = std::fs::read_to_string(file).unwrap_or_default();
    text.lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .find(|v| v["draw"].as_u64() == Some(draw))
        .and_then(|v| {
            v["v_basis"].as_array().map(|b| {
                b.iter()
                    .enumerate()
                    .filter(|(_, w)| w.as_u64().is_some_and(|w| w >> 20 & 1 == 1))
                    .map(|(t, _)| 16 + t as u64)
                    .collect()
            })
        })
        .unwrap_or_default()
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

/// (suffices?, refuting, failing, unrun, first failing branch).
fn decide(
    runs: &Path,
    arm: &str,
    draw: u64,
    sources: &[u64],
    errors: &mut Vec<String>,
) -> (Option<bool>, u32, u32, u32, Option<u64>) {
    let (mut r, mut f, mut u, mut first) = (0, 0, 0, None);
    for b in 0..8u64 {
        let name = format!("{arm}.d{draw}.b{b}");
        let Some(v) = load(&runs.join(format!("{name}.jsonl"))) else {
            u += 1;
            continue;
        };
        let (fixed, sums) = guesses(arm, b, sources);
        if !admissible(&v, draw, &fixed, &sums) {
            errors.push(format!("{name}: identity or exactness check fails"));
        }
        if refutes(&v) {
            r += 1;
        } else if fails(&v) {
            f += 1;
            first.get_or_insert(b);
        } else {
            errors.push(format!("{name}: no degree-4 reading"));
        }
    }
    let verdict = if r == 8 {
        Some(true)
    } else if f > 0 {
        Some(false)
    } else {
        None
    };
    (verdict, r, f, u, first)
}

/// `P(Bin(d, 1/2) ≥ k)`, exactly.
fn tail(k: u32, d: u32) -> f64 {
    let (mut total, mut binom) = (0f64, 1f64);
    for i in 0..=d {
        if i >= k {
            total += binom;
        }
        binom = binom * (d - i) as f64 / (i + 1) as f64;
    }
    total / 2f64.powi(d as i32)
}

fn strip_secs(scan: &Value) -> Value {
    let mut s = scan.clone();
    if let Some(a) = s.as_array_mut() {
        for e in a.iter_mut().filter_map(Value::as_object_mut) {
            e.remove("secs");
        }
    }
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let plot = args.iter().any(|a| a == "--plot-data");
    let root = Path::new(
        args.iter()
            .find(|a| !a.starts_with("--"))
            .expect("research directory"),
    );
    let runs = root.join(STUDY).join("runs");
    let mut errors = Vec::new();

    // V2: the instrument identity control.
    match (
        load(&runs.join("N.d5.jsonl")),
        load(&root.join(FIRST).join("runs/S3.d5.b0.jsonl")),
    ) {
        (Some(n), Some(c)) => {
            let (fixed, sums) = guesses("S3", 0, &[]);
            if !admissible(&n, 5, &fixed, &sums) {
                errors.push("N.d5: identity or exactness check fails".into());
            }
            if strip_secs(&n["scan"]) != strip_secs(&c["scan"]) {
                errors.push("V2: N.d5 differs from the committed S3.d5.b0".into());
            }
        }
        _ => errors.push("N.d5 or the committed S3.d5.b0 missing".into()),
    }

    let (mut kd, mut ks, mut d_suff, mut s_suff) = (0u32, 0u32, 0, 0);
    let mut cells = Vec::new();
    let mut lines = Vec::new();
    for draw in DRAWS {
        let s_runs = root.join(if draw <= 6 { FIRST } else { MORE }).join("runs");
        let sources = second_link_sources(root, draw);
        if sources.is_empty() {
            errors.push(format!("draw {draw}: no basis vector has bit 20"));
        }
        let d = decide(&runs, "L3", draw, &sources, &mut errors);
        let s = decide(&s_runs, "S3", draw, &[], &mut errors);
        for (arm, x) in [("L3", &d), ("S3", &s)] {
            if x.0.is_none() {
                errors.push(format!("{arm}.d{draw}: undecided"));
            }
            cells.push(format!(
                r#"{{"arm":"{arm}","draw":{draw},"refuted":{},"failed":{},"unrun":{},"suffices":{}}}"#,
                x.1,
                x.2,
                x.3,
                x.0.map_or("null".to_string(), |v| v.to_string())
            ));
        }
        match (d.0, s.0) {
            (Some(true), Some(false)) => kd += 1,
            (Some(false), Some(true)) => ks += 1,
            _ => {}
        }
        d_suff += u32::from(d.0 == Some(true));
        s_suff += u32::from(s.0 == Some(true));
        let show = |x: &(Option<bool>, u32, u32, u32, Option<u64>)| match x.0 {
            Some(true) => "suffices".to_string(),
            Some(false) => format!("fails at b{}", x.4.unwrap_or(9)),
            None => "undecided".to_string(),
        };
        lines.push(format!("draw {draw}: L3 {} | S3 {}", show(&d), show(&s)));
    }
    let n = kd + ks;
    let p = (2.0 * tail(kd.max(ks), n)).min(1.0);
    let verdict = if n > 0 && p <= 0.05 && kd > ks {
        "L3_better"
    } else if n > 0 && p <= 0.05 && ks > kd {
        "S3_better"
    } else {
        "no detected difference"
    };
    let valid = errors.is_empty();
    for e in &errors {
        eprintln!("INVALID: {e}");
    }
    if plot {
        println!(
            r#"{{"valid":{valid},"verdict":"{verdict}","l3_only":{kd},"s3_only":{ks},"p_two_sided":{p:.4},"l3_suffices":{d_suff},"s3_suffices":{s_suff},"cells":[{}]}}"#,
            cells.join(",")
        );
    } else {
        for l in &lines {
            println!("{l}");
        }
        println!(
            "L3 suffices on {d_suff} of 12, S3 on {s_suff}; discordant {n} (L3-only {kd}, S3-only {ks}); two-sided sign test p = {p:.4} -> {verdict}"
        );
        println!(
            "validity (identity control, identity and exactness, every draw decided): {}",
            if valid { "valid" } else { "INVALID" }
        );
    }
    if !valid {
        std::process::exit(1);
    }
}
