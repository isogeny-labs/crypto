//! `small_de_ic`: does a small endomorphism discriminant reduce the measured
//! cost of prime-field index calculus once coordinates and the factor base
//! are adapted to the endomorphism?
//!
//! Arms per cell (size, m, curve): A0 generic curve + x-interval base; A3
//! generic curve + scalar-folded base (control: folding without a small
//! `D_E`); A1 CM curve + x-interval base; A2 CM curve + endomorphism-adapted
//! base.  Matched rho (plain, negation, and the full automorphism group for
//! `D = -3, -4`) runs on the same curve and targets.  Protocol (frozen by
//! hash): `research/small_de_endo_fb_20261112/PROTOCOL.md`.
//!
//! ```text
//! small_de_ic --out-dir research/small_de_endo_fb_20261112
//! ```
//!
//! Every number printed is a `[pcc:unverified]` assertion until signed.

use clap::Parser;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::Serialize;

use crypto_lib::cryptanalysis::chained_transfer::small_de::{
    construct, endo_for, fb_aut, fb_interval, fb_layered, relation_phase, rho_cost, scalar_endo,
    solve_target, EndoKind, Fb, Inst, Native, PhaseStats, RhoCost, RhoKind, TargetStats,
};
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimePoint};

#[derive(Parser, Debug)]
#[command(about = "Small D_E with an endomorphism-adapted factor base: measured relation cost")]
struct Args {
    #[arg(long, default_value = "research/small_de_endo_fb_20261112")]
    out_dir: String,
    #[arg(long, default_value = "small_de.json")]
    json: String,
    #[arg(long, default_value = "12,16,20,24")]
    sizes: String,
    #[arg(long, default_value = "2,3")]
    ms: String,
    #[arg(long, default_value = "-3,-4,-7,-8,-11,0")]
    ds: String,
    #[arg(long, default_value_t = 16)]
    targets: usize,
    #[arg(long, default_value_t = 20261112)]
    seed: u64,
    #[arg(long, default_value_t = 4)]
    layers: usize,
    /// `bits:reps,...` relation-phase repetitions per arm.
    #[arg(long, default_value = "12:5,16:5,20:3,24:2")]
    reps: String,
    #[arg(long, default_value_t = 4)]
    threads: usize,
}

#[derive(Serialize, Clone)]
struct InstInfo {
    bits: u32,
    d_requested: i64,
    p: u64,
    a: u64,
    b: u64,
    t: i64,
    n: u64,
    h: u64,
    r: u64,
    d_k: Option<i64>,
    conductor_of_pi: Option<u64>,
    crater: Option<bool>,
    endo: Option<String>,
    lambda: Option<u64>,
    lambda_verified: bool,
    /// Least `k <= 1000` with `lambda^k = 1 (mod r)`; `None` = no such `k`
    /// (the orbit of `psi` on the subgroup is longer than 1000).
    lambda_order_le_1000: Option<u32>,
    construction_ms: u64,
}

fn small_order(lambda: u64, r: u64) -> Option<u32> {
    let mut x = 1u64;
    for k in 1..=1000u32 {
        x = x * lambda % r;
        if x == 1 {
            return Some(k);
        }
    }
    None
}

#[derive(Serialize)]
struct ArmOut {
    arm: String,
    base: String,
    curve_d: i64,
    fb_points: usize,
    columns: usize,
    dropped: u64,
    build_gae: f64,
    build_native: Native,
    reps: Vec<PhaseStats>,
    targets: Vec<TargetStats>,
}

#[derive(Serialize)]
struct RhoOut {
    kind: String,
    automorphisms: u32,
    runs: Vec<RhoCost>,
}

#[derive(Serialize)]
struct Cell {
    bits: u32,
    m: usize,
    d: i64,
    inst: InstInfo,
    n_target: usize,
    arms: Vec<ArmOut>,
    rho: Vec<RhoOut>,
}

fn splitmix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn parse_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(',')
        .filter(|x| !x.trim().is_empty())
        .filter_map(|x| x.trim().parse().ok())
        .collect()
}

