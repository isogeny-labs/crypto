//! Fixed identity artifact contracts. No experiment, census or timing result.
use prime31_aux_matched::identity_certificate::{self, Verdict};
use prime31_aux_matched::sparse_identity;

#[test]
fn sparse_quartic_uses_the_existing_certificate_contract_and_refusal_paths() {
    let checked = sparse_identity::issue_checked().unwrap();
    let certificate = &checked.certificate;
    assert_eq!(certificate.schema, identity_certificate::SCHEMA);
    assert_eq!(certificate.modulus, (1u64 << 61) - 1);
    assert_eq!(certificate.statement, sparse_identity::statement());
    assert_eq!(certificate.statement.degree_bound, 4);
    assert_eq!(certificate.points, 64);
    assert_eq!(certificate.seed, 4188);
    assert!(certificate.proof.is_none());
    assert_eq!(
        certificate.id,
        identity_certificate::statement_id(&certificate.statement)
    );
    assert!(matches!(
        identity_certificate::check_with_extra(certificate, Some((20261123, 32))),
        Ok(Verdict::Accept {
            points: 64,
            extra_points: 32,
            ..
        })
    ));
    assert_eq!(checked.audit["signed_verdict"], false);
    assert!(checked.audit["formal_proof"].is_null());
    assert_eq!(checked.audit["issuer_replay"]["status"], "accept");
    assert_eq!(checked.audit["extra_replay"]["status"], "accept");
    assert_eq!(
        checked.audit["negative_controls"]["constant_one_mutation"]["status"],
        "refused_false_identity"
    );
    assert_eq!(
        checked.audit["negative_controls"]["statement_changed_with_retained_id"],
        "refused_id_mismatch"
    );
    assert_eq!(
        checked.audit["negative_controls"]["value_digest_mutation"],
        "refused_digest_mismatch"
    );
    let bytes = serde_json::to_vec(certificate).unwrap();
    let decoded: identity_certificate::IdentityCertificate =
        serde_json::from_slice(&bytes).unwrap();
    assert_eq!(decoded, *certificate);
    assert!(matches!(
        identity_certificate::check(&decoded),
        Ok(Verdict::Accept { .. })
    ));
}
