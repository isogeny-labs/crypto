//! Exact-identity and planted-instance tests for cell G5. Tiny fixtures only; nothing here produces results.
use g5_propt_census::bases::*;
use g5_propt_census::census::*;
use g5_propt_census::chain::*;
use g5_propt_census::curve::*;
use g5_propt_census::field::Fq;
use g5_propt_census::proptt::*;
use g5_propt_census::rank::*;
use g5_propt_census::rng::Rng;

#[test]
fn field_basics() {
    for n in [5u32, 7, 11, 13] {
        let f = Fq::new(n);
        let mut rng = Rng::new(1);
        for _ in 0..200 {
            let x = f.rand(&mut rng);
            assert_eq!(f.frob(x, n), x);
            assert_eq!(f.frob_inv(f.frob(x, 3), 3), x);
            let y = f.rand(&mut rng);
            assert_eq!(f.frob(f.mul(x, y), 2), f.mul(f.frob(x, 2), f.frob(y, 2)));
            assert_eq!(f.frob(x ^ y, 2), f.frob(x, 2) ^ f.frob(y, 2));
            if x != 0 {
                assert_eq!(f.mul(x, f.inv(x)), 1);
            }
            let c = f.rand(&mut rng);
            if f.tr(c) == 0 {
                let z = f.ht(c);
                assert_eq!(f.sq(z) ^ z, c);
            }
        }
        for j in 1..n {
            assert_eq!(
                rank(&sigma_matrix(&f, j)),
                n as usize,
                "sigma^j matrix is invertible"
            );
        }
    }
}

#[test]
fn s3_matches_point_addition_for_both_a() {
    let f = Fq::new(11);
    let mut rng = Rng::new(7);
    for a in [0u32, 1] {
        let c = Curve::koblitz(&f, a);
        let mut checked = 0;
        while checked < 300 {
            let x1 = f.rand(&mut rng);
            let x2 = f.rand(&mut rng);
            let (Some(y1), Some(y2)) = (c.lift_x(x1), c.lift_x(x2)) else {
                continue;
            };
            let p = Pt::Aff(x1, y1);
            let q = Pt::Aff(x2, y2);
            assert!(c.on_curve(p) && c.on_curve(q));
            for sum in [c.add(p, q), c.add(p, c.neg(q))] {
                if let Pt::Aff(x3, _) = sum {
                    assert_eq!(s3(&f, 1, x1, x2, x3), 0, "a={a} x1={x1} x2={x2}");
                    checked += 1;
                }
            }
        }
    }
}

#[test]
fn roots_are_all_roots() {
    let f = Fq::new(7);
    let mut rng = Rng::new(3);
    for b in [1u32, 5, 77] {
        for _ in 0..200 {
            let (x1, x2) = (f.rand(&mut rng), f.rand(&mut rng));
            let mut brute: Vec<u32> = (0..f.q).filter(|&x3| s3(&f, b, x1, x2, x3) == 0).collect();
            brute.sort_unstable();
            let got = match roots(&f, b, x1, x2) {
                Roots::None => vec![],
                Roots::One(v) => vec![v],
                Roots::Two(v, w) => {
                    let mut t = vec![v, w];
                    t.sort_unstable();
                    t
                }
                Roots::All => (0..f.q).collect(),
            };
            assert_eq!(got, brute, "b={b} x1={x1} x2={x2}");
        }
    }
}

#[test]
fn primary_curve_orders() {
    // Orders and largest prime factors of the planned primary cells (a, n) = (1,11), (0,13), (1,17), (0,19).
    for (a, n, ne, r) in [
        (1u32, 11u32, 1982u32, 991u32),
        (0, 13, 8012, 2003),
        (1, 17, 131174, 65587),
        (0, 19, 523492, 130873),
    ] {
        let f = Fq::new(n);
        let (ne2, r2, chk) = curve_summary(&f, a);
        assert_eq!((ne2, r2), (ne, r), "a={a} n={n}");
        assert!(chk.is_ok(), "{chk:?}");
    }
}

fn group11() -> (Fq, Group) {
    let f = Fq::new(11);
    let g = build_group(&f, 1).expect("group");
    (f, g)
}

