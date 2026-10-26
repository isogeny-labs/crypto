//! W3a driver: the Smith walk on the elliptic side
//! (`research/smith_elliptic_walk_20261018/PROTOCOL.md`).
//!
//! ```text
//! cargo run --release --example smith_elliptic_walk -- exhaustive --p 7 --out DIR [--graph3] [--no-l] [--graph-json]
//! cargo run --release --example smith_elliptic_walk -- walk --p 1031 --arm A --walks 40 --seed 1 --out DIR
//! cargo run --release --example smith_elliptic_walk -- calib --out DIR
//! cargo run --release --example smith_elliptic_walk -- fit --dir DIR
//! ```
//!
//! `exhaustive` writes `exh_p<p>.json` (summary, controls, per-model rows,
//! per-weak-curve rows) and, with `--graph-json`, `graph_p<p>.json`.
//! `walk` appends one JSON object per walk to `walk_<arm>_p<p>.jsonl`.
//! `fit` reads every `walk_*.jsonl` and `calib.json` in `--dir` and writes
//! `fit.json`.

use crypto_lib::cryptanalysis::smith_elliptic_walk as w;
use crypto_lib::cryptanalysis::smith_prop24 as sp;
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::{fs, time::Instant};

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1).cloned())
}

fn flag(args: &[String], k: &str) -> bool {
    args.iter().any(|a| a == k)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn next_prime(mut n: u64) -> u64 {
    while !is_prime(n) {
        n += 1;
    }
    n
}

fn exhaustive(args: &[String]) {
    let p: u64 = arg(args, "--p").unwrap().parse().unwrap();
    let out = PathBuf::from(arg(args, "--out").unwrap());
    fs::create_dir_all(&out).unwrap();
    let t0 = Instant::now();
    let x = w::exhaustive(
        p,
        !flag(args, "--no-l"),
        flag(args, "--graph3"),
        flag(args, "--unreduced"),
    );
    let wall = t0.elapsed().as_secs_f64();
    let s = w::summarize(&x);
    // per-weak-curve rows and per-model rows (compact)
    let curves: Vec<Value> = x
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.weak)
        .map(|(i, n)| {
            json!({"id": i, "t": n.t, "pass": n.pass, "jv": n.jv_weak, "models": n.models.len(),
                   "comp2": n.comp2, "comp23": n.comp23, "deg2": n.adj2.len(), "deg3": n.adj3.len()})
        })
        .collect();
    let models: Vec<Value> = x
        .models
        .iter()
        .map(|m| {
            json!({"kind": m.spec.kind, "h": m.spec.h, "node": m.node, "ok": m.out.ok,
                   "ct": sp::ct_string(&m.out.cycle_type), "tract": m.out.n_tractable,
                   "trig": m.out.n_trig, "iso": m.out.n_iso, "iso_tw": m.out.n_iso_twist,
                   "full": m.out.full_success, "l_ok": m.l_ok})
        })
        .collect();
    let v = json!({
        "p": p, "wall_s": wall, "graph3": flag(args, "--graph3"),
        "unreduced": flag(args, "--unreduced"),
        "summary": s, "controls": x.controls, "curves": curves, "models": models,
    });
    let name = if flag(args, "--unreduced") {
        format!("exh_unreduced_p{p}.json")
    } else {
        format!("exh_p{p}.json")
    };
    fs::write(out.join(name), serde_json::to_string(&v).unwrap()).unwrap();
    if flag(args, "--graph-json") {
        let nodes: Vec<Value> = x
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| {
                json!({"id": i, "weak": n.weak, "pass": n.pass, "jv": n.jv_weak, "t": n.t,
                                 "comp2": n.comp2, "comp23": n.comp23,
                                 "key": [n.key.0, n.key.1, n.key.2], "a": n.a, "b": n.b})
            })
            .collect();
        let mut e2 = vec![];
        let mut e3 = vec![];
        for (i, n) in x.nodes.iter().enumerate() {
            for &j in &n.adj2 {
                if i < j {
                    e2.push(json!([i, j]));
                }
            }
            for &j in &n.adj3 {
                if i < j {
                    e3.push(json!([i, j]));
                }
            }
        }
        let g = json!({"p": p, "nodes": nodes, "edges2": e2, "edges3": e3});
        fs::write(
            out.join(format!("graph_p{p}.json")),
            serde_json::to_string(&g).unwrap(),
        )
        .unwrap();
    }
    println!(
        "p={p} wall={wall:.1}s models={} weak={} explored={} share_H_full={:.4} share_E_pass={:.4} C2={:.4} C23={:.4} Ct={:.4} controls={}",
        s.models,
        s.weak_nodes,
        s.explored_nodes,
        s.share_h_full,
        s.share_e_pass,
        s.comp2.share_weak_with_pass_in_comp,
        s.comp23.share_weak_with_pass_in_comp,
        s.share_class_t_pass,
        serde_json::to_string(&x.controls).unwrap()
    );
}

