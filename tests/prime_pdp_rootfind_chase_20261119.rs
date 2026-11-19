//! Integration & parity test suite for Workstream 1 (`prime_pdp_rootfind_chase_20261119`):
//! - Lead P0-1a: Rational-discriminant batch inversion, canonical `y_i` caching, batched query
//!   denominators, and `decompose_four` early witness exit in `semaev_projection`.
//! - Lead P0-1b: Shared factor-base subproduct-tree multipoint & batched candidate root-finding
//!   (crossover-gated vs `find_roots_fp_fast`) and lock-step multi-walker batch inversion
//!   (`W = 64..256`) in `prime_fast`.

use crypto_lib::cryptanalysis::ec_index_calculus::find_roots_fp_fast;
use crypto_lib::cryptanalysis::pkm_smooth_tower::Curve;
use crypto_lib::cryptanalysis::prime_fast::{
    collect_relations_lockstep, find_a3_curve, find_roots_fp64_cz, ic_solve, ic_solve_lockstep,
    ic_solve_table, rho_parallel, FactorBaseSubproductTree, FastCurve, FastFactorBase, Fp64,
    LockStepCollectScratch, LockStepMode, PairTable, RhoConfig, SubproductBatchScratch,
};
use crypto_lib::cryptanalysis::research_bench::bench_curves_a_minus_3;
use crypto_lib::cryptanalysis::semaev_projection::{
    quadratic_root_keys, s3_pair_roots_biguint_rational_batched,
    s3_pair_roots_biguint_reference, s3_pair_roots_rational_batched, s3_quadratic,
    BigQuadraticRoots, FpOpCounts, PairProjection,
};
use crypto_lib::ecc::field::FieldElement;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn lift_canonical_ref(curve: &Curve, x: u64) -> Option<u64> {
    let p = curve.p;
    let x128 = (x % p) as u128;
    let p128 = p as u128;
    let rhs = ((x128 * x128 % p128 * x128
        + (curve.a % p) as u128 * x128
        + (curve.b % p) as u128)
        % p128) as u64;
    let f = Fp64::new(p);
    let y = f.from_mont(f.sqrt(f.to_mont(rhs))?);
    Some(y.min(p - y))
}

