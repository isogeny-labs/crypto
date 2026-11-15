//! Public Rust API integration tests. Development correctness, not timing evidence.
use certified_relations as algebra;
use crypto_lib::cryptanalysis::f4_fp::{self, certified::*, normalise, F4Options, Ordering, Poly};
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
