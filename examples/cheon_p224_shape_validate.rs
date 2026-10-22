//! Independent validator for `research/cheon_p224_shape_20261022/`.
//!
//! Shares no code with `cheon_p224_shape_analyse`.  It reads only the raw
//! records (`runs.jsonl`, the ecbench rho `records.jsonl`, `curves.json`),
//! recomputes the headline numbers of `results.json` (per-size median
//! operations, fitted slopes by the normal equations, the 224-bit
//! extrapolation of fit A) and writes a `validator-report.json` in the
//! `validation-gate/v1` shape.
//!
//! ```text
//! cheon_p224_shape_validate --curves C --runs R --rho RECORDS [--rho ...] \
//!     --results results.json --out validator-report.json
//! ```

use serde_json::{json, Value};
use std::collections::BTreeMap;

fn args_of(name: &str) -> Vec<String> {
    let a: Vec<String> = std::env::args().collect();
    a.iter()
        .enumerate()
        .filter(|(_, x)| *x == name)
        .filter_map(|(i, _)| a.get(i + 1).cloned())
        .collect()
}

fn lines(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn sha256_hex(path: &str) -> String {
    use sha2::{Digest, Sha256};
    let b = std::fs::read(path).unwrap();
    Sha256::digest(&b)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}

fn main() {
    let curves_p = args_of("--curves").pop().unwrap();
    let runs_p = args_of("--runs").pop().unwrap();
    let rho_ps = args_of("--rho");
    let results_p = args_of("--results").pop().unwrap();
    let out_p = args_of("--out").pop().unwrap();

    let cv: Value = serde_json::from_str(&std::fs::read_to_string(&curves_p).unwrap()).unwrap();
    // n -> (family, bits)
    let mut by_n: BTreeMap<u64, (String, u64)> = BTreeMap::new();
    for c in cv["curves"].as_array().unwrap() {
        by_n.insert(
            c["n"].as_u64().unwrap(),
            (
                c["family"].as_str().unwrap().to_string(),
                c["bits"].as_u64().unwrap(),
            ),
        );
    }
    // (arm, family, bits) -> verified ops list
    let mut ops: BTreeMap<(String, String, u64), Vec<f64>> = BTreeMap::new();
    for r in lines(&runs_p) {
        if r["correct"] == true && r["recovered"] == r["planted"] {
            ops.entry((
                r["arm"].as_str().unwrap().into(),
                r["family"].as_str().unwrap().into(),
                r["bits"].as_u64().unwrap(),
            ))
            .or_default()
            .push(r["ops"].as_f64().unwrap());
        }
    }
    for p in &rho_ps {
        for r in lines(p) {
            if r["outcome"]["status"] != "verified" || r["outcome"]["matches_planted"] != true {
                continue;
            }
            let n = r["workload"]["curve"]["r"].as_u64().unwrap();
            if let Some((fam, bits)) = by_n.get(&n) {
                ops.entry(("rho.negation".into(), fam.clone(), *bits))
                    .or_default()
                    .push(r["cost"]["total_gae"].as_f64().unwrap());
            }
        }
    }
    let res: Value = serde_json::from_str(&std::fs::read_to_string(&results_p).unwrap()).unwrap();

    let mut headline = vec![];
    // 1. medians at every size/arm from Table 1
    for row in res["table1"].as_array().unwrap() {
        let key = (
            row["arm"].as_str().unwrap().to_string(),
            row["family"].as_str().unwrap().to_string(),
            row["bits"].as_u64().unwrap(),
        );
        let Some(v) = ops.get(&key) else { continue };
        let mut s = v.clone();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let m = s.len();
        let med = if m % 2 == 1 {
            s[m / 2]
        } else {
            (s[m / 2 - 1] + s[m / 2]) / 2.0
        };
        headline.push(
            json!({"name": format!("median ops {} {} {}b", key.0, key.1, key.2),
            "claimed": row["median_ops"], "recomputed": med, "rel_tol": 1e-9}),
        );
    }
    // 2. slopes by the normal equations, 3. extrapolation of fit A to 224
    for (key, fit) in res["fits"].as_object().unwrap() {
        let (fam, arm) = key.split_once('/').unwrap();
        let mut pts = vec![];
        for (k, v) in &ops {
            if k.0 == arm && k.1 == fam && v.len() >= 4 {
                // bit length of n from the curves file
                let n = by_n
                    .iter()
                    .find(|(_, (f, b))| f == fam && *b == k.2)
                    .map(|(n, _)| *n)
                    .unwrap();
                let x = (n as f64).log2();
                let y = v.iter().map(|o| o.log2()).sum::<f64>() / v.len() as f64;
                pts.push((x, y));
            }
        }
        let k = pts.len() as f64;
        let (sx, sy): (f64, f64) = pts.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0, a.1 + p.1));
        let sxx: f64 = pts.iter().map(|p| p.0 * p.0).sum();
        let sxy: f64 = pts.iter().map(|p| p.0 * p.1).sum();
        let det = k * sxx - sx * sx;
        let slope = (k * sxy - sx * sy) / det;
        let icpt = (sy * sxx - sx * sxy) / det;
        headline.push(json!({"name": format!("fit A slope {key}"), "claimed": fit["b"], "recomputed": slope, "rel_tol": 1e-6}));
        headline.push(json!({"name": format!("fit A log2 ops at log2 n = 224 {key} (extrapolation)"),
            "claimed": fit["extrap224"], "recomputed": icpt + slope * 224.000_000_17, "rel_tol": 1e-6}));
    }
    let pass = headline.iter().all(|h| {
        let (c, r) = (
            h["claimed"].as_f64().unwrap(),
            h["recomputed"].as_f64().unwrap(),
        );
        let tol = h["rel_tol"].as_f64().unwrap();
        (c - r).abs() <= tol * c.abs().max(1e-300)
    });
    let raw: Vec<Value> = std::iter::once((&curves_p, true))
        .chain(std::iter::once((&runs_p, true)))
        .chain(rho_ps.iter().map(|p| (p, true)))
        .map(|(p, _)| json!({"path": p, "sha256": sha256_hex(p)}))
        .collect();
    let doc = json!({
        "schema": "cheon-p224-shape.validator/v1",
        "status": if pass { "pass" } else { "fail" },
        "independent": true,
        "independence": "separate program sharing no code with the analyser; slopes by the normal equations from the raw run records",
        "raw_data": raw,
        "headline": headline,
    });
    std::fs::write(&out_p, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    println!(
        "validator: {} ({} headline numbers)",
        if pass { "pass" } else { "FAIL" },
        doc["headline"].as_array().unwrap().len()
    );
    if !pass {
        std::process::exit(1);
    }
}
