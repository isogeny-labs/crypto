//! Seal and replay central polynomial identities using the repository contract.
//!
//! The characteristic-two record is its all-ring integer lift with the explicit
//! twice-the-correction term. It does not evaluate a characteristic-two-only
//! equality in the contract's odd Mersenne-prime field.

use std::{env, error::Error, fs, path::PathBuf};

use serde_json::json;

// The upstream module intentionally imports the repository's hash interface.
// This shim preserves it while keeping this small checker a standalone crate.
mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};

        pub fn sha256(input: &[u8]) -> [u8; 32] {
            Sha256::digest(input).into()
        }
    }
}

#[allow(dead_code)]
#[rustfmt::skip]
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;

use identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, ProofRef, Verdict,
};

const PROOF_PATH: &str = "research/guarded_fall_degree_20261028/formal/FallDegreeIdentities.lean";
const POINTS: u32 = 32;
const EXTRA_POINTS: u32 = 32;
const SEED: u64 = 20_261_028;
const CHECKER_SEED: u64 = 0xDE63_EE05_2026_1028;

fn variable(name: &str) -> Expr {
    Expr::var(name)
}

fn constant(value: i64) -> Expr {
    Expr::Const(value)
}

fn plus(left: Expr, right: Expr) -> Expr {
    Expr::plus(left, right)
}

fn minus(left: Expr, right: Expr) -> Expr {
    plus(left, Expr::minus(right))
}

fn times(left: Expr, right: Expr) -> Expr {
    Expr::times(left, right)
}

fn square(value: Expr) -> Expr {
    times(value.clone(), value)
}

fn cube(value: Expr) -> Expr {
    times(square(value.clone()), value)
}

fn sum(values: Vec<Expr>) -> Expr {
    Expr::sum(values)
}

fn product(values: Vec<Expr>) -> Expr {
    Expr::product(values)
}

fn statement(
    name: &str,
    declaration: &str,
    variables: &[&str],
    degree_bound: u32,
    lhs: Expr,
    rhs: Expr,
) -> IdentityStatement {
    IdentityStatement {
        name: name.to_owned(),
        source: format!("{PROOF_PATH}: FallDegreeIdentities.{declaration}"),
        variables: variables.iter().map(|name| (*name).to_owned()).collect(),
        degree_bound,
        lhs,
        rhs,
    }
}