#[test]
fn group_frobenius_equivariance_exhaustive() {
    let (f, g) = group11();
    assert_eq!(g.r, 991);
    assert_eq!(g.xs.len() as u32, g.half());
    for d in 1..=g.half() {
        let x = g.xs[(d - 1) as usize];
        assert_eq!(g.dlog[x as usize], d);
        for j in 1..f.n {
            let xj = f.frob(x, j);
            let dj = g.dlog[xj as usize];
            assert_ne!(dj, NONE, "sigma maps G-classes to G-classes");
            assert_eq!(
                dj,
                g.class(g.mulmod(g.lam_pow(j), d)),
                "d(sigma^j x) = class(lam^j d)"
            );
        }
    }
}

#[test]
fn chain_membership_matches_signed_sums_m2_m3_exhaustive() {
    let (f, g) = group11();
    let mut rng = Rng::new(11);
    for m in [2usize, 3] {
        for _ in 0..12 {
            let set = random_class_set(&g, m, &mut rng);
            let mut ks = std::collections::HashSet::new();
            for mask in 0u32..(1 << m) {
                let mut s: u64 = 0;
                for (i, &d) in set.ds.iter().enumerate() {
                    s += if (mask >> i) & 1 == 0 {
                        d as u64
                    } else {
                        (g.r - d) as u64
                    };
                }
                let c = g.class((s % g.r as u64) as u32);
                if c != 0 {
                    ks.insert(c);
                }
            }
            for a in 1..=g.half() {
                let xr = g.xs[(a - 1) as usize];
                let chain = member(&f, 1, xr, &set.xs);
                assert_eq!(chain, ks.contains(&a), "m={m} a={a}");
            }
        }
    }
}

#[test]
fn chain_implies_signed_sum_and_converse_up_to_order_m4() {
    let (f, g) = group11();
    let mut rng = Rng::new(12);
    for _ in 0..6 {
        let set = random_class_set(&g, 4, &mut rng);
        let mut ks = std::collections::HashSet::new();
        for mask in 0u32..16 {
            let mut s: u64 = 0;
            for (i, &d) in set.ds.iter().enumerate() {
                s += if (mask >> i) & 1 == 0 {
                    d as u64
                } else {
                    (g.r - d) as u64
                };
            }
            let c = g.class((s % g.r as u64) as u32);
            if c != 0 {
                ks.insert(c);
            }
        }
        let perms: Vec<[usize; 4]> = {
            let mut v = vec![];
            for a in 0..4 {
                for b in 0..4 {
                    for c in 0..4 {
                        for d in 0..4 {
                            if a != b && a != c && a != d && b != c && b != d && c != d {
                                v.push([a, b, c, d]);
                            }
                        }
                    }
                }
            }
            v
        };
        for a in 1..=g.half() {
            let xr = g.xs[(a - 1) as usize];
            let mut any = false;
            for p in &perms {
                let xs: Vec<u32> = p.iter().map(|&i| set.xs[i]).collect();
                let c = member(&f, 1, xr, &xs);
                if c {
                    assert!(ks.contains(&a), "chain solution without a signed sum");
                }
                any |= c;
            }
            assert_eq!(
                any,
                ks.contains(&a),
                "a={a}: some order of the chain finds every signed-sum solution"
            );
        }
    }
}

