//! `genus_ladder_harvest`: the `genus_ladder` index-calculus arms with and
//! without harvesting the intermediate divisor of Cantor's first reduction
//! step (`crypto_lib::cryptanalysis::genus_ladder_harvest`), on the same
//! instances as `genus_ladder` (same `curve_seed`s, same targets), paired by
//! (instance, seed).
//!
//! ```text
//! genus_ladder_harvest run --primes 1031,4099 --genera 2,3,4 --curves 3 --seeds 3
//!     [--arms acR] [--cc 2.0] [--max-h 8] [--threads 4] [--rho-cap 3e7]
//!     [--seed-search 600] [--max-trials 5e7] [--no-harvest] --out FILE.jsonl
//! ```
//!
//! Arms: `A`/`a` full base (without/with the parity prefilter), `C`/`c`
//! double large prime, `R` rho.  Every index-calculus arm runs twice per
//! (instance, seed): as `genus_ladder` names it, and with `+harvest`.
//! `--no-harvest` writes only the first (the reproduction check of the
//! frozen instrument).  The JSONL layout is `genus_ladder`'s, with the
//! harvest counters added to the run records.  Protocol:
//! `research/genus_ladder_harvest_20261115/PROTOCOL.md`.  All figures are
//! `[pcc:unverified]`.

use std::io::Write;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::genus_ladder as gl;
use crypto_lib::cryptanalysis::genus_ladder::{Arm, IcParams};
use crypto_lib::cryptanalysis::genus_ladder_harvest as gh;
use serde_json::{json, Value};

struct Cfg {
    primes: Vec<u64>,
    genera: Vec<u32>,
    curves: usize,
    seeds: u64,
    arms: String,
    cc: f64,
    max_h: u64,
    threads: usize,
    rho_cap: f64,
    seed_search: u64,
    max_trials: u64,
    no_harvest: bool,
    out: String,
}

fn parse() -> Result<Cfg, String> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.first().map(String::as_str) != Some("run") {
        return Err(
            "usage: genus_ladder_harvest run --primes P,... --genera 2,3,4 --out F [options]"
                .into(),
        );
    }
    let mut c = Cfg {
        primes: vec![],
        genera: vec![2, 3, 4],
        curves: 3,
        seeds: 3,
        arms: "acR".into(),
        cc: 2.0,
        max_h: 8,
        threads: 4,
        rho_cap: 3.0e7,
        seed_search: 600,
        max_trials: 50_000_000,
        no_harvest: false,
        out: String::new(),
    };
    let mut i = 1;
    while i < a.len() {
        let flag = a[i].as_str();
        if flag == "--no-harvest" {
            c.no_harvest = true;
            i += 1;
            continue;
        }
        let v = a
            .get(i + 1)
            .cloned()
            .ok_or(format!("missing value for {flag}"))?;
        let num = |v: &str| v.parse::<f64>().map_err(|e| format!("{flag}: {e}"));
        let list = |v: &str| -> Result<Vec<u64>, String> {
            v.split(',')
                .map(|s| s.parse().map_err(|e| format!("{flag}: {e}")))
                .collect()
        };
        match flag {
            "--primes" => c.primes = list(&v)?,
            "--genera" => c.genera = list(&v)?.into_iter().map(|g| g as u32).collect(),
            "--curves" => c.curves = num(&v)? as usize,
            "--seeds" => c.seeds = num(&v)? as u64,
            "--arms" => c.arms = v,
            "--cc" => c.cc = num(&v)?,
            "--max-h" => c.max_h = num(&v)? as u64,
            "--threads" => c.threads = num(&v)? as usize,
            "--rho-cap" => c.rho_cap = num(&v)?,
            "--seed-search" => c.seed_search = num(&v)? as u64,
            "--max-trials" => c.max_trials = num(&v)? as u64,
            "--out" => c.out = v,
            _ => return Err(format!("unknown flag {flag}")),
        }
        i += 2;
    }
    if c.primes.is_empty() || c.out.is_empty() {
        return Err("--primes and --out are required".into());
    }
    Ok(c)
}

#[derive(Clone)]
struct Spec {
    p: u64,
    g: u32,
    curve_seed: u64,
    order: u64,
    h: u64,
    r: u64,
}

