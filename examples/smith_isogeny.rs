//! W3b driver: Smith's explicit isogeny `psi` on divisor classes
//! (`cryptanalysis::smith_isogeny`, `cryptanalysis::smith_route_s`), protocol
//! `research/smith_isogeny_divisor_map_20261018/PROTOCOL.md`.
//!
//! ```text
//! cargo run --release --example smith_isogeny -- verify  --q 11   # T1-T6, T1(f), image structure
//! cargo run --release --example smith_isogeny -- deg8    --q 11   # T1 on degree-8 models
//! cargo run --release --example smith_isogeny -- route-s --q 7    # T8 end to end
//! cargo run --release --example smith_isogeny -- cost    --q 65537 # T7 psi cost
//! ```
//! Every run appends JSONL rows to `--out DIR` (default the study's `data/`),
//! file `<cmd>_q<q>.jsonl`.

use crypto_lib::cryptanalysis::smith_isogeny as si;
use crypto_lib::cryptanalysis::smith_prop24::{
    analyze_curve, h_point_count, random_curve, x_point_count, SplitMix64,
};
use crypto_lib::cryptanalysis::smith_route_s as rs;
use serde_json::json;
use std::io::Write;
use std::time::Instant;

const SEED: u64 = 20261018;

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1).cloned())
}

fn emit(out: &str, name: &str, v: serde_json::Value) {
    std::fs::create_dir_all(out).unwrap();
    let path = format!("{out}/{name}.jsonl");
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .unwrap();
    writeln!(f, "{}", v).unwrap();
    println!("{}", v);
}

fn git_head() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).cloned().unwrap_or_default();
    let q: u64 = arg(&args, "--q").map(|s| s.parse().unwrap()).unwrap_or(11);
    let out = arg(&args, "--out")
        .unwrap_or_else(|| "research/smith_isogeny_divisor_map_20261018/data".into());
    let ncurves: usize = arg(&args, "--curves")
        .map(|s| s.parse().unwrap())
        .unwrap_or(6);
    let name = format!("{}_q{}", cmd.replace('-', "_"), q);
    if !matches!(cmd.as_str(), "cost-fit" | "summarize") {
        emit(
            &out,
            &name,
            json!({"rec": "start", "cmd": cmd, "q": q, "seed": SEED,
        "git_head": git_head(), "args": args}),
        );
    }
    match cmd.as_str() {
        "verify" => verify(q, ncurves, &out, &name, &args),
        "deg8" => deg8(q, ncurves, &out, &name),
        "route-s" => route_s(q, &out, &name, &args),
        "cost" => cost(q, &out, &name, &args),
        "cost-fit" => cost_fit(&out),
        "summarize" => summarize(&out),
        _ => eprintln!("usage: smith_isogeny verify|deg8|route-s|cost --q Q [--out DIR]"),
    }
}

fn route_s(q: u64, out: &str, name: &str, args: &[String]) {
    let mi: usize = arg(args, "--max-instances")
        .map(|s| s.parse().unwrap())
        .unwrap_or(usize::MAX);
    let md: usize = arg(args, "--max-dlp")
        .map(|s| s.parse().unwrap())
        .unwrap_or(usize::MAX);
    let cap: f64 = arg(args, "--cap-s")
        .map(|s| s.parse().unwrap())
        .unwrap_or(3600.0);
    let t = Instant::now();
    let run = rs::route_s(q as u32, mi, md, cap, SEED);
    for r in &run.records {
        emit(out, name, json!({"rec": "instance", "data": r}));
    }
    let c = |f: &dyn Fn(&rs::RouteSRecord) -> bool| run.records.iter().filter(|r| f(r)).count();
    emit(
        out,
        name,
        json!({"rec": "summary", "q": q,
        "split_candidates": run.split_candidates,
        "filter_instances": run.filter_instances,
        "old_rule_instances": run.old_rule_instances,
        "instances_run": run.records.len(),
        "not_reached_time_cap": run.not_reached_time_cap,
        "build_errors": c(&|r| r.build_error.is_some()),
        "h_ok": c(&|r| r.h_ok),
        "prop2_any_map": c(&|r| r.prop2_maps > 0),
        "prop4_pass": c(&|r| r.prop4_pass),
        "prop4_pass_old_rule": c(&|r| r.prop4_pass && r.old_cofactor_rule),
        "h_ok_old_rule": c(&|r| r.h_ok && r.old_cofactor_rule),
        "dlp_attempted": c(&|r| r.dlp_attempted),
        "quartic_t1_pass": c(&|r| r.quartic_t1_pass),
        "n_jac_equals_order_e0": c(&|r| r.n_jac_equals_order_e0),
        "transfer_ok": c(&|r| r.transfer_hom_ok && r.transfer_order_ok),
        "psi_g_ok": c(&|r| r.psi_g_nonzero && r.psi_g_order_ell),
        "k_found": c(&|r| r.k_found.is_some()),
        "verified_on_e": c(&|r| r.verified_on_e),
        "k_equals_planted": c(&|r| r.k_found == Some(r.k_planted)),
        "skipped": c(&|r| r.skip_reason.is_some()),
        "seconds": t.elapsed().as_secs_f64()}),
    );
}