#[test]
fn prop_t_holds_on_koblitz_all_m_and_fails_on_controls() {
    let f = Fq::new(11);
    for m in 2..=8usize {
        let mut rng = Rng::new(100 + m as u64);
        let (mut kob, mut ord, mut wrong) = (
            CellTally::default(),
            CellTally::default(),
            CellTally::default(),
        );
        for _ in 0..150 {
            kob.add(&run_trial(&f, Arm::Koblitz, m, 3, &[], &mut rng).unwrap());
            ord.add(&run_trial(&f, Arm::Ordinary, m, 3, &[], &mut rng).unwrap());
            wrong.add(&run_trial(&f, Arm::WrongDir, m, 3, &[], &mut rng).unwrap());
        }
        assert_eq!(kob.trials_eq_fail, 0, "m={m}");
        assert_eq!(kob.trials_bool_fail, 0);
        assert_eq!(kob.rec_singular, 0);
        assert_eq!(kob.plant_bad, 0);
        assert_eq!(
            kob.conj_member_fail, 0,
            "conjugated planted solution must be a solution"
        );
        assert_eq!(kob.dom_fail, 0);
        assert_eq!(kob.neg_disagree, 0);
        assert!(
            kob.planted > 80,
            "planting must mostly succeed (m={m}: {})",
            kob.planted
        );
        // ordinary curve: the identity fails in every assignment
        assert_eq!(ord.assigns_eq_fail, ord.assigns, "m={m}");
        assert!(
            ord.conj_member_fail * 10 >= ord.conj_member_n * 9,
            "ordinary conj_member fails in >= 90%"
        );
        // wrong direction: field identity fails (with K=3 assignments) in nearly every trial
        assert!(
            wrong.trials_eq_fail * 10 >= wrong.trials * 9,
            "wrongdir detection {}/{}",
            wrong.trials_eq_fail,
            wrong.trials
        );
    }
}

#[test]
fn stable_subspaces_at_n17_and_collapse() {
    let f = Fq::new(17);
    let st = stable_subspaces(&f, 8);
    assert_eq!(st.len(), 2, "x^17+1 = (x+1) f1 f2 with deg f_i = 8");
    for b in &st {
        assert!(is_sigma_stable(&f, b));
    }
    assert!(!is_sigma_stable(&f, &poly_window(8)));
    let mut rng = Rng::new(5);
    let mut tal = CellTally::default();
    for _ in 0..100 {
        tal.add(&run_trial(&f, Arm::Stable, 3, 2, &st, &mut rng).unwrap());
    }
    assert_eq!(tal.stable_cob_fail, 0);
    assert_eq!(tal.trials_eq_fail, 0);
    // a non-stable subspace has no change of coordinates for most j
    let mut cnt = 0;
    for j in 1..17 {
        let w = poly_window(8);
        if frob_basis(&f, &w, j)
            .iter()
            .all(|&v| coords(&w, v).is_some())
        {
            cnt += 1;
        }
    }
    assert_eq!(cnt, 0);
}

fn naive_census(ds: &[u32], r: u32, m: usize, mus: &[u32]) -> (u64, Vec<u64>) {
    use std::collections::HashSet;
    let cls = |v: u64| -> u32 {
        let v = (v % r as u64) as u32;
        v.min(r - v)
    };
    let n = ds.len();
    let mut idx: Vec<usize> = (0..m).collect();
    let mut nsol = 0u64;
    let mut u = vec![0u64; mus.len()];
    loop {
        let mut ks = HashSet::new();
        for mask in 0u32..(1 << m) {
            let mut s: u64 = 0;
            for (i, &k) in idx.iter().enumerate() {
                s += if (mask >> i) & 1 == 0 {
                    ds[k] as u64
                } else {
                    (r - ds[k]) as u64
                };
            }
            let c = cls(s);
            if c != 0 {
                ks.insert(c);
            }
        }
        nsol += ks.len() as u64;
        for (t, &mu) in mus.iter().enumerate() {
            for &a in &ks {
                if ks.contains(&cls(a as u64 * mu as u64)) {
                    u[t] += 1;
                }
            }
        }
        // next combination
        let mut i = m;
        loop {
            if i == 0 {
                return (nsol, u);
            }
            i -= 1;
            if idx[i] != i + n - m {
                break;
            }
            if i == 0 && idx[0] == n - m {
                return (nsol, u);
            }
        }
        idx[i] += 1;
        for k in i + 1..m {
            idx[k] = idx[k - 1] + 1;
        }
    }
}

