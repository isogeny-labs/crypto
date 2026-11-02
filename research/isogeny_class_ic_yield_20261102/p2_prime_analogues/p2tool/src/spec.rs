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

/// Follow-up 1 (FOLLOWUP_c32A_m3.md section 6): the workload list of `spec --focus`.
/// Order: primary, comparators ascending as listed in the focus file, iso models by k.
/// Every workload is re-verified in exact u64/u128 arithmetic: (gx,gy) on the curve,
/// n*G = O, and for the iso models j(a,b) = primary.j. Any failure is an Err (the caller exits nonzero).
pub fn focus_curves(c: &Value, focus: &Value, cid: &str, m: u32) -> Result<Vec<Value>, String> {
    use crate::ec::Curve;
    let fcid = focus["class_id"].as_str().unwrap_or("");
    if fcid != cid {
        return Err(format!("focus class_id {fcid} != --class {cid}"));
    }
    if focus["m"].as_u64() != Some(m as u64) {
        return Err(format!("focus m {} != --m {m}", focus["m"]));
    }
    let u = |v: &Value, what: &str| v.as_u64().ok_or_else(|| format!("missing/non-integer {what}"));
    let (p, n) = (u(&c["p"], "class p")?, u(&c["n"], "class n")?);
    if u(&focus["p"], "focus p")? != p || u(&focus["n"], "focus n")? != n {
        return Err("focus p/n differ from the class".into());
    }
    let prim = &focus["primary"];
    let pidx = u(&prim["index"], "primary.index")? as usize;
    let cj = u(&prim["j"], "primary.j")?;
    let curves = c["curves"].as_array().ok_or("class has no curves")?;
    let mut models: Vec<(String, Value)> = vec![];
    // primary and comparators come from classes.json; the focus file's primary is cross-checked against it
    let from_class = |i: usize| curves.get(i).cloned().ok_or_else(|| format!("curve index {i} not in class"));
    let pc = from_class(pidx)?;
    for k in ["a", "b", "gx", "gy"] {
        if pc[k] != prim[k] {
            return Err(format!("primary.{k} in the focus file differs from classes.json curve {pidx}"));
        }
    }
    if pc["j"].as_u64() != Some(cj) {
        return Err(format!("primary.j differs from classes.json curve {pidx}"));
    }
    models.push((format!("primary {pidx}"), pc));
    let comps = focus["comparator_indices"].as_array().ok_or("comparator_indices")?;
    let mut last = None;
    for ci in comps {
        let i = u(ci, "comparator index")? as usize;
        if last.map_or(false, |l| i <= l) || i == pidx {
            return Err("comparator indices must be strictly ascending and exclude the primary".into());
        }
        last = Some(i);
        models.push((format!("comparator {i}"), from_class(i)?));
    }
    let isos = focus["iso_models"].as_array().ok_or("iso_models")?;
    for (k, im) in isos.iter().enumerate() {
        if im["k"].as_u64() != Some(k as u64) || im["of"].as_u64() != Some(pidx as u64) {
            return Err(format!("iso model {k}: k/of mismatch"));
        }
        models.push((format!("iso k={k}"), im.clone()));
    }
    let mut out = vec![];
    for (name, cv) in models {
        let (a, b, gx, gy) = (u(&cv["a"], "a")?, u(&cv["b"], "b")?, u(&cv["gx"], "gx")?, u(&cv["gy"], "gy")?);
        let e = Curve { p, a, b };
        let g = Some((gx, gy));
        if !e.on(g) {
            return Err(format!("{name}: G not on the curve"));
        }
        if e.mul(n, g).is_some() {
            return Err(format!("{name}: n*G != O"));
        }
        if name.starts_with("iso") && e.j() != cj {
            return Err(format!("{name}: j {} != primary j {cj}", e.j()));
        }
        if name.starts_with("primary") && e.j() != cj {
            return Err(format!("{name}: computed j {} != j {cj}", e.j()));
        }
        out.push(cv);
    }
    Ok(out)
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
    let focus_path = arg(args, "--focus");
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
    let curves = if let Some(fp) = &focus_path {
        if arg(args, "--first").is_some() || arg(args, "--iso").is_some() {
            eprintln!("error: --focus cannot be combined with --first or --iso");
            std::process::exit(2);
        }
        let focus: Value = serde_json::from_str(&std::fs::read_to_string(fp).expect("read --focus file")).expect("focus json");
        match focus_curves(c, &focus, &cid, m) {
            Ok(v) => v.iter().map(|cv| mk(cv)).collect::<Vec<Value>>(),
            Err(e) => {
                eprintln!("error: focus verification failed: {e}");
                std::process::exit(1);
            }
        }
    } else {
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
        curves
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    fn load(name: &str) -> Value {
        let p = format!("{}/../{name}", env!("CARGO_MANIFEST_DIR"));
        serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
    }
    fn class() -> Value {
        load("classes.json")["classes"].as_array().unwrap().iter().find(|c| c["id"] == "c32A").unwrap().clone()
    }
    #[test]
    fn focus_builds_24_in_order() {
        let (c, f) = (class(), load("followup1_focus.json"));
        let v = focus_curves(&c, &f, "c32A", 3).unwrap();
        assert_eq!(v.len(), 24);
        assert_eq!(v[0]["a"], c["curves"][80]["a"]);
        assert_eq!(v[1]["a"], c["curves"][7]["a"]);
        assert_eq!(v[15]["a"], c["curves"][123]["a"]);
        assert_eq!(v[16]["a"], f["iso_models"][0]["a"]);
        assert_eq!(v[23]["a"], f["iso_models"][7]["a"]);
    }
    #[test]
    fn focus_rejects_bad_models() {
        let (c, f0) = (class(), load("followup1_focus.json"));
        assert!(focus_curves(&c, &f0, "c32A", 2).is_err());
        assert!(focus_curves(&c, &f0, "c24A", 3).is_err());
        let mut f = f0.clone();
        f["iso_models"][3]["gy"] = serde_json::json!(f0["iso_models"][3]["gy"].as_u64().unwrap() + 1); // off the curve
        assert!(focus_curves(&c, &f, "c32A", 3).unwrap_err().contains("not on the curve"));
        let mut f = f0.clone();
        // a different (valid-looking) model of another curve: right curve and order but wrong j
        f["iso_models"][0]["a"] = c["curves"][34]["a"].clone();
        f["iso_models"][0]["b"] = c["curves"][34]["b"].clone();
        f["iso_models"][0]["gx"] = c["curves"][34]["gx"].clone();
        f["iso_models"][0]["gy"] = c["curves"][34]["gy"].clone();
        assert!(focus_curves(&c, &f, "c32A", 3).unwrap_err().contains("j "));
        let mut f = f0.clone();
        f["primary"]["a"] = serde_json::json!(1);
        assert!(focus_curves(&c, &f, "c32A", 3).is_err());
        let mut f = f0.clone();
        f["comparator_indices"][2] = serde_json::json!(5); // not ascending
        assert!(focus_curves(&c, &f, "c32A", 3).is_err());
    }
}
