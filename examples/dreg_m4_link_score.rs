//! Scores the m = 4 per-link invariant-guess study at degree 3
//! (`research/dreg_m4_link_guess_d3_20261106/`): arm I (four pair sums on
//! `x₁ + x₂`, three guesses on `e₁ + x₃`, three on `e₂ + x₄`) against arm R
//! (ten raw guesses in the same places) on 8 unsatisfiable draws, every one of
//! the 1024 branches per arm and draw. Each record's guesses are re-derived
//! here from its line number and, for arm I's link guesses, from the draw's
//! basis; the instrument identity control is checked; the verdict is a
//! two-sided exact sign test on draws whose refuting counts differ.
//!
//! ```sh
//! cargo run --release --example dreg_m4_link_score -- research   # verdicts
//! cargo run --release --example dreg_m4_link_score -- --plot-data research
//! ```
//!
//! Fail-closed: a missing, mislabelled or inexact record makes the study
//! invalid, and an invalid study exits 1.

use serde_json::{json, Value};
use std::path::Path;

const STUDY: &str = "dreg_m4_link_guess_d3_20261106";
const DRAWS_DIR: &str = "dreg_closure_driver_20261016/draws";
const CONTROL: &str = "dreg_invariant_guess_20261029/runs/S3.d5.b0.jsonl";
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

/// The draw's committed record: (n, ℓ, basis words).
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

/// The registered guesses of `arm` on branch `b`: (`fixed`, `sums`, `link`).
fn expected(arm: &str, b: usize, n: u64, ell: u64, basis: &[u64]) -> (Value, Value, Option<Value>) {
    let c = |i: usize| ((b >> i) & 1) as u64;
    match arm {
        "R" => {
            let vars = [9u64, 8, 7, 6, 14, 13, 12, 19, 18, 17];
            let fixed: Vec<Value> = vars
                .iter()
                .enumerate()
                .map(|(i, &v)| json!([v, c(i)]))
                .collect();
            (json!(fixed), json!([]), None)
        }
        "I" => {
            let mut sums: Vec<Value> = [(9u64, 4u64), (8, 3), (7, 2), (6, 1)]
                .iter()
                .enumerate()
                .map(|(i, &(j, s))| json!([j, s, c(i)]))
                .collect();
            let t_of = |k: u64| -> Vec<u64> {
                (0..ell)
                    .filter(|&t| basis[t as usize] >> k & 1 == 1)
                    .collect()
            };
            let eligible: Vec<u64> = (0..n).rev().filter(|&k| t_of(k).len() >= 2).collect();
            let mut link = Vec::new();
            for (slot, (j, top)) in [(2u64, 1usize), (2, 2), (2, 3), (3, 1), (3, 2), (3, 3)]
                .iter()
                .enumerate()
            {
                let Some(&k) = eligible.get(top - 1) else {
                    return (json!(null), json!(null), None);
                };
                let cc = c(4 + slot);
                let target = 4 * ell + (j - 2) * n + k;
                let sources: Vec<u64> = t_of(k).iter().map(|t| j * ell + t).collect();
                sums.push(json!([target, sources, cc]));
                link.push(json!([j, k, cc]));
            }
            (json!([]), json!(sums), Some(json!(link)))
        }
        _ => unreachable!(),
    }
}

/// (refuting, non-refuting) branch counts; errors are pushed.
fn tally(
    runs: &Path,
    arm: &str,
    draw: u64,
    rec: &(u64, u64, Vec<u64>),
    errors: &mut Vec<String>,
) -> (u32, u32) {
    let name = format!("{arm}.d{draw}");
    let text = std::fs::read_to_string(runs.join(format!("{name}.jsonl"))).unwrap_or_default();
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() != BRANCHES {
        errors.push(format!("{name}: {} of {BRANCHES} branches", lines.len()));
    }
    let (n, ell, basis) = rec;
    let (mut r, mut f) = (0, 0);
    for (b, line) in lines.iter().enumerate().take(BRANCHES) {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            errors.push(format!("{name}.b{b}: not JSON"));
            continue;
        };
        let (fixed, sums, link) = expected(arm, b, *n, *ell, basis);
        let ok = v["draw"].as_u64() == Some(draw)
            && v["m"] == 4
            && v["n"].as_u64() == Some(*n)
            && v["ell"].as_u64() == Some(*ell)
            && v["committed_outcome"]["kind"] != "satisfiable"
            && v["identity_check"] == true
            && v["fixed"] == fixed
            && v["sums"] == sums
            && v.get("link") == link.as_ref();
        if !ok {
            errors.push(format!("{name}.b{b}: identity or exactness check fails"));
        }
        let scan3 = v["scan"]
            .as_array()
            .and_then(|s| s.iter().find(|e| e["degree"] == 3));
        match (v["closure_degree"].as_u64(), scan3) {
            (Some(3), _) => r += 1,
            (None, Some(e)) if e["closure_refuted"] == false => f += 1,
            _ => errors.push(format!("{name}.b{b}: no degree-3 reading")),
        }
    }
    (r, f)
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