#[test]
fn census_matches_naive_and_planted_overlaps_are_recalled() {
    let (_f, g) = group11();
    let mut rng = Rng::new(21);
    let fr = Mult::frobenius(&g);
    for m in 2..=5usize {
        let set = random_class_set(&g, 8, &mut rng);
        let out = census(&set.ds, g.r, m, &[&fr], 16, 4, None);
        let (nsol, u) = naive_census(&set.ds, g.r, m, &fr.mus);
        assert_eq!(out.nsol, nsol, "m={m}");
        assert_eq!(out.u[0], u, "m={m}");
        assert_eq!(out.nsub, binom(8, m) as u64);
    }
    // planted: one set S of m classes, a multiplier that maps a solution class to another solution class
    for m in 2..=6usize {
        let set = random_class_set(&g, m, &mut rng);
        let out = census(&set.ds, g.r, m, &[&fr], 0, 0, None);
        let ks: Vec<u32> = (1..=g.half())
            .filter(|&a| out.cnt[a as usize] > 0)
            .collect();
        assert!(ks.len() >= 2, "m={m}");
        let (a, b) = (ks[0], ks[ks.len() / 2]);
        // mu = b / a mod r
        let inv_a = pow_mod(a as u64, (g.r - 2) as u64, g.r as u64) as u32;
        let mu = g.mulmod(b, inv_a);
        if mu == 1 || mu == g.r - 1 {
            continue;
        }
        let pl = Mult::from_units(g.r, &[mu]);
        let o2 = census(&set.ds, g.r, m, &[&pl], 4, 0, None);
        assert!(
            o2.u[0][0] >= 1,
            "planted overlap (a -> b) must be recalled, m={m}"
        );
        let (_, naive) = naive_census(&set.ds, g.r, m, &[mu]);
        assert_eq!(o2.u[0][0], naive[0]);
    }
}

#[test]
fn halving_form_holds_for_every_coincidence() {
    let (_f, g) = group11();
    let mut rng = Rng::new(31);
    let fr = Mult::frobenius(&g);
    let mut total = 0;
    for m in 2..=6usize {
        for _ in 0..4 {
            let set = random_class_set(&g, (m + 4).min(14), &mut rng);
            let out = census(&set.ds, g.r, m, &[&fr], 64, 0, None);
            for s in &out.samples {
                let (pairs, ok) = halving_check(&set.ds, g.r, &s.idx, s.a, s.b, fr.mus[s.t]);
                assert!(ok && pairs >= 1, "m={m} a={} b={}", s.a, s.b);
                // not vacuous: with the wrong unit the check must fail (it ties the table entry to mu)
                let wrong = if fr.mus[s.t] + 1 == g.r - 1 || fr.mus[s.t] + 1 == 1 {
                    fr.mus[s.t] + 2
                } else {
                    fr.mus[s.t] + 1
                };
                let (_, bad) = halving_check(&set.ds, g.r, &s.idx, s.a, s.b, wrong);
                assert!(!bad, "wrong multiplier must be rejected, m={m}");
                total += 1;
            }
        }
    }
    assert!(total > 20, "enough coincidences exercised: {total}");
}

#[test]
fn census_samples_agree_with_chain_oracle() {
    let (f, g) = group11();
    let mut rng = Rng::new(41);
    let fr = Mult::frobenius(&g);
    for m in 2..=4usize {
        let set = random_class_set(&g, 9, &mut rng);
        let out = census(&set.ds, g.r, m, &[&fr], 12, 12, None);
        for s in &out.samples {
            // both targets of a coincidence are solved by the same tuple, via the chain (some order)
            let xs: Vec<u32> = s.idx.iter().map(|&k| set.xs[k]).collect();
            let ok = |a: u32| -> bool {
                let xr = g.xs[(a - 1) as usize];
                let mut v = xs.clone();
                for rot in 0..m {
                    v.rotate_left(rot.min(1));
                    if member(&f, 1, xr, &v) {
                        return true;
                    }
                }
                false
            };
            assert!(ok(s.a) && ok(s.b), "m={m}");
        }
    }
}

