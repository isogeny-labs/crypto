use std::fs::OpenOptions;
use std::io::Write;
use pplus_line_identity::identity_certificate::{
    issue, syntactic_degree, validate_statement, IssueError, Malformed,
};
use pplus_line_identity::witness::{
    counterexample_json, false_statement, statement, INDEPENDENCE, ISSUE_SEED, KEYS, POINTS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 { return Err("usage: issue_identity NEW_CERTIFICATE_DIRECTORY".into()); }
    std::fs::create_dir(&args[1])?;
    for key in KEYS {
        let s = statement(key);
        let degrees = (syntactic_degree(&s.lhs), syntactic_degree(&s.rhs));
        assert_eq!(degrees.0.max(degrees.1), s.degree_bound);
        validate_statement(&s)?;
        let mut underdeclared = s.clone();
        underdeclared.degree_bound -= 1;
        match validate_statement(&underdeclared) {
            Err(Malformed::DegreeBound { declared, syntactic }) => {
                assert_eq!(declared + 1, s.degree_bound);
                assert_eq!(syntactic, s.degree_bound);
            },
            other => return Err(format!("degree guard did not reject {key}: {other:?}").into()),
        }
        let false_counterexample = match issue(false_statement(key), ISSUE_SEED, POINTS, INDEPENDENCE, None) {
            Err(IssueError::False(c)) => c,
            other => return Err(format!("false statement did not reject {key}: {other:?}").into()),
        };
        let cert = issue(s, ISSUE_SEED, POINTS, INDEPENDENCE, None)?;
        let path = std::path::Path::new(&args[1]).join(format!("{key}.certificate.json"));
        let mut bytes = serde_json::to_vec_pretty(&cert)?;
        bytes.push(b'\n');
        let mut file = OpenOptions::new().write(true).create_new(true).open(&path)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        println!("{}", serde_json::json!({
            "stage": "issuer", "status": "pass", "key": key,
            "schema": cert.schema, "statement_id": cert.id, "modulus": cert.modulus,
            "degree_bound": cert.statement.degree_bound,
            "lhs_syntactic_degree": degrees.0, "rhs_syntactic_degree": degrees.1,
            "seed": cert.seed, "points": cert.points, "value_digest": cert.value_digest,
            "certificate_path": path,
            "certificate_sha256": hex::encode(pplus_line_identity::hash::sha256::sha256(&bytes)),
            "rhs_plus_one": {"status": "rejected", "counterexample": counterexample_json(&false_counterexample, &cert.statement.variables)},
            "underdeclared_degree": {"status": "rejected", "declared": cert.statement.degree_bound - 1},
            "proof": cert.proof, "independence": cert.independence
        }));
    }
    Ok(())
}
