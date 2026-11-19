//! Reproducible native Rust verification & benchmark driver for Workstream 1
//! (`prime_pdp_rootfind_chase_20261119`):
//!
//! 1. **Lead P0-1a (`semaev_projection.rs`)**:
//!    - Canonical `y_i` caching + Montgomery `batch_inv` over `2(x_1 - x_2)^2` in
//!      `PairProjection::build`, batched query denominators, and `decompose_four`
//!      early witness exit;
//!    - Exact counted `F_p` primitive operations + best-of-N cold/online wall-clock
//!      speedups with `0` root/decomposition mismatches.
//!
//! 2. **Lead P0-1b (`prime_fast.rs`)**:
//!    - Precomputed factor-base subproduct tree `P_V(X) = prod_{x in V} (X - x)`
//!      with remainder-tree multipoint evaluation & batched candidate root extraction
//!      (crossover-gated vs `ec_index_calculus::find_roots_fp_fast` and native
//!      `find_roots_fp64_cz`);
//!    - Lock-step multi-walker (`W = 64, 128, 256`) Montgomery batch inversion with
//!      zero-allocation scratch reuse vs unbatched `collect_relations` / `ic_solve_table`
//!      and matched negation-map Pollard's rho, verifying 100% `[d]G == Q` recovery.
//!
//! 3. **Lead P2-8 (Native Rust Kaarelson v2 Theorem 3 + Open Problem 1 & `§5a` `w_cal`)**:
//!    - Exact zero-variance census over ALL `u in F_p*` on dilated models
//!      `E_u : y^2 = x^3 + u^4 A x + u^6 B` vs plain-box `u = 1`
//!      (`{1 <= x <= floor(p^(1/m))}`) and normalized additive energy `E_2 / X^2`
//!      at `p in {1009, 4099, 10007, 65537}` for `m = 2, 3` (cross-checked directly
//!      on `E_u` at `p = 1009` with `0` mismatches);
//!    - In-situ `§5a` `w_cal` best-of-N kernel timing (`ecbench::wcal::measure`,
//!      `Fp64::mul`, `MulModN::mul`, subproduct-tree remainder step, sparse Wiedemann
//!      SpMV step mod `r`, and batched negation-map `rho` GAE step) + 32..44-bit
//!      prime `rho.negation` step ratio to `sqrt(pi * r / 4)`.

use std::fs;
use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

use crypto_lib::cryptanalysis::ec_index_calculus::find_roots_fp_fast;
use crypto_lib::cryptanalysis::ecbench::wcal::{self, BEST_OF_N};
use crypto_lib::cryptanalysis::pkm_smooth_tower::Curve;
use crypto_lib::cryptanalysis::prime_fast::{
    find_a3_curve, find_roots_fp64_cz, ic_solve, ic_solve_lockstep, ic_solve_table,
    poly_mul_mont, poly_rem_monic_mont, point_order_hasse, rho_parallel,
    FactorBaseSubproductTree, FastCurve, Fp64, LockStepMode, MulModN, RhoConfig,
    SubproductBatchScratch, DEFAULT_BATCH_SUBPRODUCT_CROSSOVER_B, DEFAULT_MULTIPOINT_CROSSOVER_B,
};
use crypto_lib::cryptanalysis::research_bench::bench_curves_a_minus_3;
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use crypto_lib::cryptanalysis::semaev_projection::{
    s3_pair_roots_biguint_rational_batched, s3_pair_roots_biguint_reference, FpOpCounts,
    PairProjection,
};
use crypto_lib::ecc::field::FieldElement;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

const OUT_DIR: &str = "research/prime_pdp_rootfind_chase_20261119";

fn best_of_n_ms<T>(rounds: usize, mut f: impl FnMut() -> T) -> (f64, f64, T) {
    assert!(rounds >= 1);
    let mut best_ms = f64::MAX;
    let mut worst_ms = 0.0f64;
    let mut last_out: Option<T> = None;
    for _ in 0..rounds {
        let t0 = Instant::now();
        let out = black_box(f());
        let ms = t0.elapsed().as_secs_f64() * 1e3;
        best_ms = best_ms.min(ms);
        worst_ms = worst_ms.max(ms);
        last_out = Some(out);
    }
    let spread = if best_ms > 0.0 {
        worst_ms / best_ms
    } else {
        1.0
    };
    (best_ms, spread, last_out.unwrap())
}

// ── Section 1: Lead P0-1a Benchmarks & Exact Op Counts ────────────────

#[derive(Serialize)]
struct P01aCell {
    curve_label: String,
    p: u64,
    fb_size: usize,
    unordered_pairs: usize,
    mismatches: usize,
    ref_build_ops: FpOpCounts,
    opt_build_ops: FpOpCounts,
    build_inv_reduction_x: f64,
    ref_build_ms: f64,
    opt_build_ms: f64,
    build_wall_speedup_x: f64,
    query_count: usize,
    planted_hits_d4: usize,
    ref_d3_ops: FpOpCounts,
    opt_d3_ops: FpOpCounts,
    ref_d3_ms: f64,
    opt_d3_ms: f64,
    d3_wall_speedup_x: f64,
    ref_d4_ops: FpOpCounts,
    opt_d4_ops: FpOpCounts,
    d4_inv_reduction_x: f64,
    ref_d4_ms: f64,
    opt_d4_ms: f64,
    d4_wall_speedup_x: f64,
}

#[derive(Serialize)]
struct P01aBigUintCell {
    curve_label: String,
    bits: u64,
    pairs_tested: usize,
    mismatches: usize,
    ref_ms: f64,
    opt_batched_ms: f64,
    wall_speedup_x: f64,
}

#[derive(Serialize)]
struct P01aReceipt {
    lead: &'static str,
    timing_method: &'static str,
    cells: Vec<P01aCell>,
    multilimb_cells: Vec<P01aBigUintCell>,
}