fn walk_cmd(args: &[String]) {
    let k_bits: u32 = arg(args, "--bits").map(|s| s.parse().unwrap()).unwrap_or(0);
    let p: u64 = if k_bits > 0 {
        next_prime(1u64 << k_bits)
    } else {
        arg(args, "--p").unwrap().parse().unwrap()
    };
    let arm = arg(args, "--arm").unwrap_or("A".into());
    let walks: u64 = arg(args, "--walks").unwrap().parse().unwrap();
    let seed: u64 = arg(args, "--seed")
        .map(|s| s.parse().unwrap())
        .unwrap_or(20261018);
    let max_nodes: u64 = arg(args, "--max-nodes")
        .map(|s| s.parse().unwrap())
        .unwrap_or(20_000_000);
    let max_secs: f64 = arg(args, "--max-secs")
        .map(|s| s.parse().unwrap())
        .unwrap_or(3300.0);
    let cell_secs: f64 = arg(args, "--cell-secs")
        .map(|s| s.parse().unwrap())
        .unwrap_or(3300.0);
    let out = PathBuf::from(arg(args, "--out").unwrap());
    fs::create_dir_all(&out).unwrap();
    let path = out.join(format!("walk_{arm}_p{p}.jsonl"));
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .unwrap();
    let k = w::Fp3::new(p);
    let cell = Instant::now();
    let mut rejected_starts = 0u64;
    let mut start_trials = 0u64;
    for i in 0..walks {
        if cell.elapsed().as_secs_f64() > cell_secs {
            eprintln!("cell budget reached after {i} walks");
            break;
        }
        let mut rng = sp::SplitMix64(
            seed ^ p.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ i.wrapping_mul(0xD1B5_4A32_D192_ED03),
        );
        rng.next_u64();
        let t0 = Instant::now();
        let (res, start_info) = if arm == "A" {
            // a failing weak start
            let start = loop {
                start_trials += 1;
                let s = w::random_weak_start(&k, &mut rng);
                let mut c = w::Cost::default();
                let (passed, _, _, _, _) = w::test_models(&k, &s, &mut rng, false, &mut c);
                if passed {
                    rejected_starts += 1;
                    continue;
                }
                break s;
            };
            k.reset();
            let r = w::walk(
                &k,
                start,
                &mut rng,
                max_nodes,
                (max_secs * 1e9) as u128,
                true,
                false,
            );
            (
                r,
                json!({"rejected_starts_so_far": rejected_starts, "start_trials_so_far": start_trials}),
            )
        } else {
            let start = w::random_full2_start(&k, &mut rng);
            k.reset();
            let r = w::walk(
                &k,
                start,
                &mut rng,
                max_nodes,
                (max_secs * 1e9) as u128,
                false,
                true,
            );
            (r, json!({}))
        };
        let wall = t0.elapsed().as_secs_f64();
        let line =
            json!({"p": p, "arm": arm, "walk": i, "wall_s": wall, "res": res, "start": start_info});
        writeln!(f, "{}", serde_json::to_string(&line).unwrap()).unwrap();
        f.flush().unwrap();
    }
    println!(
        "p={p} arm={arm} done in {:.1}s",
        cell.elapsed().as_secs_f64()
    );
}

