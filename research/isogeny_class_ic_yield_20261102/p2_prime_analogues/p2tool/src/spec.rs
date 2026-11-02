//! `p2tool spec`: write an `ecbench.spec/v1` for one class and one m.
//!
//! Fixed-|F| rule (preregistered, PROTOCOL.md section 4): on every curve of a
//! class the factor base is `prime-abscissa:size=N` with N a function of the
//! group order n and the target yield y* only (m=2: N=ceil(sqrt(y* n/2)),
//! m=3: N=ceil(cbrt(y* n/4))); the attempt budget is A = ceil(R / y*).
use crate::arg;
use serde_json::{json, Value};

/// N from the target yield, capped so that the pair table of the mitm oracle,
/// (2N)^2/2 entries, stays within `table_cap` (memory and set-up time).
pub fn fb_size(n: u64, m: u32, y: f64, table_cap: f64) -> u64 {
    let n = n as f64;
    let ny = match m {
        2 => (y * n / 2.0).sqrt().ceil(),
        3 => (y * n / 4.0).cbrt().ceil(),
        _ => panic!("m must be 2 or 3"),
    };
    ny.min((table_cap / 2.0).sqrt().floor()) as u64
}
/// Predicted yield per attempt of mitm:m. Measured in the calibration runs (smoke/, calibration/): m=2 0.0199 vs 0.0201 (24b), 0.00098 vs 0.00102 (32b), 7.0e-5 vs 6.4e-5 (36b); m=3 0.030-0.038 over N=70..1181, fitted (4/3)N^3/n.
pub fn predicted_yield(n: u64, m: u32, nfb: u64) -> f64 {
    let (n, f) = (n as f64, nfb as f64);
    if m == 2 { 2.0 * f * f / n } else { 4.0 / 3.0 * f * f * f / n }
}

pub fn run(args: &[String]) {
    let classes_path = arg(args, "--classes").expect("--classes classes.json");
    let cid = arg(args, "--class").expect("--class c24A");
    let m: u32 = arg(args, "--m").map(|s| s.parse().unwrap()).unwrap_or(2);
    let y: f64 = arg(args, "--y").map(|s| s.parse().unwrap()).unwrap_or(0.02);
    let rel: f64 = arg(args, "--relations").map(|s| s.parse().unwrap()).unwrap_or(200.0);
    let rounds: u32 = arg(args, "--rounds").map(|s| s.parse().unwrap()).unwrap_or(3);
    let targets: u32 = arg(args, "--targets").map(|s| s.parse().unwrap()).unwrap_or(4);
    let first: usize = arg(args, "--first").map(|s| s.parse().unwrap()).unwrap_or(usize::MAX);
    let iso: usize = arg(args, "--iso").map(|s| s.parse().unwrap()).unwrap_or(usize::MAX);
    let level = arg(args, "--isolation").unwrap_or("L2".into());
    let seed: u64 = arg(args, "--seed").map(|s| s.parse().unwrap()).unwrap_or(20261102);
    let out = arg(args, "--out").expect("--out spec.json");
    let doc: Value = serde_json::from_str(&std::fs::read_to_string(&classes_path).unwrap()).unwrap();
    let c = doc["classes"].as_array().unwrap().iter().find(|c| c["id"] == cid.as_str()).expect("class id");
    let n = c["n"].as_u64().unwrap();
    let p = c["p"].as_u64().unwrap();
    let table_cap: f64 = arg(args, "--table-cap").map(|s| s.parse().unwrap()).unwrap_or(4194304.0);
    let with_aa = arg(args, "--aa").map(|s| s == "1").unwrap_or(true);
    let nfb = fb_size(n, m, y, table_cap);
    let y_eff = predicted_yield(n, m, nfb).min(1.0);
    let nfb_pc = ((nfb as f64) * 1.2f64.powf(1.0 / m as f64)).round() as u64;
    let kind = arg(args, "--kind").unwrap_or("yield".into());
    assert!(kind == "yield" || kind == "solve");
    // yield: linalg=incremental-gauss-full-rank never completes while the relation count stays
    // below the number of columns (|F|), so every run makes exactly `attempts` independent
    // decomposition attempts (a fixed-n binomial count, no data-dependent stopping).
    // R is capped at 0.6 |F| for that reason.
    let rel = if kind == "yield" { rel.min(0.6 * nfb as f64) } else { rel };
    let attempts = (rel / y_eff).ceil() as u64;
    let mk = |cv: &Value| json!({"kind": "prime_explicit", "p": p, "a": cv["a"], "b": cv["b"], "group_order": n,
                                  "gx": cv["gx"], "gy": cv["gy"], "r": n});
    // Curves: the first `first` of the class plus the base curve of every selected
    // isomorphic model (so a smoke subset still has its iso pairs); all of them in a full run.
    let iso_sel: Vec<&Value> = c["iso_models"].as_array().unwrap().iter().take(iso).collect();
    let mut idx: Vec<usize> = (0..c["curves"].as_array().unwrap().len().min(first)).collect();
    for m in &iso_sel {
        idx.push(m["of"].as_u64().unwrap() as usize);
    }
    idx.sort();
    idx.dedup();
    let mut curves = vec![];
    for &i in &idx {
        curves.push(mk(&c["curves"][i]));
    }
    for cv in &iso_sel {
        curves.push(mk(cv));
    }
    let arm = |name: &str, role: &str, size: u64| {
        json!({"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
            "factor_base": format!("prime-abscissa:size={size}"),
            "oracle": format!("mitm:m={m}"),
            "max_trials": if kind == "yield" { attempts.to_string() } else { "100000000".into() },
            "linalg": if kind == "yield" { "incremental-gauss-full-rank" } else { "incremental-gauss" }}}})
    };
    let mut arms_v = vec![arm(&format!("m{m}-main"), "reference", nfb)];
    if with_aa {
        arms_v.push(arm(&format!("m{m}-aa"), "control", nfb));
    }
    arms_v.push(arm(&format!("m{m}-pc"), "candidate", nfb_pc));
    let spec = json!({
        "schema": "ecbench.spec/v1",
        "label": format!("P2 {kind} {cid} m={m}: yield on every curve of one isogeny class, |F|={nfb}"),
        "description": format!("research/isogeny_class_ic_yield_20261102/p2_prime_analogues: class {cid} (D={}, h={}, p={p}, n={n}); \
            same prime-abscissa FB size {nfb} on every curve, mitm m={m}, attempt cap {attempts}; arms main, aa (identical repeat), \
            pc (FB x1.2^(1/{m}) = planted +20% yield); the last curves are u-scaled isomorphic models.", c["D"], c["h"]),
        "arms": arms_v,
        "workloads": {"curves": curves, "targets_per_curve": targets, "target_seed": seed},
        "measurement": {"isolation_required": level, "order": "alternate", "rounds": rounds, "seed": seed % 1000 + 1,
                        "timeout_seconds": 3600, "warmup": 1},
    });
    std::fs::write(&out, serde_json::to_string_pretty(&spec).unwrap()).unwrap();
    eprintln!("wrote {out}: kind {kind} class {cid} m={m} |F|={nfb} (pc {nfb_pc}) predicted_yield={y_eff:.3e} attempts={attempts} curves={}", spec["workloads"]["curves"].as_array().unwrap().len());
}
