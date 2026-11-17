//! Public Rust API integration tests. Development correctness, not timing evidence.
use certified_relations as algebra;
use crypto_lib::cryptanalysis::f4_fp::{self, certified::*, normalise, F4Options, Ordering, Poly};
#[test]
fn pending_native_wire_needs_complete_root_replay_and_rejects_changed_proof() {
    let system = algebra::workload::generate(2147483647, 3, 7)
        .unwrap()
        .system;
    let input = serde_json::to_vec(&system).unwrap();
    for kind in ["compact", "expanded", "compact-sparse"] {
        let full = algebra::replay::produce_native_product(&system, kind).unwrap();
        let single = algebra::replay::produce_native_single_export(&system, kind).unwrap();
        let (pending, _) = algebra::replay::produce_native_for_rust(&system, kind)
            .unwrap()
            .into_parts();
        let expected = serde_json::to_vec(&full.witness).unwrap();
        assert_eq!(pending, expected);
        assert_eq!(serde_json::to_vec(&single.witness).unwrap(), expected);
        let accepted = verify_replay_certificate_json(&input, &pending).unwrap();
        let reference = verify_replay_certificate_json(&input, &expected).unwrap();
        assert_eq!(accepted.basis, reference.basis);
        let mut changed: serde_json::Value = serde_json::from_slice(&pending).unwrap();
        changed["basis"][0]["proof"] = serde_json::json!([]);
        assert!(
            verify_replay_certificate_json(&input, &serde_json::to_vec(&changed).unwrap()).is_err()
        );
        let mut changed: serde_json::Value = serde_json::from_slice(&pending).unwrap();
        changed["pair_reductions"].as_array_mut().unwrap().pop();
        assert!(
            verify_replay_certificate_json(&input, &serde_json::to_vec(&changed).unwrap()).is_err()
        );
    }
}
#[test]
fn exact_kernel_choices_keep_full_root_proof_acceptance() {
    let system = algebra::workload::generate(2147483647, 3, 7)
        .unwrap()
        .system;
    let input = serde_json::to_vec(&system).unwrap();
    let (old, _) = algebra::replay::produce_native_for_rust(&system, "compact-sparse")
        .unwrap()
        .into_parts();
    let reference = verify_replay_certificate_json(&input, &old).unwrap();
    for kernels in [
        algebra::replay::NativeKernels::Legacy,
        algebra::replay::NativeKernels::Ordered,
        algebra::replay::NativeKernels::Cached,
        algebra::replay::NativeKernels::Both,
        algebra::replay::NativeKernels::BothUpdates,
        algebra::replay::NativeKernels::BothUpdatesSum,
        algebra::replay::NativeKernels::BothUpdatesLead,
        algebra::replay::NativeKernels::BothUpdatesFused,
        algebra::replay::NativeKernels::BothUpdatesFrontier,
    ] {
        let (wire, _) = algebra::replay::produce_native_for_rust_with_kernels(
            &system,
            "compact-sparse",
            kernels,
        )
        .unwrap()
        .into_parts();
        assert_eq!(wire, old);
        assert_eq!(
            verify_replay_certificate_json(&input, &wire).unwrap().basis,
            reference.basis
        );
        let mut bad: serde_json::Value = serde_json::from_slice(&wire).unwrap();
        bad["basis"][0]["proof"] = serde_json::json!([]);
        assert!(
            verify_replay_certificate_json(&input, &serde_json::to_vec(&bad).unwrap()).is_err()
        );
    }
}
#[test]
fn separate_replay_public_api_checks_complete_basis_and_changed_identity() {
    let system = algebra::workload::generate(2147483647, 3, 7)
        .unwrap()
        .system;
    let produced = algebra::replay::produce_native_product(&system, "compact-sparse").unwrap();
    let input = serde_json::to_vec(&system).unwrap();
    let checked =
        verify_replay_certificate_json(&input, &serde_json::to_vec(&produced.witness).unwrap())
            .unwrap();
    let actual: std::collections::BTreeSet<algebra::Poly> = checked
        .basis
        .into_iter()
        .map(|r| r.into_iter().collect())
        .collect();
    let expected: std::collections::BTreeSet<algebra::Poly> = produced
        .witness
        .basis
        .iter()
        .map(|r| {
            r.polynomial
                .iter()
                .map(|t| (t.exponents.clone(), t.coefficient))
                .collect()
        })
        .collect();
    assert_eq!(actual, expected);
    assert_eq!(checked.stats.pairs, produced.witness.pair_reductions.len());
    let mut bad = produced.witness;
    bad.definitions[0].proof.clear();
    assert!(verify_replay_certificate_json(&input, &serde_json::to_vec(&bad).unwrap()).is_err());
}
fn fixture(n: usize, p: u64) -> Vec<Poly> {
    let system = algebra::workload::generate(p, n, 7).unwrap().system;
    system
        .decode()
        .unwrap()
        .into_iter()
        .map(|f| {
            f.into_iter()
                .map(|(m, c)| (m.into_iter().map(|e| e as u32).collect(), c))
                .collect()
        })
        .collect()
}
fn key(b: &[Poly], p: u64) -> std::collections::BTreeSet<Poly> {
    b.iter()
        .map(|f| normalise(f, p, Ordering::Grevlex))
        .collect()
}
#[test]
fn agrees_with_complete_native_f4_on_development_cycles() {
    let p = 2147483647;
    for n in [3, 4] {
        let input = fixture(n, p);
        let got = try_quotient_basis(&input, n, p).unwrap();
        assert!(verify_basis(&got, &input));
        assert_eq!(got.observed_solving_degree, None);
        assert_eq!(got.quotient.quotient_dimension, 4);
        assert!(got.native_checks.certification_pairs > 0);
        let mut opts = F4Options::new(Ordering::Grevlex, 12);
        opts.deadline = Some(std::time::Instant::now() + std::time::Duration::from_secs(10));
        let control = f4_fp::f4(&input, n, p, &opts);
        assert!(
            !control.timed_out
                && control.pairs_above_bound == 0
                && control.staircase_at_stop.is_none()
        );
        assert_eq!(key(&got.basis, p), key(&control.basis, p));
    }
}
#[test]
fn wide_field_duplicates_scaling_and_tampered_results() {
    let p = 9007199254740881;
    let mut input = fixture(3, p);
    let (m, c) = input[0].remove(0);
    input[0].push((m.clone(), p - 1));
    input[0].push((m, (c + 1) % p));
    let (m, c) = input[2].remove(0);
    let residue = u64::MAX % p;
    input[2].push((m.clone(), (c + p - residue) % p));
    input[2].push((m, u64::MAX));
    for (_, c) in &mut input[1] {
        *c = ((*c as u128 * 2) % p as u128) as u64;
    }
    let mut got = try_quotient_basis(&input, 3, p).unwrap();
    assert!(verify_basis(&got, &input));
    let old = got.basis[0][0].1;
    got.basis[0][0].1 = (old + 1) % p;
    assert!(!verify_basis(&got, &input));
    got.basis[0][0].1 = old;
    let original_certificate = got.certificates[0].clone();
    got.certificates[0].poly = std::collections::BTreeMap::from([(vec![1], 1)]);
    assert!(!verify_basis(&got, &input));
    got.certificates[0] = original_certificate.clone();
    got.certificates[0].proof =
        std::collections::BTreeMap::from([((0, vec![usize::MAX, 0, 0]), 1)]);
    assert!(!verify_basis(&got, &input));
    got.certificates[0] = original_certificate;
    got.certificates[0].proof.clear();
    assert!(!verify_basis(&got, &input));
}
#[test]
fn invalid_and_unrecognized_inputs_are_explicit_errors() {
    let input = fixture(3, 2147483647);
    assert!(try_quotient_basis(&input, 2, 2147483647).is_err());
    assert!(try_quotient_basis(&input, 3, 1u64 << 53).is_err());
    assert!(try_quotient_basis(&input, 3, u64::MAX).is_err());
    assert!(try_quotient_basis(&[vec![(vec![u32::MAX, 0, 0], 1)]], 3, 101).is_err());
    assert!(try_quotient_basis(&[vec![(vec![1, 0, 0], 1)]], 3, 101).is_err());
}
#[test]
fn native_polynomial_trace_preserves_incomplete_status() {
    let input = fixture(2, 2147483647);
    let trace = trace_relations(&input, 2, 2147483647, 0, 0).unwrap();
    assert_eq!(trace["baseline"]["status"], "degree_cap");
    assert!(trace["baseline"]["solving_degree"].is_null());
}
#[test]
fn six_variable_breadth_first_basis_passes_on_both_fields() {
    for p in [2147483647, 9007199254740881] {
        let input = fixture(6, p);
        let report = try_quotient_basis(&input, 6, p).unwrap();
        assert!(verify_basis(&report, &input));
        assert_eq!(report.quotient.quotient_dimension, 4);
        assert_eq!(
            report.quotient.derivation_depths.iter().copied().max(),
            Some(3)
        );
        eprintln!("six-variable development outcome: complete checked breadth-first basis, p={p}, depths={:?}, basis_degree={}, stored_certificate_product_degree={}, max_stored_basis_proof_terms={}",report.quotient.derivation_depths,report.quotient.basis_polynomial_degree,report.quotient.basis_certificate_product_degree,report.certificates.iter().map(|r|r.proof.len()).max().unwrap_or(0));
    }
}

