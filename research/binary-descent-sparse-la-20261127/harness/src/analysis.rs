//! Frozen analysis (PROTOCOL.md "Analysis"): paired DS/rho, DS/D, DSB/rho,
//! DSB/DS per rung with a hierarchical bootstrap; the §5a band and the two
//! sensitivity readings; the rho A/A floor; the shadow-echelon identity with
//! #5261's D records; D-S census; exponent fits; crossover projection
//! (model); the SVG figure.
use crate::seeds::Sm64;
use serde_json::{json, Value};
use std::collections::BTreeMap;

const KEYS: [&str; 5] = ["meq_s1_u1", "meq_s1_u01", "meq_s1_u0", "meq_s025_u1", "meq_s025_u0"];
const SENS: [&str; 2] = ["meq_s1_u1_pm1_charged", "meq_s1_u1_pm1_and_structural_charged"];
const REL_PHASES: [&str; 8] = ["fb", "setup", "targets", "descent", "rootfind", "lift", "resolve", "fb_filter"];
const ID_PHASES: [&str; 7] = ["fb", "setup", "targets", "descent", "rootfind", "lift", "resolve"];

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
    // sensitivity keys apply to the IC arm only; rho is read at s=1, u=1
    let k = if SENS.contains(&key) { "meq_s1_u1" } else { key };
    let v: Vec<f64> = match idx {
        None => cd.rho.iter().map(|r| r[k]).collect(),
        Some(ix) => ix.iter().map(|&i| cd.rho[i][k]).collect(),
    };
    Some(mean(&v))
}

type F<'a> = &'a dyn Fn(&Case, Option<&[usize]>) -> Option<f64>;

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
    if rec["phases"][phase].is_null() {
        return 0.0;
    }
    let o = crate::field::Ops::from_json(&rec["phases"][phase]);
    crate::field::meq(&o, n, ws, wu)
}
fn rel_meq(rec: &Value, n: u32) -> f64 {
    REL_PHASES.iter().map(|p| phase_meq(rec, p, n, 1.0, 1.0)).sum()
}