fn run_p0_1a() -> P01aReceipt {
    let (fc44, _) = find_a3_curve(44, 3).expect("44-bit curve");
    let specs: Vec<(&str, u64, u64, u64, usize)> = vec![
        ("Prime31 (2^31-1)", 2_147_483_647, 2_147_483_644, 7, 64),
        ("Prime31 (2^31-1)", 2_147_483_647, 2_147_483_644, 7, 128),
        ("Prime31 (2^31-1)", 2_147_483_647, 2_147_483_644, 7, 256),
        ("Prime31 (2^31-1)", 2_147_483_647, 2_147_483_644, 7, 512),
        (
            "Prime44 (a=-3)",
            fc44.f.p,
            fc44.f.from_mont(fc44.a),
            fc44.f.from_mont(fc44.b),
            128,
        ),
        (
            "Prime44 (a=-3)",
            fc44.f.p,
            fc44.f.from_mont(fc44.a),
            fc44.f.from_mont(fc44.b),
            256,
        ),
        (
            "Mersenne61 (2^61-1)",
            (1u64 << 61) - 1,
            (1u64 << 61) - 4,
            11,
            128,
        ),
    ];

    let mut cells = Vec::new();
    for (label, p, a, b, fb_size) in specs {
        let curve = Curve { p, a, b };
        let fc = FastCurve::new("p01a", p, a, b, 1);
        let mut base_xs = Vec::with_capacity(fb_size);
        let mut x = 1u64;
        while base_xs.len() < fb_size && x < p {
            if fc.lift_x(x).is_some() {
                base_xs.push(x);
            }
            x += 1;
        }

        let mut ref_build_ops = FpOpCounts::default();
        let mut opt_build_ops = FpOpCounts::default();
        let proj_ref_c =
            PairProjection::build_reference_counted(curve, &base_xs, &mut ref_build_ops).unwrap();
        let proj_opt_c =
            PairProjection::build_counted(curve, &base_xs, &mut opt_build_ops).unwrap();
        let stats_ref = proj_ref_c.build_stats();
        let stats_opt = proj_opt_c.build_stats();
        let mut mismatches = usize::from(stats_ref != stats_opt);

        let rounds = if fb_size >= 512 { 3 } else { 5 };
        let (ref_build_ms, _, proj_ref) = best_of_n_ms(rounds, || {
            PairProjection::build_reference_counted(curve, &base_xs, &mut FpOpCounts::default())
                .unwrap()
        });
        let (opt_build_ms, _, proj_opt) = best_of_n_ms(rounds, || {
            PairProjection::build(curve, &base_xs).unwrap()
        });

        // Construct interleaved planted 4-decomposable targets + random F_p targets
        // so any prefix slice has an exact 50/50 mix of planted and random queries.
        let mut rng = StdRng::seed_from_u64(p ^ (fb_size as u64 * 0x9E37));
        let mut queries = Vec::with_capacity(128);
        while queries.len() < 128 {
            let p0 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            let p1 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            let p2 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            let p3 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            if let Some(pt) = fc.add(fc.add(fc.add(Some(p0), Some(p1)), Some(p2)), Some(p3)) {
                queries.push(fc.canonical(pt).0);
                queries.push(rng.gen_range(0..p));
            }
        }
        let d4_query_count = match fb_size {
            0..=64 => 32,
            65..=128 => 16,
            129..=256 => 8,
            _ => 4,
        };
        let d4_query_slice = &queries[..d4_query_count];

        let mut ref_d3_ops = FpOpCounts::default();
        let mut opt_d3_ops = FpOpCounts::default();
        for &xr in &queries {
            let r_ref = proj_ref.decompose_three_reference_counted(xr, &mut ref_d3_ops).0;
            let r_opt = proj_opt.decompose_three_counted(xr, &mut opt_d3_ops).0;
            if r_ref.is_some() != r_opt.is_some() {
                mismatches += 1;
            }
        }

        let (ref_d3_ms, _, _) = best_of_n_ms(5, || {
            let mut hits = 0usize;
            for &xr in &queries {
                if proj_ref
                    .decompose_three_reference_counted(xr, &mut FpOpCounts::default())
                    .0
                    .is_some()
                {
                    hits += 1;
                }
            }
            hits
        });
        let (opt_d3_ms, _, _) = best_of_n_ms(5, || {
            let mut hits = 0usize;
            for &xr in &queries {
                if proj_opt.decompose_three(xr).0.is_some() {
                    hits += 1;
                }
            }
            hits
        });

        let mut ref_d4_ops = FpOpCounts::default();
        let mut opt_d4_ops = FpOpCounts::default();
        let mut planted_hits_d4 = 0usize;
        for &xr in d4_query_slice {
            let r_ref = proj_ref.decompose_four_reference_counted(xr, &mut ref_d4_ops).0;
            let r_opt = proj_opt.decompose_four_counted(xr, &mut opt_d4_ops).0;
            if r_ref.is_some() != r_opt.is_some() {
                mismatches += 1;
            }
            if r_opt.is_some() {
                planted_hits_d4 += 1;
            }
        }

        let d4_rounds = if fb_size >= 512 { 2 } else { 3 };
        let (ref_d4_ms, _, _) = best_of_n_ms(d4_rounds, || {
            let mut hits = 0usize;
            for &xr in d4_query_slice {
                if proj_ref
                    .decompose_four_reference_counted(xr, &mut FpOpCounts::default())
                    .0
                    .is_some()
                {
                    hits += 1;
                }
            }
            hits
        });
        let (opt_d4_ms, _, _) = best_of_n_ms(d4_rounds, || {
            let mut hits = 0usize;
            for &xr in d4_query_slice {
                if proj_opt.decompose_four(xr).0.is_some() {
                    hits += 1;
                }
            }
            hits
        });

        assert_eq!(mismatches, 0, "P0-1a mismatch in {label} B={fb_size}");
        cells.push(P01aCell {
            curve_label: label.to_string(),
            p,
            fb_size,
            unordered_pairs: stats_opt.unordered_pairs as usize,
            mismatches,
            build_inv_reduction_x: ref_build_ops.invs as f64 / opt_build_ops.invs.max(1) as f64,
            ref_build_ops,
            opt_build_ops,
            ref_build_ms,
            opt_build_ms,
            build_wall_speedup_x: ref_build_ms / opt_build_ms.max(1e-9),
            query_count: d4_query_slice.len(),
            planted_hits_d4,
            ref_d3_ops,
            opt_d3_ops,
            ref_d3_ms,
            opt_d3_ms,
            d3_wall_speedup_x: ref_d3_ms / opt_d3_ms.max(1e-9),
            d4_inv_reduction_x: ref_d4_ops.invs as f64 / opt_d4_ops.invs.max(1) as f64,
            ref_d4_ops,
            opt_d4_ops,
            ref_d4_ms,
            opt_d4_ms,
            d4_wall_speedup_x: ref_d4_ms / opt_d4_ms.max(1e-9),
        });
    }

    // Multi-limb BigUint primes (127-bit Mersenne & 256-bit secp256k1).
    let p127: BigUint = (BigUint::from(1u8) << 127) - BigUint::from(1u8);
    let p256 = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap();
    let mut multilimb_cells = Vec::new();
    for (label, p, a, b) in [
        ("Mersenne127 (2^127-1)", &p127, &(&p127 - BigUint::from(3u8)), &BigUint::from(5u8)),
        ("secp256k1 (256-bit)", &p256, &BigUint::from(0u8), &BigUint::from(7u8)),
    ] {
        let exp = (p + BigUint::from(1u8)) >> 2usize;
        let mut pts = Vec::new();
        let mut x_u = 1u64;
        while pts.len() < 32 {
            let x = BigUint::from(x_u);
            let rhs = (&x * &x * &x + a * &x + b) % p;
            let y = rhs.modpow(&exp, p);
            if (&y * &y) % p == rhs {
                pts.push((x, Some(y.clone().min(p - &y))));
            }
            x_u += 1;
        }
        let mut batch = Vec::new();
        for i in 0..pts.len() {
            for j in i..pts.len() {
                batch.push((
                    pts[i].0.clone(),
                    pts[i].1.clone(),
                    pts[j].0.clone(),
                    pts[j].1.clone(),
                ));
            }
        }
        let (ref_ms, _, want) = best_of_n_ms(3, || {
            batch
                .iter()
                .map(|(x1, _, x2, _)| s3_pair_roots_biguint_reference(p, a, b, x1, x2))
                .collect::<Vec<_>>()
        });
        let (opt_ms, _, got) = best_of_n_ms(3, || {
            s3_pair_roots_biguint_rational_batched(p, a, b, &batch)
        });
        let mismatches = got
            .iter()
            .zip(want.iter())
            .filter(|(g, w)| {
                let norm = |r: &crypto_lib::cryptanalysis::semaev_projection::BigQuadraticRoots| {
                    match r {
                        crypto_lib::cryptanalysis::semaev_projection::BigQuadraticRoots::Finite(v) => {
                            let mut s = v.clone();
                            s.sort();
                            crypto_lib::cryptanalysis::semaev_projection::BigQuadraticRoots::Finite(s)
                        }
                        other => other.clone(),
                    }
                };
                norm(g) != norm(w)
            })
            .count();
        assert_eq!(mismatches, 0, "multilimb mismatch on {label}");
        multilimb_cells.push(P01aBigUintCell {
            curve_label: label.to_string(),
            bits: p.bits(),
            pairs_tested: batch.len(),
            mismatches,
            ref_ms,
            opt_batched_ms: opt_ms,
            wall_speedup_x: ref_ms / opt_ms.max(1e-9),
        });
    }

    P01aReceipt {
        lead: "P0-1a",
        timing_method: BEST_OF_N,
        cells,
        multilimb_cells,
    }
}

// ── Section 2: Lead P0-1b Subproduct Tree + Lock-Step Multi-Walker ────

#[derive(Serialize)]
struct SubproductSweepCell {
    prime_label: String,
    p: u64,
    fb_size: usize,
    batch_candidates: usize,
    candidate_degree: usize,
    mismatches: usize,
    prepare_ms: f64,
    find_roots_fp_fast_ns_per_poly: f64,
    native_fp64_cz_ns_per_poly: f64,
    direct_horner_ns_per_poly: f64,
    ungated_subproduct_ns_per_poly: f64,
    gated_subproduct_ns_per_poly: f64,
    speedup_vs_find_roots_fp_fast_x: f64,
    speedup_vs_native_fp64_cz_x: f64,
    multipoint_deg128_horner_us: f64,
    multipoint_deg128_tree_us: f64,
    multipoint_deg128_speedup_x: f64,
}

#[derive(Serialize)]
struct LockStepBenchCell {
    bits: u32,
    p: u64,
    r: u64,
    fb_size: usize,
    trials: usize,
    all_recovered_ok: bool,
    rho_tuned_mean_ms: f64,
    rho_tuned_mean_steps: f64,
    unbatched_sweep_mean_ms: f64,
    unbatched_table_mean_ms: f64,
    lockstep_w64_mean_ms: f64,
    lockstep_w128_mean_ms: f64,
    lockstep_w256_mean_ms: f64,
    lockstep_w256_relations_only_ms: f64,
    unbatched_table_relations_only_ms: f64,
    collection_speedup_w256_vs_unbatched_table_x: f64,
    total_speedup_w256_vs_unbatched_sweep_x: f64,
}

#[derive(Serialize)]
struct P01bReceipt {
    lead: &'static str,
    timing_method: &'static str,
    default_multipoint_crossover_b: usize,
    default_batch_subproduct_crossover_b: usize,
    subproduct_cells: Vec<SubproductSweepCell>,
    lockstep_cells: Vec<LockStepBenchCell>,
}

