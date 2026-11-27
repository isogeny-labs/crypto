//! Frozen analysis (PROTOCOL.md part B): paired D2/rho, D2e/rho, D/rho,
//! D2/D, D2e/D, D2/D2e per rung with a hierarchical bootstrap; the §5a band;
//! the rho A/A floor; D2[all-small] ≡ D identity; phase census; exponent
//! fits; crossover projections (model); the SVG figure.  Also the pilot
//! summary (part A rule).
use crate::seeds::Sm64;
use serde_json::{json, Value};
use std::collections::BTreeMap;

const ARMS: [&str; 4] = ["D2", "D2e", "D", "D2all"];
const KEYS: [&str; 5] = ["meq_s1_u1", "meq_s1_u01", "meq_s1_u0", "meq_s025_u1", "meq_s025_u0"];
const REL_PHASES: [&str; 7] = ["fb", "setup", "targets", "descent", "rootfind", "lift", "resolve"];

#[derive(Default, Clone)]
struct Case {
    ell: f64,
    h: f64,
    rho: Vec<BTreeMap<String, f64>>,
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
    let curves: Vec<&BTreeMap<u32, Case>> = r.values().collect();
    let mut boots = Vec::with_capacity(b);
    for _ in 0..b {
        let mut v = vec![];
        for _ in 0..curves.len() {
            let cv = curves[rng.below(curves.len() as u64) as usize];
            let ts: Vec<&Case> = cv.values().collect();
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
    let o = crate::field::Ops::from_json(&rec["phases"][phase]);
    crate::field::meq(&o, n, ws, wu)
}
fn rel_meq(rec: &Value, n: u32) -> f64 {
    REL_PHASES.iter().map(|p| phase_meq(rec, p, n, 1.0, 1.0)).sum()
}

pub fn analyze(protocol: &str, runs: &str, out: &str, svg: &str) {
    let proto: Value = serde_json::from_str(&std::fs::read_to_string(protocol).unwrap()).unwrap();
    let text = std::fs::read_to_string(runs).unwrap();
    let mut tree: BTreeMap<u32, Rung> = BTreeMap::new();
    let mut counts: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let mut bad = vec![];
    let (mut ychk, mut ybad, mut unres, mut capped) = (0u64, 0u64, 0u64, 0u64);
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).unwrap();
        let arm = v["arm"].as_str().unwrap_or("").to_string();
        if arm != "RHO" && !ARMS.contains(&arm.as_str()) {
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
        if arm.starts_with("D2") {
            ychk += v["yield_checks"].as_u64().unwrap_or(0);
            ybad += v["yield_check_failures"].as_u64().unwrap_or(0);
            if v["capped"].as_bool() == Some(true) {
                capped += 1;
            }
        }
        if arm != "RHO" {
            unres += v["rows_unresolved"].as_u64().unwrap_or(0);
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
        } else {
            cd.ic.insert(arm.clone(), costs);
            cd.rec.insert(arm, v.clone());
        }
    }
    let mut rng = Sm64::keyed(&["boot", "b2lp", "main"]);
    let mut rungs = vec![];
    let (mut id_cases, mut id_equal, mut dd_cases, mut dd_equal) = (0u64, 0u64, 0u64, 0u64);
    let mut fit_rows: Vec<Value> = vec![];
    for (&n, r) in &tree {
        let np = n / 3;
        let all: Vec<&Case> = r.values().flat_map(|t| t.values()).collect();
        let ell = gmean(&all.iter().map(|c| c.ell).collect::<Vec<_>>());
        let rho_s: Vec<f64> = all.iter().filter_map(|c| rho_mean(c, "meq_s1_u1", None).map(|m| m / c.ell.sqrt())).collect();
        let mut ratios = serde_json::Map::new();
        let pairs: Vec<(String, &str, Option<&str>)> = vec![
            ("D2/rho".into(), "D2", None),
            ("D2e/rho".into(), "D2e", None),
            ("D/rho".into(), "D", None),
            ("D2/D".into(), "D2", Some("D")),
            ("D2e/D".into(), "D2e", Some("D")),
            ("D2/D2e".into(), "D2", Some("D2e")),
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
            if !band.is_empty() {
                ratios.insert(label.clone(), Value::Object(band));
            }
        }
        let aa = stat(
            r,
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[..cd.rho.len() / 2].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[cd.rho.len() / 2..].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &mut rng,
            2000,
        );
        let mut relrho = serde_json::Map::new();
        for a in ["D2", "D2e", "D"] {
            if let Some(s) = stat(r, &|cd, _| cd.rec.get(a).map(|v| rel_meq(v, n)), &|cd, ix| rho_mean(cd, "meq_s1_u1", ix), &mut rng, 2000) {
                relrho.insert(a.into(), s);
            }
        }
        let mut census = serde_json::Map::new();
        for a in ["D2", "D2e", "D", "D2all"] {
            let recs: Vec<&Value> = all.iter().filter_map(|c| c.rec.get(a)).collect();
            if recs.is_empty() {
                continue;
            }
            let per = |f: &dyn Fn(&Value) -> f64| mean(&recs.iter().map(|v| f(v)).collect::<Vec<_>>());
            let tg = per(&|v| v["targets"].as_f64().unwrap());
            let rel = per(&|v| rel_meq(v, n));
            let gr = per(&|v| if v["phases"]["graph"].is_null() { 0.0 } else { phase_meq(v, "graph", n, 1.0, 1.0) });
            let la = per(&|v| phase_meq(v, "la", n, 1.0, 1.0));
            let tot = per(&|v| v["cost"]["meq_s1_u1"].as_f64().unwrap());
            let relpt = per(&|v| rel_meq(v, n) / v["targets"].as_f64().unwrap());
            let fbn = per(&|v| v["fb"].as_f64().unwrap());
            let fs = per(&|v| v["fs"].as_f64().unwrap_or(v["fb"].as_f64().unwrap() - 1.0));
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
            m.insert("total_meq".into(), json!(tot));
            m.insert("relation_share".into(), json!(rel / tot));
            m.insert("graph_share".into(), json!(gr / tot));
            m.insert("la_share".into(), json!(la / tot));
            m.insert("wall_s".into(), json!(per(&|v| v["wall_ns"].as_f64().unwrap() / 1e9)));
            if a != "D" {
                for k in ["yields", "small_columns", "echelon_rows_in", "echelon_useless_dependencies"] {
                    m.insert(k.into(), json!(per(&|v| v[k].as_f64().unwrap_or(0.0))));
                }
                for k in ["full", "one_lp", "two_lp", "discarded_3lp", "yields_cycle", "yields_join", "groundings", "tree_edges", "vertices_touched", "reduce_steps", "max_reduce_steps", "yield_weight_mean", "components", "grounded_components"] {
                    m.insert(format!("graph_{k}"), json!(per(&|v| v["graph"][k].as_f64().unwrap_or(0.0))));
                }
            }
            if a == "D2" || a == "D2e" {
                fit_rows.push(json!({"arm": a, "n": n, "np": np, "fb": fbn, "fs": fs, "targets": tg, "relpt": relpt, "graph": gr, "la": la,
                    "rho_S": if rho_s.is_empty() { Value::Null } else { json!(mean(&rho_s)) }, "h": mean(&all.iter().map(|c| c.h).collect::<Vec<_>>())}));
            }
            census.insert(a.into(), Value::Object(m));
        }
        if rho_s.len() > 0 && census.get("D2").is_none() && census.get("D").is_none() {
            // rho-only rung (none expected)
        }
        if !rho_s.is_empty() {
            fit_rows.push(json!({"arm": "RHO", "n": n, "rho_S": mean(&rho_s), "h": mean(&all.iter().map(|c| c.h).collect::<Vec<_>>())}));
        }
        for ts in r.values() {
            for cd in ts.values() {
                if let (Some(a), Some(b)) = (cd.rec.get("D2all"), cd.rec.get("D")) {
                    id_cases += 1;
                    if a["d"] == b["d"] && a["targets"] == b["targets"] && a["relations"] == b["relations"] && a["columns"] == b["columns"] {
                        id_equal += 1;
                    }
                }
                if let Some(b) = cd.rec.get("D") {
                    for a in ["D2", "D2e"] {
                        if let Some(x) = cd.rec.get(a) {
                            dd_cases += 1;
                            if x["d"] == b["d"] {
                                dd_equal += 1;
                            }
                        }
                    }
                }
            }
        }
        rungs.push(json!({
            "n": n, "np": np, "log2_ell": ell.log2(), "cases": all.len(),
            "rho_runs": all.iter().map(|c| c.rho.len()).sum::<usize>(),
            "rho_S": if rho_s.is_empty() { Value::Null } else { json!(mean(&rho_s)) },
            "ratios": Value::Object(ratios), "rho_AA": aa, "relation_phase_over_rho": Value::Object(relrho),
            "census": Value::Object(census),
        }));
    }
    // exponent fits
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
        ("D_all(n<=48)", "D", 0, 99, false),
        ("D2_all", "D2", 0, 99, false),
        ("D2_n>=36", "D2", 36, 99, false),
        ("D2_n>=48", "D2", 48, 99, false),
        ("D2_relation_phase_all", "D2", 0, 99, true),
        ("D2e_all", "D2e", 0, 99, false),
        ("D2e_n>=36", "D2e", 36, 99, false),
        ("D2e_n>=48", "D2e", 48, 99, false),
        ("D2e_relation_phase_all", "D2e", 0, 99, true),
    ] {
        let mut r2 = Sm64::keyed(&["boot", "b2lp", "slope", label]);
        let s = series(arm, nmin, nmax, relonly);
        if let Some((b, lo, hi, a)) = fit(&s, &mut r2) {
            fits.insert(label.into(), json!({"slope": b, "lo": lo, "hi": hi, "intercept": a, "points": s.values().map(|v| v.len()).sum::<usize>()}));
        }
    }
    let proj = projection(&proto, &fit_rows, &fits);
    let s = json!({
        "schema": "b2lp-summary/v1",
        "records": counts.iter().map(|(k, v)| (k.clone(), json!({"records": v.0, "failed": v.1}))).collect::<serde_json::Map<_, _>>(),
        "failed_records": bad,
        "D2all_vs_D_identity": {"cases": id_cases, "equal_d_targets_relations_columns": id_equal},
        "D2_vs_D_log_crosscheck": {"cases": dd_cases, "equal_d": dd_equal},
        "yield_group_law_checks": {"checked": ychk, "failures": ybad},
        "rows_unresolved": unres, "d2_capped_records": capped,
        "rungs": rungs,
        "exponent_fits_log2cost_vs_log2ell": Value::Object(fits),
        "projection": proj,
        "bootstrap": {"B_primary": 10000, "B_band_and_other": 2000, "streams": "boot|b2lp|main, boot|b2lp|slope|<label>"},
    });
    std::fs::write(out, serde_json::to_string_pretty(&s).unwrap()).unwrap();
    std::fs::write(svg, figure(&s)).unwrap();
    println!("{}", table(&s));
}

