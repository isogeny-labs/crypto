//! Frozen analysis (PROTOCOL.md part B): paired D2S/rho, D2S23/rho, DS/rho,
//! D2/rho, D2S/DS, D2S/D2, D2S23/D2S per rung with a hierarchical bootstrap;
//! the §5a band; the rho A/A floor; D2Sall ≡ DS identity; census; exponent
//! fits; crossover projections (model); the SVG figure.  Also the part A
//! pilot rule and the prediction summary.  Adapted from #5291's analysis.rs.
use crate::seeds::Sm64;
use serde_json::{json, Value};
use std::collections::BTreeMap;

const ARMS: [&str; 5] = ["D2S", "D2S23", "D2Sall", "DS", "D2"];
const KEYS: [&str; 5] = ["meq_s1_u1", "meq_s1_u01", "meq_s1_u0", "meq_s025_u1", "meq_s025_u0"];
const REL_PHASES: [&str; 7] = ["fb", "setup", "targets", "descent", "rootfind", "lift", "resolve"];

#[derive(Default, Clone)]
struct Case {
    ell: f64,
    h: f64,
    rho: Vec<BTreeMap<String, f64>>,
    rho_w64: Vec<f64>,
    ic: BTreeMap<String, BTreeMap<String, f64>>,
    rec: BTreeMap<String, Value>,
}