fn calib(args: &[String]) {
    let out = PathBuf::from(arg(args, "--out").unwrap());
    fs::create_dir_all(&out).unwrap();
    let mut rows = vec![];
    for p in [1031u64, 65537, 1048583] {
        let (t_mul, inv_ratio, k_ratio) = w::calibrate(p);
        rows.push(json!({"p": p, "sec_per_fp_mul": t_mul, "inv_over_mul": inv_ratio, "kmul_over_fpmul": k_ratio}));
    }
    // Smith cost per call on a typical descended H at a mid prime
    let p = 65537;
    let k = w::Fp3::new(p);
    let mut rng = sp::SplitMix64(77);
    let mut times = vec![];
    for _ in 0..30 {
        let s = w::random_weak_start(&k, &mut rng);
        let ms = w::jv_models(&k, &s.a, &s.b, &s.roots, &mut rng, None);
        for m in ms.iter().take(2) {
            let f = w::descend(&k, &[0, m.c, 0], &m.alpha).unwrap();
            let t0 = Instant::now();
            let _ = w::smith_outcome(&f, p);
            times.push(t0.elapsed().as_secs_f64());
        }
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let v = json!({"rows": rows, "smith_median_s_p65537": times[times.len() / 2], "smith_calls": times.len()});
    fs::write(
        out.join("calib.json"),
        serde_json::to_string_pretty(&v).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

// ---------------------------------------------------------------------------
// Fit.

fn ols(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxx: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let slope = sxy / sxx;
    (slope, my - slope * mx)
}

fn fit(args: &[String]) {
    let dir = PathBuf::from(arg(args, "--dir").unwrap());
    let calib: Value =
        serde_json::from_str(&fs::read_to_string(dir.join("calib.json")).unwrap()).unwrap();
    let t_mul = calib["rows"][1]["sec_per_fp_mul"].as_f64().unwrap();
    let inv_ratio = calib["rows"][1]["inv_over_mul"].as_f64().unwrap();
    let mut per: std::collections::BTreeMap<(String, u64), Vec<Value>> = Default::default();
    for e in fs::read_dir(&dir).unwrap() {
        let e = e.unwrap();
        let name = e.file_name().to_string_lossy().to_string();
        if name.starts_with("walk_") && name.ends_with(".jsonl") {
            for line in fs::read_to_string(e.path()).unwrap().lines() {
                let v: Value = serde_json::from_str(line).unwrap();
                per.entry((
                    v["arm"].as_str().unwrap().to_string(),
                    v["p"].as_u64().unwrap(),
                ))
                .or_default()
                .push(v);
            }
        }
    }
    let mut rows = vec![];
    for ((arm, p), ws) in &per {
        let n = ws.len() as f64;
        let succ = ws
            .iter()
            .filter(|v| v["res"]["passed"].as_bool().unwrap())
            .count() as f64;
        let g = |v: &Value, path: &[&str]| -> f64 {
            let mut c = v;
            for s in path {
                c = &c[*s];
            }
            c.as_f64().unwrap_or(0.0)
        };
        let sum = |f: &dyn Fn(&Value) -> f64| -> f64 { ws.iter().map(f).sum() };
        let steps = sum(&|v| g(v, &["res", "steps"]));
        let nodes = sum(&|v| g(v, &["res", "nodes_tested"]));
        let hits = sum(&|v| g(v, &["res", "jv_weak_hits"]));
        let cost_of = |v: &Value| -> f64 {
            let mut m = 0.0;
            let mut iv = 0.0;
            for c in ["cost_step", "cost_test", "cost_descent"] {
                m += g(v, &["res", c, "mults"]);
                iv += g(v, &["res", c, "invs"]);
            }
            let smith = g(v, &["res", "smith_ns"]) * 1e-9 / t_mul;
            m + iv * inv_ratio + smith
        };
        let cost_nosmith = |v: &Value| -> f64 {
            let mut m = 0.0;
            let mut iv = 0.0;
            for c in ["cost_step", "cost_test", "cost_descent"] {
                m += g(v, &["res", c, "mults"]);
                iv += g(v, &["res", c, "invs"]);
            }
            m + iv * inv_ratio
        };
        let cost = sum(&cost_of);
        // labelled sensitivity: walks censored by a trap (step cap, trap
        // rule, isolated start) left out of the numerator
        let trapped = |v: &&Value| {
            matches!(
                v["res"]["censor_cause"].as_str(),
                Some("steps") | Some("trapped") | Some("isolated")
            )
        };
        let steps_excl: f64 = ws
            .iter()
            .filter(|v| !trapped(v))
            .map(|v| g(v, &["res", "steps"]))
            .sum();
        let cost_excl: f64 = ws.iter().filter(|v| !trapped(v)).map(&cost_of).sum();
        let cause = |c: &str| {
            ws.iter()
                .filter(|v| v["res"]["censor_cause"].as_str() == Some(c))
                .count()
        };
        let wall = sum(&|v| g(v, &["wall_s"]));
        let smith_s = sum(&|v| g(v, &["res", "smith_ns"])) * 1e-9;
        // censored-exponential means: total / successes
        let denom = succ.max(1.0);
        rows.push(json!({
            "arm": arm, "p": p, "walks": n, "successes": succ,
            "mean_steps": steps / denom, "mean_nodes": nodes / denom,
            "mean_hits": hits / denom, "mean_cost": cost / denom,
            "mean_cost_no_smith": sum(&cost_nosmith) / denom,
            "mean_wall_s": wall / denom, "mean_smith_s": smith_s / denom,
            "hit_rate": hits / nodes.max(1.0),
            "pass_per_hit": succ / hits.max(1.0),
            "steps_per_node": steps / nodes.max(1.0),
            "cost_per_step": cost / steps.max(1.0),
            "reflect_share": sum(&|v| g(v, &["res", "reflects"])) / steps.max(1.0),
            "censored": ws.iter().filter(|v| v["res"]["censored"].as_bool().unwrap()).count(),
            "censored_nodes": cause("nodes"), "censored_steps": cause("steps"),
            "censored_wall": cause("wall"), "censored_trapped": cause("trapped"),
            "censored_isolated": cause("isolated"),
            // per-operation unit costs (measured mults; Smith derived from wall)
            "step_mults_per_step": sum(&|v| g(v, &["res", "cost_step", "mults"])) / steps.max(1.0),
            "test_mults_per_node": sum(&|v| g(v, &["res", "cost_test", "mults"])) / nodes.max(1.0),
            "descent_mults_per_model": sum(&|v| g(v, &["res", "cost_descent", "mults"]))
                / sum(&|v| g(v, &["res", "models_tried"])).max(1.0),
            "smith_mults_per_call_derived": smith_s / t_mul
                / sum(&|v| g(v, &["res", "smith_calls"])).max(1.0),
            "mean_cost_excl_trapped": cost_excl / succ.max(1.0),
            // labelled sensitivity: walks stopped by the step cap (reflection
            // traps) left out of numerator and denominator
            "mean_steps_excl_trapped": steps_excl / succ.max(1.0),
        }));
    }
    // fits per arm: ln(mean) on ln p, bootstrap over walks within each p
    let mut fits = vec![];
    // `frozen` is the protocol's fit. The others are post hoc, labelled:
    // primes = 3 mod 4 only (where the walk is a non-branching path), and the
    // conditional mean over successful walks only (censored walks dropped).
    for arm in ["A", "B"] {
        for (variant, mod4_only, succ_only) in [
            ("frozen", false, false),
            ("post_hoc_p3mod4", true, false),
            ("post_hoc_successes_only", false, true),
            ("post_hoc_p3mod4_successes_only", true, true),
        ] {
            let ps: Vec<u64> = per
                .keys()
                .filter(|k| k.0 == arm && (!mod4_only || k.1 % 4 == 3))
                .map(|k| k.1)
                .collect();
            if ps.len() < 3 {
                continue;
            }
            for (metric, f) in [
                (
                    "steps",
                    &(|v: &Value| v["res"]["steps"].as_f64().unwrap()) as &dyn Fn(&Value) -> f64,
                ),
                (
                    "nodes",
                    &(|v: &Value| v["res"]["nodes_tested"].as_f64().unwrap()),
                ),
                (
                    "cost",
                    &(|v: &Value| {
                        let g = |c: &str, f: &str| v["res"][c][f].as_f64().unwrap_or(0.0);
                        let m = g("cost_step", "mults")
                            + g("cost_test", "mults")
                            + g("cost_descent", "mults");
                        let iv = g("cost_step", "invs")
                            + g("cost_test", "invs")
                            + g("cost_descent", "invs");
                        m + iv * inv_ratio
                            + v["res"]["smith_ns"].as_f64().unwrap_or(0.0) * 1e-9 / t_mul
                    }),
                ),
            ] {
                let est = |pset: &[u64], sel: &dyn Fn(&Vec<Value>) -> Vec<Value>| -> (f64, f64) {
                    let mut xs = vec![];
                    let mut ys = vec![];
                    for &p in pset {
                        let base: Vec<Value> = per[&(arm.to_string(), p)]
                            .iter()
                            .filter(|v| !succ_only || v["res"]["passed"].as_bool().unwrap())
                            .cloned()
                            .collect();
                        let ws = sel(&base);
                        let succ = ws
                            .iter()
                            .filter(|v| v["res"]["passed"].as_bool().unwrap())
                            .count() as f64;
                        if succ == 0.0 {
                            continue;
                        }
                        let tot: f64 = ws.iter().map(f).sum();
                        xs.push((p as f64).ln());
                        ys.push((tot / succ).ln());
                    }
                    ols(&xs, &ys)
                };
                let (a, b) = est(&ps, &|w| w.clone());
                let rng = std::cell::RefCell::new(sp::SplitMix64(99));
                let resample = |ws: &Vec<Value>| -> Vec<Value> {
                    (0..ws.len())
                        .map(|_| ws[rng_below(&mut rng.borrow_mut(), ws.len())].clone())
                        .collect()
                };
                let ci = |slopes: &mut Vec<f64>| -> (f64, f64) {
                    slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    (
                        slopes[(slopes.len() as f64 * 0.025) as usize],
                        slopes[(slopes.len() as f64 * 0.975) as usize],
                    )
                };
                // (i) walks resampled within each prime
                let mut slopes = vec![];
                for _ in 0..2000 {
                    let (s, _) = est(&ps, &resample);
                    if s.is_finite() {
                        slopes.push(s);
                    }
                }
                let (lo_w, hi_w) = ci(&mut slopes);
                // (ii) primes resampled with replacement, then walks within each
                // (the protocol's slope interval; the frozen verdict uses it)
                let mut slopes_p = vec![];
                for _ in 0..2000 {
                    let pset: Vec<u64> = (0..ps.len())
                        .map(|_| ps[rng_below(&mut rng.borrow_mut(), ps.len())])
                        .collect();
                    let mut uniq = pset.clone();
                    uniq.sort_unstable();
                    uniq.dedup();
                    if uniq.len() < 2 {
                        continue;
                    }
                    let (s, _) = est(&pset, &resample);
                    if s.is_finite() {
                        slopes_p.push(s);
                    }
                }
                let (lo, hi) = ci(&mut slopes_p);
                let no_success: Vec<u64> = ps
                    .iter()
                    .copied()
                    .filter(|p| {
                        !per[&(arm.to_string(), *p)]
                            .iter()
                            .any(|v| v["res"]["passed"].as_bool().unwrap())
                    })
                    .collect();
                let verdict = if !no_success.is_empty() {
                    "undetermined: prime(s) with no success"
                } else if hi < 1.0 {
                    "o(p)"
                } else if lo > 1.0 {
                    "omega(p)"
                } else {
                    "Theta~(p)"
                };
                fits.push(json!({"arm": arm, "variant": variant, "metric": metric,
                       "exponent": a, "intercept": b,
                       "ci95": [lo, hi], "ci95_walks_only": [lo_w, hi_w],
                       "verdict": verdict, "n_primes": ps.len(),
                       "primes_without_success": no_success}));
            }
        }
    }
    let v = json!({"t_mul_s": t_mul, "inv_over_mul": inv_ratio, "rows": rows, "fits": fits});
    fs::write(
        dir.join("fit.json"),
        serde_json::to_string_pretty(&v).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

fn rng_below(r: &mut sp::SplitMix64, n: usize) -> usize {
    r.below(n as u64) as usize
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("exhaustive") => exhaustive(&args),
        Some("walk") => walk_cmd(&args),
        Some("calib") => calib(&args),
        Some("fit") => fit(&args),
        _ => eprintln!("usage: exhaustive | walk | calib | fit"),
    }
}
