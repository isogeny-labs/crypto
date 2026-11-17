//! Certified edge resultants and shared-vertex univariate gcd consequences.
//! No Groebner basis or witness is used in construction.
include!("kernels/projection.rs");

#[cfg(test)]
mod packed_product_tests {
    use super::*;
    #[test]
    fn packed_products_and_local_coefficients_equal_legacy_with_counters() {
        let mut state = 0x73ae19u64;
        for p in [2147483647u64, 9007199254740881] {
            for n in [3, 4, 6, 16, 17] {
                for _ in 0..12 {
                    let mut make = || {
                        let mut f = Poly::new();
                        for _ in 0..12 {
                            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                            let mut m = vec![0; n];
                            m[state as usize % n] += 1;
                            m[(state >> 17) as usize % n] += 1;
                            f.insert(m, state % p);
                        }
                        f
                    };
                    let a = make();
                    let b = make();
                    let row = Row {
                        poly: a.clone(),
                        proof: a
                            .iter()
                            .enumerate()
                            .map(|(i, (m, c))| ((i % 3, m.clone()), *c))
                            .collect(),
                    };
                    reset_counters();
                    let old = with_native_kernel_bits(103, || product(&a, &b, p));
                    let count = serde_json::to_value(counters()).unwrap();
                    reset_counters();
                    let new = with_native_kernel_bits(231, || product(&a, &b, p));
                    assert_eq!(old, new);
                    assert_eq!(count, serde_json::to_value(counters()).unwrap());
                    reset_counters();
                    let old = with_native_kernel_bits(103, || times(&row, &b, p));
                    let count = serde_json::to_value(counters()).unwrap();
                    reset_counters();
                    let new = with_native_kernel_bits(231, || times(&row, &b, p));
                    assert_eq!(old, new);
                    assert_eq!(count, serde_json::to_value(counters()).unwrap());
                }
            }
        }
    }
    #[test]
    fn cancellation_empty_and_outside_packing_domain_keep_exact_semantics() {
        let p = 2147483647;
        let a = Poly::from([(vec![1, 0, 0], 1), (vec![0, 1, 0], 1)]);
        let b = Poly::from([(vec![1, 0, 0], 1), (vec![0, 1, 0], p - 1)]);
        let expected = with_native_kernel_bits(103, || product(&a, &b, p));
        let actual = with_native_kernel_bits(231, || product(&a, &b, p));
        assert_eq!(expected, actual);
        assert!(!actual.contains_key(&vec![1, 1, 0]));
        for left in [
            Poly::new(),
            Poly::from([(vec![65, 0, 0], 1)]),
            Poly::from([(vec![1], 1)]),
        ] {
            assert_eq!(
                with_native_kernel_bits(103, || product(&left, &b, p)),
                with_native_kernel_bits(231, || product(&left, &b, p))
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn embedded_cycle(n: usize, p: u64) -> Vec<Poly> {
        workload::generate(p, 3, 7)
            .unwrap()
            .system
            .decode()
            .unwrap()
            .into_iter()
            .map(|f| {
                f.into_iter()
                    .map(|(mut m, c)| {
                        m.resize(n, 0);
                        (m, c)
                    })
                    .collect()
            })
            .collect()
    }
    fn compare_selected(inputs: &[Poly], n: usize, p: u64) -> (Vertex, Work) {
        let pairs = cheap::consequences(inputs, n, p);
        let (selected, vertex, work) = selected_quadratic(inputs, n, p, &pairs).unwrap();
        let (all, vertices, all_work) = all_quadratic(inputs, n, p);
        let expected = vertices
            .iter()
            .filter(|v| (2..=8).contains(&v.degree))
            .max_by_key(|v| v.variable)
            .unwrap();
        assert_eq!(vertex.variable, expected.variable);
        assert_eq!(vertex.degree, expected.degree);
        let row = all
            .iter()
            .find(|r| {
                r.poly.keys().all(|m| {
                    m.iter()
                        .enumerate()
                        .all(|(i, e)| i == expected.variable || *e == 0)
                }) && degree(&r.poly) == expected.degree
            })
            .unwrap();
        assert_eq!(&selected, row, "original cofactor row must match exactly");
        assert!(verify(&selected, inputs, p));
        assert!(work.edge_resultants <= all_work.edge_resultants);
        (vertex, work)
    }
    #[test]
    fn selected_root_skips_unused_and_single_incident_vertices() {
        for p in [2147483647, 9007199254740881] {
            for n in [4, 6] {
                let mut inputs = embedded_cycle(n, p);
                if n == 6 {
                    for z in [0, 1] {
                        inputs.push(cheap::f3(n, [4, 5], 2, 3, z, p));
                    }
                }
                let (root, work) = compare_selected(&inputs, n, p);
                assert_eq!(root.variable, 2);
                assert_eq!(work.edge_resultants, 2);
                assert_eq!(work.vertex_gcd_folds, 1);
                assert!(quotient::compact::solve(&inputs, n, p).is_err());
                assert!(quotient::compact::solve_all_vertices(&inputs, n, p).is_err());
            }
        }
    }
    #[test]
    fn selected_root_caches_edges_when_higher_vertices_are_ineligible() {
        let p = 2147483647;
        let mut inputs = embedded_cycle(6, p);
        for (edge, a, b) in [([3, 4], 2, 3), ([4, 5], 5, 7), ([3, 5], 9, 11)] {
            for z in [0, 1] {
                inputs.push(cheap::f3(6, edge, a, b, z, p));
            }
        }
        let (root, work) = compare_selected(&inputs, 6, p);
        assert_eq!(root.variable, 2);
        // Three upper vertices rejected, then the lower cycle supplies a root.
        // Shared upper edges are each constructed once despite that fallback.
        assert_eq!(work.edge_resultants, 5);
        assert_eq!(work.vertex_gcd_folds, 4);
    }
    #[test]
    fn direct_quadratic_identity_agrees_with_sylvester_in_degenerate_and_scaled_cases() {
        for p in [5, 7, 101] {
            for (a, b) in [(0, 0), (0, 3 % p), (2, 3 % p), (p - 1, p - 1)] {
                for (u, v) in [(0, 1), (1, 2), (1, p - 1)] {
                    let mut f = vec![
                        cheap::f3(2, [0, 1], a, b, u, p),
                        cheap::f3(2, [0, 1], a, b, v, p),
                    ];
                    for c in f[1].values_mut() {
                        *c = mul(*c, p - 1, p);
                    }
                    let rows = input_rows(&f, 2);
                    let direct = quadratic_resultant(&rows[0], &rows[1], [0, 1], 2, p);
                    let determinant = resultant(&rows[0], &rows[1], [0, 1], 2, p);
                    assert_eq!(direct.poly, determinant.poly);
                    assert!(verify(&direct, &f, p));
                    assert!(certificate_degree(&direct, &f) <= 11);
                }
            }
        }
    }
    #[test]
    fn quadratic_projection_preserves_full_ideal_on_both_requested_fields() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 4] {
                let fixture = workload::generate(p, n, 7).unwrap();
                let f = fixture.system.decode().unwrap();
                let (new, info) = augment_quadratic(&f, n, p);
                let (old, _) = augment(&f, n, p);
                assert_eq!(new,old,"construction refactor must preserve the ordered augmented inputs on these fixtures");
                assert_eq!(info.len(), n);
                assert!(info.iter().all(|v| v.degree == 4));
                let original = reference(&f, n, p).unwrap();
                let augmented = reference_criteria(&new, n, p).unwrap();
                assert_eq!(
                    original
                        .iter()
                        .map(|r| r.poly.clone())
                        .collect::<BTreeSet<_>>(),
                    augmented.iter().map(|r| r.poly.clone()).collect()
                );
            }
        }
    }
    #[test]
    fn edge_projection_is_certified_and_reused_vertex_gcds_are_quartic() {
        for p in [2147483647, 9007199254740881] {
            let fixture = workload::generate(p, 3, 7).unwrap();
            let f = fixture.system.decode().unwrap();
            let (a, info) = augment(&f, 3, p);
            assert_eq!(info.len(), 3);
            assert!(info.iter().all(|v| v.degree == 4));
            let gb = reference(&f, 3, p).unwrap();
            let ab = reference(&a, 3, p).unwrap();
            assert_eq!(
                gb.iter().map(|r| r.poly.clone()).collect::<BTreeSet<_>>(),
                ab.iter().map(|r| r.poly.clone()).collect()
            );
        }
    }
    #[test]
    fn no_shared_vertex_falls_back_to_cubics() {
        let fixture = workload::generate(2147483647, 2, 7).unwrap();
        let f = fixture.system.decode().unwrap();
        let (a, info) = augment(&f, 2, fixture.system.prime);
        assert!(info.is_empty());
        assert_eq!(a.len(), 4);
    }
}