fn gmean(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn pct(v: &mut Vec<f64>, p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = ((v.len() - 1) as f64 * p).round() as usize;
    v[k]
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn ols(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let b = sxy / sxx;
    (b, my - b * mx)
}

type Rung = BTreeMap<u32, BTreeMap<u32, Case>>;

fn rho_mean(cd: &Case, key: &str, idx: Option<&[usize]>) -> Option<f64> {
    if cd.rho.is_empty() {
        return None;
    }
    let v: Vec<f64> = match idx {
        None => cd.rho.iter().map(|r| r[key]).collect(),
        Some(ix) => ix.iter().map(|&i| cd.rho[i][key]).collect(),
    };
    Some(mean(&v))
}

type F<'a> = &'a dyn Fn(&Case, Option<&[usize]>) -> Option<f64>;

/// Geometric mean over cases of num/den, 95% hierarchical bootstrap (curves, targets, rho seeds).
fn stat(r: &Rung, num: F, den: F, rng: &mut Sm64, b: usize) -> Option<Value> {
    let mut pts = vec![];
    for ts in r.values() {
        for cd in ts.values() {
            if let (Some(a), Some(d)) = (num(cd, None), den(cd, None)) {
                pts.push(a / d);
            }
        }
    }
    if pts.is_empty() {
        return None;
    }
    let point = gmean(&pts);
    // only curves/targets where both sides exist enter the resampling
    let curves: Vec<Vec<&Case>> = r
        .values()
        .map(|ts| ts.values().filter(|cd| num(cd, None).is_some() && den(cd, None).is_some()).collect::<Vec<_>>())
        .filter(|v| !v.is_empty())
        .collect();
    let mut boots = Vec::with_capacity(b);
    for _ in 0..b {
        let mut v = vec![];
        for _ in 0..curves.len() {
            let ts = &curves[rng.below(curves.len() as u64) as usize];
            for _ in 0..ts.len() {
                let cd = ts[rng.below(ts.len() as u64) as usize];
                let ns = cd.rho.len().max(1);
                let idx: Vec<usize> = (0..ns).map(|_| rng.below(ns as u64) as usize).collect();
                if let (Some(a), Some(d)) = (num(cd, Some(&idx)), den(cd, Some(&idx))) {
                    v.push(a / d);
                }
            }
        }
        if !v.is_empty() {
            boots.push(gmean(&v));
        }
    }
    Some(json!({"ratio": point, "lo": pct(&mut boots.clone(), 0.025), "hi": pct(&mut boots, 0.975), "cases": pts.len()}))
}

fn phase_meq(rec: &Value, phase: &str, n: u32, ws: f64, wu: f64) -> f64 {
    if rec["phases"][phase].is_null() {
        return 0.0;
    }
    let o = crate::field::Ops::from_json(&rec["phases"][phase]);
    crate::field::meq(&o, n, ws, wu)
}
fn rel_meq(rec: &Value, n: u32) -> f64 {
    REL_PHASES.iter().map(|p| phase_meq(rec, p, n, 1.0, 1.0)).sum()
}
fn fsz(v: &Value) -> f64 {
    v["fs"].as_f64().unwrap_or(v["fb"].as_f64().unwrap() - 1.0)
}

pub fn analyze(protocol: &str, runs: &str, out: &str, svg: &str) {
    let proto: Value = serde_json::from_str(&std::fs::read_to_string(protocol).unwrap()).unwrap();
    let text = std::fs::read_to_string(runs).unwrap();
    let mut tree: BTreeMap<u32, Rung> = BTreeMap::new();
    let mut counts: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let mut bad = vec![];
    let mut chk = BTreeMap::<&str, u64>::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).unwrap();
        let arm = v["arm"].as_str().unwrap_or("").to_string();
        if arm != "RHO" && arm != "RHO_W64" && !ARMS.contains(&arm.as_str()) {
            continue;
        }
        let c = &v["case"];
        let (n, ci, ti) = (c["n"].as_u64().unwrap() as u32, c["ci"].as_u64().unwrap() as u32, c["ti"].as_u64().unwrap() as u32);
        let ok = v["correct"].as_bool() == Some(true) && v["verified"].as_bool() == Some(true);
        let e = counts.entry(arm.clone()).or_default();
        e.0 += 1;
        if !ok {
            e.1 += 1;
            bad.push(json!({"n": n, "ci": ci, "ti": ti, "arm": arm, "seed_index": v["seed_index"], "capped": v["capped"]}));
        }
        if arm.starts_with("D2S") {
            *chk.entry("d2s_records").or_default() += 1;
            *chk.entry("row_checks").or_default() += v["row_checks"].as_u64().unwrap_or(0);
            *chk.entry("row_check_failures").or_default() += v["row_check_failures"].as_u64().unwrap_or(0);
            *chk.entry("lp_shadow_compares").or_default() += v["lp_shadow_compares"].as_u64().unwrap_or(0);
            *chk.entry("lp_shadow_mismatches").or_default() += v["lp_shadow_mismatches"].as_u64().unwrap_or(0);
            *chk.entry("rows_unresolved").or_default() += v["rows_unresolved"].as_u64().unwrap_or(0);
            if v["capped"].as_bool() == Some(true) {
                *chk.entry("capped").or_default() += 1;
            }
            match v["track_check"].as_bool() {
                Some(true) => *chk.entry("track_check_pass").or_default() += 1,
                Some(false) => *chk.entry("track_check_fail").or_default() += 1,
                None => {}
            }
            match v["echelon_xcheck"].as_bool() {
                Some(true) => *chk.entry("echelon_xcheck_pass").or_default() += 1,
                Some(false) => *chk.entry("echelon_xcheck_fail").or_default() += 1,
                None => {}
            }
        }
        let cd = tree.entry(n).or_default().entry(ci).or_default().entry(ti).or_default();
        cd.ell = c["curve"]["ell"].as_f64().unwrap();
        cd.h = c["curve"]["h"].as_f64().unwrap_or(2.0);
        if !ok {
            continue;
        }
        let costs: BTreeMap<String, f64> = KEYS.iter().map(|k| (k.to_string(), v["cost"][k].as_f64().unwrap())).collect();
        if arm == "RHO" {
            cd.rho.push(costs);
        } else if arm == "RHO_W64" {
            cd.rho_w64.push(costs["meq_s1_u1"]);
        } else {
            cd.ic.insert(arm.clone(), costs);
            cd.rec.insert(arm, v.clone());
        }
    }
    let mut rng = Sm64::keyed(&["boot", "b2lsp", "main"]);
    let mut rungs = vec![];
    let mut ident = BTreeMap::<&str, u64>::new();
    let mut fit_rows: Vec<Value> = vec![];
    for (&n, r) in &tree {
        let np = n / 3;
        let all: Vec<&Case> = r.values().flat_map(|t| t.values()).collect();
        let ell = gmean(&all.iter().map(|c| c.ell).collect::<Vec<_>>());
        let rho_s: Vec<f64> = all.iter().filter_map(|c| rho_mean(c, "meq_s1_u1", None).map(|m| m / c.ell.sqrt())).collect();
        let mut ratios = serde_json::Map::new();
        let pairs: Vec<(&str, &str, Option<&str>)> = vec![
            ("D2S/rho", "D2S", None),
            ("D2S23/rho", "D2S23", None),
            ("DS/rho", "DS", None),
            ("D2/rho", "D2", None),
            ("D2S/DS", "D2S", Some("DS")),
            ("D2S/D2", "D2S", Some("D2")),
            ("D2S23/DS", "D2S23", Some("DS")),
            ("D2S23/D2S", "D2S23", Some("D2S")),
            ("D2Sall/DS", "D2Sall", Some("DS")),
        ];
        for (label, a, b) in &pairs {
            let mut band = serde_json::Map::new();
            for key in KEYS {
                let bb = if key == "meq_s1_u1" { 10000 } else { 2000 };
                let s = match b {
                    None => stat(r, &|cd, _| cd.ic.get(*a).map(|m| m[key]), &|cd, ix| rho_mean(cd, key, ix), &mut rng, bb),
                    Some(b) => stat(r, &|cd, _| cd.ic.get(*a).map(|m| m[key]), &|cd, _| cd.ic.get(*b).map(|m| m[key]), &mut rng, bb),
                };
                if let Some(s) = s {
                    band.insert(key.into(), s);
                }
            }
            // pm1-charged sensitivity for the D2S arms against rho
            if b.is_none() && a.starts_with("D2S") {
                if let Some(s) = stat(r, &|cd, _| cd.rec.get(*a).and_then(|v| v["cost"]["meq_s1_u1_pm1_charged"].as_f64()), &|cd, ix| rho_mean(cd, "meq_s1_u1", ix), &mut rng, 2000) {
                    band.insert("meq_s1_u1_pm1_charged".into(), s);
                }
            }
            if !band.is_empty() {
                ratios.insert(label.to_string(), Value::Object(band));
            }
        }
        // rho-lane sensitivity (n = 51: W = 64 references of #5290)
        let w64 = stat(r, &|cd, _| cd.ic.get("D2S").map(|m| m["meq_s1_u1"]), &|cd, _| if cd.rho_w64.is_empty() { None } else { Some(mean(&cd.rho_w64)) }, &mut rng, 2000);
        let aa = stat(
            r,
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[..cd.rho.len() / 2].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[cd.rho.len() / 2..].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &mut rng,
            2000,
        );
        let mut relrho = serde_json::Map::new();
        for a in ["D2S", "D2S23", "DS", "D2"] {
            if let Some(s) = stat(r, &|cd, _| cd.rec.get(a).map(|v| rel_meq(v, n)), &|cd, ix| rho_mean(cd, "meq_s1_u1", ix), &mut rng, 2000) {
                relrho.insert(a.into(), s);
            }
        }
        let mut census = serde_json::Map::new();
        for a in ARMS {
            let recs: Vec<&Value> = all.iter().filter_map(|c| c.rec.get(a)).collect();
            if recs.is_empty() {
                continue;
            }
            let per = |f: &dyn Fn(&Value) -> f64| mean(&recs.iter().map(|v| f(v)).collect::<Vec<_>>());
            let tg = per(&|v| v["targets"].as_f64().unwrap());
            let rel = per(&|v| rel_meq(v, n));
            let gr = per(&|v| phase_meq(v, "graph", n, 1.0, 1.0));
            let la = per(&|v| phase_meq(v, "la", n, 1.0, 1.0));
            let tot = per(&|v| v["cost"]["meq_s1_u1"].as_f64().unwrap());
            let relpt = per(&|v| rel_meq(v, n) / v["targets"].as_f64().unwrap());
            let fbn = per(&|v| v["fb"].as_f64().unwrap());
            let fs = per(&|v| fsz(v));
            let mut m = serde_json::Map::new();
            m.insert("targets".into(), json!(tg));
            m.insert("targets_over_qp".into(), json!(tg / (1u64 << np) as f64));
            m.insert("fb".into(), json!(fbn));
            m.insert("fs".into(), json!(fs));
            m.insert("relations".into(), json!(per(&|v| v["relations"].as_f64().unwrap())));
            m.insert("relation_meq_per_target".into(), json!(relpt));
            m.insert("relation_phase_meq".into(), json!(rel));
            m.insert("graph_meq".into(), json!(gr));
            m.insert("la_meq".into(), json!(la));
            m.insert("la_over_fs2".into(), json!(per(&|v| phase_meq(v, "la", n, 1.0, 1.0) / fsz(v).powi(2))));
            m.insert("total_meq".into(), json!(tot));
            m.insert("relation_share".into(), json!(rel / tot));
            m.insert("graph_share".into(), json!(gr / tot));
            m.insert("la_share".into(), json!(la / tot));
            m.insert("wall_s".into(), json!(per(&|v| v["wall_ns"].as_f64().unwrap() / 1e9)));
            if a.starts_with("D2S") {
                m.insert("la_rows".into(), json!(per(&|v| v["la_rows"].as_f64().unwrap_or(0.0))));
                m.insert("la_calls".into(), json!(per(&|v| v["stop_rule"]["la_calls"].as_f64().unwrap_or(0.0))));
                m.insert("checks".into(), json!(per(&|v| v["stop_rule"]["checks"].as_f64().unwrap_or(0.0))));
                for k in ["rows_out", "nnz_out", "core_rows", "sge_pivots"] {
                    m.insert(format!("la_{k}"), json!(per(&|v| v["la_report"][k].as_f64().unwrap_or(0.0))));
                }
                for k in ["full", "one_lp", "two_lp", "discarded_3lp", "yields_full", "yields_cycle", "groundings", "comps_bfs", "max_depth", "mean_depth_used", "reduce_steps", "max_reduce_steps", "yield_weight_mean", "yield_weight_max", "cycle_len_mean", "cycle_len_max", "builds"] {
                    m.insert(format!("graph_{k}"), json!(per(&|v| v["graph"][k].as_f64().unwrap_or(0.0))));
                }
                let ylw: Vec<f64> = recs.iter().map(|v| v["graph"]["yield_weight_mean"].as_f64().unwrap_or(0.0)).collect();
                m.insert("graph_yield_weight_maxcase".into(), json!(ylw.iter().cloned().fold(0.0, f64::max)));
                m.insert("pm1_charged_total_meq".into(), json!(per(&|v| v["cost"]["meq_s1_u1_pm1_charged"].as_f64().unwrap())));
                m.insert("pm1_structural_charged_total_meq".into(), json!(per(&|v| v["cost"]["meq_s1_u1_pm1_and_structural_charged"].as_f64().unwrap())));
            }
            if a == "D2" {
                m.insert("graph_yield_weight_mean".into(), json!(per(&|v| v["graph"]["yield_weight_mean"].as_f64().unwrap_or(0.0))));
            }
            if a == "D2S" || a == "D2S23" {
                fit_rows.push(json!({"arm": a, "n": n, "np": np, "fb": fbn, "fs": fs, "targets": tg, "relpt": relpt, "graph": gr, "la": la,
                    "rho_S": if rho_s.is_empty() { Value::Null } else { json!(mean(&rho_s)) }, "h": mean(&all.iter().map(|c| c.h).collect::<Vec<_>>())}));
            }
            census.insert(a.into(), Value::Object(m));
        }
        if !rho_s.is_empty() {
            fit_rows.push(json!({"arm": "RHO", "n": n, "rho_S": mean(&rho_s), "h": mean(&all.iter().map(|c| c.h).collect::<Vec<_>>())}));
        }
        for ts in r.values() {
            for cd in ts.values() {
                if let (Some(a), Some(b)) = (cd.rec.get("D2Sall"), cd.rec.get("DS")) {
                    *ident.entry("D2Sall_DS_cases").or_default() += 1;
                    if a["d"] == b["d"] && a["targets"] == b["targets"] && a["relations"] == b["relations"] && a["phases"]["la"] == b["phases"]["la"] && a["la_report"] == b["la_report"] && a["stop_rule"]["la_calls"] == b["stop_rule"]["la_calls"] {
                        *ident.entry("D2Sall_DS_equal").or_default() += 1;
                    }
                }
                for (x, y, k1, k2) in [("D2S", "DS", "D2S_DS_cases", "D2S_DS_equal_d"), ("D2S", "D2", "D2S_D2_cases", "D2S_D2_equal_d"), ("D2S23", "DS", "D2S23_DS_cases", "D2S23_DS_equal_d")] {
                    if let (Some(a), Some(b)) = (cd.rec.get(x), cd.rec.get(y)) {
                        *ident.entry(k1).or_default() += 1;
                        if a["d"] == b["d"] {
                            *ident.entry(k2).or_default() += 1;
                        }
                    }
                }
            }
        }
        rungs.push(json!({
            "n": n, "np": np, "log2_ell": ell.log2(), "cases": all.len(),
            "rho_runs": all.iter().map(|c| c.rho.len()).sum::<usize>(),
            "rho_S": if rho_s.is_empty() { Value::Null } else { json!(mean(&rho_s)) },
            "ratios": Value::Object(ratios), "D2S_over_rho_W64": w64, "rho_AA": aa, "relation_phase_over_rho": Value::Object(relrho),
            "census": Value::Object(census),
        }));
    }
    // exponent fits: OLS of log2 cost on log2 ell, bootstrap within rung
    let series = |arm: &str, nmin: u32, nmax: u32, relonly: bool| -> BTreeMap<u32, Vec<(f64, f64)>> {
        let mut m: BTreeMap<u32, Vec<(f64, f64)>> = BTreeMap::new();
        for (&n, r) in &tree {
            if n > nmax || n < nmin {
                continue;
            }
            for ts in r.values() {
                for cd in ts.values() {
                    let y = if arm == "RHO" {
                        rho_mean(cd, "meq_s1_u1", None)
                    } else if relonly {
                        cd.rec.get(arm).map(|v| rel_meq(v, n))
                    } else {
                        cd.ic.get(arm).map(|m| m["meq_s1_u1"])
                    };
                    if let Some(y) = y {
                        m.entry(n).or_default().push((cd.ell.log2(), y.log2()));
                    }
                }
            }
        }
        m
    };
    let fit = |s: &BTreeMap<u32, Vec<(f64, f64)>>, rng: &mut Sm64| -> Option<(f64, f64, f64, f64)> {
        let pts: Vec<(f64, f64)> = s.values().flatten().copied().collect();
        if s.len() < 2 {
            return None;
        }
        let (b, a) = ols(&pts.iter().map(|p| p.0).collect::<Vec<_>>(), &pts.iter().map(|p| p.1).collect::<Vec<_>>());
        let mut bs = vec![];
        for _ in 0..2000 {
            let (mut xs, mut ys) = (vec![], vec![]);
            for v in s.values() {
                for _ in 0..v.len() {
                    let p = v[rng.below(v.len() as u64) as usize];
                    xs.push(p.0);
                    ys.push(p.1);
                }
            }
            bs.push(ols(&xs, &ys).0);
        }
        Some((b, pct(&mut bs.clone(), 0.025), pct(&mut bs, 0.975), a))
    };
    let mut fits = serde_json::Map::new();
    for (label, arm, nmin, nmax, relonly) in [
        ("RHO_all", "RHO", 0, 99, false),
        ("RHO_n>=36", "RHO", 36, 99, false),
        ("RHO_n>=48", "RHO", 48, 99, false),
        ("RHO_36..54", "RHO", 36, 54, false),
        ("DS_all", "DS", 0, 99, false),
        ("DS_n>=36", "DS", 36, 99, false),
        ("DS_n>=48", "DS", 48, 99, false),
        ("D2_n>=36", "D2", 36, 99, false),
        ("D2S_all", "D2S", 0, 99, false),
        ("D2S_n>=36", "D2S", 36, 99, false),
        ("D2S_n>=48", "D2S", 48, 99, false),
        ("D2S_36..54", "D2S", 36, 54, false),
        ("D2S_relation_phase_all", "D2S", 0, 99, true),
        ("D2S_relation_phase_n>=36", "D2S", 36, 99, true),
        ("D2S23_all", "D2S23", 0, 99, false),
        ("D2S23_n>=36", "D2S23", 36, 99, false),
        ("D2S23_n>=48", "D2S23", 48, 99, false),
    ] {
        let mut r2 = Sm64::keyed(&["boot", "b2lsp", "slope", label]);
        let s = series(arm, nmin, nmax, relonly);
        if let Some((b, lo, hi, a)) = fit(&s, &mut r2) {
            fits.insert(label.into(), json!({"slope": b, "lo": lo, "hi": hi, "intercept": a, "points": s.values().map(|v| v.len()).sum::<usize>()}));
        }
    }
    // slope difference D2S - rho on n >= 36 (paired rung bootstrap of both fits)
    let mut sd = serde_json::Map::new();
    for (a, rh, nmin) in [("D2S", "RHO", 36u32), ("D2S", "RHO", 48), ("D2S23", "RHO", 36)] {
        let (sa, sr) = (series(a, nmin, 99, false), series(rh, nmin, 99, false));
        if sa.len() < 2 || sr.len() < 2 {
            continue;
        }
        let mut r3 = Sm64::keyed(&["boot", "b2lsp", "slopediff", a, &nmin.to_string()]);
        let one = |s: &BTreeMap<u32, Vec<(f64, f64)>>, rg: &mut Sm64| {
            let (mut xs, mut ys) = (vec![], vec![]);
            for v in s.values() {
                for _ in 0..v.len() {
                    let p = v[rg.below(v.len() as u64) as usize];
                    xs.push(p.0);
                    ys.push(p.1);
                }
            }
            ols(&xs, &ys).0
        };
        let pa: Vec<(f64, f64)> = sa.values().flatten().copied().collect();
        let pr: Vec<(f64, f64)> = sr.values().flatten().copied().collect();
        let point = ols(&pa.iter().map(|p| p.0).collect::<Vec<_>>(), &pa.iter().map(|p| p.1).collect::<Vec<_>>()).0 - ols(&pr.iter().map(|p| p.0).collect::<Vec<_>>(), &pr.iter().map(|p| p.1).collect::<Vec<_>>()).0;
        let mut ds = vec![];
        for _ in 0..2000 {
            ds.push(one(&sa, &mut r3) - one(&sr, &mut r3));
        }
        sd.insert(format!("{a}-{rh}_n>={nmin}"), json!({"diff": point, "lo": pct(&mut ds.clone(), 0.025), "hi": pct(&mut ds, 0.975)}));
    }
    let proj = projection(&proto, &fit_rows, &fits);
    let s = json!({
        "schema": "b2lsp-summary/v1",
        "records": counts.iter().map(|(k, v)| (k.clone(), json!({"records": v.0, "failed": v.1}))).collect::<serde_json::Map<_, _>>(),
        "failed_records": bad,
        "checks": chk.iter().map(|(k, v)| (k.to_string(), json!(v))).collect::<serde_json::Map<_, _>>(),
        "identities": ident.iter().map(|(k, v)| (k.to_string(), json!(v))).collect::<serde_json::Map<_, _>>(),
        "rungs": rungs,
        "exponent_fits_log2cost_vs_log2ell": Value::Object(fits),
        "slope_differences": Value::Object(sd),
        "projection": proj,
        "bootstrap": {"B_primary": 10000, "B_band_and_other": 2000, "streams": "boot|b2lsp|main, boot|b2lsp|slope|<label>, boot|b2lsp|slopediff|<arm>|<nmin>"},
    });
    std::fs::write(out, serde_json::to_string_pretty(&s).unwrap()).unwrap();
    std::fs::write(svg, figure(&s)).unwrap();
    println!("{}", table(&s));
}

