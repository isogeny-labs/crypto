//! Reuse the repository's unchanged native certificate issuer/checker.
#[path = "../../../../src/hash/sha256.rs"]
pub mod canonical_sha256;
pub mod hash {
    pub use crate::canonical_sha256 as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;

use identity_certificate::{check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict};
use std::{fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: norm-bridge-identity-control PROOF_SHA256 OUTPUT_NEW_FILE".into());
    }
    let a = Expr::var("A");
    let b = Expr::var("B");
    let f = Expr::var("F");
    let statement = IdentityStatement {
        name: "short-form coordinate multiplication matrix determinant".into(),
        source: "Mathlib CoordinateRing.norm_smul_basis; NormGroupBridge.normFunctionNorm; literal det[[A,-F*B],[-B,A]] expansion only".into(),
        variables: vec!["A".into(), "B".into(), "F".into()],
        degree_bound: 3,
        lhs: Expr::plus(
            Expr::times(a.clone(), a.clone()),
            Expr::minus(Expr::times(
                Expr::minus(Expr::times(f.clone(), b.clone())),
                Expr::minus(b.clone()),
            )),
        ),
        rhs: Expr::plus(
            Expr::times(a.clone(), a),
            Expr::minus(Expr::times(f, Expr::times(b.clone(), b))),
        ),
    };
    let cert = issue(
        statement,
        0x2026_1113_0000_0001,
        32,
        "unsigned producer invocation of unchanged canonical issuer/checker; no independent signer; extra replay stream uses separate fixed seed; formal pointer is checked separately by Lean gate/fresh",
        Some(ProofRef {
            kind: "lean4".into(),
            path: "research/prime_norm_rank_20261112/lean/NormGroupBridge.lean".into(),
            sha256: args[1].clone(),
        }),
    )?;
    let verdict = check_with_extra(&cert, Some((0x4e4f_524d_4252_4944, 16)))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("certificate rejected: {verdict:?}").into());
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    output.write_all(serde_json::to_string_pretty(&cert)?.as_bytes())?;
    println!(
        "{}: {verdict:?}; unsigned native certificate replay",
        cert.id
    );
    Ok(())
}
