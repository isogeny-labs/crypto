//! Complete End-to-End Verification, Scaling, and Whole-Solve Comparator Suite for Lead P2-6:
//! 1. Exterior-4 Plücker `70/70/70` residual-blind rank audit;
//! 2. Sharp `(7,5,3,1)`-weighted Macaulay threshold scan + 4-variable Sparse Wiedemann vs
//!    4-variable Dense Elimination vs `DirectCubicQuotient` & `Reduced4VarQuotient` (on single,
//!    UNSAT, and planted `corank = 2` distinct-abscissa collision fibres);
//! 3. Blinded `B = 4` (`YunSymmetricQuotientTower`) and `B = 8` (`DirectCubicQuotient`,
//!    `Reduced4VarQuotient`, `4-var Dense`, `4-var Sparse Wiedemann`) complete-fibre grader
//!    controls (`0` forbidden tuple-enumeration calls, `100%` set-level match);
//! 4. Scaling across `B = {8, 12, 16, 24, 31}` on `PRIME31` (`p = 2^{31}-1`) with exact relation
//!    hit probabilities and work-charged UNSAT relation-collection costs;
//! 5. End-to-end whole-solve ECDLP recovery (`[d]G == Q`) head-to-head against native
//!    `NegationSharedBsgs` and matched `negation_pollard_rho` (both on a live prime-order control
//!    curve where 100% non-enumerative `RR9` closes `[d]G == Q` end-to-end, AND across 8 planted
//!    challenges on `PRIME31`!).

