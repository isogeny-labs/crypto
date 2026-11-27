//! Frozen analysis (PROTOCOL.md "Analysis"): paired D/rho, F/rho, M2/rho,
//! D/F, D/M2 per rung with a hierarchical bootstrap; the §5a band; the
//! rho A/A noise floor; D-vs-F identity on shared cases; exponent fits;
//! D phase census; crossover projections; the SVG figure.
use crate::seeds::Sm64;
use serde_json::{json, Value};
use std::collections::BTreeMap;

const ARMS: [&str; 3] = ["D", "F", "M2"];
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

type Rung = BTreeMap<u32, BTreeMap<u32, Case>>; // curve -> target -> case

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

/// Geometric mean over cases of num/den with a hierarchical bootstrap
/// (curves, targets within curve, rho seeds within target).
fn stat(r: &Rung, num: &dyn Fn(&Case, Option<&[usize]>) -> Option<f64>, den: &dyn Fn(&Case, Option<&[usize]>) -> Option<f64>, rng: &mut Sm64, b: usize) -> Option<Value> {
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

pub fn analyze(_protocol: &str, runs: &str, out: &str, svg: &str) {
    let text = std::fs::read_to_string(runs).unwrap();
    let mut tree: BTreeMap<u32, Rung> = BTreeMap::new();
    let mut counts: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let mut bad = vec![];
    let (mut fmis, mut fchk, mut bmis, mut bchk, mut unres, mut fallbacks) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).unwrap();
        let arm = v["arm"].as_str().unwrap().to_string();
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
            bad.push(json!({"n": n, "ci": ci, "ti": ti, "arm": arm, "seed_index": v["seed_index"]}));
        }
        if arm == "D" {
            fmis += v["fcheck_mismatches"].as_u64().unwrap_or(0);
            fchk += v["fcheck_targets"].as_u64().unwrap_or(0);
            bmis += v["brute_mismatches"].as_u64().unwrap_or(0);
            bchk += v["brute_targets"].as_u64().unwrap_or(0);
            let ds = &v["dstats"];
            fallbacks += ["fallback_rank", "fallback_uzero", "fallback_quartic_zero"].iter().map(|k| ds[k].as_u64().unwrap_or(0)).sum::<u64>();
        }
        if arm != "RHO" {
            unres += v["rows_unresolved"].as_u64().unwrap_or(0);
        }
        let cd = tree.entry(n).or_default().entry(ci).or_default().entry(ti).or_default();
        cd.ell = c["curve"]["ell"].as_f64().unwrap();
        cd.h = c["curve"]["h"].as_f64().unwrap_or(2.0);
        let costs: BTreeMap<String, f64> = KEYS.iter().map(|k| (k.to_string(), v["cost"][k].as_f64().unwrap())).collect();
        if !ok {
            continue;
        }
        if arm == "RHO" {
            cd.rho.push(costs);
        } else {
            cd.ic.insert(arm.clone(), costs);
            cd.rec.insert(arm, v.clone());
        }
    }
    let mut rng = Sm64::keyed(&["boot", "bwd", "main"]);
    let mut rungs = vec![];
    // D vs F identity on shared cases
    let (mut id_cases, mut id_equal) = (0u64, 0u64);
    // per-rung fit inputs
    let mut fit_rows = vec![];
    for (&n, r) in &tree {
        let np = n / 3;
        let all: Vec<&Case> = r.values().flat_map(|t| t.values()).collect();
        let ell = gmean(&all.iter().map(|c| c.ell).collect::<Vec<_>>());
        let rho_s: Vec<f64> = all.iter().filter_map(|c| rho_mean(c, "meq_s1_u1", None).map(|m| m / c.ell.sqrt())).collect();
        let mut arms = serde_json::Map::new();
        for arm in ARMS {
            let mut band = serde_json::Map::new();
            for key in KEYS {
                let b = if key == "meq_s1_u1" { 10000 } else { 2000 };
                if let Some(s) = stat(r, &|cd, _| cd.ic.get(arm).map(|m| m[key]), &|cd, ix| rho_mean(cd, key, ix), &mut rng, b) {
                    band.insert(key.into(), s);
                }
            }
            if !band.is_empty() {
                arms.insert(format!("{arm}/rho"), Value::Object(band));
            }
        }
        for other in ["F", "M2"] {
            let mut band = serde_json::Map::new();
            for key in KEYS {
                let b = if key == "meq_s1_u1" { 10000 } else { 2000 };
                if let Some(s) = stat(r, &|cd, _| cd.ic.get("D").map(|m| m[key]), &|cd, _| cd.ic.get(other).map(|m| m[key]), &mut rng, b) {
                    band.insert(key.into(), s);
                }
            }
            if !band.is_empty() {
                arms.insert(format!("D/{other}"), Value::Object(band));
            }
        }
        // A/A: rho seeds 0..7 against 8..15 on the same Q
        let aa = stat(
            r,
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[..cd.rho.len() / 2].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[cd.rho.len() / 2..].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &mut rng,
            2000,
        );
        // D census
        let mut census = serde_json::Map::new();
        let drecs: Vec<&Value> = all.iter().filter_map(|c| c.rec.get("D")).collect();
        if !drecs.is_empty() {
            let per = |f: &dyn Fn(&Value) -> f64| mean(&drecs.iter().map(|v| f(v)).collect::<Vec<_>>());
            let tg = per(&|v| v["targets"].as_f64().unwrap());
            let rel = per(&|v| REL_PHASES.iter().map(|p| phase_meq(v, p, n, 1.0, 1.0)).sum());
            let la = per(&|v| phase_meq(v, "la", n, 1.0, 1.0));
            let oracle_pt = per(&|v| ["descent", "rootfind", "lift"].iter().map(|p| phase_meq(v, p, n, 1.0, 1.0)).sum::<f64>() / v["targets"].as_f64().unwrap());
            let relpt = per(&|v| REL_PHASES.iter().map(|p| phase_meq(v, p, n, 1.0, 1.0)).sum::<f64>() / v["targets"].as_f64().unwrap());
            let fbn = per(&|v| v["fb"].as_f64().unwrap());
            for p in ["fb", "setup", "targets", "descent", "rootfind", "lift", "resolve", "la", "verify"] {
                census.insert(format!("meq_{p}"), json!(per(&|v| phase_meq(v, p, n, 1.0, 1.0))));
            }
            census.insert("targets".into(), json!(tg));
            census.insert("targets_over_qp".into(), json!(tg / (1u64 << np) as f64));
            census.insert("fb".into(), json!(fbn));
            census.insert("relations".into(), json!(per(&|v| v["relations"].as_f64().unwrap())));
            census.insert("decomp_per_target".into(), json!(per(&|v| v["dstats"]["decompositions"].as_f64().unwrap() / v["dstats"]["targets"].as_f64().unwrap())));
            census.insert("predicted_decomp_per_target_4F3_over_3N".into(), json!(mean(&all.iter().filter_map(|c| c.rec.get("D").map(|v| { let f = v["fb"].as_f64().unwrap(); 4.0 * f.powi(3) / (3.0 * c.ell * c.h) })).collect::<Vec<_>>())));
            census.insert("deg_u_mean".into(), json!(per(&|v| v["dstats"]["deg_u_mean"].as_f64().unwrap())));
            census.insert("oracle_meq_per_target".into(), json!(oracle_pt));
            census.insert("relation_meq_per_target".into(), json!(relpt));
            census.insert("relation_phase_meq".into(), json!(rel));
            census.insert("la_meq".into(), json!(la));
            census.insert("la_share".into(), json!(la / (rel + la)));
            census.insert("wall_s".into(), json!(per(&|v| v["wall_ns"].as_f64().unwrap() / 1e9)));
            fit_rows.push((n, np, fbn, tg, oracle_pt, relpt, rel, la, mean(&rho_s), ell, gmean(&all.iter().map(|c| c.h).collect::<Vec<_>>())));
        }
        // D-rel / rho (relation phase only, i.e. LA excluded) — diagnostic
        let drel = stat(
            r,
            &|cd, _| cd.rec.get("D").map(|v| REL_PHASES.iter().map(|p| phase_meq(v, p, n, 1.0, 1.0)).sum::<f64>()),
            &|cd, ix| rho_mean(cd, "meq_s1_u1", ix),
            &mut rng,
            2000,
        );
        for ts in r.values() {
            for cd in ts.values() {
                if let (Some(d), Some(f)) = (cd.rec.get("D"), cd.rec.get("F")) {
                    id_cases += 1;
                    if d["d"] == f["d"] && d["targets"] == f["targets"] && d["relations"] == f["relations"] && d["columns"] == f["columns"] {
                        id_equal += 1;
                    }
                }
            }
        }
        rungs.push(json!({
            "n": n, "np": np, "log2_ell": ell.log2(), "cases": all.len(),
            "rho_runs": all.iter().map(|c| c.rho.len()).sum::<usize>(),
            "rho_S": if rho_s.is_empty() { Value::Null } else { json!(mean(&rho_s)) },
            "ratios": Value::Object(arms), "rho_AA": aa, "D_relation_phase_over_rho": drel,
            "D_census": Value::Object(census),
        }));
    }
    // exponent fits (log2 cost on log2 ell), bootstrap within rung
    let mut fits = serde_json::Map::new();
    let series = |arm: &str, nmax: u32, relonly: bool| -> BTreeMap<u32, Vec<(f64, f64)>> {
        let mut m: BTreeMap<u32, Vec<(f64, f64)>> = BTreeMap::new();
        for (&n, r) in &tree {
            if n > nmax {
                continue;
            }
            for ts in r.values() {
                for cd in ts.values() {
                    let y = if arm == "RHO" {
                        rho_mean(cd, "meq_s1_u1", None)
                    } else if relonly {
                        cd.rec.get(arm).map(|v| REL_PHASES.iter().map(|p| phase_meq(v, p, n, 1.0, 1.0)).sum::<f64>())
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
    let fit = |s: &BTreeMap<u32, Vec<(f64, f64)>>, rng: &mut Sm64| -> Option<(f64, f64, f64, Vec<f64>)> {
        let pts: Vec<(f64, f64)> = s.values().flatten().copied().collect();
        if s.len() < 2 {
            return None;
        }
        let (b, _) = ols(&pts.iter().map(|p| p.0).collect::<Vec<_>>(), &pts.iter().map(|p| p.1).collect::<Vec<_>>());
        let mut bs = vec![];
        for _ in 0..2000 {
            let mut xs = vec![];
            let mut ys = vec![];
            for v in s.values() {
                for _ in 0..v.len() {
                    let p = v[rng.below(v.len() as u64) as usize];
                    xs.push(p.0);
                    ys.push(p.1);
                }
            }
            bs.push(ols(&xs, &ys).0);
        }
        let bs2 = bs.clone();
        Some((b, pct(&mut bs.clone(), 0.025), pct(&mut bs, 0.975), bs2))
    };
    let mut frng = Sm64::keyed(&["boot", "bwd", "slope"]);
    for (label, arm, nmax, relonly) in [
        ("RHO_all", "RHO", 99, false),
        ("RHO_n<=33", "RHO", 33, false),
        ("D_all", "D", 99, false),
        ("D_n<=33", "D", 33, false),
        ("D_relation_phase_all", "D", 99, true),
        ("F_n<=33", "F", 33, false),
        ("M2_all", "M2", 99, false),
        ("M2_n<=33", "M2", 33, false),
    ] {
        let mut frng2 = Sm64::keyed(&["boot", "bwd", "slope", label]);
        let _ = &mut frng;
        if let Some((b, lo, hi, _)) = fit(&series(arm, nmax, relonly), &mut frng2) {
            fits.insert(label.into(), json!({"slope": b, "lo": lo, "hi": hi, "points": series(arm, nmax, relonly).values().map(|v| v.len()).sum::<usize>()}));
        }
    }
    // crossover projection (model; PROTOCOL.md "Crossover formula")
    let proj = projection(&fit_rows);
    let s = json!({
        "schema": "bwd-summary/v1",
        "records": counts.iter().map(|(k, v)| (k.clone(), json!({"records": v.0, "failed": v.1}))).collect::<serde_json::Map<_, _>>(),
        "failed_records": bad,
        "D_vs_F_identity": {"cases": id_cases, "equal_d_targets_relations_columns": id_equal},
        "D_set_checks": {"fcheck_targets": fchk, "fcheck_mismatches": fmis, "brute_targets": bchk, "brute_mismatches": bmis, "fallbacks": fallbacks},
        "rows_unresolved": unres,
        "rungs": rungs,
        "exponent_fits_log2cost_vs_log2ell": Value::Object(fits),
        "projection": proj,
        "bootstrap": {"B_primary": 10000, "B_band_and_other": 2000, "streams": "boot|bwd|main, boot|bwd|slope|<label>"},
    });
    std::fs::write(out, serde_json::to_string_pretty(&s).unwrap()).unwrap();
    std::fs::write(svg, figure(&s)).unwrap();
    println!("{}", table(&s));
}

/// Fit c(n') = α + β·n' (relation Meq per target), κ = targets/q',
/// LA = A·F^γ (echelon), rho S at the top rung; solve for crossings.
fn projection(rows: &[(u32, u32, f64, f64, f64, f64, f64, f64, f64, f64, f64)]) -> Value {
    if rows.len() < 3 {
        return Value::Null;
    }
    let nps: Vec<f64> = rows.iter().map(|r| r.1 as f64).collect();
    let relpt: Vec<f64> = rows.iter().map(|r| r.5).collect();
    let (beta, alpha) = ols(&nps, &relpt);
    let kappa = mean(&rows.iter().map(|r| r.3 / 2f64.powi(r.1 as i32)).collect::<Vec<_>>());
    let fbr = mean(&rows.iter().map(|r| r.2 / 2f64.powi(r.1 as i32)).collect::<Vec<_>>());
    let big: Vec<&(u32, u32, f64, f64, f64, f64, f64, f64, f64, f64, f64)> = rows.iter().filter(|r| r.2 >= 256.0).collect();
    let (gam, la0) = ols(&big.iter().map(|r| r.2.log2()).collect::<Vec<_>>(), &big.iter().map(|r| r.7.log2()).collect::<Vec<_>>());
    let top = rows.iter().filter(|r| r.8.is_finite() && r.8 > 0.0).last().unwrap();
    let s_top = top.8;
    let hbar = mean(&rows.iter().map(|r| r.10).collect::<Vec<_>>());
    let rho = |np: f64| s_top * (2f64.powf(3.0 * np) / hbar).sqrt();
    let drel = |np: f64| kappa * 2f64.powf(np) * (alpha + beta * np);
    let fbf = |np: f64| fbr * 2f64.powf(np);
    let la_e = |np: f64| 2f64.powf(la0 + gam * fbf(np).log2());
    let la_w = |np: f64| 10.0 * fbf(np).powi(2);
    let mut grid = vec![];
    let (mut x_rel, mut x_w, mut x_e) = (None, None, None);
    let (mut min_w, mut min_e) = ((f64::MAX, 0.0), (f64::MAX, 0.0));
    let mut min_b = (f64::MAX, 0.0, 1.0);
    let mut np = 6.0;
    while np <= 64.0 {
        let r = rho(np);
        let a = drel(np) / r;
        let w = (drel(np) + la_w(np)) / r;
        let e = (drel(np) + la_e(np)) / r;
        // balanced base of F/λ classes (λ ≥ 1): relations ×λ², LA ×λ⁻²
        let lam = ((la_w(np) / drel(np)).powf(0.25)).max(1.0);
        let bal = (drel(np) * lam * lam + la_w(np) / (lam * lam)) / r;
        if bal < min_b.0 {
            min_b = (bal, np, lam);
        }
        if x_rel.is_none() && a <= 1.0 {
            x_rel = Some(np);
        }
        if x_w.is_none() && w <= 1.0 {
            x_w = Some(np);
        }
        if x_e.is_none() && e <= 1.0 {
            x_e = Some(np);
        }
        if w < min_w.0 {
            min_w = (w, np);
        }
        if e < min_e.0 {
            min_e = (e, np);
        }
        if (np * 4.0).round() % 4.0 == 0.0 {
            grid.push(json!({"np": np, "n": 3.0 * np, "rho": r, "D_rel_over_rho": a, "D_rel_plus_wiedemann_model_over_rho": w, "D_rel_plus_echelon_fit_over_rho": e, "D_balanced_base_wiedemann_model_over_rho": (drel(np) * ((la_w(np) / drel(np)).powf(0.25)).max(1.0).powi(2) + la_w(np) / ((la_w(np) / drel(np)).powf(0.25)).max(1.0).powi(2)) / r}));
        }
        np += 0.25;
    }
    json!({
        "model": "rho(n') = S_top*sqrt(2^(3n')/h); D_rel(n') = kappa*2^n'*(alpha + beta*n'); LA_echelon = 2^(la0)*F^gamma (fit on F >= 256); LA_wiedemann_model = 10*F^2 (2F mat-vecs of weight 3 plus Berlekamp-Massey, in mod-ell units); F = fb_ratio*2^n'; balanced base: F/lambda classes, lambda = max(1, (LA_w/D_rel)^(1/4)), relation phase x lambda^2 (decomposition probability lambda^-3, lambda^-1 as many relations needed), LA x lambda^-2",
        "alpha": alpha, "beta": beta, "kappa_targets_over_qp": kappa, "fb_over_qp": fbr,
        "la_echelon_log2_coeff": la0, "la_echelon_exponent_gamma": gam,
        "rho_S_top": s_top, "rho_S_top_n": top.0, "h_mean": hbar,
        "crossover_np_relation_phase_only": x_rel, "crossover_np_with_wiedemann_model": x_w, "crossover_np_with_echelon_fit": x_e,
        "min_ratio_with_wiedemann_model": {"ratio": min_w.0, "np": min_w.1},
        "min_ratio_with_echelon_fit": {"ratio": min_e.0, "np": min_e.1},
        "min_ratio_balanced_base_wiedemann_model": {"ratio": min_b.0, "np": min_b.1, "lambda": min_b.2},
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
    let mut t = String::from("| n | n' | log2 ell | cases | rho S | D/rho | F/rho | M2/rho | D/F | D/M2 | rho A/A | D LA share |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in s["rungs"].as_array().unwrap() {
        let g = |k: &str| fmt(&r["ratios"][k]["meq_s1_u1"]);
        t += &format!(
            "| {} | {} | {:.1} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            r["n"], r["np"], r["log2_ell"].as_f64().unwrap(), r["cases"],
            r["rho_S"].as_f64().map(|x| format!("{:.1}", x)).unwrap_or("—".into()),
            g("D/rho"), g("F/rho"), g("M2/rho"), g("D/F"), g("D/M2"), fmt(&r["rho_AA"]),
            r["D_census"]["la_share"].as_f64().map(|x| format!("{:.3}", x)).unwrap_or("—".into()),
        );
    }
    t
}

fn figure(s: &Value) -> String {
    let (w, h) = (760.0, 460.0);
    let (x0, x1, y0, y1) = (70.0, 600.0, 30.0, 400.0);
    let rungs = s["rungs"].as_array().unwrap();
    let nmin = 18.0;
    let nmax = rungs.iter().map(|r| r["n"].as_f64().unwrap()).fold(33.0, f64::max).max(48.0);
    let (lmin, lmax) = (-1.0f64, 6.0f64); // log10 range
    let px = |n: f64| x0 + (n - nmin) / (nmax - nmin) * (x1 - x0);
    let py = |v: f64| y1 - (v.log10().clamp(lmin, lmax) - lmin) / (lmax - lmin) * (y1 - y0);
    let mut o = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
    o += &format!("<text x='{}' y='18' font-size='14'>Cold single-target cost / matched negation-map rho (Meq, s=1, u=1), 95% CI</text>", x0);
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
        n += 3.0;
    }
    o += &format!("<text x='{}' y='{}' text-anchor='middle'>n (field F_2^n, subfield n' = n/3)</text>", (x0 + x1) / 2.0, y1 + 34.0);
    let series = [("D/rho", "#d62728", "D (Weil descent)"), ("M2/rho", "#1f77b4", "M2 (hash MITM, #5253)"), ("F/rho", "#7f7f7f", "F (tuned S4 root finder, #5253)")];
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
            o += &format!("<circle cx='{}' cy='{}' r='3.5' fill='{col}'/>", px(n), py(p));
            pts.push(format!("{},{}", px(n), py(p)));
        }
        o += &format!("<polyline points='{}' fill='none' stroke='{col}' stroke-width='1.5'/>", pts.join(" "));
        o += &format!("<rect x='{}' y='{}' width='12' height='3' fill='{col}'/><text x='{}' y='{}'>{lab}</text>", x0 + 10.0, y0 + 14.0 + 16.0 * k as f64, x0 + 28.0, y0 + 18.0 + 16.0 * k as f64);
    }
    // relation-phase-only D and projections
    let mut pts = vec![];
    for r in rungs {
        let v = &r["D_relation_phase_over_rho"];
        if v.is_null() {
            continue;
        }
        let n = r["n"].as_f64().unwrap();
        pts.push(format!("{},{}", px(n), py(v["ratio"].as_f64().unwrap())));
    }
    o += &format!("<polyline points='{}' fill='none' stroke='#d62728' stroke-dasharray='2,3' stroke-width='1.5'/>", pts.join(" "));
    o += &format!("<text x='{}' y='{}' fill='#d62728'>dotted: D relation phase only (LA excluded)</text>", x0 + 10.0, y0 + 18.0 + 16.0 * 3.0);
    if let Some(g) = s["projection"]["grid"].as_array() {
        let mut p1 = vec![];
        let mut p2 = vec![];
        for e in g {
            let n = e["n"].as_f64().unwrap();
            if n > nmax {
                continue;
            }
            p1.push(format!("{},{}", px(n), py(e["D_rel_over_rho"].as_f64().unwrap())));
            p2.push(format!("{},{}", px(n), py(e["D_rel_plus_wiedemann_model_over_rho"].as_f64().unwrap())));
        }
        o += &format!("<polyline points='{}' fill='none' stroke='#ff9896' stroke-dasharray='8,4'/>", p1.join(" "));
        o += &format!("<polyline points='{}' fill='none' stroke='#9467bd' stroke-dasharray='8,4'/>", p2.join(" "));
        o += &format!("<text x='{}' y='{}' fill='#ff9896'>dashed pink: projected D relation phase / rho (model)</text>", x0 + 10.0, y0 + 18.0 + 16.0 * 4.0);
        o += &format!("<text x='{}' y='{}' fill='#9467bd'>dashed purple: projected D + Wiedemann-model LA / rho</text>", x0 + 10.0, y0 + 18.0 + 16.0 * 5.0);
    }
    o += "</svg>";
    o
}