fn reps_for(spec: &str, bits: u32) -> usize {
    spec.split(',')
        .filter_map(|kv| {
            let (k, v) = kv.split_once(':')?;
            (k.trim().parse::<u32>().ok()? == bits).then(|| v.trim().parse().ok())?
        })
        .next()
        .unwrap_or(1)
}

fn run_arm(
    inst: &Inst,
    fb: &Fb,
    label: &str,
    m: usize,
    reps: usize,
    ntargets: usize,
    seed: u64,
) -> ArmOut {
    let mut stats = vec![];
    let mut first_known = None;
    for k in 0..reps {
        let out = relation_phase(inst, fb, m, splitmix(seed ^ (k as u64) << 20));
        stats.push(out.stats);
        if first_known.is_none() {
            first_known = Some(out.known);
        }
    }
    let known = first_known.unwrap();
    // the same target stream for every arm of the cell
    let mut tr = StdRng::seed_from_u64(inst.curve.p ^ 0x5EED_7A26);
    let mut targets = vec![];
    let mut ops = GroupOps::default();
    for k in 0..ntargets {
        let t = tr.gen_range(1..inst.r);
        let tp = inst.curve.mul(&mut ops, inst.g, t);
        let mut ts = solve_target(
            inst,
            fb,
            &known,
            m,
            tp,
            splitmix(seed ^ 0xA5 ^ (k as u64) << 8),
            200_000,
        );
        // a recovered log that is not the planted one is a failure, not a success
        if ts.recovered.is_some_and(|x| x != t) {
            ts.certified = false;
        }
        targets.push(ts);
    }
    ArmOut {
        arm: label.to_string(),
        base: fb.label.clone(),
        curve_d: inst.d,
        fb_points: fb.len(),
        columns: fb.ncols,
        dropped: fb.dropped,
        build_gae: fb.build_ops.gae(),
        build_native: fb.build_native,
        reps: stats,
        targets,
    }
}

fn run_cell(inst: &Inst, info: &InstInfo, bits: u32, m: usize, a: &Args) -> Cell {
    let reps = reps_for(&a.reps, bits);
    let n_target = (inst.n as f64).powf(1.0 / m as f64).ceil() as usize;
    let seed = splitmix(a.seed ^ ((bits as u64) << 32) ^ ((m as u64) << 48) ^ (inst.d as u64));
    let mut arms = vec![];
    if inst.d == 0 {
        let e = scalar_endo(inst, seed ^ 0xC0DE);
        let a3 = fb_layered(inst, &e, n_target, a.layers);
        let a0 = fb_interval(inst, a3.len());
        arms.push(run_arm(inst, &a3, "A3", m, reps, a.targets, seed ^ 3));
        arms.push(run_arm(inst, &a0, "A0", m, reps, a.targets, seed));
    } else {
        let e = endo_for(inst).expect("endomorphism");
        let a2 = match e.kind {
            EndoKind::Zeta { .. } | EndoKind::Iota { .. } => fb_aut(inst, &e, n_target),
            _ => fb_layered(inst, &e, n_target, a.layers),
        };
        let a1 = fb_interval(inst, a2.len());
        arms.push(run_arm(inst, &a2, "A2", m, reps, a.targets, seed ^ 2));
        arms.push(run_arm(inst, &a1, "A1", m, reps, a.targets, seed ^ 1));
    }
    // matched rho on the same curve and the same target stream
    let mut tr = StdRng::seed_from_u64(inst.curve.p ^ 0x5EED_7A26);
    let mut ops = GroupOps::default();
    let tps: Vec<PrimePoint> = (0..a.targets)
        .map(|_| {
            let t = tr.gen_range(1..inst.r);
            inst.curve.mul(&mut ops, inst.g, t)
        })
        .collect();
    let mut kinds = vec![("plain", RhoKind::Plain), ("negation", RhoKind::Negation)];
    if inst.d == -3 || inst.d == -4 {
        kinds.push(("aut", RhoKind::Aut));
    }
    let rho = kinds
        .into_iter()
        .map(|(name, kind)| {
            let runs: Vec<RhoCost> = tps
                .iter()
                .enumerate()
                .filter_map(|(k, tp)| rho_cost(inst, kind, *tp, splitmix(seed ^ 0xB0 ^ (k as u64))))
                .collect();
            RhoOut {
                kind: name.to_string(),
                automorphisms: runs.first().map(|r| r.automorphisms).unwrap_or(0),
                runs,
            }
        })
        .collect();
    Cell {
        bits,
        m,
        d: inst.d,
        inst: info.clone(),
        n_target,
        arms,
        rho,
    }
}