#[test]
fn compact_chain_crosses_native_boundary_and_rejects_mutation() {
    for p in [2147483647, 9007199254740881] {
        let input = fixture(6, p);
        let mut report = try_compact_quotient_basis(&input, 6, p).unwrap();
        let all =
            crypto_lib::cryptanalysis::f4_fp::certified::try_compact_quotient_basis_all_vertices(
                &input, 6, p,
            )
            .unwrap();
        assert!(verify_compact_basis(&all, &input));
        assert_eq!(report.basis, all.basis);
        assert_eq!(report.certificate.definitions, all.certificate.definitions);
        assert_eq!(report.certificate.basis, all.certificate.basis);
        assert_eq!(report.info.projection_work.edge_resultants, 2);
        assert_eq!(report.info.projection_work.vertex_gcd_folds, 1);
        assert_eq!(all.info.projection_work.edge_resultants, 6);
        assert_eq!(all.info.projection_work.vertex_gcd_folds, 6);
        assert!(verify_compact_basis(&report, &input));
        assert_eq!(report.observed_solving_degree, None);
        assert!(report.native_checks.certification_pairs > 0);
        eprintln!("compact native fixed case p={p}: stored_proof_terms={}, definition_nodes={}, local_product_degree={}, expanded_original_product_degree_upper_bound={}",report.info.stored_proof_terms,report.info.definition_nodes,report.info.max_local_certificate_product_degree,report.info.expanded_original_product_degree_upper_bound);
        let certificate = report.certificate.clone();
        report.certificate.definitions[0].proof =
            std::collections::BTreeMap::from([((input.len(), vec![0; 6]), 1)]);
        assert!(!verify_compact_basis(&report, &input));
        report.certificate = certificate;
        report.basis[0][0].1 = (report.basis[0][0].1 + 1) % p;
        assert!(!verify_compact_basis(&report, &input));
    }
}