#[test]
fn stable_base_has_exact_equivariance_and_nonstable_does_not() {
    let f = Fq::new(17);
    let g = build_group(&f, 1).expect("group 17");
    assert_eq!(g.r, 65587);
    let st = stable_subspaces(&f, 8);
    let fr = Mult::frobenius(&g);
    let v = span(&st[0]);
    let set = BaseSet::from_elems(&g, &v);
    assert!(set.is_sigma_closed(&f));
    let out = census(&set.ds, g.r, 2, &[&fr], 0, 0, None);
    let ind = ind_stats(&out.cnt, &fr);
    assert!(
        ind.count_equal.iter().all(|&c| c == ind.ncls),
        "stable base: |Sol(a)| = |Sol(lam^j a)| for all a, j"
    );
    // a random subspace of the same dimension is not closed and not equivariant (nonempty solution sets)
    let mut rng = Rng::new(51);
    let w = span(&random_subspace(&f, 9, &mut rng));
    let set2 = BaseSet::from_elems(&g, &w);
    assert!(!set2.is_sigma_closed(&f));
    let out2 = census(&set2.ds, g.r, 3, &[&fr], 0, 0, None);
    let ind2 = ind_stats(&out2.cnt, &fr);
    assert!(ind2.count_equal.iter().any(|&c| c < ind2.ncls));
}

#[test]
fn shared_leg_lower_bound_holds() {
    // chi_j(a) = 1 (a solved by a tuple whose legs all stay in V after sigma^j) implies chi(a) = chi(lam^j a) = 1
    let f = Fq::new(11);
    let g = build_group(&f, 1).unwrap();
    let fr = Mult::frobenius(&g);
    let mut rng = Rng::new(61);
    for m in 2..=3usize {
        for _ in 0..3 {
            let v = span(&random_subspace(&f, 8, &mut rng));
            let set = BaseSet::from_elems(&g, &v);
            let out = census(&set.ds, g.r, m, &[&fr], 0, 0, None);
            let ind = ind_stats(&out.cnt, &fr);
            for j in 1..=(f.n - 1) / 2 {
                let sj = set.shared_with_shift(&f, &g, j);
                let cj = solution_counts(&sj.ds, g.r, m);
                let lowers = (1..cj.len()).filter(|&a| cj[a] > 0).count() as u64;
                assert!(ind.joint[(j - 1) as usize] >= lowers, "m={m} j={j}");
            }
        }
    }
}

#[test]
fn sigma_closure_is_empty_on_nonstable_and_small_example() {
    let f = Fq::new(11);
    let g = build_group(&f, 1).unwrap();
    let mut rng = Rng::new(71);
    let v = span(&random_subspace(&f, 8, &mut rng));
    let set = BaseSet::from_elems(&g, &v);
    let (targets, pairs, closed) = sigma_closure(&set, &f, g.r, 2);
    assert!(targets > 0 && pairs == targets * 10);
    assert_eq!(closed, 0);
}

#[test]
fn rank_orbit_basis_conjugates_add_nothing_and_controls_grow() {
    let f = Fq::new(11);
    let g = build_group(&f, 1).unwrap();
    let ob = build_orbits(&f, &g);
    assert_eq!(
        ob.n_orbits as u32 * f.n,
        g.half(),
        "all orbits of G-classes have size n"
    );
    let mut rng = Rng::new(81);
    let mut saw_growth = false;
    for m in 2..=4usize {
        let v = span(&random_subspace(&f, 7, &mut rng));
        let set = BaseSet::from_elems(&g, &v);
        let ro = rank_census(&f, &g, &ob, &set.ds, m, 12, 60, &mut rng);
        assert!(ro.n_rel > 0, "m={m}");
        assert_eq!(
            ro.rows_bad, 0,
            "every row satisfies its identity with the true logarithms"
        );
        assert_eq!(ro.incons, 0);
        assert_eq!(
            ro.nonproportional, 0,
            "orbit rows of conjugates are lam^j multiples"
        );
        assert_eq!(
            ro.rank_orbit_n, ro.rank_orbit_1,
            "conjugates add no rank in the orbit basis (m={m})"
        );
        assert!(ro.rank_plain_n >= ro.rank_plain_1);
        assert!(ro.rank_orbit_wrong >= ro.rank_orbit_n);
        if ro.rank_plain_n > ro.rank_plain_1
            || ro.rank_orbit_wrong > ro.rank_orbit_n
            || ro.rank_orbit_indep > ro.rank_orbit_1
        {
            saw_growth = true;
        }
    }
    assert!(saw_growth, "the controls must be able to see rank growth");
}