fn main() {
    let a = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(a.threads)
        .build_global()
        .ok();
    let sizes: Vec<u32> = parse_list(&a.sizes);
    let ms: Vec<usize> = parse_list(&a.ms);
    let ds: Vec<i64> = parse_list(&a.ds);
    // 1. curves (one-time construction, timed apart)
    let keys: Vec<(u32, i64)> = sizes
        .iter()
        .flat_map(|&b| ds.iter().map(move |&d| (b, d)))
        .collect();
    let built: Vec<(u32, i64, Option<(Inst, InstInfo)>)> = keys
        .par_iter()
        .map(|&(bits, d)| {
            let t0 = std::time::Instant::now();
            let inst = construct(d, bits, a.seed);
            let ms_c = t0.elapsed().as_millis() as u64;
            let res = inst.map(|inst| {
                let e = if d == 0 { None } else { endo_for(&inst) };
                let info = InstInfo {
                    bits,
                    d_requested: d,
                    p: inst.curve.p,
                    a: inst.curve.a,
                    b: inst.curve.b,
                    t: inst.t,
                    n: inst.n,
                    h: inst.h,
                    r: inst.r,
                    d_k: inst.d_k,
                    conductor_of_pi: inst.f,
                    crater: inst.crater,
                    endo: e.as_ref().map(|e| e.name.to_string()),
                    lambda: e.as_ref().map(|e| e.lambda),
                    lambda_verified: e.is_some(),
                    lambda_order_le_1000: e.as_ref().and_then(|e| small_order(e.lambda, inst.r)),
                    construction_ms: ms_c,
                };
                (inst, info)
            });
            if let Some((_, i)) = &res {
                println!(
                    "curve bits={bits} D={d}: p={} t={} n={}=h{}*r{} d_K={:?} f={:?} crater={:?} endo={:?} lambda={:?} ({} ms)",
                    i.p, i.t, i.n, i.h, i.r, i.d_k, i.conductor_of_pi, i.crater, i.endo, i.lambda, i.construction_ms
                );
            } else {
                println!("curve bits={bits} D={d}: NO INSTANCE");
            }
            (bits, d, res)
        })
        .collect();
    // 2. cells
    let mut jobs: Vec<(u32, usize, usize)> = vec![];
    for (ci, (bits, _, r)) in built.iter().enumerate() {
        if r.is_some() {
            for &m in &ms {
                jobs.push((*bits, m, ci));
            }
        }
    }
    // largest first
    jobs.sort_by_key(|&(bits, m, _)| std::cmp::Reverse((bits, m)));
    let mut cells: Vec<Cell> = jobs
        .par_iter()
        .map(|&(bits, m, ci)| {
            let (_, _, r) = &built[ci];
            let (inst, info) = r.as_ref().unwrap();
            let t0 = std::time::Instant::now();
            let cell = run_cell(inst, info, bits, m, &a);
            let sol: Vec<String> = cell
                .arms
                .iter()
                .map(|x| {
                    format!(
                        "{}:{}/{} certified",
                        x.arm,
                        x.targets.iter().filter(|t| t.certified).count(),
                        x.targets.len()
                    )
                })
                .collect();
            println!(
                "cell bits={bits} m={m} D={}: {} ({} ms)",
                inst.d,
                sol.join(" "),
                t0.elapsed().as_millis()
            );
            cell
        })
        .collect();
    cells.sort_by_key(|c| (c.bits, c.m, c.d));
    let doc = serde_json::json!({
        "schema": "small_de/v1",
        "protocol": "research/small_de_endo_fb_20261112/PROTOCOL.md",
        "seed": a.seed,
        "layers": a.layers,
        "targets_per_cell": a.targets,
        "reps": a.reps,
        "cells": cells,
    });
    let path = format!("{}/{}", a.out_dir, a.json);
    std::fs::write(&path, serde_json::to_string(&doc).unwrap()).expect("write json");
    println!("wrote {path}");
}
