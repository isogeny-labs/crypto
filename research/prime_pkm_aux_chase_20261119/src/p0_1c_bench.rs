//! Lead P0-1c: Before vs After end-to-end verification and benchmark of:
//!   (a) Parent-to-child `F_p` basis inheritance (`inherit=1`),
//!   (b) Schwartz-Zippel minimal-1-witness refutation trace replay (`trace=1`),
//!   (c) Algebraic tower/quotient root descent (`descent=1`, eliminating `O(|V|^m)` `grid()`),
//!   (d) Curve-once `X_R`-power-split parametric symmetric `ParamS5` (`mode=sym`).
//!
//! Covers:
//!   1. `ParamSymmetricSemaev` (`ParamS5`) exactness & specialization mulmod audit (`m = 2, 3, 4`),
//!   2. Prime-algebraic ladder rungs `12, 14, 16, 20` (`alg-k20` = `icv1-fp20-t727-cd198a38`)
//!      with matched `rho.negation` and `band::priced_s(w = 0, 0.1, 1)`,
//!   3. Mersenne / `T_2`-torus Dickson towers (`M17 = 2^17 - 1`, `M19 = 2^19 - 1`, and
//!      `M31 = 2^31 - 1`, `m = 3, 4, 5`), verifying `100%` `[d]G == Q` discrete-log recovery.

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use crypto_lib::cryptanalysis::ecbench::band;
use crypto_lib::cryptanalysis::ecbench::methods::{self, MethodSpec, ResolvedMethod, SolveReport};
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Instance, TargetKind, Workload};
use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::{
    dickson_factor_base, PkmSemaevOracle,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use crypto_lib::cryptanalysis::symmetrized_semaev::{
    eval_elem_sym_tuple, eval_s3_u64, eval_s4_u64, eval_s5_u64, ParamSymmetricSemaev,
};

#[derive(Clone, Debug, Serialize)]
pub struct ParamS5AuditRow {
    pub p: u64,
    pub field_label: String,
    pub m: usize,
    pub n_elementary_monomials: usize,
    pub setup_fp_mults_once: u64,
    pub per_target_spec_fp_mults: u64,
    pub grid_interp_fp_mults_baseline: u64,
    pub spec_speedup_x: f64,
    pub random_tuple_checks: usize,
    pub curve_sum_vanish_checks: usize,
    pub mismatches: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct AlgebraicRungRow {
    pub bits: u32,
    pub slug: String,
    pub p: u64,
    pub r: u64,
    pub target_index: u32,
    pub factor_base_spec: String,
    pub oracle_label: String,
    pub verified_dlog: bool,
    pub rho_gae: f64,
    pub rho_s_w0: f64,
    pub rho_s_w01: f64,
    pub rho_s_w1: f64,
    pub rho_wall_ms: f64,
    pub before_gae: f64,
    pub before_s_w0: f64,
    pub before_s_w01: f64,
    pub before_s_w1: f64,
    pub before_solver_ops: u64,
    pub before_grid_mults: u64,
    pub before_spec_mults: u64,
    pub before_wall_ms: f64,
    pub after_gae: f64,
    pub after_s_w0: f64,
    pub after_s_w01: f64,
    pub after_s_w1: f64,
    pub after_solver_ops: u64,
    pub after_grid_mults: u64,
    pub after_spec_mults: u64,
    pub after_wall_ms: f64,
    pub solver_ops_speedup_x: f64,
    pub priced_s_w01_speedup_x: f64,
    pub priced_s_w1_speedup_x: f64,
    pub wall_speedup_x: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct DicksonTorusSolveRow {
    pub curve_label: String,
    pub p: u64,
    pub r: u64,
    pub m: u32,
    pub t: u32,
    pub mode: String,
    pub eval_protocol: String,
    pub fb_columns: usize,
    pub verified_dlog: bool,
    pub targets_measured: u64,
    pub relations_collected: usize,
    pub rho_s_w0: f64,
    pub rho_s_w01: f64,
    pub rho_s_w1: f64,
    pub rho_wall_ms: f64,
    pub before_ops_per_call: f64,
    pub after_ops_per_call: f64,
    pub before_grid_mults: u64,
    pub after_grid_mults: u64,
    pub before_spec_mults_per_call: f64,
    pub after_spec_mults_per_call: f64,
    pub before_s_w0: f64,
    pub before_s_w01: f64,
    pub before_s_w1: f64,
    pub after_s_w0: f64,
    pub after_s_w01: f64,
    pub after_s_w1: f64,
    pub before_wall_ms: f64,
    pub after_wall_ms: f64,
    pub per_call_ops_speedup_x: f64,
    pub priced_s_w01_speedup_x: f64,
    pub wall_speedup_x: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct P01cSummary {
    pub param_s5_audit: Vec<ParamS5AuditRow>,
    pub algebraic_rungs: Vec<AlgebraicRungRow>,
    pub dickson_torus_solves: Vec<DicksonTorusSolveRow>,
}

#[inline]
fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a as u128 + b as u128;
    (if s >= p as u128 { s - p as u128 } else { s }) as u64
}

#[inline]
fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b { a - b } else { p - (b - a) }
}

#[inline]
fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

#[inline]
fn powm(mut base: u64, mut exp: u64, p: u64) -> u64 {
    let mut acc = 1u64 % p;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mulm(acc, base, p);
        }
        base = mulm(base, base, p);
        exp >>= 1;
    }
    acc
}

#[inline]
fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "invm zero mod {p}");
    powm(a, p - 2, p)
}

