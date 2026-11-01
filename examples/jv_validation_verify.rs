//! Independent recomputation for `research/jv_sieve_validation_20261101`.
//!
//! This deliberately does not call the frozen analysis implementation.  It
//! re-parses the raw producer rows, reconstructs the four headline paired
//! bootstrap intervals, and compares them with `analysis.json`.

use std::fs;
use std::path::{Path, PathBuf};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const NBOOT: usize = 10_000;
const BASE_SEED: u64 = 20_261_101;

#[derive(Clone)]
struct Sample {
    seed: u64,
    total: f64,
    native_price: f64,
    native_count: f64,
    row: Value,
}

fn number(v: &Value) -> f64 {
    if let Some(x) = v.as_f64() {
        x
    } else {
        v.as_str().expect("numeric value").parse().expect("number")
    }
}

fn one_line(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let lines: Vec<&str> = text.lines().filter(|x| !x.trim().is_empty()).collect();
    assert_eq!(lines.len(), 1, "{} must contain one row", path.display());
    serde_json::from_str(lines[0]).expect("producer JSON")
}

fn sample(row: Value) -> Sample {
    let p = row["p"].as_u64().expect("p");
    let width = (64 - p.leading_zeros()) as f64;
    let counts = &row["total"]["counts"];
    let words = number(&counts["word_op"]);
    let probes = number(&counts["hash_probe"]);
    Sample {
        seed: row["seed"].as_u64().expect("seed"),
        total: number(&row["total_bitops"]),
        native_price: 64.0 * words + 4.0 * width * probes,
        native_count: words + probes,
        row,
    }
}