fn verify(q: u64, ncurves: usize, out: &str, name: &str, args: &[String]) {
    let sizes: Vec<usize> = arg(args, "--sizes")
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![30, 30, 100, 3]);
    let image = arg(args, "--image").map(|s| s == "1").unwrap_or(q <= 13);
    let deg6 = arg(args, "--deg6").map(|s| s == "1").unwrap_or(true);
    let mut found = 0;
    let mut idx = 0u64;
    while found < ncurves {
        let f = si::random_curve7(q, SEED, idx);
        let mut rng = SplitMix64(SEED ^ (idx << 8) ^ q);
        idx += 1;
        let sds = si::smith_data_with_subgroups(&f, q, &mut rng);
        let Some((sd, sg)) = sds.iter().find(|(s, _)| s.rational()).cloned() else {
            continue;
        };
        found += 1;
        let t0 = Instant::now();
        let label = sd.label.clone();
        let sm = si::Smith::new(sd);
        let (cr, quartic) = sm.t1_curve(&mut rng);
        let mut row = json!({"rec": "curve", "q": q, "index": idx - 1, "scanned": idx, "f": f,
            "label": label, "maps": sds.len(), "t1": cr, "t1_pass": cr.pass()});
        if let Some(qc) = quartic {
            row["quartic"] = json!(qc.f.coeffs());
            match si::W1Bridge::new(&qc) {
                Ok(w1) => {
                    let s = sm.draw_sample(
                        &sg,
                        cr.n_jac as u128,
                        (sizes[0], sizes[1], sizes[2], sizes[3]),
                        &mut rng,
                    );
                    row["n_s_rational"] = json!(s.n_s_rational);
                    row["base_fallbacks"] = json!(s.base_fallbacks);
                    row["t5_ells"] = json!(si::odd_prime_factors(cr.n_jac as u128));
                    row["w1"] = json!(si::run_engine(&w1, &s));
                    if deg6 {
                        match si::Pic::new(qc.clone(), SEED ^ idx) {
                            Ok(pic) => row["deg6"] = json!(si::run_engine(&pic, &s)),
                            Err(e) => row["deg6_error"] = json!(format!("{:?}", e)),
                        }
                    }
                    if image {
                        row["image"] = json!(si::image_structure(
                            &sm,
                            &w1,
                            &sg,
                            cr.n_jac as u128,
                            &mut rng
                        ));
                    }
                    row["w1_bad_cubic_space"] = json!(w1.bad_cubic_space.get());
                }
                Err(e) => row["w1_error"] = json!(e),
            }
        }
        row["seconds"] = json!(t0.elapsed().as_secs_f64());
        emit(out, name, row);
    }
    emit(
        out,
        name,
        json!({"rec": "scan", "q": q, "curves_scanned": idx, "curves_used": found}),
    );
}

