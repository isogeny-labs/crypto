use g6_large_m::analyze::*;
use g6_large_m::arms::{self, Arm};
use g6_large_m::census;
use g6_large_m::curve::{Curve, Pt};
use g6_large_m::field::Fq;
use g6_large_m::normal;
use g6_large_m::rng::Rng;
use g6_large_m::sizes;

fn all_points(c: &Curve) -> Vec<Pt> {
    let mut v = vec![Pt::Inf];
    for x in 0..c.f.q {
        if let Some(y) = c.lift_x(x) {
            v.push(Pt::Aff(x, y));
            if x != 0 {
                v.push(Pt::Aff(x, x ^ y));
            }
        }
    }
    v
}

#[test]
fn field_tables_and_trace() {
    for n in [7u32, 11, 13] {
        let f = Fq::new(n);
        for a in 1..f.q {
            assert_eq!(f.mul(a, f.inv(a)), 1);
        }
        assert_eq!(f.pow(3, 5), f.mul(f.mul(f.mul(f.mul(3, 3), 3), 3), 3));
        // Tr is F_2-linear and Frobenius invariant
        for a in 0..64u32 {
            assert_eq!(f.tr(a), f.tr(f.sq(a)));
        }
    }
}

#[test]
fn field_n23_is_a_field_and_has_the_type2_onb() {
    let f = Fq::new(23);
    assert_eq!(f.mul(12345, f.inv(12345)), 1);
    let b = normal::type2_onb(&f).expect("23: 2n+1 = 47 prime, ord_47(2) = 23");
    assert_eq!(normal::rank_f2(&b), 23);
    for i in 0..23 {
        for j in 0..23 {
            assert_eq!(
                f.tr(f.mul(b[i], b[j])),
                (i == j) as u32,
                "optimal normal basis is self-dual at ({i},{j})"
            );
        }
        assert_eq!(b[(i + 1) % 23], f.sq(b[i]));
    }
}

#[test]
fn type2_onb_n11_and_not_applicable_n13() {
    let f = Fq::new(11);
    let b = normal::type2_onb(&f).unwrap();
    assert_eq!(normal::rank_f2(&b), 11);
    for i in 0..11 {
        for j in 0..11 {
            assert_eq!(f.tr(f.mul(b[i], b[j])), (i == j) as u32);
        }
    }
    assert!(normal::type2_onb(&Fq::new(13)).is_none()); // 27 is not prime
    assert!(normal::type2_onb(&Fq::new(7)).is_none()); // 15 is not prime
}

#[test]
fn iota_is_translation_by_t_on_x() {
    for (n, a) in [(7u32, 0u32), (7, 1), (11, 0), (11, 1)] {
        let f = Fq::new(n);
        let c = Curve::koblitz(&f, a);
        let t = c.t2();
        assert!(c.on_curve(t));
        assert_eq!(c.add(t, t), Pt::Inf);
        for p in all_points(&c) {
            if let Pt::Aff(x, _) = p {
                if x == 0 {
                    continue; // P = T: P + T = O
                }
                match c.add(p, t) {
                    Pt::Aff(x2, _) => assert_eq!(x2, c.iota_x(x)),
                    Pt::Inf => panic!("P+T = O only for P = T"),
                }
            }
        }
    }
}

#[test]
fn no_iota_stable_subspace_of_dim_2_or_3_at_n7() {
    // iota-stable: x in V, x != 0  =>  sqrt(b)/x in V. Exhaustive over all subspaces of dim 1..=3 of F_{2^7}.
    let f = Fq::new(7);
    let c = Curve::koblitz(&f, 0);
    let mut found: std::collections::BTreeSet<Vec<u32>> = Default::default();
    let q = f.q;
    let stable = |v: &Vec<u32>| v.iter().all(|&x| x == 0 || v.contains(&c.iota_x(x)));
    for x1 in 1..q {
        let s1 = normal::span(&[x1]);
        if stable(&s1) {
            let mut s = s1.clone();
            s.sort();
            found.insert(s);
        }
        for x2 in x1 + 1..q {
            if normal::rank_f2(&[x1, x2]) < 2 {
                continue;
            }
            let s2 = normal::span(&[x1, x2]);
            assert!(
                !stable(&s2),
                "a 2-dim iota-stable subspace would contradict the F_4 argument"
            );
            for x3 in x2 + 1..q {
                if normal::rank_f2(&[x1, x2, x3]) < 3 {
                    continue;
                }
                assert!(!stable(&normal::span(&[x1, x2, x3])));
            }
        }
    }
    // dim 1: only {0, x0} with x0^2 = sqrt(b) = 1 here, i.e. x0 = 1
    assert_eq!(found.len(), 1);
    assert_eq!(found.iter().next().unwrap(), &vec![0, 1]);
}

