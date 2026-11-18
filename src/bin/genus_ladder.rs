//! `genus_ladder`: the discrete logarithm on Jacobians of genus 2, 3, 4, 5
//! over one prime field, by index calculus (full base; double large prime)
//! and by negation-map rho, exact counts, every logarithm certified.
//! Optional arms: Smith's isogeny conditions on the genus-3 curves, and
//! smooth plane quartics (genus 3, not hyperelliptic) with Diem's relations.
//!
//! ```text
//! genus_ladder run --primes 131,257 --genera 2,3,4,5 --curves 4 --seeds 3
//!     [--arms AaCcR] [--cc 2.0] [--max-h 64] [--threads 4] [--rho-cap 3e7]
//!     [--seed-search 600] [--max-trials 5e7] [--smith] [--smith-census N]
//!     [--quartic] [--qc 3.0] [--restart-sqrt] [--sge W,F]... --out FILE.jsonl
//! ```
//!
//! Arms: `A` full base, `a` full base with the parity prefilter, `C` double
//! large prime, `c` double large prime with the prefilter, `R` rho.
//! `--sge W,F` (repeatable; default off) runs every index-calculus arm a
//! second time on the same instance and seed with structured Gaussian
//! elimination before Wiedemann (`w_max = W`, `fill_max = F`), named
//! `<arm>+sge(W,F)`; the arm without it runs and is recorded unchanged.
//! Protocol of the merge: `research/genus_ladder_sge_20261115/PROTOCOL.md`.
//! Arm `s` is arm `w` with the merge: `--sge W,F` (default 4,1.5) reduces the
//! peeled core, then the coordinate Wiedemann solves the reduced matrix; the
//! arm is named `...+sgecoord+sge(W,F)` and only runs merged.
//! One JSON object per line.  Protocol:
//! `research/genus_ladder_20261114/PROTOCOL.md`.  All figures are
//! `[pcc:unverified]`.

use std::io::Write;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::genus_ladder as gl;
use crypto_lib::cryptanalysis::genus_ladder::{Arm, IcParams};
use crypto_lib::cryptanalysis::ic_boundary::{RhoWalk, WalkPolicy};
use crypto_lib::cryptanalysis::quartic_ladder as ql;
use crypto_lib::cryptanalysis::smith_prop24 as smith;
use serde_json::{json, Value};
use std::hint::black_box;

struct Cfg {
    primes: Vec<u64>,
    genera: Vec<u32>,
    curves: usize,
    seeds: u64,
    arms: String,
    cc: f64,
    qc: f64,
    max_h: u64,
    threads: usize,
    rho_cap: f64,
    seed_search: u64,
    /// `--curve-seeds p=s1/s2/...,p=...`: evaluate only these curve seeds in
    /// the hyperelliptic cells of `p` (to re-run admitted instances of an
    /// earlier study without its seed search).  Same admission path.
    curve_seeds: std::collections::HashMap<u64, Vec<u64>>,
    /// Targets per admitted curve (`--targets`, default 1: the frozen grid).
    targets: u64,
    /// Emit a timing calibration per instance (`--calibrate`).
    calibrate: bool,
    max_trials: u64,
    restart_sqrt: bool,
    smith: bool,
    smith_census: u64,
    quartic: bool,
    sge: Vec<gl::SgeParams>,
    out: String,
}