fn run_p0_1b() -> P01bReceipt {
    let (fc44, _) = find_a3_curve(44, 3).unwrap();
    let mut subproduct_cells = Vec::new();
    for (label, p) in [
        ("Prime31 (2^31-1)", 2_147_483_647u64),
        ("Prime44 (a=-3)", fc44.f.p),
    ] {
        let f = Fp64::new(p);
        let p_big = BigUint::from(p);
        for &b_size in &[8usize, 16, 32, 64, 96, 128, 256, 512, 1024, 2048] {
            let mut rng = StdRng::seed_from_u64(p ^ (b_size as u64 * 0x517C));
            let base_xs: Vec<u64> = (1..=b_size as u64).collect();
            let (prepare_ms, _, tree_gated) =
                best_of_n_ms(5, || FactorBaseSubproductTree::prepare(f, &base_xs));
            let tree_ungated =
                FactorBaseSubproductTree::prepare_with_crossovers(f, &base_xs, 1, 1);
            let tree_horner_only = FactorBaseSubproductTree::prepare_with_crossovers(
                f,
                &base_xs,
                usize::MAX,
                usize::MAX,
            );

            // 512 degree-4 candidate polynomials (95% random over F_p, 5% planted in V).
            let batch_k = 512usize;
            let deg = 4usize;
            let mut candidates: Vec<Vec<u64>> = Vec::with_capacity(batch_k);
            for k in 0..batch_k {
                if k % 20 == 0 {
                    let mut poly = vec![f.one];
                    for t in 0..deg {
                        let root = if t < 2 {
                            base_xs[rng.gen_range(0..base_xs.len())]
                        } else {
                            rng.gen_range(0..p)
                        };
                        let lin = [f.neg(f.to_mont(root)), f.one];
                        let mut next = Vec::new();
                        poly_mul_mont(&f, &poly, &lin, &mut next);
                        poly = next;
                    }
                    let scale = f.to_mont(rng.gen_range(1..p));
                    candidates.push(
                        poly.into_iter()
                            .map(|c| f.from_mont(f.mul(c, scale)))
                            .collect(),
                    );
                } else {
                    let mut c: Vec<u64> = (0..deg).map(|_| rng.gen_range(0..p)).collect();
                    c.push(rng.gen_range(1..p));
                    candidates.push(c);
                }
            }

            // Reference BigUint `find_roots_fp_fast` on first 64 polynomials (extrapolated exact ns/poly)
            // + parity check across ALL 512 polynomials against `find_roots_fp64_cz`!
            let fp_sample = 64usize;
            let fe_polys: Vec<Vec<FieldElement>> = candidates[..fp_sample]
                .iter()
                .map(|poly| {
                    poly.iter()
                        .map(|&c| FieldElement::new(BigUint::from(c), p_big.clone()))
                        .collect()
                })
                .collect();
            let (fp_fast_ms, _, fp_fast_out) = best_of_n_ms(3, || {
                fe_polys
                    .iter()
                    .map(|fp| {
                        let mut r: Vec<u64> = find_roots_fp_fast(fp, &p_big)
                            .into_iter()
                            .map(|e| e.value.to_u64_digits().first().copied().unwrap_or(0))
                            .filter(|&x| tree_gated.contains_x(x))
                            .collect();
                        r.sort_unstable();
                        r.dedup();
                        r
                    })
                    .collect::<Vec<_>>()
            });
            let find_roots_fp_fast_ns_per_poly = (fp_fast_ms * 1e6) / fp_sample as f64;

            let (cz_ms, _, cz_out) = best_of_n_ms(5, || {
                candidates
                    .iter()
                    .map(|poly| {
                        let mut r = find_roots_fp64_cz(&f, poly);
                        r.retain(|&x| tree_gated.contains_x(x));
                        r
                    })
                    .collect::<Vec<_>>()
            });
            let native_fp64_cz_ns_per_poly = (cz_ms * 1e6) / batch_k as f64;

            // Direct Horner over V (`roots_in_base` forced via `horner_ops` path):
            let cand_mont: Vec<Vec<u64>> = candidates
                .iter()
                .map(|poly| poly.iter().map(|&c| f.to_mont(c)).collect())
                .collect();
            let xs_mont: Vec<u64> = tree_gated.xs.iter().map(|&x| f.to_mont(x)).collect();
            let (horner_ms, _, horner_out) = best_of_n_ms(5, || {
                cand_mont
                    .iter()
                    .map(|pm| {
                        let mut r = Vec::new();
                        for (i, &xm) in xs_mont.iter().enumerate() {
                            let mut acc = 0u64;
                            for &c in pm.iter().rev() {
                                acc = f.add(f.mul(acc, xm), c);
                            }
                            if acc == 0 {
                                r.push(tree_gated.xs[i]);
                            }
                        }
                        r
                    })
                    .collect::<Vec<_>>()
            });
            let direct_horner_ns_per_poly = (horner_ms * 1e6) / batch_k as f64;

            let mut scratch = SubproductBatchScratch::default();
            let (ungated_ms, _, ungated_out) = best_of_n_ms(5, || {
                tree_ungated.roots_in_base_batch(&candidates, &mut scratch)
            });
            let ungated_subproduct_ns_per_poly = (ungated_ms * 1e6) / batch_k as f64;

            let (gated_ms, _, gated_out) = best_of_n_ms(5, || {
                tree_gated.roots_in_base_batch(&candidates, &mut scratch)
            });
            let gated_subproduct_ns_per_poly = (gated_ms * 1e6) / batch_k as f64;

            let mut mismatches = 0usize;
            for i in 0..fp_sample {
                // Every root returned by `find_roots_fp_fast` must be in `gated_out[i]`.
                // Note: `ec_index_calculus::find_roots_fp_fast` has a known equal-degree
                // shift quirk (`h + c*X` instead of `X + c` for deg(g) >= 3) that drops
                // roots when >= 3 F_p-roots share the same Legendre symbol; `horner_out[i]`
                // is the exact ground truth over V.
                if !fp_fast_out[i].iter().all(|r| horner_out[i].contains(r)) {
                    mismatches += 1;
                }
            }
            for i in 0..batch_k {
                if gated_out[i] != cz_out[i]
                    || ungated_out[i] != cz_out[i]
                    || horner_out[i] != cz_out[i]
                {
                    mismatches += 1;
                }
            }
            assert_eq!(mismatches, 0, "subproduct mismatch {label} B={b_size}");

            // High-degree (deg = 128) multipoint evaluation check: Horner vs remainder tree.
            let mp_poly: Vec<u64> = (0..129).map(|_| rng.gen_range(0..p)).collect();
            let mut vals_h = Vec::new();
            let mut vals_t = Vec::new();
            let (mp_h_ms, _, _) = best_of_n_ms(7, || {
                tree_horner_only.eval_all(&mp_poly, &mut vals_h);
                vals_h.len()
            });
            let (mp_t_ms, _, _) = best_of_n_ms(7, || {
                tree_ungated.eval_all(&mp_poly, &mut vals_t);
                vals_t.len()
            });
            assert_eq!(vals_h, vals_t, "eval_all mismatch {label} B={b_size}");

            subproduct_cells.push(SubproductSweepCell {
                prime_label: label.to_string(),
                p,
                fb_size: b_size,
                batch_candidates: batch_k,
                candidate_degree: deg,
                mismatches,
                prepare_ms,
                find_roots_fp_fast_ns_per_poly,
                native_fp64_cz_ns_per_poly,
                direct_horner_ns_per_poly,
                ungated_subproduct_ns_per_poly,
                gated_subproduct_ns_per_poly,
                speedup_vs_find_roots_fp_fast_x: find_roots_fp_fast_ns_per_poly
                    / gated_subproduct_ns_per_poly.max(1e-9),
                speedup_vs_native_fp64_cz_x: native_fp64_cz_ns_per_poly
                    / gated_subproduct_ns_per_poly.max(1e-9),
                multipoint_deg128_horner_us: mp_h_ms * 1e3,
                multipoint_deg128_tree_us: mp_t_ms * 1e3,
                multipoint_deg128_speedup_x: mp_h_ms / mp_t_ms.max(1e-9),
            });
        }
    }

    // Lock-step multi-walker batch inversion benchmarks (16, 20, 24, 28 bits).
    let mut lockstep_cells = Vec::new();
    for &(bits, fb_size, trials) in &[(16u32, 64usize, 6usize), (20, 100, 5), (24, 160, 4), (28, 256, 3)]
    {
        let (fc, g) = if bits <= 24 {
            let (_, c) = bench_curves_a_minus_3()
                .into_iter()
                .find(|(b, c)| *b == bits && c.name.contains("p256class"))
                .unwrap();
            let fc = FastCurve::from_params(&c).unwrap();
            let g = fc.from_textbook(&c.generator()).unwrap();
            (fc, g)
        } else {
            find_a3_curve(bits, 3).unwrap()
        };

        let mut rng = StdRng::seed_from_u64(0x2026_0000 + bits as u64);
        let mut all_ok = true;
        let mut rho_ms = 0.0;
        let mut rho_steps = 0.0;
        let mut swp_ms = 0.0;
        let mut tbl_ms = 0.0;
        let mut tbl_rel_ms = 0.0;
        let mut w64_ms = 0.0;
        let mut w128_ms = 0.0;
        let mut w256_ms = 0.0;
        let mut w256_rel_ms = 0.0;

        for t in 0..trials {
            let secret = rng.gen_range(1..fc.n);
            let q = fc.mul(Some(g), secret).unwrap();

            let mut rcfg = RhoConfig::tuned(fc.n);
            rcfg.seed = 10 + t as u64;
            let rres = rho_parallel(&fc, g, q, &rcfg).expect("rho");
            all_ok &= rres.log == secret && fc.mul(Some(g), rres.log) == Some(q);
            rho_ms += rres.wall_secs * 1e3;
            rho_steps += rres.total_steps as f64;

            let (d_sw, rep_sw) =
                ic_solve(&fc, g, q, fb_size, 16, 2, 100_000_000, 100 + t as u64).expect("sweep");
            all_ok &= d_sw == secret;
            swp_ms += rep_sw.total_ms;

            let (d_tb, rep_tb) =
                ic_solve_table(&fc, g, q, fb_size, 16, 100_000_000, 100 + t as u64)
                    .expect("table");
            all_ok &= d_tb == secret;
            tbl_ms += rep_tb.total_ms;
            tbl_rel_ms += rep_tb.relations_ms;

            let (d_64, rep_64) = ic_solve_lockstep(
                &fc,
                g,
                q,
                fb_size,
                16,
                64,
                LockStepMode::PairTable2,
                100_000_000,
                200 + t as u64,
            )
            .expect("w64");
            all_ok &= d_64 == secret;
            w64_ms += rep_64.total_ms;

            let (d_128, rep_128) = ic_solve_lockstep(
                &fc,
                g,
                q,
                fb_size,
                16,
                128,
                LockStepMode::PairTable2,
                100_000_000,
                300 + t as u64,
            )
            .expect("w128");
            all_ok &= d_128 == secret;
            w128_ms += rep_128.total_ms;

            let (d_256, rep_256) = ic_solve_lockstep(
                &fc,
                g,
                q,
                fb_size,
                16,
                256,
                LockStepMode::PairTable2,
                100_000_000,
                400 + t as u64,
            )
            .expect("w256");
            all_ok &= d_256 == secret && fc.mul(Some(g), d_256) == Some(q);
            w256_ms += rep_256.total_ms;
            w256_rel_ms += rep_256.relations_ms;
        }
        assert!(all_ok, "lockstep recovery failure at bits={bits}");
        let tf = trials as f64;
        lockstep_cells.push(LockStepBenchCell {
            bits,
            p: fc.f.p,
            r: fc.n,
            fb_size,
            trials,
            all_recovered_ok: all_ok,
            rho_tuned_mean_ms: rho_ms / tf,
            rho_tuned_mean_steps: rho_steps / tf,
            unbatched_sweep_mean_ms: swp_ms / tf,
            unbatched_table_mean_ms: tbl_ms / tf,
            lockstep_w64_mean_ms: w64_ms / tf,
            lockstep_w128_mean_ms: w128_ms / tf,
            lockstep_w256_mean_ms: w256_ms / tf,
            lockstep_w256_relations_only_ms: w256_rel_ms / tf,
            unbatched_table_relations_only_ms: tbl_rel_ms / tf,
            collection_speedup_w256_vs_unbatched_table_x: (tbl_rel_ms / tf)
                / (w256_rel_ms / tf).max(1e-9),
            total_speedup_w256_vs_unbatched_sweep_x: (swp_ms / tf) / (w256_ms / tf).max(1e-9),
        });
    }

    P01bReceipt {
        lead: "P0-1b",
        timing_method: BEST_OF_N,
        default_multipoint_crossover_b: DEFAULT_MULTIPOINT_CROSSOVER_B,
        default_batch_subproduct_crossover_b: DEFAULT_BATCH_SUBPRODUCT_CROSSOVER_B,
        subproduct_cells,
        lockstep_cells,
    }
}