/// The 2LP counting shape for targets/F (#5291's post-freeze sensitivity):
/// targets/F = K·(c + α)/(1 − (1 − α)³), α = fs/F.
fn shape(a: f64, c: f64) -> f64 {
    (c + a) / (1.0 - (1.0 - a).powi(3))
}

/// Crossover projections (models, PROTOCOL.md part B "Crossover model").
fn projection(proto: &Value, rows: &[Value], fits: &serde_json::Map<String, Value>) -> Value {
    let g = |v: &Value, k: &str| v[k].as_f64().unwrap();
    let rho_rows: Vec<&Value> = rows.iter().filter(|r| r["arm"] == "RHO").collect();
    let top: Vec<&Value> = rho_rows.iter().rev().take(2).copied().collect();
    let s_top = mean(&top.iter().map(|r| g(r, "rho_S")).collect::<Vec<_>>());
    let hbar = mean(&rho_rows.iter().map(|r| g(r, "h")).collect::<Vec<_>>());
    let rho = |np: f64| s_top * (2f64.powf(3.0 * np) / hbar).sqrt();
    let d2: Vec<&Value> = rows.iter().filter(|r| r["arm"] == "D2S" || r["arm"] == "D2S23").collect();
    if d2.len() < 4 {
        return Value::Null;
    }
    // relation Meq per target = a + b n'
    let (cb, ca) = ols(&d2.iter().map(|r| g(r, "np")).collect::<Vec<_>>(), &d2.iter().map(|r| g(r, "relpt")).collect::<Vec<_>>());
    // targets/F: 2LP counting shape fitted on unsaturated rows (grid over c, K closed form)
    let unsat: Vec<(f64, f64)> = d2.iter().filter(|r| g(r, "fs") < g(r, "fb") - 1.5 && g(r, "fb") >= 256.0).map(|r| (g(r, "fs") / g(r, "fb"), g(r, "targets") / g(r, "fb"))).collect();
    let mut best = (f64::MAX, 0.0, 0.0);
    let mut c = 0.0;
    while c <= 20.0 && !unsat.is_empty() {
        let lk = mean(&unsat.iter().map(|&(a, t)| t.ln() - shape(a, c).ln()).collect::<Vec<_>>());
        let sse: f64 = unsat.iter().map(|&(a, t)| (t.ln() - lk - shape(a, c).ln()).powi(2)).sum();
        if sse < best.0 {
            best = (sse, c, lk.exp());
        }
        c += 0.005;
    }
    let (sse, cc, kk) = best;
    if unsat.len() < 2 {
        return json!({"model": "not fitted: fewer than 2 unsaturated D2S/D2S23 rows"});
    }
    // graph Meq per target, LA = 2^l0 fs^gamma (fs >= 256), and LA = kappa fs^2 (gamma fixed at 2)
    let gpt = mean(&d2.iter().map(|r| g(r, "graph") / g(r, "targets")).collect::<Vec<_>>());
    let big: Vec<&&Value> = d2.iter().filter(|r| g(r, "fs") >= 256.0 && g(r, "la") > 0.0).collect();
    let (gam, l0) = ols(&big.iter().map(|r| g(r, "fs").log2()).collect::<Vec<_>>(), &big.iter().map(|r| g(r, "la").log2()).collect::<Vec<_>>());
    let kappa2 = gmean(&big.iter().map(|r| g(r, "la") / g(r, "fs").powi(2)).collect::<Vec<_>>());
    let fbr = mean(&d2.iter().map(|r| g(r, "fb") / 2f64.powf(g(r, "np"))).collect::<Vec<_>>());
    let cost = |np: f64, fs: f64, la_mode: u8| -> (f64, f64, f64) {
        let f = fbr * 2f64.powf(np);
        let fs = fs.min(f - 1.0).max(1.0);
        let tg = f * kk * shape(fs / f, cc);
        let rel = tg * (ca + cb * np + gpt);
        let la = if la_mode == 0 { 2f64.powf(l0 + gam * fs.log2()) } else { kappa2 * fs * fs };
        (rel, la, rel + la)
    };
    let arms = &proto["arms_d2s"];
    let mut grid = vec![];
    let mut cross: BTreeMap<String, Option<f64>> = BTreeMap::new();
    let mut minr: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    let mut note = |k: String, v: f64, np: f64, e: &mut serde_json::Map<String, Value>| {
        e.insert(k.clone(), json!(v));
        if cross.get(&k).map(|x| x.is_none()).unwrap_or(true) {
            cross.insert(k.clone(), if v <= 1.0 { Some(np) } else { None });
        }
        let m = minr.entry(k).or_insert((f64::MAX, 0.0));
        if v < m.0 {
            *m = (v, np);
        }
    };
    let mut np = 6.0;
    while np <= 64.0 {
        let f = fbr * 2f64.powf(np);
        let r = rho(np);
        let mut e = serde_json::Map::new();
        e.insert("np".into(), json!(np));
        e.insert("n".into(), json!(3.0 * np));
        for arm in ["D2S", "D2S23"] {
            let (b, t) = (arms[arm]["beta"].as_f64().unwrap(), arms[arm]["theta"].as_f64().unwrap());
            let fs = (t * f.powf(b)).ceil();
            for (lab, mode) in [("la_fit", 0u8), ("la_fs2", 1u8)] {
                let (rel, _la, tot) = cost(np, fs, mode);
                note(format!("{arm}_{lab}_over_rho"), tot / r, np, &mut e);
                e.insert(format!("{arm}_relation_over_rho"), json!(rel / r));
            }
        }
        for (lab, mode) in [("la_fit", 0u8), ("la_fs2", 1u8)] {
            let mut bst = (f64::MAX, 0.0);
            let mut fs = 16.0;
            while fs < f {
                let (_, _, tot) = cost(np, fs, mode);
                if tot < bst.0 {
                    bst = (tot, fs);
                }
                fs *= 1.05;
            }
            note(format!("rebalanced_{lab}_over_rho"), bst.0 / r, np, &mut e);
            e.insert(format!("rebalanced_{lab}_fs"), json!(bst.1));
        }
        if np.fract() == 0.0 {
            grid.push(Value::Object(e));
        }
        np += 0.25;
    }
    let mut lines = serde_json::Map::new();
    for (a, rh) in [("D2S_n>=36", "RHO_n>=36"), ("D2S_n>=48", "RHO_n>=48"), ("D2S23_n>=36", "RHO_n>=36"), ("D2S_36..54", "RHO_36..54")] {
        if let (Some(x), Some(y)) = (fits.get(a), fits.get(rh)) {
            let (sa, ia, sr, ir) = (g(x, "slope"), g(x, "intercept"), g(y, "slope"), g(y, "intercept"));
            let l2 = if sr > sa { Some((ia - ir) / (sr - sa)) } else { None };
            lines.insert(format!("{a} vs {rh}"), json!({"slope_arm": sa, "slope_rho": sr, "crossover_log2_ell": l2, "crossover_n_approx": l2.map(|l| l + 1.5)}));
        }
    }
    json!({
        "model": "rho(n') = S_top*sqrt(2^(3n')/h), S_top = mean S of the top two rho rungs; F = fb_ratio*2^n'; targets = F*K*(c+a)/(1-(1-a)^3), a = fs/F, fitted on unsaturated D2S/D2S23 rows with F >= 256; relation + graph = targets*(alpha + beta*n' + g); LA either 2^l0*fs^gamma (fit, fs >= 256) or kappa*fs^2 (gamma fixed at 2); frozen rules and fs re-optimised at each n'",
        "fit": {"alpha": ca, "beta": cb, "graph_meq_per_target": gpt, "targets_shape": {"K": kk, "c": cc, "rms_log_residual": (sse / unsat.len().max(1) as f64).sqrt(), "rows": unsat.len()},
                "la_fit": {"log2_coeff": l0, "gamma": gam, "rows": big.len()}, "la_fs2_kappa": kappa2, "fb_over_qp": fbr, "rho_S_top": s_top, "h_mean": hbar},
        "crossover_np": cross,
        "min_ratio": minr.iter().map(|(k, v)| (k.clone(), json!({"ratio": v.0, "np": v.1}))).collect::<serde_json::Map<_, _>>(),
        "slope_lines": Value::Object(lines),
        "grid": grid,
    })
}