fn load_first(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(text.lines().find(|l| !l.trim().is_empty())?).ok()
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
        load_first(&runs.join("N.d5.jsonl")),
        load_first(&root.join(CONTROL)),
    ) {
        (Some(nr), Some(c)) => {
            let same_guesses =
                nr["fixed"] == json!([[23, 0], [24, 0]]) && nr["sums"] == json!([[15, 7, 0]]);
            if !same_guesses || nr["identity_check"] != true || nr["draw"] != 5 {
                errors.push("N.d5: identity or exactness check fails".into());
            }
            if strip_secs(&nr["scan"]) != strip_secs(&c["scan"]) {
                errors.push("V2: N.d5 differs from the committed S3.d5.b0".into());
            }
        }
        _ => errors.push("N.d5 or the committed S3.d5.b0 missing".into()),
    }

    let (mut ki, mut kr) = (0u32, 0u32);
    let (mut any_refutes, mut pool_i, mut pool_r) = (false, 0u32, 0u32);
    let mut cells = Vec::new();
    let mut lines = Vec::new();
    for (draw, file) in DRAWS {
        let Some(rec) = draw_record(root, file, draw) else {
            errors.push(format!("draw {draw}: not found in {file}"));
            continue;
        };
        let i = tally(&runs, "I", draw, &rec, &mut errors);
        let r = tally(&runs, "R", draw, &rec, &mut errors);
        any_refutes |= i.0 > 0 || r.0 > 0;
        pool_i += i.0;
        pool_r += r.0;
        match i.0.cmp(&r.0) {
            std::cmp::Ordering::Greater => ki += 1,
            std::cmp::Ordering::Less => kr += 1,
            std::cmp::Ordering::Equal => {}
        }
        for (arm, x) in [("I", i), ("R", r)] {
            cells.push(format!(
                r#"{{"arm":"{arm}","draw":{draw},"n":{},"refuted":{},"not_refuted":{}}}"#,
                rec.0, x.0, x.1
            ));
        }
        lines.push(format!(
            "draw {draw} (n = {}): I {}/1024 | R {}/1024",
            rec.0, i.0, r.0
        ));
    }
    let d = ki + kr;
    let p = (2.0 * tail(ki.max(kr), d)).min(1.0);
    let verdict = if !any_refutes {
        "no signal"
    } else if d > 0 && p <= 0.05 && ki > kr {
        "I_better"
    } else if d > 0 && p <= 0.05 && kr > ki {
        "R_better"
    } else {
        "no detected difference"
    };
    let valid = errors.is_empty();
    for e in errors.iter().take(40) {
        eprintln!("INVALID: {e}");
    }
    if errors.len() > 40 {
        eprintln!("INVALID: … {} more", errors.len() - 40);
    }
    if plot {
        println!(
            r#"{{"valid":{valid},"verdict":"{verdict}","i_more":{ki},"r_more":{kr},"p_two_sided":{p:.4},"pooled_i":{pool_i},"pooled_r":{pool_r},"cells":[{}]}}"#,
            cells.join(",")
        );
    } else {
        for l in &lines {
            println!("{l}");
        }
        println!(
            "pooled: I {pool_i}/8192, R {pool_r}/8192; draws I > R {ki}, R > I {kr}; two-sided sign test p = {p:.4} -> {verdict}"
        );
        println!(
            "validity (identity control, identity and exactness, all branches present): {}",
            if valid { "valid" } else { "INVALID" }
        );
    }
    if !valid {
        std::process::exit(1);
    }
}
