//! `ic_positive_control`: can the lab's index-calculus instrument report a
//! win?  Genus-3 IC on a full Jacobian (basic, single large prime, double
//! large prime) against negation-map rho, exact counts, every log certified.
//!
//! ```text
//! ic_positive_control run --primes 131,257,... [--curves K] [--seeds S]
//!     [--cb 2.0] [--cc 2.0] [--threads 3] [--json F] [--svg F] [--max-h 64]
//! ```
//!
//! Protocol: `research/ic_positive_control_20261112/PROTOCOL.md`.  All figures
//! are `[pcc:unverified]`.

use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::ic_control as ic;
use crypto_lib::cryptanalysis::chained_transfer::ic_control::{
    fb_size, jacobian_order_bsgs, random_curve, run_ic, run_rho, split_order, Arm, IcParams,
    IcResult, Instance, RhoOut,
};
use crypto_lib::cryptanalysis::ic_boundary::CountedGroup;
use serde_json::{json, Value};

/// `a b mod r` for `r < 2^50` (the lab's linear-algebra multiplication), for the
/// informational timing calibration only.
fn mulmod_r(a: u64, b: u64, r: u64) -> u64 {
    let q = ((a as f64) * (b as f64) / (r as f64)) as u64;
    let t = a.wrapping_mul(b).wrapping_sub(q.wrapping_mul(r)) as i64;
    let ri = r as i64;
    (if t < 0 {
        t + ri
    } else if t >= ri {
        t - ri
    } else {
        t
    }) as u64
}

struct Cfg {
    primes: Vec<u64>,
    curves: usize,
    curves_map: Vec<(u64, usize)>,
    seeds: usize,
    cb: f64,
    cc: f64,
    threads: usize,
    max_h: u64,
    json: Option<String>,
    svg: Option<String>,
    arms: String,
    curve_seed0: u64,
    full_cap: u64,
}

fn parse() -> Result<Cfg, String> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.first().map(String::as_str) != Some("run") {
        return Err("usage: ic_positive_control run --primes P,P,... [options]".into());
    }
    let mut c = Cfg {
        primes: vec![],
        curves: 1,
        curves_map: vec![],
        seeds: 1,
        cb: 2.0,
        cc: 2.0,
        threads: 3,
        max_h: 64,
        json: None,
        svg: None,
        arms: "ABCR".into(),
        curve_seed0: 1,
        full_cap: 1 << 14,
    };
    let mut i = 1;
    while i < a.len() {
        let v = a.get(i + 1).cloned().ok_or("missing value")?;
        match a[i].as_str() {
            "--primes" => {
                c.primes = v.split(',').map(|s| s.parse().unwrap()).collect();
            }
            "--curves" => c.curves = v.parse().unwrap(),
            "--curves-map" => {
                c.curves_map = v
                    .split(',')
                    .map(|kv| {
                        let (k, n) = kv.split_once('=').unwrap();
                        (k.parse().unwrap(), n.parse().unwrap())
                    })
                    .collect();
            }
            "--seeds" => c.seeds = v.parse().unwrap(),
            "--cb" => c.cb = v.parse().unwrap(),
            "--cc" => c.cc = v.parse().unwrap(),
            "--threads" => c.threads = v.parse().unwrap(),
            "--max-h" => c.max_h = v.parse().unwrap(),
            "--json" => c.json = Some(v),
            "--svg" => c.svg = Some(v),
            "--arms" => c.arms = v,
            "--curve-seed0" => c.curve_seed0 = v.parse().unwrap(),
            "--full-cap" => c.full_cap = v.parse().unwrap(),
            o => return Err(format!("unknown option {o}")),
        }
        i += 2;
    }
    if c.primes.is_empty() {
        return Err("--primes required".into());
    }
    Ok(c)
}

fn next_prime(mut n: u64) -> u64 {
    while !crypto_lib::cryptanalysis::residual_walk::is_prime_u64(n) {
        n += 1;
    }
    n
}