fn fmt(v: &Value) -> String {
    if v.is_null() {
        return "—".into();
    }
    let f = |x: f64| if x >= 1000.0 || x < 0.01 { format!("{:.3e}", x) } else if x >= 10.0 { format!("{:.1}", x) } else { format!("{:.3}", x) };
    format!("{} [{}, {}]", f(v["ratio"].as_f64().unwrap()), f(v["lo"].as_f64().unwrap()), f(v["hi"].as_f64().unwrap()))
}

fn table(s: &Value) -> String {
    let mut t = String::from("| n | cases | D2S/rho | D2S23/rho | DS/rho | D2/rho | D2S/DS | D2S/D2 | D2S23/D2S | rho A/A | D2S fs/F | D2S rel/graph/LA | D2S weight | cycle len mean/max |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in s["rungs"].as_array().unwrap() {
        let g = |k: &str| fmt(&r["ratios"][k]["meq_s1_u1"]);
        let c = &r["census"]["D2S"];
        let cs = |f: &dyn Fn(&Value) -> String| if c.is_null() { "—".to_string() } else { f(c) };
        t += &format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            r["n"], r["cases"], g("D2S/rho"), g("D2S23/rho"), g("DS/rho"), g("D2/rho"), g("D2S/DS"), g("D2S/D2"), g("D2S23/D2S"), fmt(&r["rho_AA"]),
            cs(&|c| format!("{:.0}/{:.0}", c["fs"].as_f64().unwrap(), c["fb"].as_f64().unwrap())),
            cs(&|c| format!("{:.2}/{:.3}/{:.2}", c["relation_share"].as_f64().unwrap(), c["graph_share"].as_f64().unwrap(), c["la_share"].as_f64().unwrap())),
            cs(&|c| format!("{:.1}", c["graph_yield_weight_mean"].as_f64().unwrap())),
            cs(&|c| format!("{:.1}/{:.0}", c["graph_cycle_len_mean"].as_f64().unwrap(), c["graph_cycle_len_max"].as_f64().unwrap())),
        );
    }
    t
}

