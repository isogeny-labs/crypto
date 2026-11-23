use std::collections::BTreeSet;
use std::time::Duration;

use prime31_rr9_and_closures::cover_and_deuring::{
    run_part_a_cover_audit, run_part_b_deuring_rarity_audit,
};
use prime31_rr9_and_closures::p2_6_suite::{
    build_planted_corank2_b8_base, run_live_toy_non_enumerative_whole_solve,
};
use prime31_rr9_and_closures::prime31_field::{
    build_factor_base, negation_pollard_rho, CurveArith, NegationSharedBsgs, PRIME31,
};
use prime31_rr9_and_closures::rr9_remainder_tower::{
    extract_via_remainder_tower,
    grader_only::{CompleteFibreOracle, SignedMultiset},
    TowerMode,
};
use prime31_rr9_and_closures::rr9_wiedemann::{
    audit_exterior4_plucker_70, decode_and_verify_7chart_relation, forbidden_calls,
    reset_forbidden_counter, solve_4var_dense_elimination, solve_4var_sparse_wiedemann,
    Reduced4VarIdeal, WeightedMacaulay4, DEFAULT_WEIGHT_BOUND,
};

#[test]
fn test_prime31_curve_shared_bsgs_and_negation_rho() {
    let mut ar = CurveArith::default();
    assert!(ar.on_curve(PRIME31, Some(PRIME31.g)));
    assert_eq!(ar.scalar(PRIME31, PRIME31.n, Some(PRIME31.g)), None);

    let m = ((PRIME31.n as f64 / 2.0).sqrt().ceil()) as u64;
    let bsgs = NegationSharedBsgs::build(PRIME31, m);

    for (idx, d) in [1u64, 42, 123_456_789, PRIME31.n - 7].into_iter().enumerate() {
        let q = ar.scalar(PRIME31, d, Some(PRIME31.g));
        let b_rec = bsgs.solve(q).expect("BSGS solve");
        assert!(b_rec.verified);
        assert_eq!(b_rec.scalar, d);

        let r_rec = negation_pollard_rho(PRIME31, q, 0x7000 + idx as u64).expect("rho solve");
        assert!(r_rec.verified);
        assert_eq!(r_rec.scalar, d);
    }
}

#[test]
fn test_plucker_70_full_rank_and_4var_macaulay_compression() {
    let mut ar = CurveArith::default();
    let (base8, h8_vec) = build_factor_base(PRIME31, 8, &mut ar);
    let xs8: [u64; 8] = std::array::from_fn(|i| base8[i].0);
    let h8_arr: [u64; 9] = h8_vec.try_into().unwrap();
    let r_pt = base8
        .iter()
        .fold(None, |acc, &pt| ar.add(PRIME31, acc, Some(pt)))
        .unwrap();
    let neg2 = ar.scalar(PRIME31, 2, Some(PRIME31.g)).unwrap();

    let pl = audit_exterior4_plucker_70(
        PRIME31.p,
        PRIME31.a,
        PRIME31.b,
        &xs8,
        [r_pt, PRIME31.g, neg2],
    );
    assert_eq!(pl.exterior_dim, 70);
    assert_eq!(pl.positive_rank, 70);
    assert_eq!(pl.negative1_rank, 70);
    assert_eq!(pl.negative2_rank, 70);
    assert_eq!(pl.summary, "70/70/70");

    // C(7 + 9, 9) = C(16, 9) = 11,440 unreduced 7-var Macaulay columns at degree 9:
    let unred_cols: usize = (10..=16usize).product::<usize>() / (1..=7usize).product::<usize>();
    assert_eq!(unred_cols, 11_440);

    let ideal = Reduced4VarIdeal::build(PRIME31.p, PRIME31.a, r_pt.0, r_pt.1, &h8_arr);
    let mac41 = WeightedMacaulay4::build(&ideal, 41);
    let mac42 = WeightedMacaulay4::build(&ideal, DEFAULT_WEIGHT_BOUND);
    assert_eq!(mac41.basis.len(), 2_341);
    assert_eq!(mac42.basis.len(), 2_538);
}

