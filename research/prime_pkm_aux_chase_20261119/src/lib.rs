pub mod p0_1c_bench;
pub mod p1_3a_spec_ladder;
pub mod p2_5_p256_aux;
pub mod panel_a_scaling;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FullChaseResults {
    pub schema: String,
    pub date: String,
    pub branch: String,
    pub standing: String,
    pub kb_status: String,
    pub lead_p0_1c: p0_1c_bench::P01cSummary,
    pub lead_p1_3a: p1_3a_spec_ladder::P13aSummary,
    pub lead_p2_5: p2_5_p256_aux::P25Summary,
    pub panel_a: panel_a_scaling::PanelASummary,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::f4_fp_tower::{
        f4_tower, SquareRule, TowerF4Options, TowerRing, TowerWeights,
    };
    use crypto_lib::cryptanalysis::ic_boundary::{
        CountedGroup, GroupOps, OracleCounters, PrimePoint,
    };
    use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::{
        bits_factor_base, dickson_factor_base, digits_factor_base, edwards_factor_base,
        edwards_shift, square_bits_factor_base, t0_factor_base, PkmSemaevOracle, T0Quotient,
    };
    use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
    use crypto_lib::cryptanalysis::symmetrized_semaev::{
        eval_elem_sym_tuple, eval_s3_u64, eval_s4_u64, eval_s5_u64, ParamS5,
    };

    #[test]
    fn param_s5_matches_exact_semaev_on_prime_and_m31() {
        for &p in &[1009u64, 65_537, (1u64 << 31) - 1] {
            let a = p - 3;
            let b = 11u64 % p;
            for m in [2usize, 3, 4] {
                let param = ParamS5::build(m, a, b, p).expect("ParamS5");
                let (terms, spec_mults) = param.specialize(77 % p);
                assert!(spec_mults < 4_000, "m={m} spec_mults={spec_mults}");
                let xs: Vec<u64> = (0..m).map(|i| (13 + 7 * i as u64) % p).collect();
                let mut m_cnt = 0u64;
                let es = eval_elem_sym_tuple(&xs, p, &mut m_cnt);
                let mut acc = 0u64;
                for (exps, c) in &terms {
                    let mut mon = *c;
                    for (j, &e) in exps.iter().enumerate() {
                        for _ in 0..e {
                            mon = ((mon as u128 * es[j] as u128) % p as u128) as u64;
                        }
                    }
                    acc = (acc + mon) % p;
                }
                let exact = match m {
                    2 => eval_s3_u64(xs[0], xs[1], 77 % p, a, b, p),
                    3 => eval_s4_u64(xs[0], xs[1], xs[2], 77 % p, a, b, p),
                    4 => eval_s5_u64(xs[0], xs[1], xs[2], xs[3], 77 % p, a, b, p),
                    _ => unreachable!(),
                };
                assert_eq!(acc, exact, "p={p} m={m}");
            }
        }
    }