#[test]
fn on_curve_status_is_constant_along_a_frobenius_orbit_of_x() {
    // the mechanism that makes NI slot sizes perfectly correlated
    for (n, a) in [(11u32, 0u32), (11, 1), (13, 0)] {
        let f = Fq::new(n);
        let c = Curve::koblitz(&f, a);
        for x in 1..f.q {
            assert_eq!(c.lift_x(x).is_some(), c.lift_x(f.sq(x)).is_some());
        }
    }
}

#[test]
fn ni_full_slots_are_frobenius_rotations_of_each_other() {
    let f = Fq::new(11);
    let c = Curve::koblitz(&f, 0);
    let slots = arms::build_normal(&f, &c, Arm::NI, 4, 777); // dims 3,3,3,2
    let sz = arms::sizes(&slots);
    assert_eq!(sz[0], sz[1]);
    assert_eq!(sz[1], sz[2]);
    // slot 1 = sigma(slot 0) as sets
    let mut s0: Vec<Pt> = slots[0].iter().map(|&p| c.frob_pt(p)).collect();
    let mut s1 = slots[1].clone();
    let key = |p: &Pt| c.key(*p);
    s0.sort_by_key(key);
    s1.sort_by_key(key);
    assert_eq!(s0, s1);
}

fn brute(c: &Curve, slots: &[Vec<Pt>]) -> Vec<u32> {
    let mut h = vec![0u32; c.key_space()];
    let mut idx = vec![0usize; slots.len()];
    loop {
        let mut s = Pt::Inf;
        for (i, sl) in slots.iter().enumerate() {
            s = c.add(s, sl[idx[i]]);
        }
        h[c.key(s)] += 1;
        let mut k = 0;
        loop {
            if k == slots.len() {
                return h;
            }
            idx[k] += 1;
            if idx[k] < slots[k].len() {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
    }
}

#[test]
fn mitm_histogram_equals_brute_force() {
    for (n, a, m, arm) in [
        (7u32, 0u32, 3usize, Arm::NI),
        (7, 1, 4, Arm::NR),
        (11, 0, 5, Arm::NI),
        (11, 1, 6, Arm::NC),
    ] {
        let f = Fq::new(n);
        let c = Curve::koblitz(&f, a);
        let slots = arms::build_normal(&f, &c, arm, m, 99 + m as u64);
        let mut h = Vec::new();
        let cen = census::histogram_into(&c, &slots, &mut h);
        assert_eq!(h, brute(&c, &slots), "n={n} m={m}");
        let st = census::stats(&h, c.count(), cen.t);
        assert!(st.conserved);
        let mut rng = Rng::new(5);
        assert_eq!(census::planted_hits(&c, &slots, &h, 100, &mut rng), 100);
    }
}

#[test]
fn rotating_all_slots_by_sigma_rotates_the_histogram() {
    // Proposition T in the census: H_{sigma(V)}[sigma R] = H_V[R]
    let f = Fq::new(11);
    let c = Curve::koblitz(&f, 0);
    let slots = arms::build_normal(&f, &c, Arm::NI, 5, 31);
    let rot: Vec<Vec<Pt>> = slots
        .iter()
        .map(|s| s.iter().map(|&p| c.frob_pt(p)).collect())
        .collect();
    let (mut h, mut hr) = (Vec::new(), Vec::new());
    census::histogram_into(&c, &slots, &mut h);
    census::histogram_into(&c, &rot, &mut hr);
    for p in all_points(&c) {
        assert_eq!(h[c.key(p)], hr[c.key(c.frob_pt(p))]);
    }
}

#[test]
fn negation_symmetry_of_h() {
    let f = Fq::new(11);
    let c = Curve::koblitz(&f, 1);
    let slots = arms::build_normal(&f, &c, Arm::NR, 5, 8);
    let mut h = Vec::new();
    census::histogram_into(&c, &slots, &mut h);
    for p in all_points(&c) {
        assert_eq!(h[c.key(p)], h[c.key(c.neg(p))]);
    }
}

#[test]
fn orbit_map_partitions_the_group_and_tamper_gives_n() {
    let f = Fq::new(11);
    let c = Curve::koblitz(&f, 0);
    let om = census::build_orbits(&c);
    let ne = c.count() as usize;
    assert_eq!(om.keys.len() % 11, 0);
    assert_eq!(om.keys.len() + om.fixed, ne);
    let slots = arms::build_normal(&f, &c, Arm::NI, 5, 4);
    let mut h = Vec::new();
    census::histogram_into(&c, &slots, &mut h);
    let t = census::orbit_fano(&h, &om, true);
    assert!(
        (t - 11.0).abs() < 1e-9,
        "tamper must give exactly n, got {t}"
    );
    let f0 = census::orbit_fano(&h, &om, false);
    assert!(f0.is_finite() && f0 > 0.0);
}

#[test]
fn orbit_union_slots_collapse_the_attempts() {
    // negative-of-independence: a sigma-stable slot set (union of full x-orbits) makes H sigma-invariant, so Fano = n
    let f = Fq::new(11);
    let c = Curve::koblitz(&f, 0);
    let mut pts = vec![];
    let mut x = 3u32;
    let mut got = 0;
    while got < 2 {
        if c.lift_x(x).is_some() {
            let mut z = x;
            let orb: Vec<u32> = (0..11)
                .map(|_| {
                    let v = z;
                    z = f.sq(z);
                    v
                })
                .collect();
            pts.extend(c.points_for_xs(&orb));
            got += 1;
        }
        x += 1;
    }
    let slots = vec![pts.clone(), pts.clone()];
    let (mut h, om) = (Vec::new(), census::build_orbits(&c));
    census::histogram_into(&c, &slots, &mut h);
    for p in all_points(&c) {
        assert_eq!(h[c.key(p)], h[c.key(c.frob_pt(p))]);
    }
    assert!((census::orbit_fano(&h, &om, false) - 11.0).abs() < 1e-6);
}

#[test]
fn matched_generic_has_the_exact_profile_and_spans() {
    let f = Fq::new(11);
    let c = Curve::koblitz(&f, 0);
    for m in [4usize, 5, 9] {
        let ni = arms::build_normal(&f, &c, Arm::NI, m, 1234 + m as u64);
        let prof = arms::sizes(&ni);
        let up = arms::build_matched_generic(&f, &c, m, 55, &prof).expect("profile reachable");
        assert_eq!(arms::sizes(&up), prof);
        let up2 = arms::build_matched_generic(&f, &c, m, 56, &prof).unwrap();
        assert_eq!(arms::sizes(&up2), prof);
        assert_ne!(up, up2);
    }
}

#[test]
fn duplicate_slot_control_is_visible_in_dispersion() {
    // exchange symmetry doubles the multiplicity of off-diagonal tuples: Var/mean -> about 2
    let f = Fq::new(13);
    let c = Curve::koblitz(&f, 0);
    let ne = c.count();
    let (mut r_up, mut r_dup, mut cnt) = (0.0, 0.0, 0);
    for s in 0..12u64 {
        let ni = arms::build_normal(&f, &c, Arm::NI, 4, 10 + s);
        let prof = arms::sizes(&ni);
        let Some(up) = arms::build_matched_generic(&f, &c, 4, 20 + s, &prof) else {
            continue;
        };
        let dup = arms::build_dup(&up);
        let (mut h, mut h2) = (Vec::new(), Vec::new());
        let a = census::histogram_into(&c, &up, &mut h);
        let b = census::histogram_into(&c, &dup, &mut h2);
        let (su, sd) = (census::stats(&h, ne, a.t), census::stats(&h2, ne, b.t));
        if su.mu > 0.2 {
            r_up += su.disp;
            r_dup += sd.disp;
            cnt += 1;
        }
    }
    assert!(cnt >= 3, "too few usable seeds");
    let ratio = (r_dup / cnt as f64) / (r_up / cnt as f64);
    assert!(ratio > 1.5, "positive control ratio {ratio}");
}

#[test]
fn iota_quotient_identity_holds_at_small_sizes() {
    use g6_large_m::iota::iota_audit;
    for (n, a, m, k) in [
        (7u32, 0u32, 3usize, 3usize),
        (11, 0, 4, 2),
        (11, 1, 5, 5),
        (13, 0, 9, 3),
    ] {
        let f = Fq::new(n);
        let c = Curve::koblitz(&f, a);
        let r = iota_audit(&c, m, k, 3).unwrap();
        assert!(
            r.translate_invariant && r.multiple_of_2k1 && r.quotient_identity,
            "n={n} m={m} k={k}"
        );
        assert_eq!(r.tuples_iota, r.tuples_plain << k);
    }
}

#[test]
fn sizes_anchors_and_degrees() {
    assert_eq!(sizes::chain_sat_vars(83, 6), 105_908); // source test value
    assert_eq!(sizes::rotated_inputs(83, 10), 2_996); // source check
    assert_eq!(sizes::chain_sat_patterns(12), 1024);
    assert_eq!(sizes::chain_sat_patterns(16), 16384);
    assert_eq!(sizes::chain_sat_patterns(18), 65536); // the scout wrote 16384 for m = 18; that is m = 16
    assert_eq!(
        sizes::chain_sat_and_gates(83, 6) * 3,
        sizes::chain_sat_cnf_clauses(83, 6)
    );
    assert_eq!(sizes::s3_bit_degree(), 3);
    assert_eq!(sizes::s3_total_degree(), 4);
    assert_eq!(sizes::iota_stabiliser_order(10), 512);
    let l = sizes::symmetric_l_for_unit_yield(23, 12);
    assert!((l - 4.316).abs() < 0.01, "{l}");
    assert!((sizes::log2_factorial(20) - 61.077).abs() < 0.01);
    // the toy orbit-union regime is oversized: one full orbit at n=23, m=9 has log2 Y > 8
    assert!(sizes::single_orbit_log2_yield(23, 9) > 8.0);
    // at n=131, m=20 one orbit is undersized and about 3 orbits are needed
    assert!(sizes::single_orbit_log2_yield(131, 20) < -30.0);
    assert_eq!(sizes::orbits_for_unit_yield(131, 20), Some(3));
    let t = sizes::table();
    assert!(t.lines().count() > 40);
}

#[test]
fn decision_rule_classifies_synthetic_contrasts() {
    let noise = |i: usize| ((i as f64 * 0.7).sin()) * 0.01;
    let u: Vec<f64> = (0..32).map(|i| 0.1 * (i as f64 % 5.0)).collect();
    let same: Vec<f64> = u.iter().enumerate().map(|(i, v)| v + noise(i)).collect();
    assert_eq!(contrast_paired(&same, &u).verdict, Verdict::Equivalent);
    let shifted: Vec<f64> = u
        .iter()
        .enumerate()
        .map(|(i, v)| v + 0.4 + noise(i))
        .collect();
    assert_eq!(contrast_paired(&shifted, &u).verdict, Verdict::Different);
    let wide: Vec<f64> = u
        .iter()
        .enumerate()
        .map(|(i, v)| v + 0.05 + if i % 2 == 0 { 0.6 } else { -0.6 })
        .collect();
    assert_eq!(contrast_paired(&wide, &u).verdict, Verdict::Unresolved);
    assert_eq!(
        contrast_paired(&same[..5], &u[..5]).verdict,
        Verdict::TooFew
    );
    // a shift inside the practical margin with a tiny se is EQUIVALENT, not DIFFERENT
    let tiny: Vec<f64> = u
        .iter()
        .enumerate()
        .map(|(i, v)| v + 0.05 + noise(i))
        .collect();
    assert_eq!(contrast_paired(&tiny, &u).verdict, Verdict::Equivalent);
    // Welch: same
    assert_eq!(contrast_welch(&same, &u).verdict, Verdict::Equivalent);
    assert_eq!(contrast_welch(&wide, &u).verdict, Verdict::Unresolved);
    assert!(t_tost(7) > t_tost(30) && t_diff(7) > t_diff(60));
}

#[test]
fn frozen_constants_and_cell_list() {
    use g6_large_m::cell::*;
    assert_eq!(
        (DRAWS, K_MAX, K_AA, K_CTRL, K_SEC, MIN_PAIRS),
        (3000, 32, 16, 8, 32, 8)
    );
    assert_eq!(
        (
            LMIN,
            LMAX,
            MIN_HITS,
            CAP_LOG2_TUPLES,
            WINDOW_HALF,
            TUPLE_BUDGET_LOG2
        ),
        (-10.0, 5.0, 1024.0, 28.0, 0.5, 33.0)
    );
    assert!((DELTA - 1.1f64.ln()).abs() < 1e-12 && (DUP_MIN_LOG_RATIO - 1.5f64.ln()).abs() < 1e-12);
    let l = cell_list();
    assert_eq!(l.len(), 8 + 14 + 4);
    assert!(
        l.iter().filter(|&&(n, _, _)| n < 19).count() == 4,
        "slot-size control cells (n = 13, 17 at m = 9, 10)"
    );
    assert!(l
        .iter()
        .all(|&(n, _, m)| m >= 9 && m <= 20 && (m as u32) < n));
    assert!(l.iter().any(|&(_, _, m)| m == 20) && l.iter().any(|&(_, _, m)| m == 9));
}

#[test]
fn mde_is_the_effect_the_rule_can_detect() {
    use g6_large_m::analyze::*;
    let c = Contrast {
        n: 32,
        d: 0.0,
        se: 0.05,
        df: 31,
        verdict: Verdict::Unresolved,
    };
    let want = ((t_diff(31) + MDE_POWER_Z) * 0.05).exp();
    assert!((c.mde_ratio() - want).abs() < 1e-12);
    // never below the practical margin
    let tiny = Contrast {
        n: 32,
        d: 0.0,
        se: 1e-6,
        df: 31,
        verdict: Verdict::Equivalent,
    };
    assert!((tiny.mde_ratio() - DELTA.exp()).abs() < 1e-12);
    // the tabulated Bonferroni values dominate the exact Student-t quantiles at alpha = 0.05/338 (7.412, 5.916, 5.035, 4.668, 4.342, 4.053)
    for (df, exact) in [
        (7usize, 7.412),
        (10, 5.916),
        (15, 5.035),
        (20, 4.668),
        (30, 4.342),
        (60, 4.053),
    ] {
        assert!(t_diff(df) >= exact && t_diff(df) < exact + 0.15, "df {df}");
    }
}

#[test]
fn duplicate_control_prefers_slots_with_more_than_one_point() {
    let f = Fq::new(13);
    let c = Curve::koblitz(&f, 0);
    let t = c.t2();
    let pts: Vec<Pt> = (2..200u32)
        .filter_map(|x| c.lift_x(x).map(|y| Pt::Aff(x, y)))
        .take(2)
        .collect();
    let big = |p: Pt| vec![t, p, c.neg(p)];
    let base = vec![vec![t], vec![t], big(pts[0]), big(pts[1])];
    let d = arms::build_dup(&base);
    assert!(
        d != base,
        "the control must not be a no-op when two multi-point slots exist"
    );
    assert_eq!(d[3], d[2]);
    let all_single = vec![vec![t], vec![t], vec![t]];
    assert!(
        arms::build_dup(&all_single) == all_single,
        "all-single-point profiles are a no-op and are skipped by the cell"
    );
}

fn synthetic_rows(chain: bool, tamper_pred: bool) -> Vec<g6_large_m::analyze::R> {
    use g6_large_m::analyze::R;
    let mk = |arm: &str, idx: u64, disp: f64, chain: bool, pd: f64| R {
        arm: arm.into(),
        idx,
        l2mu: -5.0 + 0.01 * idx as f64,
        p: 0.01 * (1.0 + 0.001 * idx as f64),
        eff: 0.9,
        disp,
        fano: 1.0 + 0.01 * idx as f64,
        fano_tamper: 23.0,
        planted: g6_large_m::cell::PLANTS,
        conserved: true,
        a_len: 100,
        b_len: 200,
        adds_build: 300,
        log2t: 17.0,
        pred_p: if chain {
            0.01 * (1.0 + 0.001 * idx as f64)
        } else {
            f64::NAN
        },
        pred_disp: if chain { pd } else { f64::NAN },
    };
    let mut v = vec![];
    for i in 0..12u64 {
        let ni_disp = 5.0 + 0.01 * i as f64;
        v.push(mk(
            "NI",
            i,
            ni_disp,
            chain,
            if tamper_pred && i == 3 {
                ni_disp * 1.01
            } else {
                ni_disp
            },
        ));
        let u = 1.0 + 0.01 * ((i * 7) % 5) as f64;
        v.push(mk("UP", i, u, false, 0.0));
        v.push(mk(
            "UP2",
            i,
            u * (1.0 + 0.001 * ((i % 3) as f64 - 1.0)),
            false,
            0.0,
        ));
        v.push(mk("DUP", i, u * 1.7, false, 0.0));
    }
    v
}

#[test]
fn chain_stratum_is_derived_not_tested_and_a_model_mismatch_voids_the_cell() {
    use g6_large_m::analyze::analyze_cell;
    let meta = "n\t23\na\t0\nm\t12\nstatus\tOK\nwindow_center\t-5\n";
    let ok = analyze_cell("n23a0m12", meta, &synthetic_rows(true, false));
    assert!(ok.valid, "{:?}", ok.void_reasons);
    assert_eq!((ok.chain_pairs, ok.stat_pairs), (12, 0));
    assert!(
        ok.yield_verdict
            .starts_with("NO_STATISTICAL_PAIRS(0 non-chain)")
            && ok.yield_verdict.contains("CHAIN_DERIVED"),
        "{}",
        ok.yield_verdict
    );
    assert!(!ok.yield_verdict.contains("YIELD_DIFFERS"));
    let bad = analyze_cell("n23a0m12", meta, &synthetic_rows(true, true));
    assert!(
        !bad.valid
            && bad
                .void_reasons
                .iter()
                .any(|r| r.starts_with("DIGIT_MODEL_DISAGREES")),
        "{:?}",
        bad.void_reasons
    );
    // the same rows without the chain flag are tested statistically: NI dispersion is 5x UP, the rule says DIFFERENT
    let stat = analyze_cell("n23a0m12", meta, &synthetic_rows(false, false));
    assert!(
        stat.valid && stat.stat_pairs == 12 && stat.yield_verdict.starts_with("YIELD_DIFFERS"),
        "{} {:?}",
        stat.yield_verdict,
        stat.void_reasons
    );
}

#[test]
fn tuple_budget_rule_counts_every_enumerated_instance() {
    use g6_large_m::cell::*;
    // NI + UP + DUP + UP2 + ONB pair, and NC/NR up to half a bit above the median
    let u32_ = cell_tuple_units(32);
    assert!((u32_ - (64.0 + 8.0 + 16.0 + 2.0 + 64.0 * SEC_WINDOW_FACTOR)).abs() < 1e-9);
    assert!(
        cell_tuple_units(8) / 8.0 > cell_tuple_units(32) / 32.0,
        "small k pays the fixed controls over fewer pairs"
    );
    assert!((SEC_WINDOW_FACTOR - 2f64.sqrt()).abs() < 1e-3);
}
