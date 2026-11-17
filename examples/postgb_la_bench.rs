//! **The linear stage after F4 alone, on a corpus of Gröbner bases: dense,
//! sparse FGLM (the leaderboard incumbent) and the transposed read-out.**
//!
//! Each corpus system goes through F4 once. Its grevlex basis is then handed
//! to every arm, on the same seed:
//! * `dense`: `jv_cover::linear_stage_from_basis`, `LinMode::Dense`;
//! * `sparse`: the same with `LinMode::Sparse`, the incumbent of the IC
//!   leaderboard's "Linear stage after F4" section;
//! * `fast`: `zero_dim_la::fast_stage`, falling back to `dense` when it gives up.
//!
//! The arms are interleaved (A B C, B C A, …), `--reps` rounds. A system counts
//! only if every arm returns the same solution set and contains its planted
//! point. Timings are the wall time of the linear stage alone, from the basis
//! to the checked points.
//!
//! ```text
//! cargo run --release --example postgb_la_bench -- --families all --reps 7 --json out.json
//! ```

use std::collections::HashMap;
use std::env;
use std::time::Instant;

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering as F4Ordering, Poly};
use crypto_lib::cryptanalysis::gaudry_cubic::{generate_instance3, SymmetrisedS4};
use crypto_lib::cryptanalysis::gaudry_quartic::{generate_instance4, SymmetrisedS5};
use crypto_lib::cryptanalysis::gaudry_quartic_d4::{generate_d4_instance, SymmetrisedD4};
use crypto_lib::cryptanalysis::jv_cover::{
    self, linear_stage_from_basis, nagao_systems, LinMode, ZeroDimStats,
};
use crypto_lib::cryptanalysis::zero_dim_la::{fast_stage, staircase_size, FastStats};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

#[derive(Clone)]
struct Sys {
    family: String,
    label: String,
    n: usize,
    p: u64,
    input: Vec<Poly>,
    planted: Option<Vec<u64>>,
    /// Built just before the system's F4 call, so that its budget (an
    /// absolute deadline) starts then and not when the corpus is built.
    opts: fn() -> F4Options,
}

#[derive(Serialize, Default, Clone)]
struct ArmRow {
    arm: String,
    ms: Vec<f64>,
    median_ms: f64,
    muls: u64,
    sols: usize,
    /// sparse: the incumbent's own fallbacks to dense; fast: gave up
    fallback: bool,
    gave_up: Option<String>,
    fast: Option<FastStats>,
}

#[derive(Serialize)]
struct Row {
    family: String,
    label: String,
    n: usize,
    p: u64,
    delta: usize,
    f4_ms: f64,
    planted_found: Option<bool>,
    agree: bool,
    arms: Vec<ArmRow>,
}

fn random_poly(n: usize, d: u32, p: u64, rng: &mut StdRng) -> Vec<(Vec<u32>, u64)> {
    // every monomial of total degree <= d
    let mut out = Vec::new();
    let mut e = vec![0u32; n];
    fn rec(k: usize, left: u32, e: &mut Vec<u32>, out: &mut Vec<Vec<u32>>) {
        if k == e.len() {
            out.push(e.clone());
            return;
        }
        for x in 0..=left {
            e[k] = x;
            rec(k + 1, left - x, e, out);
        }
        e[k] = 0;
    }
    let mut monos = Vec::new();
    rec(0, d, &mut e, &mut monos);
    for m in monos {
        out.push((m, rng.gen_range(0..p)));
    }
    out
}

/// `n` random dense polynomials of degree `d`, shifted through a planted point.
fn dense_system(n: usize, d: u32, p: u64, seed: u64) -> (Vec<Poly>, Vec<u64>) {
    let mut rng = StdRng::seed_from_u64(seed);
    let pt: Vec<u64> = (0..n).map(|_| rng.gen_range(0..p)).collect();
    let sys = (0..n)
        .map(|_| {
            let t = random_poly(n, d, p, &mut rng);
            let f0 = f4_fp::normalise(&t, p, F4Ordering::Grevlex);
            let v = f4_fp::eval(&f0, &pt, p);
            let mut t2 = f0.clone();
            t2.push((vec![0u32; n], (p - v) % p));
            f4_fp::normalise(&t2, p, F4Ordering::Grevlex)
        })
        .collect();
    (sys, pt)
}