/// Crossover projections (models, PROTOCOL.md "Crossover model").
fn projection(proto: &Value, rows: &[Value], fits: &serde_json::Map<String, Value>) -> Value {
    let g = |v: &Value, k: &str| v[k].as_f64().unwrap();
    let rho_rows: Vec<&Value> = rows.iter().filter(|r| r["arm"] == "RHO").collect();
    let top: Vec<&Value> = rho_rows.iter().rev().take(2).copied().collect();
    let s_top = mean(&top.iter().map(|r| g(r, "rho_S")).collect::<Vec<_>>());
    let hbar = mean(&rho_rows.iter().map(|r| g(r, "h")).collect::<Vec<_>>());
    let rho = |np: f64| s_top * (2f64.powf(3.0 * np) / hbar).sqrt();
    let mut outm = serde_json::Map::new();
    // pooled fits over D2 and D2e: relation Meq per target c(n') = α + βn';
    // targets/F = A·(fs/F)^p·F^r (log-linear); LA echelon = 2^{l0}·fs^γ (fs ≥ 256);
    // graph Meq per target-relation proportional to targets (g per target).
    let d2: Vec<&Value> = rows.iter().filter(|r| r["arm"] == "D2" || r["arm"] == "D2e").collect();
    if d2.len() < 4 {
        return Value::Null;
    }
    let (cb, ca) = ols(&d2.iter().map(|r| g(r, "np")).collect::<Vec<_>>(), &d2.iter().map(|r| g(r, "relpt")).collect::<Vec<_>>());
    // targets/F on log(fs/F) and log F (multiple regression by normal equations)
    let unsat: Vec<&&Value> = d2.iter().filter(|r| g(r, "fs") < g(r, "fb") - 1.5).collect();
    let xs: Vec<(f64, f64, f64)> = unsat.iter().map(|r| ((g(r, "fs") / g(r, "fb")).ln(), g(r, "fb").ln(), (g(r, "targets") / g(r, "fb")).ln())).collect();
    let (p_alpha, p_f, a0) = ols2(&xs);
    let big: Vec<&&Value> = d2.iter().filter(|r| g(r, "fs") >= 256.0 && g(r, "la") > 0.0).collect();
    let (gam, l0) = ols(&big.iter().map(|r| g(r, "fs").log2()).collect::<Vec<_>>(), &big.iter().map(|r| g(r, "la").log2()).collect::<Vec<_>>());
    let gpt = mean(&d2.iter().map(|r| g(r, "graph") / g(r, "targets")).collect::<Vec<_>>());
    let wbar = 12.0; // yield weight used by the sparse-LA model (census mean, rounded; PROTOCOL.md)
    let fbr = mean(&d2.iter().map(|r| g(r, "fb") / 2f64.powf(g(r, "np"))).collect::<Vec<_>>());
    let cost = |np: f64, fs: f64, la_sparse: bool| -> (f64, f64, f64) {
        let f = fbr * 2f64.powf(np);
        let fs = fs.min(f - 1.0);
        let tg = f * (a0 + p_alpha * (fs / f).ln() + p_f * f.ln()).exp();
        let rel = tg * (ca + cb * np + gpt);
        let la = if la_sparse { 3.0 * wbar * fs * fs } else { 2f64.powf(l0 + gam * fs.log2()) };
        (rel, la, rel + la)
    };
    let arms = &proto["arms_d2"];
    let mut grid = vec![];
    let mut cross: BTreeMap<String, Option<f64>> = BTreeMap::new();
    let mut minr: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    let mut np = 6.0;
    while np <= 64.0 {
        let f = fbr * 2f64.powf(np);
        let r = rho(np);
        let mut e = serde_json::Map::new();
        e.insert("np".into(), json!(np));
        e.insert("n".into(), json!(3.0 * np));
        for arm in ["D2", "D2e"] {
            let (b, t) = (arms[arm]["beta"].as_f64().unwrap(), arms[arm]["theta"].as_f64().unwrap());
            let fs = (t * f.powf(b)).ceil();
            for (lab, sp) in [("echelon_fit", false), ("sparse_model", true)] {
                let (rel, _la, tot) = cost(np, fs, sp);
                let k = format!("{arm}_{lab}_over_rho");
                e.insert(k.clone(), json!(tot / r));
                e.insert(format!("{arm}_relation_over_rho"), json!(rel / r));
                if cross.get(&k).map(|x| x.is_none()).unwrap_or(true) {
                    cross.insert(k.clone(), if tot / r <= 1.0 { Some(np) } else { None });
                }
                let m = minr.entry(k).or_insert((f64::MAX, 0.0));
                if tot / r < m.0 {
                    *m = (tot / r, np);
                }
            }
        }
        // re-balanced base at every n' (model): fs minimising the sparse-model cost
        let mut best = (f64::MAX, 0.0);
        let mut fs = 16.0;
        while fs < f {
            let (_, _, tot) = cost(np, fs, true);
            if tot < best.0 {
                best = (tot, fs);
            }
            fs *= 1.1;
        }
        let k = "rebalanced_sparse_model_over_rho".to_string();
        e.insert(k.clone(), json!(best.0 / r));
        e.insert("rebalanced_fs".into(), json!(best.1));
        if cross.get(&k).map(|x| x.is_none()).unwrap_or(true) {
            cross.insert(k.clone(), if best.0 / r <= 1.0 { Some(np) } else { None });
        }
        let m = minr.entry(k).or_insert((f64::MAX, 0.0));
        if best.0 / r < m.0 {
            *m = (best.0 / r, np);
        }
        if (np * 4.0).round() % 4.0 == 0.0 {
            grid.push(Value::Object(e));
        }
        np += 0.25;
    }
    // straight-line extrapolation of the measured slopes
    let mut lines = serde_json::Map::new();
    for (a, rh) in [("D2_n>=48", "RHO_n>=48"), ("D2e_n>=48", "RHO_n>=48"), ("D2_n>=36", "RHO_n>=36"), ("D2e_n>=36", "RHO_n>=36")] {
        if let (Some(x), Some(y)) = (fits.get(a), fits.get(rh)) {
            let (sa, ia, sr, ir) = (g(x, "slope"), g(x, "intercept"), g(y, "slope"), g(y, "intercept"));
            let l2 = if sr > sa { Some((ia - ir) / (sr - sa)) } else { None };
            lines.insert(format!("{a} vs {rh}"), json!({"slope_arm": sa, "slope_rho": sr, "crossover_log2_ell": l2, "crossover_n_approx": l2.map(|l| l + 1.5)}));
        }
    }
    outm.insert("model".into(), json!("rho(n') = S_top*sqrt(2^(3n')/h), S_top = mean S of the top two rho rungs; F = fb_ratio*2^n'; targets = F*exp(a0 + p_alpha*ln(fs/F) + p_F*ln F) fitted on all unsaturated D2/D2e cases; relation phase + graph = targets*(alpha + beta*n' + g); LA echelon = 2^l0*fs^gamma (fit fs >= 256); LA sparse model = 3*wbar*fs^2 mod-ell units (Wiedemann-type, not measured, wbar = 12); rebalanced: fs re-optimised at each n' under the sparse model"));
    outm.insert("fit".into(), json!({"alpha": ca, "beta": cb, "graph_meq_per_target": gpt, "targets_over_F": {"a0": a0, "p_alpha": p_alpha, "p_F": p_f, "cases": xs.len()},
        "la_echelon": {"log2_coeff": l0, "gamma": gam, "cases": big.len()}, "fb_over_qp": fbr, "rho_S_top": s_top, "h_mean": hbar, "wbar": wbar}));
    outm.insert("crossover_np".into(), json!(cross));
    outm.insert("min_ratio".into(), json!(minr.iter().map(|(k, v)| (k.clone(), json!({"ratio": v.0, "np": v.1}))).collect::<serde_json::Map<_, _>>()));
    outm.insert("slope_lines".into(), Value::Object(lines));
    outm.insert("grid".into(), json!(grid));
    Value::Object(outm)
}