/// Instances at `p`: the first `k` random monic curves from `seed0` whose
/// `#J = h r` with `h <= max_h`.
fn instances(p: u64, k: usize, seed0: u64, max_h: u64) -> (Vec<Instance>, u64, u64) {
    let mut out = Vec::new();
    let (mut tried, mut rejected_order) = (0u64, 0u64);
    let mut cs = seed0;
    while out.len() < k && tried < 4000 {
        tried += 1;
        let cur = cs;
        cs += 1;
        let Some(curve) = random_curve(p, cur) else {
            continue;
        };
        let n2 = curve.count_points(2);
        let Ok(order) = jacobian_order_bsgs(&curve, n2, cur) else {
            rejected_order += 1;
            continue;
        };
        let Some((h, r)) = split_order(order, max_h) else {
            continue;
        };
        if let Ok(inst) = ic::instance_from(curve, order, h, r, cur * 7919 + out.len() as u64) {
            out.push((inst, cur));
        }
    }
    let seeds: Vec<u64> = out.iter().map(|x| x.1).collect();
    let _ = seeds;
    (
        out.into_iter().map(|x| x.0).collect(),
        tried,
        rejected_order,
    )
}

fn ic_json(i: &IcResult, inst: &Instance, seed: u64, cs: usize, w_rel: f64, w_la: f64) -> Value {
    json!({
        "kind": "ic", "arm": i.arm, "p": i.p, "curve_idx": cs, "seed": seed,
        "order": inst.order, "h": inst.h, "r": inst.r,
        "certified": i.certified, "exhausted": i.exhausted,
        "group_ops": i.group_ops(), "setup_ops": i.setup_ops, "walk_ops": i.walk_ops,
        "verify_ops": i.verify_ops,
        "rel_mul": i.rel.mul, "rel_add": i.rel.add,
        "la_mul": i.la.mul, "la_add": i.la.add,
        "la_graph_mul": i.la_graph.mul, "la_graph_add": i.la_graph.add,
        "la_peel_add": i.la_peel.add,
        "la_wied_mul": i.la_wied.mul, "la_wied_add": i.la_wied.add,
        "native_mul": i.native_mul(), "native_all": i.native_all(),
        "total_w0": i.total(0.0), "total_w01": i.total(0.1), "total_w1": i.total(1.0),
        "total_cal": i.group_ops() as f64 + w_rel * i.rel.mul as f64 + w_la * i.la.mul as f64,
        "fb_cols": i.fb_cols, "trials": i.trials, "splits": i.splits,
        "relations": i.relations, "rels_lp0": i.rels_by_lp[0], "rels_lp1": i.rels_by_lp[1],
        "rels_lp2": i.rels_by_lp[2], "cycles": i.cycles,
        "core_rows": i.core_rows, "core_cols": i.core_cols, "attempts": i.attempts,
        "wall_ns": i.wall_ns
    })
}

fn rho_json(o: &RhoOut, inst: &Instance, seed: u64, cs: usize, w_rel: f64) -> Value {
    json!({
        "kind": "rho", "arm": "R_rho_negation", "p": o.p, "curve_idx": cs, "seed": seed,
        "order": inst.order, "h": inst.h, "r": inst.r,
        "certified": o.certified,
        "group_ops": o.group_ops(), "walk_ops": o.walk_ops, "setup_and_other_ops": o.setup_and_other_ops,
        "native_mul": o.native, "steps": o.steps,
        "total_w0": o.total(0.0), "total_w01": o.total(0.1), "total_w1": o.total(1.0),
        "total_cal": o.group_ops() as f64 + w_rel * o.native as f64,
        "wall_ns": o.wall_ns
    })
}