fn check_valid_four_witness(
    curve: &Curve,
    base_xs: &[u64],
    xr: u64,
    w: [u64; 4],
) -> bool {
    let f = Fp64::new(curve.p);
    let fc = FastCurve::new("chk", curve.p, curve.a, curve.b, 1);
    for &xi in &w {
        assert!(base_xs.contains(&xi), "witness {xi} not in factor base");
        assert!(fc.lift_x(xi).is_some(), "witness {xi} off curve");
    }
    let Some(pr) = fc.lift_x(xr % curve.p) else {
        return false;
    };
    let p0 = fc.lift_x(w[0]).unwrap();
    let p1 = fc.lift_x(w[1]).unwrap();
    let p2 = fc.lift_x(w[2]).unwrap();
    let p3 = fc.lift_x(w[3]).unwrap();
    let _ = f;
    for s0 in [false, true] {
        for s1 in [false, true] {
            for s2 in [false, true] {
                for s3 in [false, true] {
                    let q0 = if s0 { fc.neg(p0) } else { p0 };
                    let q1 = if s1 { fc.neg(p1) } else { p1 };
                    let q2 = if s2 { fc.neg(p2) } else { p2 };
                    let q3 = if s3 { fc.neg(p3) } else { p3 };
                    let sum = fc.add(fc.add(fc.add(Some(q0), Some(q1)), Some(q2)), Some(q3));
                    if let Some(pt) = sum {
                        if fc.canonical(pt).0 == fc.canonical(pr).0 {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

#[test]
fn p0_1a_exhaustive_grid_zero_root_mismatches_with_torsion_and_diagonals() {
    // Includes 2-torsion y_i = 0, singular curve, x_1 = x_2 diagonals, and p = 1 mod 4 / 3 mod 4.
    let curves = [
        Curve { p: 17, a: 0, b: 0 }, // singular + y=0 at x=0
        Curve { p: 19, a: 18, b: 0 }, // three rational 2-torsion points x in {0, 1, 18}
        Curve { p: 29, a: 4, b: 20 },
        Curve { p: 41, a: 3, b: 0 }, // p = 1 mod 4 with y = 0 at x = 0
        Curve { p: 43, a: 40, b: 7 },
    ];
    for curve in curves {
        let mut pairs = Vec::with_capacity((curve.p * curve.p) as usize);
        let mut expected = Vec::with_capacity((curve.p * curve.p) as usize);
        for x1 in 0..curve.p {
            let y1 = lift_canonical_ref(&curve, x1);
            for x2 in 0..curve.p {
                let y2 = lift_canonical_ref(&curve, x2);
                pairs.push((x1, y1, x2, y2));
                expected.push(quadratic_root_keys(s3_quadratic(&curve, x1, x2), curve.p));
            }
        }
        let mut ops = FpOpCounts::default();
        let got = s3_pair_roots_rational_batched(&curve, &pairs, &mut ops);
        let norm_qr = |r: &crypto_lib::cryptanalysis::semaev_projection::QuadraticRoots| {
            match r {
                crypto_lib::cryptanalysis::semaev_projection::QuadraticRoots::Finite(v) => {
                    let mut s = v.clone();
                    s.sort();
                    crypto_lib::cryptanalysis::semaev_projection::QuadraticRoots::Finite(s)
                }
                other => other.clone(),
            }
        };
        assert_eq!(got.len(), expected.len());
        for (idx, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                norm_qr(g),
                norm_qr(e),
                "mismatch on p={} a={} b={} pair={:?}",
                curve.p,
                curve.a,
                curve.b,
                pairs[idx]
            );
        }

        // Also explicitly verify off-curve (`y_i == None`) combinations:
        // (on-curve, off-curve), (off-curve, on-curve), and (off-curve, off-curve).
        let off_curve_subset: Vec<_> = pairs
            .iter()
            .copied()
            .filter(|&(_, y1, _, y2)| y1.is_none() || y2.is_none())
            .collect();
        let expected_off: Vec<_> = off_curve_subset
            .iter()
            .map(|&(x1, _, x2, _)| quadratic_root_keys(s3_quadratic(&curve, x1, x2), curve.p))
            .collect();
        let got_off =
            s3_pair_roots_rational_batched(&curve, &off_curve_subset, &mut FpOpCounts::default());
        for (g, e) in got_off.iter().zip(expected_off.iter()) {
            assert_eq!(norm_qr(g), norm_qr(e));
        }
    }
}

#[test]
fn p0_1a_prime31_and_61bit_pair_projection_and_early_exit_parity() {
    for &(p, a, b, fb_target) in &[
        (1_009u64, 1_006, 5, 48usize),
        (2_147_483_647u64, 2_147_483_644, 7, 128usize), // Prime31 = 2^31 - 1
        ((1u64 << 61) - 1, (1u64 << 61) - 4, 11, 96usize), // 61-bit Mersenne prime
    ] {
        let curve = Curve { p, a, b };
        let fc = FastCurve::new("p01a", p, a, b, 1);
        let mut base_xs = Vec::with_capacity(fb_target);
        let mut x = 0u64;
        while base_xs.len() < fb_target && x < p {
            if fc.lift_x(x).is_some() {
                base_xs.push(x);
            }
            x += 1;
        }
        let mut build_ref_ops = FpOpCounts::default();
        let mut build_opt_ops = FpOpCounts::default();
        let proj_ref =
            PairProjection::build_reference_counted(curve, &base_xs, &mut build_ref_ops).unwrap();
        let proj_opt =
            PairProjection::build_counted(curve, &base_xs, &mut build_opt_ops).unwrap();
        let stats_ref = proj_ref.build_stats();
        let stats_opt = proj_opt.build_stats();

        assert_eq!(stats_opt, stats_ref, "BuildStats mismatch p={p}");
        assert!(
            build_opt_ops.sqrts <= fb_target as u64,
            "PairProjection::build_counted should only pay <= B cached lifts: got {} vs B={}",
            build_opt_ops.sqrts,
            fb_target
        );
        assert!(
            build_opt_ops.invs < build_ref_ops.invs / 20,
            "Expected massive inversion reduction: opt={} vs ref={}",
            build_opt_ops.invs,
            build_ref_ops.invs
        );

        // Verify decompose_three and decompose_four across on-curve, off-curve, and planted sums.
        let mut rng = StdRng::seed_from_u64(p ^ 0x2026_1119);
        let mut targets = Vec::new();
        // Planted 3-sums and 4-sums from base_xs:
        for _ in 0..24 {
            let p0 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            let p1 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            let p2 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            let p3 = fc.lift_x(base_xs[rng.gen_range(0..base_xs.len())]).unwrap();
            if let Some(s3) = fc.add(fc.add(Some(p0), Some(p1)), Some(p2)) {
                targets.push(fc.canonical(s3).0);
            }
            if let Some(s4) = fc.add(fc.add(fc.add(Some(p0), Some(p1)), Some(p2)), Some(p3)) {
                targets.push(fc.canonical(s4).0);
            }
        }
        // Random targets (mix of on-curve and off-curve):
        for _ in 0..40 {
            targets.push(rng.gen_range(0..p));
        }

        let mut ref_d4_ops = FpOpCounts::default();
        let mut opt_d4_ops = FpOpCounts::default();
        for &xr in &targets {
            let d3_ref = proj_ref
                .decompose_three_reference_counted(xr, &mut FpOpCounts::default())
                .0;
            let d3_opt = proj_opt
                .decompose_three_counted(xr, &mut FpOpCounts::default())
                .0;
            assert_eq!(d3_opt.is_some(), d3_ref.is_some(), "d3 existence mismatch p={p} xr={xr}");

            let d4_ref = proj_ref
                .decompose_four_reference_counted(xr, &mut ref_d4_ops)
                .0;
            let d4_opt = proj_opt.decompose_four_counted(xr, &mut opt_d4_ops).0;
            assert_eq!(d4_opt.is_some(), d4_ref.is_some(), "d4 existence mismatch p={p} xr={xr}");
            if let Some(w) = d4_opt {
                assert!(
                    check_valid_four_witness(&curve, &base_xs, xr, w.x),
                    "invalid decompose_four witness p={p} xr={xr} w={w:?}"
                );
            }
        }
        assert!(
            opt_d4_ops.invs < ref_d4_ops.invs / 10 && opt_d4_ops.sqrts < ref_d4_ops.sqrts / 10,
            "decompose_four should sharply cut invs/sqrts: opt={opt_d4_ops:?} ref={ref_d4_ops:?}"
        );
    }
}

#[test]
fn p0_1a_multilimb_127bit_and_256bit_zero_mismatches() {
    // 127-bit Mersenne prime 2^127 - 1 (p = 3 mod 4) and secp256k1 field prime (256-bit, p = 3 mod 4).
    let p127: BigUint = (BigUint::from(1u8) << 127) - BigUint::from(1u8);
    let p256 = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap();

    for (p, a, b) in [
        (&p127, &(&p127 - BigUint::from(3u8)), &BigUint::from(5u8)),
        (&p256, &BigUint::from(0u8), &BigUint::from(7u8)),
    ] {
        let mut on_curve: Vec<(BigUint, Option<BigUint>)> = Vec::new();
        let exp = (p + BigUint::from(1u8)) >> 2usize;
        for x_u in 0u64..48 {
            let x = BigUint::from(x_u);
            let rhs = (&x * &x * &x + a * &x + b) % p;
            let y = rhs.modpow(&exp, p);
            if (&y * &y) % p == rhs {
                let y_can = y.clone().min(p - &y);
                on_curve.push((x, Some(y_can)));
            } else {
                on_curve.push((x, None));
            }
        }
        let mut batch = Vec::new();
        for i in 0..20 {
            for j in i..20 {
                batch.push((
                    on_curve[i].0.clone(),
                    on_curve[i].1.clone(),
                    on_curve[j].0.clone(),
                    on_curve[j].1.clone(),
                ));
            }
        }
        let got = s3_pair_roots_biguint_rational_batched(p, a, b, &batch);
        for (idx, (x1, _, x2, _)) in batch.iter().enumerate() {
            let want = s3_pair_roots_biguint_reference(p, a, b, x1, x2);
            match (&got[idx], &want) {
                (BigQuadraticRoots::Finite(g), BigQuadraticRoots::Finite(w)) => {
                    let mut gs = g.clone();
                    let mut ws = w.clone();
                    gs.sort();
                    ws.sort();
                    assert_eq!(gs, ws, "multilimb root set mismatch at {idx}");
                }
                _ => assert_eq!(got[idx], want, "multilimb variant mismatch at {idx}"),
            }
        }
    }
}

#[test]
fn p0_1b_subproduct_tree_multipoint_and_batched_roots_zero_mismatches() {
    for &p in &[65_537u64, 2_147_483_647u64, (1u64 << 44) - 117] {
        // Ensure prime:
        let p = if p == (1u64 << 44) - 117 {
            let (fc, _) = find_a3_curve(44, 3).unwrap();
            fc.f.p
        } else {
            p
        };
        let f = Fp64::new(p);
        let p_big = BigUint::from(p);
        let mut rng = StdRng::seed_from_u64(p ^ 0x517C_2026);

        for &b_size in &[8usize, 32, 64, 128, 256, 512] {
            let mut xs = Vec::with_capacity(b_size);
            let mut cur = 1u64;
            while xs.len() < b_size {
                xs.push(cur);
                cur += 1 + rng.gen_range(0..4);
            }
            let tree = FactorBaseSubproductTree::prepare(f, &xs);

            // 1. Verify multipoint evaluation `eval_all` on degree-200 polynomial.
            let big_poly: Vec<u64> = (0..180).map(|_| rng.gen_range(0..p)).collect();
            let mut vals = Vec::new();
            tree.eval_all(&big_poly, &mut vals);
            assert_eq!(vals.len(), tree.len());
            for (i, &x) in tree.xs.iter().enumerate() {
                let xm = f.to_mont(x);
                let mut acc = 0u64;
                for &c in big_poly.iter().rev() {
                    acc = f.add(f.mul(acc, xm), f.to_mont(c));
                }
                assert_eq!(vals[i], f.from_mont(acc), "eval_all mismatch p={p} B={b_size} i={i}");
            }

            // 2. Verify single & batched factor-base root-finding against `find_roots_fp_fast`
            //    and `find_roots_fp64_cz` across 120 candidate polynomials (quartics, quadratics,
            //    non-split, roots outside V, roots inside V, repeated roots).
            let mut candidates: Vec<Vec<u64>> = Vec::with_capacity(120);
            for t in 0..120usize {
                let deg = match t % 4 {
                    0 => 2,
                    1 => 4,
                    2 => 4,
                    _ => 5,
                };
                let poly = if t % 3 == 0 {
                    let mut acc = vec![f.one];
                    for r_idx in 0..deg {
                        let root = if r_idx % 2 == 0 {
                            tree.xs[rng.gen_range(0..tree.len())]
                        } else {
                            rng.gen_range(0..p)
                        };
                        let lin = [f.neg(f.to_mont(root)), f.one];
                        let mut next = Vec::new();
                        crypto_lib::cryptanalysis::prime_fast::poly_mul_mont(&f, &acc, &lin, &mut next);
                        acc = next;
                    }
                    let lead = f.to_mont(rng.gen_range(1..p));
                    acc.into_iter().map(|c| f.from_mont(f.mul(c, lead))).collect()
                } else {
                    let mut c: Vec<u64> = (0..deg).map(|_| rng.gen_range(0..p)).collect();
                    c.push(rng.gen_range(1..p));
                    c
                };
                candidates.push(poly);
            }

            let mut scratch = SubproductBatchScratch::default();
            let batch_got = tree.roots_in_base_batch(&candidates, &mut scratch);

            for (idx, poly) in candidates.iter().enumerate() {
                let fe_poly: Vec<FieldElement> = poly
                    .iter()
                    .map(|&c| FieldElement::new(BigUint::from(c), p_big.clone()))
                    .collect();
                let mut fp_all: Vec<u64> = find_roots_fp_fast(&fe_poly, &p_big)
                    .into_iter()
                    .map(|e| e.value.to_u64_digits().first().copied().unwrap_or(0))
                    .collect();
                fp_all.sort_unstable();
                fp_all.dedup();

                let cz_all = find_roots_fp64_cz(&f, poly);
                for &r in &cz_all {
                    let rm = f.to_mont(r);
                    let mut val = 0u64;
                    for &c in poly.iter().rev() {
                        val = f.add(f.mul(val, rm), f.to_mont(c));
                    }
                    assert_eq!(val, 0, "cz returned non-root r={r} p={p}");
                }
                // `find_roots_fp_fast` returns a subset when >= 3 roots share a Legendre symbol
                // (due to its `h + c*X` equal-degree shift in `ec_index_calculus.rs`), and equals
                // `cz_all` whenever <= 2 roots exist in F_p.
                assert!(
                    fp_all.iter().all(|r| cz_all.contains(r)),
                    "fp_all not subset of cz_all: fp={fp_all:?} cz={cz_all:?}"
                );
                if cz_all.len() <= 2 {
                    assert_eq!(cz_all, fp_all, "find_roots_fp64_cz mismatch p={p} idx={idx}");
                }

                // Exact ground-truth roots in V via direct Horner evaluation over `tree.xs`:
                let want_in_v: Vec<u64> = tree
                    .xs
                    .iter()
                    .copied()
                    .filter(|&x| {
                        let xm = f.to_mont(x);
                        let mut val = 0u64;
                        for &c in poly.iter().rev() {
                            val = f.add(f.mul(val, xm), f.to_mont(c));
                        }
                        val == 0
                    })
                    .collect();
                let cz_in_v: Vec<u64> = cz_all
                    .into_iter()
                    .filter(|&r| tree.contains_x(r))
                    .collect();
                assert_eq!(cz_in_v, want_in_v, "cz_in_v != horner p={p} B={b_size} idx={idx}");
                let single_got = tree.roots_in_base(poly);
                assert_eq!(
                    single_got, want_in_v,
                    "roots_in_base mismatch p={p} B={b_size} idx={idx}"
                );
                assert_eq!(
                    batch_got[idx], want_in_v,
                    "roots_in_base_batch mismatch p={p} B={b_size} idx={idx}"
                );
            }
        }
    }
}

#[test]
fn p0_1b_lockstep_multi_walker_dlog_recovery_matches_negation_rho() {
    for bits in [16u32, 20, 24] {
        let (_, params) = bench_curves_a_minus_3()
            .into_iter()
            .find(|(b, c)| *b == bits && c.name.contains("p256class"))
            .unwrap();
        let fc = FastCurve::from_params(&params).unwrap();
        let g = fc.from_textbook(&params.generator()).unwrap();

        // Check existing table/sweep invariant remains 100% preserved!
        let q_inv = fc.mul(Some(g), 12345 % fc.n).unwrap();
        let (x_sw, rep_sw) = ic_solve(&fc, g, q_inv, 80, 12, 2, 20_000_000, 7).unwrap();
        let (x_tb, rep_tb) = ic_solve_table(&fc, g, q_inv, 80, 12, 20_000_000, 7).unwrap();
        assert_eq!(x_sw, x_tb);
        assert_eq!(rep_sw.targets, rep_tb.targets);

        let mut rng = StdRng::seed_from_u64(0xBEEF_0000 + bits as u64);
        let mut scratch = LockStepCollectScratch::default();
        let fb = FastFactorBase::build(&fc, 110);
        let pairs = PairTable::build(&fc, &fb);

        for trial in 0..3u64 {
            let secret_d = rng.gen_range(1..fc.n);
            let q = fc.mul(Some(g), secret_d).unwrap();

            let mut rho_cfg = RhoConfig::tuned(fc.n);
            rho_cfg.seed = 100 + trial;
            let rho_res = rho_parallel(&fc, g, q, &rho_cfg).expect("negation rho solves");
            assert_eq!(rho_res.log, secret_d);
            assert_eq!(fc.mul(Some(g), rho_res.log), Some(q));

            for &w in &[64usize, 128, 256] {
                let (d_api, _rep) = ic_solve_lockstep(
                    &fc,
                    g,
                    q,
                    110,
                    24,
                    w,
                    LockStepMode::PairTable2,
                    50_000_000,
                    2_000 + trial * 31 + w as u64,
                )
                .expect("ic_solve_lockstep PairTable2");
                assert_eq!(d_api, rho_res.log);
                assert_eq!(fc.mul(Some(g), d_api), Some(q));
            }
            if bits <= 20 && trial == 0 {
                let (d_sw_ls, _) = ic_solve_lockstep(
                    &fc,
                    g,
                    q,
                    90,
                    24,
                    128,
                    LockStepMode::DirectSweep,
                    50_000_000,
                    9_000 + bits as u64,
                )
                .expect("ic_solve_lockstep DirectSweep");
                assert_eq!(d_sw_ls, secret_d);
                assert_eq!(fc.mul(Some(g), d_sw_ls), Some(q));

                let (rels, stats) = collect_relations_lockstep(
                    &fc,
                    g,
                    q,
                    &fb,
                    Some(&pairs),
                    fb.len() + 28,
                    50_000_000,
                    128,
                    LockStepMode::PairTable2,
                    7_777,
                    &mut scratch,
                );
                assert_eq!(stats.rejected, 0);
                assert_eq!(rels.len(), fb.len() + 28);
            }
        }
    }
}

#[test]
fn p0_1b_repeated_roots_and_same_legendre_quartic_parity() {
    let p = 2_147_483_647u64;
    let f = Fp64::new(p);
    let base: Vec<u64> = (1..=256).collect();
    let tree = FactorBaseSubproductTree::prepare(f, &base);

    let mul_lin = |poly: &[u64], root: u64| -> Vec<u64> {
        let mut out = vec![0u64; poly.len() + 1];
        let neg_r = if root == 0 { 0 } else { p - (root % p) };
        for (i, &c) in poly.iter().enumerate() {
            out[i] = ((out[i] as u128 + (c as u128) * (neg_r as u128)) % p as u128) as u64;
            out[i + 1] = (out[i + 1] + c) % p;
        }
        out
    };

    // 1. Quartic whose 4 roots [1, 4, 9, 16] are ALL quadratic residues mod p:
    let mut q_qr = vec![1u64];
    for r in [1u64, 4, 9, 16] {
        q_qr = mul_lin(&q_qr, r);
    }
    // 2. Quartic with repeated roots (X - 7)^2 * (X - 19)^2:
    let mut q_rep = vec![1u64];
    for r in [7u64, 7, 19, 19] {
        q_rep = mul_lin(&q_rep, r);
    }
    // 3. Quartic with 1 root in base and 3 roots outside base [42, 500, 1000, 9999]:
    let mut q_mix = vec![1u64];
    for r in [42u64, 500, 1_000, 9_999] {
        q_mix = mul_lin(&q_mix, r);
    }

    assert_eq!(find_roots_fp64_cz(&f, &q_qr), vec![1, 4, 9, 16]);
    assert_eq!(find_roots_fp64_cz(&f, &q_rep), vec![7, 19]);
    assert_eq!(find_roots_fp64_cz(&f, &q_mix), vec![42, 500, 1_000, 9_999]);

    assert_eq!(tree.roots_in_base(&q_qr), vec![1, 4, 9, 16]);
    assert_eq!(tree.roots_in_base(&q_rep), vec![7, 19]);
    assert_eq!(tree.roots_in_base(&q_mix), vec![42]);

    let batch = vec![q_qr, q_rep, q_mix];
    let mut scratch = SubproductBatchScratch::default();
    let got = tree.roots_in_base_batch(&batch, &mut scratch);
    assert_eq!(got, vec![vec![1, 4, 9, 16], vec![7, 19], vec![42]]);
}