/// OLS of z on (x, y) with intercept: returns (bx, by, a).
fn ols2(p: &[(f64, f64, f64)]) -> (f64, f64, f64) {
    let n = p.len() as f64;
    let (mx, my, mz) = (p.iter().map(|t| t.0).sum::<f64>() / n, p.iter().map(|t| t.1).sum::<f64>() / n, p.iter().map(|t| t.2).sum::<f64>() / n);
    let (mut sxx, mut syy, mut sxy, mut sxz, mut syz) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for t in p {
        let (x, y, z) = (t.0 - mx, t.1 - my, t.2 - mz);
        sxx += x * x;
        syy += y * y;
        sxy += x * y;
        sxz += x * z;
        syz += y * z;
    }
    let det = sxx * syy - sxy * sxy;
    let bx = (sxz * syy - syz * sxy) / det;
    let by = (syz * sxx - sxz * sxy) / det;
    (bx, by, mz - bx * mx - by * my)
}

fn fmt(v: &Value) -> String {
    if v.is_null() {
        return "—".into();
    }
    let f = |x: f64| if x >= 1000.0 || x < 0.01 { format!("{:.3e}", x) } else if x >= 10.0 { format!("{:.1}", x) } else { format!("{:.3}", x) };
    format!("{} [{}, {}]", f(v["ratio"].as_f64().unwrap()), f(v["lo"].as_f64().unwrap()), f(v["hi"].as_f64().unwrap()))
}

