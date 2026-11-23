use vitse_prime31_identity::identity_certificate::{
    check_with_extra, statement_id, IdentityCertificate, Verdict, MODULUS,
};
use vitse_prime31_identity::witness::{
    statement, verdict_json, wrong_sign_statement, CHECK_SEED, INDEPENDENCE, ISSUE_SEED, POINTS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: check_identity CERTIFICATE_JSON".into());
    }
    let bytes = std::fs::read(&args[1])?;
    let cert: IdentityCertificate = serde_json::from_slice(&bytes)?;
    assert_eq!(cert.statement, statement());
    assert_eq!((cert.modulus, cert.seed, cert.points), (MODULUS, ISSUE_SEED, POINTS));
    assert_eq!(cert.independence, INDEPENDENCE);
    assert_eq!(cert.proof, None);
    let accepted = check_with_extra(&cert, Some((CHECK_SEED, POINTS)))?;
    assert!(matches!(accepted, Verdict::Accept { points: POINTS, extra_points: POINTS, .. }));

    let mut bad_sign = cert.clone();
    bad_sign.statement = wrong_sign_statement();
    bad_sign.id = statement_id(&bad_sign.statement);
    let rejected_sign = check_with_extra(&bad_sign, Some((CHECK_SEED, POINTS)))?;
    assert!(matches!(rejected_sign, Verdict::Reject(_)));

    let mut bad_digest = cert.clone();
    let replacement = if bad_digest.value_digest.starts_with('0') { "1" } else { "0" };
    bad_digest.value_digest.replace_range(..1, replacement);
    let rejected_digest = check_with_extra(&bad_digest, Some((CHECK_SEED, POINTS)))?;
    assert!(matches!(rejected_digest, Verdict::DigestMismatch { .. }));
    println!("{}", serde_json::json!({
        "stage": "checker", "status": "pass", "statement_id": cert.id,
        "certificate_path": args[1], "certificate_sha256": hex::encode(vitse_prime31_identity::hash::sha256::sha256(&bytes)),
        "checker_seed": CHECK_SEED, "checker_extra_points": POINTS,
        "original": verdict_json(&accepted),
        "wrong_rhs_sign_with_recomputed_id": verdict_json(&rejected_sign),
        "digest_mutation": verdict_json(&rejected_digest),
        "proof": cert.proof, "independence": cert.independence
    }));
    Ok(())
}
