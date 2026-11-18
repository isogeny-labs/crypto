//! Acyclic local identities: checked shared consequences, without expanding
//! every later certificate back into the original input generators.
//! Polynomial construction kernels are shared with the expanded control.
include!("../kernels/compact.rs");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn root_dag_handles_connected_paths_and_refuses_disconnected_inputs() {
        for p in [2147483647, 9007199254740881] {
            let n = 6;
            let edges: Vec<_> = (0..n - 1).map(|i| (i, i + 1)).collect();
            let system = workload::generate_graph(p, n, 7, &edges).unwrap().system;
            let inputs = system.decode().unwrap();
            let old = solve_sparse_product(&inputs, n, p).unwrap().output;
            let new = solve_sparse_dag_product(&inputs, n, p).unwrap();
            assert_eq!(
                new.certificate
                    .basis
                    .iter()
                    .map(|r| &r.poly)
                    .collect::<Vec<_>>(),
                old.certificate
                    .basis
                    .iter()
                    .map(|r| &r.poly)
                    .collect::<Vec<_>>()
            );
            assert!(verify_dag_product(&new.certificate, &inputs, n, p));
            assert!(solve_sparse_dag_product(&inputs[..2], n, p).is_err());
        }
    }
    #[test]
    fn root_dag_retains_full_ideal_and_old_version_budget() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 6] {
                for hub in [0, n - 1] {
                    let edges: Vec<_> = (0..n).filter(|v| *v != hub).map(|v| (hub, v)).collect();
                    let system = workload::generate_graph(p, n, 7, &edges).unwrap().system;
                    let inputs = system.decode().unwrap();
                    let old = solve_sparse_product(&inputs, n, p).unwrap().output;
                    for bits in [103, 231, 487] {
                        let new = with_native_kernel_bits(bits, || {
                            solve_sparse_dag_product(&inputs, n, p)
                        })
                        .unwrap();
                        assert_eq!(new.info.root_projection_definitions, 2);
                        assert_eq!(new.certificate.definitions.len(), n + 2);
                        assert_eq!(
                            new.certificate
                                .basis
                                .iter()
                                .map(|r| &r.poly)
                                .collect::<Vec<_>>(),
                            old.certificate
                                .basis
                                .iter()
                                .map(|r| &r.poly)
                                .collect::<Vec<_>>()
                        );
                        assert!(verify_dag_product(&new.certificate, &inputs, n, p));
                        assert!(!verify_product(&new.certificate, &inputs, n, p));
                        let mut changed = inputs.clone();
                        let coefficient = changed[0].values_mut().next().unwrap();
                        *coefficient = (*coefficient + 1) % p;
                        assert!(!verify_dag_product(&new.certificate, &changed, n, p));
                    }
                }
            }
        }
    }
    #[test]
    fn sufficient_subset_keeps_full_checked_star_ideals() {
        for hub in [0, 7] {
            let edges: Vec<_> = (0..8).filter(|v| *v != hub).map(|v| (hub, v)).collect();
            let system = workload::generate_graph(2147483647, 8, 7, &edges)
                .unwrap()
                .system;
            let input = system.decode().unwrap();
            let old = solve(&input, 8, system.prime).unwrap();
            let new = solve_sparse(&input, 8, system.prime).unwrap();
            assert!(new.subset_succeeded);
            assert_eq!(
                new.output
                    .certificate
                    .basis
                    .iter()
                    .map(|r| &r.poly)
                    .collect::<Vec<_>>(),
                old.certificate
                    .basis
                    .iter()
                    .map(|r| &r.poly)
                    .collect::<Vec<_>>()
            );
            assert_eq!(old.info.projection_work.edge_resultants, 7);
            assert_eq!(new.output.info.projection_work.edge_resultants, 2);
            assert!(verify(&new.output.certificate, &input, 8, system.prime));
            let metadata = cheap::pair_metadata(&input, 8, system.prime);
            let cubic = cheap::consequences(&input, 8, system.prime);
            assert_eq!(
                metadata,
                cubic
                    .iter()
                    .map(|pair| cheap::PairMetadata {
                        inputs: pair.inputs,
                        variables: pair.variables
                    })
                    .collect::<Vec<_>>()
            );
        }
    }
    #[test]
    fn research_capacity_is_explicit_and_bounded() {
        for n in [3, 8, 9, 16, 17] {
            let input = vec![BTreeMap::from([(vec![0; n], 1)])];
            assert_eq!(admitted(&input, n, 2147483647), n <= MAX_VARIABLES);
        }
        assert_eq!(
            MAX_VARIABLES,
            if cfg!(feature = "scale-screen") {
                16
            } else {
                8
            }
        );
    }
    #[test]
    fn compact_chain_keeps_existing_fixed_case_bases_and_smaller_six_cycle_proofs() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 4, 6] {
                let inputs = workload::generate(p, n, 7)
                    .unwrap()
                    .system
                    .decode()
                    .unwrap();
                let compact = solve(&inputs, n, p).unwrap();
                let all = solve_all_vertices(&inputs, n, p).unwrap();
                assert_eq!(compact.certificate.definitions, all.certificate.definitions);
                assert_eq!(compact.certificate.basis, all.certificate.basis);
                assert_eq!(compact.info.root_variable, n - 1);
                assert_eq!(compact.info.projection_work.edge_resultants, 2);
                assert_eq!(compact.info.projection_work.vertex_gcd_folds, 1);
                assert_eq!(all.info.projection_work.edge_resultants, n);
                assert_eq!(all.info.projection_work.vertex_gcd_folds, n);
                assert!(verify(&all.certificate, &inputs, n, p));
                assert!(verify(&compact.certificate, &inputs, n, p));
                let (expanded, _) = super::super::solve(&inputs, n, p).unwrap();
                assert_eq!(
                    compact
                        .certificate
                        .basis
                        .iter()
                        .map(|r| &r.poly)
                        .collect::<Vec<_>>(),
                    expanded.iter().map(|r| &r.poly).collect::<Vec<_>>()
                );
                if n == 6 {
                    let expanded_terms: usize = expanded.iter().map(|r| r.proof.len()).sum();
                    assert!(compact.info.stored_proof_terms < expanded_terms);
                    eprintln!("compact fixed case p={p}: total_local_terms={}, expanded_final_basis_terms={expanded_terms}, local_product_degree={}, expanded_original_product_degree_upper_bound={}", compact.info.stored_proof_terms,compact.info.max_local_certificate_product_degree,compact.info.expanded_original_product_degree_upper_bound);
                }
            }
        }
    }
    #[test]
    fn rejects_circular_forged_missing_and_malformed_chain_data() {
        let p = 2147483647;
        let inputs = workload::generate(p, 3, 7)
            .unwrap()
            .system
            .decode()
            .unwrap();
        let output = solve(&inputs, 3, p).unwrap();
        let mut bad = output.certificate.clone();
        bad.definitions[0].proof = BTreeMap::from([((inputs.len(), vec![0; 3]), 1)]);
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        bad.definitions[0].proof.clear();
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        bad.definitions.remove(0);
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        bad.basis[0].proof = BTreeMap::from([((0, vec![usize::MAX, 1, 0]), 1)]);
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        *bad.definitions[0].poly.values_mut().next().unwrap() = p;
        assert!(!verify(&bad, &inputs, 3, p));
        assert!(!verify(&output.certificate, &inputs, 3, 1u64 << 53));
    }
    #[test]
    fn wrong_final_ideal_and_shape_are_not_accepted() {
        let p = 2147483647;
        let inputs = workload::generate(p, 3, 7)
            .unwrap()
            .system
            .decode()
            .unwrap();
        let output = solve(&inputs, 3, p).unwrap();
        let mut bad = output.certificate;
        bad.basis.pop();
        assert!(!verify(&bad, &inputs, 3, p));
        assert!(solve(&inputs, 2, p).is_err());
        assert!(solve(&inputs, 3, 1u64 << 53).is_err());
    }
}
