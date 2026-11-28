//! Emit and replay evaluated identity records using the unchanged production
//! identity checker. These records have `proof: null`; they are not formal
//! proofs over Z and do not certify a relation extractor or a speedup.
//!
//! ```text
//! dickson-s3-identity-records > identity-records.json
//! dickson-s3-identity-records --check identity-records.json 20261123
//! ```

use std::{env, fs, process};

use crypto_lib::cryptanalysis::identity_certificate::{
    check, check_with_extra, issue, Counterexample, Expr, IdentityCertificate, IdentityStatement,
    IssueError, Malformed, Verdict, MODULUS,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const RECORD_SCHEMA: &str = "dickson-s3-identity-records/v1";
const POINTS: u32 = 32;
const EXTRA_POINTS: u32 = 32;
const SEEDS: [u64; 2] = [0x2026_1122_d1c5_0001, 0x2026_1122_d1c5_0002];
const EVIDENCE_NOTE: &str = "Finite evaluations over F_(2^61-1), not a formal proof over Z. \
    The checker's reported false-accept estimate assumes independent uniform points; \
    its actual streams are deterministic from seeds. No relation-completeness, \
    group-law replay, or performance claim is made by these records.";
const INDEPENDENCE: &str = "Issued by the Dickson/S3 qualifier using production issue(); \
    issuer-side check() replay only. A separate --check invocation supplies its own \
    checker seed and re-runs production check_with_extra(); no independent human \
    review or formal proof is asserted.";

#[derive(Debug, Serialize, Deserialize)]
struct Records {
    schema: String,
    evidence_tier: String,
    evidence_note: String,
    modulus: u64,
    certificates: Vec<IdentityCertificate>,
    issuance_checks: Vec<Value>,
    negative_controls: Vec<Value>,
    all_expected_outcomes: bool,
}

fn var(name: &str) -> Expr {
    Expr::var(name)
}

fn sub(lhs: Expr, rhs: Expr) -> Expr {
    Expr::plus(lhs, Expr::minus(rhs))
}

fn square(value: Expr) -> Expr {
    Expr::times(value.clone(), value)
}

fn s3_symmetric_rhs(e1_coefficient: i64) -> Expr {
    let e1 = Expr::sum(vec![var("x"), var("y"), var("z")]);
    let e2 = Expr::sum(vec![
        Expr::times(var("x"), var("y")),
        Expr::times(var("x"), var("z")),
        Expr::times(var("y"), var("z")),
    ]);
    let e3 = Expr::product(vec![var("x"), var("y"), var("z")]);
    Expr::plus(
        square(sub(e2, var("A"))),
        Expr::product(vec![
            Expr::Const(e1_coefficient),
            e1,
            Expr::plus(e3, var("B")),
        ]),
    )
}

fn s3_statement() -> IdentityStatement {
    let xy = Expr::times(var("x"), var("y"));
    let x_plus_y = Expr::plus(var("x"), var("y"));
    let lhs = Expr::sum(vec![
        Expr::times(square(sub(var("x"), var("y"))), square(var("z"))),
        Expr::product(vec![
            Expr::Const(-2),
            Expr::plus(
                Expr::times(x_plus_y.clone(), Expr::plus(xy.clone(), var("A"))),
                Expr::times(Expr::Const(2), var("B")),
            ),
            var("z"),
        ]),
        square(sub(xy, var("A"))),
        Expr::product(vec![Expr::Const(-4), var("B"), x_plus_y]),
    ]);
    IdentityStatement {
        name: "Standard expanded S3 equals (e2-A)^2 - 4 e1 (e3+B)".into(),
        source: "src/cryptanalysis/symmetrized_semaev.rs::expand_s3_in_xs; \
            research/dickson_s3_quadratic_20261122"
            .into(),
        variables: ["x", "y", "z", "A", "B"].map(String::from).to_vec(),
        // A and B are variables too. No cancellation is used in this bound.
        degree_bound: 4,
        lhs,
        rhs: s3_symmetric_rhs(-4),
    }
}

fn chord_rhs(mixed_coefficient: i64) -> Expr {
    Expr::sum(vec![
        Expr::product(vec![var("T"), var("T"), var("T")]),
        Expr::product(vec![
            Expr::Const(-1),
            square(var("lambda")),
            square(var("T")),
        ]),
        Expr::times(
            Expr::plus(
                var("A"),
                Expr::product(vec![
                    Expr::Const(mixed_coefficient),
                    var("lambda"),
                    var("nu"),
                ]),
            ),
            var("T"),
        ),
        var("B"),
        Expr::minus(square(var("nu"))),
    ])
}

fn chord_statement() -> IdentityStatement {
    IdentityStatement {
        name: "Short-Weierstrass polynomial minus squared line: coefficient expansion".into(),
        source: "research/dickson_s3_quadratic_20261122: \
            f(T)=T^3+A*T+B and line(T)=lambda*T+nu"
            .into(),
        variables: ["T", "A", "B", "lambda", "nu"].map(String::from).to_vec(),
        // lambda^2*T^2 has syntactic degree four, even though f is cubic in T.
        degree_bound: 4,
        lhs: Expr::sum(vec![
            Expr::product(vec![var("T"), var("T"), var("T")]),
            Expr::times(var("A"), var("T")),
            var("B"),
            Expr::minus(square(Expr::plus(
                Expr::times(var("lambda"), var("T")),
                var("nu"),
            ))),
        ]),
        rhs: chord_rhs(-2),
    }
}

fn expected_statements() -> [IdentityStatement; 2] {
    [s3_statement(), chord_statement()]
}

fn counterexample_json(counterexample: &Counterexample) -> Value {
    json!({
        "outcome": "counterexample",
        "index": counterexample.index,
        "assignment_in_statement_variable_order": counterexample.assignment,
        "lhs": counterexample.lhs,
        "rhs": counterexample.rhs,
    })
}

fn check_json(result: Result<Verdict, Malformed>) -> (Value, bool) {
    match result {
        Ok(Verdict::Accept {
            points,
            extra_points,
            false_accept_log2,
        }) => (
            json!({
                "outcome": "accepted_evaluations",
                "points": points,
                "extra_points": extra_points,
                "checker_reported_false_accept_log2": false_accept_log2,
            }),
            true,
        ),
        Ok(Verdict::Reject(counterexample)) => (counterexample_json(&counterexample), false),
        Ok(Verdict::DigestMismatch {
            recorded,
            recomputed,
        }) => (
            json!({
                "outcome": "digest_mismatch", "recorded": recorded, "recomputed": recomputed,
            }),
            false,
        ),
        Err(error) => (
            json!({"outcome": "malformed", "reason": error.to_string()}),
            false,
        ),
    }
}

fn negative_controls(cert: &IdentityCertificate, extra: Option<(u64, u32)>) -> (Vec<Value>, bool) {
    let mut wrong_sign = s3_statement();
    wrong_sign
        .name
        .push_str(" [negative control: +4 instead of -4]");
    wrong_sign.rhs = s3_symmetric_rhs(4);
    let mut wrong_coefficient = chord_statement();
    wrong_coefficient
        .name
        .push_str(" [negative control: -1 instead of -2]");
    wrong_coefficient.rhs = chord_rhs(-1);

    let control_seed = extra.map_or(SEEDS[0] ^ 0xffff, |(seed, _)| seed);
    let mut records = Vec::new();
    let mut all_expected = true;
    for (name, statement) in [
        ("s3_wrong_sign", wrong_sign),
        ("chord_wrong_mixed_coefficient", wrong_coefficient),
    ] {
        let (observed, expected) =
            match issue(statement.clone(), control_seed, POINTS, INDEPENDENCE, None) {
                Err(IssueError::False(counterexample)) => {
                    (counterexample_json(&counterexample), true)
                }
                Err(IssueError::Malformed(error)) => (
                    json!({
                        "outcome": "malformed", "reason": error.to_string(),
                    }),
                    false,
                ),
                Ok(unexpected) => (
                    json!({
                        "outcome": "unexpected_issuance", "certificate_id": unexpected.id,
                    }),
                    false,
                ),
            };
        all_expected &= expected;
        records.push(json!({
            "name": name, "operation": "issue", "statement": statement,
            "seed": control_seed, "points": POINTS,
            "expected_outcome": "counterexample", "observed": observed,
            "expected_outcome_observed": expected,
        }));
    }

    let mut tampered = cert.clone();
    // Keep the 64-hex shape valid so this exercises digest comparison.
    let replacement = if tampered.value_digest.starts_with('0') {
        "1"
    } else {
        "0"
    };
    tampered.value_digest.replace_range(..1, replacement);
    let result = match extra {
        Some(extra) => check_with_extra(&tampered, Some(extra)),
        None => check(&tampered),
    };
    let expected = matches!(&result, Ok(Verdict::DigestMismatch { .. }));
    let (observed, _) = check_json(result);
    all_expected &= expected;
    records.push(json!({
        "name": "digest_tamper",
        "operation": if extra.is_some() { "check_with_extra" } else { "check" },
        "source_certificate_id": cert.id,
        "expected_outcome": "digest_mismatch", "observed": observed,
        "expected_outcome_observed": expected,
    }));
    (records, all_expected)
}

fn generate_records() -> Result<Records, String> {
    let mut certificates = Vec::new();
    let mut issuance_checks = Vec::new();
    let mut all_expected_outcomes = true;
    for (statement, seed) in expected_statements().into_iter().zip(SEEDS) {
        let cert = issue(statement, seed, POINTS, INDEPENDENCE, None)
            .map_err(|error| format!("positive identity issuance failed: {error}"))?;
        let (observed, accepted) = check_json(check(&cert));
        all_expected_outcomes &= accepted;
        issuance_checks.push(json!({"certificate_id": cert.id, "observed": observed}));
        certificates.push(cert);
    }
    let (negative_controls, controls_passed) = negative_controls(&certificates[0], None);
    all_expected_outcomes &= controls_passed;
    Ok(Records {
        schema: RECORD_SCHEMA.into(),
        evidence_tier: "evaluated".into(),
        evidence_note: EVIDENCE_NOTE.into(),
        modulus: MODULUS,
        certificates,
        issuance_checks,
        negative_controls,
        all_expected_outcomes,
    })
}

fn replay_records(records: &Records, checker_seed: u64) -> Result<(Value, bool), String> {
    if records.schema != RECORD_SCHEMA
        || records.evidence_tier != "evaluated"
        || records.modulus != MODULUS
    {
        return Err("expected evaluated Dickson/S3 records over 2^61-1".into());
    }
    let expected = expected_statements();
    if records.certificates.len() != expected.len() {
        return Err("expected exactly the S3 and chord certificates, in that order".into());
    }
    let mut checks = Vec::new();
    let mut all_expected_outcomes = true;
    for (cert, statement) in records.certificates.iter().zip(expected) {
        // Production checks validate IDs and algebra, but do not require this
        // study's intended statement. Pin it here to reject a swapped tautology.
        if cert.statement != statement || cert.proof.is_some() {
            return Err(
                "certificate must contain the exact expected statement and proof:null".into(),
            );
        }
        if checker_seed == cert.seed {
            return Err("choose a fresh checker seed distinct from each issuer seed".into());
        }
        let (observed, accepted) =
            check_json(check_with_extra(cert, Some((checker_seed, EXTRA_POINTS))));
        all_expected_outcomes &= accepted;
        checks.push(json!({"certificate_id": cert.id, "observed": observed}));
    }
    // Recompute controls rather than trusting the input's stored receipt flags.
    // Generate a clean positive record for the digest mutation, even if the
    // supplied certificate already has a bad digest.
    let clean = issue(s3_statement(), SEEDS[0], POINTS, INDEPENDENCE, None)
        .map_err(|error| format!("control setup failed: {error}"))?;
    let (controls, controls_passed) = negative_controls(&clean, Some((checker_seed, EXTRA_POINTS)));
    all_expected_outcomes &= controls_passed;
    Ok((
        json!({
            "schema": "dickson-s3-identity-replay/v1",
            "evidence_tier": "evaluated", "evidence_note": EVIDENCE_NOTE,
            "modulus": MODULUS, "checker_seed": checker_seed,
            "extra_points_per_certificate": EXTRA_POINTS,
            "seed_freshness": "Distinct from issuer seeds; caller must ensure freshness across checker runs.",
            "input_receipts_trusted": false,
            "certificate_checks": checks, "negative_controls": controls,
            "all_expected_outcomes": all_expected_outcomes,
        }),
        all_expected_outcomes,
    ))
}

fn parse_seed(text: &str) -> Result<u64, String> {
    let parsed = if let Some(hex) = text.strip_prefix("0x") {
        u64::from_str_radix(hex, 16)
    } else {
        text.parse::<u64>()
    };
    parsed.map_err(|_| "SEED must be a decimal u64 or a 0x-prefixed hexadecimal u64".into())
}

fn run(args: &[String]) -> Result<(Value, bool), String> {
    match args {
        [] => {
            let records = generate_records()?;
            let passed = records.all_expected_outcomes;
            Ok((
                serde_json::to_value(records).map_err(|error| error.to_string())?,
                passed,
            ))
        }
        [flag, path, seed] if flag == "--check" => {
            let bytes = fs::read(path).map_err(|error| format!("read {path}: {error}"))?;
            let records = serde_json::from_slice::<Records>(&bytes)
                .map_err(|error| format!("parse records: {error}"))?;
            replay_records(&records, parse_seed(seed)?)
        }
        _ => Err("usage: dickson-s3-identity-records [--check PATH SEED]".into()),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (output, status) = match run(&args) {
        Ok((output, passed)) => (output, if passed { 0 } else { 1 }),
        Err(error) => (
            json!({
                "schema": "dickson-s3-identity-error/v1", "error": error,
                "all_expected_outcomes": false,
            }),
            2,
        ),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("JSON output serialises")
    );
    if status != 0 {
        process::exit(status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::identity_certificate::{
        syntactic_degree, validate_statement, ProofRef,
    };

    #[test]
    fn exact_statements_cover_all_variables_and_syntactic_degrees() {
        for mut statement in expected_statements() {
            assert_eq!(validate_statement(&statement), Ok(()));
            assert_eq!(syntactic_degree(&statement.lhs), 4);
            assert_eq!(syntactic_degree(&statement.rhs), 4);
            statement.degree_bound = 3;
            assert_eq!(
                validate_statement(&statement),
                Err(Malformed::DegreeBound {
                    declared: 3,
                    syntactic: 4
                })
            );
        }
    }

    #[test]
    fn identities_issue_over_the_production_modulus_without_proof_pointers() {
        let records = generate_records().expect("both standard identities issue");
        assert_eq!(records.modulus, (1u64 << 61) - 1);
        assert!(records.all_expected_outcomes);
        assert_eq!(records.certificates.len(), 2);
        for cert in &records.certificates {
            assert!(cert.proof.is_none());
            assert_eq!(cert.modulus, MODULUS);
            assert!(matches!(
                check(cert),
                Ok(Verdict::Accept {
                    points: POINTS,
                    extra_points: 0,
                    ..
                })
            ));
        }
    }

    #[test]
    fn controls_fail_for_the_intended_reasons() {
        let records = generate_records().unwrap();
        for control in &records.negative_controls[..2] {
            assert_eq!(control["observed"]["outcome"], "counterexample");
            assert_ne!(control["observed"]["lhs"], control["observed"]["rhs"]);
            assert_eq!(control["expected_outcome_observed"], true);
        }
        assert_eq!(
            records.negative_controls[2]["observed"]["outcome"],
            "digest_mismatch"
        );
        assert_eq!(
            records.negative_controls[2]["expected_outcome_observed"],
            true
        );
    }

    #[test]
    fn fresh_replay_uses_extra_points_and_ignores_stored_receipts() {
        let mut records = generate_records().unwrap();
        records.all_expected_outcomes = false;
        records.issuance_checks = vec![json!({"untrusted": true})];
        records.negative_controls = vec![json!({"untrusted": true})];
        let encoded = serde_json::to_vec(&records).unwrap();
        let decoded: Records = serde_json::from_slice(&encoded).unwrap();
        let (output, passed) = replay_records(&decoded, 0x1122_3344).unwrap();
        assert!(passed);
        assert_eq!(output["input_receipts_trusted"], false);
        for result in output["certificate_checks"].as_array().unwrap() {
            assert_eq!(result["observed"]["outcome"], "accepted_evaluations");
            assert_eq!(result["observed"]["extra_points"], EXTRA_POINTS);
        }
    }

    #[test]
    fn replay_refuses_resealed_replacement_statements_and_formal_proof_claims() {
        let mut records = generate_records().unwrap();
        let mut tautology = s3_statement();
        tautology.rhs = tautology.lhs.clone();
        records.certificates[0] = issue(tautology, SEEDS[0], POINTS, INDEPENDENCE, None).unwrap();
        assert!(replay_records(&records, 7).is_err());
        records = generate_records().unwrap();
        records.certificates[0].proof = Some(ProofRef {
            kind: "test-only-untrusted-pointer".into(),
            path: "test-only-not-a-proof-artifact".into(),
            sha256: "0".repeat(64),
        });
        assert!(replay_records(&records, 7).is_err());
        records = generate_records().unwrap();
        records.evidence_tier = "proved".into();
        assert!(replay_records(&records, 7).is_err());
    }

    #[test]
    fn digest_and_id_tampering_fail_even_with_true_stored_receipts() {
        let mut records = generate_records().unwrap();
        let digest = &mut records.certificates[0].value_digest;
        let replacement = if digest.starts_with('0') { "1" } else { "0" };
        digest.replace_range(..1, replacement);
        let (output, passed) = replay_records(&records, 7).unwrap();
        assert!(!passed);
        assert_eq!(
            output["certificate_checks"][0]["observed"]["outcome"],
            "digest_mismatch"
        );
        records = generate_records().unwrap();
        records.certificates[0].id.push('0');
        let (output, passed) = replay_records(&records, 7).unwrap();
        assert!(!passed);
        assert_eq!(
            output["certificate_checks"][0]["observed"]["outcome"],
            "malformed"
        );
    }

    #[test]
    fn seed_parser_and_freshness_checks_reject_invalid_requests() {
        assert_eq!(parse_seed("42"), Ok(42));
        assert_eq!(parse_seed("0x2a"), Ok(42));
        assert!(parse_seed("-1").is_err());
        assert!(parse_seed("18446744073709551616").is_err());
        let records = generate_records().unwrap();
        for seed in SEEDS {
            assert!(replay_records(&records, seed).is_err());
        }
    }
}