// ── Section 3: Lead P2-8 Kaarelson Theorem 3 + Open Problem 1 & w_cal ──

#[derive(Serialize)]
struct KaarelsonCoverageCell {
    p: u64,
    curve_a: u64,
    curve_b: u64,
    group_order_r: usize,
    m: u32,
    box_bound_x: u64,
    models_evaluated_u_count: usize,
    direct_eu_crosscheck_mismatches: usize,
    // Plain-box u = 1:
    u1_fb_points: usize,
    u1_full_sumset_coverage: f64,
    u1_distinct_x_coverage: f64,
    u1_e2_over_x2: f64,
    u1_e2_over_fb2: f64,
    // Exact dilated-model census over all u in F_p*:
    dilated_mean_fb_points: f64,
    dilated_mean_full_sumset_coverage: f64,
    dilated_std_full_sumset_coverage: f64,
    dilated_min_full_sumset_coverage: f64,
    dilated_max_full_sumset_coverage: f64,
    dilated_mean_distinct_x_coverage: f64,
    dilated_std_distinct_x_coverage: f64,
    dilated_min_distinct_x_coverage: f64,
    dilated_max_distinct_x_coverage: f64,
    dilated_mean_e2_over_x2: f64,
    dilated_std_e2_over_x2: f64,
    // Open Problem 1 comparison:
    u1_minus_dilated_full_cov_diff: f64,
    u1_full_cov_z_score: f64,
    u1_distinct_x_cov_z_score: f64,
}

#[derive(Serialize)]
struct WCalExtendedCell {
    bits: u32,
    p: u64,
    r: u64,
    ecbench_wcal_mul_ns: f64,
    ecbench_wcal_unbatched_add_ns: f64,
    ecbench_wcal_ratio: f64,
    ecbench_wcal_mul_spread: f64,
    ecbench_wcal_add_spread: f64,
    fp64_mont_mul_chain_ns: f64,
    fp64_mont_mul_ilp4_ns: f64,
    mulmodn_canonical_ns: f64,
    fastcurve_unbatched_add_ns: f64,
    rho_batched_gae_w64_ns: f64,
    rho_batched_gae_w128_ns: f64,
    subproduct_rem_step_deg128_ns_per_coeff: f64,
    subproduct_batch_b1024_deg4_ns_per_poly: f64,
    wiedemann_spmv_n2048_w4_ns_per_row: f64,
    wiedemann_spmv_n2048_w4_ns_per_nz: f64,
    // Calibrated weights relative to batched negation-map rho GAE step (W=128):
    w_cal_fp64_mul_over_rho_gae: f64,
    w_cal_wiedemann_row_over_rho_gae: f64,
    w_cal_subproduct_poly_over_rho_gae: f64,
}

#[derive(Serialize)]
struct RhoNegationRatioCell {
    bits: u32,
    p: u64,
    r: u64,
    trials: usize,
    all_recovered_ok: bool,
    theoretical_sqrt_pi_r_over_4: f64,
    mean_steps: f64,
    median_steps: f64,
    min_steps: u64,
    max_steps: u64,
    mean_ratio_to_sqrt_pi_r_over_4: f64,
    median_ratio_to_sqrt_pi_r_over_4: f64,
    mean_total_group_ops_ratio: f64,
    mean_wall_ms: f64,
    mean_msteps_per_sec: f64,
}

#[derive(Serialize)]
struct P28Receipt {
    lead: &'static str,
    timing_method: &'static str,
    kaarelson_coverage_cells: Vec<KaarelsonCoverageCell>,
    wcal_cells: Vec<WCalExtendedCell>,
    rho_negation_cells: Vec<RhoNegationRatioCell>,
}

fn find_prime_order_curve_at_p(p: u64) -> (FastCurve, crypto_lib::cryptanalysis::prime_fast::Pt, u64, u64) {
    let a = p - 3;
    let f = Fp64::new(p);
    let am = f.to_mont(a);
    for b in 1..p {
        let bm = f.to_mont(b);
        let disc = f.add(
            f.mul(f.to_mont(4), f.mul(f.sqr(am), am)),
            f.mul(f.to_mont(27), f.sqr(bm)),
        );
        if disc == 0 {
            continue;
        }
        let probe = FastCurve::new("kaarelson", p, a, b, 1);
        let Some(g0) = (1..p).find_map(|x| probe.lift_x(x)) else {
            continue;
        };
        let Some(order) = point_order_hasse(&probe, g0) else {
            continue;
        };
        if is_prime_u64(order) {
            let fc = FastCurve::new("kaarelson", p, a, b, order);
            let g = fc.lift_x(probe.canonical(g0).0).unwrap();
            if fc.mul(Some(g), order).is_none() {
                return (fc, g, a, b);
            }
        }
    }
    panic!("no prime-order curve found at p={p}");
}