pub fn analyze(_protocol: &str, runs: &str, out: &str, svg: &str) {
    let text = std::fs::read_to_string(runs).unwrap();
    let mut tree: BTreeMap<u32, Rung> = BTreeMap::new();
    let mut counts: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let mut bad = vec![];
    let mut arms_seen: std::collections::BTreeSet<String> = Default::default();
    let (mut track_n, mut track_ok, mut unres) = (0u64, 0u64, 0u64);
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).unwrap();
        let arm = v["arm"].as_str().unwrap().to_string();
        if !(arm == "RHO" || arm == "D" || arm == "DS" || arm.starts_with("DSB_")) {
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
        if arm.starts_with("DS") {
            unres += v["rows_unresolved"].as_u64().unwrap_or(0);
            if let Some(t) = v["track_check"].as_bool() {
                track_n += 1;
                if t {
                    track_ok += 1;
                }
            }
        }
        let cd = tree.entry(n).or_default().entry(ci).or_default().entry(ti).or_default();
        cd.ell = c["curve"]["ell"].as_f64().unwrap();
        cd.h = c["curve"]["h"].as_f64().unwrap_or(2.0);
        let mut costs: BTreeMap<String, f64> = KEYS.iter().map(|k| (k.to_string(), v["cost"][k].as_f64().unwrap())).collect();
        for k in SENS {
            if let Some(x) = v["cost"][k].as_f64() {
                costs.insert(k.to_string(), x);
            }
        }
        if !ok {
            continue;
        }
        if arm == "RHO" {
            cd.rho.push(costs);
        } else {
            arms_seen.insert(arm.clone());
            cd.ic.insert(arm.clone(), costs);
            cd.rec.insert(arm, v.clone());
        }
    }
    let dsb_arms: Vec<String> = arms_seen.iter().filter(|a| a.starts_with("DSB_")).cloned().collect();
    let mut rng = Sm64::keyed(&["boot", "bdsl", "main"]);
    let mut rungs = vec![];
    let (mut id_cases, mut id_equal, mut id_examples) = (0u64, 0u64, vec![]);
    let (mut rel_ge, mut rel_cases) = (0u64, 0u64);
    let mut fit_rows = vec![];
    for (&n, r) in &tree {
        let np = n / 3;
        let all: Vec<&Case> = r.values().flat_map(|t| t.values()).collect();
        let ell = gmean(&all.iter().map(|c| c.ell).collect::<Vec<_>>());
        let rho_s: Vec<f64> = all.iter().filter_map(|c| rho_mean(c, "meq_s1_u1", None).map(|m| m / c.ell.sqrt())).collect();
        let mut ratios = serde_json::Map::new();
        let mut arms_here: Vec<String> = vec!["DS".into(), "D".into()];
        arms_here.extend(dsb_arms.iter().cloned());
        for arm in &arms_here {
            let mut band = serde_json::Map::new();
            let keys: Vec<&str> = if arm == "D" { KEYS.to_vec() } else { KEYS.iter().chain(SENS.iter()).copied().collect() };
            for key in keys {
                let b = if key == "meq_s1_u1" { 10000 } else { 2000 };
                if let Some(s) = stat(r, &|cd, _| cd.ic.get(arm).and_then(|m| m.get(key).copied()), &|cd, ix| rho_mean(cd, key, ix), &mut rng, b) {
                    band.insert(key.into(), s);
                }
            }
            if !band.is_empty() {
                ratios.insert(format!("{arm}/rho"), Value::Object(band));
            }
        }
        let mut pairs: Vec<(String, String)> = vec![("DS".into(), "D".into())];
        for a in &dsb_arms {
            pairs.push((a.clone(), "DS".into()));
        }
        for (a, bb) in pairs {
            let mut band = serde_json::Map::new();
            for key in KEYS {
                let b = if key == "meq_s1_u1" { 10000 } else { 2000 };
                if let Some(s) = stat(r, &|cd, _| cd.ic.get(&a).map(|m| m[key]), &|cd, _| cd.ic.get(&bb).map(|m| m[key]), &mut rng, b) {
                    band.insert(key.into(), s);
                }
            }
            if !band.is_empty() {
                ratios.insert(format!("{a}/{bb}"), Value::Object(band));
            }
        }
        let aa = stat(
            r,
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[..cd.rho.len() / 2].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &|cd, _| if cd.rho.len() >= 2 { Some(mean(&cd.rho[cd.rho.len() / 2..].iter().map(|m| m["meq_s1_u1"]).collect::<Vec<_>>())) } else { None },
            &mut rng,
            2000,
        );
        let ds_rel_rho = stat(r, &|cd, _| cd.rec.get("DS").map(|v| rel_meq(v, n)), &|cd, ix| rho_mean(cd, "meq_s1_u1", ix), &mut rng, 2000);
        let ds_la_rho = stat(r, &|cd, _| cd.rec.get("DS").map(|v| phase_meq(v, "la", n, 1.0, 1.0)), &|cd, ix| rho_mean(cd, "meq_s1_u1", ix), &mut rng, 2000);
        let ds_rel_over_d_rel = stat(r, &|cd, _| cd.rec.get("DS").map(|v| rel_meq(v, n)), &|cd, _| cd.rec.get("D").map(|v| rel_meq(v, n)), &mut rng, 2000);
        let ds_la_over_d_la = stat(r, &|cd, _| cd.rec.get("DS").map(|v| phase_meq(v, "la", n, 1.0, 1.0)), &|cd, _| cd.rec.get("D").map(|v| phase_meq(v, "la", n, 1.0, 1.0)), &mut rng, 2000);
        // shadow identity with #5261's D record (d, targets, relations, columns, relation-phase counters)
        for ts in r.values() {
            for cd in ts.values() {
                if let (Some(s), Some(dr)) = (cd.rec.get("DS"), cd.rec.get("D")) {
                    let sh = &s["shadow_echelon"];
                    if sh.is_null() {
                        continue;
                    }
                    id_cases += 1;
                    let mut eq = sh["d"] == dr["d"] && sh["targets"] == dr["targets"] && sh["relations"] == dr["relations"] && sh["columns"] == dr["columns"];
                    for p in ID_PHASES {
                        eq &= sh["phases"][p] == dr["phases"][p];
                    }
                    if eq {
                        id_equal += 1;
                    } else if id_examples.len() < 5 {
                        id_examples.push(json!({"n": n, "ci": s["case"]["ci"], "ti": s["case"]["ti"]}));
                    }
                    rel_cases += 1;
                    if s["relations"].as_u64() >= dr["relations"].as_u64() {
                        rel_ge += 1;
                    }
                }
            }
        }
        // D-S census
        let mut census = serde_json::Map::new();
        let recs: Vec<&Value> = all.iter().filter_map(|c| c.rec.get("DS")).collect();
        if !recs.is_empty() {
            let per = |f: &dyn Fn(&Value) -> f64| mean(&recs.iter().map(|v| f(v)).collect::<Vec<_>>());
            let fbn = per(&|v| v["fb_cols_used"].as_f64().unwrap());
            let rel = per(&|v| rel_meq(v, n));
            let la = per(&|v| phase_meq(v, "la", n, 1.0, 1.0));
            let tg = per(&|v| v["targets"].as_f64().unwrap());
            census.insert("fb_cols".into(), json!(fbn));
            census.insert("targets".into(), json!(tg));
            census.insert("targets_over_qp".into(), json!(tg / (1u64 << np) as f64));
            census.insert("relations".into(), json!(per(&|v| v["relations"].as_f64().unwrap())));
            census.insert("relations_over_fb".into(), json!(per(&|v| v["relations"].as_f64().unwrap() / v["fb_cols_used"].as_f64().unwrap())));
            census.insert("relation_phase_meq".into(), json!(rel));
            census.insert("relation_meq_per_target".into(), json!(per(&|v| rel_meq(v, n) / v["targets"].as_f64().unwrap())));
            census.insert("la_meq".into(), json!(la));
            census.insert("la_meq_u0".into(), json!(per(&|v| phase_meq(v, "la", n, 1.0, 0.0))));
            census.insert("la_share".into(), json!(la / (rel + la)));
            census.insert("la_over_F2".into(), json!(per(&|v| phase_meq(v, "la", n, 1.0, 1.0) / v["fb_cols_used"].as_f64().unwrap().powi(2))));
            census.insert("la_pm1_charged_over_F2".into(), json!(per(&|v| (phase_meq(v, "la", n, 1.0, 1.0) + v["la_pm1_products_uncharged"].as_f64().unwrap()) / v["fb_cols_used"].as_f64().unwrap().powi(2))));
            census.insert("la_structural_over_F2".into(), json!(per(&|v| v["la_structural_ops_uncharged"].as_f64().unwrap() / v["fb_cols_used"].as_f64().unwrap().powi(2))));
            for k in ["core_rows", "core_cols", "rows_out", "cols_out", "nnz_out", "sge_pivots", "sge_singletons", "wiedemann_attempts", "wiedemann_den_zero", "wiedemann_fail", "minpoly_deg", "z_sge", "za_sge", "z_wied", "za_wied"] {
                census.insert(k.into(), json!(per(&|v| v["la_report"][k].as_f64().unwrap_or(0.0))));
            }
            census.insert("core_rows_over_fb".into(), json!(per(&|v| v["la_report"]["core_rows"].as_f64().unwrap() / v["fb_cols_used"].as_f64().unwrap())));
            census.insert("rows_out_over_fb".into(), json!(per(&|v| v["la_report"]["rows_out"].as_f64().unwrap() / v["fb_cols_used"].as_f64().unwrap())));
            census.insert("weight_out".into(), json!(per(&|v| v["la_report"]["nnz_out"].as_f64().unwrap() / v["la_report"]["rows_out"].as_f64().unwrap().max(1.0))));
            census.insert("la_calls".into(), json!(per(&|v| v["stop_rule"]["la_calls"].as_f64().unwrap())));
            census.insert("max_la_calls".into(), json!(recs.iter().map(|v| v["stop_rule"]["la_calls"].as_u64().unwrap()).max()));
            census.insert("sge_found_d".into(), json!(recs.iter().filter(|v| v["la_report"]["sge_empty_useful"].as_bool() == Some(true)).count()));
            census.insert("wall_s".into(), json!(per(&|v| v["wall_ns"].as_f64().unwrap() / 1e9)));
            census.insert("la_wall_note".into(), json!("wall is indicative; shadow echelon and track checks are excluded (check_wall_ns)"));
            let drecs: Vec<&Value> = all.iter().filter_map(|c| c.rec.get("D")).collect();
            if !drecs.is_empty() {
                census.insert("D_relations".into(), json!(mean(&drecs.iter().map(|v| v["relations"].as_f64().unwrap()).collect::<Vec<_>>())));
                census.insert("D_la_meq".into(), json!(mean(&drecs.iter().map(|v| phase_meq(v, "la", n, 1.0, 1.0)).collect::<Vec<_>>())));
            }
            fit_rows.push((n, np, fbn, tg, rel, la, if rho_s.is_empty() { f64::NAN } else { mean(&rho_s) }, ell, gmean(&all.iter().map(|c| c.h).collect::<Vec<_>>())));
        }
        let mut dsb = serde_json::Map::new();
        for a in &dsb_arms {
            let recs: Vec<&Value> = all.iter().filter_map(|c| c.rec.get(a)).collect();
            if recs.is_empty() {
                continue;
            }
            let per = |f: &dyn Fn(&Value) -> f64| mean(&recs.iter().map(|v| f(v)).collect::<Vec<_>>());
            let rel = per(&|v| rel_meq(v, n));
            let la = per(&|v| phase_meq(v, "la", n, 1.0, 1.0));
            dsb.insert(a.clone(), json!({"cases": recs.len(), "fb_cols": per(&|v| v["fb_cols_used"].as_f64().unwrap()), "targets": per(&|v| v["targets"].as_f64().unwrap()),
                "relations": per(&|v| v["relations"].as_f64().unwrap()), "relation_phase_meq": rel, "la_meq": la, "la_share": la / (rel + la)}));
        }
        rungs.push(json!({
            "n": n, "np": np, "log2_ell": ell.log2(), "cases": all.len(),
            "rho_runs": all.iter().map(|c| c.rho.len()).sum::<usize>(),
            "rho_S": if rho_s.is_empty() { Value::Null } else { json!(mean(&rho_s)) },
            "ratios": Value::Object(ratios), "rho_AA": aa,
            "DS_relation_phase_over_rho": ds_rel_rho, "DS_la_over_rho": ds_la_rho,
            "DS_relation_phase_over_D_relation_phase": ds_rel_over_d_rel, "DS_la_over_D_la": ds_la_over_d_la,
            "DS_census": Value::Object(census), "DSB_census": Value::Object(dsb),
        }));
    }
    // exponent fits (log2 cost on log2 ell), bootstrap within rung
    #[derive(Clone, Copy)]
    enum Y {
        Total,
        Rel,
        La,
    }
    let series = |arm: &str, nmin: u32, nmax: u32, y: Y, xf: bool| -> BTreeMap<u32, Vec<(f64, f64)>> {
        let mut m: BTreeMap<u32, Vec<(f64, f64)>> = BTreeMap::new();
        for (&n, r) in &tree {
            if n > nmax || n < nmin {
                continue;
            }
            for ts in r.values() {
                for cd in ts.values() {
                    let yy = if arm == "RHO" {
                        rho_mean(cd, "meq_s1_u1", None)
                    } else {
                        match y {
                            Y::Total => cd.ic.get(arm).map(|m| m["meq_s1_u1"]),
                            Y::Rel => cd.rec.get(arm).map(|v| rel_meq(v, n)),
                            Y::La => cd.rec.get(arm).map(|v| phase_meq(v, "la", n, 1.0, 1.0)),
                        }
                    };
                    let x = if xf { cd.rec.get(arm).and_then(|v| v["fb_cols_used"].as_f64().or(v["fb"].as_f64())).map(|f| f.log2()) } else { Some(cd.ell.log2()) };
                    if let (Some(yy), Some(x)) = (yy, x) {
                        if yy > 0.0 {
                            m.entry(n).or_default().push((x, yy.log2()));
                        }
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
        Some((b, pct(&mut bs.clone(), 0.025), pct(&mut bs, 0.975), a))
    };
    let mut fits = serde_json::Map::new();
    for (label, arm, nmin, nmax, y, xf) in [
        ("RHO_all", "RHO", 0, 99, Y::Total, false),
        ("D_all", "D", 0, 99, Y::Total, false),
        ("DS_all", "DS", 0, 99, Y::Total, false),
        ("DS_n>=36", "DS", 36, 99, Y::Total, false),
        ("DS_relation_phase_all", "DS", 0, 99, Y::Rel, false),
        ("DS_la_all_vs_ell", "DS", 0, 99, Y::La, false),
        ("DS_la_vs_log2F_n>=30", "DS", 30, 99, Y::La, true),
        ("D_la_vs_log2F_n>=30", "D", 30, 99, Y::La, true),
    ] {
        let mut frng = Sm64::keyed(&["boot", "bdsl", "slope", label]);
        let s = series(arm, nmin, nmax, y, xf);
        if let Some((b, lo, hi, a)) = fit(&s, &mut frng) {
            fits.insert(label.into(), json!({"slope": b, "lo": lo, "hi": hi, "intercept_log2": a, "points": s.values().map(|v| v.len()).sum::<usize>(), "x": if xf { "log2 F" } else { "log2 ell" }}));
        }
    }
    let proj = projection(&fit_rows);
    let s = json!({
        "schema": "bdsl-summary/v1",
        "records": counts.iter().map(|(k, v)| (k.clone(), json!({"records": v.0, "failed": v.1}))).collect::<serde_json::Map<_, _>>(),
        "failed_records": bad,
        "shadow_identity_with_5261_D": {"cases": id_cases, "equal_d_targets_relations_columns_relation_phase_counters": id_equal, "mismatch_examples": id_examples},
        "DS_relations_ge_D_relations": {"cases": rel_cases, "holds": rel_ge},
        "track_checks": {"cases": track_n, "ok": track_ok},
        "rows_unresolved": unres,
        "rungs": rungs,
        "exponent_fits_log2cost": Value::Object(fits),
        "projection": proj,
        "bootstrap": {"B_primary": 10000, "B_band_and_other": 2000, "streams": "boot|bdsl|main, boot|bdsl|slope|<label>"},
    });
    std::fs::write(out, serde_json::to_string_pretty(&s).unwrap()).unwrap();
    std::fs::write(svg, figure(&s)).unwrap();
    println!("{}", table(&s));
}

/// Crossover model (PROTOCOL.md "Crossover formula"): fitted, not measured.
fn projection(rows: &[(u32, u32, f64, f64, f64, f64, f64, f64, f64)]) -> Value {
    if rows.len() < 3 {
        return Value::Null;
    }
    let nps: Vec<f64> = rows.iter().map(|r| r.1 as f64).collect();
    let relpt: Vec<f64> = rows.iter().map(|r| r.4 / r.3).collect();
    let (beta, alpha) = ols(&nps, &relpt);
    let kappa = mean(&rows.iter().map(|r| r.3 / 2f64.powi(r.1 as i32)).collect::<Vec<_>>());
    let fbr = mean(&rows.iter().map(|r| r.2 / 2f64.powi(r.1 as i32)).collect::<Vec<_>>());
    let big: Vec<_> = rows.iter().filter(|r| r.2 >= 256.0).collect();
    let (gam, la0) = ols(&big.iter().map(|r| r.2.log2()).collect::<Vec<_>>(), &big.iter().map(|r| r.5.log2()).collect::<Vec<_>>());
    // c in LA = c F^2, from the top three rungs with F >= 256
    let top3: Vec<_> = big.iter().rev().take(3).collect();
    let c2 = mean(&top3.iter().map(|r| r.5 / r.2.powi(2)).collect::<Vec<_>>());
    let measured: Vec<_> = rows.iter().filter(|r| r.6.is_finite() && r.6 > 0.0).collect();
    let top = measured.last().unwrap();
    let s_top = top.6;
    let hbar = mean(&rows.iter().map(|r| r.8).collect::<Vec<_>>());
    let rho = |np: f64| s_top * (2f64.powf(3.0 * np) / hbar).sqrt();
    let drel = |np: f64| kappa * 2f64.powf(np) * (alpha + beta * np);
    let fbf = |np: f64| fbr * 2f64.powf(np);
    let la_fit = |np: f64| 2f64.powf(la0 + gam * fbf(np).log2());
    let la_c2 = |np: f64| c2 * fbf(np).powi(2);
    let mut grid = vec![];
    let (mut x_rel, mut x_fit, mut x_c2) = (None, None, None);
    let (mut min_fit, mut min_c2) = ((f64::MAX, 0.0), (f64::MAX, 0.0));
    let mut min_b = (f64::MAX, 0.0, 1.0);
    let mut np = 6.0;
    while np <= 64.0 {
        let r = rho(np);
        let a = drel(np) / r;
        let f1 = (drel(np) + la_fit(np)) / r;
        let f2 = (drel(np) + la_c2(np)) / r;
        let lam = ((la_c2(np) / drel(np)).powf(0.25)).max(1.0);
        let bal = (drel(np) * lam * lam + la_c2(np) / (lam * lam)) / r;
        if bal < min_b.0 {
            min_b = (bal, np, lam);
        }
        if x_rel.is_none() && a <= 1.0 {
            x_rel = Some(np);
        }
        if x_fit.is_none() && f1 <= 1.0 {
            x_fit = Some(np);
        }
        if x_c2.is_none() && f2 <= 1.0 {
            x_c2 = Some(np);
        }
        if f1 < min_fit.0 {
            min_fit = (f1, np);
        }
        if f2 < min_c2.0 {
            min_c2 = (f2, np);
        }
        if (np * 4.0).round() % 4.0 == 0.0 {
            grid.push(json!({"np": np, "n": 3.0 * np, "rho": r, "DS_rel_over_rho": a, "DS_fitLA_over_rho": f1, "DS_c2LA_over_rho": f2, "DS_balanced_c2LA_over_rho": bal, "lambda": lam}));
        }
        np += 0.25;
    }
    json!({
        "label": "MODEL, fitted to measured constants; not a measurement",
        "model": "rho(n') = S_top*sqrt(2^(3n')/h); DS_rel(n') = kappa*2^n'*(alpha + beta*n'); LA_fit = 2^la0 * F^gamma (OLS on rungs with F >= 256); LA_c2 = c2*F^2 (c2 = mean LA/F^2 of the top three rungs); F = fb_ratio*2^n'; balanced base F/lambda: relation phase x lambda^2, LA x lambda^-2, lambda = max(1, (LA_c2/DS_rel)^(1/4))",
        "alpha": alpha, "beta": beta, "kappa_targets_over_qp": kappa, "fb_over_qp": fbr,
        "la_fit_log2_coeff": la0, "la_fit_exponent_gamma": gam, "la_c2": c2,
        "rho_S_top": s_top, "rho_S_top_n": top.0, "h_mean": hbar,
        "crossover_np_relation_phase_only": x_rel, "crossover_np_with_fitted_LA": x_fit, "crossover_np_with_c2_LA": x_c2,
        "min_ratio_with_fitted_LA": {"ratio": min_fit.0, "np": min_fit.1},
        "min_ratio_with_c2_LA": {"ratio": min_c2.0, "np": min_c2.1},
        "min_ratio_balanced_base_c2_LA": {"ratio": min_b.0, "np": min_b.1, "lambda": min_b.2},
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
    let mut t = String::from("| n | n' | log2 ell | cases | rho S | DS/rho | DS/D | D/rho | DS rel/rho | DS LA share | LA/F^2 | rho A/A |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in s["rungs"].as_array().unwrap() {
        let g = |k: &str| fmt(&r["ratios"][k]["meq_s1_u1"]);
        t += &format!(
            "| {} | {} | {:.1} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            r["n"], r["np"], r["log2_ell"].as_f64().unwrap(), r["cases"],
            r["rho_S"].as_f64().map(|x| format!("{:.1}", x)).unwrap_or("—".into()),
            g("DS/rho"), g("DS/D"), g("D/rho"), fmt(&r["DS_relation_phase_over_rho"]),
            r["DS_census"]["la_share"].as_f64().map(|x| format!("{:.3}", x)).unwrap_or("—".into()),
            r["DS_census"]["la_over_F2"].as_f64().map(|x| format!("{:.3}", x)).unwrap_or("—".into()),
            fmt(&r["rho_AA"]),
        );
    }
    t
}

fn figure(s: &Value) -> String {
    let (w, h) = (780.0, 480.0);
    let (x0, x1, y0, y1) = (70.0, 620.0, 30.0, 410.0);
    let rungs = s["rungs"].as_array().unwrap();
    let nmin = 18.0;
    let nmax = 90.0;
    let (lmin, lmax) = (-1.0f64, 3.0f64);
    let px = |n: f64| x0 + (n - nmin) / (nmax - nmin) * (x1 - x0);
    let py = |v: f64| y1 - (v.log10().clamp(lmin, lmax) - lmin) / (lmax - lmin) * (y1 - y0);
    let mut o = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
    o += &format!("<text x='{}' y='18' font-size='14'>Gap to matched negation-map rho: cold single-target cost / rho (Meq, s=1, u=1), 95% CI</text>", x0);
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
    o += &format!("<text x='{}' y='{}' text-anchor='middle'>n (field F_2^n, subfield n' = n/3); dashed lines: model only, not measured</text>", (x0 + x1) / 2.0, y1 + 34.0);
    let mut series: Vec<(String, &str, String)> = vec![("D/rho".into(), "#7f7f7f", "D: echelon LA (#5261)".into()), ("DS/rho".into(), "#d62728", "D-S: SGE + Wiedemann (this study)".into())];
    let cols = ["#1f77b4", "#2ca02c"];
    let mut k = 0;
    if let Some(r0) = rungs.iter().find(|r| r["ratios"].as_object().map(|m| m.keys().any(|k| k.starts_with("DSB_") && k.ends_with("/rho"))).unwrap_or(false)) {
        for key in r0["ratios"].as_object().unwrap().keys() {
            if key.starts_with("DSB_") && key.ends_with("/rho") && k < 2 {
                series.push((key.clone(), cols[k], format!("{} (reduced base)", key.trim_end_matches("/rho"))));
                k += 1;
            }
        }
    }
    for (i, (key, col, lab)) in series.iter().enumerate() {
        let mut pts = vec![];
        for r in rungs {
            let v = &r["ratios"][key.as_str()]["meq_s1_u1"];
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
        o += &format!("<rect x='{}' y='{}' width='12' height='3' fill='{col}'/><text x='{}' y='{}'>{lab}</text>", x0 + 330.0, y0 + 14.0 + 16.0 * i as f64, x0 + 348.0, y0 + 18.0 + 16.0 * i as f64);
    }
    let mut pts = vec![];
    for r in rungs {
        let v = &r["DS_relation_phase_over_rho"];
        if v.is_null() {
            continue;
        }
        pts.push(format!("{},{}", px(r["n"].as_f64().unwrap()), py(v["ratio"].as_f64().unwrap())));
    }
    let ny = series.len() as f64;
    o += &format!("<polyline points='{}' fill='none' stroke='#d62728' stroke-dasharray='2,3' stroke-width='1.5'/>", pts.join(" "));
    o += &format!("<text x='{}' y='{}' fill='#d62728'>dotted: D-S relation phase only</text>", x0 + 348.0, y0 + 18.0 + 16.0 * ny);
    if let Some(g) = s["projection"]["grid"].as_array() {
        let last_n = rungs.iter().filter(|r| !r["ratios"]["DS/rho"].is_null()).map(|r| r["n"].as_f64().unwrap()).fold(0.0, f64::max);
        let mut p1 = vec![];
        let mut p2 = vec![];
        for e in g {
            let n = e["n"].as_f64().unwrap();
            if n < last_n || n > nmax {
                continue;
            }
            p1.push(format!("{},{}", px(n), py(e["DS_c2LA_over_rho"].as_f64().unwrap())));
            p2.push(format!("{},{}", px(n), py(e["DS_balanced_c2LA_over_rho"].as_f64().unwrap())));
        }
        o += &format!("<polyline points='{}' fill='none' stroke='#d62728' stroke-dasharray='8,4'/>", p1.join(" "));
        o += &format!("<polyline points='{}' fill='none' stroke='#9467bd' stroke-dasharray='8,4'/>", p2.join(" "));
        o += &format!("<text x='{}' y='{}' fill='#d62728'>dashed red: D-S model, LA = c F^2</text>", x0 + 348.0, y0 + 18.0 + 16.0 * (ny + 1.0));
        o += &format!("<text x='{}' y='{}' fill='#9467bd'>dashed purple: balanced base model</text>", x0 + 348.0, y0 + 18.0 + 16.0 * (ny + 2.0));
    }
    o += "</svg>";
    o
}