use crate::prime31_field::{
    build_factor_base, fp_add, fp_mul, fp_neg, fp_sub, negation_pollard_rho, poly_mul, splitmix64,
    Curve, CurveArith, ModLinearSystem, NegationSharedBsgs, Point, PRIME31,
};
use crate::rr9_remainder_tower::{
    extract_via_remainder_tower,
    grader_only::{CompleteFibreOracle, SignedMultiset},
    reset_and_assert_zero_forbidden, TowerExtractReceipt, TowerMode,
};
use crate::rr9_wiedemann::{
    audit_exterior4_plucker_70, decode_and_verify_7chart_relation, forbidden_calls,
    solve_4var_dense_elimination, solve_4var_sparse_wiedemann, Dense4VarReceipt,
    PluckerAuditReceipt, WiedemannReceipt, DEFAULT_WEIGHT_BOUND,
};
use crate::residual_walk::is_prime_u64;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeightThresholdRow {
    pub weight_bound: u16,
    pub rows: usize,
    pub cols: usize,
    pub nnz: usize,
    pub pos_corank: usize,
    pub pos_extracted: bool,
    pub neg_corank: usize,
    pub neg_unit_certified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtractorHeadToHeadRow {
    pub case_name: String,
    pub expected_fibre_size: usize,
    pub wiedemann: WiedemannReceipt,
    pub dense_4var: Dense4VarReceipt,
    pub direct_quotient_f4: TowerExtractReceipt,
    pub reduced_4var_f4: TowerExtractReceipt,
    pub all_four_exact_set_match: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlindedControlCase {
    pub label: String,
    pub b_size: usize,
    pub extractor: String,
    pub expected_fibre_count: usize,
    pub decoded_fibre_count: usize,
    pub exact_set_agreement: bool,
    pub complete_traversal: bool,
    pub forbidden_calls_delta: u64,
    pub field_ops_or_muls: u64,
    pub wall_ms: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlindedGraderSummary {
    pub b4_oracle_total_multisets: usize,
    pub b4_oracle_distinct_residuals: usize,
    pub b8_oracle_distinct_abscissa_tuples: usize,
    pub b8_oracle_distinct_abscissa_residuals: usize,
    pub cases: Vec<BlindedControlCase>,
    pub b8_batch16_exact_matches: usize,
    pub b8_batch16_total: usize,
    pub total_forbidden_calls: u64,
    pub all_controls_pass: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScalingRowB {
    pub b_size: usize,
    pub distinct_8tuples_count: u128,
    pub multiset_8tuples_count: u128,
    pub hit_prob_distinct: f64,
    pub expected_walks_per_hit: f64,
    pub relations_needed_b_plus_1: usize,
    pub expected_total_calls_charged: f64,
    pub direct_quotient_vars: usize,
    pub direct_quotient_eqs: usize,
    pub direct_quotient_pos_status: String,
    pub direct_quotient_unsat_status: String,
    pub direct_quotient_max_cols: usize,
    pub direct_quotient_solving_deg: u32,
    pub direct_quotient_unsat_ops: u64,
    pub direct_quotient_unsat_ms: f64,
    pub reduced_4var_vars: usize,
    pub reduced_4var_unsat_status: String,
    pub reduced_4var_max_cols: usize,
    pub reduced_4var_unsat_ops: u64,
    pub reduced_4var_unsat_ms: f64,
    pub frozen_pr4692_vars: usize,
    pub frozen_pr4692_status: String,
    pub frozen_pr4692_max_cols: usize,
    pub whole_solve_charged_fp_ops_lower_bound: f64,
    pub ratio_vs_prime31_negation_rho_muls: f64,
    pub ratio_vs_prime31_shared_bsgs_online_muls: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToyEndToEndRr9Receipt {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub order_r: u64,
    pub b_size: usize,
    pub planted_scalar_d: u64,
    pub recovered_scalar_d: u64,
    pub d_g_eq_q_verified: bool,
    pub total_random_walks_charged: usize,
    pub unsat_walks_charged: usize,
    pub sat_walks: usize,
    pub independent_relations_collected: usize,
    pub forbidden_calls: u64,
    pub rr9_total_fp_ops_charged: u64,
    pub rr9_wall_ms: f64,
    pub toy_bsgs_cold_plus_online_muls: u64,
    pub toy_bsgs_verified: bool,
    pub toy_rho_muls: u64,
    pub toy_rho_verified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prime31WholeSolveSummary {
    pub num_challenges: usize,
    pub planted_scalars: Vec<u64>,
    pub bsgs_m: u64,
    pub bsgs_all_verified: bool,
    pub bsgs_setup_group_adds: u64,
    pub bsgs_setup_fp_muls: u64,
    pub bsgs_setup_wall_ms: f64,
    pub bsgs_mean_online_giant_steps: f64,
    pub bsgs_mean_online_fp_muls: f64,
    pub bsgs_mean_online_wall_ms: f64,
    pub bsgs_cold_single_solve_fp_muls: f64,
    pub bsgs_cold_single_solve_wall_ms: f64,
    pub rho_all_verified: bool,
    pub rho_theoretical_sqrt_pi_n_over_4: f64,
    pub rho_mean_walk_steps: f64,
    pub rho_step_ratio_to_theory: f64,
    pub rho_mean_fp_muls: f64,
    pub rho_mean_wall_ms: f64,
    pub live_rr9_random_walk_trials_b8: usize,
    pub live_rr9_direct_mean_unsat_fp_ops: f64,
    pub live_rr9_direct_mean_unsat_wall_ms: f64,
    pub live_rr9_wiedemann_mean_unsat_fp_muls: f64,
    pub live_rr9_dense4_mean_unsat_fp_muls: f64,
    pub rr9_b8_expected_whole_solve_fp_ops_direct: f64,
    pub rr9_b8_expected_whole_solve_fp_muls_wiedemann: f64,
    pub rr9_b8_expected_whole_solve_fp_muls_dense4: f64,
    pub ratio_rr9_direct_vs_rho_muls: f64,
    pub ratio_rr9_direct_vs_bsgs_cold_muls: f64,
    pub ratio_rr9_direct_vs_bsgs_online_muls: f64,
    pub ratio_rr9_wiedemann_vs_rho_muls: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeadP26FullReport {
    pub plucker_audit: PluckerAuditReceipt,
    pub unreduced_7var_d9_cols: usize,
    pub reduced_4var_w42_cols: usize,
    pub macaulay_col_compression_ratio: f64,
    pub weight_threshold_scan: Vec<WeightThresholdRow>,
    pub b8_head_to_head: Vec<ExtractorHeadToHeadRow>,
    pub blinded_grader_controls: BlindedGraderSummary,
    pub prime31_whole_solve: Prime31WholeSolveSummary,
    pub b_scaling_prime31: Vec<ScalingRowB>,
    pub toy_end_to_end_rr9_whole_solve: ToyEndToEndRr9Receipt,
}

fn binom_u128(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut num = 1u128;
    for i in 0..k {
        num = num * ((n - i) as u128) / ((i + 1) as u128);
    }
    num
}

/// Construct a blinded `B = 8` factor base on `PRIME31` possessing a genuine `corank = 2`
/// distinct-abscissa collision: `P_3 = -(P_0 + P_1 + P_2)` with all 8 x-coordinates distinct,
/// so `(+P_0 + P_1 + P_2 + P_3) + P_4 + P_5 + P_6 + P_7 == (-P_0 - P_1 - P_2 - P_3) + P_4 + P_5 + P_6 + P_7 == R`!
pub fn build_planted_corank2_b8_base(ar: &mut CurveArith) -> (Vec<(u64, u64)>, [u64; 9], (u64, u64)) {
    let (mut base, _) = build_factor_base(PRIME31, 9, ar);
    let sum01 = ar.add(PRIME31, Some(base[0]), Some(base[1]));
    let sum012 = ar.add(PRIME31, sum01, Some(base[2]));
    let p3 = ar.neg(PRIME31, sum012).expect("non-identity -(P0+P1+P2)");
    // Replace index 3 with `p3` and pick 4 more points from `base` with pairwise-distinct abscissas:
    let mut chosen = vec![base[0], base[1], base[2], p3];
    for cand in base.drain(3..) {
        if chosen.len() == 8 {
            break;
        }
        if chosen.iter().all(|&(x, _)| x != cand.0) {
            chosen.push(cand);
        }
    }
    assert_eq!(chosen.len(), 8);
    let mut h_vec = vec![1u64];
    for &(x, _) in &chosen {
        h_vec = poly_mul(&h_vec, &[fp_neg(x, PRIME31.p), 1], PRIME31.p);
    }
    let h8: [u64; 9] = h_vec.try_into().unwrap();
    let mut res = None;
    for pt in &chosen[4..8] {
        res = ar.add(PRIME31, res, Some(*pt));
    }
    (chosen, h8, res.expect("non-identity collision residual"))
}

/// Find a deterministic prime-order elliptic curve over `F_257` (`E: y^2 = x^3 + a x + b`, prime `#E = r`)
/// and run a 100% live non-enumerative `RR9` (`DirectCubicQuotient` -> jet decode -> `ModLinearSystem` mod `r`)
/// end-to-end whole-solve recovering planted `d` with `[d]G == Q` and ALL failed/UNSAT random walks charged!
pub fn run_live_toy_non_enumerative_whole_solve() -> ToyEndToEndRr9Receipt {
    let p = 257u64;
    let mut found_curve = None;
    'search: for a in 1..20u64 {
        for b in 1..50u64 {
            // Discriminant `4 a^3 + 27 b^2 != 0 mod p`:
            let disc = fp_add(
                fp_mul(4, fp_mul(a, fp_mul(a, a, p), p), p),
                fp_mul(27, fp_mul(b, b, p), p),
                p,
            );
            if disc == 0 {
                continue;
            }
            let mut pts = Vec::new();
            for x in 0..p {
                let rhs = fp_add(fp_add(fp_mul(x, fp_mul(x, x, p), p), fp_mul(a, x, p), p), b, p);
                if rhs == 0 {
                    pts.push((x, 0));
                } else if let Some(y) = crate::prime31_field::fp_sqrt(rhs, p) {
                    pts.push((x, y));
                    pts.push((x, p - y));
                }
            }
            let r_ord = (pts.len() + 1) as u64;
            if r_ord >= 241 && is_prime_u64(r_ord) {
                let g = pts[0];
                let c = Curve {
                    name: "toy_prime_257",
                    p,
                    a,
                    b,
                    n: r_ord,
                    g,
                };
                let mut ar = CurveArith::default();
                if ar.verify_curve(c).is_ok() {
                    found_curve = Some(c);
                    break 'search;
                }
            }
        }
    }
    let curve = found_curve.expect("prime-order curve over F_257");
    let mut ar = CurveArith::default();
    let b_size = 8usize;

    // Build deterministic factor base `F_8` from x-coordinate scan (`build_factor_base` does NOT know discrete logs of `F_8`!):
    let (base8, h8_vec) = build_factor_base(curve, b_size, &mut ar);
    let planted_d = 173u64 % curve.n;
    let q_target = ar.scalar(curve, planted_d, Some(curve.g));

    let before_forb = forbidden_calls();
    let t0 = Instant::now();

    // Unknown variables in `ModLinearSystem`: `x_0..x_{B-1}` = discrete logs `log_G(P_i)` (`i = 0..8`),
    // and `x_B` = target discrete log `d = log_G(Q)` (`9` unknowns total, solved purely from relations
    // `\sum_{i=0}^7 c_i P_i == u G + v Q  =>  \sum_{i=0}^7 c_i log_G(P_i) - v d == u (mod r)`!).
    let mut linsys = ModLinearSystem::new(b_size + 1, curve.n);
    let mut rng = 0x2026_1119_0257_0008u64;
    let mut total_walks = 0usize;
    let mut unsat_walks = 0usize;
    let mut sat_walks = 0usize;
    let mut total_rr9_ops = 0u64;

    while linsys.rank < b_size + 1 {
        let u = 1 + splitmix64(&mut rng) % (curve.n - 1);
        let v = 1 + splitmix64(&mut rng) % (curve.n - 1);
        let ug = ar.scalar(curve, u, Some(curve.g));
        let vq = ar.scalar(curve, v, q_target);
        let Some(res_pt) = ar.add(curve, ug, vq) else {
            continue;
        };
        // Avoid `x(R) in F_8` just to keep generic distinct-abscissa decoding simple (still charge if skipped? No, test directly!):
        total_walks += 1;
        let rec = extract_via_remainder_tower(
            curve.p,
            curve.a,
            res_pt,
            &h8_vec,
            TowerMode::DirectCubicQuotient,
            8,
            Duration::from_secs(5),
        )
        .expect("toy DirectCubicQuotient");
        total_rr9_ops += rec.f4_field_ops + rec.root_field_muls;

        if rec.chart_solutions.is_empty() {
            unsat_walks += 1;
            continue;
        }
        sat_walks += 1;
        for c7 in &rec.chart_solutions {
            if let Some(ms) = decode_and_verify_7chart_relation(curve, &base8, res_pt, c7, &mut ar)
            {
                // Build linear equation `\sum c_i log_G(P_i) + (-v) d == u (mod r)`:
                let mut row = vec![0u64; b_size + 2];
                for (bi, sgn, mult) in ms {
                    let term = (mult as u64) % curve.n;
                    if sgn > 0 {
                        row[bi] = fp_add(row[bi], term, curve.n);
                    } else {
                        row[bi] = fp_sub(row[bi], term, curve.n);
                    }
                }
                row[b_size] = fp_neg(v, curve.n);
                row[b_size + 1] = u % curve.n;
                let _ = linsys.insert(row, &mut ar).expect("consistent relation");
                if linsys.rank == b_size + 1 {
                    break;
                }
            }
        }
    }

    let rr9_wall_ms = t0.elapsed().as_secs_f64() * 1e3;
    let delta_forb = forbidden_calls() - before_forb;
    assert_eq!(delta_forb, 0, "zero forbidden calls in live whole-solve");

    let sol_vec = linsys.solve(&mut ar).expect("full rank B+1 solution");
    let recovered_d = sol_vec[b_size];
    let verified = ar.scalar(curve, recovered_d, Some(curve.g)) == q_target;
    assert_eq!(recovered_d, planted_d);
    assert!(verified);

    // Compare against `NegationSharedBsgs` and `negation_pollard_rho` on the exact same curve & target:
    let m_toy = ((curve.n as f64 / 2.0).sqrt().ceil() as u64).max(1);
    let bsgs_toy = NegationSharedBsgs::build(curve, m_toy);
    let bsgs_q = bsgs_toy.solve(q_target).expect("toy BSGS");
    assert_eq!(bsgs_q.scalar, planted_d);

    let rho_q = negation_pollard_rho(curve, q_target, 0x1234_5678).expect("toy rho");
    assert_eq!(rho_q.scalar, planted_d);

    ToyEndToEndRr9Receipt {
        p: curve.p,
        a: curve.a,
        b: curve.b,
        order_r: curve.n,
        b_size,
        planted_scalar_d: planted_d,
        recovered_scalar_d: recovered_d,
        d_g_eq_q_verified: verified,
        total_random_walks_charged: total_walks,
        unsat_walks_charged: unsat_walks,
        sat_walks,
        independent_relations_collected: linsys.rank,
        forbidden_calls: delta_forb,
        rr9_total_fp_ops_charged: total_rr9_ops,
        rr9_wall_ms,
        toy_bsgs_cold_plus_online_muls: bsgs_toy.setup_counts.fp_muls + bsgs_q.online_counts.fp_muls,
        toy_bsgs_verified: bsgs_q.verified,
        toy_rho_muls: rho_q.counts.fp_muls,
        toy_rho_verified: rho_q.verified,
    }
}

pub fn run_lead_p2_6_suite() -> LeadP26FullReport {
    reset_and_assert_zero_forbidden();
    let mut ar = CurveArith::default();
    ar.verify_curve(PRIME31).expect("canonical PRIME31");

    // 1. Canonical B=8 factor base & residuals:
    let (base8, h8_vec) = build_factor_base(PRIME31, 8, &mut ar);
    let h8_arr: [u64; 9] = h8_vec.clone().try_into().unwrap();
    let mut xs8 = [0u64; 8];
    for (i, &(x, _)) in base8.iter().enumerate() {
        xs8[i] = x;
    }
    let mut pos_pt = None;
    for &pt in &base8 {
        pos_pt = ar.add(PRIME31, pos_pt, Some(pt));
    }
    let pos_res = pos_pt.unwrap();
    let neg1_res = PRIME31.g;
    let neg2_res = ar.scalar(PRIME31, 2, Some(PRIME31.g)).unwrap();

    // Part 1: Plücker 70/70/70 rank audit:
    let plucker_audit = audit_exterior4_plucker_70(
        PRIME31.p,
        PRIME31.a,
        PRIME31.b,
        &xs8,
        [pos_res, neg1_res, neg2_res],
    );
    assert_eq!(plucker_audit.positive_rank, 70);
    assert_eq!(plucker_audit.negative1_rank, 70);
    assert_eq!(plucker_audit.negative2_rank, 70);

    // Part 2a: Sharp `(7,5,3,1)`-weighted Macaulay threshold scan (`W = 36, 39, 40, 41, 42`):
    let mut weight_scan = Vec::new();
    for w_test in [36u16, 39, 40, 41, DEFAULT_WEIGHT_BOUND] {
        let d_pos = solve_4var_dense_elimination(PRIME31.p, PRIME31.a, pos_res, &h8_arr, w_test);
        let d_neg = solve_4var_dense_elimination(PRIME31.p, PRIME31.a, neg1_res, &h8_arr, w_test);
        weight_scan.push(WeightThresholdRow {
            weight_bound: w_test,
            rows: d_pos.rows,
            cols: d_pos.cols,
            nnz: d_pos.nnz,
            pos_corank: d_pos.corank,
            pos_extracted: !d_pos.solutions.is_empty(),
            neg_corank: d_neg.corank,
            neg_unit_certified: d_neg.unit_pivot_found && d_neg.corank == 0,
        });
    }

    // Part 2b: Head-to-head on `pos`, `neg1`, `neg2`, and blinded `corank = 2` collision fibre:
    let (col_base8, col_h8_arr, col_res) = build_planted_corank2_b8_base(&mut ar);
    let col_h8_vec = col_h8_arr.to_vec();
    let col_oracle = CompleteFibreOracle::build_exhaustive(PRIME31, &col_base8);
    let expected_col_set = col_oracle
        .distinct_abscissa_by_residual
        .get(&col_res)
        .expect("planted corank=2 collision fibre")
        .clone();
    assert_eq!(expected_col_set.len(), 2);

    let oracle8 = CompleteFibreOracle::build_exhaustive(PRIME31, &base8);
    let expected_pos_set = oracle8
        .distinct_abscissa_by_residual
        .get(&pos_res)
        .expect("planted pos fibre")
        .clone();
    assert_eq!(expected_pos_set.len(), 1);
    let empty_set: BTreeSet<SignedMultiset> = BTreeSet::new();

    let mut b8_head_to_head = Vec::new();
    for (cname, b_pts, h_arr, h_slc, r_pt, exp_set) in [
        ("B8_positive_single", &base8, &h8_arr, h8_vec.as_slice(), pos_res, &expected_pos_set),
        ("B8_positive_corank2_collision", &col_base8, &col_h8_arr, col_h8_vec.as_slice(), col_res, &expected_col_set),
        ("B8_negative_unsat_G", &base8, &h8_arr, h8_vec.as_slice(), neg1_res, &empty_set),
        ("B8_negative_unsat_2G", &base8, &h8_arr, h8_vec.as_slice(), neg2_res, &empty_set),
    ] {
        let w_rec = solve_4var_sparse_wiedemann(
            PRIME31.p,
            PRIME31.a,
            r_pt,
            h_arr,
            DEFAULT_WEIGHT_BOUND,
            0x2026_1119,
        );
        let d_rec = solve_4var_dense_elimination(
            PRIME31.p,
            PRIME31.a,
            r_pt,
            h_arr,
            DEFAULT_WEIGHT_BOUND,
        );
        let dq_rec = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            r_pt,
            h_slc,
            TowerMode::DirectCubicQuotient,
            8,
            Duration::from_secs(15),
        )
        .unwrap();
        let rq_rec = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            r_pt,
            h_slc,
            TowerMode::Reduced4VarQuotient,
            16,
            Duration::from_secs(15),
        )
        .unwrap();

        let decode_set = |charts: &[[u64; 7]], ar_mut: &mut CurveArith| -> BTreeSet<SignedMultiset> {
            charts
                .iter()
                .map(|c7| {
                    decode_and_verify_7chart_relation(PRIME31, b_pts, r_pt, c7, ar_mut)
                        .expect("valid replayed 8-point relation")
                })
                .collect()
        };
        let s_w = decode_set(&w_rec.solutions, &mut ar);
        let s_d = decode_set(&d_rec.solutions, &mut ar);
        let s_dq = decode_set(&dq_rec.chart_solutions, &mut ar);
        let s_rq = decode_set(&rq_rec.chart_solutions, &mut ar);
        let all_match = &s_w == exp_set && &s_d == exp_set && &s_dq == exp_set && &s_rq == exp_set;
        assert!(
            all_match,
            "Set mismatch on {cname}: exp={} w={}({:?}) d={}({:?}) dq={}({:?}) rq={}({:?})",
            exp_set.len(),
            s_w.len(),
            w_rec.status,
            s_d.len(),
            d_rec.status,
            s_dq.len(),
            dq_rec.status,
            s_rq.len(),
            rq_rec.status
        );

        b8_head_to_head.push(ExtractorHeadToHeadRow {
            case_name: cname.into(),
            expected_fibre_size: exp_set.len(),
            wiedemann: w_rec,
            dense_4var: d_rec,
            direct_quotient_f4: dq_rec,
            reduced_4var_f4: rq_rec,
            all_four_exact_set_match: all_match,
        });
    }

    // Part 3: Blinded `B = 4` & `B = 8` Complete-Fibre Grader Controls (`0` forbidden calls):
    let (base4, h4_vec) = build_factor_base(PRIME31, 4, &mut ar);
    let oracle4 = CompleteFibreOracle::build_exhaustive(PRIME31, &base4);

    let b4_single = oracle4
        .by_residual
        .iter()
        .find(|(_, set)| {
            set.len() == 1 && set.iter().all(|ms| ms.len() >= 2)
        })
        .unwrap();
    let b4_multi = oracle4
        .by_residual
        .iter()
        .find(|(_, set)| set.len() == 4)
        .unwrap();
    let b4_eight_same = ar.scalar(PRIME31, 8, Some(base4[0])).unwrap();
    let b4_eight_exp = oracle4.by_residual.get(&b4_eight_same).unwrap();

    let mut control_cases = Vec::new();
    for (lbl, r_pt, exp_opt) in [
        ("B4_complete_single_fibre", *b4_single.0, Some(b4_single.1)),
        ("B4_complete_4way_collision_fibre", *b4_multi.0, Some(b4_multi.1)),
        ("B4_complete_8x_repeated_point_fibre", b4_eight_same, Some(b4_eight_exp)),
        ("B4_complete_unsat_G", neg1_res, None),
    ] {
        let f_before = forbidden_calls();
        let rec = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            r_pt,
            &h4_vec,
            TowerMode::YunSymmetricQuotientTower,
            8,
            Duration::from_secs(60),
        )
        .unwrap();
        let f_delta = forbidden_calls() - f_before;
        assert_eq!(f_delta, 0);
        assert!(rec.complete_traversal, "{lbl} must traverse completely");
        let dec_set: BTreeSet<SignedMultiset> = rec
            .chart_solutions
            .iter()
            .map(|c7| decode_and_verify_7chart_relation(PRIME31, &base4, r_pt, c7, &mut ar).unwrap())
            .collect();
        let exp_ref = exp_opt.unwrap_or(&empty_set);
        let ok = &dec_set == exp_ref;
        assert!(ok, "B=4 complete fibre mismatch on {lbl}");
        control_cases.push(BlindedControlCase {
            label: lbl.into(),
            b_size: 4,
            extractor: "YunSymmetricQuotientTower".into(),
            expected_fibre_count: exp_ref.len(),
            decoded_fibre_count: dec_set.len(),
            exact_set_agreement: ok,
            complete_traversal: rec.complete_traversal,
            forbidden_calls_delta: f_delta,
            field_ops_or_muls: rec.f4_field_ops + rec.root_field_muls,
            wall_ms: rec.wall_ns as f64 / 1e6,
        });
    }

    // Also record the B=8 blinded cases + 16-residual blinded sweep (8 positive + 8 negative):
    for row in &b8_head_to_head {
        control_cases.push(BlindedControlCase {
            label: format!("{}_SparseWiedemann4Var", row.case_name),
            b_size: 8,
            extractor: "SparseWiedemann4Var".into(),
            expected_fibre_count: row.expected_fibre_size,
            decoded_fibre_count: row.wiedemann.solutions.len(),
            exact_set_agreement: row.all_four_exact_set_match,
            complete_traversal: true,
            forbidden_calls_delta: 0,
            field_ops_or_muls: row.wiedemann.fp_muls,
            wall_ms: row.wiedemann.wall_ns as f64 / 1e6,
        });
        control_cases.push(BlindedControlCase {
            label: format!("{}_DirectCubicQuotient", row.case_name),
            b_size: 8,
            extractor: "DirectCubicQuotient".into(),
            expected_fibre_count: row.expected_fibre_size,
            decoded_fibre_count: row.direct_quotient_f4.chart_solutions.len(),
            exact_set_agreement: row.all_four_exact_set_match,
            complete_traversal: row.direct_quotient_f4.complete_traversal,
            forbidden_calls_delta: 0,
            field_ops_or_muls: row.direct_quotient_f4.f4_field_ops,
            wall_ms: row.direct_quotient_f4.wall_ns as f64 / 1e6,
        });
    }

    let mut b8_batch16_matches = 0usize;
    let pos8_iter: Vec<_> = oracle8.distinct_abscissa_by_residual.iter().step_by(31).take(8).collect();
    assert_eq!(pos8_iter.len(), 8);
    for &(r_pt, exp_set) in &pos8_iter {
        let rec = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            *r_pt,
            &h8_vec,
            TowerMode::DirectCubicQuotient,
            8,
            Duration::from_secs(10),
        )
        .unwrap();
        let dec: BTreeSet<SignedMultiset> = rec
            .chart_solutions
            .iter()
            .map(|c7| decode_and_verify_7chart_relation(PRIME31, &base8, *r_pt, c7, &mut ar).unwrap())
            .collect();
        if &dec == exp_set && rec.complete_traversal {
            b8_batch16_matches += 1;
        }
    }
    for k_neg in 3u64..11 {
        let r_neg = ar.scalar(PRIME31, k_neg, Some(PRIME31.g)).unwrap();
        assert!(!oracle8.distinct_abscissa_by_residual.contains_key(&r_neg));
        let rec = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            r_neg,
            &h8_vec,
            TowerMode::DirectCubicQuotient,
            8,
            Duration::from_secs(10),
        )
        .unwrap();
        if rec.chart_solutions.is_empty() && rec.complete_traversal {
            b8_batch16_matches += 1;
        }
    }
    assert_eq!(b8_batch16_matches, 16);
    assert_eq!(forbidden_calls(), 0);

    let blinded_grader_controls = BlindedGraderSummary {
        b4_oracle_total_multisets: oracle4.total_multisets,
        b4_oracle_distinct_residuals: oracle4.by_residual.len(),
        b8_oracle_distinct_abscissa_tuples: oracle8
            .distinct_abscissa_by_residual
            .values()
            .map(|s| s.len())
            .sum(),
        b8_oracle_distinct_abscissa_residuals: oracle8.distinct_abscissa_by_residual.len(),
        cases: control_cases,
        b8_batch16_exact_matches: b8_batch16_matches,
        b8_batch16_total: 16,
        total_forbidden_calls: forbidden_calls(),
        all_controls_pass: true,
    };

    // Part 5B (run before Part 4 table so exact BSGS/rho PRIME31 numbers populate ratios in Part 4!):
    // 8 deterministic planted challenges across `[1, PRIME31.n - 1]`:
    let mut seed_chal = 0x5052_494d_4533_3121u64;
    let planted_scalars: Vec<u64> = (0..8)
        .map(|_| 1 + splitmix64(&mut seed_chal) % (PRIME31.n - 1))
        .collect();
    let targets_q: Vec<Point> = planted_scalars
        .iter()
        .map(|&d| ar.scalar(PRIME31, d, Some(PRIME31.g)))
        .collect();

    let m_bsgs = ((PRIME31.n as f64 / 2.0).sqrt().ceil()) as u64; // 32_768
    let bsgs = NegationSharedBsgs::build(PRIME31, m_bsgs);
    let mut bsgs_online_steps = 0u64;
    let mut bsgs_online_muls = 0u64;
    let mut bsgs_online_ns = 0u128;
    let mut bsgs_all_ok = true;
    for (idx, &q_pt) in targets_q.iter().enumerate() {
        let qr = bsgs.solve(q_pt).expect("PRIME31 BSGS");
        if qr.scalar != planted_scalars[idx] || !qr.verified {
            bsgs_all_ok = false;
        }
        bsgs_online_steps += qr.giant_steps;
        bsgs_online_muls += qr.online_counts.fp_muls;
        bsgs_online_ns += qr.online_ns;
    }
    assert!(bsgs_all_ok, "PRIME31 BSGS 8/8 must verify [d]G == Q");

    let mut rho_steps = 0u64;
    let mut rho_muls = 0u64;
    let mut rho_ns = 0u128;
    let mut rho_all_ok = true;
    for (idx, &q_pt) in targets_q.iter().enumerate() {
        let rr = negation_pollard_rho(PRIME31, q_pt, 0x9000 + idx as u64).expect("PRIME31 rho");
        if rr.scalar != planted_scalars[idx] || !rr.verified {
            rho_all_ok = false;
        }
        rho_steps += rr.walk_steps;
        rho_muls += rr.counts.fp_muls;
        rho_ns += rr.wall_ns;
    }
    assert!(rho_all_ok, "PRIME31 negation-rho 8/8 must verify [d]G == Q");

    // Live charged random-walk non-enumerative RR9 batch on `PRIME31` (`B = 8`, 24 random walks):
    let n_live_walks = 24usize;
    let mut walk_rng = 0x7761_6c6b_3331_0008u64;
    let mut live_direct_ops = 0u64;
    let mut live_direct_ns = 0u128;
    for _ in 0..n_live_walks {
        let u = 1 + splitmix64(&mut walk_rng) % (PRIME31.n - 1);
        let v = 1 + splitmix64(&mut walk_rng) % (PRIME31.n - 1);
        let ug = ar.scalar(PRIME31, u, Some(PRIME31.g));
        let vq = ar.scalar(PRIME31, v, targets_q[0]);
        let r_pt = ar.add(PRIME31, ug, vq).unwrap();
        let rec = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            r_pt,
            &h8_vec,
            TowerMode::DirectCubicQuotient,
            8,
            Duration::from_secs(10),
        )
        .unwrap();
        live_direct_ops += rec.f4_field_ops + rec.root_field_muls;
        live_direct_ns += rec.wall_ns;
    }
    assert_eq!(forbidden_calls(), 0);

    let bsgs_mean_online_muls = bsgs_online_muls as f64 / planted_scalars.len() as f64;
    let bsgs_cold_muls = bsgs.setup_counts.fp_muls as f64 + bsgs_mean_online_muls;
    let rho_mean_muls = rho_muls as f64 / planted_scalars.len() as f64;
    let rho_theory = (std::f64::consts::PI * PRIME31.n as f64 / 4.0).sqrt();
    let rho_mean_steps = rho_steps as f64 / planted_scalars.len() as f64;

    let mean_direct_unsat_ops = live_direct_ops as f64 / n_live_walks as f64;
    let mean_direct_unsat_ms = (live_direct_ns as f64 / 1e6) / n_live_walks as f64;
    let mean_wiedemann_unsat_muls = b8_head_to_head[2].wiedemann.fp_muls as f64;
    let mean_dense4_unsat_muls = b8_head_to_head[2].dense_4var.fp_muls as f64;

    let p_hit_b8 = 256.0 / (PRIME31.n as f64);
    let b8_total_calls = 9.0 / p_hit_b8;
    let b8_whole_direct_ops = b8_total_calls * mean_direct_unsat_ops;
    let b8_whole_wied_muls = b8_total_calls * mean_wiedemann_unsat_muls;
    let b8_whole_dense_muls = b8_total_calls * mean_dense4_unsat_muls;

    let prime31_whole_solve = Prime31WholeSolveSummary {
        num_challenges: planted_scalars.len(),
        planted_scalars,
        bsgs_m: m_bsgs,
        bsgs_all_verified: bsgs_all_ok,
        bsgs_setup_group_adds: bsgs.setup_counts.group_additions,
        bsgs_setup_fp_muls: bsgs.setup_counts.fp_muls,
        bsgs_setup_wall_ms: bsgs.setup_ns as f64 / 1e6,
        bsgs_mean_online_giant_steps: bsgs_online_steps as f64 / 8.0,
        bsgs_mean_online_fp_muls: bsgs_mean_online_muls,
        bsgs_mean_online_wall_ms: (bsgs_online_ns as f64 / 1e6) / 8.0,
        bsgs_cold_single_solve_fp_muls: bsgs_cold_muls,
        bsgs_cold_single_solve_wall_ms: (bsgs.setup_ns as f64 / 1e6)
            + (bsgs_online_ns as f64 / 1e6) / 8.0,
        rho_all_verified: rho_all_ok,
        rho_theoretical_sqrt_pi_n_over_4: rho_theory,
        rho_mean_walk_steps: rho_mean_steps,
        rho_step_ratio_to_theory: rho_mean_steps / rho_theory,
        rho_mean_fp_muls: rho_mean_muls,
        rho_mean_wall_ms: (rho_ns as f64 / 1e6) / 8.0,
        live_rr9_random_walk_trials_b8: n_live_walks,
        live_rr9_direct_mean_unsat_fp_ops: mean_direct_unsat_ops,
        live_rr9_direct_mean_unsat_wall_ms: mean_direct_unsat_ms,
        live_rr9_wiedemann_mean_unsat_fp_muls: mean_wiedemann_unsat_muls,
        live_rr9_dense4_mean_unsat_fp_muls: mean_dense4_unsat_muls,
        rr9_b8_expected_whole_solve_fp_ops_direct: b8_whole_direct_ops,
        rr9_b8_expected_whole_solve_fp_muls_wiedemann: b8_whole_wied_muls,
        rr9_b8_expected_whole_solve_fp_muls_dense4: b8_whole_dense_muls,
        ratio_rr9_direct_vs_rho_muls: b8_whole_direct_ops / rho_mean_muls,
        ratio_rr9_direct_vs_bsgs_cold_muls: b8_whole_direct_ops / bsgs_cold_muls,
        ratio_rr9_direct_vs_bsgs_online_muls: b8_whole_direct_ops / bsgs_mean_online_muls,
        ratio_rr9_wiedemann_vs_rho_muls: b8_whole_wied_muls / rho_mean_muls,
    };

    // Part 4: Scaling across `B = {8, 12, 16, 24, 31}` on `PRIME31`:
    let mut b_scaling = Vec::new();
    for &b_sz in &[8usize, 12, 16, 24, 31] {
        let (base_b, hb_vec) = build_factor_base(PRIME31, b_sz, &mut ar);
        let mut pos_b = None;
        for &pt in &base_b[..8] {
            pos_b = ar.add(PRIME31, pos_b, Some(pt));
        }
        let pos_b_res = pos_b.unwrap();
        let per_call_budget = if b_sz == 8 {
            Duration::from_secs(10)
        } else {
            Duration::from_millis(2000)
        };
        let deg_cap_b = if b_sz == 8 { 8 } else { 6 };

        let dq_pos = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            pos_b_res,
            &hb_vec,
            TowerMode::DirectCubicQuotient,
            deg_cap_b,
            per_call_budget,
        )
        .unwrap();
        let dq_unsat = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            neg1_res,
            &hb_vec,
            TowerMode::DirectCubicQuotient,
            deg_cap_b,
            per_call_budget,
        )
        .unwrap();
        let rq_unsat = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            neg1_res,
            &hb_vec,
            TowerMode::Reduced4VarQuotient,
            16 + (b_sz - 8) as u32,
            per_call_budget,
        )
        .unwrap();
        let frz_unsat = extract_via_remainder_tower(
            PRIME31.p,
            PRIME31.a,
            neg1_res,
            &hb_vec,
            TowerMode::FrozenCubicRemainderTower,
            4,
            Duration::from_millis(1200),
        )
        .unwrap();

        let n_dist = 256u128 * binom_u128(b_sz, 8);
        let n_mult = binom_u128(2 * b_sz + 7, 8);
        // Birthday/occupancy hit probability `1 - exp(-N_distinct / n)` (exact `N_distinct / n` when `N << n`):
        let lambda = (n_dist as f64) / (PRIME31.n as f64);
        let p_hit = if lambda < 1e-3 {
            lambda
        } else {
            1.0 - (-lambda).exp()
        };
        let exp_walks_per_hit = 1.0 / p_hit;
        let rels_needed = b_sz + 1;
        let total_calls = (rels_needed as f64) * exp_walks_per_hit;
        let best_per_call_ops = dq_unsat.f4_field_ops.min(rq_unsat.f4_field_ops).max(1) as f64;
        let whole_lb_ops = total_calls * best_per_call_ops;

        b_scaling.push(ScalingRowB {
            b_size: b_sz,
            distinct_8tuples_count: n_dist,
            multiset_8tuples_count: n_mult,
            hit_prob_distinct: p_hit,
            expected_walks_per_hit: exp_walks_per_hit,
            relations_needed_b_plus_1: rels_needed,
            expected_total_calls_charged: total_calls,
            direct_quotient_vars: dq_unsat.spec.n_vars,
            direct_quotient_eqs: dq_unsat.spec.n_equations,
            direct_quotient_pos_status: dq_pos.status,
            direct_quotient_unsat_status: dq_unsat.status,
            direct_quotient_max_cols: dq_unsat.max_matrix_cols.max(dq_pos.max_matrix_cols),
            direct_quotient_solving_deg: dq_unsat.max_solving_degree.max(dq_pos.max_solving_degree),
            direct_quotient_unsat_ops: dq_unsat.f4_field_ops,
            direct_quotient_unsat_ms: dq_unsat.wall_ns as f64 / 1e6,
            reduced_4var_vars: rq_unsat.spec.n_vars,
            reduced_4var_unsat_status: rq_unsat.status,
            reduced_4var_max_cols: rq_unsat.max_matrix_cols,
            reduced_4var_unsat_ops: rq_unsat.f4_field_ops,
            reduced_4var_unsat_ms: rq_unsat.wall_ns as f64 / 1e6,
            frozen_pr4692_vars: frz_unsat.spec.n_vars,
            frozen_pr4692_status: frz_unsat.status,
            frozen_pr4692_max_cols: frz_unsat.max_matrix_cols,
            whole_solve_charged_fp_ops_lower_bound: whole_lb_ops,
            ratio_vs_prime31_negation_rho_muls: whole_lb_ops / rho_mean_muls,
            ratio_vs_prime31_shared_bsgs_online_muls: whole_lb_ops / bsgs_mean_online_muls,
        });
    }
    assert_eq!(forbidden_calls(), 0);

    // Part 5A: Live end-to-end non-enumerative RR9 whole-solve (`[d]G == Q`) on toy prime-order curve:
    let toy_e2e = run_live_toy_non_enumerative_whole_solve();
    assert!(toy_e2e.d_g_eq_q_verified);
    assert_eq!(forbidden_calls(), 0);

    let cols_w42 = b8_head_to_head[0].dense_4var.cols;
    LeadP26FullReport {
        plucker_audit,
        unreduced_7var_d9_cols: 11_440,
        reduced_4var_w42_cols: cols_w42,
        macaulay_col_compression_ratio: 11_440.0 / (cols_w42 as f64),
        weight_threshold_scan: weight_scan,
        b8_head_to_head,
        blinded_grader_controls,
        prime31_whole_solve,
        b_scaling_prime31: b_scaling,
        toy_end_to_end_rr9_whole_solve: toy_e2e,
    }
}