fn table(s: &Value) -> String {
    let mut t = String::from("| n | n' | log2 ell | cases | rho S | D2/rho | D2e/rho | D/rho | D2/D | D2e/D | D2/D2e | rho A/A | D2 fs/F | D2 rel/graph/LA share |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in s["rungs"].as_array().unwrap() {
        let g = |k: &str| fmt(&r["ratios"][k]["meq_s1_u1"]);
        let c = &r["census"]["D2"];
        t += &format!(
            "| {} | {} | {:.1} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            r["n"], r["np"], r["log2_ell"].as_f64().unwrap(), r["cases"],
            r["rho_S"].as_f64().map(|x| format!("{:.1}", x)).unwrap_or("—".into()),
            g("D2/rho"), g("D2e/rho"), g("D/rho"), g("D2/D"), g("D2e/D"), g("D2/D2e"), fmt(&r["rho_AA"]),
            if c.is_null() { "—".into() } else { format!("{:.0}/{:.0}", c["fs"].as_f64().unwrap(), c["fb"].as_f64().unwrap()) },
            if c.is_null() { "—".into() } else { format!("{:.2}/{:.2}/{:.2}", c["relation_share"].as_f64().unwrap(), c["graph_share"].as_f64().unwrap(), c["la_share"].as_f64().unwrap()) },
        );
    }
    t
}