#[test]
fn null_multipliers_calibrate_the_independence_model() {
    // E[U_null] = U_pred exactly when the multiplier is uniform on the units other than +-1 (a property of the
    // model, checked here by averaging many random multiplier sets on one fixed base).
    let (_f, g) = group11();
    let mut rng = Rng::new(91);
    let set = random_class_set(&g, 14, &mut rng);
    let m = 3usize;
    let jn = 5usize;
    let mut tot_u = 0.0;
    let mut tot_p = 0.0;
    for _ in 0..400 {
        let nl = Mult::random_null(g.r, jn, &mut rng);
        let out = census(&set.ds, g.r, m, &[&nl], 0, 0, None);
        tot_u += out.u[0].iter().sum::<u64>() as f64;
        tot_p += u_pred(&out, g.r, jn);
    }
    let ratio = tot_u / tot_p;
    assert!((ratio - 1.0).abs() < 0.03, "null/model = {ratio}");
}

#[test]
fn orbit_union_is_sigma_closed_and_equivariant() {
    let f = Fq::new(11);
    let g = build_group(&f, 1).unwrap();
    let mut rng = Rng::new(97);
    let set = orbit_union(&f, &g, 3, &mut rng);
    assert_eq!(set.len(), 33);
    assert!(set.is_sigma_closed(&f));
    let fr = Mult::frobenius(&g);
    let out = census(&set.ds, g.r, 2, &[&fr], 0, 0, None);
    let ind = ind_stats(&out.cnt, &fr);
    assert!(
        ind.count_equal.iter().all(|&c| c == ind.ncls),
        "sigma-closed base: |Sol(a)| = |Sol(lam^j a)|"
    );
}

#[test]
fn orbit_strata_partition_the_counts() {
    let f = Fq::new(11);
    let g = build_group(&f, 1).unwrap();
    let ob = build_orbits(&f, &g);
    let fr = Mult::frobenius(&g);
    let mut rng = Rng::new(99);
    let nl = Mult::random_null(g.r, fr.len(), &mut rng);
    let set = orbit_union(&f, &g, 2, &mut rng);
    let orbs: Vec<u32> = set.ds.iter().map(|&d| ob.orbit[(d - 1) as usize]).collect();
    for m in 2..=4usize {
        let out = census(&set.ds, g.r, m, &[&fr, &nl], 0, 0, Some(&orbs));
        assert_eq!(out.strat_nsub.iter().sum::<u64>(), out.nsub);
        assert_eq!(out.strat_nsol.iter().sum::<u64>(), out.nsol);
        assert_eq!(
            out.strat_u[0].iter().sum::<u64>(),
            out.u[0].iter().sum::<u64>()
        );
        assert_eq!(
            out.strat_u[1].iter().sum::<u64>(),
            out.u[1].iter().sum::<u64>()
        );
        if m >= 3 {
            assert_eq!(out.strat_nsub[2], 0, "only two orbits present");
        }
    }
}

/// Every sampled coincidence of the Frobenius set is a coincidence of that very multiplier (sample.t indexes it).
#[test]
fn samples_carry_their_lag() {
    let (_f, g) = group11();
    let mut rng = Rng::new(33);
    let fr = Mult::frobenius(&g);
    let set = random_class_set(&g, 10, &mut rng);
    let out = census(&set.ds, g.r, 3, &[&fr], 32, 0, None);
    assert!(!out.samples.is_empty());
    for s in &out.samples {
        assert_eq!(fr.tabs[s.t][s.a as usize], s.b);
    }
}