/// Least-squares slope and intercept of `ln y` on `ln x`.
fn fit(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    if xs.len() < 2 {
        return None;
    }
    let n = xs.len() as f64;
    let (lx, ly): (Vec<f64>, Vec<f64>) = xs.iter().zip(ys).map(|(x, y)| (x.ln(), y.ln())).unzip();
    let mx = lx.iter().sum::<f64>() / n;
    let my = ly.iter().sum::<f64>() / n;
    let sxx: f64 = lx.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = lx.iter().zip(&ly).map(|(x, y)| (x - mx) * (y - my)).sum();
    if sxx == 0.0 {
        return None;
    }
    let s = sxy / sxx;
    Some((s, my - s * mx))
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn svg(summary: &[Value], path: &str) {
    // series: name -> Vec<(p, cost)>
    let series_defs: [(&str, &str, &str, &str, usize); 10] = [
        ("A full (w=0.1)", "A_full", "total_w01", "#8c564b", 0),
        (
            "B single LP (w=0.1)",
            "B_single_lp",
            "total_w01",
            "#9467bd",
            0,
        ),
        ("C double LP w=0", "C_double_lp", "total_w0", "#1f77b4", 0),
        (
            "C double LP w=0.1",
            "C_double_lp",
            "total_w01",
            "#2ca02c",
            0,
        ),
        ("C double LP w=1", "C_double_lp", "total_w1", "#d62728", 0),
        ("R rho w=0", "R_rho_negation", "total_w0", "#111111", 0),
        ("R rho w=0.1", "R_rho_negation", "total_w01", "#555555", 1),
        ("R rho w=1", "R_rho_negation", "total_w1", "#999999", 2),
        (
            "C double LP calibrated",
            "C_double_lp",
            "total_cal",
            "#ff7f0e",
            1,
        ),
        (
            "R rho calibrated",
            "R_rho_negation",
            "total_cal",
            "#7f7f7f",
            1,
        ),
    ];
    let mut pts: Vec<Vec<(f64, f64)>> = vec![vec![]; series_defs.len()];
    for row in summary {
        let p = row["p"].as_f64().unwrap();
        for (k, (_, arm, key, _, _)) in series_defs.iter().enumerate() {
            if let Some(v) = row["arms"][*arm][*key].as_f64() {
                pts[k].push((p, v));
            }
        }
    }
    let all: Vec<(f64, f64)> = pts.iter().flatten().copied().collect();
    if all.is_empty() {
        return;
    }
    let (x0, x1) = all
        .iter()
        .fold((f64::MAX, 0f64), |a, b| (a.0.min(b.0), a.1.max(b.0)));
    let (y0, y1) = all
        .iter()
        .fold((f64::MAX, 0f64), |a, b| (a.0.min(b.1), a.1.max(b.1)));
    let (lx0, lx1) = (x0.log10().floor(), x1.log10().ceil());
    let (ly0, ly1) = (y0.log10().floor(), y1.log10().ceil());
    let (w, h, ml, mr, mt, mb) = (860.0, 560.0, 70.0, 230.0, 40.0, 60.0);
    let sx = |x: f64| ml + (x.log10() - lx0) / (lx1 - lx0) * (w - ml - mr);
    let sy = |y: f64| h - mb - (y.log10() - ly0) / (ly1 - ly0) * (h - mt - mb);
    let mut s = String::new();
    s.push_str(&format!(
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' font-family='sans-serif' font-size='12'>\n<rect width='{w}' height='{h}' fill='white'/>\n"
    ));
    s.push_str(&format!(
        "<text x='{ml}' y='24' font-size='15'>Genus-3 full Jacobian: cost (GAE) vs p, group ops + w x native multiplications [pcc:unverified]</text>\n"
    ));
    let mut e = ly0;
    while e <= ly1 {
        let y = sy(10f64.powf(e));
        s.push_str(&format!(
            "<line x1='{ml}' y1='{y}' x2='{}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>1e{}</text>\n",
            w - mr, ml - 6.0, y + 4.0, e as i32
        ));
        e += 1.0;
    }
    let mut e = lx0;
    while e <= lx1 {
        let x = sx(10f64.powf(e));
        s.push_str(&format!(
            "<line x1='{x}' y1='{mt}' x2='{x}' y2='{}' stroke='#ddd'/><text x='{x}' y='{}' text-anchor='middle'>1e{}</text>\n",
            h - mb, h - mb + 16.0, e as i32
        ));
        e += 1.0;
    }
    s.push_str(&format!(
        "<text x='{}' y='{}' text-anchor='middle'>p</text><text x='16' y='{}' transform='rotate(-90 16 {})' text-anchor='middle'>cost (GAE)</text>\n",
        (ml + w - mr) / 2.0, h - 14.0, h / 2.0, h / 2.0
    ));
    for (k, (name, _, _, color, dash)) in series_defs.iter().enumerate() {
        if pts[k].is_empty() {
            continue;
        }
        let d: Vec<String> = pts[k]
            .iter()
            .map(|&(x, y)| format!("{:.1},{:.1}", sx(x), sy(y)))
            .collect();
        let da = match dash {
            1 => "stroke-dasharray='6 3'",
            2 => "stroke-dasharray='2 3'",
            _ => "",
        };
        s.push_str(&format!(
            "<polyline fill='none' stroke='{color}' stroke-width='2' {da} points='{}'/>\n",
            d.join(" ")
        ));
        for &(x, y) in &pts[k] {
            s.push_str(&format!(
                "<circle cx='{:.1}' cy='{:.1}' r='3' fill='{color}'/>\n",
                sx(x),
                sy(y)
            ));
        }
        let ly = mt + 20.0 + k as f64 * 20.0;
        s.push_str(&format!(
            "<line x1='{}' y1='{ly}' x2='{}' y2='{ly}' stroke='{color}' stroke-width='2' {da}/><text x='{}' y='{}'>{name}</text>\n",
            w - mr + 12.0, w - mr + 40.0, w - mr + 46.0, ly + 4.0
        ));
    }
    s.push_str("</svg>\n");
    let _ = std::fs::write(path, s);
}

fn main() -> ExitCode {
    let cfg = match parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    let t0 = Instant::now();
    let mut all_rows: Vec<Value> = Vec::new();
    let mut summary: Vec<Value> = Vec::new();
    let mut all_certified = true;
    // calibration (informational): ns per Cantor add and per mulmod
    let mut cal = json!({});
    for &p0 in &cfg.primes {
        let p = next_prime(p0);
        let ti = Instant::now();
        let k_curves = cfg
            .curves_map
            .iter()
            .find(|(q, _)| *q == p)
            .map(|x| x.1)
            .unwrap_or(cfg.curves);
        let (insts, tried, rej) = instances(p, k_curves, cfg.curve_seed0, cfg.max_h);
        println!(
            "p={p}: {} instance(s) after {tried} curves ({rej} order rejections) in {:.1}s",
            insts.len(),
            ti.elapsed().as_secs_f64()
        );
        if insts.is_empty() {
            println!("p={p}: no instance, skipped");
            continue;
        }
        for (ci, inst) in insts.iter().enumerate() {
            println!(
                "  curve {ci}: #J={} = {} x {} (r ~ p^{:.3})",
                inst.order,
                inst.h,
                inst.r,
                (inst.r as f64).ln() / (p as f64).ln()
            );
        }
        // calibration (informational): ns per Cantor add, per mulmod mod p, per mulmod mod r
        let (ns_add, ns_mp, ns_mr) = {
            let inst = &insts[0];
            let mut best_add = f64::MAX;
            for _ in 0..5 {
                let mut ops = Default::default();
                let mut x = inst.g;
                let tt = Instant::now();
                let n = 2000;
                for _ in 0..n {
                    x = inst.curve.add(&mut ops, x, inst.target);
                }
                best_add = best_add.min(tt.elapsed().as_nanos() as f64 / n as f64);
                std::hint::black_box(x);
            }
            let mut best_p = f64::MAX;
            let mut best_r = f64::MAX;
            for _ in 0..5 {
                let n = 4_000_000u64;
                let mut x = 12345u64 % p;
                let tt = Instant::now();
                for _ in 0..n {
                    x = (x * x + 1) % p;
                }
                best_p = best_p.min(tt.elapsed().as_nanos() as f64 / n as f64);
                std::hint::black_box(x);
                let r = inst.r;
                let mut y = 12345u64 % r;
                let tt = Instant::now();
                for _ in 0..n {
                    y = mulmod_r(y, y, r) + 1;
                    if y >= r {
                        y -= r;
                    }
                }
                best_r = best_r.min(tt.elapsed().as_nanos() as f64 / n as f64);
                std::hint::black_box(y);
            }
            (best_add, best_p, best_r)
        };
        let (w_rel, w_la) = (ns_mp / ns_add, ns_mr / ns_add);
        cal[p.to_string()] = json!({
            "ns_per_group_add": ns_add, "ns_per_mulmod_mod_p": ns_mp, "ns_per_mulmod_mod_r": ns_mr,
            "w_rel": w_rel, "w_la": w_la
        });
        println!("  calibration: {ns_add:.0} ns/group add, {ns_mp:.2} ns/mulmod mod p, {ns_mr:.2} ns/mulmod mod r");
        // jobs
        struct Job {
            ci: usize,
            arm: char,
            seed: u64,
        }
        let mut jobs: Vec<Job> = Vec::new();
        for ci in 0..insts.len() {
            for s in 0..cfg.seeds {
                for arm in cfg.arms.chars() {
                    if arm == 'A' && p > cfg.full_cap {
                        continue;
                    }
                    jobs.push(Job {
                        ci,
                        arm,
                        seed: 1000 * (s as u64 + 1) + ci as u64,
                    });
                }
            }
        }
        // biggest first
        jobs.sort_by_key(|j| match j.arm {
            'R' => 0,
            'A' => 1,
            'B' => 2,
            _ => 3,
        });
        let next = AtomicUsize::new(0);
        let rows: Mutex<Vec<(usize, Value)>> = Mutex::new(Vec::new());
        let bad = Mutex::new(false);
        std::thread::scope(|sc| {
            for _ in 0..cfg.threads {
                sc.spawn(|| loop {
                    let k = next.fetch_add(1, Ordering::Relaxed);
                    if k >= jobs.len() {
                        break;
                    }
                    let j = &jobs[k];
                    let inst = &insts[j.ci];
                    let v = match j.arm {
                        'R' => {
                            let o = run_rho(inst, j.seed);
                            if !o.certified {
                                *bad.lock().unwrap() = true;
                            }
                            rho_json(&o, inst, j.seed, j.ci, w_rel)
                        }
                        a => {
                            let arm = match a {
                                'A' => Arm::Full,
                                'B' => Arm::SingleLp,
                                _ => Arm::DoubleLp,
                            };
                            let o = run_ic(
                                inst,
                                IcParams {
                                    arm,
                                    nb: fb_size(arm, p, cfg.cb, cfg.cc),
                                    seed: j.seed,
                                    max_trials: 400 * p * p.min(1 << 12),
                                    verify_relations: false,
                                },
                            );
                            if !o.certified {
                                *bad.lock().unwrap() = true;
                            }
                            ic_json(&o, inst, j.seed, j.ci, w_rel, w_la)
                        }
                    };
                    println!(
                        "    p={p} curve={} arm={} seed={} cert={} group_ops={} total_w1={:.3e}",
                        j.ci,
                        v["arm"].as_str().unwrap(),
                        j.seed,
                        v["certified"],
                        v["group_ops"],
                        v["total_w1"].as_f64().unwrap()
                    );
                    rows.lock().unwrap().push((k, v));
                });
            }
        });
        if *bad.lock().unwrap() {
            all_certified = false;
        }
        let mut rv = rows.into_inner().unwrap();
        rv.sort_by_key(|x| x.0);
        let rv: Vec<Value> = rv.into_iter().map(|x| x.1).collect();
        // per-arm means
        let mut arms = serde_json::Map::new();
        for name in ["A_full", "B_single_lp", "C_double_lp", "R_rho_negation"] {
            let rs: Vec<&Value> = rv.iter().filter(|r| r["arm"] == name).collect();
            if rs.is_empty() {
                continue;
            }
            let g = |k: &str| {
                mean(
                    &rs.iter()
                        .map(|r| r[k].as_f64().unwrap())
                        .collect::<Vec<_>>(),
                )
            };
            let certified = rs.iter().all(|r| r["certified"] == true);
            let mut o = json!({
                "n": rs.len(), "certified": certified,
                "group_ops": g("group_ops"), "native_mul": g("native_mul"),
                "total_w0": g("total_w0"), "total_w01": g("total_w01"), "total_w1": g("total_w1"), "total_cal": g("total_cal"),
            });
            if name != "R_rho_negation" {
                for k in [
                    "rel_mul",
                    "rel_add",
                    "la_mul",
                    "la_add",
                    "setup_ops",
                    "walk_ops",
                    "trials",
                    "relations",
                ] {
                    o[k] = json!(g(k));
                }
                o["native_all"] = json!(g("native_all"));
            } else {
                for k in ["walk_ops", "setup_and_other_ops", "steps"] {
                    o[k] = json!(g(k));
                }
            }
            arms.insert(name.into(), o);
        }
        summary.push(json!({"p": p, "n_curves": insts.len(), "arms": arms}));
        all_rows.extend(rv);
    }

    // fits and crossovers
    let mut fits = serde_json::Map::new();
    let mut cross = serde_json::Map::new();
    for name in ["A_full", "B_single_lp", "C_double_lp", "R_rho_negation"] {
        for key in [
            "group_ops",
            "total_w0",
            "total_w01",
            "total_w1",
            "total_cal",
        ] {
            let (xs, ys): (Vec<f64>, Vec<f64>) = summary
                .iter()
                .filter_map(|s| {
                    s["arms"][name][key]
                        .as_f64()
                        .map(|v| (s["p"].as_f64().unwrap(), v))
                })
                .filter(|(p, _)| *p >= 500.0)
                .unzip();
            if let Some((sl, ic)) = fit(&xs, &ys) {
                fits.insert(
                    format!("{name}/{key}"),
                    json!({"slope": sl, "coef": ic.exp(), "points_p": xs, "n_points": xs.len()}),
                );
            }
        }
    }
    for (key, label) in [
        ("total_w0", "0"),
        ("total_w01", "0.1"),
        ("total_w1", "1"),
        ("total_cal", "cal"),
    ] {
        let first = summary.iter().find_map(|s| {
            let c = s["arms"]["C_double_lp"][key].as_f64()?;
            let r = s["arms"]["R_rho_negation"][key].as_f64()?;
            let ok = s["arms"]["C_double_lp"]["certified"] == true
                && s["arms"]["R_rho_negation"]["certified"] == true;
            (c < r && ok).then(|| s["p"].clone())
        });
        let fc = fits.get(&format!("C_double_lp/{key}"));
        let fr = fits.get(&format!("R_rho_negation/{key}"));
        let extrap = match (fc, fr) {
            (Some(a), Some(b)) => {
                let (sa, ca) = (a["slope"].as_f64().unwrap(), a["coef"].as_f64().unwrap());
                let (sb, cb) = (b["slope"].as_f64().unwrap(), b["coef"].as_f64().unwrap());
                if sa < sb {
                    json!((cb / ca).powf(1.0 / (sa - sb)))
                } else {
                    Value::Null
                }
            }
            _ => Value::Null,
        };
        cross.insert(
            label.into(),
            json!({"first_p_C_below_R": first, "fit_crossover_p": extrap}),
        );
    }
    let strict = summary.iter().find_map(|s| {
        let ok = ["total_w0", "total_w01", "total_w1"].iter().all(|key| {
            match (
                s["arms"]["C_double_lp"][*key].as_f64(),
                s["arms"]["R_rho_negation"][*key].as_f64(),
            ) {
                (Some(c), Some(r)) => c < r,
                _ => false,
            }
        });
        ok.then(|| s["p"].clone())
    });
    let out = json!({
        "schema": "ic_positive_control/v1",
        "claims": "[pcc:unverified]",
        "cfg": {"primes": cfg.primes, "curves": cfg.curves, "seeds": cfg.seeds, "cb": cfg.cb, "cc": cfg.cc,
                "max_h": cfg.max_h, "arms": cfg.arms, "full_cap": cfg.full_cap, "curve_seed0": cfg.curve_seed0},
        "all_certified": all_certified,
        "calibration_ns_informational": cal,
        "summary": summary,
        "fits": fits,
        "crossover": cross,
        "strict_all_prices_first_p": strict,
        "rows": all_rows,
        "wall_s": t0.elapsed().as_secs_f64(),
    });
    if let Some(j) = &cfg.json {
        std::fs::write(j, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    }
    if let Some(sv) = &cfg.svg {
        svg(out["summary"].as_array().unwrap(), sv);
    }
    println!(
        "all_certified={all_certified} wall={:.1}s",
        t0.elapsed().as_secs_f64()
    );
    println!("crossover={}", out["crossover"]);
    println!(
        "strict_all_prices_first_p={}",
        out["strict_all_prices_first_p"]
    );
    if all_certified {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
