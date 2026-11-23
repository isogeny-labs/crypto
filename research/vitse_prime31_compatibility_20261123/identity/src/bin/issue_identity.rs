use std::fs::OpenOptions;
use std::io::Write;
use vitse_prime31_identity::identity_certificate::{
    issue, syntactic_degree, validate_statement, IssueError, Malformed,
};
use vitse_prime31_identity::witness::{
    counterexample_json, statement, wrong_sign_statement, INDEPENDENCE, ISSUE_SEED, POINTS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: issue_identity OUTPUT_CERTIFICATE_JSON".into());
    }
    let statement = statement();
    let lhs_degree = syntactic_degree(&statement.lhs);
    let rhs_degree = syntactic_degree(&statement.rhs);
    assert_eq!((lhs_degree, rhs_degree), (4, 4));
    validate_statement(&statement)?;
    let mut too_small = statement.clone();
    too_small.degree_bound = 3;
    let degree_guard = validate_statement(&too_small);
    assert!(matches!(degree_guard, Err(Malformed::DegreeBound { declared: 3, syntactic: 4 })));

    let false_counterexample = match issue(wrong_sign_statement(), ISSUE_SEED, POINTS, INDEPENDENCE, None) {
        Err(IssueError::False(c)) => c,
        other => return Err(format!("wrong-sign issuer control did not reject: {other:?}").into()),
    };
    let cert = issue(statement, ISSUE_SEED, POINTS, INDEPENDENCE, None)?;
    let mut bytes = serde_json::to_vec_pretty(&cert)?;
    bytes.push(b'\n');
    let mut output = OpenOptions::new().write(true).create_new(true).open(&args[1])?;
    output.write_all(&bytes)?;
    output.sync_all()?;
    println!("{}", serde_json::json!({
        "stage": "issuer", "status": "pass", "certificate_schema": cert.schema,
        "statement_id": cert.id, "modulus": cert.modulus, "degree_bound": cert.statement.degree_bound,
        "lhs_syntactic_degree": lhs_degree, "rhs_syntactic_degree": rhs_degree,
        "seed": cert.seed, "points": cert.points, "value_digest": cert.value_digest,
        "certificate_path": args[1], "certificate_sha256": hex::encode(vitse_prime31_identity::hash::sha256::sha256(&bytes)),
        "wrong_rhs_sign": {"status": "rejected", "counterexample": counterexample_json(&false_counterexample)},
        "underdeclared_degree": {"status": "rejected", "declared": 3, "syntactic": 4},
        "proof": cert.proof, "independence": cert.independence
    }));
    Ok(())
}