/// Naive check of the valuation theorem (PROTOCOL 3.4): for every single-orbit tuple and every lag with
/// m^2 2^s (2^(j/2)+1)^2 < r there is no Frobenius coincidence; computed here without the census code.
fn naive_low_height_hits(f: &Fq, g: &Group, ob: &Orbits, ds: &[u32], m: usize) -> (u64, u64) {
    let jn = (g.n - 1) / 2;
    let mut pairs = 0u64;
    let mut hits = 0u64;
    let nx = ds.len();
    let mut idx: Vec<usize> = (0..m).collect();
    loop {
        let orbs: Vec<u32> = idx
            .iter()
            .map(|&i| ob.orbit[(ds[i] - 1) as usize])
            .collect();
        if orbs.iter().all(|&o| o == orbs[0]) {
            let mut ks: Vec<u32> = idx.iter().map(|&i| ob.k[(ds[i] - 1) as usize]).collect();
            let sp = circular_spread(&mut ks, g.n);
            // K(S) by brute force
            let mut kset: Vec<u32> = vec![];
            for mask in 0u32..(1 << m) {
                let mut sum: u64 = 0;
                for (t, &i) in idx.iter().enumerate() {
                    let d = ds[i] as u64;
                    sum += if (mask >> t) & 1 == 0 {
                        d
                    } else {
                        g.r as u64 - d
                    };
                }
                let v = (sum % g.r as u64) as u32;
                let c = v.min(g.r - v);
                if c != 0 && !kset.contains(&c) {
                    kset.push(c);
                }
            }
            for j in 1..=jn {
                if low_height(m, sp, j, g.r) {
                    pairs += 1;
                    let mu = g.lam_pow(j);
                    for &a in &kset {
                        let v = g.mulmod(mu, a);
                        let b = v.min(g.r - v);
                        if kset.contains(&b) {
                            hits += 1;
                        }
                    }
                }
            }
        }
        // next combination
        let mut k = m;
        loop {
            if k == 0 {
                let _ = f;
                return (pairs, hits);
            }
            k -= 1;
            if idx[k] < nx - (m - k) {
                idx[k] += 1;
                for t in k + 1..m {
                    idx[t] = idx[t - 1] + 1;
                }
                break;
            }
            if k == 0 {
                return (pairs, hits);
            }
        }
    }
}

#[test]
fn low_height_single_orbit_pairs_never_coincide() {
    for &(a, n) in &[(1u32, 11u32), (0, 13), (0, 19)] {
        let f = Fq::new(n);
        let g = build_group(&f, a).unwrap();
        let ob = build_orbits(&f, &g);
        let fr = Mult::frobenius(&g);
        let mut rng = Rng::new(1000 + n as u64);
        let nl = Mult::random_null(g.r, fr.len(), &mut rng);
        let mut total_pairs = 0u64;
        for m in 2..=4usize {
            // two whole orbits: plenty of single-orbit tuples with small and large spread
            let set = orbit_union(&f, &g, 2, &mut rng);
            let orbs: Vec<u32> = set.ds.iter().map(|&d| ob.orbit[(d - 1) as usize]).collect();
            let ks: Vec<u32> = set.ds.iter().map(|&d| ob.k[(d - 1) as usize]).collect();
            let oi = OrbInfo {
                orbit: &orbs,
                k: Some(&ks),
                n: g.n,
            };
            let out = census_ex(&set.ds, g.r, m, &[&fr, &nl], 0, 0, Some(&oi));
            let (np, nh) = naive_low_height_hits(&f, &g, &ob, &set.ds, m);
            assert_eq!(out.s1l_pairs, np, "pair count a={a} n={n} m={m}");
            assert_eq!(nh, 0, "naive: theorem violated a={a} n={n} m={m}");
            assert_eq!(
                out.s1l_u[0], 0,
                "census: Frobenius coincidence on a provably free pair a={a} n={n} m={m}"
            );
            total_pairs += out.s1l_pairs;
        }
        assert!(
            total_pairs > 100,
            "the check is not vacuous a={a} n={n}: {total_pairs}"
        );
    }
}

