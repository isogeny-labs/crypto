//! Certified rational-univariate representation and grevlex change of order.
//! No witness or reference Groebner basis is used by construction.
pub mod compact;
include!("kernels/quotient.rs");

#[cfg(test)]
mod batch_unit_tests {
    use super::*;
    #[test]
    fn batched_units_equal_separate_checked_inverses_in_nonfield_quotients() {
        for p in [101u64, 2147483647, 9007199254740881] {
            for n in [3, 6, 16] {
                for t in [0, n - 1] {
                    let m = |degree: usize| {
                        let mut m = vec![0; n];
                        m[t] = degree;
                        m
                    };
                    let h = Poly::from([(m(2), 1)]);
                    let values: Vec<_> = (1..=5)
                        .map(|c| Poly::from([(m(0), c), (m(1), 1)]))
                        .collect();
                    let batch = batch_inverse_mod(&values, &h, n, t, p).unwrap();
                    for (a, b) in values.iter().zip(&batch) {
                        assert_eq!(*b, inverse_mod(a, &h, n, t, p).unwrap());
                    }
                    let mut nonunits = values.clone();
                    nonunits[2] = Poly::from([(m(1), 1)]);
                    assert!(batch_inverse_mod(&nonunits, &h, n, t, p).is_none());
                    assert!(
                        batch_inverse_mod(&values, &Poly::from([(m(2), 2)]), n, t, p).is_none()
                    );
                    assert!(batch_inverse_mod(&values[..1], &h, n, t, p).is_none());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::product;

    fn staircase_trace(
        n: usize,
        cap: usize,
        ideal: &[Mono],
        frontier: bool,
    ) -> (Vec<Mono>, Vec<Mono>, usize) {
        let mut border = MonomialFrontier::new(n, cap);
        let mut grid = if frontier {
            Vec::new()
        } else {
            (0..=cap)
                .flat_map(|d| monomials_legacy(n, d, true).into_iter().rev())
                .collect::<Vec<_>>()
        }
        .into_iter();
        let mut known: Vec<Mono> = Vec::new();
        let mut trace = Vec::new();
        let mut standards = Vec::new();
        loop {
            let next = if frontier { border.pop() } else { grid.next() };
            let Some(m) = next else {
                break;
            };
            if known.iter().any(|lm| divides(lm, &m)) {
                continue;
            }
            trace.push(m.clone());
            if ideal.iter().any(|lm| divides(lm, &m)) {
                known.push(m);
            } else {
                if frontier {
                    border.expand(&m);
                }
                standards.push(m);
            }
        }
        (trace, standards, border.seen.len())
    }
    #[test]
    fn frontier_matches_exact_grid_order_on_mixed_nonradical_staircases() {
        for n in [3, 8, 16] {
            for active in [(0, n - 1), (n - 2, n - 1)] {
                for mixed in [false, true] {
                    let mut ideal = Vec::new();
                    for i in 0..n {
                        let mut m = vec![0; n];
                        m[i] = if i == active.0 || i == active.1 { 2 } else { 1 };
                        ideal.push(m);
                    }
                    if mixed {
                        let mut m = vec![0; n];
                        m[active.0] = 1;
                        m[active.1] = 1;
                        ideal.push(m);
                    }
                    let dimension = if mixed { 3 } else { 4 };
                    let old = staircase_trace(n, dimension, &ideal, false);
                    let new = staircase_trace(n, dimension, &ideal, true);
                    assert_eq!(new.0, old.0);
                    assert_eq!(new.1, old.1);
                    assert_eq!(new.1.len(), dimension);
                    assert!(new.2 <= 1 + n * dimension);
                }
            }
        }
        let ideal: Vec<_> = (0..3)
            .map(|i| {
                let mut m = vec![0; 3];
                m[i] = 2;
                m
            })
            .collect();
        let old = staircase_trace(3, 8, &ideal, false);
        let new = staircase_trace(3, 8, &ideal, true);
        assert_eq!(new.0, old.0);
        assert_eq!(new.1, old.1);
        assert_eq!(new.1.len(), 8);
        assert!(new.2 <= 25);
    }
    #[test]
    fn frontier_full_bases_provenance_and_field_counters_equal_grid_on_both_fields() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, MAX_VARIABLES] {
                for root in [0, n - 1] {
                    let edges: Vec<_> = (0..n).filter(|i| *i != root).map(|i| (root, i)).collect();
                    #[cfg(feature = "scale-screen")]
                    let system = workload::generate_scale_graph(p, n, 7, &edges)
                        .unwrap()
                        .system;
                    #[cfg(not(feature = "scale-screen"))]
                    let system = workload::generate_graph(p, n, 7, &edges).unwrap().system;
                    let inputs = system.decode().unwrap();
                    reset_counters();
                    let old = with_native_kernel_bits(39, || solve_product(&inputs, n, p)).unwrap();
                    let counts = serde_json::to_value(counters()).unwrap();
                    reset_counters();
                    let new =
                        with_native_kernel_bits(103, || solve_product(&inputs, n, p)).unwrap();
                    assert_eq!(new.0, old.0);
                    assert_eq!(
                        serde_json::to_value(new.1).unwrap(),
                        serde_json::to_value(old.1).unwrap()
                    );
                    assert_eq!(serde_json::to_value(counters()).unwrap(), counts);
                    assert_eq!(native_kernel_bits(), 0);
                }
            }
        }
    }
    #[test]
    fn frontier_keeps_coordinate_failure_and_forgery_checks() {
        let p = 101;
        let mut r0 = variable(3, 0);
        axpy(&mut r0, &variable(3, 2), p - 1, p);
        let mut r1 = variable(3, 1);
        axpy(&mut r1, &variable(3, 2), p - 1, p);
        axpy(&mut r1, &one(3), p - 1, p);
        let h = BTreeMap::from([(vec![0, 0, 2], 1), (vec![0, 0, 0], 100)]);
        let inputs = vec![r0, r1, h];
        let rows = input_rows(&inputs, 3);
        let mut rules = vec![Some(rows[0].clone()), Some(rows[1].clone()), None];
        let old = change_order_grid(&rows[2], &rules, 2, p).unwrap();
        assert_eq!(change_order_frontier(&rows[2], &rules, 2, p).unwrap(), old);
        rules[0] = None;
        assert_eq!(
            change_order_frontier(&rows[2], &rules, 2, p).unwrap_err(),
            change_order_grid(&rows[2], &rules, 2, p).unwrap_err()
        );
        rules[0] = Some(rows[0].clone());
        axpy(&mut rules[0].as_mut().unwrap().poly, &one(3), 1, p);
        let forged = change_order_frontier(&rows[2], &rules, 2, p).unwrap();
        assert_eq!(forged, change_order_grid(&rows[2], &rules, 2, p).unwrap());
        assert!(!check_reference(&forged, &inputs, 3, p));
    }
    #[test]
    fn polynomial_containment_keeps_full_remainders_and_rejects_nonzero_inputs() {
        let p = 101;
        let t = 2;
        let mut r0 = variable(3, 0);
        axpy(&mut r0, &variable(3, t), p - 1, p);
        let mut r1 = variable(3, 1);
        axpy(&mut r1, &variable(3, t), p - 1, p);
        axpy(&mut r1, &one(3), p - 1, p);
        let h = BTreeMap::from([(vec![0, 0, 2], 1), (vec![0, 0, 0], 100)]);
        let inputs = vec![r0, r1, h];
        let rows = input_rows(&inputs, 3);
        let rules = vec![Some(rows[0].clone()), Some(rows[1].clone()), None];
        let bare = |r: &Row| Row {
            poly: r.poly.clone(),
            proof: Proof::new(),
        };
        let bare_rules: Vec<_> = rules.iter().map(|r| r.as_ref().map(bare)).collect();
        for f in inputs.iter().chain(std::iter::once(&one(3))) {
            let full = rewrite(
                Row {
                    poly: f.clone(),
                    proof: Proof::new(),
                },
                &rows[2],
                &rules,
                t,
                p,
            )
            .unwrap();
            let lean = rewrite(
                Row {
                    poly: f.clone(),
                    proof: Proof::new(),
                },
                &bare(&rows[2]),
                &bare_rules,
                t,
                p,
            )
            .unwrap();
            assert_eq!(full.poly, lean.poly);
            if f == &one(3) {
                assert_eq!(lean.poly, one(3));
            } else {
                assert!(lean.poly.is_empty());
            }
            assert!(lean.proof.is_empty());
        }
    }
    #[test]
    fn six_variable_development_basis_fits_unchanged_caps() {
        for p in [2147483647, 9007199254740881] {
            let system = workload::generate(p, 6, 7).unwrap().system;
            let inputs = system.decode().unwrap();
            let (basis, info) = solve(&inputs, 6, p).unwrap();
            assert_eq!(info.quotient_dimension, 4);
            assert_eq!(info.propagation_policy, "breadth-first");
            assert_eq!(info.derivation_depths.iter().copied().max(), Some(3));
            assert!(basis.iter().all(|r| r.proof.len() <= PROOF_CAP));
            assert!(check_reference_legacy(&basis, &inputs, 6, p));
            if p == 2147483647 {
                let legacy = solve_input_order(&inputs, 6, p).unwrap_err();
                assert!(legacy.contains("phase=change-order"));
                assert!(legacy.contains("[1, 2, 3, 4, 1, 0]"));
            }
        }
    }
    #[test]
    fn missing_rules_fail_and_forged_rules_cannot_pass_final_check() {
        let p = 101;
        let mut r0 = variable(3, 0);
        axpy(&mut r0, &variable(3, 2), p - 1, p);
        let mut r1 = variable(3, 1);
        axpy(&mut r1, &variable(3, 2), p - 1, p);
        axpy(&mut r1, &one(3), p - 1, p);
        let h = BTreeMap::from([(vec![0, 0, 2], 1), (vec![0, 0, 0], 100)]);
        let inputs = vec![r0, r1, h];
        let rows = input_rows(&inputs, 3);
        let mut rules = vec![Some(rows[0].clone()), Some(rows[1].clone()), None];
        let valid = change_order(&rows[2], &rules, 2, p).unwrap();
        assert!(check_reference(&valid, &inputs, 3, p));
        rules[0] = None;
        assert!(change_order(&rows[2], &rules, 2, p).is_err());
        rules[0] = Some(rows[0].clone());
        axpy(&mut rules[0].as_mut().unwrap().poly, &one(3), 1, p);
        let forged = change_order(&rows[2], &rules, 2, p).unwrap();
        assert!(!check_reference(&forged, &inputs, 3, p));
    }
    #[test]
    fn nonunits_are_not_inverted_and_unit_identity_is_exact() {
        let h = BTreeMap::from([(vec![2], 1), (vec![0], 100)]);
        let nonunit = BTreeMap::from([(vec![1], 1), (vec![0], 100)]);
        assert!(inverse_mod(&nonunit, &h, 1, 0, 101).is_none());
        let unit = BTreeMap::from([(vec![1], 1), (vec![0], 2)]);
        let inverse = inverse_mod(&unit, &h, 1, 0, 101).unwrap();
        let actual = normal_form(
            Row {
                poly: product(&unit, &inverse, 101),
                proof: Proof::new(),
            },
            &[Row {
                poly: h,
                proof: Proof::new(),
            }],
            101,
        );
        assert_eq!(actual.poly, one(1));
    }
    #[test]
    fn quotient_basis_matches_full_reference_on_both_fields() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 4] {
                let fixture = workload::generate(p, n, 7).unwrap();
                let inputs = fixture.system.decode().unwrap();
                let (basis, info) = solve(&inputs, n, p).unwrap();
                let reference = reference(&inputs, n, p).unwrap();
                assert_eq!(
                    basis.iter().map(|r| &r.poly).collect::<Vec<_>>(),
                    reference.iter().map(|r| &r.poly).collect::<Vec<_>>()
                );
                assert_eq!(info.quotient_dimension, 4);
                assert_eq!(info.coordinates_constructed, n - 1);
                assert!(check_reference_legacy(&basis, &inputs, n, p));
            }
        }
    }
    #[test]
    fn dependence_requires_exact_linear_membership() {
        let cols = vec![vec![1, 0, 0], vec![0, 1, 0]];
        assert_eq!(dependence(&[3, 7, 0], &cols, 101), Some(vec![3, 7]));
        assert_eq!(dependence(&[3, 7, 1], &cols, 101), None);
    }
    #[test]
    fn unsupported_shapes_are_explicit_failures() {
        assert!(solve(&[], 2, 101).is_err());
        assert!(solve(&[Poly::new()], 3, 101).is_err());
        let f = vec![
            cheap::f3(3, [0, 1], 2, 3, 1, 101),
            cheap::f3(3, [0, 1], 2, 3, 2, 101),
        ];
        assert!(solve(&f, 3, 101).is_err());
    }
}
