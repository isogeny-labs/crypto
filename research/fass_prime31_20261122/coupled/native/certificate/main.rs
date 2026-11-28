//! Fixed-d native evaluation certificate, using the unchanged published checker.
//! This does not certify the generic-rank theorem or an identity for all d.

#[allow(dead_code)]
#[path = "../../../../../src/hash/sha256.rs"]
pub mod original_sha256;

pub mod hash {
    pub use crate::original_sha256 as sha256;
}

#[allow(dead_code)]
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;

use identity_certificate::{
    check_with_extra, issue, syntactic_degree, Expr, IdentityCertificate, IdentityStatement,
    IssueError, Verdict, MODULUS,
};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const D: usize = 6;
const POINTS: u32 = 32;
const ISSUER_SEED: u64 = 0x4355_5350_5f44_3036;
const PRODUCER_REPLAY_SEED: u64 = 0x4336_4558_5452_4131;
const SCOPE: &str = "fixed_d6_univariate_identity_over_2^61-1_evaluated_only";

fn pow(base: Expr, exponent: usize) -> Expr {
    Expr::product(vec![base; exponent])
}

fn binomial(n: usize, k: usize) -> i64 {
    let mut value = 1i64;
    for i in 1..=k.min(n - k) {
        value = value * (n + 1 - i) as i64 / i as i64;
    }
    value
}

fn statement() -> IdentityStatement {
    let t = Expr::var("T");
    let alpha = pow(Expr::plus(Expr::Const(1), Expr::minus(t.clone())), 2);
    let beta = Expr::product(vec![
        Expr::Const(-2),
        t.clone(),
        Expr::plus(Expr::Const(1), t.clone()),
    ]);
    let gamma = pow(t.clone(), 2);

    // If r1+r2=-beta and r1*r2=alpha*gamma, then
    // U_j=r1^j+r2^j=-beta*U_(j-1)-alpha*gamma*U_(j-2).
    let mut u_previous = Expr::Const(2);
    let mut u_current = Expr::minus(beta.clone());
    for _ in 2..=D {
        let u_next = Expr::sum(vec![
            Expr::minus(Expr::times(beta.clone(), u_current.clone())),
            Expr::minus(Expr::product(vec![
                alpha.clone(),
                gamma.clone(),
                u_previous,
            ])),
        ]);
        u_previous = u_current;
        u_current = u_next;
    }
    let lhs = Expr::sum(vec![pow(alpha, D), pow(gamma, D), Expr::minus(u_current)]);

    let even_binomial_sum = Expr::sum(
        (0..=D)
            .map(|j| Expr::times(Expr::Const(binomial(2 * D, 2 * j)), pow(t.clone(), j)))
            .collect(),
    );
    let rhs = Expr::sum(vec![
        pow(Expr::plus(t.clone(), Expr::Const(-1)), 2 * D),
        pow(t.clone(), 2 * D),
        Expr::product(vec![Expr::Const(-2), pow(t, D), even_binomial_sum]),
    ]);
    IdentityStatement {
        name: "Coupled S3 singular specialization: d=6 quadratic norm recurrence equals closed h(T)".into(),
        source: "Written specialization of H=Res_z(z^d-1,S3(x,z,t)) modulo x^d-1 at A=B=0, d=6. This record evaluates only the displayed univariate recurrence identity; it is not the generic-rank proof.".into(),
        variables: vec!["T".into()],
        degree_bound: (2 * D) as u32,
        lhs,
        rhs,
    }
}

fn check_expected(certificate: &IdentityCertificate, seed: u64) -> (u32, u32) {
    assert_eq!(certificate.statement, statement());
    assert_eq!(certificate.modulus, MODULUS);
    assert!(certificate.proof.is_none());
    match check_with_extra(certificate, Some((seed, POINTS))).expect("well-formed certificate") {
        Verdict::Accept {
            points,
            extra_points,
            ..
        } => (points, extra_points),
        verdict => panic!("certificate rejected: {verdict:?}"),
    }
}

fn mutation_controls(certificate: &IdentityCertificate) -> (bool, bool) {
    let mut false_statement = statement();
    false_statement.rhs = Expr::plus(false_statement.rhs, Expr::Const(1));
    assert!(matches!(
        issue(
            false_statement,
            ISSUER_SEED,
            POINTS,
            "negative control",
            None
        ),
        Err(IssueError::False(_))
    ));
    let mut changed_digest = certificate.clone();
    let replacement = if &changed_digest.value_digest[..1] == "0" {
        "1"
    } else {
        "0"
    };
    changed_digest.value_digest.replace_range(..1, replacement);
    assert!(matches!(
        check_with_extra(&changed_digest, Some((PRODUCER_REPLAY_SEED, POINTS))),
        Ok(Verdict::DigestMismatch { .. })
    ));
    (true, true)
}

fn emit_summary(
    mode: &str,
    certificate: &IdentityCertificate,
    extra_seed: u64,
    counts: (u32, u32),
    controls: Option<(bool, bool)>,
) {
    println!(
        "{}",
        serde_json::json!({
            "kind": "identity_certificate_check",
            "mode": mode,
            "id": certificate.id,
            "d": D,
            "modulus": MODULUS,
            "degree_bound": certificate.statement.degree_bound,
            "lhs_syntactic_degree": syntactic_degree(&certificate.statement.lhs),
            "rhs_syntactic_degree": syntactic_degree(&certificate.statement.rhs),
            "points": counts.0,
            "extra_points": counts.1,
            "extra_seed": extra_seed,
            "negative_statement_rejected": controls.map(|v| v.0),
            "changed_digest_rejected": controls.map(|v| v.1),
            "proof": certificate.proof,
            "scope": SCOPE,
            "passed": true
        })
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("issue") if args.len() == 3 => {
            let certificate = issue(
                statement(),
                ISSUER_SEED,
                POINTS,
                "Producer-issued and producer-replayed with a separate deterministic point stream. No signed independent validation or formal proof is claimed; any independent replay is recorded in a separate receipt.",
                None,
            )
            .expect("fixed-d recurrence agrees with the closed expression");
            let counts = check_expected(&certificate, PRODUCER_REPLAY_SEED);
            let controls = mutation_controls(&certificate);
            let mut bytes = serde_json::to_vec_pretty(&certificate).expect("certificate serializes");
            bytes.push(b'\n');
            let path = Path::new(&args[2]);
            let mut output = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .expect("certificate path must not already exist");
            output.write_all(&bytes).expect("certificate write succeeds");
            output.sync_all().expect("certificate sync succeeds");
            let read_back: IdentityCertificate = serde_json::from_slice(
                &fs::read(path).expect("certificate reads back"),
            )
            .expect("written certificate parses");
            assert_eq!(certificate, read_back);
            emit_summary("issue_and_producer_replay", &certificate, PRODUCER_REPLAY_SEED, counts, Some(controls));
        }
        Some("check") if args.len() == 3 || args.len() == 4 => {
            let seed = args.get(3).map_or(PRODUCER_REPLAY_SEED, |s| s.parse().expect("decimal checker seed"));
            let certificate: IdentityCertificate = serde_json::from_slice(
                &fs::read(&args[2]).expect("certificate is readable"),
            )
            .expect("certificate JSON parses");
            let counts = check_expected(&certificate, seed);
            emit_summary("replay", &certificate, seed, counts, None);
        }
        _ => panic!("usage: fass-coupled-cusp-certificate issue OUTPUT | check CERTIFICATE [DECIMAL_CHECKER_SEED]"),
    }
}
