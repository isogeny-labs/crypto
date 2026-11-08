//! Fit Callgrind instructions against counted `bitop/v1` gates for a set of
//! `ecbench audit-ir` records: `Ir = a + b·gates` by ordinary least squares,
//! on the whole bracketed solve and again with named fixed parts removed.
//!
//! Input: one or more `ecbench.audit_ir/v1` JSON Lines files and, optionally,
//! an attribution file (`--attrib`) whose lines carry `seq` and any number of
//! `*_ir` fields that are fixed per-solve parts measured by
//! `callgrind_annotate --inclusive=yes` on the record's Callgrind parts.  The
//! fields named by `--fixed` are subtracted before the second fit.
//!
//! Output: one JSON object on stdout (the fits, and per record its gates, Ir,
//! Ir per gate with and without the fixed parts).
//!
//! ```sh
//! cargo run --release --example ir_gate_fit -- --audit ladder.jsonl \
//!     --attrib attrib.jsonl --fixed classes_new_ir,full_price_reprice_ir
//! ```

use std::collections::BTreeMap;

use serde_json::{json, Value};

fn ols(points: &[(f64, f64)]) -> Value {
    let n = points.len() as f64;
    if points.len() < 2 {
        return json!({"n": points.len(), "fit": null});
    }
    let mx = points.iter().map(|p| p.0).sum::<f64>() / n;
    let my = points.iter().map(|p| p.1).sum::<f64>() / n;
    let sxx = points.iter().map(|p| (p.0 - mx).powi(2)).sum::<f64>();
    let sxy = points.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum::<f64>();
    let syy = points.iter().map(|p| (p.1 - my).powi(2)).sum::<f64>();
    let b = sxy / sxx;
    let a = my - b * mx;
    let sse = points
        .iter()
        .map(|p| (p.1 - a - b * p.0).powi(2))
        .sum::<f64>();
    let r2 = if syy > 0.0 { 1.0 - sse / syy } else { 1.0 };
    // Standard errors (n − 2 degrees of freedom); null with two points.
    let (se_a, se_b) = if points.len() > 2 {
        let s2 = sse / (n - 2.0);
        (
            Some((s2 * (1.0 / n + mx * mx / sxx)).sqrt()),
            Some((s2 / sxx).sqrt()),
        )
    } else {
        (None, None)
    };
    json!({
        "n": points.len(),
        "intercept_ir": a,
        "intercept_ir_se": se_a,
        "slope_ir_per_gate": b,
        "slope_ir_per_gate_se": se_b,
        "r2": r2,
    })
}

fn usage() -> ! {
    eprintln!("usage: ir_gate_fit --audit FILE[,FILE..] [--attrib FILE] [--fixed F1,F2..]");
    std::process::exit(2)
}

fn main() {
    let mut audits: Vec<String> = Vec::new();
    let mut attrib: Option<String> = None;
    let mut fixed: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = || args.next().unwrap_or_else(|| usage());
        match a.as_str() {
            "--audit" => audits = val().split(',').map(str::to_string).collect(),
            "--attrib" => attrib = Some(val()),
            "--fixed" => fixed = val().split(',').map(str::to_string).collect(),
            _ => usage(),
        }
    }
    if audits.is_empty() {
        usage();
    }
    let read = |path: &str| -> Vec<Value> {
        std::fs::read_to_string(path)
            .expect("read input")
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).expect("json line"))
            .collect()
    };
    let mut parts: BTreeMap<u64, Value> = BTreeMap::new();
    if let Some(p) = &attrib {
        for v in read(p) {
            parts.insert(v["seq"].as_u64().expect("seq"), v);
        }
    }
    let mut whole = Vec::new();
    let mut walk = Vec::new();
    let mut rows = Vec::new();
    for path in &audits {
        for r in read(path) {
            let seq = r["seq"].as_u64().expect("seq");
            let gates: f64 = r["bitops"]
                .as_str()
                .expect("bitops")
                .parse()
                .expect("bitops integer");
            let ir = r["solve_ir"].as_f64().expect("solve_ir");
            let removed: f64 = fixed
                .iter()
                .map(|f| parts.get(&seq).and_then(|v| v[f].as_f64()).unwrap_or(0.0))
                .sum();
            whole.push((gates, ir));
            if !fixed.is_empty() {
                walk.push((gates, ir - removed));
            }
            rows.push(json!({
                "seq": seq,
                "method": r["method"],
                "gates": gates,
                "solve_ir": ir,
                "ir_per_gate": ir / gates,
                "fixed_removed_ir": removed,
                "ir_per_gate_without_fixed": (ir - removed) / gates,
            }));
        }
    }
    let out = json!({
        "schema": "ir_gate_fit/v1",
        "inputs": audits,
        "attribution": attrib,
        "fixed_parts": fixed,
        "fit_whole_solve": ols(&whole),
        "fit_without_fixed": ols(&walk),
        "records": rows,
    });
    println!("{}", serde_json::to_string_pretty(&out).expect("json"));
}