fn pool<T: Sync, F: Fn(&T) + Sync>(items: &[T], threads: usize, f: F) {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= items.len() {
                    break;
                }
                f(&items[i]);
            });
        }
    });
}

fn counts_json(sc: &gh::SourceCounts) -> Value {
    json!({"candidates": sc.candidates, "prefilter_rejects": sc.prefilter_rejects,
        "splits": sc.splits, "relations": sc.relations, "rels_by_lp": sc.rels_by_lp,
        "bad_relations": sc.bad_relations, "overflow": sc.overflow})
}

fn main() -> ExitCode {
    let cfg = match parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    let file = std::fs::File::create(&cfg.out).expect("create --out");
    let out = Mutex::new(std::io::BufWriter::new(file));
    let emit = |v: Value| {
        let mut w = out.lock().unwrap();
        writeln!(w, "{v}").unwrap();
        w.flush().unwrap();
    };
    let t0 = Instant::now();
    emit(
        json!({"kind": "config", "tool": "genus_ladder_harvest", "primes": cfg.primes,
        "genera": cfg.genera, "curves": cfg.curves, "seeds": cfg.seeds, "arms": cfg.arms,
        "cc": cfg.cc, "max_h": cfg.max_h, "rho_cap": cfg.rho_cap, "seed_search": cfg.seed_search,
        "max_trials": cfg.max_trials, "threads": cfg.threads, "restart": gl::RESTART,
        "harvest": !cfg.no_harvest}),
    );

    let mut cells: Vec<(u64, u32)> = Vec::new();
    for &p in &cfg.primes {
        for &g in &cfg.genera {
            cells.push((p, g));
        }
    }
    let specs: Mutex<Vec<Spec>> = Mutex::new(Vec::new());
    pool(&cells, cfg.threads, |&(p, g)| {
        let mut admitted = 0usize;
        let mut tried = 0u64;
        let mut rejects = 0u64;
        let ts = Instant::now();
        for cs in 1..=cfg.seed_search {
            if admitted >= cfg.curves {
                break;
            }
            tried += 1;
            match gl::make_instance(p, g, cs, cs, cfg.max_h) {
                Ok(inst) => {
                    admitted += 1;
                    emit(
                        json!({"kind": "instance", "family": "hyperelliptic", "p": p, "g": g,
                        "curve_seed": cs, "f": inst.curve.f_coeffs(), "order": inst.order,
                        "h": inst.h, "r": inst.r}),
                    );
                    specs.lock().unwrap().push(Spec {
                        p,
                        g,
                        curve_seed: cs,
                        order: inst.order,
                        h: inst.h,
                        r: inst.r,
                    });
                }
                Err(_) => rejects += 1,
            }
        }
        emit(
            json!({"kind": "cell", "family": "hyperelliptic", "p": p, "g": g, "tried": tried,
            "admitted": admitted, "rejected": rejects, "search_wall_s": ts.elapsed().as_secs_f64()}),
        );
    });

    let mut tasks: Vec<(Spec, u64)> = Vec::new();
    let mut sp = specs.into_inner().unwrap();
    sp.sort_by(|a, b| {
        let ka = (a.r as f64).sqrt() * f64::from(a.g * a.g);
        let kb = (b.r as f64).sqrt() * f64::from(b.g * b.g);
        ka.partial_cmp(&kb)
            .unwrap()
            .then(a.curve_seed.cmp(&b.curve_seed))
    });
    for s in &sp {
        for seed in 1..=cfg.seeds {
            tasks.push((s.clone(), seed));
        }
    }
    pool(&tasks, cfg.threads, |(s, seed)| {
        let curve =
            gl::JCurve::new(s.p, s.g, &gl::random_f(s.p, s.g, s.curve_seed)).expect("admitted");
        let inst = gl::instance_from(curve, s.order, s.h, s.r, s.curve_seed).expect("admitted");
        let head = json!({"family": "hyperelliptic", "p": s.p, "g": s.g,
            "curve_seed": s.curve_seed, "seed": seed, "r": s.r, "h": s.h});
        for ch in cfg.arms.chars() {
            let (arm, pre) = match ch {
                'A' => (Arm::Full, false),
                'a' => (Arm::Full, true),
                'C' => (Arm::DoubleLp, false),
                'c' => (Arm::DoubleLp, true),
                'R' => {
                    let expect = (std::f64::consts::PI * s.r as f64 / 4.0).sqrt();
                    let mut rec = head.clone();
                    rec["arm"] = json!("R_rho_neg");
                    if expect > cfg.rho_cap {
                        rec["kind"] = json!("skip");
                        rec["reason"] = json!("rho_cap");
                        rec["expected_steps"] = json!(expect);
                    } else {
                        let o = gl::run_rho(&inst, *seed);
                        rec["kind"] = json!("run");
                        rec["certified"] = json!(o.certified);
                        rec["ok"] = json!(o.recovered == Some(inst.t_secret));
                        rec["group_ops"] = json!(o.group_ops());
                        rec["walk_ops"] = json!(o.walk_ops);
                        rec["cantor_mul"] = json!(o.cantor_mul);
                        rec["cantor_inv"] = json!(o.cantor_inv);
                        rec["native"] = json!(o.native);
                        rec["steps"] = json!(o.steps);
                        rec["wall_ns"] = json!(o.wall_ns);
                    }
                    emit(rec);
                    continue;
                }
                _ => continue,
            };
            let modes: &[bool] = if cfg.no_harvest {
                &[false]
            } else {
                &[false, true]
            };
            for &harvest in modes {
                let hr = gh::run_ic_harvest(
                    &inst,
                    IcParams {
                        arm,
                        nb: gl::fb_size_2lp(s.p, s.g, cfg.cc),
                        seed: *seed,
                        max_trials: cfg.max_trials,
                        restart: gl::RESTART,
                        prefilter: pre,
                        verify_relations: false,
                        sge: None,
                    },
                    harvest,
                );
                let o = &hr.ic;
                let mut rec = head.clone();
                rec["kind"] = json!("run");
                rec["arm"] = json!(format!(
                    "{}{}{}",
                    o.arm,
                    if pre { "+pre" } else { "" },
                    if harvest { "+harvest" } else { "" }
                ));
                rec["certified"] = json!(o.certified);
                rec["exhausted"] = json!(o.exhausted);
                rec["ok"] = json!(o.recovered == Some(inst.t_secret));
                rec["group_ops"] = json!(o.group_ops());
                rec["setup_ops"] = json!(o.setup_ops);
                rec["walk_ops"] = json!(o.walk_ops);
                rec["verify_ops"] = json!(o.verify_ops);
                rec["cantor_mul"] = json!(o.cantor_mul);
                rec["cantor_inv"] = json!(o.cantor_inv);
                rec["rel_mul"] = json!(o.rel.mul);
                rec["rel_add"] = json!(o.rel.add);
                rec["la_mul"] = json!(o.la.mul);
                rec["la_add"] = json!(o.la.add);
                rec["la_wied_mul"] = json!(o.la_wied.mul);
                rec["fb_cols"] = json!(o.fb_cols);
                rec["trials"] = json!(o.trials);
                rec["candidates"] = json!(o.candidates);
                rec["prefilter_rejects"] = json!(o.prefilter_rejects);
                rec["splits"] = json!(o.splits);
                rec["relations"] = json!(o.relations);
                rec["rels_by_lp"] = json!(o.rels_by_lp);
                rec["core_rows"] = json!(o.core_rows);
                rec["core_cols"] = json!(o.core_cols);
                rec["attempts"] = json!(o.attempts);
                rec["harvest"] = json!(harvest);
                rec["intermediates"] = json!(hr.intermediates);
                rec["harvest_candidates"] = json!(hr.harvested.candidates);
                rec["harvest_prefilter_rejects"] = json!(hr.harvested.prefilter_rejects);
                rec["harvest_splits"] = json!(hr.harvested.splits);
                rec["harvest_relations"] = json!(hr.harvested.relations);
                rec["harvest_overflow"] = json!(hr.harvested.overflow);
                rec["main"] = counts_json(&hr.main);
                rec["harvested"] = counts_json(&hr.harvested);
                rec["cycles"] = json!(o.cycles);
                rec["wall_ns"] = json!(o.wall_ns);
                emit(rec);
            }
        }
    });
    emit(json!({"kind": "done", "wall_s": t0.elapsed().as_secs_f64()}));
    ExitCode::SUCCESS
}