fn figure(s: &Value) -> String {
    let (w, h) = (820.0, 500.0);
    let (x0, x1, y0, y1) = (70.0, 640.0, 34.0, 430.0);
    let rungs = s["rungs"].as_array().unwrap();
    let nmin = 18.0;
    let nmax = 120.0;
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
        n += 6.0;
    }
    let nlast = rungs.iter().map(|r| r["n"].as_f64().unwrap()).fold(0.0, f64::max);
    o += &format!("<line x1='{0}' x2='{0}' y1='{y0}' y2='{y1}' stroke='#bbb'/>", px(nlast));
    o += &format!("<text x='{}' y='{}' text-anchor='middle'>n (field F_2^n, subfield n' = n/3); right of n = {}: model only (dashed)</text>", (x0 + x1) / 2.0, y1 + 34.0, nlast);
    let series = [("D/rho", "#7f7f7f", "D (full base, #5261)"), ("D2/rho", "#d62728", "D2 (2LP, beta = 2/3)"), ("D2e/rho", "#1f77b4", "D2e (2LP, beta = 0.528)")];
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
        o += &format!("<rect x='{}' y='{}' width='12' height='3' fill='{col}'/><text x='{}' y='{}'>{lab}</text>", x1 - 230.0, y0 + 14.0 + 16.0 * k as f64, x1 - 212.0, y0 + 18.0 + 16.0 * k as f64);
    }
    if let Some(gr) = s["projection"]["grid"].as_array() {
        for (key, col) in [("D2_echelon_fit_over_rho", "#d62728"), ("D2e_echelon_fit_over_rho", "#1f77b4"), ("rebalanced_sparse_model_over_rho", "#2ca02c")] {
            let pts: Vec<String> = gr
                .iter()
                .filter(|e| e["n"].as_f64().unwrap() >= nlast && e["n"].as_f64().unwrap() <= nmax)
                .map(|e| format!("{},{}", px(e["n"].as_f64().unwrap()), py(e[key].as_f64().unwrap())))
                .collect();
            o += &format!("<polyline points='{}' fill='none' stroke='{col}' stroke-dasharray='7,4' stroke-width='1.3'/>", pts.join(" "));
        }
        o += &format!("<text x='{}' y='{}' fill='#2ca02c'>dashed green: re-balanced base, sparse-LA model (model)</text>", x1 - 330.0, y0 + 18.0 + 16.0 * 3.0);
        o += &format!("<text x='{}' y='{}' fill='#555'>dashed red/blue: frozen rules, fitted echelon LA (model)</text>", x1 - 330.0, y0 + 18.0 + 16.0 * 4.0);
    }
    o += "</svg>";
    o
}