fn figure(s: &Value) -> String {
    let (w, h) = (860.0, 520.0);
    let (x0, x1, y0, y1) = (70.0, 660.0, 34.0, 440.0);
    let rungs = s["rungs"].as_array().unwrap();
    let (nmin, nmax) = (18.0, 150.0);
    let (lmin, lmax) = (-1.0f64, 3.0f64);
    let px = |n: f64| x0 + (n - nmin) / (nmax - nmin) * (x1 - x0);
    let py = |v: f64| y1 - (v.log10().clamp(lmin, lmax) - lmin) / (lmax - lmin) * (y1 - y0);
    let mut o = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
    o += &format!("<text x='{}' y='20' font-size='14'>Cold single-target cost / matched negation-map rho (Meq, s=1, u=1), 95% CI [pcc:unverified]</text>", x0);
    let mut e = lmin;
    while e <= lmax {
        let y = py(10f64.powf(e));
        o += &format!("<line x1='{x0}' x2='{x1}' y1='{y}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>1e{}</text>", x0 - 6.0, y + 4.0, e);
        e += 1.0;
    }
    let yp = py(1.0);
    o += &format!("<line x1='{x0}' x2='{x1}' y1='{yp}' y2='{yp}' stroke='#000' stroke-dasharray='6,3'/><text x='{}' y='{}'>parity</text>", x1 + 4.0, yp + 4.0);
    let mut n = nmin;
    while n <= nmax {
        o += &format!("<text x='{}' y='{}' text-anchor='middle'>{}</text>", px(n), y1 + 16.0, n);
        n += 12.0;
    }
    let nlast = rungs.iter().filter(|r| !r["ratios"]["D2S/rho"].is_null()).map(|r| r["n"].as_f64().unwrap()).fold(0.0, f64::max);
    o += &format!("<line x1='{0}' x2='{0}' y1='{y0}' y2='{y1}' stroke='#bbb'/>", px(nlast));
    o += &format!("<text x='{}' y='{}' text-anchor='middle'>n (F_2^n, subfield n' = n/3); right of n = {}: model only (dashed)</text>", (x0 + x1) / 2.0, y1 + 34.0, nlast);
    let series = [
        ("D2/rho", "#7f7f7f", "D2 (2LP, echelon, #5291)"),
        ("DS/rho", "#ff7f0e", "D-S (full base, sparse LA, #5290)"),
        ("D2S23/rho", "#1f77b4", "D2S23 (2LP + BFS + sparse, beta = 2/3)"),
        ("D2S/rho", "#d62728", "D2S (2LP + BFS + sparse, pilot rule)"),
    ];
    for (k, (key, col, lab)) in series.iter().enumerate() {
        let mut pts = vec![];
        for r in rungs {
            let v = &r["ratios"][*key]["meq_s1_u1"];
            if v.is_null() {
                continue;
            }
            let n = r["n"].as_f64().unwrap();
            let (p, lo, hi) = (v["ratio"].as_f64().unwrap(), v["lo"].as_f64().unwrap(), v["hi"].as_f64().unwrap());
            o += &format!("<line x1='{0}' x2='{0}' y1='{1}' y2='{2}' stroke='{col}'/>", px(n), py(lo), py(hi));
            o += &format!("<circle cx='{}' cy='{}' r='3.2' fill='{col}'/>", px(n), py(p));
            pts.push(format!("{},{}", px(n), py(p)));
        }
        o += &format!("<polyline points='{}' fill='none' stroke='{col}' stroke-width='1.5'/>", pts.join(" "));
        o += &format!("<rect x='{}' y='{}' width='12' height='3' fill='{col}'/><text x='{}' y='{}'>{lab}</text>", x1 - 300.0, y0 + 14.0 + 16.0 * k as f64, x1 - 282.0, y0 + 18.0 + 16.0 * k as f64);
    }
    if let Some(gr) = s["projection"]["grid"].as_array() {
        for (key, col) in [("D2S_la_fit_over_rho", "#d62728"), ("D2S23_la_fit_over_rho", "#1f77b4"), ("rebalanced_la_fit_over_rho", "#2ca02c")] {
            let pts: Vec<String> = gr
                .iter()
                .filter(|e| e["n"].as_f64().unwrap() >= nlast && e["n"].as_f64().unwrap() <= nmax)
                .map(|e| format!("{},{}", px(e["n"].as_f64().unwrap()), py(e[key].as_f64().unwrap())))
                .collect();
            o += &format!("<polyline points='{}' fill='none' stroke='{col}' stroke-dasharray='7,4' stroke-width='1.3'/>", pts.join(" "));
        }
        o += &format!("<text x='{}' y='{}' fill='#2ca02c'>dashed green: base re-balanced per n', fitted LA (model)</text>", x1 - 300.0, y0 + 18.0 + 16.0 * 4.0);
        o += &format!("<text x='{}' y='{}' fill='#555'>dashed red/blue: frozen rules, fitted LA (model)</text>", x1 - 300.0, y0 + 18.0 + 16.0 * 5.0);
    }
    o += "</svg>";
    o
}

