//! Native standard IDC wrapper for the universally proved cubic identity.
#![allow(dead_code)]
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::Path;

pub mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            Sha256::digest(bytes).into()
        }
    }
}
#[path = "../../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, ProofRef, Verdict};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: identity LEAN_PROOF FRESH_OUT");
        std::process::exit(64);
    }
    let proof = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    assert!(!out.exists());
    let proof_sha = hex::encode(Sha256::digest(std::fs::read(proof).unwrap()));
    let (a, x, y, z) = (E::var("a"), E::var("x"), E::var("y"), E::var("z"));
    let lhs = E::sum(vec![
        E::product(vec![a.clone(), a.clone(), a.clone()]),
        E::minus(E::product(vec![
            a.clone(),
            a.clone(),
            E::sum(vec![x.clone(), y.clone(), z.clone()]),
        ])),
        E::times(
            a.clone(),
            E::sum(vec![
                E::times(x.clone(), y.clone()),
                E::times(x.clone(), z.clone()),
                E::times(y.clone(), z.clone()),
            ]),
        ),
        E::minus(E::product(vec![x.clone(), y.clone(), z.clone()])),
    ]);
    let rhs = E::product(vec![
        E::plus(a.clone(), E::minus(x)),
        E::plus(a.clone(), E::minus(y)),
        E::plus(a, E::minus(z)),
    ]);
    let statement = IdentityStatement {
        name: "Cubic evaluated at a peeled factor".into(),
        source: "RepeatedFactor.cubic_identity; collision_certificate/proof/RepeatedFactor.lean"
            .into(),
        variables: vec!["a".into(), "x".into(), "y".into(), "z".into()],
        degree_bound: 3,
        lhs,
        rhs,
    };
    let cert=identity::issue(statement.clone(),20261025,64,
        "Native issue/check in one process; checker uses a separate extra-point domain. Universal Lean identity independently kernel-checked by the retained gate; no distinct-model attestation claimed.",
        Some(ProofRef{kind:"lean4".into(),path:args[1].clone(),sha256:proof_sha.clone()})).unwrap();
    std::fs::create_dir_all(out).unwrap();
    let cert_path = out.join("identity.json");
    std::fs::write(&cert_path, serde_json::to_vec_pretty(&cert).unwrap()).unwrap();
    let stored: identity::IdentityCertificate =
        serde_json::from_slice(&std::fs::read(&cert_path).unwrap()).unwrap();
    let checked = identity::check_with_extra(&stored, Some((0x544749435f434f4c, 64))).unwrap();
    let (points, extra_points) = match checked {
        Verdict::Accept {
            points,
            extra_points,
            ..
        } => (points, extra_points),
        other => panic!("IDC rejected: {other:?}"),
    };
    let mut false_statement = statement;
    false_statement.lhs = E::plus(false_statement.lhs, E::Const(1));
    let false_control = identity::issue(
        false_statement,
        20261025,
        64,
        "False-identity rejection control",
        None,
    );
    assert!(matches!(false_control, Err(identity::IssueError::False(_))));
    std::fs::write(
        out.join("false-identity-control.txt"),
        format!("Expected refusal: {false_control:?}\n"),
    )
    .unwrap();
    std::fs::write(out.join("summary.json"),serde_json::to_vec_pretty(&json!({"schema":"twist-collision-cubic-idc/v1","status":"pass","id":cert.id,
        "proof_sha256":proof_sha,"points":points,"extra_points":extra_points,"false_identity_refused":true,
        "scope":"Standard IDC for the universal cubic identity. Instance rejection certificates are separately expanded over their actual small prime field; this fixed-modulus IDC is not a replacement for those checks."})).unwrap()).unwrap();
    println!("{}", cert.id);
}