fn central_statements() -> Vec<(&'static str, IdentityStatement)> {
    let a = variable("A");
    let b = variable("B");
    let c = variable("C");
    let d = variable("D");
    let u = variable("u");

    let e = minus(a.clone(), c.clone());
    let j = minus(b.clone(), d.clone());
    let f = sum(vec![
        times(constant(2), j.clone()),
        Expr::minus(square(a.clone())),
        square(c.clone()),
    ]);
    let n = minus(
        times(f.clone(), u.clone()),
        times(e.clone(), plus(b.clone(), d.clone())),
    );
    let l = plus(times(e.clone(), u.clone()), j.clone());
    let resultant = sum(vec![
        square(j.clone()),
        Expr::minus(product(vec![a.clone(), e.clone(), j.clone()])),
        times(b.clone(), square(e.clone())),
    ]);
    let odd = statement(
        "odd_resultant_cancellation",
        "odd_resultant_cancellation",
        &["A", "B", "C", "D", "u"],
        4,
        minus(times(e, n), times(f, l)),
        times(constant(-2), resultant),
    );

    let e = plus(a.clone(), c);
    let j = plus(b.clone(), d);
    let k = square(e.clone());
    let l = plus(times(e.clone(), u.clone()), j.clone());
    let p = plus(times(k.clone(), u.clone()), times(e.clone(), j.clone()));
    let h1 = sum(vec![
        square(u.clone()),
        times(a.clone(), u.clone()),
        b.clone(),
    ]);
    let binary_lhs = sum(vec![
        times(k.clone(), h1),
        times(u.clone(), p.clone()),
        times(j.clone(), l),
        times(a.clone(), p),
    ]);
    let binary_resultant = sum(vec![
        square(j.clone()),
        product(vec![a.clone(), e.clone(), j.clone()]),
        times(b, k.clone()),
    ]);
    let correction = sum(vec![
        times(k.clone(), square(u.clone())),
        product(vec![k, a, u.clone()]),
        product(vec![e, j, u]),
    ]);
    let binary_lift = statement(
        "char_two_resultant_integer_lift",
        "char_two_resultant_integer_lift",
        &["A", "B", "C", "D", "u"],
        4,
        binary_lhs,
        plus(binary_resultant, times(constant(2), correction)),
    );

    let v = variable("v");
    let delta = variable("d");
    let q = variable("q");
    let mut guard = statement(
        "inverse_guard_divided_difference",
        "inverse_guard_divided_difference",
        &["v", "d", "q"],
        3,
        minus(
            product(vec![v.clone(), delta.clone(), q.clone()]),
            times(q.clone(), minus(times(v, delta), constant(1))),
        ),
        q,
    );
    guard.source.push_str("; specialization gx=d*q, gy=0");

    let s1 = variable("s1");
    let s2 = variable("s2");
    let s3 = variable("s3");
    let x = variable("x");
    let y = variable("y");
    let w = minus(minus(s1.clone(), x.clone()), y.clone());
    let g = |value: Expr| {
        sum(vec![
            cube(value.clone()),
            Expr::minus(times(s1.clone(), square(value.clone()))),
            times(s2.clone(), value),
            Expr::minus(s3.clone()),
        ])
    };
    let q = sum(vec![
        square(x.clone()),
        times(x.clone(), y.clone()),
        square(y.clone()),
        Expr::minus(times(s1.clone(), plus(x.clone(), y.clone()))),
        s2.clone(),
    ]);
    let missing = statement(
        "cubic_missing_root",
        "cubic_missing_root",
        &["s1", "s2", "s3", "x", "y"],
        3,
        g(w),
        plus(
            g(y.clone()),
            times(minus(minus(s1.clone(), x), times(constant(2), y)), q),
        ),
    );

    vec![
        ("odd_resultant_cancellation", odd),
        ("char_two_resultant_integer_lift", binary_lift),
        ("inverse_guard_divided_difference", guard),
        ("cubic_missing_root", missing),
    ]
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("check");
    if args.len() > 2 || !matches!(command, "issue" | "check") {
        return Err("usage: identity_records [issue|check]".into());
    }
    let study = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("cannot locate study directory")?
        .to_owned();
    let formal = study.join("formal/FallDegreeIdentities.lean");
    let proof_bytes = fs::read(&formal)?;
    let proof_text = std::str::from_utf8(&proof_bytes)?;
    for declaration in [
        "theorem odd_resultant_cancellation",
        "theorem char_two_resultant_integer_lift",
        "theorem inverse_guard_divided_difference",
        "theorem cubic_missing_root",
    ] {
        if !proof_text.contains(declaration) {
            return Err(
                format!("formal artifact lacks {declaration}; records were not issued").into(),
            );
        }
    }
    let proof = ProofRef {
        kind: "lean4".to_owned(),
        path: PROOF_PATH.to_owned(),
        sha256: hex::encode(hash::sha256::sha256(&proof_bytes)),
    };
    let directory = study.join("identities");
    if command == "issue" {
        fs::create_dir_all(&directory)?;
    }
    let mut records = Vec::new();
    for (position, (slug, statement)) in central_statements().into_iter().enumerate() {
        let path = directory.join(format!("{slug}.json"));
        if command == "issue" {
            let certificate = issue(
                statement.clone(),
                SEED + position as u64,
                POINTS,
                "native issue/check with statement-bound points and a domain-separated extra stream; not an independent-host or Lean-prover invocation",
                Some(proof.clone()),
            )?;
            fs::write(&path, serde_json::to_string_pretty(&certificate)? + "\n")?;
        }
        // Replay serialized records instead of trusting the issuer's object.
        let certificate: IdentityCertificate = serde_json::from_slice(&fs::read(&path)?)?;
        if certificate.statement != statement || certificate.proof.as_ref() != Some(&proof) {
            return Err(format!(
                "statement or proof artifact binding mismatch in {}",
                path.display()
            )
            .into());
        }
        match check_with_extra(
            &certificate,
            Some((CHECKER_SEED + position as u64, EXTRA_POINTS)),
        )? {
            Verdict::Accept {
                points,
                extra_points,
                false_accept_log2,
            } => records.push(json!({
                "file": format!("{slug}.json"),
                "id": certificate.id,
                "name": certificate.statement.name,
                "modulus": certificate.modulus,
                "points": points,
                "extra_checker_points": extra_points,
                "false_accept_log2": false_accept_log2,
                "proof_sha256": proof.sha256,
                "verdict": "accept"
            })),
            other => return Err(format!("identity replay rejected {slug}: {other:?}").into()),
        }
    }
    let report = json!({
        "schema": "fall-degree-identity-replay/v1",
        "records_replayed": records.len(),
        "all_accepted": true,
        "proof": proof,
        "records": records,
        "scope": "probabilistic identity checks over the repository Mersenne prime; the binary record is the all-ring integer lift, and Lean proof references are recorded rather than executed here"
    });
    if command == "issue" {
        fs::write(
            directory.join("replay.json"),
            serde_json::to_string_pretty(&report)? + "\n",
        )?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("identity_records: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_four_all_ring_identities_issue_and_replay() {
        for (_, statement) in central_statements() {
            let record = issue(statement, SEED, POINTS, "native unit check", None).unwrap();
            assert!(matches!(
                check_with_extra(&record, Some((CHECKER_SEED, EXTRA_POINTS))).unwrap(),
                Verdict::Accept { .. }
            ));
        }
    }

    #[test]
    fn characteristic_two_equality_is_not_misrepresented_in_an_odd_field() {
        let mut statement = central_statements().remove(1).1;
        if let Expr::Add(parts) = &mut statement.rhs {
            parts.pop(); // Discard the twice-correction term: false in this odd field.
        } else {
            panic!("integer-lift right-hand side shape changed");
        }
        assert!(matches!(
            issue(statement, SEED, POINTS, "negative native unit check", None),
            Err(identity_certificate::IssueError::False(_))
        ));
    }
}