/// Part A pilot rule (PROTOCOL.md): α̂ per rung by a parabola in ln α, then β̂, θ̂ by OLS.
pub fn pilot_summary(path: &str) {
    let mut m: BTreeMap<u64, BTreeMap<String, (f64, Vec<f64>, f64)>> = BTreeMap::new();
    let mut bad = 0;
    for l in std::fs::read_to_string(path).unwrap().lines() {
        let v: Value = serde_json::from_str(l).unwrap();
        if v["correct"].as_bool() != Some(true) || v["verified"].as_bool() != Some(true) {
            bad += 1;
            continue;
        }
        let a = v["alpha"].as_f64().unwrap();
        let e = m.entry(v["case"]["n"].as_u64().unwrap()).or_default().entry(format!("{a:.4}")).or_insert((a, vec![], v["fb"].as_f64().unwrap()));
        e.1.push(v["cost"]["meq_s1_u1"].as_f64().unwrap());
    }
    println!("alpha pilot: {bad} failed/incorrect records");
    let (mut xs, mut ys) = (vec![], vec![]);
    let mut fstars = vec![];
    for (n, g) in &m {
        let mut pts: Vec<(f64, f64, usize, f64)> = g.values().map(|(a, v, f)| (*a, gmean(v).ln(), v.len(), *f)).collect();
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for p in &pts {
            println!("n={n} alpha={:.3} gmean={:.4e} k={} S={:.4}", p.0, p.1.exp(), p.2, p.1);
        }
        let mut k = 0;
        for i in 1..pts.len() {
            if pts[i].1 < pts[k].1 {
                k = i;
            }
        }
        let ah = if k > 0 && k + 1 < pts.len() {
            let (x0, x1, x2) = (pts[k - 1].0.ln(), pts[k].0.ln(), pts[k + 1].0.ln());
            let (y0, y1, y2) = (pts[k - 1].1, pts[k].1, pts[k + 1].1);
            let den = (x0 - x1) * (x0 - x2) * (x1 - x2);
            let a = (x2 * (y1 - y0) + x1 * (y0 - y2) + x0 * (y2 - y1)) / den;
            let b = (x2 * x2 * (y0 - y1) + x1 * x1 * (y2 - y0) + x0 * x0 * (y1 - y2)) / den;
            let xv = if a > 0.0 { (-b / (2.0 * a)).clamp(x0, x2) } else { x1 };
            xv.exp()
        } else {
            pts[k].0
        };
        let f = pts[0].3;
        let fs = ah * (f - 1.0);
        println!("n={n}: argmin alpha={:.3}, alpha_hat={ah:.4}, F={f}, fs*={fs:.1}", pts[k].0);
        xs.push(f.ln());
        ys.push(fs.ln());
        fstars.push((f, fs));
    }
    let (mut b, a) = ols(&xs, &ys);
    let mut th = a.exp();
    let raw = (b, th);
    if !(0.5..=1.0).contains(&b) {
        b = b.clamp(0.5, 1.0);
        th = gmean(&fstars.iter().map(|&(f, fs)| fs / f.powf(b)).collect::<Vec<_>>());
    }
    let th23 = gmean(&fstars.iter().map(|&(f, fs)| fs / f.powf(2.0 / 3.0)).collect::<Vec<_>>());
    println!("OLS raw beta={:.4} theta={:.4}; D2S: beta={b:.4} theta={th:.4}; D2S23: beta=2/3 theta={th23:.4}", raw.0, raw.1);
    println!("{}", json!({"D2S": {"beta": b, "theta": th}, "D2S23": {"beta": 2.0 / 3.0, "theta": th23}, "raw": {"beta": raw.0, "theta": raw.1}}));
}