/// The null has no such obstruction: on the same low-height pairs random multipliers do hit (n = 19, m = 4 has
/// enough weight), so the zero above is a property of Frobenius, not of the pair selection.
#[test]
fn null_hits_exist_on_low_height_pairs_when_weight_is_large() {
    let f = Fq::new(13);
    let g = build_group(&f, 0).unwrap();
    let ob = build_orbits(&f, &g);
    let fr = Mult::frobenius(&g);
    let mut tot = 0u64;
    let mut expected = 0.0;
    for seed in 0..30u64 {
        let mut rng = Rng::new(5000 + seed);
        let nl = Mult::random_null(g.r, fr.len(), &mut rng);
        let set = orbit_union(&f, &g, 2, &mut rng);
        let orbs: Vec<u32> = set.ds.iter().map(|&d| ob.orbit[(d - 1) as usize]).collect();
        let ks: Vec<u32> = set.ds.iter().map(|&d| ob.k[(d - 1) as usize]).collect();
        let oi = OrbInfo {
            orbit: &orbs,
            k: Some(&ks),
            n: g.n,
        };
        let out = census_ex(&set.ds, g.r, 3, &[&fr, &nl], 0, 0, Some(&oi));
        assert_eq!(out.s1l_u[0], 0);
        tot += out.s1l_u[1];
        expected += out.s1l_cc1 as f64 / ((g.r as f64 - 3.0) / 2.0);
    }
    assert!(expected > 5.0, "expected null hits {expected}");
    assert!(tot > 0, "null hits {tot} vs expected {expected}");
}

#[test]
fn planted_multipliers_guarantee_a_hit_per_lag() {
    let (_f, g) = group11();
    let mut rng = Rng::new(123);
    let fr = Mult::frobenius(&g);
    for m in 2..=5usize {
        for _ in 0..5 {
            let set = random_class_set(&g, 9, &mut rng);
            let pl = Mult::planted(g.r, &set.ds, m, fr.len(), &mut rng).expect("planted");
            let n2 = Mult::random_null(g.r, fr.len(), &mut rng);
            let out = census_ex(&set.ds, g.r, m, &[&fr, &n2, &pl], 0, 0, None);
            assert!(
                out.u[2].iter().all(|&h| h >= 1),
                "every lag has its guaranteed hit, m={m}"
            );
            let (_, naive) = naive_census(&set.ds, g.r, m, &pl.mus);
            assert_eq!(out.u[2], naive);
        }
    }
}

#[test]
fn sigma_closure_runs_its_full_path_on_orbit_unions() {
    // orbit unions are sigma-closed, so every image of every solution is inside the base and the closure test
    // really compares solution sets (the non-stable case exits early); closed pairs must still be zero here
    let f = Fq::new(11);
    let g = build_group(&f, 1).unwrap();
    let mut rng = Rng::new(131);
    let mut checked = 0;
    for m in 2..=4usize {
        let set = orbit_union(&f, &g, 2, &mut rng);
        assert!(set.is_sigma_closed(&f));
        let (targets, pairs, closed) = sigma_closure(&set, &f, g.r, m);
        assert!(targets > 0 && pairs == targets * 10);
        assert_eq!(closed, 0, "m={m}");
        checked += pairs;
    }
    assert!(checked > 0);
}

#[test]
fn rank_cap_keeps_the_controls_able_to_grow() {
    let f = Fq::new(13);
    let g = build_group(&f, 0).unwrap();
    let ob = build_orbits(&f, &g);
    let mut rng = Rng::new(141);
    let (mut wrong_grew, mut indep_grew, mut matched) = (0, 0, 0);
    for m in 3..=5usize {
        for _ in 0..4 {
            let v = span(&random_subspace(&f, 7, &mut rng));
            let set = BaseSet::from_elems(&g, &v);
            let ro = rank_census(&f, &g, &ob, &set.ds, m, 16, 40, &mut rng);
            assert!(
                ro.row_cap_eff <= (ro.orbits_touched / 3).max(2),
                "cap {} touched {}",
                ro.row_cap_eff,
                ro.orbits_touched
            );
            assert!(ro.n_rel <= ro.row_cap_eff);
            assert_eq!(ro.nonproportional, 0);
            assert_eq!(ro.rank_orbit_n, ro.rank_orbit_1);
            if ro.rank_orbit_1 < ro.cols_orbit {
                wrong_grew += (ro.rank_orbit_wrong > ro.rank_orbit_n) as usize;
                indep_grew += (ro.rank_orbit_indep > ro.rank_orbit_1) as usize;
            }
            matched += (ro.n_rel_indep >= ro.indep_need) as usize;
        }
    }
    assert!(
        wrong_grew >= 6 && indep_grew >= 6,
        "controls grew: wrong {wrong_grew} indep {indep_grew}"
    );
    assert!(
        matched >= 6,
        "independent control row-matched in {matched} of 12"
    );
}
