//! Run with `rustc --edition=2021 --test compatibility_tests.rs -o compatibility-tests`.
//! No timing or ECDLP-performance claim is made by these contract tests.

#[path = "compatibility.rs"]
mod compatibility;
use compatibility::*;

// Exact coordinates of the eight PUBLIC INPUT POINTS in committed evidence:
// crypto@7d61e3b45de6e93d235345a0061dae08db1f8b44
// research/prime31_four_summand_20261122/evidence/b8-positive-01/stdout.txt.
// The original record's extractor failure is preserved; these tests do not
// upgrade it. These eight coordinates are a supplied finite-domain control,
// NOT a claim to enumerate the full Prime31 factor base, a complete production
// selector fiber, or all curve points. Tests passing them as both layers are
// explicitly testing the API with equal supplied lists.
const PUBLIC_B8_X: [u64; 8] = [0, 1, 2, 3, 5, 6, 8, 9];

#[test]
fn fixed_prime31_fixture_is_not_an_alternate_curve() {
    assert_eq!(PRIME31_P, (1_u64 << 31) - 1);
    assert_eq!((PRIME31_A, PRIME31_B), (847_514_846, 2_087_444_724));
    assert_eq!(PRIME31_ORDER, 2_147_446_831);
    assert_eq!(PRIME31_GENERATOR, (625_037_715, 2_100_627_251));
}

#[test]
fn public_b8_identity_preserves_equal_supplied_lists() {
    let audit = audit_selector_support(&PUBLIC_B8_X, &PUBLIC_B8_X, Mobius::identity()).unwrap();
    assert!(audit.both_nonvacuously_closed);
    assert_eq!(audit.selector.cardinality, 8);
    assert!(audit.removed_by_filters.is_empty());
    assert_eq!(audit.scope, EvidenceScope::FiniteCoordinateSetsOnly);
}

#[test]
fn actual_public_b8_coordinate_negation_is_not_closed() {
    let audit = audit_domain(&PUBLIC_B8_X, Mobius::coordinate_negation()).unwrap();
    assert!(!audit.set_closed);
    assert_eq!(audit.escaped.len(), 7);
    assert_eq!(audit.escaped[0], (1, PRIME31_P - 1));
    assert!(audit.poles.is_empty());
    assert_eq!(audit.images[&0], Some(0));
}

#[test]
fn actual_public_b8_reciprocal_reports_pole_and_escape() {
    let audit = audit_domain(&PUBLIC_B8_X, Mobius::reciprocal()).unwrap();
    assert!(!audit.nonvacuous_bijection);
    assert_eq!(audit.poles, vec![0]);
    assert_eq!(audit.images[&1], Some(1));
    // 2 * 1_073_741_824 = p + 1, independently fixing this inverse.
    assert!(audit.escaped.contains(&(2, 1_073_741_824)));
    assert_eq!(audit.images[&0], None);
}

#[test]
fn actual_public_b8_translation_keeps_failure_witness() {
    let audit = audit_domain(&PUBLIC_B8_X, Mobius::affine(1, 1).unwrap()).unwrap();
    assert!(!audit.set_closed);
    assert!(audit.escaped.contains(&(3, 4)));
    assert!(audit.escaped.contains(&(6, 7)));
    assert!(audit.escaped.contains(&(9, 10)));
}

#[test]
fn closed_selector_does_not_imply_closed_retained_support() {
    // Synthetic FIELD-SET control in the same fixed field; it is not a
    // replacement ECDLP curve, point base, or geometric action.
    let selector = [1, PRIME31_P - 1];
    let audit = audit_selector_support(&selector, &[1], Mobius::coordinate_negation()).unwrap();
    assert!(audit.selector.nonvacuous_bijection);
    assert!(!audit.retained.nonvacuous_bijection);
    assert_eq!(audit.retained.escaped, vec![(1, PRIME31_P - 1)]);
    assert_eq!(audit.removed_by_filters, vec![PRIME31_P - 1]);
    assert!(!audit.both_nonvacuously_closed);
}

#[test]
fn reciprocal_closed_control_is_finite_set_evidence_only() {
    let audit = audit_domain(&[1, PRIME31_P - 1], Mobius::reciprocal()).unwrap();
    assert!(audit.nonvacuous_bijection);
    assert_eq!(audit.scope, EvidenceScope::FiniteCoordinateSetsOnly);
    assert_eq!(audit.images[&(PRIME31_P - 1)], Some(PRIME31_P - 1));
}

#[test]
fn centered_reciprocal_is_exact_and_keeps_its_pole() {
    // x -> 5 + 4/(x-5) swaps 6 and 9 and has a pole at 5.
    let map = Mobius::centered_reciprocal(5, 4).unwrap();
    assert_eq!(map.apply(6).unwrap(), Some(9));
    assert_eq!(map.apply(9).unwrap(), Some(6));
    assert_eq!(map.apply(5).unwrap(), None);
    assert!(audit_domain(&[6, 9], map).unwrap().nonvacuous_bijection);
    assert!(!audit_domain(&[5, 6, 9], map).unwrap().set_closed);
}

#[test]
fn projectively_scaled_map_has_same_action() {
    let scaled_identity = Mobius::new(2, 0, 0, 2).unwrap();
    assert_eq!(scaled_identity.coefficients(), [2, 0, 0, 2]);
    let left = audit_domain(&PUBLIC_B8_X, Mobius::identity()).unwrap();
    let right = audit_domain(&PUBLIC_B8_X, scaled_identity).unwrap();
    assert_eq!(left, right);
}