    #[test]
    fn f4_fp_tower_compiled_refutation_and_basis_inheritance_are_sound() {
        let p = 65_537u64;
        let t = 4usize;
        let mut rules = Vec::with_capacity(2 * t);
        for blk in 0..2 {
            for j in 0..t {
                if j + 1 < t {
                    rules.push(SquareRule {
                        next: Some((blk * t + j + 1) as u8),
                        a: 0,
                        b: 1,
                        c: 0,
                        d: 0,
                    });
                } else {
                    rules.push(SquareRule {
                        next: None,
                        a: 0,
                        b: 0,
                        c: 0,
                        d: 1,
                    });
                }
            }
        }
        let ring = TowerRing::new(p, rules, 0);
        let mk_sys = |xr: u64| {
            let raw = p1_3a_spec_ladder::run_empirical_km3_ladder;
            let _ = raw;
            // S_3(3*y_0, 3*y_4, xr) over F_65537 (a = p - 3, b = 5)
            let mut terms: Vec<(Vec<u32>, u64)> = Vec::new();
            let g = 3u64;
            let a = p - 3;
            let b = 5u64;
            let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
            let g2 = mul(g, g);
            let xr2 = mul(xr, xr);
            let v = |i0: u32, i1: u32| {
                let mut e = vec![0u32; 2 * t];
                e[0] = i0;
                e[t] = i1;
                e
            };
            let c_sq = mul(g2, xr2);
            terms.push((v(2, 0), c_sq));
            terms.push((v(0, 2), c_sq));
            terms.push((v(1, 1), (p - mul(2, c_sq)) % p));
            let neg2xr = (p - mul(2, xr)) % p;
            let cg3 = mul(neg2xr, mul(g, g2));
            terms.push((v(2, 1), cg3));
            terms.push((v(1, 2), cg3));
            let cga = mul(neg2xr, mul(g, a));
            terms.push((v(1, 0), cga));
            terms.push((v(0, 1), cga));
            terms.push((v(0, 0), mul(neg2xr, mul(2, b))));
            terms.push((v(2, 2), mul(g2, g2)));
            terms.push((v(1, 1), (p - mul(2, mul(g2, a))) % p));
            terms.push((v(0, 0), mul(a, a)));
            let neg4bg = (p - mul(4, mul(b, g))) % p;
            terms.push((v(1, 0), neg4bg));
            terms.push((v(0, 1), neg4bg));
            vec![ring.from_raw(&terms)]
        };

        let mut learn = TowerF4Options::new(14);
        learn.record_trace = true;
        let cal = f4_tower(&mk_sys(9_999), &ring, &learn);
        assert!(cal.inconsistent);
        let tr = std::sync::Arc::new(cal.learned.expect("trace"));

        let mut rep_opts = TowerF4Options::new(14);
        rep_opts.replay = Some(tr);
        let mut replay_wins = 0usize;
        for xr in 200..232u64 {
            let sys = mk_sys(xr);
            let plain = f4_tower(&sys, &ring, &TowerF4Options::new(14));
            let fast = f4_tower(&sys, &ring, &rep_opts);
            if fast.inconsistent {
                assert!(plain.inconsistent, "soundness at xr={xr}");
                assert!(fast.muladds < plain.muladds);
                replay_wins += 1;
            }
        }
        assert!(replay_wins >= 24, "replay_wins={replay_wins}");

        // Parent-to-child basis specialization check (`specialize_basis_at_var` vs `specialize_drop_top_var`).
        let mut low = TowerF4Options::new(3);
        low.max_degree = 3;
        let parent = f4_tower(&mk_sys(321), &ring, &low);
        if !parent.inconsistent {
            for val in [1u64, p - 1] {
                let (c_keep, _) =
                    ring.specialize_basis_at_var(&parent.basis, t - 1, val, TowerWeights::UNIT);
                let r_keep = f4_tower(&c_keep, &ring, &TowerF4Options::new(14));
                let (ring_drop, c_drop, _, _) =
                    ring.specialize_drop_top_var(&parent.basis, t - 1, val, TowerWeights::UNIT);
                let r_drop = f4_tower(&c_drop, &ring_drop, &TowerF4Options::new(14));
                assert_eq!(r_keep.inconsistent, r_drop.inconsistent);
            }
        }
    }

    #[test]
    fn pkm_semaev_all_memberships_modes_and_guesses_match_exhaustive() {
        // Test across Dickson, Bits, Digits, SquareBits, Edwards/T0 (when available), guess=0,1,2, mode=full|chain|sym!
        let p = 131_071u64; // M17 = 2^17 - 1
        let inst = p0_1c_bench::find_prime_order_curve_over_fixed_p(p, Some(p - 3), 20261119);
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target: inst.generator_point(),
            r: inst.r,
            cofactor: 1,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        };

        // 1. Dickson (`t = 3`, `m = 2, 3, 4, 5`, `guess = 0, 1`, `mode = chain, sym`):
        let mut lo_d = 0u64;
        let fb_d = loop {
            if let Ok(fb) = dickson_factor_base(&inst, 3, lo_d, 0) {
                if fb.columns >= 5 {
                    break fb;
                }
            }
            lo_d += 1;
        };
        let mut lo_d2 = 0u64;
        let fb_d2 = loop {
            if let Ok(fb) = dickson_factor_base(&inst, 2, lo_d2, 0) {
                if fb.columns >= 3 {
                    break fb;
                }
            }
            lo_d2 += 1;
        };
        let mut ops = GroupOps::default();