fn deg8(q: u64, ncurves: usize, out: &str, name: &str) {
    let mut found = 0;
    let mut idx = 0u64;
    // T1(f): twist relation N_1(X) = 2(q + 1) - N_1(H) on maps with lc(s) a
    // non-square (x_point_count exists for deg F = 8 and k = 1 only)
    let (mut tw_agree, mut tw_disagree, mut tw_na, mut tw_maps) = (0u64, 0u64, 0u64, 0u64);
    while found < ncurves {
        let (f, mut rng) = random_curve(q, SEED, idx);
        idx += 1;
        if let Some(co) = analyze_curve(&f, q, &mut rng.clone()) {
            let h = h_point_count(&f, q) as i64;
            for s in co.subgroups.iter().filter(|s| s.trig_rational()) {
                for m in s.maps.iter().filter(|m| m.valid() && m.alpha_chi == -1) {
                    tw_maps += 1;
                    match (h != q as i64 + 1)
                        .then(|| x_point_count(&f, &m.map, q))
                        .flatten()
                    {
                        None => tw_na += 1,
                        Some(x) if x as i64 == 2 * (q as i64 + 1) - h => tw_agree += 1,
                        Some(_) => tw_disagree += 1,
                    }
                }
            }
        }
        let sds = si::smith_data_with_subgroups(&f, q, &mut rng);
        let Some((sd, _)) = sds.iter().find(|(s, _)| s.rational()).cloned() else {
            continue;
        };
        found += 1;
        let sm = si::Smith::new(sd);
        let (cr, _) = sm.t1_curve(&mut rng);
        emit(
            out,
            name,
            json!({"rec": "curve_deg8", "q": q, "index": idx - 1, "scanned": idx,
            "f": f, "t1": cr, "t1_pass": cr.pass()}),
        );
    }
    emit(
        out,
        name,
        json!({"rec": "scan", "q": q, "curves_scanned": idx, "curves_used": found,
        "t1f_twist_maps": tw_maps, "t1f_agree": tw_agree, "t1f_disagree": tw_disagree,
        "t1f_not_applicable": tw_na}),
    );
}

fn cost(q: u64, out: &str, name: &str, args: &[String]) {
    let n: usize = arg(args, "--n").map(|s| s.parse().unwrap()).unwrap_or(200);
    let mut idx = 0u64;
    let (sm, scanned) = loop {
        let f = si::random_curve7(q, SEED, idx);
        let mut rng = SplitMix64(SEED ^ (idx << 8) ^ q);
        idx += 1;
        if let Some((sd, _)) = si::smith_data_with_subgroups(&f, q, &mut rng)
            .into_iter()
            .find(|(s, _)| s.rational())
        {
            break (si::Smith::new(sd), idx);
        }
    };
    let mut rng = SplitMix64(SEED ^ q);
    let ds: Vec<si::Mum> = (0..n).map(|_| sm.h.random_generic3(&mut rng)).collect();
    let (inv_ratio, mul_ns) = si::inv_mul_ratio(q, 6);
    let (mut muls, mut invs, mut ns) = (Vec::new(), Vec::new(), Vec::new());
    let mut degenerate = 0;
    for d in &ds {
        sm.reset_counters();
        let t = Instant::now();
        let r = sm.psi_effective(d, &mut rng);
        let el = t.elapsed().as_nanos() as f64;
        if r.is_err() {
            degenerate += 1;
            continue;
        }
        let (m, i) = sm.counters();
        muls.push(m as f64);
        invs.push(i as f64);
        ns.push(el);
    }
    let q4 = |v: &mut Vec<f64>| -> [f64; 3] {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if v.is_empty() {
            return [0.0; 3];
        }
        [v[v.len() / 4], v[v.len() / 2], v[3 * v.len() / 4]]
    };
    let (mm, mi, mt) = (q4(&mut muls), q4(&mut invs), q4(&mut ns));
    // an F_{q^6} multiplication counts 6^2 + 6*5 = 66 F_q multiplications
    let fq6_mul_units = 66.0;
    emit(
        out,
        name,
        json!({"rec": "cost", "q": q, "curve_index": scanned - 1, "n": n,
        "degenerate": degenerate, "fq_mul_q1_med_q3": mm, "fqd_inv_q1_med_q3": mi,
        "inv_over_fq6_mul_time": inv_ratio, "fq6_mul_ns": mul_ns,
        "inv_price_fq_mul": inv_ratio * fq6_mul_units,
        "priced_fq_mul_median": mm[1] + inv_ratio * fq6_mul_units * mi[1],
        "wall_ns_q1_med_q3": mt}),
    );
}

