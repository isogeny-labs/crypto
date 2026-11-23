use pplus_line_identity::identity_certificate::{
    check_with_extra, statement_id, IdentityCertificate, Verdict, MODULUS,
};
use pplus_line_identity::witness::{
    false_statement, statement, verdict_json, CHECK_SEEDS, INDEPENDENCE, ISSUE_SEED, KEYS, POINTS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 { return Err("usage: check_identity CERTIFICATE_DIRECTORY".into()); }
    for key in KEYS {
        let path = std::path::Path::new(&args[1]).join(format!("{key}.certificate.json"));
        let bytes = std::fs::read(&path)?;
        let cert: IdentityCertificate = serde_json::from_slice(&bytes)?;
        assert_eq!(cert.statement, statement(key));
        assert_eq!((cert.modulus, cert.seed, cert.points), (MODULUS, ISSUE_SEED, POINTS));
        assert_eq!(cert.independence, INDEPENDENCE);
        assert_eq!(cert.proof, None);
        let mut accepts = Vec::new();
        for seed in CHECK_SEEDS {
            let v = check_with_extra(&cert, Some((seed, POINTS)))?;
            assert!(matches!(v, Verdict::Accept { points: POINTS, extra_points: POINTS, .. }));
            accepts.push(serde_json::json!({"seed": seed, "verdict": verdict_json(&v, &cert.statement.variables)}));
        }
        let mut false_cert = cert.clone();
        false_cert.statement = false_statement(key);
        false_cert.id = statement_id(&false_cert.statement);
        let false_verdict = check_with_extra(&false_cert, Some((CHECK_SEEDS[0], POINTS)))?;
        assert!(matches!(false_verdict, Verdict::Reject(_)));
        let mut changed_digest = cert.clone();
        let replacement = if changed_digest.value_digest.starts_with('0') { "1" } else { "0" };
        changed_digest.value_digest.replace_range(..1, replacement);
        let digest_verdict = check_with_extra(&changed_digest, Some((CHECK_SEEDS[0], POINTS)))?;
        assert!(matches!(digest_verdict, Verdict::DigestMismatch { .. }));
        println!("{}", serde_json::json!({
            "stage": "checker", "status": "pass", "key": key,
            "statement_id": cert.id, "certificate_path": path,
            "certificate_sha256": hex::encode(pplus_line_identity::hash::sha256::sha256(&bytes)),
            "extra_seed_checks": accepts,
            "rhs_plus_one_with_recomputed_id": verdict_json(&false_verdict, &cert.statement.variables),
            "digest_mutation": verdict_json(&digest_verdict, &cert.statement.variables),
            "proof": cert.proof, "independence": cert.independence
        }));
    }
    Ok(())
}