/// Predictions: the abstract simulation's targets, yields and counted graph/LA
/// units, priced with #5291's measured D2 relation Meq per target (same oracle;
/// α + βn′ fit beyond n = 54) and #5291's rho S per rung (S_top beyond).
pub fn predict_summary(pred: &str, b2lp_summary: &str) {
    let b: Value = serde_json::from_str(&std::fs::read_to_string(b2lp_summary).unwrap()).unwrap();
    let mut relpt: BTreeMap<u64, (f64, f64, f64)> = BTreeMap::new();
    for r in b["rungs"].as_array().unwrap() {
        let c = &r["census"]["D2"];
        relpt.insert(r["n"].as_u64().unwrap(), (c["relation_meq_per_target"].as_f64().unwrap(), r["rho_S"].as_f64().unwrap(), r["log2_ell"].as_f64().unwrap()));
    }
    let p = &b["projection"]["fit"];
    let (al, be, stop) = (p["alpha"].as_f64().unwrap(), p["beta"].as_f64().unwrap(), p["rho_S_top"].as_f64().unwrap());
    let mut m: BTreeMap<(String, u64), Vec<Value>> = BTreeMap::new();
    for l in std::fs::read_to_string(pred).unwrap().lines() {
        let v: Value = serde_json::from_str(l).unwrap();
        m.entry((format!("{:.4}/{:.4}", v["beta"].as_f64().unwrap(), v["theta"].as_f64().unwrap()), v["n"].as_u64().unwrap())).or_default().push(v);
    }
    println!("| arm (beta/theta) | n | F | fs | targets/q' | yields | weight | cycle len mean/max | graph Meq | LA Meq | LA/fs^2 | total Meq | pred /rho | LA share |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    let mut fitp: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for ((arm, n), vs) in &m {
        let np = *n / 3;
        let g = |k: &str| mean(&vs.iter().map(|v| v[k].as_f64().unwrap()).collect::<Vec<_>>());
        let (rp, s, l2) = relpt.get(n).copied().unwrap_or((al + be * np as f64, stop, 3.0 * np as f64 - 3.38f64.log2()));
        let tg = g("targets");
        let tot = tg * rp + g("graph_units") + g("la_units");
        let rho = s * 2f64.powf(l2 / 2.0);
        fitp.entry(arm.clone()).or_default().push((l2, tot.log2()));
        println!(
            "| {arm} | {n} | {:.0} | {:.0} | {:.2} | {:.0} | {:.1} | {:.1}/{:.0} | {:.3e} | {:.3e} | {:.2} | {:.3e} | {:.1} | {:.2} |",
            g("F"), g("fs"), g("targets_over_qp"), g("yields"), g("yield_weight_mean"), g("cycle_len_mean"), g("cycle_len_max"),
            g("graph_units"), g("la_units"), g("la_units") / g("fs").powi(2), tot, tot / rho, g("la_units") / tot
        );
    }
    for (arm, p) in &fitp {
        let big: Vec<&(f64, f64)> = p.iter().filter(|x| x.0 >= 33.0).collect();
        if big.len() >= 2 {
            let (sl, _) = ols(&big.iter().map(|x| x.0).collect::<Vec<_>>(), &big.iter().map(|x| x.1).collect::<Vec<_>>());
            println!("predicted slope (log2 cost on log2 ell, n >= 36) {arm}: {sl:.3}");
        }
    }
}