/// T7 fit: OLS of log2(cost) on log2(q) over the recorded `cost` rows, with a
/// 95% t-interval on the slope (`out/cost_fit.json`).
fn cost_fit(out: &str) {
    let mut pts: Vec<(f64, f64, f64, f64)> = Vec::new(); // (q, priced, mul, wall)
    for e in std::fs::read_dir(out).unwrap() {
        let path = e.unwrap().path();
        let n = path.file_name().unwrap().to_string_lossy().to_string();
        if !n.starts_with("cost_q") || !n.ends_with(".jsonl") {
            continue;
        }
        for line in std::fs::read_to_string(&path).unwrap().lines() {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            if v["rec"] == "cost" {
                pts.push((
                    v["q"].as_f64().unwrap(),
                    v["priced_fq_mul_median"].as_f64().unwrap(),
                    v["fq_mul_q1_med_q3"][1].as_f64().unwrap(),
                    v["wall_ns_q1_med_q3"][1].as_f64().unwrap(),
                ));
            }
        }
    }
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let fit = |ys: Vec<f64>| -> serde_json::Value {
        let xs: Vec<f64> = pts.iter().map(|p| p.0.log2()).collect();
        let ys: Vec<f64> = ys.iter().map(|y| y.log2()).collect();
        let n = xs.len() as f64;
        let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
        let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
        let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
        let b = sxy / sxx;
        let a = my - b * mx;
        let rss: f64 = xs
            .iter()
            .zip(&ys)
            .map(|(x, y)| (y - a - b * x).powi(2))
            .sum();
        let se = (rss / (n - 2.0) / sxx).sqrt();
        // t_{0.975, df} for df = n - 2 (table, df 1..30)
        let tt = [
            12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201, 2.179,
            2.160, 2.145, 2.131, 2.120, 2.110, 2.101, 2.093, 2.086,
        ];
        let t = tt[((n as usize).saturating_sub(3)).min(tt.len() - 1)];
        json!({"n": n, "slope": b, "intercept": a, "slope_lo": b - t * se, "slope_hi": b + t * se, "t": t})
    };
    let v = json!({"rec": "cost_fit", "x": "log2 q", "y": "log2 median per psi(D)",
        "points": pts.iter().map(|p| json!({"q": p.0, "priced": p.1, "mul": p.2, "wall_ns": p.3})).collect::<Vec<_>>(),
        "priced": fit(pts.iter().map(|p| p.1).collect()),
        "mul": fit(pts.iter().map(|p| p.2).collect()),
        "wall": fit(pts.iter().map(|p| p.3).collect())});
    std::fs::write(
        format!("{out}/cost_fit.json"),
        serde_json::to_string_pretty(&v).unwrap(),
    )
    .unwrap();
    println!("{}", v);
}