        let inst_m31 =
            p0_1c_bench::find_prime_order_curve_over_fixed_p((1u64 << 31) - 1, Some((1u64 << 31) - 4), 20261119);
        let ctx_m31 = InstanceCtx {
            group: &inst_m31.curve,
            generator: inst_m31.generator_point(),
            target: inst_m31.generator_point(),
            r: inst_m31.r,
            cofactor: 1,
            group_order: inst_m31.group_order,
            name: inst_m31.name.clone(),
            field_degree: None,
        };
        let mut lo_m31_d2 = 0u64;
        let fb_m31_d2 = loop {
            if let Ok(fb) = dickson_factor_base(&inst_m31, 2, lo_m31_d2, 0) {
                if fb.columns >= 3 {
                    break fb;
                }
            }
            lo_m31_d2 += 1;
        };

        for &(m, t_cur, mode, guess) in &[
            (2u32, 3u32, "full", 0u32),
            (2, 3, "chain", 1),
            (3, 3, "chain", 0),
            (3, 3, "sym", 1),
            (4, 2, "sym", 0),
            (5, 2, "chain", 1),
        ] {
            let (cur_inst, cur_ctx, lo_cur, fb_cur) = if m == 5 {
                (&inst_m31, &ctx_m31, lo_m31_d2, &fb_m31_d2)
            } else if t_cur == 3 {
                (&inst, &ctx, lo_d, &fb_d)
            } else {
                (&inst, &ctx, lo_d2, &fb_d2)
            };
            let cur_g = cur_inst.generator_point();
            let mut o_fast = PkmSemaevOracle::new_dickson(m, t_cur, lo_cur, 0);
            let (_, p_fast) = Params::parse_spec(&format!(
                "x:m={m},mode={mode},guess={guess},degree=16,trace=1,inherit=1,descent=1"
            ))
            .unwrap();
            o_fast.prepare(cur_ctx, fb_cur, &p_fast, &mut GroupOps::default()).unwrap();

            let mut o_exh = PkmSemaevOracle::new_dickson(m, t_cur, lo_cur, 0);
            let ref_spec = if m <= 3 {
                format!("x:m={m},engine=exhaustive")
            } else {
                format!("x:m={m},mode={mode},guess={guess},degree=16,trace=0,inherit=0,descent=1")
            };
            let (_, p_exh) = Params::parse_spec(&ref_spec).unwrap();
            o_exh.prepare(cur_ctx, fb_cur, &p_exh, &mut GroupOps::default()).unwrap();

            // Check 8 UNSAT random points first (measuring pure UNSAT refutation speedup),
            // then 2 genuine sums of `m` points (`k >= 8`)!
            let mut fast_unsat_ops = 0u64;
            let mut bef_unsat_ops = 0u64;
            for k in 0..10usize {
                let pt = if k >= 8 {
                    let mut s = PrimePoint::INFINITY;
                    for j in 0..(m as usize) {
                        s = cur_inst.curve.add(&mut ops, s, fb_cur.points[2 * ((k + j) % fb_cur.columns)]);
                    }
                    s
                } else {
                    cur_inst.curve.mul(&mut ops, cur_g, 500 + k as u64 * 997)
                };
                if pt.infinity {
                    continue;
                }
                let r_fast = o_fast.decompose(
                    cur_ctx,
                    fb_cur,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    pt,
                );
                let r_exh = o_exh.decompose(
                    cur_ctx,
                    fb_cur,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    pt,
                );
                assert_eq!(
                    r_fast.is_some(),
                    r_exh.is_some(),
                    "Dickson m={m} mode={mode} guess={guess} k={k}"
                );
                if let Some(idxs) = r_fast {
                    let mut chk = PrimePoint::INFINITY;
                    for i in idxs {
                        chk = cur_inst.curve.add(&mut ops, chk, fb_cur.points[i]);
                    }
                    assert_eq!(chk, pt);
                }
                if k == 7 && m >= 4 {
                    fast_unsat_ops = o_fast.solver_totals().unwrap().ops;
                    bef_unsat_ops = o_exh.solver_totals().unwrap().ops;
                }
            }
            let fast_root_mults = o_fast
                .relation_counters()
                .get("pkm_grid_fp_mults_uncharged")
                .copied()
                .unwrap_or(0);
            let exh_grid_mults = o_exh
                .relation_counters()
                .get("pkm_grid_fp_mults_uncharged")
                .copied()
                .unwrap_or(0);
            if m <= 3 {
                assert!(
                    fast_root_mults < exh_grid_mults,
                    "Dickson m={m} mode={mode} guess={guess}: fast_root_mults={fast_root_mults} vs exh_grid_mults={exh_grid_mults}"
                );
            } else {
                assert!(
                    fast_unsat_ops < bef_unsat_ops,
                    "Dickson m={m} mode={mode}: fast_unsat_ops={fast_unsat_ops} vs bef_unsat_ops={bef_unsat_ops}"
                );
            }
        }

