//! The existing native identity-certificate format applied to the sparse H2
//! quartic expansion. Evaluation is in the format's 2^61-1 field, not Prime31.
//! Fixed reproducible seeds do not provide an unconditional statistical bound.

use crate::identity_certificate::{
    self, Expr, IdentityCertificate, IdentityStatement, IssueError, Malformed, Verdict,
};
use serde_json::{json, Value};

pub const ISSUER_SEED: u64 = 4188;
pub const ISSUER_POINTS: u32 = 64;
pub const EXTRA_CHECKER_SEED: u64 = 20261123;
pub const EXTRA_CHECKER_POINTS: u32 = 32;

pub struct CheckedSparseIdentity {
    pub certificate: IdentityCertificate,
    pub audit: Value,
}

/// (x^2-1)^2+4x^2-z*x*(x^2-1) = x^4-z*x^3+2x^2+z*x+1.
/// This is an expression-tree expansion, with no rational denominators hidden
/// in the statement and syntactic degree four on both sides.
pub fn statement() -> IdentityStatement {
    let x = Expr::var("x");
    let z = Expr::var("z");
    let x2 = Expr::times(x.clone(), x.clone());
    let numerator1 = Expr::plus(x2.clone(), Expr::Const(-1));
    let lhs = Expr::sum(vec![
        Expr::times(numerator1.clone(), numerator1.clone()),
        Expr::times(Expr::Const(4), x2.clone()),
        Expr::minus(Expr::product(vec![z.clone(), x.clone(), numerator1])),
    ]);
    let rhs = Expr::sum(vec![
        Expr::times(x2.clone(), x2.clone()),
        Expr::minus(Expr::product(vec![z.clone(), x2.clone(), x.clone()])),
        Expr::times(Expr::Const(2), x2),
        Expr::times(z, x),
        Expr::Const(1),
    ]);
    IdentityStatement {
        name: "Normalized full-two-torsion quotient quartic membership expansion".into(),
        source: "research/prime31_aux_matched_20261122/sparse-h2-followup/MATHEMATICS.md; composition of x-1/x and z+4/z, followed by denominator clearing".into(),
        variables: vec!["x".into(), "z".into()],
        degree_bound: 4,
        lhs,
        rhs,
    }
}

fn accepted(verdict: Verdict, expected_extra: u32) -> Result<Value, String> {
    match verdict {
        Verdict::Accept {
            points,
            extra_points,
            false_accept_log2,
        } if points == ISSUER_POINTS && extra_points == expected_extra => Ok(json!({
            "status":"accept",
            "issuer_points":points,
            "extra_points":extra_points,
            "library_conditional_false_accept_log2":false_accept_log2
        })),
        other => Err(format!("unexpected native identity verdict: {other:?}")),
    }
}

/// Issue and replay this one fixed identity through the repository's existing
/// native issuer. Negative controls exercise issuance, statement binding and
/// value-digest refusal. No file is written and no verdict is signed here.
pub fn issue_checked() -> Result<CheckedSparseIdentity, String> {
    let source = statement();
    let independence = "Unsigned producer-native issuance and replay; extra-point stream uses fixed separately registered seed 20261123. No independent operator or formal proof is asserted.";
    let certificate = identity_certificate::issue(
        source.clone(),
        ISSUER_SEED,
        ISSUER_POINTS,
        independence,
        None,
    )
    .map_err(|error| error.to_string())?;
    let issuer_replay = accepted(
        identity_certificate::check(&certificate).map_err(|error| error.to_string())?,
        0,
    )?;
    let extra_replay = accepted(
        identity_certificate::check_with_extra(
            &certificate,
            Some((EXTRA_CHECKER_SEED, EXTRA_CHECKER_POINTS)),
        )
        .map_err(|error| error.to_string())?,
        EXTRA_CHECKER_POINTS,
    )?;

    let mut false_statement = source.clone();
    false_statement.rhs = Expr::plus(false_statement.rhs, Expr::Const(1));
    let false_issuance = match identity_certificate::issue(
        false_statement.clone(),
        ISSUER_SEED,
        ISSUER_POINTS,
        "Unsigned negative control: right side changed by the nonzero constant one",
        None,
    ) {
        Err(IssueError::False(counterexample)) => json!({
            "status":"refused_false_identity",
            "point_index":counterexample.index,
            "assignment":counterexample.assignment,
            "lhs":counterexample.lhs,
            "rhs":counterexample.rhs
        }),
        other => {
            return Err(format!(
                "constant-one false identity was not refused: {other:?}"
            ))
        }
    };
    let mut swapped = certificate.clone();
    swapped.statement = false_statement;
    if !matches!(
        identity_certificate::check(&swapped),
        Err(Malformed::IdMismatch { .. })
    ) {
        return Err("statement mutation under the retained identity id was not refused".into());
    }
    let mut bad_digest = certificate.clone();
    let replacement = if bad_digest.value_digest.starts_with('0') {
        "1"
    } else {
        "0"
    };
    bad_digest.value_digest.replace_range(..1, replacement);
    if !matches!(
        identity_certificate::check(&bad_digest),
        Ok(Verdict::DigestMismatch { .. })
    ) {
        return Err("value-digest mutation was not refused".into());
    }
    let audit = json!({
        "schema":"prime31-sparse-h2.identity-check/v1",
        "certificate_id":certificate.id,
        "certificate_schema":certificate.schema,
        "evaluation_modulus":certificate.modulus,
        "issuer_seed":ISSUER_SEED,
        "issuer_points":ISSUER_POINTS,
        "extra_checker_seed":EXTRA_CHECKER_SEED,
        "extra_checker_points":EXTRA_CHECKER_POINTS,
        "issuer_replay":issuer_replay,
        "extra_replay":extra_replay,
        "negative_controls":{
            "constant_one_mutation":false_issuance,
            "statement_changed_with_retained_id":"refused_id_mismatch",
            "value_digest_mutation":"refused_digest_mismatch"
        },
        "signed_verdict":false,
        "formal_proof":null,
        "evidence_scope":"Unsigned native evaluation/replay of this fixed degree-four polynomial identity over 2^61-1; it does not certify the Prime31 kernel, rational pole handling, complete ECDLP recovery, or performance.",
        "statistical_scope":"The native library reports the Schwartz-Zippel expression conditional on independent uniformly random evaluation points. These registered seeds and derived streams are deterministic; no unconditional false-accept probability is asserted.",
        "independence":independence
    });
    Ok(CheckedSparseIdentity { certificate, audit })
}