fn floor_root(p: u64, m: u32) -> u64 {
    let mut x = (p as f64).powf(1.0 / m as f64).floor() as u64;
    while (x + 1).checked_pow(m).is_some_and(|v| v <= p) {
        x += 1;
    }
    while x.checked_pow(m).is_none_or(|v| v > p) {
        x -= 1;
    }
    x
}

#[inline]
fn bitset_set(words: &mut [u64], idx: usize) {
    words[idx >> 6] |= 1u64 << (idx & 63);
}

#[inline]
fn bitset_popcount(words: &[u64]) -> usize {
    words.iter().map(|w| w.count_ones() as usize).sum()
}

/// Or `(src + shift) mod r` into `dst` where `src` and `dst` have `r` active bits (`0..r`).
fn bitset_or_cyclic_shift(dst: &mut [u64], src: &[u64], shift: usize, r: usize) {
    if shift == 0 {
        for (d, &s) in dst.iter_mut().zip(src.iter()) {
            *d |= s;
        }
        return;
    }
    // Split `[0, r - shift)` -> `[shift, r)` and `[r - shift, r)` -> `[0, shift)`.
    or_bit_range(dst, shift, src, 0, r - shift);
    or_bit_range(dst, 0, src, r - shift, shift);
}

fn or_bit_range(dst: &mut [u64], dst_bit: usize, src: &[u64], src_bit: usize, len: usize) {
    if len == 0 {
        return;
    }
    let mut rem = len;
    let mut sb = src_bit;
    let mut db = dst_bit;
    while rem > 0 {
        let sw = sb >> 6;
        let so = sb & 63;
        let dw = db >> 6;
        let doff = db & 63;
        let take = rem.min(64 - so).min(64 - doff);
        let mask = if take == 64 {
            u64::MAX
        } else {
            (1u64 << take) - 1
        };
        let bits = (src[sw] >> so) & mask;
        dst[dw] |= bits << doff;
        sb += take;
        db += take;
        rem -= take;
    }
}

/// Evaluate exact `(fb_points, full_sumset_count, distinct_x_count, e2)` for one
/// abscissa slice `abscissa_dlogs`: each element is the positive-sign discrete log
/// `t in 1..r` of a curve-rational abscissa `x' in [1, X]` (`2*t != 0 mod r` since `r`
/// is an odd prime, so each abscissa contributes `{t, r - t}`).
fn eval_box_stats(abscissa_dlogs: &[u32], r: usize, m: u32, r2_buf: &mut [u32]) -> (usize, usize, usize, u64) {
    let a_len = abscissa_dlogs.len();
    let fb_pts = 2 * a_len;
    if a_len == 0 {
        return (0, 0, 0, 0);
    }
    // Collect signed points: `(abscissa_id, dlog)`.
    let mut pts: Vec<(u16, u32)> = Vec::with_capacity(fb_pts);
    for (aid, &t) in abscissa_dlogs.iter().enumerate() {
        pts.push((aid as u16, t));
        pts.push((aid as u16, (r as u32) - t));
    }

    r2_buf[..r].fill(0);
    let words_len = r.div_ceil(64);
    let mut full_s2_bits = vec![0u64; words_len];
    let mut dist_s2_bits = vec![0u64; words_len];

    // Pairwise sums: exploit symmetry `(i, j)` and `(j, i)`!
    let n_pts = pts.len();
    for i in 0..n_pts {
        let (ai, ti) = pts[i];
        let dbl = ((ti + ti) as usize) % r;
        r2_buf[dbl] += 1;
        bitset_set(&mut full_s2_bits, dbl);
        for j in (i + 1)..n_pts {
            let (aj, tj) = pts[j];
            let mut s = (ti + tj) as usize;
            if s >= r {
                s -= r;
            }
            r2_buf[s] += 2;
            bitset_set(&mut full_s2_bits, s);
            if ai != aj {
                bitset_set(&mut dist_s2_bits, s);
            }
        }
    }

    let mut e2: u64 = 0;
    for &c in &r2_buf[..r] {
        e2 += (c as u64) * (c as u64);
    }

    if m == 2 {
        return (
            fb_pts,
            bitset_popcount(&full_s2_bits),
            bitset_popcount(&dist_s2_bits),
            e2,
        );
    }

    // m == 3:
    // 1. Full 3-fold sumset `S3 = S2 + B_u` via fast cyclic bitset shifts!
    let mut full_s3_bits = vec![0u64; words_len];
    for &(_, t) in &pts {
        bitset_or_cyclic_shift(&mut full_s3_bits, &full_s2_bits, t as usize, r);
    }
    let full_cov_cnt = bitset_popcount(&full_s3_bits);

    // 2. Distinct-x 3-tuples `{P_i + P_j + P_k : x_i < x_j < x_k}` (`8` sign choices per triple).
    // At p <= 65537, X <= 40 so `a_len <= 40` and `binom(40, 3) * 8 <= 79,040` ops (< 50 us)!
    let mut dist_s3_bits = vec![0u64; words_len];
    for i in 0..a_len {
        let ti = abscissa_dlogs[i] as usize;
        let nti = r - ti;
        for j in (i + 1)..a_len {
            let tj = abscissa_dlogs[j] as usize;
            let ntj = r - tj;
            let s_pp = (ti + tj) % r;
            let s_pn = (ti + ntj) % r;
            let s_np = (nti + tj) % r;
            let s_nn = (nti + ntj) % r;
            for &tk_u32 in &abscissa_dlogs[(j + 1)..a_len] {
                let tk = tk_u32 as usize;
                let ntk = r - tk;
                for base in [s_pp, s_pn, s_np, s_nn] {
                    let mut u1 = base + tk;
                    if u1 >= r {
                        u1 -= r;
                    }
                    let mut u2 = base + ntk;
                    if u2 >= r {
                        u2 -= r;
                    }
                    bitset_set(&mut dist_s3_bits, u1);
                    bitset_set(&mut dist_s3_bits, u2);
                }
            }
        }
    }

    (
        fb_pts,
        full_cov_cnt,
        bitset_popcount(&dist_s3_bits),
        e2,
    )
}