/// A sparse coefficient map over `F_p` as an `f4_fp` polynomial.
fn comps_poly<const N: usize>(c: &HashMap<[u8; N], u64>, p: u64) -> Poly {
    let t: Vec<(Vec<u32>, u64)> = c
        .iter()
        .filter(|(_, &v)| v % p != 0)
        .map(|(e, &v)| (e.iter().map(|&x| x as u32).collect(), v % p))
        .collect();
    f4_fp::normalise(&t, p, F4Ordering::Grevlex)
}

fn gb_opts() -> F4Options {
    F4Options::new(F4Ordering::Grevlex, 64).with_budget(std::time::Duration::from_secs(900))
}

fn dense_opts() -> F4Options {
    F4Options::new(F4Ordering::Grevlex, 64).with_budget(std::time::Duration::from_secs(600))
}

fn jv_opts() -> F4Options {
    jv_cover::jv_options(24, 300.0)
}

fn corpus(families: &str, per: usize, shapes_only: Option<&str>) -> Vec<Sys> {
    let want = |f: &str| families == "all" || families.split(',').any(|x| x == f);
    let mut out = Vec::new();
    if want("jv-nagao") {
        for &p in &[53u64, 101, 251, 1009] {
            let sys = nagao_systems(p, 1, per, per / 2);
            for (k, (input, planted)) in sys.into_iter().enumerate() {
                out.push(Sys {
                    family: "jv-nagao".into(),
                    label: format!("p{p}-{k}{}", if planted { "c" } else { "" }),
                    n: 6,
                    p,
                    input,
                    planted: None,
                    opts: jv_opts,
                });
            }
        }
    }
    if want("gaudry-s4") {
        // symmetrised S4 over F_{p^3}, Weil-restricted: 3 quartics in (e1, e2, e3)
        for &p in &[271u64, 1039, 8191] {
            let inst = generate_instance3(p, 1);
            let pre = SymmetrisedS4::precompute(&inst.curve);
            let mut rng = StdRng::seed_from_u64(p ^ 0x54);
            for k in 0..per {
                let r = inst
                    .curve
                    .mul(&inst.curve.g, rng.gen_range(1..inst.curve.n));
                let comps = pre.weil_restrict(&inst.curve.field, &r.x);
                out.push(Sys {
                    family: "gaudry-s4".into(),
                    label: format!("p{p}-{k}"),
                    n: 3,
                    p,
                    input: comps.iter().map(|c| comps_poly(c, p)).collect(),
                    planted: None,
                    opts: gb_opts,
                });
            }
        }
    }
    if want("gaudry-d4") {
        // D4-invariant S5 over F_{p^4}: 5 equations in (σ1..σ4, π)
        for &p in &[97u64, 1033, 4093] {
            let inst = generate_d4_instance(p, 1);
            let mut rng = StdRng::seed_from_u64(p ^ 0xD4);
            let Ok(pre) = SymmetrisedD4::precompute(&inst, &mut rng, 8) else {
                eprintln!("gaudry-d4 p{p}: precompute failed");
                continue;
            };
            let mut k = 0;
            while k < per {
                let x = inst.curve.f.random(&mut rng);
                let Some(r) = inst.curve.lift_x(&x, &mut rng) else {
                    continue;
                };
                let y_r = inst.y_of_x(&r.x);
                let comps = pre.system(&inst.curve.f, &y_r);
                out.push(Sys {
                    family: "gaudry-d4".into(),
                    label: format!("p{p}-{k}"),
                    n: 5,
                    p,
                    input: comps.iter().map(|c| comps_poly(c, p)).collect(),
                    planted: None,
                    opts: gb_opts,
                });
                k += 1;
            }
        }
    }
    if want("gaudry-s5") {
        // full symmetrised S5 over F_{p^4}: 4 octics in (e1..e4), Bézout 4096
        for &p in &[13u64, 101] {
            let inst = generate_instance4(p, 1);
            let mut rng = StdRng::seed_from_u64(p ^ 0x55);
            let pre = SymmetrisedS5::precompute(&inst.curve, &mut rng);
            for k in 0..per.min(2) {
                let r = inst.curve.mul(&inst.g, rng.gen_range(1..inst.n));
                let comps = pre.weil_restrict(&inst.curve.f, &r.x);
                out.push(Sys {
                    family: "gaudry-s5".into(),
                    label: format!("p{p}-{k}"),
                    n: 4,
                    p,
                    input: comps.iter().map(|c| comps_poly(c, p)).collect(),
                    planted: None,
                    opts: gb_opts,
                });
            }
        }
    }
    let primes = [65_521u64, 2_147_483_647, 4_294_967_291];
    // (n, d): Bézout degree d^n from 64 to 1024
    let shapes: &[(usize, u32)] = &[
        (2, 8),
        (2, 16),
        (2, 32),
        (3, 4),
        (3, 6),
        (3, 8),
        (3, 10),
        (4, 3),
        (4, 4),
        (4, 5),
        (5, 2),
        (5, 3),
        (5, 4),
        (6, 2),
        (6, 3),
        (7, 2),
        (8, 2),
    ];
    if want("dense") {
        // `--shapes n2d8,n3d10,…` restricts the dense family (to split a run)
        let keep = |n: usize, d: u32| {
            shapes_only.is_none_or(|v| v.split(',').any(|x| x == format!("n{n}d{d}")))
        };
        for &(n, d) in shapes.iter().filter(|(n, d)| keep(*n, *d)) {
            for (pi, &p) in primes.iter().enumerate() {
                for k in 0..per.min(3) {
                    let (input, pt) = dense_system(
                        n,
                        d,
                        p,
                        1000 * n as u64 + 10 * d as u64 + k as u64 + 7 * pi as u64,
                    );
                    out.push(Sys {
                        family: "dense".into(),
                        label: format!("n{n}-d{d}-p{}-{k}", 64 - p.leading_zeros()),
                        n,
                        p,
                        input,
                        planted: Some(pt),
                        opts: dense_opts,
                    });
                }
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut families = "all".to_string();
    let mut reps = 5usize;
    let mut per = 6usize;
    let mut json: Option<String> = None;
    let mut max_delta = 1usize << 14;
    let mut arm_list = "dense,sparse,fast".to_string();
    let mut shapes_only: Option<String> = None;
    let mut primes_only: Option<Vec<u64>> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--families" => families = args[i + 1].clone(),
            "--reps" => reps = args[i + 1].parse().unwrap(),
            "--per" => per = args[i + 1].parse().unwrap(),
            "--json" => json = Some(args[i + 1].clone()),
            "--max-delta" => max_delta = args[i + 1].parse().unwrap(),
            "--arms" => arm_list = args[i + 1].clone(),
            "--shapes" => shapes_only = Some(args[i + 1].clone()),
            "--p" => {
                primes_only = Some(args[i + 1].split(',').map(|x| x.parse().unwrap()).collect())
            }
            a => panic!("unknown argument {a}"),
        }
        i += 2;
    }
    let systems: Vec<Sys> = corpus(&families, per, shapes_only.as_deref())
        .into_iter()
        .filter(|s| primes_only.as_ref().is_none_or(|v| v.contains(&s.p)))
        .collect();
    eprintln!("{} systems", systems.len());
    let mut rows = Vec::new();
    let mut skipped: Vec<serde_json::Value> = Vec::new();
    for (si, s) in systems.iter().enumerate() {
        let t = Instant::now();
        let r = f4_fp::f4(&s.input, s.n, s.p, &(s.opts)());
        let f4_ms = t.elapsed().as_secs_f64() * 1e3;
        if r.timed_out || r.inconsistent || r.pairs_above_bound > 0 {
            let why = if r.timed_out {
                "F4 timed out"
            } else if r.inconsistent {
                "inconsistent"
            } else {
                "F4 degree bound"
            };
            eprintln!("{} {}: F4 not usable ({why}), skipped", s.family, s.label);
            skipped.push(serde_json::json!({
                "family": s.family, "label": s.label, "n": s.n, "p": s.p,
                "f4_ms": f4_ms, "reason": why,
            }));
            continue;
        }
        // PROTOCOL.md: the dense arm (cubic in δ) is not run above δ = 1024
        let delta0 = staircase_size(&r.basis, s.n, 1 << 16).unwrap_or(0);
        let arms: Vec<&str> = arm_list
            .split(',')
            .filter(|&a| !(a == "dense" && delta0 > 1024))
            .collect();
        let mut res: Vec<ArmRow> = arms
            .iter()
            .map(|a| ArmRow {
                arm: a.to_string(),
                ..Default::default()
            })
            .collect();
        let mut sols_of: Vec<Option<Vec<Vec<u64>>>> = vec![None; arms.len()];
        let mut delta = 0usize;
        for round in 0..reps {
            for k in 0..arms.len() {
                let a = (k + round) % arms.len();
                let mut rng = StdRng::seed_from_u64(0x5EED ^ si as u64);
                let t = Instant::now();
                let (sols, muls, fb, gu, fs, d) = match arms[a] {
                    "dense" | "sparse" => {
                        let mode = if arms[a] == "dense" {
                            LinMode::Dense
                        } else {
                            LinMode::Sparse
                        };
                        let mut st = ZeroDimStats::default();
                        let sols = linear_stage_from_basis(
                            &r.basis, &s.input, s.n, s.p, &mut rng, mode, &mut st, t,
                        );
                        (
                            sols,
                            st.lin_muls,
                            st.lin_fallback > 0 || st.incomplete,
                            None,
                            None,
                            st.delta,
                        )
                    }
                    _ => {
                        let mut st = FastStats::default();
                        match fast_stage(&r.basis, &s.input, s.n, s.p, &mut rng, &mut st) {
                            Some(sols) => {
                                let d = st.delta;
                                (sols, st.muls(), false, None, Some(st), d)
                            }
                            None => {
                                let gu = st.gave_up.map(|x| x.to_string());
                                let mut st2 = ZeroDimStats::default();
                                let sols = linear_stage_from_basis(
                                    &r.basis,
                                    &s.input,
                                    s.n,
                                    s.p,
                                    &mut rng,
                                    LinMode::Dense,
                                    &mut st2,
                                    t,
                                );
                                let m = st.muls() + st2.lin_muls;
                                (sols, m, true, gu, Some(st), st2.delta)
                            }
                        }
                    }
                };
                let ms = t.elapsed().as_secs_f64() * 1e3;
                delta = delta.max(d);
                let row = &mut res[a];
                row.ms.push(ms);
                row.muls = muls;
                row.sols = sols.len();
                row.fallback = fb;
                row.gave_up = gu;
                row.fast = fs;
                sols_of[a] = Some(sols);
            }
            if delta > max_delta {
                break;
            }
        }
        for row in res.iter_mut() {
            let mut v = row.ms.clone();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            row.median_ms = v[v.len() / 2];
        }
        let agree = sols_of.iter().all(|x| x == &sols_of[0]);
        let planted_found = s
            .planted
            .as_ref()
            .map(|pt| sols_of[0].as_ref().unwrap().contains(pt));
        let cells: Vec<String> = res
            .iter()
            .map(|r| format!("{}={:9.3}", r.arm, r.median_ms))
            .collect();
        let gave_up = res
            .iter()
            .filter_map(|r| {
                r.gave_up
                    .as_ref()
                    .map(|g| format!(" {}-gave-up:{g}", r.arm))
            })
            .collect::<String>();
        eprintln!(
            "{:9} {:22} δ={:5} f4={:9.1}ms {} sols={} agree={} planted={:?}{}",
            s.family,
            s.label,
            delta,
            f4_ms,
            cells.join(" "),
            res[0].sols,
            agree,
            planted_found,
            gave_up
        );
        rows.push(Row {
            family: s.family.clone(),
            label: s.label.clone(),
            n: s.n,
            p: s.p,
            delta,
            f4_ms,
            planted_found,
            agree,
            arms: res,
        });
    }
    if let Some(path) = json {
        let doc = serde_json::json!({
            "schema": "postgb_la_bench/v1",
            "simd": crypto_lib::cryptanalysis::zero_dim_la::simd_name(),
            "fast_threads": crypto_lib::cryptanalysis::zero_dim_la::krylov_threads(),
            "families": families,
            "reps": reps,
            "per": per,
            "arms": arm_list,
            "shapes": shapes_only,
            "primes": primes_only,
            "rows": rows,
            "skipped": skipped,
        });
        std::fs::write(&path, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    }
}
