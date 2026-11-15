//! Literal target norm-derivative factorization, separate from the conditional fiber proofs.
#[path = "../../../../../src/hash/sha256.rs"]
pub mod canonical_sha256;
pub mod hash {
    pub use crate::canonical_sha256 as sha256;
}
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;
use identity_certificate::{check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict};
use std::{fs::OpenOptions, io::Write};
fn square(x: &Expr) -> Expr {
    Expr::times(x.clone(), x.clone())
}
fn sub(a: Expr, b: Expr) -> Expr {
    Expr::plus(a, Expr::minus(b))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: target_fiber PROOF_SHA256 NEW_OUTPUT_FILE".into());
    }
    let [b, z, d, f, c] = ["B", "Z", "D", "F", "C"].map(Expr::var);
    let lhs = sub(
        sub(
            Expr::product(vec![Expr::Const(2), z.clone(), b.clone(), d.clone()]),
            Expr::times(f.clone(), square(&b)),
        ),
        Expr::product(vec![Expr::Const(2), square(&z), b.clone(), c.clone()]),
    );
    let rhs = Expr::times(
        b.clone(),
        sub(
            sub(
                Expr::product(vec![Expr::Const(2), z.clone(), d]),
                Expr::times(f, b),
            ),
            Expr::product(vec![Expr::Const(2), square(&z), c]),
        ),
    );
    let statement=IdentityStatement {
        name:"literal evaluated norm-derivative factor".into(),
        source:"NormTargetFiber.normQuotientTargetFactor after target substitutions; literal factoring only, no curve premise, parameter divisibility, specialization, or row-equivalence assertion".into(),
        variables:["B","Z","D","F","C"].map(str::to_owned).to_vec(),degree_bound:4,lhs,rhs,
    };
    let cert=issue(statement,0x2026_1114_0000_0002,32,"unsigned literal algebra certificate; conditional ring/field and parameter-polynomial proofs are separate",
        Some(ProofRef{kind:"lean4".into(),path:"research/prime_norm_rank_20261112/lean/NormTargetFiber.lean".into(),sha256:args[1].clone()}))?;
    let verdict = check_with_extra(&cert, Some((0x5441_5247_4649_4245, 16)))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}").into());
    }
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    out.write_all(serde_json::to_string_pretty(&cert)?.as_bytes())?;
    println!("{}: {verdict:?}; literal factor only, unsigned", cert.id);
    Ok(())
}