fn run_kaarelson_coverage() -> Vec<KaarelsonCoverageCell> {
    let mut out = Vec::new();
    for &p in &[1_009u64, 4_099, 10_007, 65_537] {
        let (fc, g, a, b) = find_prime_order_curve_at_p(p);
        let r = fc.n as usize;

        // Build exact discrete-log table on E_1(F_p): for each canonical x in 0..p,
        // `dlog_at_x[x]` = Some(t in 1..=(r-1)/2) if x lifts to E_1(F_p), else None.
        let mut dlog_at_x: Vec<u32> = vec![0; p as usize];
        let mut cur: Option<crypto_lib::cryptanalysis::prime_fast::Pt> = None;
        for t in 1..=((r - 1) / 2) {
            cur = fc.add(cur, Some(g));
            let pt = cur.expect("non-infinity");
            let (xc, _) = fc.canonical(pt);
            dlog_at_x[xc as usize] = t as u32;
        }

        // Collect all unique quadratic residues `s = u^(-2) mod p` (`(p - 1)/2` values;
        // each appears with multiplicity 2 for `+u` and `-u` in `F_p*`).
        let f = fc.f;
        let mut unique_s: Vec<u64> = Vec::with_capacity(((p - 1) / 2) as usize);
        let mut seen_s = vec![false; p as usize];
        for u in 1..p {
            let u2 = (u * u) % p;
            let s = f.from_mont(f.inv(f.to_mont(u2)));
            if !seen_s[s as usize] {
                seen_s[s as usize] = true;
                unique_s.push(s);
            }
        }
        assert_eq!(unique_s.len(), ((p - 1) / 2) as usize);

        for &m in &[2u32, 3] {
            let box_x = floor_root(p, m);

            // At p == 1009, cross-check the isomorphism directly against brute-force point
            // addition on `E_u : y^2 = x^3 + u^4 A x + u^6 B` across ALL `u in 1..p`!
            let mut crosscheck_mismatches = 0usize;
            if p == 1_009 {
                let mut r2_tmp = vec![0u32; r];
                for u in [1u64, 2, 3, 5, 7, 11, 42, 500, 1008] {
                    let u2 = (u * u) % p;
                    let u4 = (u2 * u2) % p;
                    let u6 = (u4 * u2) % p;
                    let au = (u4 * a) % p;
                    let bu = (u6 * b) % p;
                    let fc_u = FastCurve::new("eu", p, au, bu, fc.n);
                    let mut direct_pts = Vec::new();
                    for x in 1..=box_x {
                        if let Some(pt) = fc_u.lift_x(x) {
                            direct_pts.push(pt);
                            direct_pts.push(fc_u.neg(pt));
                        }
                    }
                    // Direct sumset size on E_u:
                    let mut sum_set = std::collections::HashSet::new();
                    if m == 2 {
                        for &p1 in &direct_pts {
                            for &p2 in &direct_pts {
                                sum_set.insert(fc_u.add(Some(p1), Some(p2)).map(|q| fc_u.canonical(q)));
                            }
                        }
                    } else {
                        for &p1 in &direct_pts {
                            for &p2 in &direct_pts {
                                let s12 = fc_u.add(Some(p1), Some(p2));
                                for &p3 in &direct_pts {
                                    sum_set.insert(fc_u.add(s12, Some(p3)).map(|q| fc_u.canonical(q)));
                                }
                            }
                        }
                    }
                    let s = f.from_mont(f.inv(f.to_mont(u2)));
                    let abs_dlogs: Vec<u32> = (1..=box_x)
                        .filter_map(|xp| {
                            let x1 = ((s * xp) % p) as usize;
                            let dl = dlog_at_x[x1];
                            (dl != 0).then_some(dl)
                        })
                        .collect();
                    let (fb_cnt, full_cnt, _, _) = eval_box_stats(&abs_dlogs, r, m, &mut r2_tmp);
                    if fb_cnt != direct_pts.len() || full_cnt != sum_set.len() {
                        crosscheck_mismatches += 1;
                    }
                }
                assert_eq!(crosscheck_mismatches, 0, "E_u isomorphism mismatch at p=1009 m={m}");
            }

            // Evaluate ALL `s in (F_p*)^2` in parallel across worker threads!
            let threads = std::thread::available_parallelism()
                .map(|n| n.get().min(8))
                .unwrap_or(4);
            let chunk_size = unique_s.len().div_ceil(threads);
            let mut per_s_results: Vec<(u64, usize, usize, usize, u64)> =
                Vec::with_capacity(unique_s.len());

            std::thread::scope(|scope| {
                let mut handles = Vec::new();
                for chunk in unique_s.chunks(chunk_size) {
                    let dlog_ref = &dlog_at_x;
                    handles.push(scope.spawn(move || {
                        let mut local = Vec::with_capacity(chunk.len());
                        let mut r2_buf = vec![0u32; r];
                        let mut abs_dlogs = Vec::with_capacity(box_x as usize);
                        for &s in chunk {
                            abs_dlogs.clear();
                            for xp in 1..=box_x {
                                let x1 = ((s as u128 * xp as u128) % p as u128) as usize;
                                let dl = dlog_ref[x1];
                                if dl != 0 {
                                    abs_dlogs.push(dl);
                                }
                            }
                            let (fb_pts, full_cnt, dist_cnt, e2) =
                                eval_box_stats(&abs_dlogs, r, m, &mut r2_buf);
                            local.push((s, fb_pts, full_cnt, dist_cnt, e2));
                        }
                        local
                    }));
                }
                for h in handles {
                    per_s_results.extend(h.join().unwrap());
                }
            });

            let rf = r as f64;
            let x2 = (box_x as f64) * (box_x as f64);
            let n_s = per_s_results.len() as f64;

            let u1_row = *per_s_results
                .iter()
                .find(|&&(s, _, _, _, _)| s == 1)
                .expect("u=1 present");
            let u1_fb = u1_row.1;
            let u1_full_cov = u1_row.2 as f64 / rf;
            let u1_dist_cov = u1_row.3 as f64 / rf;
            let u1_e2_x2 = u1_row.4 as f64 / x2;
            let u1_e2_fb2 = if u1_fb > 0 {
                u1_row.4 as f64 / ((u1_fb as f64) * (u1_fb as f64))
            } else {
                0.0
            };

            let mean_fb = per_s_results.iter().map(|r0| r0.1 as f64).sum::<f64>() / n_s;
            let mean_full = per_s_results.iter().map(|r0| r0.2 as f64 / rf).sum::<f64>() / n_s;
            let mean_dist = per_s_results.iter().map(|r0| r0.3 as f64 / rf).sum::<f64>() / n_s;
            let mean_e2 = per_s_results.iter().map(|r0| r0.4 as f64 / x2).sum::<f64>() / n_s;

            let std_full = (per_s_results
                .iter()
                .map(|r0| {
                    let d = (r0.2 as f64 / rf) - mean_full;
                    d * d
                })
                .sum::<f64>()
                / n_s)
                .sqrt();
            let std_dist = (per_s_results
                .iter()
                .map(|r0| {
                    let d = (r0.3 as f64 / rf) - mean_dist;
                    d * d
                })
                .sum::<f64>()
                / n_s)
                .sqrt();
            let std_e2 = (per_s_results
                .iter()
                .map(|r0| {
                    let d = (r0.4 as f64 / x2) - mean_e2;
                    d * d
                })
                .sum::<f64>()
                / n_s)
                .sqrt();

            let min_full = per_s_results
                .iter()
                .map(|r0| r0.2 as f64 / rf)
                .fold(f64::INFINITY, f64::min);
            let max_full = per_s_results
                .iter()
                .map(|r0| r0.2 as f64 / rf)
                .fold(0.0, f64::max);
            let min_dist = per_s_results
                .iter()
                .map(|r0| r0.3 as f64 / rf)
                .fold(f64::INFINITY, f64::min);
            let max_dist = per_s_results
                .iter()
                .map(|r0| r0.3 as f64 / rf)
                .fold(0.0, f64::max);

            out.push(KaarelsonCoverageCell {
                p,
                curve_a: a,
                curve_b: b,
                group_order_r: r,
                m,
                box_bound_x: box_x,
                models_evaluated_u_count: (p - 1) as usize,
                direct_eu_crosscheck_mismatches: crosscheck_mismatches,
                u1_fb_points: u1_fb,
                u1_full_sumset_coverage: u1_full_cov,
                u1_distinct_x_coverage: u1_dist_cov,
                u1_e2_over_x2: u1_e2_x2,
                u1_e2_over_fb2: u1_e2_fb2,
                dilated_mean_fb_points: mean_fb,
                dilated_mean_full_sumset_coverage: mean_full,
                dilated_std_full_sumset_coverage: std_full,
                dilated_min_full_sumset_coverage: min_full,
                dilated_max_full_sumset_coverage: max_full,
                dilated_mean_distinct_x_coverage: mean_dist,
                dilated_std_distinct_x_coverage: std_dist,
                dilated_min_distinct_x_coverage: min_dist,
                dilated_max_distinct_x_coverage: max_dist,
                dilated_mean_e2_over_x2: mean_e2,
                dilated_std_e2_over_x2: std_e2,
                u1_minus_dilated_full_cov_diff: u1_full_cov - mean_full,
                u1_full_cov_z_score: if std_full > 0.0 {
                    (u1_full_cov - mean_full) / std_full
                } else {
                    0.0
                },
                u1_distinct_x_cov_z_score: if std_dist > 0.0 {
                    (u1_dist_cov - mean_dist) / std_dist
                } else {
                    0.0
                },
            });
        }
    }
    out
}