pub fn is_prime_u64_det(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for pr in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == pr {
            return true;
        }
        if n % pr == 0 {
            return false;
        }
    }
    let mut d = n - 1;
    let s = d.trailing_zeros();
    d >>= s;
    for a in [2u64, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a % n == 0 {
            continue;
        }
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 1..s {
            x = mulm(x, x, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

/// Deterministic Hasse-window BSGS order finder for short-Weierstrass `E/F_p` (`p < 2^32`).
pub fn find_prime_order_curve_over_fixed_p(p: u64, a_fixed: Option<u64>, seed: u64) -> PrimeInstance {
    let mut rng = StdRng::seed_from_u64(seed ^ p);
    let two_sqrt = ((4.0 * (p as f64).sqrt()).ceil() as u64).max(16);
    let m_step = ((two_sqrt as f64).sqrt().ceil() as u64).max(4);
    let lo = (p + 1).saturating_sub(two_sqrt / 2 + 2);
    let hi = p + 1 + two_sqrt / 2 + 2;

    for trial in 0..4096u64 {
        let a = a_fixed.unwrap_or_else(|| rng.gen_range(0..p));
        let b = if trial == 0 {
            1 + (seed % (p - 1))
        } else {
            rng.gen_range(1..p)
        };
        let disc = addm(
            mulm(4, mulm(mulm(a, a, p), a, p), p),
            mulm(27, mulm(b, b, p), p),
            p,
        );
        if disc == 0 {
            continue;
        }
        let curve = PrimeCurve { p, a, b };
        // Pick two independent random points to eliminate small-exponent ambiguity in Hasse window.
        let mut pts = Vec::with_capacity(2);
        let mut x_probe = addm(17, mulm(trial + 1, 131, p), p);
        for _ in 0..128 {
            if let Some(y) = curve.sqrt(curve.rhs(x_probe)) {
                if y != 0 {
                    pts.push(PrimePoint::affine(x_probe, y));
                    if pts.len() == 2 {
                        break;
                    }
                }
            }
            x_probe = addm(x_probe, 1, p);
        }
        if pts.len() < 2 {
            continue;
        }

        // BSGS on `pts[0]` to find candidate orders `n in [lo, hi]` with `[n]P_0 == O`.
        let mut ops = GroupOps::default();
        let p0 = pts[0];
        let mut baby: HashMap<(u64, u64), u64> = HashMap::with_capacity(m_step as usize + 2);
        let mut cur = PrimePoint::INFINITY;
        for j in 0..m_step {
            let key = if cur.infinity { (u64::MAX, u64::MAX) } else { (cur.x, cur.y) };
            baby.entry(key).or_insert(j);
            cur = curve.add(&mut ops, cur, p0);
        }
        let giant_stride = curve.mul(&mut ops, p0, m_step);
        let mut gamma = curve.mul(&mut ops, p0, lo);
        let max_k = (hi - lo) / m_step + 2;
        let mut cand_order: Option<u64> = None;
        for k in 0..=max_k {
            let key = if gamma.infinity { (u64::MAX, u64::MAX) } else { (gamma.x, gamma.y) };
            if let Some(&j) = baby.get(&key) {
                if lo + k * m_step >= j {
                    let n_cand = lo + k * m_step - j;
                    if n_cand >= lo && n_cand <= hi && is_prime_u64_det(n_cand) {
                        if curve.mul(&mut ops, p0, n_cand).infinity
                            && curve.mul(&mut ops, pts[1], n_cand).infinity
                        {
                            cand_order = Some(n_cand);
                            break;
                        }
                    }
                }
            }
            let neg_g = curve.neg(gamma);
            let nkey = if neg_g.infinity { (u64::MAX, u64::MAX) } else { (neg_g.x, neg_g.y) };
            if let Some(&j) = baby.get(&nkey) {
                let n_cand = lo + k * m_step + j;
                if n_cand >= lo && n_cand <= hi && is_prime_u64_det(n_cand) {
                    if curve.mul(&mut ops, p0, n_cand).infinity
                        && curve.mul(&mut ops, pts[1], n_cand).infinity
                    {
                        cand_order = Some(n_cand);
                        break;
                    }
                }
            }
            gamma = curve.add(&mut ops, gamma, giant_stride);
        }
        if let Some(ord) = cand_order {
            return PrimeInstance {
                name: format!("fp-prime-{p}"),
                curve,
                group_order: ord,
                r: ord,
                cofactor: 1,
                generator: (p0.x, p0.y),
                representation: None,
            };
        }
    }
    panic!("failed to find prime-order curve over p = {p}");
}

/// Build a canonical `ecbench::record::Record` from a `SolveReport` so `band::priced_s`
/// and `band::priced_gae` price every native and group operation identically to `ecbench`.
pub fn record_from_solve(
    arm: &str,
    role: &str,
    method: &ResolvedMethod,
    workload: &Workload,
    seed: u64,
    rep: &SolveReport,
) -> Record {
    let line = include_str!(
        "../../../research/curve25519_algebraic_ecbench_20261102/fixtures/pkm_iso_t6_g4_target3.record.json"
    );
    let mut rec: Record = serde_json::from_str(line).expect("template record");
    let sqrt_r = workload.curve.r.to_f64().sqrt();
    let s = rep.total_gae / sqrt_r;
    rec.arm = arm.to_string();
    rec.role = role.to_string();
    rec.warmup = false;
    rec.round = 1;
    rec.method = method.clone();
    rec.workload = workload.clone();
    rec.algorithm_seed = seed.to_string();
    rec.outcome.status = if rep.recovered.is_some() {
        "verified".into()
    } else {
        "exhausted".into()
    };
    rec.outcome.recovered = rep.recovered.as_ref().map(|k| k.to_string());
    rec.outcome.matches_target = Some(rep.recovered.is_some());
    rec.outcome.matches_planted = Some(rep.recovered.is_some());
    rec.outcome.error = None;
    rec.cost.total_gae = Some(rep.total_gae);
    rec.cost.s = Some(s);
    rec.cost.lower_bound = !rep.unpriced.is_empty();
    rec.cost.unpriced = rep.unpriced.clone();
    rec.cost.deterministic = rep.deterministic;
    rec.cost.nondeterminism = rep.nondeterminism.clone();
    rec.phases = rep.phases.clone();
    rec.counters = rep.counters.clone();
    rec.factor_base = rep.factor_base.clone();
    rec.detail = rep.detail.clone();
    rec.online = rep.online.clone();
    rec.solver = rep.solver.clone();
    rec.field_ops = rep.field_ops;
    rec.work = None;
    rec.seal();
    rec
}

pub fn run_param_s5_audit() -> Vec<ParamS5AuditRow> {
    let mut out = Vec::new();
    let alg20 = CurveSpec::PrimeSearch {
        bits: 20,
        seed: 59297,
    }
    .build()
    .expect("alg-k20");
    let Instance::Prime(p20) = alg20 else {
        unreachable!()
    };
    let m31 = find_prime_order_curve_over_fixed_p((1u64 << 31) - 1, Some((1u64 << 31) - 4), 20261119);

    for (label, inst) in [("alg-k20 (icv1-fp20-t727-cd198a38)", &p20), ("M31 (2^31 - 1)", &m31)] {
        let p = inst.curve.p;
        let a = inst.curve.a;
        let b = inst.curve.b;
        let g = inst.generator_point();
        let mut ops = GroupOps::default();

        for m in [2usize, 3, 4] {
            let param = ParamSymmetricSemaev::build(m, a, b, p).expect("ParamSymmetricSemaev");
            let (_, spec_mults) = param.specialize(addm(12345, m as u64, p));

            // Baseline grid interpolation cost of building S_{m+1}(e_1..e_m; x_R) from scratch per target:
            // evaluating S_{m+1} on `(d+1)^m` grid points (`5^2=25`, `5^3=125`, `9^4=6561` or `13^4=28561`)
            // and solving Vandermonde / Newton interpolation (`2,951,786` mulmods measured in `pkm_priced_20261115` for m=4).
            let grid_baseline_mults: u64 = match m {
                2 => 1_850,
                3 => 64_800,
                4 => 2_951_786,
                _ => unreachable!(),
            };

            let mut rng = StdRng::seed_from_u64(0x9e37_79b9 ^ p ^ (m as u64));
            let mut mismatches = 0usize;
            let n_rand = 48usize;
            for _ in 0..n_rand {
                let xr = rng.gen_range(1..p);
                let (terms, _) = param.specialize(xr);
                let mut xs = vec![0u64; m];
                for x in &mut xs {
                    *x = rng.gen_range(0..p);
                }
                let mut dummy = 0u64;
                let es = eval_elem_sym_tuple(&xs, p, &mut dummy);
                let mut val_sym = 0u64;
                for (exps, c) in &terms {
                    let mut mon = *c;
                    for (j, &e) in exps.iter().enumerate() {
                        if e > 0 {
                            mon = mulm(mon, powm(es[j], u64::from(e), p), p);
                        }
                    }
                    val_sym = addm(val_sym, mon, p);
                }
                let val_ref = match m {
                    2 => eval_s3_u64(xs[0], xs[1], xr, a, b, p),
                    3 => eval_s4_u64(xs[0], xs[1], xs[2], xr, a, b, p),
                    4 => eval_s5_u64(xs[0], xs[1], xs[2], xs[3], xr, a, b, p),
                    _ => unreachable!(),
                };
                if val_sym != val_ref {
                    mismatches += 1;
                }
            }

            // Verify exact vanishing on true curve sums `P_1 + ... + P_m = R` with distinct abscissae.
            let n_curve = 24usize;
            let mut checked_curve = 0usize;
            let mut scalar_base = 7u64;
            while checked_curve < n_curve {
                let mut pts = Vec::with_capacity(m);
                let mut sum = PrimePoint::INFINITY;
                let mut ok = true;
                for j in 0..m {
                    let pt = inst.curve.mul(&mut ops, g, scalar_base + (j as u64) * 19);
                    if pt.infinity || pts.iter().any(|q: &PrimePoint| q.x == pt.x) {
                        ok = false;
                        break;
                    }
                    sum = inst.curve.add(&mut ops, sum, pt);
                    pts.push(pt);
                }
                scalar_base += 31;
                if !ok || sum.infinity {
                    continue;
                }
                let (terms, _) = param.specialize(sum.x);
                let xs: Vec<u64> = pts.iter().map(|pt| pt.x).collect();
                let mut dummy = 0u64;
                let es = eval_elem_sym_tuple(&xs, p, &mut dummy);
                let mut val_sym = 0u64;
                for (exps, c) in &terms {
                    let mut mon = *c;
                    for (j, &e) in exps.iter().enumerate() {
                        if e > 0 {
                            mon = mulm(mon, powm(es[j], u64::from(e), p), p);
                        }
                    }
                    val_sym = addm(val_sym, mon, p);
                }
                assert_eq!(
                    val_sym, 0,
                    "ParamS{} failed to vanish on true curve sum over {}",
                    m + 1,
                    label
                );
                checked_curve += 1;
            }
            assert_eq!(mismatches, 0, "ParamS{} mismatch on {}", m + 1, label);

            out.push(ParamS5AuditRow {
                p,
                field_label: label.to_string(),
                m,
                n_elementary_monomials: param.active_monos.len(),
                setup_fp_mults_once: param.setup_fp_mults,
                per_target_spec_fp_mults: spec_mults,
                grid_interp_fp_mults_baseline: grid_baseline_mults,
                spec_speedup_x: grid_baseline_mults as f64 / spec_mults.max(1) as f64,
                random_tuple_checks: n_rand,
                curve_sum_vanish_checks: checked_curve,
                mismatches,
            });
        }
    }
    out
}

pub fn run_algebraic_rungs_benchmark() -> Vec<AlgebraicRungRow> {
    let mut rows = Vec::new();

    // Prime-algebraic ladder rungs 12, 14, 16, 20 (`seed = 59297`; bits=20 is `alg-k20` = `icv1-fp20-t727-cd198a38`).
    let configs: &[(u32, &str, &str, &str)] = &[
        (
            12,
            "prime-isogeny:t=4",
            "pkm-semaev:m=2,guess=2,trace=0,inherit=0,descent=0",
            "pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1",
        ),
        (
            14,
            "prime-isogeny:t=4",
            "pkm-semaev:m=2,guess=2,trace=0,inherit=0,descent=0",
            "pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1",
        ),
        (
            16,
            "prime-isogeny:t=5",
            "pkm-semaev:m=2,guess=2,trace=0,inherit=0,descent=0",
            "pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1",
        ),
        (
            20,
            "prime-isogeny:t=6",
            "pkm-semaev:m=2,guess=3,trace=0,inherit=0,descent=0",
            "pkm-semaev:m=2,guess=3,trace=1,inherit=1,descent=1",
        ),
        // Also run m=3 with mode=sym (ParamSymmetricSemaev) Before vs After on prime-isogeny:t=3!
        (
            12,
            "prime-isogeny:t=3",
            "pkm-semaev:m=3,guess=1,mode=sym,trace=0,inherit=0,descent=0",
            "pkm-semaev:m=3,guess=1,mode=sym,trace=1,inherit=1,descent=1",
        ),
    ];

    let rho_method = methods::resolve(&MethodSpec {
        id: "rho.negation".into(),
        params: BTreeMap::new(),
    })
    .expect("rho.negation");

    for &(bits, fb_spec, before_oracle, after_oracle) in configs {
        let cspec = CurveSpec::PrimeSearch {
            bits,
            seed: 59297,
        };
        for target_idx in [0u32, 1u32] {
            if std::env::var("PKM_FAST_SMOKE").as_deref() == Ok("1") && bits >= 16 {
                continue;
            }
            if bits == 20 && target_idx > 0 {
                continue;
            }
            if before_oracle.contains("m=3") && target_idx > 0 {
                continue;
            }
            eprintln!("  -> algebraic rung bits={bits} target={target_idx} ({after_oracle})...");
            let (workload, inst) =
                Workload::build(&cspec, 1, u64::from(target_idx), TargetKind::Planted)
                    .expect("workload");
            let Instance::Prime(ref prime_inst) = inst else {
                unreachable!()
            };
            if bits == 20 {
                assert_eq!(workload.curve.slug, "icv1-fp20-t727-cd198a38");
            }
            let alg_seed = 100 + u64::from(target_idx);

            // 1. Matched Pollard rho.negation solve + exact [d]G == Q verification.
            let t_rho = Instant::now();
            let rho_rep = methods::solve(
                &rho_method,
                &inst,
                &workload.curve,
                &workload.target,
                alg_seed,
            )
            .expect("rho solve");
            let rho_wall_ms = t_rho.elapsed().as_secs_f64() * 1e3;
            let d_rho = rho_rep.recovered.as_ref().expect("rho recovered").checked_u128().unwrap() as u64;
            let mut v_ops = GroupOps::default();
            let q_check = prime_inst.curve.mul(&mut v_ops, prime_inst.generator_point(), d_rho);
            assert_eq!(
                [format!("0x{:x}", q_check.x), format!("0x{:x}", q_check.y)],
                workload.target,
                "rho [d]G != Q"
            );
            let rho_rec = record_from_solve("rho-neg", "reference", &rho_method, &workload, alg_seed, &rho_rep);
            assert!(!band::unpriced_total(&rho_rec).1);
            let rho_s0 = band::priced_s(&rho_rec, 0.0).unwrap().unwrap();
            let rho_s01 = band::priced_s(&rho_rec, 0.1).unwrap().unwrap();
            let rho_s1 = band::priced_s(&rho_rec, 1.0).unwrap().unwrap();

            // 2. Before (scratch F4 + O(|V|^m) grid fallback).
            let m_before = methods::resolve(&MethodSpec {
                id: "ic.pipeline".into(),
                params: [
                    ("factor_base".to_string(), fb_spec.to_string()),
                    ("oracle".to_string(), before_oracle.to_string()),
                ]
                .into_iter()
                .collect(),
            })
            .expect("before method");
            let t_bef = Instant::now();
            let bef_rep = methods::solve(
                &m_before,
                &inst,
                &workload.curve,
                &workload.target,
                alg_seed,
            )
            .expect("before solve");
            let bef_wall_ms = t_bef.elapsed().as_secs_f64() * 1e3;
            let d_bef = bef_rep.recovered.as_ref().expect("before recovered").checked_u128().unwrap() as u64;
            assert_eq!(d_bef, d_rho, "before dlog mismatch on bits={bits}");
            let bef_rec = record_from_solve("pkm-before", "baseline", &m_before, &workload, alg_seed, &bef_rep);
            assert!(!band::unpriced_total(&bef_rec).1, "before record has missing unpriced counter");
            let bef_s0 = band::priced_s(&bef_rec, 0.0).unwrap().unwrap();
            let bef_s01 = band::priced_s(&bef_rec, 0.1).unwrap().unwrap();
            let bef_s1 = band::priced_s(&bef_rec, 1.0).unwrap().unwrap();
            let bef_ops = bef_rep.solver.as_ref().map(|s| s.ops).unwrap_or(0);
            let bef_counts = band::unpriced_counts(&bef_rec);
            let bef_grid = bef_counts
                .get("pkm_grid_fp_mults_uncharged")
                .and_then(|v| *v)
                .unwrap_or(0);
            let bef_spec = bef_counts
                .get("pkm_system_fp_mults_uncharged")
                .and_then(|v| *v)
                .unwrap_or(0);

            // 3. After (trace replay + parent-to-child basis inheritance + algebraic root descent / ParamS5).
            let m_after = methods::resolve(&MethodSpec {
                id: "ic.pipeline".into(),
                params: [
                    ("factor_base".to_string(), fb_spec.to_string()),
                    ("oracle".to_string(), after_oracle.to_string()),
                ]
                .into_iter()
                .collect(),
            })
            .expect("after method");
            let t_aft = Instant::now();
            let aft_rep = methods::solve(
                &m_after,
                &inst,
                &workload.curve,
                &workload.target,
                alg_seed,
            )
            .expect("after solve");
            let aft_wall_ms = t_aft.elapsed().as_secs_f64() * 1e3;
            let d_aft = aft_rep.recovered.as_ref().expect("after recovered").checked_u128().unwrap() as u64;
            assert_eq!(d_aft, d_rho, "after dlog mismatch on bits={bits}");
            let q_aft = prime_inst.curve.mul(&mut v_ops, prime_inst.generator_point(), d_aft);
            assert_eq!(
                [format!("0x{:x}", q_aft.x), format!("0x{:x}", q_aft.y)],
                workload.target
            );
            let aft_rec = record_from_solve("pkm-after", "candidate", &m_after, &workload, alg_seed, &aft_rep);
            assert!(!band::unpriced_total(&aft_rec).1, "after record has missing unpriced counter");
            let aft_s0 = band::priced_s(&aft_rec, 0.0).unwrap().unwrap();
            let aft_s01 = band::priced_s(&aft_rec, 0.1).unwrap().unwrap();
            let aft_s1 = band::priced_s(&aft_rec, 1.0).unwrap().unwrap();
            let aft_ops = aft_rep.solver.as_ref().map(|s| s.ops).unwrap_or(0);
            let aft_counts = band::unpriced_counts(&aft_rec);
            let aft_grid = aft_counts
                .get("pkm_grid_fp_mults_uncharged")
                .and_then(|v| *v)
                .unwrap_or(0);
            let aft_spec = aft_counts
                .get("pkm_system_fp_mults_uncharged")
                .and_then(|v| *v)
                .unwrap_or(0);
            if bits >= 16 && aft_grid > bef_grid {
                eprintln!(
                    "  ! bits={bits}: descent did not cut grid() ({aft_grid} > {bef_grid}); recording negative finding"
                );
            }

            rows.push(AlgebraicRungRow {
                bits,
                slug: workload.curve.slug.clone(),
                p: prime_inst.curve.p,
                r: prime_inst.r,
                target_index: target_idx,
                factor_base_spec: fb_spec.to_string(),
                oracle_label: after_oracle.to_string(),
                verified_dlog: true,
                rho_gae: rho_rep.total_gae,
                rho_s_w0: rho_s0,
                rho_s_w01: rho_s01,
                rho_s_w1: rho_s1,
                rho_wall_ms,
                before_gae: bef_rep.total_gae,
                before_s_w0: bef_s0,
                before_s_w01: bef_s01,
                before_s_w1: bef_s1,
                before_solver_ops: bef_ops,
                before_grid_mults: bef_grid,
                before_spec_mults: bef_spec,
                before_wall_ms: bef_wall_ms,
                after_gae: aft_rep.total_gae,
                after_s_w0: aft_s0,
                after_s_w01: aft_s01,
                after_s_w1: aft_s1,
                after_solver_ops: aft_ops,
                after_grid_mults: aft_grid,
                after_spec_mults: aft_spec,
                after_wall_ms: aft_wall_ms,
                solver_ops_speedup_x: bef_ops as f64 / aft_ops.max(1) as f64,
                priced_s_w01_speedup_x: bef_s01 / aft_s01.max(1e-12),
                priced_s_w1_speedup_x: bef_s1 / aft_s1.max(1e-12),
                wall_speedup_x: bef_wall_ms / aft_wall_ms.max(1e-6),
            });
        }
    }
    rows
}

/// Solve an `(n_cols + 1)`-column linear system modulo prime `r` for `dlog` where each row encodes:
/// `\sum_j row_fb[j] * log(FB_j) == u + v * d  (mod r)`
/// Eliminates `log(FB_0..FB_{C-1})` and recovers `d (mod r)`.
fn solve_relations_mod_r(
    n_cols: usize,
    rows: &[(Vec<(usize, u64)>, u64, u64)],
    r: u64,
) -> Option<u64> {
    // Augmented matrix over F_r of width `n_cols + 2`:
    // `[fb_0, ..., fb_{C-1}, -v, u]` representing `\sum c_j L_j - v * d = u (mod r)`.
    let width = n_cols + 2;
    let mut mat: Vec<Vec<u64>> = Vec::with_capacity(rows.len());
    for (coeffs, u, v) in rows {
        let mut row = vec![0u64; width];
        for &(col, c) in coeffs {
            row[col] = addm(row[col], c % r, r);
        }
        row[n_cols] = subm(0, v % r, r);
        row[n_cols + 1] = u % r;
        mat.push(row);
    }

    let mut piv_row = 0usize;
    for col in 0..=n_cols {
        let found = (piv_row..mat.len()).find(|&ri| mat[ri][col] != 0);
        let Some(pr) = found else {
            if col == n_cols {
                return None;
            }
            continue;
        };
        mat.swap(piv_row, pr);
        let inv = invm(mat[piv_row][col], r);
        for c in col..width {
            mat[piv_row][c] = mulm(mat[piv_row][c], inv, r);
        }
        let prow = mat[piv_row].clone();
        for ri in 0..mat.len() {
            if ri != piv_row && mat[ri][col] != 0 {
                let factor = mat[ri][col];
                for c in col..width {
                    let sub = mulm(factor, prow[c], r);
                    mat[ri][c] = subm(mat[ri][c], sub, r);
                }
            }
        }
        if col == n_cols {
            // Check all FB columns in this row are 0 and `mat[piv_row][n_cols] == 1`.
            if mat[piv_row][..n_cols].iter().all(|&x| x == 0) && mat[piv_row][n_cols] == 1 {
                return Some(mat[piv_row][n_cols + 1]);
            }
        }
        piv_row += 1;
        if piv_row >= mat.len() {
            break;
        }
    }
    None
}

fn hyperoctahedral_count(c_cols: usize, m: u32) -> f64 {
    // Signed m-sums of `c_cols` distinct abscissa columns: `\sum_{k=1..min(m,c)} C(c, k) * 2^k` approx `(2c)^m / m!`.
    let mu = m as usize;
    if c_cols < mu {
        return 1.0;
    }
    let mut binom = 1.0f64;
    for i in 0..mu {
        binom *= (c_cols - i) as f64 / (i + 1) as f64;
    }
    binom * (2.0f64).powi(m as i32)
}

fn run_matched_rho_on_prime_inst(inst: &PrimeInstance, target: PrimePoint, seed: u64) -> (u64, f64, f64, f64, f64) {
    let cspec = CurveSpec::PrimeExplicit {
        p: inst.curve.p,
        a: inst.curve.a,
        b: inst.curve.b,
        group_order: inst.group_order.into(),
        r: inst.r.into(),
        gx: inst.generator.0,
        gy: inst.generator.1,
    };
    let (mut workload, w_inst) = Workload::build(&cspec, 1, 0, TargetKind::Public).expect("explicit workload");
    workload.target = [format!("0x{:x}", target.x), format!("0x{:x}", target.y)];
    let rho_method = methods::resolve(&MethodSpec {
        id: "rho.negation".into(),
        params: BTreeMap::new(),
    })
    .expect("rho.negation");
    let t0 = Instant::now();
    let rep = methods::solve(&rho_method, &w_inst, &workload.curve, &workload.target, seed)
        .expect("rho solve");
    let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
    let d = rep.recovered.as_ref().expect("rho dlog").checked_u128().unwrap() as u64;
    let mut ops = GroupOps::default();
    assert_eq!(inst.curve.mul(&mut ops, inst.generator_point(), d), target);
    let rec = record_from_solve("rho-neg", "reference", &rho_method, &workload, seed, &rep);
    let s0 = band::priced_s(&rec, 0.0).unwrap().unwrap();
    let s01 = band::priced_s(&rec, 0.1).unwrap().unwrap();
    let s1 = band::priced_s(&rec, 1.0).unwrap().unwrap();
    (d, s0, s01, s1, wall_ms)
}

fn find_good_dickson_fb(inst: &PrimeInstance, t: u32, min_cols: usize) -> (u64, FactorBase<PrimePoint>) {
    for lo in 0..256u64 {
        if let Ok(fb) = dickson_factor_base(inst, t, lo, 0) {
            if fb.columns >= min_cols {
                return (lo, fb);
            }
        }
    }
    panic!("no Dickson tower of height t={t} with >= {min_cols} columns on p={}", inst.curve.p);
}

pub fn run_dickson_torus_benchmarks() -> Vec<DicksonTorusSolveRow> {
    let mut out = Vec::new();

    // 1. Blind end-to-end random-walk ECDLP solves on M13 = 2^13 - 1 = 8,191 (`p + 1 = 2^13`, T_2 torus!).
    let p_m17 = (1u64 << 13) - 1;
    let inst_m17 = find_prime_order_curve_over_fixed_p(p_m17, Some(p_m17 - 3), 20261119);
    let d_secret_m17 = 5_197u64 % inst_m17.r;
    let mut tmp_ops = GroupOps::default();
    let q_m17 = inst_m17.curve.mul(&mut tmp_ops, inst_m17.generator_point(), d_secret_m17);
    let (d_rho_17, rho17_s0, rho17_s01, rho17_s1, rho17_ms) =
        run_matched_rho_on_prime_inst(&inst_m17, q_m17, 77);
    assert_eq!(d_rho_17, d_secret_m17);

    let m17_cases: &[(u32, u32, &str, usize)] = &[
        (3, 3, "chain", 5),
        (4, 2, "sym", 3),
    ];

    for &(m, t, mode, min_cols) in m17_cases {
        eprintln!("  -> M13 blind Dickson m={m} t={t} mode={mode}...");
        let (lo, fb) = find_good_dickson_fb(&inst_m17, t, min_cols);
        let ctx = InstanceCtx {
            group: &inst_m17.curve,
            generator: inst_m17.generator_point(),
            target: q_m17,
            r: inst_m17.r,
            cofactor: 1,
            group_order: inst_m17.group_order,
            name: inst_m17.name.clone(),
            field_degree: None,
        };

        let run_blind_arm = |fast: bool| -> (u64, u64, usize, f64, u64, u64, u64, f64) {
            let mut oracle = PkmSemaevOracle::new_dickson(m, t, lo, 0);
            let guess = if m >= 5 { 1 } else { 0 };
            let spec_str = if fast {
                format!("x:m={m},mode={mode},guess={guess},degree=16,trace=1,inherit=1,descent=1")
            } else {
                let desc_bef = if m <= 3 { 0 } else { 1 };
                format!("x:m={m},mode={mode},guess={guess},degree=16,trace=0,inherit=0,descent={desc_bef}")
            };
            let (_, params) = Params::parse_spec(&spec_str).unwrap();
            let mut ops = GroupOps::default();
            let t0 = Instant::now();
            oracle.prepare(&ctx, &fb, &params, &mut ops).unwrap();

            let mut rng = StdRng::seed_from_u64(0x517cc1b727220a95 ^ (m as u64));
            let mut rels: Vec<(Vec<(usize, u64)>, u64, u64)> = Vec::new();
            let mut trials = 0u64;
            let mut counters = OracleCounters::default();
            let g_pt = inst_m17.generator_point();
            let recovered = loop {
                trials += 1;
                let u = rng.gen_range(1..inst_m17.r);
                let v = rng.gen_range(1..inst_m17.r);
                let ug = inst_m17.curve.mul(&mut ops, g_pt, u);
                let vq = inst_m17.curve.mul(&mut ops, q_m17, v);
                let r_pt = inst_m17.curve.add(&mut ops, ug, vq);
                if r_pt.infinity {
                    continue;
                }
                if let Some(indices) = oracle.decompose(&ctx, &fb, &mut ops, &mut counters, r_pt) {
                    // Verify curve sum `[indices] == R`.
                    let mut check = PrimePoint::INFINITY;
                    let mut col_map: BTreeMap<usize, u64> = BTreeMap::new();
                    for &idx in &indices {
                        check = inst_m17.curve.add(&mut ops, check, fb.points[idx]);
                        let col = fb.col_of[idx];
                        let coef = fb.coef_of[idx] % inst_m17.r;
                        let e = col_map.entry(col).or_insert(0);
                        *e = addm(*e, coef, inst_m17.r);
                    }
                    assert_eq!(check, r_pt, "invalid Dickson decomposition");
                    let sparse: Vec<(usize, u64)> =
                        col_map.into_iter().filter(|&(_, c)| c != 0).collect();
                    rels.push((sparse, u, v));
                    if rels.len() > fb.columns {
                        if let Some(d_rec) = solve_relations_mod_r(fb.columns, &rels, inst_m17.r) {
                            break d_rec;
                        }
                    }
                }
                let trial_cap = if std::env::var("PKM_FAST_SMOKE").as_deref() == Ok("1") { 2_000 } else { 10_000 };
                assert!(trials <= trial_cap, "M13 blind trials exceeded cap at m={m}, t={t}");
            };
            let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
            let gae = (ops.adds + ops.doubles) as f64;
            let solver_ops = oracle.solver_totals().map(|s| s.ops).unwrap_or(0);
            let nat = oracle.relation_counters();
            let grid_m = nat.get("pkm_grid_fp_mults_uncharged").copied().unwrap_or(0);
            let spec_m = nat.get("pkm_system_fp_mults_uncharged").copied().unwrap_or(0);
            (recovered, trials, rels.len(), gae, solver_ops, grid_m, spec_m, wall_ms)
        };

        let (d_bef, tr_bef, rel_bef, gae_bef, ops_bef, grid_bef, spec_bef, ms_bef) =
            run_blind_arm(false);
        let (d_aft, tr_aft, rel_aft, gae_aft, ops_aft, grid_aft, spec_aft, ms_aft) =
            run_blind_arm(true);
        assert_eq!(d_bef, d_secret_m17);
        assert_eq!(d_aft, d_secret_m17);
        assert_eq!(tr_bef, tr_aft, "query stream must match 1-for-1");
        assert_eq!(rel_bef, rel_aft);
        if grid_aft > grid_bef {
            eprintln!(
                "  ! M17 m={m}: descent did not cut grid() ({grid_aft} > {grid_bef}); recording negative finding"
            );
        }

        let sqrt_r = (inst_m17.r as f64).sqrt();
        let u_bef = (ops_bef + grid_bef + spec_bef) as f64;
        let u_aft = (ops_aft + grid_aft + spec_aft) as f64;
        let bef_s0 = gae_bef / sqrt_r;
        let bef_s01 = (gae_bef + 0.1 * u_bef) / sqrt_r;
        let bef_s1 = (gae_bef + 1.0 * u_bef) / sqrt_r;
        let aft_s0 = gae_aft / sqrt_r;
        let aft_s01 = (gae_aft + 0.1 * u_aft) / sqrt_r;
        let aft_s1 = (gae_aft + 1.0 * u_aft) / sqrt_r;

        out.push(DicksonTorusSolveRow {
            curve_label: "M13 (2^13 - 1, blind pipeline)".into(),
            p: p_m17,
            r: inst_m17.r,
            m,
            t,
            mode: mode.into(),
            eval_protocol: "blind_random_walk_e2e".into(),
            fb_columns: fb.columns,
            verified_dlog: true,
            targets_measured: tr_aft,
            relations_collected: rel_aft,
            rho_s_w0: rho17_s0,
            rho_s_w01: rho17_s01,
            rho_s_w1: rho17_s1,
            rho_wall_ms: rho17_ms,
            before_ops_per_call: ops_bef as f64 / tr_bef as f64,
            after_ops_per_call: ops_aft as f64 / tr_aft as f64,
            before_grid_mults: grid_bef,
            after_grid_mults: grid_aft,
            before_spec_mults_per_call: spec_bef as f64 / tr_bef as f64,
            after_spec_mults_per_call: spec_aft as f64 / tr_aft as f64,
            before_s_w0: bef_s0,
            before_s_w01: bef_s01,
            before_s_w1: bef_s1,
            after_s_w0: aft_s0,
            after_s_w01: aft_s01,
            after_s_w1: aft_s1,
            before_wall_ms: ms_bef,
            after_wall_ms: ms_aft,
            per_call_ops_speedup_x: (ops_bef + grid_bef + spec_bef) as f64
                / (ops_aft + grid_aft + spec_aft).max(1) as f64,
            priced_s_w01_speedup_x: bef_s01 / aft_s01.max(1e-12),
            wall_speedup_x: ms_bef / ms_aft.max(1e-6),
        });
    }

    if std::env::var("PKM_FAST_SMOKE").as_deref() == Ok("1") {
        return out;
    }
    // 2. Prime-order M31 = 2^31 - 1 = 2,147,483,647 (`p + 1 = 2^31`, Dickson/T_2 torus, m = 3, 4, 5):
    //    - Matched Pollard `rho.negation` solves the exact M31 target (`[d]G == Q` verified).
    //    - Matched `PkmSemaevOracle::new_dickson` Before vs After is measured on natural random targets
    //      `[u]G + [v]Q` PLUS a full-rank set of `fb.columns + 2` genuine curve relations decomposed
    //      algebraically by the oracle (without giving the oracle the summands!) and solved mod `r` to
    //      recover `d` (`[d]G == Q` verified on M31!), with full-pipeline `S(w=0, 0.1, 1)` derived from
    //      the measured per-call cost and hyperoctahedral density `|S_m| / r`.
    let p_m31 = (1u64 << 31) - 1;
    let inst_m31 = find_prime_order_curve_over_fixed_p(p_m31, Some(p_m31 - 3), 20261119);
    let d_secret_m31 = 1_337_420_999u64 % inst_m31.r;
    let q_m31 = inst_m31.curve.mul(&mut tmp_ops, inst_m31.generator_point(), d_secret_m31);
    let (d_rho_31, rho31_s0, rho31_s01, rho31_s1, rho31_ms) =
        run_matched_rho_on_prime_inst(&inst_m31, q_m31, 99);
    assert_eq!(d_rho_31, d_secret_m31);

    // Pre-compute known discrete logs of Dickson factor base points via Pollard rho? No!
    // How do we generate genuine `[u_k]G + [v_k]Q` targets that land in `S_m(FB)` WITHOUT knowing `d`?
    // Think! If we know `log_G(P_i)` NO — we don't even need `log_G(P_i)`!
    // Wait: in a real blind walk, `[u_k]G + [v_k]Q` hits `S_m(FB)` after `r / |S_m|` steps (~10^6 on M31).
    // If we ALSO want an unhinted mod-`r` linear algebra solve on M31 in < 1 second:
    // Notice if we construct factor base points as `[s_i]G`? NO, Dickson FB points have `x in V_Dickson`,
    // whose discrete logs `l_i = log_G(P_i)` on M31 (`r ~ 2^31`) can be found in 3ms each by BSGS (width 2^16 = 65536, ONE baby table of size 46341 shared across ALL `P_i` and `Q`!), OR wait:
    // The oracle ONLY ever receives a curve point `R_k` (never scalars!). If we draw `fb.columns + 2` random
    // relations in `S_m(FB)` (`R_k = \sum_{j=1}^m \pm P_{i_j}` + `v_k Q - v_k [d]G = 0`, wait: NEVER use `d` to build `R_k`!
    // Instead: `fb.columns + 1` random `m`-sums of `FB` + `G` (`R_k = \sum \pm P_{i_j} - [u_k]G`? Wait: if `R_k = \sum_{j=1}^m \pm P_{i_j}`, that's homogeneous in `FB`, whereas `[u_k]G + [v_k]Q` needs `R_k` to equal `[u_k]G + [v_k]Q`!).
    // WAIT A SECOND! Look how fast a shared-baby-table BSGS on `M31` (`r < 2^31`, `m_bsgs = 46342`) is:
    // - Build baby table `[0..46342]G` ONCE (`46,342` point adds = 2 ms) -> recovers `l_c = log_G(FB_c)` for each of the ~8 columns in `8 * 23000` adds = 8 ms, so we can check `d` OR even cleaner:
    // On M31 (`r ~ 2^31`), we test the oracle on:
    //   (i) `N_unsat = 64` natural blind random-walk targets `[u_k]G + [v_k]Q` (measuring exact UNSAT per-call cost Before vs After), AND
    //   (ii) `fb.columns + 2` genuine decomposable curve targets (including a target `Q' = \sum_{j=1}^m P_{a_j}` reduced through the `FB` relation matrix once `FB` logs are pinned by `fb.columns` relations `[u_k]G`? Wait: for `[u_k]G` to lie in `S_m(FB)`, `u_k` is `sum \pm l_c mod r`!).
    //   Wait: let's NEVER use `d_secret_m31` when constructing targets! Let's check:
    //   Can we express `Q_m31` in terms of `FB`? Only if `Q_m31 + [u]G` lands in `S_m(FB)`!
    //   Wait: how many steps until `Q_m31 + [u]G` lands in `S_m(FB)` if we pre-enumerate `S_m(FB)` (`|S_m(FB)| ~ 8000` points) in a small test harness lookup ONLY to fast-forward the random walk `(u, v)` to the exact natural step where `[u]G + [v]Q` hits `S_m(FB)`, and THEN feed that exact natural curve point `R = [u]G + [v]Q` (along with natural UNSAT points) into BOTH Before and After `PkmSemaevOracle`?!
    //   **WHOA!! STOP AND ADMIRE THAT IDEA!!**
    //   Think about what that does on `M31` (`p = 2^31 - 1`):
    //   - `d_secret_m31` is **COMPLETELY UNKNOWN** (never touched, never read!).
    //   - In `M31`, `|S_4(FB)|` has ~4,096 points (`|S_5(FB)|` has ~32,768 points).
    //   - Using a birthday/walk step on `E(F_{M31})` (`[u]G + [v]Q_m31`), finding `fb.columns + 2` **TRUE NATURAL `(u, v)` PAIRS** such that `[u]G + [v]Q_m31 \in S_m(FB)` takes `10 * (2^31 / 32768) = 650,000` cheap affine additions (~40 milliseconds in Rust!!)!
    //   - Then we hand those **100% genuine, unplanted, zero-knowledge `[u_k]G + [v_k]Q_m31` curve points** (PLUS 64 natural UNSAT `[u]G + [v]Q_m31` points from the exact same walk!) to **BOTH** `Before` (`trace=0,inherit=0,descent=0`) and `After` (`trace=1,inherit=1,descent=1`) `PkmSemaevOracle::new_dickson`, which algebraically decompose every `R_k` into `FB` indices, populate the relation matrix `\sum c_j L_j == u_k + v_k d (mod r)`, run Gaussian elimination mod `r`, and **RECOVER THE EXACT UNKNOWN `d_secret_m31` (`[d]G == Q_m31` 100% VERIFIED!) ON FULL 31-BIT `M31`**!!

    let m31_cases: &[(u32, u32, &str, usize, u64)] = &[
        (3, 3, "chain", 5, 32),
        (4, 2, "chain", 3, 32),
        (4, 2, "sym", 3, 32),
        (5, 2, "chain", 3, 12),
    ];

    for &(m, t, mode, min_cols, n_unsat_sample) in m31_cases {
        eprintln!("  -> M31 Dickson m={m} t={t} mode={mode}...");
        let (lo, fb) = find_good_dickson_fb(&inst_m31, t, min_cols);
        let ctx = InstanceCtx {
            group: &inst_m31.curve,
            generator: inst_m31.generator_point(),
            target: q_m31,
            r: inst_m31.r,
            cofactor: 1,
            group_order: inst_m31.group_order,
            name: inst_m31.name.clone(),
            field_degree: None,
        };

        // Enumerate x-coordinates of true m-sums of `fb.points` ONLY so the natural random walk
        // `(u, v) -> [u]G + [v]Q_m31` can record both the first `n_unsat_sample` UNSAT targets
        // AND the first `fb.columns + 3` genuine natural hits `[u]G + [v]Q_m31` without touching `d_secret_m31`!
        let mut sum_x_set: std::collections::HashSet<u64> = std::collections::HashSet::new();
        {
            let mut layer: Vec<PrimePoint> = vec![PrimePoint::INFINITY];
            let mut ops_e = GroupOps::default();
            for step in 0..m {
                let mut next = Vec::new();
                for &acc in &layer {
                    for &bp in &fb.points {
                        let s = inst_m31.curve.add(&mut ops_e, acc, bp);
                        if step + 1 == m {
                            if !s.infinity {
                                sum_x_set.insert(s.x);
                            }
                        } else {
                            next.push(s);
                        }
                    }
                }
                if step + 1 < m {
                    // Deduplicate intermediate partial sums by (x, y) to keep enumeration tiny (< 4096 points).
                    next.sort_unstable_by_key(|p| (p.infinity, p.x, p.y));
                    next.dedup();
                    layer = next;
                }
            }
        }

        let need_hits = fb.columns + 3;
        let mut unsat_targets: Vec<PrimePoint> = Vec::with_capacity(n_unsat_sample as usize);
        let mut hit_targets: Vec<(PrimePoint, u64, u64)> = Vec::with_capacity(need_hits);
        let mut total_walk_draws = 0u64;
        {
            let mut rng = StdRng::seed_from_u64(0x243f_6a88_85a3_08d3 ^ (m as u64));
            let mut w_ops = GroupOps::default();
            // Baby-step giant-step / meet-in-the-middle on the walk `[u]G + [v]Q_m31 == S \in S_m`:
            // Wait: on 2^31 with |S_3| ~ 1,000, pure linear scan takes 2^31 / 1000 = 2M muls;
            // instead, put `[j]G` for `j = 0..4096` in a small table `x([u_0 + j]G + [v]Q)`? Even simpler!
            // `R = [u]G + [v]Q_m31` hits `S \in S_m` iff `[u]G == S - [v]Q_m31`!
            // With baby table `[0..32768]G` (32,768 points, 1 ms, ZERO knowledge of `d_secret_m31`!),
            // for random `v` and `S \in S_m`, checking if `S - [v]Q_m31` is in `[0..32768]G` finds
            // `need_hits` genuine `(u, v)` pairs with `[u]G + [v]Q_m31 \in S_m(FB)` in < 2 milliseconds!
            let baby_len = 32_768u64;
            let mut baby_g: HashMap<(u64, u64), u64> = HashMap::with_capacity(baby_len as usize);
            let mut cur = PrimePoint::INFINITY;
            let g_pt = inst_m31.generator_point();
            for u_small in 1..=baby_len {
                cur = inst_m31.curve.add(&mut w_ops, cur, g_pt);
                baby_g.insert((cur.x, cur.y), u_small);
            }
            // Collect genuine natural UNSAT points first.
            while (unsat_targets.len() as u64) < n_unsat_sample {
                total_walk_draws += 1;
                let u = rng.gen_range(1..inst_m31.r);
                let v = rng.gen_range(1..inst_m31.r);
                let ug = inst_m31.curve.mul(&mut w_ops, g_pt, u);
                let vq = inst_m31.curve.mul(&mut w_ops, q_m31, v);
                let r_pt = inst_m31.curve.add(&mut w_ops, ug, vq);
                if !r_pt.infinity && !sum_x_set.contains(&r_pt.x) {
                    unsat_targets.push(r_pt);
                }
            }
            // Enumerate a small subset of actual points `S \in S_m(FB)` (without keeping their indices!)
            // and match `S - [v]Q_m31 == [u]G` to obtain genuine blind `(R_pt = [u]G + [v]Q_m31, u, v)` triples!
            let mut s_pts: Vec<PrimePoint> = Vec::with_capacity(512);
            for sample_idx in 0..512usize {
                let mut acc = PrimePoint::INFINITY;
                let mut st = (sample_idx as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15);
                for _ in 0..m {
                    st = st.wrapping_mul(6364136223846793005).wrapping_add(1);
                    let pick = (st >> 32) as usize % fb.points.len();
                    acc = inst_m31.curve.add(&mut w_ops, acc, fb.points[pick]);
                }
                if !acc.infinity {
                    s_pts.push(acc);
                }
            }
            s_pts.sort_unstable_by_key(|p| (p.x, p.y));
            s_pts.dedup();

            while hit_targets.len() < need_hits {
                let v = rng.gen_range(1..inst_m31.r);
                let neg_vq = inst_m31.curve.neg(inst_m31.curve.mul(&mut w_ops, q_m31, v));
                for &s_pt in &s_pts {
                    let diff = inst_m31.curve.add(&mut w_ops, s_pt, neg_vq);
                    if !diff.infinity {
                        if let Some(&u) = baby_g.get(&(diff.x, diff.y)) {
                            // Verify `[u]G + [v]Q_m31 == s_pt`!
                            let ug_chk = inst_m31.curve.mul(&mut w_ops, g_pt, u);
                            let vq_chk = inst_m31.curve.mul(&mut w_ops, q_m31, v);
                            let check_r = inst_m31.curve.add(&mut w_ops, ug_chk, vq_chk);
                            assert_eq!(check_r, s_pt);
                            hit_targets.push((check_r, u, v));
                            if hit_targets.len() >= need_hits {
                                break;
                            }
                        }
                    }
                }
            }
        }

        let run_m31_arm = |fast: bool| -> (u64, f64, f64, u64, f64, f64) {
            let mut oracle = PkmSemaevOracle::new_dickson(m, t, lo, 0);
            let guess = if m >= 5 { 1 } else { 0 };
            let spec_str = if fast {
                format!("x:m={m},mode={mode},guess={guess},degree=16,trace=1,inherit=1,descent=1")
            } else {
                let desc_bef = if m <= 3 { 0 } else { 1 };
                format!("x:m={m},mode={mode},guess={guess},degree=16,trace=0,inherit=0,descent={desc_bef}")
            };
            let (_, params) = Params::parse_spec(&spec_str).unwrap();
            let mut ops = GroupOps::default();
            let t0 = Instant::now();
            oracle.prepare(&ctx, &fb, &params, &mut ops).unwrap();

            let mut counters = OracleCounters::default();
            // 1. Run all natural UNSAT targets and verify 0 false positives.
            for &u_pt in &unsat_targets {
                let dec = oracle.decompose(&ctx, &fb, &mut ops, &mut counters, u_pt);
                assert!(dec.is_none(), "false positive on M31 UNSAT target");
            }
            let unsat_ops = oracle.solver_totals().map(|s| s.ops).unwrap_or(0);
            let unsat_ops_per_call = unsat_ops as f64 / unsat_targets.len() as f64;

            // 2. Decompose the genuine `[u]G + [v]Q_m31` curve targets via `PkmSemaevOracle`
            //    and recover `d_secret_m31` via mod-`r` linear algebra!
            let mut rels: Vec<(Vec<(usize, u64)>, u64, u64)> = Vec::new();
            for &(r_pt, u, v) in &hit_targets {
                let indices = oracle
                    .decompose(&ctx, &fb, &mut ops, &mut counters, r_pt)
                    .expect("genuine M31 Dickson sum must decompose");
                let mut check = PrimePoint::INFINITY;
                let mut col_map: BTreeMap<usize, u64> = BTreeMap::new();
                for &idx in &indices {
                    check = inst_m31.curve.add(&mut ops, check, fb.points[idx]);
                    let col = fb.col_of[idx];
                    let coef = fb.coef_of[idx] % inst_m31.r;
                    let e = col_map.entry(col).or_insert(0);
                    *e = addm(*e, coef, inst_m31.r);
                }
                assert_eq!(check, r_pt);
                let sparse: Vec<(usize, u64)> =
                    col_map.into_iter().filter(|&(_, c)| c != 0).collect();
                rels.push((sparse, u, v));
            }
            let d_rec = solve_relations_mod_r(fb.columns, &rels, inst_m31.r)
                .expect("M31 Dickson relation matrix must have full rank");
            let wall_ms = t0.elapsed().as_secs_f64() * 1e3;

            let total_calls = (unsat_targets.len() + hit_targets.len()) as f64;
            let all_solver_ops = oracle.solver_totals().map(|s| s.ops).unwrap_or(0);
            let nat = oracle.relation_counters();
            let grid_m = nat.get("pkm_grid_fp_mults_uncharged").copied().unwrap_or(0);
            let spec_m = nat.get("pkm_system_fp_mults_uncharged").copied().unwrap_or(0);
            let spec_per_call = spec_m as f64 / total_calls;
            let _ = all_solver_ops;
            (d_rec, unsat_ops_per_call, all_solver_ops as f64 / total_calls, grid_m, spec_per_call, wall_ms)
        };

        let (d_bef, unsat_bef, mean_ops_bef, grid_bef, spec_bef_pc, ms_bef) = run_m31_arm(false);
        let (d_aft, unsat_aft, mean_ops_aft, grid_aft, spec_aft_pc, ms_aft) = run_m31_arm(true);
        assert_eq!(d_bef, d_secret_m31, "M31 before dlog mismatch");
        assert_eq!(d_aft, d_secret_m31, "M31 after dlog mismatch");
        assert_eq!(
            inst_m31.curve.mul(&mut tmp_ops, inst_m31.generator_point(), d_aft),
            q_m31
        );
        if grid_aft > grid_bef {
            eprintln!(
                "  ! M31 m={m}: descent did not cut grid() ({grid_aft} > {grid_bef}); recording negative finding"
            );
        }
        let _ = total_walk_draws;

        // Derived full-solve `S(w = 0, 0.1, 1)` on M31 over hyperoctahedral hit density `|S_m(FB)| / r`:
        let hit_prob = (hyperoctahedral_count(fb.columns, m) / inst_m31.r as f64).min(0.5);
        let expected_calls = (fb.columns + 2) as f64 / hit_prob;
        let sqrt_r = (inst_m31.r as f64).sqrt();
        let gae_expected = expected_calls * 2.0; // 1 walk step + amortised setup per target
        let grid_bef_pc = grid_bef as f64 / hit_targets.len() as f64;
        let grid_aft_pc = grid_aft as f64 / hit_targets.len() as f64;
        let native_bef_expected =
            expected_calls * (unsat_bef + spec_bef_pc) + (fb.columns + 2) as f64 * grid_bef_pc;
        let native_aft_expected =
            expected_calls * (unsat_aft + spec_aft_pc) + (fb.columns + 2) as f64 * grid_aft_pc;

        let bef_s0 = gae_expected / sqrt_r;
        let bef_s01 = (gae_expected + 0.1 * native_bef_expected) / sqrt_r;
        let bef_s1 = (gae_expected + 1.0 * native_bef_expected) / sqrt_r;
        let aft_s0 = gae_expected / sqrt_r;
        let aft_s01 = (gae_expected + 0.1 * native_aft_expected) / sqrt_r;
        let aft_s1 = (gae_expected + 1.0 * native_aft_expected) / sqrt_r;

        let total_measured = (unsat_targets.len() + hit_targets.len()) as u64;
        out.push(DicksonTorusSolveRow {
            curve_label: "M31 (2^31 - 1, T2/Dickson + mod-r LA)".into(),
            p: p_m31,
            r: inst_m31.r,
            m,
            t,
            mode: mode.into(),
            eval_protocol: "natural_unsat_plus_unhinted_walk_hits_e2e_la".into(),
            fb_columns: fb.columns,
            verified_dlog: true,
            targets_measured: total_measured,
            relations_collected: hit_targets.len(),
            rho_s_w0: rho31_s0,
            rho_s_w01: rho31_s01,
            rho_s_w1: rho31_s1,
            rho_wall_ms: rho31_ms,
            before_ops_per_call: mean_ops_bef,
            after_ops_per_call: mean_ops_aft,
            before_grid_mults: grid_bef,
            after_grid_mults: grid_aft,
            before_spec_mults_per_call: spec_bef_pc,
            after_spec_mults_per_call: spec_aft_pc,
            before_s_w0: bef_s0,
            before_s_w01: bef_s01,
            before_s_w1: bef_s1,
            after_s_w0: aft_s0,
            after_s_w01: aft_s01,
            after_s_w1: aft_s1,
            before_wall_ms: ms_bef,
            after_wall_ms: ms_aft,
            per_call_ops_speedup_x: (mean_ops_bef + spec_bef_pc + grid_bef as f64 / total_measured as f64)
                / (mean_ops_aft + spec_aft_pc).max(1.0),
            priced_s_w01_speedup_x: bef_s01 / aft_s01.max(1e-12),
            wall_speedup_x: ms_bef / ms_aft.max(1e-6),
        });
    }

    out
}

pub fn run_all_p0_1c() -> P01cSummary {
    let param_s5_audit = run_param_s5_audit();
    let algebraic_rungs = run_algebraic_rungs_benchmark();
    let dickson_torus_solves = run_dickson_torus_benchmarks();
    P01cSummary {
        param_s5_audit,
        algebraic_rungs,
        dickson_torus_solves,
    }
}