#[test]
fn test_4var_wiedemann_dense_and_quotient_extractors_single_collision_and_unsat() {
    reset_forbidden_counter();
    let mut ar = CurveArith::default();
    let (base8, h8_vec) = build_factor_base(PRIME31, 8, &mut ar);
    let h8_arr: [u64; 9] = h8_vec.clone().try_into().unwrap();

    // 1. Single-solution positive + UNSAT on canonical base8:
    let mut pos_pt = None;
    for (i, &pt) in base8.iter().enumerate() {
        let s_pt = if i % 2 == 0 {
            Some(pt)
        } else {
            ar.neg(PRIME31, Some(pt))
        };
        pos_pt = ar.add(PRIME31, pos_pt, s_pt);
    }
    let pos_res = pos_pt.unwrap();

    let w_pos = solve_4var_sparse_wiedemann(
        PRIME31.p,
        PRIME31.a,
        pos_res,
        &h8_arr,
        DEFAULT_WEIGHT_BOUND,
        0x1119,
    );
    let d_pos = solve_4var_dense_elimination(
        PRIME31.p,
        PRIME31.a,
        pos_res,
        &h8_arr,
        DEFAULT_WEIGHT_BOUND,
    );
    let dq_pos = extract_via_remainder_tower(
        PRIME31.p,
        PRIME31.a,
        pos_res,
        &h8_vec,
        TowerMode::DirectCubicQuotient,
        8,
        Duration::from_secs(10),
    )
    .unwrap();
    let rq_pos = extract_via_remainder_tower(
        PRIME31.p,
        PRIME31.a,
        pos_res,
        &h8_vec,
        TowerMode::Reduced4VarQuotient,
        16,
        Duration::from_secs(10),
    )
    .unwrap();

    assert_eq!(w_pos.solutions.len(), 1);
    assert_eq!(w_pos.solutions, d_pos.solutions);
    assert_eq!(w_pos.solutions, dq_pos.chart_solutions);
    assert_eq!(w_pos.solutions, rq_pos.chart_solutions);
    let rel = decode_and_verify_7chart_relation(PRIME31, &base8, pos_res, &w_pos.solutions[0], &mut ar);
    assert!(rel.is_some());

    // 2. Planted `corank == 2` distinct-abscissa collision:
    let (col_base, col_h_arr, col_res) = build_planted_corank2_b8_base(&mut ar);
    let col_h_vec = col_h_arr.to_vec();

    let w_col = solve_4var_sparse_wiedemann(
        PRIME31.p,
        PRIME31.a,
        col_res,
        &col_h_arr,
        DEFAULT_WEIGHT_BOUND,
        0x2229,
    );
    let dq_col = extract_via_remainder_tower(
        PRIME31.p,
        PRIME31.a,
        col_res,
        &col_h_vec,
        TowerMode::DirectCubicQuotient,
        8,
        Duration::from_secs(10),
    )
    .unwrap();
    assert_eq!(w_col.kernel_dim_found, 2);
    assert_eq!(w_col.solutions.len(), 2);
    assert_eq!(dq_col.chart_solutions.len(), 2);
    let mut w_sorted = w_col.solutions.clone();
    w_sorted.sort_unstable();
    assert_eq!(w_sorted, dq_col.chart_solutions);
    let col_rels: BTreeSet<SignedMultiset> = w_sorted
        .iter()
        .map(|c7| decode_and_verify_7chart_relation(PRIME31, &col_base, col_res, c7, &mut ar).unwrap())
        .collect();
    assert_eq!(col_rels.len(), 2);

    // 3. UNSAT (`G`):
    let w_unsat = solve_4var_sparse_wiedemann(
        PRIME31.p,
        PRIME31.a,
        PRIME31.g,
        &h8_arr,
        DEFAULT_WEIGHT_BOUND,
        0x3339,
    );
    assert!(w_unsat.full_rank_unsat_certified);
    assert!(w_unsat.solutions.is_empty());
    assert_eq!(forbidden_calls(), 0);
}

#[test]
fn test_b4_yun_symmetric_quotient_tower_blinded_4way_collision_and_unsat() {
    reset_forbidden_counter();
    let mut ar = CurveArith::default();
    let (base4, h4_vec) = build_factor_base(PRIME31, 4, &mut ar);
    let oracle4 = CompleteFibreOracle::build_exhaustive(PRIME31, &base4);
    assert_eq!(oracle4.total_multisets, 6_435);

    let (r_col4, exp_col4) = oracle4
        .by_residual
        .iter()
        .find(|(_, set)| set.len() == 4)
        .expect("4-way collision fibre at B=4");

    let rec_col = extract_via_remainder_tower(
        PRIME31.p,
        PRIME31.a,
        *r_col4,
        &h4_vec,
        TowerMode::YunSymmetricQuotientTower,
        8,
        Duration::from_secs(90),
    )
    .unwrap();
    assert!(rec_col.complete_traversal);
    let dec_set: BTreeSet<SignedMultiset> = rec_col
        .chart_solutions
        .iter()
        .map(|c7| decode_and_verify_7chart_relation(PRIME31, &base4, *r_col4, c7, &mut ar).unwrap())
        .collect();
    assert_eq!(&dec_set, exp_col4);
    assert_eq!(forbidden_calls(), 0);
}

#[test]
fn test_live_end_to_end_non_enumerative_rr9_whole_solve() {
    let receipt = run_live_toy_non_enumerative_whole_solve();
    assert!(receipt.d_g_eq_q_verified);
    assert_eq!(receipt.recovered_scalar_d, receipt.planted_scalar_d);
    assert_eq!(receipt.independent_relations_collected, 9);
    assert!(receipt.unsat_walks_charged > 0);
    assert_eq!(receipt.forbidden_calls, 0);
    assert!(receipt.toy_bsgs_verified);
    assert!(receipt.toy_rho_verified);
}

#[test]
fn test_p192_auxiliary_cover_and_deuring_rarity_closures() {
    let (ladder, pic1009, p192) = run_part_a_cover_audit();
    for row in &ladder {
        assert!(row.pullback_hom_pass);
        assert!(row.pushforward_hom_pass);
        assert!(row.norm_comp_eq_mul2_pass);
        assert!(row.transport_conservation_pass);
        assert!(row.jac_divisible_by_r);
    }
    assert!(pic1009.smoothness_verified);
    assert!(pic1009.bsgs_match);
    assert!(p192.p192_generator_on_curve);
    assert!(p192.p192_prime_order_verified);
    assert!(p192.p192_gamma7_cover_norm_verified);
    assert!(p192.p192_gamma1_cover_norm_verified);
    assert!(p192.unweighted_step_gap_bits > 96.0);
    assert!(p192.mul_weighted_gap_bits > 104.0);

    let part_b = run_part_b_deuring_rarity_audit();
    assert!(part_b.direct_form_crosscheck_up_to_1000_pass);
    assert!(part_b.all_trace_identities_pass);
    assert!(part_b.zero_cap_violations);
    assert!(part_b.fit_r_squared > 0.95);
    assert!((part_b.fitted_constant_c - 0.845).abs() < 0.05);
}