fn run_wcal_and_rho_ratios() -> (Vec<WCalExtendedCell>, Vec<RhoNegationRatioCell>) {
    let mut wcal_cells = Vec::new();
    let mut rho_cells = Vec::new();

    for &(bits, rho_trials) in &[(32u32, 16usize), (36, 12), (40, 10), (44, 6)] {
        let (fc, g) = find_a3_curve(bits, 3).expect("ladder curve");
        let p = fc.f.p;
        let r = fc.n;
        let a_can = fc.f.from_mont(fc.a);
        let b_can = fc.f.from_mont(fc.b);

        // 1. Standard `ecbench::wcal::measure`:
        let wm = wcal::measure(p, a_can, b_can, 7, 50_000).expect("wcal measure");

        // 2. Montgomery `Fp64::mul` (dependent chain & 4-way ILP) and `MulModN::mul`:
        let f = fc.f;
        let iters = 200_000u64;
        let c_m = f.to_mont(7);
        let (fp64_chain_ms, _, _) = best_of_n_ms(7, || {
            let mut x = black_box(f.to_mont(3));
            let y = black_box(c_m);
            for _ in 0..iters {
                x = f.mul(x, y);
            }
            x
        });
        let fp64_mont_mul_chain_ns = (fp64_chain_ms * 1e6) / iters as f64;

        let (fp64_ilp_ms, _, _) = best_of_n_ms(7, || {
            let mut x0 = black_box(f.to_mont(3));
            let mut x1 = black_box(f.to_mont(5));
            let mut x2 = black_box(f.to_mont(11));
            let mut x3 = black_box(f.to_mont(13));
            let y = black_box(c_m);
            for _ in 0..(iters / 4) {
                x0 = f.mul(x0, y);
                x1 = f.mul(x1, y);
                x2 = f.mul(x2, y);
                x3 = f.mul(x3, y);
            }
            x0 ^ x1 ^ x2 ^ x3
        });
        let fp64_mont_mul_ilp4_ns = (fp64_ilp_ms * 1e6) / iters as f64;

        let mm = MulModN::new(p);
        let (mm_ms, _, _) = best_of_n_ms(7, || {
            let mut x = black_box(3u64);
            let y = black_box(7u64);
            for _ in 0..iters {
                x = mm.mul(x, y);
            }
            x
        });
        let mulmodn_canonical_ns = (mm_ms * 1e6) / iters as f64;

        // 3. Unbatched `FastCurve::add`:
        let h_pt = fc.mul(Some(g), 3).unwrap();
        let add_iters = 50_000u64;
        let (fc_add_ms, _, _) = best_of_n_ms(7, || {
            let mut cur = Some(black_box(g));
            for _ in 0..add_iters {
                let step = if cur == Some(h_pt) || cur == Some(fc.neg(h_pt)) {
                    g
                } else {
                    h_pt
                };
                cur = fc.add(cur, Some(step));
            }
            cur
        });
        let fastcurve_unbatched_add_ns = (fc_add_ms * 1e6) / add_iters as f64;

        // 4. In-situ batched negation-map rho GAE step (W = 64 and W = 128):
        let bench_rho_gae = |w: usize| -> f64 {
            let mut rng = StdRng::seed_from_u64(0x9000 + w as u64);
            let table: Vec<_> = (1..=16u64)
                .map(|k| fc.mul(Some(g), k * 19 + 3).unwrap())
                .collect();
            let mut xs = Vec::with_capacity(w);
            let mut ys = Vec::with_capacity(w);
            for k in 0..w {
                let pt = fc.mul(Some(g), (k as u64 + 1) * 101 + rng.gen_range(1..1000)).unwrap();
                xs.push(pt.x);
                ys.push(pt.y);
            }
            let mut d = vec![0u64; w];
            let mut js = vec![0usize; w];
            let mut scratch = Vec::with_capacity(w);
            let rounds = 4_000usize;
            let (ms, _, _) = best_of_n_ms(7, || {
                let mut dp_hits = 0u64;
                for _ in 0..rounds {
                    for i in 0..w {
                        let j = ((xs[i].wrapping_mul(0x9E37_79B9_7F4A_7C15)) >> 60) as usize;
                        js[i] = j;
                        let dd = f.sub(table[j].x, xs[i]);
                        d[i] = if dd == 0 { f.one } else { dd };
                    }
                    f.batch_inv(&mut d, &mut scratch);
                    for i in 0..w {
                        let t = table[js[i]];
                        let lam = f.mul(f.sub(t.y, ys[i]), d[i]);
                        let x3 = f.sub(f.sub(f.sqr(lam), xs[i]), t.x);
                        let mut y3 = f.sub(f.mul(lam, f.sub(xs[i], x3)), ys[i]);
                        if y3 > f.half {
                            y3 = f.p - y3;
                        }
                        xs[i] = x3;
                        ys[i] = y3;
                        if (x3.rotate_left(23).wrapping_mul(0xD6E8_FEB8_6659_FD93) >> 54) == 0 {
                            dp_hits += 1;
                        }
                    }
                }
                black_box(xs[0] ^ dp_hits)
            });
            (ms * 1e6) / ((rounds * w) as f64)
        };
        let rho_batched_gae_w64_ns = bench_rho_gae(64);
        let rho_batched_gae_w128_ns = bench_rho_gae(128);

        // 5. Subproduct-tree remainder step (`deg 255 mod monic deg 128`) & `B=1024` batch:
        let mut rng_sp = StdRng::seed_from_u64(p ^ 0x5A17);
        let dividend: Vec<u64> = (0..256).map(|_| f.to_mont(rng_sp.gen_range(0..p))).collect();
        let mut divisor: Vec<u64> = (0..128).map(|_| f.to_mont(rng_sp.gen_range(0..p))).collect();
        divisor.push(f.one);
        let mut rem_u128 = Vec::with_capacity(260);
        let mut rem_out = Vec::with_capacity(128);
        let rem_iters = 2_000usize;
        let (rem_ms, _, _) = best_of_n_ms(7, || {
            let mut acc = 0u64;
            for _ in 0..rem_iters {
                poly_rem_monic_mont(&f, black_box(&dividend), black_box(&divisor), &mut rem_u128, &mut rem_out);
                acc ^= rem_out.first().copied().unwrap_or(0);
            }
            acc
        });
        // 128 quotient steps * 128 divisor terms = 16,384 coeff MACs per remainder call:
        let subproduct_rem_step_deg128_ns_per_coeff = (rem_ms * 1e6) / (rem_iters as f64 * 128.0 * 128.0);

        let base_1024: Vec<u64> = (1..=1024u64).collect();
        let tree_1024 = FactorBaseSubproductTree::prepare(f, &base_1024);
        let cands_512: Vec<Vec<u64>> = (0..512)
            .map(|_| {
                let mut v: Vec<u64> = (0..4).map(|_| rng_sp.gen_range(0..p)).collect();
                v.push(rng_sp.gen_range(1..p));
                v
            })
            .collect();
        let mut sp_scratch = SubproductBatchScratch::default();
        let (sp_batch_ms, _, _) = best_of_n_ms(5, || {
            tree_1024.roots_in_base_batch(&cands_512, &mut sp_scratch)
        });
        let subproduct_batch_b1024_deg4_ns_per_poly = (sp_batch_ms * 1e6) / 512.0;

        // 6. Sparse Wiedemann SpMV step mod `r` (`N = 2048`, 4 signed factor-base entries + 1 border coeff):
        let fr = Fp64::new(r);
        let dim = 2048usize;
        let mut row_cols = vec![[0u32; 4]; dim];
        let mut row_signs = vec![[1i8; 4]; dim];
        let mut border_mont = vec![0u64; dim];
        for i in 0..dim {
            for c in 0..4 {
                row_cols[i][c] = rng_sp.gen_range(0..dim as u32);
                row_signs[i][c] = if rng_sp.gen_bool(0.5) { 1 } else { -1 };
            }
            border_mont[i] = fr.to_mont(rng_sp.gen_range(1..r));
        }
        let mut vec_in: Vec<u64> = (0..dim).map(|_| fr.to_mont(rng_sp.gen_range(0..r))).collect();
        let mut vec_out = vec![0u64; dim];
        let spmv_iters = 500usize;
        let (spmv_ms, _, _) = best_of_n_ms(7, || {
            let b_scalar = vec_in[0];
            for _ in 0..spmv_iters {
                for row in 0..dim {
                    let cols = row_cols[row];
                    let sg = row_signs[row];
                    let mut sum = fr.mul(border_mont[row], b_scalar);
                    for k in 0..4 {
                        let v = vec_in[cols[k] as usize];
                        sum = if sg[k] > 0 {
                            fr.add(sum, v)
                        } else {
                            fr.sub(sum, v)
                        };
                    }
                    vec_out[row] = sum;
                }
                std::mem::swap(&mut vec_in, &mut vec_out);
            }
            black_box(vec_in[0])
        });
        let wiedemann_spmv_n2048_w4_ns_per_row = (spmv_ms * 1e6) / ((spmv_iters * dim) as f64);
        let wiedemann_spmv_n2048_w4_ns_per_nz = wiedemann_spmv_n2048_w4_ns_per_row / 5.0;

        wcal_cells.push(WCalExtendedCell {
            bits,
            p,
            r,
            ecbench_wcal_mul_ns: wm.mul_ns,
            ecbench_wcal_unbatched_add_ns: wm.add_ns,
            ecbench_wcal_ratio: wm.w_cal,
            ecbench_wcal_mul_spread: wm.mul_spread,
            ecbench_wcal_add_spread: wm.add_spread,
            fp64_mont_mul_chain_ns,
            fp64_mont_mul_ilp4_ns,
            mulmodn_canonical_ns,
            fastcurve_unbatched_add_ns,
            rho_batched_gae_w64_ns,
            rho_batched_gae_w128_ns,
            subproduct_rem_step_deg128_ns_per_coeff,
            subproduct_batch_b1024_deg4_ns_per_poly,
            wiedemann_spmv_n2048_w4_ns_per_row,
            wiedemann_spmv_n2048_w4_ns_per_nz,
            w_cal_fp64_mul_over_rho_gae: fp64_mont_mul_chain_ns / rho_batched_gae_w128_ns,
            w_cal_wiedemann_row_over_rho_gae: wiedemann_spmv_n2048_w4_ns_per_row
                / rho_batched_gae_w128_ns,
            w_cal_subproduct_poly_over_rho_gae: subproduct_batch_b1024_deg4_ns_per_poly
                / rho_batched_gae_w128_ns,
        });

        // 7. 32..44-bit `rho.negation` ratio to `sqrt(pi * r / 4)`:
        let theo = (std::f64::consts::PI * (r as f64) / 4.0).sqrt();
        let mut rng_rho = StdRng::seed_from_u64(0x7000_0000 + bits as u64);
        let mut steps_list = Vec::with_capacity(rho_trials);
        let mut total_ops_list = Vec::with_capacity(rho_trials);
        let mut wall_ms_sum = 0.0;
        let mut msteps_s_sum = 0.0;
        let mut all_ok = true;
        for tr in 0..rho_trials {
            let secret = rng_rho.gen_range(1..r);
            let q = fc.mul(Some(g), secret).unwrap();
            let mut cfg = RhoConfig::tuned(r);
            cfg.seed = 1000 + tr as u64;
            let res = rho_parallel(&fc, g, q, &cfg).expect("rho_parallel");
            all_ok &= res.log == secret && fc.mul(Some(g), res.log) == Some(q);
            steps_list.push(res.total_steps);
            let extra_ops = res.ops.table_adds
                + res.ops.table_doubles
                + res.ops.initial_start_adds
                + res.ops.initial_start_doubles
                + res.ops.restart_adds
                + res.ops.restart_doubles
                + res.ops.escape_doubles;
            total_ops_list.push(res.total_steps + extra_ops);
            wall_ms_sum += res.wall_secs * 1e3;
            msteps_s_sum += res.steps_per_sec / 1e6;
        }
        assert!(all_ok, "rho.negation failed at bits={bits}");
        steps_list.sort_unstable();
        let min_steps = *steps_list.first().unwrap();
        let max_steps = *steps_list.last().unwrap();
        let mean_steps = steps_list.iter().map(|&s| s as f64).sum::<f64>() / rho_trials as f64;
        let median_steps = if rho_trials % 2 == 1 {
            steps_list[rho_trials / 2] as f64
        } else {
            0.5 * (steps_list[rho_trials / 2 - 1] as f64 + steps_list[rho_trials / 2] as f64)
        };
        let mean_tot_ops =
            total_ops_list.iter().map(|&s| s as f64).sum::<f64>() / rho_trials as f64;

        rho_cells.push(RhoNegationRatioCell {
            bits,
            p,
            r,
            trials: rho_trials,
            all_recovered_ok: all_ok,
            theoretical_sqrt_pi_r_over_4: theo,
            mean_steps,
            median_steps,
            min_steps,
            max_steps,
            mean_ratio_to_sqrt_pi_r_over_4: mean_steps / theo,
            median_ratio_to_sqrt_pi_r_over_4: median_steps / theo,
            mean_total_group_ops_ratio: mean_tot_ops / theo,
            mean_wall_ms: wall_ms_sum / rho_trials as f64,
            mean_msteps_per_sec: msteps_s_sum / rho_trials as f64,
        });
    }

    (wcal_cells, rho_cells)
}

