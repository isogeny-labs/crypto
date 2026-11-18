use g4audit::cryptanalysis::{
    genus_ladder as gl,
    ic_boundary::{rho_reference_walk, RhoWalk},
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};
mod independent;
fn rows(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
fn n(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or(0)
}
fn arm(v: &Value) -> &str {
    let a = v["arm"].as_str().unwrap_or("");
    if a == "c" || a == "v" || a == "w" || a == "R" || a == "P" {
        a
    } else if a.contains("unitT") {
        "w"
    } else if a.contains("coord") {
        "v"
    } else if a.starts_with("C_") {
        "c"
    } else if a.contains("plain") {
        "P"
    } else if a.starts_with('R') {
        "R"
    } else {
        a
    }
}
fn sum(v: &[&Value], k: &str) -> f64 {
    v.iter().map(|x| n(x, k) as f64).sum()
}
fn cost(v: &Value, w: f64, mode: &str) -> f64 {
    let g = n(v, "group_ops") as f64;
    let m = (n(v, "rel_mul") + n(v, "la_mul")) as f64;
    let a = (n(v, "rel_add") + n(v, "la_add")) as f64;
    if v.get("rel_mul").is_some() {
        g + w
            * (m + if mode == "Q1" || mode == "strictmul" {
                0.
            } else {
                a
            })
    } else {
        let c = n(v, "native") as f64;
        let k = n(v, "compares_uncharged") as f64;
        g + w * match mode {
            "strictmul" => 0.,
            "Q3" => c + k,
            "Q4" => c + k + 3. * n(v, "steps") as f64,
            "lean_floor" => c,
            "lean_actual" => {
                if arm(v) == "P" {
                    0.
                } else {
                    c
                }
            }
            _ => c,
        }
    }
}
fn boot(v: &[&Value], d: &[&Value], w: f64, mode: &str, b: usize) -> Value {
    let mut groups: BTreeMap<(u64, u64), Vec<(&Value, &Value)>> = BTreeMap::new();
    for x in v {
        let y = d
            .iter()
            .find(|y| {
                n(y, "curve_seed") == n(x, "curve_seed")
                    && n(y, "target_seed") == n(x, "target_seed")
                    && n(y, "seed") == n(x, "seed")
            })
            .expect("paired row");
        groups
            .entry((n(x, "curve_seed"), n(x, "target_seed")))
            .or_default()
            .push((x, y));
    }
    let gg: Vec<_> = groups.values().collect();
    let mut rng = StdRng::seed_from_u64(20261116);
    let mut bs = Vec::with_capacity(b);
    for _ in 0..b {
        let (mut a, mut z) = (0., 0.);
        for _ in 0..gg.len() {
            let g = gg[rng.gen_range(0..gg.len())];
            for _ in 0..g.len() {
                let (x, y) = g[rng.gen_range(0..g.len())];
                a += cost(x, w, mode);
                z += cost(y, w, mode);
            }
        }
        bs.push(a / z);
    }
    bs.sort_by(f64::total_cmp);
    json!([bs[b / 40], bs[b * 39 / 40]])
}
fn recalc(path: &str) {
    let rr = rows(path);
    let runs: Vec<_> = rr.iter().filter(|x| x["kind"] == "run").collect();
    let primes: BTreeSet<_> = runs.iter().map(|x| n(x, "p")).collect();
    let mut out = Vec::new();
    for p in primes {
        let ww: Vec<_> = runs
            .iter()
            .copied()
            .filter(|x| n(x, "p") == p && arm(x) == "w")
            .collect();
        let cc: Vec<_> = runs
            .iter()
            .copied()
            .filter(|x| n(x, "p") == p && arm(x) == "c")
            .collect();
        let vv: Vec<_> = runs
            .iter()
            .copied()
            .filter(|x| n(x, "p") == p && arm(x) == "v")
            .collect();
        let rho: Vec<_> = runs
            .iter()
            .copied()
            .filter(|x| n(x, "p") == p && arm(x) == "R")
            .collect();
        let pp: Vec<_> = runs
            .iter()
            .copied()
            .filter(|x| n(x, "p") == p && arm(x) == "P")
            .collect();
        let mut prices = Vec::new();
        for mode in [
            "Q1",
            "strictmul",
            "Q2",
            "Q3",
            "Q4",
            "lean_floor",
            "lean_actual",
        ] {
            if ((mode == "Q3" || mode == "Q4")
                && rho.iter().any(|x| x.get("compares_uncharged").is_none()))
                || (mode.starts_with("lean") && pp.is_empty())
            {
                continue;
            }
            for w in [0., 0.1, 1.] {
                let dd = if mode.starts_with("lean") { &pp } else { &rho };
                let a: f64 = ww.iter().map(|x| cost(x, w, mode)).sum();
                let d: f64 = dd.iter().map(|x| cost(x, w, mode)).sum();
                let losses = ww
                    .iter()
                    .filter(|x| {
                        let y = dd
                            .iter()
                            .find(|y| {
                                n(y, "curve_seed") == n(x, "curve_seed")
                                    && n(y, "target_seed") == n(x, "target_seed")
                                    && n(y, "seed") == n(x, "seed")
                            })
                            .unwrap();
                        cost(x, w, mode) > cost(y, w, mode)
                    })
                    .count();
                prices.push(json!({"mode":mode,"w":w,"ratio":a/d,"ic_mean":a/ww.len()as f64,"reference_mean":d/dd.len()as f64,"w_over_c":if cc.is_empty(){Value::Null}else{json!(a/cc.iter().map(|x|cost(x,w,mode)).sum::<f64>())},"paired_losses":losses,"paired_cluster_bootstrap95":boot(&ww,dd,w,mode,100000)}));
            }
        }
        let mut means = serde_json::Map::new();
        for k in [
            "group_ops",
            "setup_ops",
            "walk_ops",
            "verify_ops",
            "rel_mul",
            "rel_add",
            "la_mul",
            "la_add",
            "relations",
            "trials",
            "core_cols",
            "core_rows",
            "cantor_mul",
            "cantor_inv",
            "la_fallbacks",
            "attempts",
        ] {
            means.insert(k.into(), json!(sum(&ww, k) / ww.len() as f64));
        }
        let la_mul = if cc.is_empty() {
            Value::Null
        } else {
            json!(sum(&ww, "la_mul") / sum(&cc, "la_mul"))
        };
        let la_all = if cc.is_empty() {
            Value::Null
        } else {
            json!(
                (sum(&ww, "la_mul") + sum(&ww, "la_add"))
                    / (sum(&cc, "la_mul") + sum(&cc, "la_add"))
            )
        };
        out.push(json!({"p":p,"runs_w":ww.len(),"unique_curves":ww.iter().map(|x|n(x,"curve_seed")).collect::<BTreeSet<_>>().len(),"means_w":means,"la_w_over_c_mul":la_mul,"la_w_over_c_mul_add":la_all,"la_w_over_v_mul":if vv.is_empty(){Value::Null}else{json!(sum(&ww,"la_mul")/sum(&vv,"la_mul"))},"la_w_over_v_mul_add":if vv.is_empty(){Value::Null}else{json!((sum(&ww,"la_mul")+sum(&ww,"la_add"))/(sum(&vv,"la_mul")+sum(&vv,"la_add")))},"addition_break_even_vs_frozen_R_w1":(sum(&rho,"group_ops")+sum(&rho,"native")-sum(&ww,"group_ops")-sum(&ww,"rel_mul")-sum(&ww,"la_mul"))/(sum(&ww,"rel_add")+sum(&ww,"la_add")),"prices":prices}));
    }
    let failures = runs
        .iter()
        .filter(|x| x["ok"] != true || x["certified"] != true || x["exhausted"] == true)
        .count();
    println!("{}",serde_json::to_string_pretty(&json!({"source":path,"runs":runs.len(),"failed_rows":failures,"bootstrap":"100000 paired cluster draws; curve clusters for original, curve/target for followup; StdRng seed20261116; distinct from producer RNG","per_p":out})).unwrap());
}
fn replay_one(x: &Value, instrow: &Value, verify: bool) -> Value {
    let p = n(x, "p");
    let cs = n(x, "curve_seed");
    let seed = n(x, "seed");
    let ts = x["target_seed"].as_u64().unwrap_or(cs);
    let f: Vec<u64> = instrow["f"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap())
        .collect();
    assert_eq!(f, gl::random_f(p, 4, cs));
    let curve = gl::JCurve::new(p, 4, &f).unwrap();
    let inst = gl::instance_from(
        curve,
        n(instrow, "order"),
        n(instrow, "h"),
        n(instrow, "r"),
        ts,
    )
    .unwrap();
    let a = arm(x);
    let mut o;
    if ["c", "v", "w"].contains(&a) {
        let z = gl::run_ic_ext(
            &inst,
            gl::IcParams {
                arm: gl::Arm::DoubleLp,
                nb: gl::fb_size_2lp(p, 4, 2.),
                seed,
                max_trials: 50_000_000,
                restart: 4096,
                prefilter: true,
                verify_relations: verify,
                sge: None,
            },
            gl::IcExt {
                coord_square: a != "c",
                unit_t: a == "w",
            },
        );
        o = json!({"group_ops":z.group_ops(),"setup_ops":z.setup_ops,"walk_ops":z.walk_ops,"verify_ops":z.verify_ops,"rel_mul":z.rel.mul,"rel_add":z.rel.add,"la_mul":z.la.mul,"la_add":z.la.add,"cantor_mul":z.cantor_mul,"cantor_inv":z.cantor_inv,"trials":z.trials,"relations":z.relations,"core_cols":z.core_cols,"core_rows":z.core_rows,"fb_cols":z.fb_cols,"attempts":z.attempts,"la_fallbacks":z.la_fallbacks,"recovered":z.recovered,"certified":z.certified,"bad_relations":z.bad_relations,"exhausted":z.exhausted});
    } else {
        let mut w = if a == "P" || a == "Q" {
            RhoWalk::plain()
        } else {
            RhoWalk::negation()
        };
        if a == "Q" {
            w.cycle_window = 0;
        }
        let fm0 = inst.curve.fmul();
        let fi0 = inst.curve.finv();
        let z = rho_reference_walk(
            &inst.curve,
            inst.g,
            inst.target,
            inst.r,
            seed,
            (60. * (inst.r as f64).sqrt()) as u64 + 10000,
            w,
        );
        o = json!({"group_ops":z.group_ops.adds+z.group_ops.doubles,"steps":z.steps,"native":z.counters.get("canonicalisations_uncharged").copied().unwrap_or(0).max(z.steps),"cantor_mul":inst.curve.fmul()-fm0,"cantor_inv":inst.curve.finv()-fi0,"recovered":z.recovered,"certified":z.verified,"compares":z.work_native.values().sum::<u64>(),"rho_counters":z.counters});
    }
    let d = o["recovered"].as_u64().expect("scalar");
    assert_eq!(d, inst.t_secret);
    assert_eq!(inst.curve.mul_rtl(inst.g, d), inst.target);
    let mut mismatches = Vec::new();
    for (k, v) in o.as_object().unwrap() {
        if let Some(w) = x.get(k) {
            if v != w && !(verify && (k == "cantor_mul" || k == "cantor_inv")) {
                mismatches.push(json!({"field":k,"recorded":w,"replay":v}));
            }
        }
    }
    o["kind"] = json!("replay");
    o["arm"] = json!(a);
    o["p"] = json!(p);
    o["curve_seed"] = json!(cs);
    o["target_seed"] = json!(ts);
    o["seed"] = json!(seed);
    o["mismatches"] = json!(mismatches);
    o["relation_checks_enabled"] = json!(verify);
    o["G"] = json!([inst.curve.u_coeffs(&inst.g), inst.curve.v_coeffs(&inst.g)]);
    o["T"] = json!([
        inst.curve.u_coeffs(&inst.target),
        inst.curve.v_coeffs(&inst.target)
    ]);
    o["f"] = json!(f);
    o["r"] = json!(inst.r);
    o
}
fn replay(path: &str, arms: &str, threads: usize, verify: bool) {
    let rr = rows(path);
    let runs: Vec<_> = rr
        .iter()
        .filter(|x| x["kind"] == "run" && arms.contains(arm(x)))
        .collect();
    let instances: Vec<_> = rr.iter().filter(|x| x["kind"] == "instance").collect();
    let index = std::sync::atomic::AtomicUsize::new(0);
    let out = std::sync::Mutex::new(std::io::stdout());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let j = index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if j >= runs.len() {
                    break;
                }
                let x = runs[j];
                let i = instances
                    .iter()
                    .find(|i| n(i, "p") == n(x, "p") && n(i, "curve_seed") == n(x, "curve_seed"))
                    .unwrap();
                let o = replay_one(x, i, verify);
                writeln!(out.lock().unwrap(), "{o}").unwrap();
            });
        }
    });
}
fn calibrated(path: &str) {
    let rr = rows(path);
    let mut out = Vec::new();
    for scale in [0., 1., 10.] {
        let (mut ic, mut rho, mut count) = (0., 0., 0);
        for v in rr.iter().filter(|v| v["kind"] == "run" && arm(v) == "w") {
            let c = rr
                .iter()
                .find(|c| c["kind"] == "calibration" && n(c, "curve_seed") == n(v, "curve_seed"))
                .unwrap();
            let ns = |key: &str| c[key].as_f64().unwrap();
            let wp = ns("ns_mulmod_p") / ns("ns_cantor_add");
            let wr = ns("ns_mulmod_r") / ns("ns_cantor_add");
            let wa = ns("ns_addmod_p") / ns("ns_cantor_add");
            assert!((wp - ns("w_cal_mul_p")).abs() < 1e-14);
            assert!((wr - ns("w_cal_mul_r")).abs() < 1e-14);
            assert!((wa - ns("w_cal_add_p")).abs() < 1e-14);
            let r = rr
                .iter()
                .find(|r| {
                    r["kind"] == "run"
                        && arm(r) == "R"
                        && n(r, "curve_seed") == n(v, "curve_seed")
                        && n(r, "seed") == n(v, "seed")
                        && n(r, "target_seed") == n(v, "target_seed")
                })
                .unwrap();
            ic += n(v, "group_ops") as f64
                + scale
                    * (wp * n(v, "rel_mul") as f64
                        + wr * n(v, "la_mul") as f64
                        + wa * (n(v, "rel_add") + n(v, "la_add")) as f64);
            rho += n(r, "group_ops") as f64;
            count += 1;
        }
        out.push(json!({"scale_of_recorded_calibration":scale,"n":count,"ic_mean":ic/count as f64,"rho_mean":rho/count as f64,"ratio":ic/rho}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"source":path,"status":"recalculation of producer shared-host calibration, not a new local timing","limitations":"no mod-r addition calibration; rho native overhead untimed; one mod-r relation product priced at mod-p rate as in producer","readings":out})).unwrap());
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "recalc" => recalc(&a[2]),
        "certify" => independent::certify(&a[2]),
        "relations" => independent::relations(&a[2]),
        "transfer" => independent::transfer(&a[2]),
        "calibrated" => calibrated(&a[2]),
        "replay" => replay(
            &a[2],
            a.get(3).map(String::as_str).unwrap_or("cvwR"),
            a.get(4).map(|x| x.parse().unwrap()).unwrap_or(3),
            a.get(5).is_some_and(|s| s == "verify"),
        ),
        _ => panic!("recalc|certify|relations|transfer|replay PATH [ARMS THREADS verify]"),
    };
    if let Ok(s) = std::fs::read_to_string("/proc/self/status") {
        for l in s
            .lines()
            .filter(|l| l.starts_with("VmHWM:") || l.starts_with("VmPeak:"))
        {
            eprintln!("{l}");
        }
    }
}