/// Totals over the recorded rows (`out/summary.json`): per q and engine the
/// pass / fail / skip counts of T2-T6, T1 outcomes, image checks, T1(f), and
/// the Route S funnel.  Reads the JSONL files only; computes nothing new.
fn summarize(out: &str) {
    use serde_json::Value;
    let read = |n: &str| -> Vec<Value> {
        std::fs::read_to_string(format!("{out}/{n}"))
            .map(|t| {
                t.lines()
                    .map(|l| serde_json::from_str(l).unwrap())
                    .collect()
            })
            .unwrap_or_default()
    };
    let checks = [
        "t2_well_defined",
        "t3_homomorphism",
        "t4_kernel_s",
        "t4_outside_nonzero",
        "t5_order_nonzero",
        "t5_order_annihilated",
        "t6_group_order",
    ];
    let mut verify = Vec::new();
    let mut deg8 = Vec::new();
    let mut routes = Vec::new();
    for q in [7u64, 11, 13, 17, 19, 23] {
        let rows = read(&format!("verify_q{q}.jsonl"));
        let curves: Vec<&Value> = rows.iter().filter(|r| r["rec"] == "curve").collect();
        let scan = rows
            .iter()
            .find(|r| r["rec"] == "scan")
            .cloned()
            .unwrap_or(Value::Null);
        let mut eng = serde_json::Map::new();
        for e in ["w1", "deg6"] {
            let mut m = serde_json::Map::new();
            for c in checks {
                let (mut p, mut f, mut s) = (0u64, 0u64, 0u64);
                for r in &curves {
                    if let Some(t) = r.get(e).and_then(|x| x.get(c)) {
                        p += t["pass"].as_u64().unwrap();
                        f += t["fail"].as_u64().unwrap();
                        s += t["skip"].as_u64().unwrap();
                    }
                }
                m.insert(c.into(), json!({"pass": p, "fail": f, "skip": s}));
            }
            eng.insert(e.into(), Value::Object(m));
        }
        let t1_pass = curves.iter().filter(|r| r["t1_pass"] == true).count();
        let hyper = curves
            .iter()
            .filter(|r| {
                r["t1"]["fit_error"]
                    .as_str()
                    .map_or(false, |s| s.contains("hyperelliptic"))
            })
            .count();
        let holdout: u64 = curves
            .iter()
            .map(|r| r["t1"]["holdout_points"].as_u64().unwrap_or(0))
            .sum();
        let holdout_fail: u64 = curves
            .iter()
            .map(|r| r["t1"]["holdout_failures"].as_u64().unwrap_or(0))
            .sum();
        let imgs: Vec<&Value> = curves.iter().filter_map(|r| r.get("image")).collect();
        let img_ok = imgs
            .iter()
            .filter(|i| {
                i["kernel_equals_s"] == true
                    && i["errors"] == 0
                    && i["fibre_min"] == i["n_s_rational"]
                    && i["fibre_max"] == i["n_s_rational"]
                    && i["n_jac_enumerated"].as_u64() == i["n_jac_lpoly"].as_u64()
            })
            .count();
        let img_elems: u64 = imgs
            .iter()
            .map(|i| i["n_jac_enumerated"].as_u64().unwrap())
            .sum();
        let img_retry: u64 = imgs
            .iter()
            .map(|i| i["first_try_errors"].as_u64().unwrap())
            .sum();
        verify.push(
            json!({"q": q, "curves": curves.len(), "curves_scanned": scan["curves_scanned"],
            "t1_pass": t1_pass, "x_hyperelliptic": hyper, "holdout_points": holdout,
            "holdout_failures": holdout_fail, "engines": eng, "image_curves": imgs.len(),
            "image_ok": img_ok, "image_elements": img_elems, "image_first_try_errors": img_retry}),
        );
        let d8 = read(&format!("deg8_q{q}.jsonl"));
        let dc: Vec<&Value> = d8.iter().filter(|r| r["rec"] == "curve_deg8").collect();
        let ds = d8
            .iter()
            .find(|r| r["rec"] == "scan")
            .cloned()
            .unwrap_or(Value::Null);
        deg8.push(json!({"q": q, "curves": dc.len(),
            "t1_pass": dc.iter().filter(|r| r["t1_pass"] == true).count(),
            "x_hyperelliptic": dc.iter().filter(|r| r["t1"]["fit_error"].as_str().is_some_and(|s| s.contains("hyperelliptic"))).count(),
            "t1f": {"maps": ds["t1f_twist_maps"], "agree": ds["t1f_agree"], "disagree": ds["t1f_disagree"], "not_applicable": ds["t1f_not_applicable"]}}));
        let rs_rows = read(&format!("route_s_q{q}.jsonl"));
        if let Some(sm) = rs_rows.iter().find(|r| r["rec"] == "summary") {
            routes.push(sm.clone());
        }
    }
    let v = json!({"verify": verify, "deg8": deg8, "route_s": routes});
    std::fs::write(
        format!("{out}/summary.json"),
        serde_json::to_string_pretty(&v).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}