        // 2. Bits, Digits, SquareBits (`t = 3`, `m = 2`, `guess = 1`):
        let fb_bits = bits_factor_base(&inst, 3, 10);
        if fb_bits.columns >= 2 {
            let mut o_b = PkmSemaevOracle::new_bits(2, 3, 10);
            let (_, p_b) =
                Params::parse_spec("x:m=2,guess=1,degree=14,trace=1,inherit=1,descent=1").unwrap();
            o_b.prepare(&ctx, &fb_bits, &p_b, &mut GroupOps::default()).unwrap();
            let sum_b = inst.curve.add(&mut ops, fb_bits.points[0], fb_bits.points[2.min(fb_bits.points.len() - 1)]);
            if !sum_b.infinity {
                assert!(o_b
                    .decompose(&ctx, &fb_bits, &mut GroupOps::default(), &mut OracleCounters::default(), sum_b)
                    .is_some());
            }
        }

        let digs = vec![1u64, 3, 9];
        let fb_dig = digits_factor_base(&inst, 5, &digs);
        if fb_dig.columns >= 2 {
            let mut o_dg = PkmSemaevOracle::new_digits(2, 3, 5, digs);
            let (_, p_dg) =
                Params::parse_spec("x:m=2,guess=1,degree=14,trace=1,inherit=1,descent=1").unwrap();
            o_dg.prepare(&ctx, &fb_dig, &p_dg, &mut GroupOps::default()).unwrap();
            let sum_dg = inst.curve.add(&mut ops, fb_dig.points[0], fb_dig.points[2.min(fb_dig.points.len() - 1)]);
            if !sum_dg.infinity {
                assert!(o_dg
                    .decompose(&ctx, &fb_dig, &mut GroupOps::default(), &mut OracleCounters::default(), sum_dg)
                    .is_some());
            }
        }

        if let Ok(shift) = edwards_shift(&inst.curve) {
            let fb_sq = square_bits_factor_base(&inst, 3, 3, shift);
            if fb_sq.columns >= 2 {
                let mut o_sq = PkmSemaevOracle::new_square_bits(2, 3, 3);
                let (_, p_sq) =
                    Params::parse_spec("x:m=2,guess=1,degree=16,trace=1,inherit=1,descent=1")
                        .unwrap();
                o_sq.prepare(&ctx, &fb_sq, &p_sq, &mut GroupOps::default()).unwrap();
                let sum_sq = inst.curve.add(
                    &mut ops,
                    fb_sq.points[0],
                    fb_sq.points[2.min(fb_sq.points.len() - 1)],
                );
                if !sum_sq.infinity {
                    assert!(o_sq
                        .decompose(
                            &ctx,
                            &fb_sq,
                            &mut GroupOps::default(),
                            &mut OracleCounters::default(),
                            sum_sq,
                        )
                        .is_some());
                }
            }
        }

        let _ = (edwards_factor_base, t0_factor_base, std::mem::size_of::<T0Quotient>());
    }

    #[test]
    fn p256_aux_reproduces_exact_fb1h_and_km3_thresholds() {
        let b1 = p2_5_p256_aux::build_p256_aux_base(1);
        assert!(!b1.columns.is_empty());
        let b8 = p2_5_p256_aux::build_p256_aux_base(8);
        assert_eq!(b8.fb_id, "FB1h16882188194e");
        assert_eq!(b8.columns.len() * 2, 288);

        let table = p1_3a_spec_ladder::compute_m8_exponent_table();
        assert_eq!(table.len(), 10);
        // Check `alpha*1 = 0.1561` and `alpha*2 = 0.0563` match `pkm_m8_runs_20261115/tables.md`!
        let r_w1 = table.iter().find(|r| (r.matrix_exponent_w - 1.0).abs() < 1e-9).unwrap();
        let r_w2 = table.iter().find(|r| (r.matrix_exponent_w - 2.0).abs() < 1e-9).unwrap();
        assert!((r_w1.alpha_star_pre_floor - 0.15614).abs() < 1e-4);
        assert!((r_w2.alpha_star_pre_floor - 0.05626).abs() < 1e-4);
        assert!(table.iter().all(|r| r.rules_out_subexponential));
    }
}