#[test]
fn invalid_maps_cannot_create_a_false_closed_action() {
    assert_eq!(Mobius::new(1, 2, 2, 4), Err(AuditError::SingularMobius));
    assert_eq!(Mobius::affine(0, 1), Err(AuditError::SingularMobius));
    assert_eq!(Mobius::centered_reciprocal(5, 0), Err(AuditError::SingularMobius));
    assert!(matches!(Mobius::new(PRIME31_P, 0, 0, 1),
        Err(AuditError::NonCanonicalCoefficient { .. })));
}

#[test]
fn malformed_or_mismatched_domains_are_rejected() {
    assert!(matches!(audit_domain(&[1, 1], Mobius::identity()),
        Err(AuditError::DuplicateCoordinate { .. })));
    assert!(matches!(audit_domain(&[PRIME31_P], Mobius::identity()),
        Err(AuditError::NonCanonicalCoordinate { .. })));
    assert_eq!(audit_selector_support(&[1, 2], &[1, 3], Mobius::identity()),
        Err(AuditError::RetainedOutsideSelector { value: 3 }));
    assert!(matches!(audit_selector_support(&[1, 2], &[1, 1], Mobius::identity()),
        Err(AuditError::DuplicateCoordinate { .. })));
    assert!(matches!(Mobius::reciprocal().apply(PRIME31_P),
        Err(AuditError::NonCanonicalCoordinate { .. })));
}

#[test]
fn empty_support_is_never_nonvacuous_evidence() {
    let empty = audit_domain(&[], Mobius::reciprocal()).unwrap();
    assert!(empty.set_closed);
    assert!(!empty.nonvacuous_bijection);
    let audit = audit_selector_support(&PUBLIC_B8_X, &[], Mobius::identity()).unwrap();
    assert!(!audit.both_nonvacuously_closed);
}

#[test]
fn identical_actual_leaf_domains_are_exchangeable() {
    let leaves: [&[u64]; 4] = [&PUBLIC_B8_X, &PUBLIC_B8_X, &PUBLIC_B8_X, &PUBLIC_B8_X];
    let audit = audit_leaf_permutation(&leaves, &[3, 1, 0, 2]).unwrap();
    assert!(audit.leafwise_equal && audit.cartesian_domain_equal);
    assert!(audit.nonvacuous_domain_compatible);
    assert_eq!(audit.scope, EvidenceScope::FiniteCoordinateSetsOnly);
}

#[test]
fn smaller_oriented_fourth_arm_blocks_full_leaf_symmetry() {
    let fourth = &PUBLIC_B8_X[..4];
    let leaves: [&[u64]; 4] = [&PUBLIC_B8_X, &PUBLIC_B8_X, &PUBLIC_B8_X, fourth];
    let first_three = audit_leaf_permutation(&leaves, &[2, 0, 1, 3]).unwrap();
    assert!(first_three.nonvacuous_domain_compatible);
    let crossed = audit_leaf_permutation(&leaves, &[3, 1, 2, 0]).unwrap();
    assert!(!crossed.cartesian_domain_equal);
    assert_eq!(crossed.mismatches.len(), 2);
    assert_eq!(crossed.mismatches[0].destination_only, vec![5, 6, 8, 9]);
    assert_eq!(crossed.mismatches[1].source_only, vec![5, 6, 8, 9]);
}

#[test]
fn permutation_uses_set_identity_not_cardinality_or_order() {
    let a = [0, 1, 2];
    let reordered = [2, 0, 1];
    let different = [0, 1, 3];
    assert!(audit_leaf_permutation(&[&a, &reordered], &[1, 0]).unwrap()
        .nonvacuous_domain_compatible);
    assert!(!audit_leaf_permutation(&[&a, &different], &[1, 0]).unwrap()
        .cartesian_domain_equal);
}

#[test]
fn invalid_leaf_permutations_are_rejected() {
    let leaves: [&[u64]; 2] = [&PUBLIC_B8_X, &PUBLIC_B8_X];
    assert_eq!(audit_leaf_permutation(&leaves, &[0, 0]),
        Err(AuditError::RepeatedPermutationSource { source: 0 }));
    assert!(matches!(audit_leaf_permutation(&leaves, &[0]),
        Err(AuditError::PermutationLength { .. })));
    assert!(matches!(audit_leaf_permutation(&leaves, &[0, 2]),
        Err(AuditError::PermutationIndex { .. })));
    assert_eq!(audit_leaf_permutation(&[], &[]), Err(AuditError::NoLeaves));
    assert!(matches!(audit_leaf_permutation(&[&[1, 1]], &[0]),
        Err(AuditError::DuplicateCoordinate { .. })));
}

#[test]
fn empty_cartesian_domain_is_explicitly_vacuous() {
    let empty: [u64; 0] = [];
    let a = [1, 2];
    let audit = audit_leaf_permutation(&[&empty, &a], &[1, 0]).unwrap();
    assert!(!audit.leafwise_equal);
    assert!(audit.cartesian_domain_equal);
    assert!(!audit.nonvacuous_domain_compatible);
    assert_eq!(audit.empty_leaves, vec![0]);
}