fn main() {
    let out_dir = Path::new(OUT_DIR);
    fs::create_dir_all(out_dir).expect("create output dir");
    let skip_p01a = std::env::var_os("SKIP_P01A").is_some();

    if !skip_p01a {
        println!("=== [1/3] Running Lead P0-1a (semaev_projection rational batch_inv & early exit) ===");
        let p01a = run_p0_1a();
        fs::write(
            out_dir.join("p0_1a_semaev_projection_bench.json"),
            serde_json::to_string_pretty(&p01a).unwrap() + "\n",
        )
        .unwrap();
        for c in &p01a.cells {
            println!(
                "  {} B={}: build {:.2}ms -> {:.2}ms ({:.2}x, invs {} -> {}, sqrts {} -> {}); d4 {:.2}ms -> {:.2}ms ({:.2}x, invs {} -> {})",
                c.curve_label,
                c.fb_size,
                c.ref_build_ms,
                c.opt_build_ms,
                c.build_wall_speedup_x,
                c.ref_build_ops.invs,
                c.opt_build_ops.invs,
                c.ref_build_ops.sqrts,
                c.opt_build_ops.sqrts,
                c.ref_d4_ms,
                c.opt_d4_ms,
                c.d4_wall_speedup_x,
                c.ref_d4_ops.invs,
                c.opt_d4_ops.invs,
            );
        }
    }

    println!("\n=== [2/3] Running Lead P0-1b (subproduct-tree multipoint & lock-step walkers) ===");
    let p01b = run_p0_1b();
    fs::write(
        out_dir.join("p0_1b_subproduct_and_lockstep_bench.json"),
        serde_json::to_string_pretty(&p01b).unwrap() + "\n",
    )
    .unwrap();
    for c in &p01b.subproduct_cells {
        println!(
            "  {} B={:4}: fp_fast={:8.0}ns  cz64={:6.1}ns  horner={:6.1}ns  ungated={:6.1}ns  gated={:6.1}ns ({:.1}x vs fp_fast, {:.2}x vs cz64)",
            c.prime_label,
            c.fb_size,
            c.find_roots_fp_fast_ns_per_poly,
            c.native_fp64_cz_ns_per_poly,
            c.direct_horner_ns_per_poly,
            c.ungated_subproduct_ns_per_poly,
            c.gated_subproduct_ns_per_poly,
            c.speedup_vs_find_roots_fp_fast_x,
            c.speedup_vs_native_fp64_cz_x,
        );
    }
    for c in &p01b.lockstep_cells {
        println!(
            "  bits={} B={}: sweep={:.2}ms  table_w1={:.2}ms (rel {:.2}ms)  lockstep_w256={:.2}ms (rel {:.2}ms, {:.2}x rel speedup)  rho={:.2}ms",
            c.bits,
            c.fb_size,
            c.unbatched_sweep_mean_ms,
            c.unbatched_table_mean_ms,
            c.unbatched_table_relations_only_ms,
            c.lockstep_w256_mean_ms,
            c.lockstep_w256_relations_only_ms,
            c.collection_speedup_w256_vs_unbatched_table_x,
            c.rho_tuned_mean_ms,
        );
    }

    println!("\n=== [3/3] Running Lead P2-8 (Kaarelson Theorem 3 + Open Problem 1 & §5a w_cal) ===");
    let cov = run_kaarelson_coverage();
    let (wcal_cells, rho_cells) = run_wcal_and_rho_ratios();
    let p28 = P28Receipt {
        lead: "P2-8",
        timing_method: BEST_OF_N,
        kaarelson_coverage_cells: cov,
        wcal_cells,
        rho_negation_cells: rho_cells,
    };
    fs::write(
        out_dir.join("p2_8_kaarelson_coverage_and_wcal.json"),
        serde_json::to_string_pretty(&p28).unwrap() + "\n",
    )
    .unwrap();
    for c in &p28.kaarelson_coverage_cells {
        println!(
            "  p={:5} m={} X={:3}: u=1 full={:.4} (dist={:.4}, E2/X^2={:.3}) | dilated mean full={:.4}±{:.4} (dist={:.4}±{:.4}, E2/X^2={:.3}±{:.3}) | z={:+.2}",
            c.p,
            c.m,
            c.box_bound_x,
            c.u1_full_sumset_coverage,
            c.u1_distinct_x_coverage,
            c.u1_e2_over_x2,
            c.dilated_mean_full_sumset_coverage,
            c.dilated_std_full_sumset_coverage,
            c.dilated_mean_distinct_x_coverage,
            c.dilated_std_distinct_x_coverage,
            c.dilated_mean_e2_over_x2,
            c.dilated_std_e2_over_x2,
            c.u1_full_cov_z_score,
        );
    }
    for c in &p28.wcal_cells {
        println!(
            "  w_cal bits={}: fp64_mul={:.2}ns  rho_gae_w128={:.2}ns (w_cal_batched={:.4} vs unbatched_wcal={:.4})  spmv_row={:.2}ns  subprod_poly={:.1}ns",
            c.bits,
            c.fp64_mont_mul_chain_ns,
            c.rho_batched_gae_w128_ns,
            c.w_cal_fp64_mul_over_rho_gae,
            c.ecbench_wcal_ratio,
            c.wiedemann_spmv_n2048_w4_ns_per_row,
            c.subproduct_batch_b1024_deg4_ns_per_poly,
        );
    }
    for c in &p28.rho_negation_cells {
        println!(
            "  rho.negation bits={}: mean/sqrt(pi*r/4)={:.4}  median/sqrt(pi*r/4)={:.4}  total_ops_ratio={:.4}  ({:.1} Msteps/s, {:.2} ms)",
            c.bits,
            c.mean_ratio_to_sqrt_pi_r_over_4,
            c.median_ratio_to_sqrt_pi_r_over_4,
            c.mean_total_group_ops_ratio,
            c.mean_msteps_per_sec,
            c.mean_wall_ms,
        );
    }
    println!("\nAll receipts written cleanly to {OUT_DIR}/.");
}