fn parse() -> Result<Cfg, String> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.first().map(String::as_str) != Some("run") {
        return Err(
            "usage: genus_ladder run --primes P,... --genera 2,3,4,5 --out F [options]".into(),
        );
    }
    let mut c = Cfg {
        primes: vec![],
        genera: vec![2, 3, 4, 5],
        curves: 2,
        seeds: 2,
        arms: "AaCcR".into(),
        cc: 2.0,
        qc: 3.0,
        max_h: 64,
        threads: 4,
        rho_cap: 3.0e7,
        seed_search: 600,
        curve_seeds: Default::default(),
        targets: 1,
        calibrate: false,
        max_trials: 50_000_000,
        restart_sqrt: false,
        smith: false,
        smith_census: 0,
        quartic: false,
        sge: Vec::new(),
        out: String::new(),
    };
    let mut i = 1;
    while i < a.len() {
        let flag = a[i].as_str();
        if flag == "--smith" {
            c.smith = true;
            i += 1;
            continue;
        }
        if flag == "--restart-sqrt" {
            c.restart_sqrt = true;
            i += 1;
            continue;
        }
        if flag == "--quartic" {
            c.quartic = true;
            i += 1;
            continue;
        }
        if flag == "--calibrate" {
            c.calibrate = true;
            i += 1;
            continue;
        }
        let v = a
            .get(i + 1)
            .cloned()
            .ok_or(format!("missing value for {flag}"))?;
        let num = |v: &str| v.parse::<f64>().map_err(|e| format!("{flag}: {e}"));
        match flag {
            "--primes" => {
                c.primes = v
                    .split(',')
                    .map(|s| s.parse().map_err(|e| format!("{e}")))
                    .collect::<Result<_, _>>()?
            }
            "--genera" => {
                c.genera = v
                    .split(',')
                    .map(|s| s.parse().map_err(|e| format!("{e}")))
                    .collect::<Result<_, _>>()?
            }
            "--curves" => c.curves = num(&v)? as usize,
            "--targets" => c.targets = num(&v)? as u64,
            "--seeds" => c.seeds = num(&v)? as u64,
            "--arms" => c.arms = v,
            "--cc" => c.cc = num(&v)?,
            "--qc" => c.qc = num(&v)?,
            "--max-h" => c.max_h = num(&v)? as u64,
            "--threads" => c.threads = num(&v)? as usize,
            "--rho-cap" => c.rho_cap = num(&v)?,
            "--seed-search" => c.seed_search = num(&v)? as u64,
            "--curve-seeds" => {
                for cell in v.split(',') {
                    let (p, ss) = cell
                        .split_once('=')
                        .ok_or(format!("--curve-seeds wants p=s1/s2,..., not {cell}"))?;
                    let p: u64 = p.parse().map_err(|e| format!("--curve-seeds p: {e}"))?;
                    let ss = ss
                        .split('/')
                        .map(|x| x.parse::<u64>().map_err(|e| format!("--curve-seeds: {e}")))
                        .collect::<Result<Vec<_>, _>>()?;
                    c.curve_seeds.insert(p, ss);
                }
            }
            "--max-trials" => c.max_trials = num(&v)? as u64,
            "--smith-census" => c.smith_census = num(&v)? as u64,
            "--sge" => {
                let (w, fm) = v
                    .split_once(',')
                    .ok_or(format!("--sge wants W,F, not {v}"))?;
                let w_max = w.parse::<usize>().map_err(|e| format!("--sge W: {e}"))?;
                let fill_max = num(fm)?;
                if w_max < 1 || fill_max < 1.0 || !fill_max.is_finite() {
                    return Err(format!("--sge wants W >= 1 and F >= 1, not {v}"));
                }
                c.sge.push(gl::SgeParams { w_max, fill_max });
            }
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

/// Plain data for one admitted instance (rebuilt inside each worker).
#[derive(Clone)]
enum Spec {
    Hyper {
        p: u64,
        g: u32,
        curve_seed: u64,
        /// The target's seed: the curve seed for the first target (the
        /// frozen instrument), then `curve_seed + k · 1_000_003` (`--targets`).
        target_seed: u64,
        order: u64,
        h: u64,
        r: u64,
    },
    Quartic {
        q: u32,
        curve_seed: u64,
    },
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
    let mut config = json!({"kind": "config", "primes": cfg.primes, "genera": cfg.genera, "curves": cfg.curves,
        "seeds": cfg.seeds, "arms": cfg.arms, "cc": cfg.cc, "qc": cfg.qc, "max_h": cfg.max_h,
        "rho_cap": cfg.rho_cap, "seed_search": cfg.seed_search, "max_trials": cfg.max_trials,
        "smith": cfg.smith, "smith_census": cfg.smith_census, "quartic": cfg.quartic,
        "threads": cfg.threads,
        "restart": if cfg.restart_sqrt { json!("min(4096, max(16, sqrt(r)/2))") } else { json!(gl::RESTART) }});
    if !cfg.sge.is_empty() {
        // Only with the flag, so a run without it writes the frozen layout.
        config["sge"] = json!(cfg
            .sge
            .iter()
            .map(|s| json!({"w_max": s.w_max, "fill_max": s.fill_max}))
            .collect::<Vec<_>>());
    }
    emit(config);

    // ── instance search, one cell per (p, g) and one per quartic prime ──
    let mut cells: Vec<(u64, u32)> = Vec::new();
    for &p in &cfg.primes {
        for &g in &cfg.genera {
            cells.push((p, g));
        }
        if cfg.quartic {
            cells.push((p, 0)); // genus tag 0: the plane-quartic cell
        }
    }
    let specs: Mutex<Vec<Spec>> = Mutex::new(Vec::new());
    pool(&cells, cfg.threads, |&(p, g)| {
        let mut admitted = 0usize;
        let mut tried = 0u64;
        let mut reasons: std::collections::BTreeMap<String, u64> = Default::default();
        let ts = Instant::now();
        let listed = if g == 0 {
            None
        } else {
            cfg.curve_seeds.get(&p)
        };
        let seeds: Vec<u64> = match listed {
            Some(v) => v.clone(),
            None => (1..=cfg.seed_search).collect(),
        };
        for cs in seeds {
            if admitted >= cfg.curves {
                break;
            }
            tried += 1;
            if g == 0 {
                match ql::make_instance(p as u32, cs, cs, cfg.max_h) {
                    Ok(inst) => {
                        admitted += 1;
                        emit(
                            json!({"kind": "instance", "family": "quartic", "p": p, "g": 3,
                            "curve_seed": cs, "f": inst.f.to_vec(), "order": inst.order,
                            "h": inst.h, "r": inst.r, "n_points": inst.n_points}),
                        );
                        specs.lock().unwrap().push(Spec::Quartic {
                            q: p as u32,
                            curve_seed: cs,
                        });
                    }
                    Err(e) => *reasons.entry(reason_class(&e)).or_default() += 1,
                }
                continue;
            }
            match gl::make_instance(p, g, cs, cs, cfg.max_h) {
                Ok(inst) => {
                    admitted += 1;
                    let mut rec = json!({"kind": "instance", "family": "hyperelliptic", "p": p, "g": g,
                        "curve_seed": cs, "f": inst.curve.f_coeffs(), "order": inst.order,
                        "h": inst.h, "r": inst.r});
                    if cfg.smith && g == 3 {
                        rec["smith"] = smith_record(&inst.curve.f_coeffs(), p, cs);
                    }
                    emit(rec);
                    for k in 0..cfg.targets.max(1) {
                        specs.lock().unwrap().push(Spec::Hyper {
                            p,
                            g,
                            curve_seed: cs,
                            target_seed: cs + k * 1_000_003,
                            order: inst.order,
                            h: inst.h,
                            r: inst.r,
                        });
                    }
                }
                Err(e) => *reasons.entry(reason_class(&e)).or_default() += 1,
            }
        }
        emit(
            json!({"kind": "cell", "family": if g == 0 { "quartic" } else { "hyperelliptic" },
            "p": p, "g": if g == 0 { 3 } else { g }, "tried": tried, "admitted": admitted,
            "rejects": reasons, "search_wall_s": ts.elapsed().as_secs_f64()}),
        );
    });

    // ── Smith census: random genus-3 curves with no admission filter ──
    if cfg.smith_census > 0 {
        for &p in &cfg.primes {
            let (mut n, mut ok, mut tract) = (0u64, 0u64, 0u64);
            let mut cs = 1_000_000u64;
            while n < cfg.smith_census {
                cs += 1;
                let f = gl::random_f(p, 3, cs);
                if gl::JCurve::new(p, 3, &f).is_err() {
                    continue;
                }
                let rec = smith_record(&f, p, cs);
                n += 1;
                ok += u64::from(rec["full_success"].as_bool().unwrap_or(false));
                tract += u64::from(rec["n_tractable"].as_u64().unwrap_or(0) > 0);
            }
            emit(
                json!({"kind": "smith_census", "p": p, "curves": n, "with_tractable_subgroup": tract,
                "full_success": ok}),
            );
        }
    }

    // ── runs ──
    let mut tasks: Vec<(Spec, u64)> = Vec::new();
    let mut sp = specs.into_inner().unwrap();
    // Cheap cells first, so a truncated session still holds whole cells.
    sp.sort_by(|a, b| spec_cost(a).partial_cmp(&spec_cost(b)).unwrap());
    for s in &sp {
        for seed in 1..=cfg.seeds {
            tasks.push((s.clone(), seed));
        }
    }
    pool(&tasks, cfg.threads, |(spec, seed)| match spec {
        Spec::Hyper {
            p,
            g,
            curve_seed,
            target_seed,
            order,
            h,
            r,
        } => {
            let curve = gl::JCurve::new(*p, *g, &gl::random_f(*p, *g, *curve_seed))
                .expect("admitted curve");
            let inst =
                gl::instance_from(curve, *order, *h, *r, *target_seed).expect("admitted instance");
            let head = json!({"family": "hyperelliptic", "p": p, "g": g, "curve_seed": curve_seed,
                "target_seed": target_seed, "seed": seed, "r": r, "h": h});
            if cfg.calibrate && *target_seed == *curve_seed && *seed == 1 {
                emit(calibrate(&inst, *p, *g, *curve_seed));
            }
            for ch in cfg.arms.chars() {
                // v: c + coordinate Wiedemann on a square row subset;
                // w: v + the capped unit-T walk (gl::IcExt).
                let no_ext = gl::IcExt::default();
                let (arm, pre, ext) = match ch {
                    'A' => (Arm::Full, false, no_ext),
                    'a' => (Arm::Full, true, no_ext),
                    'C' => (Arm::DoubleLp, false, no_ext),
                    'c' => (Arm::DoubleLp, true, no_ext),
                    'v' => (
                        Arm::DoubleLp,
                        true,
                        gl::IcExt {
                            coord_square: true,
                            unit_t: false,
                            ferrari: false,
                            sge_coord: false,
                        },
                    ),
                    'f' => (
                        Arm::DoubleLp,
                        true,
                        gl::IcExt {
                            coord_square: true,
                            unit_t: true,
                            ferrari: true,
                            sge_coord: false,
                        },
                    ),
                    // s: w + structured Gaussian elimination, then the
                    // coordinate solve on the reduced matrix (needs --sge;
                    // without it the single setting 4,1.5 is used).
                    'x' => (
                        // arm w + Ferrari quartic test + SGE before coordinate Wiedemann
                        Arm::DoubleLp,
                        true,
                        gl::IcExt {
                            coord_square: true,
                            unit_t: true,
                            ferrari: true,
                            sge_coord: true,
                        },
                    ),
                    's' => (
                        Arm::DoubleLp,
                        true,
                        gl::IcExt {
                            coord_square: true,
                            unit_t: true,
                            ferrari: false,
                            sge_coord: true,
                        },
                    ),
                    'w' => (
                        Arm::DoubleLp,
                        true,
                        gl::IcExt {
                            coord_square: true,
                            unit_t: true,
                            ferrari: false,
                            sge_coord: false,
                        },
                    ),
                    'z' => {
                        // Sieved relation search (genus_ladder_sieve), two large
                        // primes. Native work is reported in the same fields as
                        // the IC arms: rel_* includes the individual-log phase.
                        use crypto_lib::cryptanalysis::genus_ladder_sieve as gs;
                        let nb = gl::fb_size_2lp(*p, *g, cfg.cc);
                        let x = gs::run_sieve_ic(&inst, gs::SieveParams::new(nb, *seed, 2));
                        let mut rec = head.clone();
                        rec["kind"] = json!("run");
                        rec["arm"] = json!("Z_sieve_lp2");
                        rec["certified"] = json!(x.certified);
                        rec["exhausted"] = json!(x.exhausted);
                        rec["ok"] = json!(x.recovered == Some(inst.t_secret));
                        rec["group_ops"] = json!(x.group_ops());
                        rec["ind_group_ops"] = json!(x.ind_group_ops);
                        rec["verify_ops"] = json!(x.verify_ops);
                        rec["cantor_mul"] = json!(x.cantor_mul);
                        rec["cantor_inv"] = json!(x.cantor_inv);
                        rec["rel_mul"] = json!(x.rel.mul + x.ind.mul);
                        rec["rel_add"] = json!(x.rel.add + x.ind.add);
                        rec["ind_mul"] = json!(x.ind.mul);
                        rec["ind_add"] = json!(x.ind.add);
                        rec["la_mul"] = json!(x.la.mul);
                        rec["la_add"] = json!(x.la.add);
                        rec["la_wied_mul"] = json!(x.la_wied.mul);
                        rec["la_fallbacks"] = json!(x.la_fallbacks);
                        rec["fb_cols"] = json!(x.fb_cols);
                        rec["lp_max_eff"] = json!(x.lp_max_eff);
                        rec["families"] = json!(x.families);
                        rec["relations"] = json!(x.relations);
                        rec["rels_by_lp"] = json!(x.rels_by_lp);
                        rec["sieve_hits"] = json!(x.sieve_hits);
                        rec["ind_rows"] = json!(x.ind_rows);
                        rec["ind_families"] = json!(x.ind_families);
                        rec["core_rows"] = json!(x.core_rows);
                        rec["core_cols"] = json!(x.core_cols);
                        rec["attempts"] = json!(x.attempts);
                        rec["wall_ns"] = json!(x.wall_ns);
                        emit(rec);
                        continue;
                    }
                    'R' | 'P' | 'B' | 'Q' => {
                        // R: negation-map rho; P: plain rho (walk on points).
                        // B: negation rho, no per-step window, Brent cycle
                        // detection (WalkPolicy::v2). Q: plain rho, no window.
                        let plain = ch == 'P' || ch == 'Q';
                        let expect = (std::f64::consts::PI * *r as f64
                            / if plain { 2.0 } else { 4.0 })
                        .sqrt();
                        let mut rec = head.clone();
                        rec["arm"] = json!(match ch {
                            'P' => "R_rho_plain",
                            'Q' => "R_rho_plain_nowin",
                            'B' => "R_rho_neg_brent",
                            _ => "R_rho_neg",
                        });
                        if expect > cfg.rho_cap {
                            rec["kind"] = json!("skip");
                            rec["reason"] = json!("rho_cap");
                            rec["expected_steps"] = json!(expect);
                        } else {
                            let mut walk = if plain {
                                RhoWalk::plain()
                            } else {
                                RhoWalk::negation()
                            };
                            let policy = if ch == 'B' {
                                WalkPolicy::v2()
                            } else {
                                WalkPolicy::legacy()
                            };
                            if ch == 'B' || ch == 'Q' {
                                walk.cycle_window = 0;
                            }
                            let o = gl::run_rho_policy(&inst, *seed, walk, policy);
                            rec["kind"] = json!("run");
                            rec["certified"] = json!(o.certified);
                            rec["ok"] = json!(o.recovered == Some(inst.t_secret));
                            rec["group_ops"] = json!(o.group_ops());
                            rec["walk_ops"] = json!(o.walk_ops);
                            rec["cantor_mul"] = json!(o.cantor_mul);
                            rec["cantor_inv"] = json!(o.cantor_inv);
                            rec["native"] = json!(o.native);
                            rec["steps"] = json!(o.steps);
                            // Per-step work the walk does not emit, in closed
                            // form: one partition hash and two coefficient
                            // additions modulo r per step (symmetric pricing).
                            rec["compares_uncharged"] = json!(o.compares);
                            rec["hash_uncharged"] = json!(o.steps);
                            rec["coef_adds_uncharged"] = json!(2 * o.steps);
                            rec["wall_ns"] = json!(o.wall_ns);
                        }
                        emit(rec);
                        continue;
                    }
                    _ => continue,
                };
                let settings: Vec<Option<gl::SgeParams>> = if ext.sge_coord {
                    // arm s has no unmerged run: the merge is the arm
                    if cfg.sge.is_empty() {
                        vec![Some(gl::SgeParams {
                            w_max: 4,
                            fill_max: 1.5,
                        })]
                    } else {
                        cfg.sge.iter().copied().map(Some).collect()
                    }
                } else {
                    std::iter::once(None)
                        .chain(cfg.sge.iter().copied().map(Some))
                        .collect()
                };
                for sge in settings {
                    let o = gl::run_ic_ext(
                        &inst,
                        IcParams {
                            arm,
                            nb: gl::fb_size_2lp(*p, *g, cfg.cc),
                            seed: *seed,
                            max_trials: cfg.max_trials,
                            restart: if cfg.restart_sqrt {
                                gl::restart_sqrt(*r)
                            } else {
                                gl::RESTART
                            },
                            prefilter: pre,
                            verify_relations: false,
                            sge,
                        },
                        ext,
                    );
                    let suffix = match sge {
                        None => String::new(),
                        Some(s) => format!("+sge({},{})", s.w_max, s.fill_max),
                    };
                    let mut rec = head.clone();
                    rec["kind"] = json!("run");
                    rec["arm"] = json!(format!(
                        "{}{}{}{}{}{}{}",
                        o.arm,
                        if pre { "+pre" } else { "" },
                        if ext.coord_square || ext.sge_coord {
                            "+coord"
                        } else {
                            ""
                        },
                        if ext.unit_t { "+unitT" } else { "" },
                        if ext.ferrari { "+ferrari" } else { "" },
                        if ext.sge_coord { "+sgecoord" } else { "" },
                        suffix
                    ));
                    rec["la_fallbacks"] = json!(o.la_fallbacks);
                    rec["ferrari_undecided"] = json!(o.ferrari_undecided);
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
                    if let Some(sp) = sge {
                        // Merge-stage fields, only on merged runs.
                        let st = o.sge;
                        rec["w_max"] = json!(sp.w_max);
                        rec["fill_max"] = json!(sp.fill_max);
                        rec["la_sge_mul"] = json!(o.la_sge.mul);
                        rec["la_sge_add"] = json!(o.la_sge.add);
                        rec["la_wied_add"] = json!(o.la_wied.add);
                        rec["sge_rows_in"] = json!(st.rows_in);
                        rec["sge_cols_in"] = json!(st.cols_in);
                        rec["sge_nnz_in"] = json!(st.nnz_in);
                        rec["sge_generic_in"] = json!(st.generic_in);
                        rec["sge_rows_out"] = json!(st.rows_out);
                        rec["sge_cols_out"] = json!(st.cols_out);
                        rec["sge_nnz_out"] = json!(st.nnz_out);
                        rec["sge_generic_out"] = json!(st.generic_out);
                        rec["sge_merged"] = json!(st.merged);
                        rec["sge_peeled"] = json!(st.peeled);
                        rec["sge_empty"] = json!(st.empty);
                        rec["sge_duplicates"] = json!(st.duplicates);
                        rec["sge_refused"] = json!(st.refused);
                    }
                    rec["wall_ns"] = json!(o.wall_ns);
                    emit(rec);
                }
            }
        }
        Spec::Quartic { q, curve_seed } => {
            let inst = ql::make_instance(*q, *curve_seed, *curve_seed, cfg.max_h)
                .expect("admitted quartic");
            let head = json!({"family": "quartic", "p": q, "g": 3, "curve_seed": curve_seed,
                "seed": seed, "r": inst.r, "h": inst.h});
            for (arm, nb) in [
                (ql::QArm::Full, 0usize),
                (
                    ql::QArm::DoubleLp,
                    (cfg.qc * f64::from(*q).sqrt()).ceil() as usize,
                ),
            ] {
                let o = ql::run_diem(
                    &inst,
                    ql::QIcParams {
                        arm,
                        nb,
                        seed: *seed,
                        max_lines: cfg.max_trials,
                        verify_relations: false,
                    },
                );
                let mut rec = head.clone();
                rec["kind"] = json!("run");
                rec["arm"] = json!(format!("Q_{}", o.arm));
                rec["certified"] = json!(o.certified);
                rec["exhausted"] = json!(o.exhausted);
                rec["ok"] = json!(o.recovered == Some(inst.t_secret));
                rec["group_ops"] = json!(o.group_ops);
                rec["cantor_mul"] = json!(o.pic_mul);
                rec["cantor_inv"] = json!(o.pic_inv);
                rec["rel_mul"] = json!(o.rel.mul);
                rec["rel_add"] = json!(o.rel.add);
                rec["la_mul"] = json!(o.la.mul);
                rec["la_add"] = json!(o.la.add);
                rec["fb_cols"] = json!(o.fb_cols);
                rec["trials"] = json!(o.lines_tried);
                rec["relations"] = json!(o.relations);
                rec["rels_by_lp"] = json!(o.rels_by_lp);
                rec["decomp_trials"] = json!(o.decomp_trials);
                rec["core_rows"] = json!(o.core_rows);
                rec["core_cols"] = json!(o.core_cols);
                rec["attempts"] = json!(o.attempts);
                rec["wall_ns"] = json!(o.wall_ns);
                emit(rec);
            }
            let expect = (std::f64::consts::PI * inst.r as f64 / 4.0).sqrt();
            let mut rec = head.clone();
            rec["arm"] = json!("Q_rho_neg");
            // A Pic addition is some hundreds of times a genus-3 Cantor step.
            if expect > cfg.rho_cap / 100.0 {
                rec["kind"] = json!("skip");
                rec["reason"] = json!("rho_cap");
                rec["expected_steps"] = json!(expect);
            } else {
                let o = ql::run_rho(&inst, *seed);
                rec["kind"] = json!("run");
                rec["certified"] = json!(o.certified);
                rec["ok"] = json!(o.recovered == Some(inst.t_secret));
                rec["group_ops"] = json!(o.group_ops);
                rec["cantor_mul"] = json!(o.pic_mul);
                rec["cantor_inv"] = json!(o.pic_inv);
                rec["steps"] = json!(o.steps);
                rec["wall_ns"] = json!(o.wall_ns);
            }
            emit(rec);
        }
    });
    emit(json!({"kind": "done", "wall_s": t0.elapsed().as_secs_f64()}));
    ExitCode::SUCCESS
}

fn spec_cost(s: &Spec) -> f64 {
    match s {
        Spec::Hyper { r, g, .. } => (*r as f64).sqrt() * f64::from(*g * *g),
        Spec::Quartic { q, .. } => f64::from(*q).powf(1.5) * 300.0,
    }
}

fn reason_class(e: &str) -> String {
    let e = e.to_ascii_lowercase();
    for (pat, name) in [
        ("squarefree", "not_squarefree"),
        ("not h*r", "order_not_near_prime"),
        ("no h*r", "order_not_near_prime"),
        ("ambiguous", "order_ambiguous"),
        ("no candidate", "order_no_candidate"),
        ("too wide", "order_window_too_wide"),
        ("generator", "no_generator"),
    ] {
        if e.contains(pat) {
            return name.into();
        }
    }
    "other".into()
}

/// Smith's conditions (Propositions 2 and 4) on `y² = f(x)`, `deg f = 7`.
fn smith_record(f: &[u64], p: u64, seed: u64) -> Value {
    let mut rng = smith::SplitMix64(seed ^ p.rotate_left(17));
    match smith::analyze_curve(f, p, &mut rng) {
        None => json!({"analysed": false}),
        Some(o) => json!({
            "analysed": true,
            "cycle_type": o.cycle_type,
            "n_tractable": o.n_tractable(),
            "n_trig": o.n_trig(),
            "full_success": o.full_success(false),
            "full_success_twist": o.full_success(true),
        }),
    }
}

/// Informational timing (never part of a verdict): ns per Cantor addition,
/// per multiplication and addition modulo p, and per multiplication modulo r
/// (u128 reduction, as the linear algebra uses for r >= 2^50), best of 5.
fn calibrate(inst: &gl::Instance, p: u64, g: u32, curve_seed: u64) -> serde_json::Value {
    use std::time::Instant;
    let best = |f: &mut dyn FnMut() -> u64, n: u64| -> f64 {
        let mut b = f64::MAX;
        for _ in 0..5 {
            let t = Instant::now();
            black_box(f());
            b = b.min(t.elapsed().as_nanos() as f64 / n as f64);
        }
        b
    };
    let mut st = curve_seed ^ 0x5eed;
    let (d1, d2) = (
        inst.curve.random_divisor(&mut st),
        inst.curve.random_divisor(&mut st),
    );
    let n_add = 20_000u64;
    let ns_cantor = best(
        &mut || {
            let mut a = d1;
            for _ in 0..n_add {
                a = inst.curve.cantor(a, d2);
            }
            a.deg as u64
        },
        n_add,
    );
    let n = 20_000_000u64;
    let ns_mul_p = best(
        &mut || {
            let mut a = 3u64;
            for i in 0..n {
                a = black_box(a * (i | 1) % p);
            }
            a
        },
        n,
    );
    let ns_add_p = best(
        &mut || {
            let mut a = 3u64;
            for i in 0..n {
                let s = a + (i % p);
                a = black_box(if s >= p { s - p } else { s });
            }
            a
        },
        n,
    );
    let r = inst.r;
    let ns_mul_r = best(
        &mut || {
            let mut a = 3u64;
            for i in 0..n {
                a = black_box(((a as u128 * (i | 1) as u128) % r as u128) as u64);
            }
            a
        },
        n,
    );
    json!({"kind": "calibration", "p": p, "g": g, "curve_seed": curve_seed, "r": r,
        "ns_cantor_add": ns_cantor, "ns_mulmod_p": ns_mul_p, "ns_addmod_p": ns_add_p,
        "ns_mulmod_r": ns_mul_r,
        "w_cal_mul_p": ns_mul_p / ns_cantor, "w_cal_add_p": ns_add_p / ns_cantor,
        "w_cal_mul_r": ns_mul_r / ns_cantor})
}