fn average(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn paired(
    left: &[Sample],
    right: &[Sample],
    cost: impl Fn(&Sample) -> f64,
    seed: u64,
) -> (f64, f64, f64) {
    assert_eq!(left.len(), right.len());
    assert_eq!(left.len(), 8);
    assert!(left.iter().zip(right).all(|(a, b)| a.seed == b.seed));
    let a: Vec<f64> = left.iter().map(&cost).collect();
    let b: Vec<f64> = right.iter().map(&cost).collect();
    let point = average(&a) / average(&b);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut draws = Vec::with_capacity(NBOOT);
    for _ in 0..NBOOT {
        let mut sa = 0.0;
        let mut sb = 0.0;
        for _ in 0..a.len() {
            let j = rng.gen_range(0..a.len());
            sa += a[j];
            sb += b[j];
        }
        draws.push(sa / sb);
    }
    draws.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let q = |x: f64| draws[((draws.len() - 1) as f64 * x).round() as usize];
    (point, q(0.025), q(0.975))
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut panel: Option<PathBuf> = None;
    let mut analysis: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--panel" => panel = Some(PathBuf::from(&args[i + 1])),
            "--analysis" => analysis = Some(PathBuf::from(&args[i + 1])),
            "--out" => out = Some(PathBuf::from(&args[i + 1])),
            other => panic!("unknown argument {other}"),
        }
        i += 2;
    }
    let panel = panel.expect("--panel");
    let analysis_path = analysis.expect("--analysis");
    let out = out.expect("--out");
    let analysis: Value = serde_json::from_str(&fs::read_to_string(&analysis_path).unwrap()).unwrap();
    let p = analysis["p"].as_u64().expect("analysis p");
    let mut ic = Vec::new();
    let mut rho = Vec::new();
    let mut files = Vec::new();
    for entry in fs::read_dir(panel.join("raw")).expect("raw") {
        let path = entry.expect("entry").path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        if name.starts_with("ic_s") || name.starts_with("rho_R3_run0_") {
            let x = sample(one_line(&path));
            let evidence_root = Path::new("research/jv_sieve_validation_20261101");
            files.push(
                path.strip_prefix(evidence_root)
                    .unwrap_or(&path)
                    .display()
                    .to_string(),
            );
            if name.starts_with("ic_s") {
                ic.push(x);
            } else {
                rho.push(x);
            }
        }
    }
    ic.sort_by_key(|x| x.seed);
    rho.sort_by_key(|x| x.seed);
    files.sort();
    let symmetry_path = panel.join("raw/symmetry.jsonl");
    let symmetry_rows: Vec<Value> = fs::read_to_string(&symmetry_path)
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let symmetry_verified = symmetry_rows.len() == 8
        && symmetry_rows.iter().all(|row| {
            row["p"].as_u64() == Some(p)
                && row["j_in_Fp2"].as_bool() == Some(false)
                && row["j_in_Fp3"].as_bool() == Some(false)
        });
    let evidence_root = Path::new("research/jv_sieve_validation_20261101");
    files.push(
        symmetry_path
            .strip_prefix(evidence_root)
            .unwrap_or(&symmetry_path)
            .display()
            .to_string(),
    );
    files.sort();
    let fq = &rho[0].row["fq3_prices"];
    let bits = (64 - p.leading_zeros()) as f64;
    let gae = 2.0 * number(&fq["mul"]["bitops"])
        + number(&fq["sq"]["bitops"])
        + number(&fq["inv"]["bitops"])
        + 42.0 * 13.0 * bits;

    let got = paired(&ic, &rho, |x| x.total, BASE_SEED ^ (p << 8));
    let recorded = &analysis["comparison"]["bitop_v1"];
    let want = (
        number(&recorded["ratio"]),
        number(&recorded["ci95"][0]),
        number(&recorded["ci95"][1]),
    );
    let mut checks = vec![json!({
        "metric": "bitop_v1",
        "recomputed": [got.0, got.1, got.2],
        "recorded": [want.0, want.1, want.2],
        "pass": close(got.0, want.0) && close(got.1, want.1) && close(got.2, want.2),
    })];
    for (k, w) in [0.0, 0.1, 1.0].into_iter().enumerate() {
        let got = paired(
            &ic,
            &rho,
            |x| x.total - x.native_price + w * gae * x.native_count,
            BASE_SEED ^ (p << 8) ^ 0x40 ^ k as u64,
        );
        let recorded = &analysis["comparison"]["gae_band"][k]["result"];
        let want = (
            number(&recorded["ratio"]),
            number(&recorded["ci95"][0]),
            number(&recorded["ci95"][1]),
        );
        checks.push(json!({
            "metric": format!("band_{w}"),
            "recomputed": [got.0, got.1, got.2],
            "recorded": [want.0, want.1, want.2],
            "pass": close(got.0, want.0) && close(got.1, want.1) && close(got.2, want.2),
        }));
    }
    let scalars_verified = ic.iter().all(|x| {
        x.row["correct"].as_bool() == Some(true)
            && x.row["verified_on_challenge"].as_bool() == Some(true)
    }) && rho.iter().all(|x| {
        x.row["correct"].as_bool() == Some(true)
            && x.row["verified_on_challenge"].as_bool() == Some(true)
    });
    let pass = checks.iter().all(|x| x["pass"].as_bool() == Some(true))
        && scalars_verified
        && symmetry_verified;
    let mut headline = Vec::new();
    for check in &checks {
        for (index, suffix) in ["point", "lo95", "hi95"].into_iter().enumerate() {
            headline.push(json!({
                "name": format!("{}_{}", check["metric"].as_str().unwrap(), suffix),
                "claimed": number(&check["recorded"][index]),
                "recomputed": number(&check["recomputed"][index]),
                "rel_tol": 1e-12,
            }));
        }
    }
    let report = json!({
        "schema": "jv_sieve_validation_validator/v1",
        "status": if pass { "pass" } else { "fail" },
        "independent": true,
        "independence_basis": "separate clean workflow job and a verifier implementation that does not call the producer analysis",
        "p": p,
        "raw_data": files,
        "runs": ic.iter().map(|x| x.seed).collect::<Vec<_>>(),
        "scalars_verified": scalars_verified,
        "symmetry_verified": symmetry_verified,
        "headline": headline,
        "checks": checks,
        "tolerance": "relative 1e-12",
    });
    fs::create_dir_all(out.parent().unwrap()).unwrap();
    fs::write(&out, serde_json::to_string_pretty(&report).unwrap() + "\n").unwrap();
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    if !pass {
        std::process::exit(1);
    }
}