/// Part A pilot rule: θ* per β, and rho W* per new rung.
pub fn pilot_summary(theta: &str, rhof: &str) {
    let mut m: BTreeMap<(String, String, u64), Vec<f64>> = BTreeMap::new();
    let mut bad = 0;
    for l in std::fs::read_to_string(theta).unwrap().lines() {
        let v: Value = serde_json::from_str(l).unwrap();
        if v["correct"].as_bool() != Some(true) || v["verified"].as_bool() != Some(true) {
            bad += 1;
            continue;
        }
        m.entry((format!("{:.3}", v["beta"].as_f64().unwrap()), format!("{}", v["theta"]), v["case"]["n"].as_u64().unwrap())).or_default().push(v["cost"]["meq_s1_u1"].as_f64().unwrap());
    }
    println!("theta pilot: {bad} failed/incorrect records");
    let mut score: BTreeMap<(String, String), (f64, Vec<String>)> = BTreeMap::new();
    for ((b, t, n), v) in &m {
        let e = score.entry((b.clone(), t.clone())).or_insert((0.0, vec![]));
        e.0 += gmean(v).ln();
        e.1.push(format!("n={n}:{:.3e}(k={})", gmean(v), v.len()));
    }
    for ((b, t), (s, d)) in &score {
        println!("beta={b} theta={t} score={s:.4} {}", d.join(" "));
    }
    if let Ok(txt) = std::fs::read_to_string(rhof) {
        let mut r: BTreeMap<(u64, u64), Vec<f64>> = BTreeMap::new();
        let mut rbad = 0;
        for l in txt.lines() {
            let v: Value = serde_json::from_str(l).unwrap();
            if v["correct"].as_bool() != Some(true) {
                rbad += 1;
                continue;
            }
            r.entry((v["n"].as_u64().unwrap(), v["lanes"].as_u64().unwrap())).or_default().push(v["cost"]["meq_s1_u1"].as_f64().unwrap());
        }
        println!("rho pilot: {rbad} incorrect");
        for ((n, w), v) in &r {
            println!("rho n={n} W={w} mean={:.4e} k={}", mean(v), v.len());
        }
    }
}

