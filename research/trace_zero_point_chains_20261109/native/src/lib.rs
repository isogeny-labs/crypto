#[cfg(test)]
mod constant_fold_tests;
#[cfg(test)]
mod kummer_tests;
#[cfg(test)]
mod subspace_tests;
#[cfg(test)]
mod tau_transport_tests;
#[cfg(test)]
mod tests {
    use full_trace_zero_backend::binary_ecc::{
        curve::{point_add, point_neg, scalar_mul},
        BinaryPoint, F2mElement, TraceZeroVariety,
    };
    use full_trace_zero_backend::cryptanalysis::{
        sat::SolveResult,
        trace_zero_point_chain::{
            decompose_full_point_chain, full_point_chain_max_variables, point_chain_branches,
            FullPointChainEncoding, PointAdditionBranch as Branch,
        },
        trace_zero_relations::{
            TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
        },
    };
    use num_bigint::BigUint;
    use std::time::{Duration, Instant};
    use trace_zero_leads::fixture;

    fn subgroup() -> Vec<BinaryPoint> {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let mut points = vec![BinaryPoint::Infinity];
        for x in 0..32u32 {
            for y in 0..32u32 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&BigUint::from(x), 5),
                    y: F2mElement::from_biguint(&BigUint::from(y), 5),
                };
                if v.contains(&p) {
                    points.push(p);
                }
            }
        }
        points
    }
    fn branch(p: &BinaryPoint, q: &BinaryPoint) -> Branch {
        if *p == BinaryPoint::Infinity {
            return Branch::ReturnRight;
        }
        if *p == point_neg(q) {
            Branch::Opposite
        } else if p == q {
            Branch::Double
        } else {
            Branch::Generic
        }
    }
    fn exists(base: &[BinaryPoint], target: &BinaryPoint, m: usize) -> bool {
        fn visit(
            base: &[BinaryPoint],
            target: &BinaryPoint,
            left: usize,
            sum: &BinaryPoint,
        ) -> bool {
            if left == 0 {
                return sum == target;
            }
            base.iter()
                .any(|p| visit(base, target, left - 1, &point_add(&fixture(5), sum, p)))
        }
        visit(base, target, m, &BinaryPoint::Infinity)
    }
    #[test]
    fn every_small_finite_pair_has_the_exact_addition_branch_and_rejects_wrong_output() {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let group = subgroup();
        let base = &group[1..];
        let mut checked = 0;
        for p in base {
            for q in base {
                let t = point_add(&c, p, q);
                let branches = [branch(p, q)];
                let mut e = FullPointChainEncoding::build(&v, base, &t, &branches, 10_000, 100_000)
                    .unwrap();
                e.pin_summands(&[p.clone(), q.clone()]).unwrap();
                assert_eq!(e.solver.solve(), SolveResult::Sat, "{p:?} {q:?}");
                assert!(e.verify_model());
                checked += 1;
                if t != BinaryPoint::Infinity {
                    let wrong = base.iter().find(|p| **p != t).unwrap();
                    let mut bad =
                        FullPointChainEncoding::build(&v, base, wrong, &branches, 10_000, 100_000)
                            .unwrap();
                    bad.pin_summands(&[p.clone(), q.clone()]).unwrap();
                    assert_eq!(bad.solver.solve(), SolveResult::Unsat);
                }
            }
        }
        println!("EXACT_PAIR_BRANCHES n=5 ordered_pairs={checked}");
    }
    #[test]
    fn nonzero_denominator_and_identity_branch_admission_are_enforced() {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let group = subgroup();
        let g = group[1].clone();
        let t = point_add(&c, &g, &g);
        let mut e = FullPointChainEncoding::build(
            &v,
            &[g.clone()],
            &t,
            &[Branch::Generic],
            10_000,
            100_000,
        )
        .unwrap();
        e.pin_summands(&[g.clone(), g.clone()]).unwrap();
        assert_eq!(e.solver.solve(), SolveResult::Unsat);
        assert!(FullPointChainEncoding::build(
            &v,
            &[g.clone()],
            &g,
            &[Branch::ReturnRight],
            10_000,
            100_000
        )
        .is_err());
        assert!(FullPointChainEncoding::build(
            &v,
            &[g.clone()],
            &g,
            &[Branch::Opposite],
            10_000,
            100_000
        )
        .is_err());
        assert!(
            FullPointChainEncoding::build(&v, &[g.clone()], &t, &[Branch::Double], 1, 100_000)
                .is_err()
        );
        assert!(
            FullPointChainEncoding::build(&v, &[g.clone()], &t, &[Branch::Double], 10_000, 0)
                .is_err()
        );
        assert_eq!(full_point_chain_max_variables(131, 3), Some(106110));
    }
    #[test]
    fn branch_paths_cover_every_small_triple_including_identity_intermediates() {
        let c = fixture(5);
        let group = subgroup();
        let base = &group[1..];
        for p in base {
            for q in base {
                for r in base {
                    let s = point_add(&c, p, q);
                    let t = point_add(&c, &s, r);
                    let path = vec![branch(p, q), branch(&s, r)];
                    let (patterns, complete) =
                        point_chain_branches(3, t == BinaryPoint::Infinity, 100);
                    assert!(complete && patterns.contains(&path));
                }
            }
        }
        assert_eq!(point_chain_branches(3, false, 100).0.len(), 5);
        assert_eq!(point_chain_branches(3, true, 100).0.len(), 2);
        assert!(!point_chain_branches(3, false, 1).1);
    }
    #[test]
    fn complete_point_oracle_matches_all_small_targets_and_sign_restrictions() {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let group = subgroup();
        let g = group[1].clone();
        for base in [
            vec![g.clone()],
            vec![g.clone(), point_neg(&g)],
            vec![g.clone(), scalar_mul(&c, &g, &BigUint::from(2u32))],
        ] {
            for m in [2, 3, 4] {
                for t in &group {
                    let result = decompose_full_point_chain(
                        &v,
                        &base,
                        t,
                        m,
                        Limits {
                            max_patterns: 128,
                            ..Limits::default()
                        },
                    );
                    assert_ne!(result.disposition, Disposition::Inconclusive, "{result:?}");
                    assert_eq!(
                        result.disposition == Disposition::Found,
                        exists(&base, t, m)
                    );
                    if let Some(path) = result.indices {
                        assert_eq!(
                            path.iter()
                                .fold(BinaryPoint::Infinity, |s, &i| point_add(&c, &s, &base[i])),
                            *t
                        );
                    }
                }
            }
        }
        let capped = decompose_full_point_chain(
            &v,
            &[g.clone()],
            &g,
            3,
            Limits {
                max_patterns: 1,
                ..Limits::default()
            },
        );
        assert_eq!(capped.disposition, Disposition::Inconclusive);
    }
    #[test]
    fn wide_identity_path_replays_without_membership_ordinate_search() {
        for n in [83, 131] {
            let c = fixture(n);
            let v = TraceZeroVariety::new(&c).unwrap();
            let base = vec![c.generator.clone(), point_neg(&c.generator)];
            let mut e = FullPointChainEncoding::build(
                &v,
                &base,
                &c.generator,
                &[Branch::Opposite, Branch::ReturnRight],
                300_000,
                1_000_000,
            )
            .unwrap();
            e.pin_summands(&[
                c.generator.clone(),
                point_neg(&c.generator),
                c.generator.clone(),
            ])
            .unwrap();
            e.solver.conflict_budget = 2000;
            e.solver.deadline = Some(Instant::now() + Duration::from_secs(2));
            assert_eq!(e.solver.solve(), SolveResult::Sat);
            assert!(e.verify_model());
            println!(
                "FULL_POINT_IDENTITY n={n} variables={} clauses={} xors={}",
                e.solver.n_vars(),
                e.solver.n_clauses(),
                e.solver.n_xors()
            );
        }
    }
}