/// Predictions (PROTOCOL.md part B): the abstract-graph simulation's targets,
/// yields and counted graph/LA units, priced with #5261's measured relation
/// Meq per target (fit α + βn′ beyond n = 48) and #5261's rho S (S = 8.5, the
/// mean of n = 45, 48, beyond), ℓ ≈ 2^{3n′}/h̄, h̄ = 3.32.
pub fn predict_summary(pred: &str, bwd: &str) {
    let b: Value = serde_json::from_str(&std::fs::read_to_string(bwd).unwrap()).unwrap();
    let mut relpt: BTreeMap<u64, (f64, f64, f64, f64)> = BTreeMap::new();
    for r in b["rungs"].as_array().unwrap() {
        let c = &r["D_census"];
        let tot = c["relation_phase_meq"].as_f64().unwrap() + c["la_meq"].as_f64().unwrap();
        relpt.insert(r["n"].as_u64().unwrap(), (c["relation_meq_per_target"].as_f64().unwrap(), r["rho_S"].as_f64().unwrap(), r["log2_ell"].as_f64().unwrap(), tot));
    }
    let (al, be) = (b["projection"]["alpha"].as_f64().unwrap(), b["projection"]["beta"].as_f64().unwrap());
    let mut m: BTreeMap<(String, u64), Vec<Value>> = BTreeMap::new();
    for l in std::fs::read_to_string(pred).unwrap().lines() {
        let v: Value = serde_json::from_str(l).unwrap();
        m.entry((format!("{:.3}/{}", v["beta"].as_f64().unwrap(), v["theta"]), v["n"].as_u64().unwrap())).or_default().push(v);
    }
    println!("| arm (beta/theta) | n | F | fs | targets/q' | yields | 2LP share of used rows | yield weight | graph Meq | LA Meq | total Meq | pred D2/rho | pred D2/D |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    let mut fitp: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for ((arm, n), vs) in &m {
        let np = *n / 3;
        let g = |k: &str| mean(&vs.iter().map(|v| v[k].as_f64().unwrap()).collect::<Vec<_>>());
        let (rp, s, l2, dtot) = relpt.get(n).copied().unwrap_or((al + be * np as f64, 8.5, 3.0 * np as f64 - 3.32f64.log2(), f64::NAN));
        let tg = g("targets");
        let tot = tg * rp + g("graph_units") + g("la_units");
        let rho = s * 2f64.powf(l2 / 2.0);
        fitp.entry(arm.clone()).or_default().push((l2, tot.log2()));
        println!(
            "| {arm} | {n} | {:.0} | {:.0} | {:.2} | {:.0} | {:.2} | {:.1} | {:.3e} | {:.3e} | {:.3e} | {:.1} | {} |",
            g("F"), g("fs"), g("targets_over_qp"), g("yields"), g("two_lp") / (g("full") + g("one_lp") + g("two_lp")), g("yield_weight_mean"),
            g("graph_units"), g("la_units"), tot, tot / rho, if dtot.is_nan() { "—".into() } else { format!("{:.3}", tot / dtot) }
        );
    }
    for (arm, p) in &fitp {
        let big: Vec<&(f64, f64)> = p.iter().filter(|x| x.0 >= 33.0).collect();
        let (sl, _) = ols(&big.iter().map(|x| x.0).collect::<Vec<_>>(), &big.iter().map(|x| x.1).collect::<Vec<_>>());
        println!("predicted slope (log2 cost on log2 ell, n >= 36) {arm}: {sl:.3}");
    }
}
