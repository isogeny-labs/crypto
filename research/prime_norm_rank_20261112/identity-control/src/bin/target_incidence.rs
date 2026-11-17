//! Seal only the literal target-completion expansion, using the unchanged canonical checker.
#[path = "../../../../../src/hash/sha256.rs"]
pub mod canonical_sha256;
pub mod hash {
    pub use crate::canonical_sha256 as sha256;
}
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;
use identity_certificate::{check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict};
use std::{fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: target_incidence PROOF_SHA256 NEW_OUTPUT_FILE".into());
    }
    let d = Expr::var("D");
    let b = Expr::var("B");
    let z = Expr::var("Z");
    let completed = Expr::plus(
        d.clone(),
        Expr::plus(Expr::times(z.clone(), b.clone()), Expr::minus(d)),
    );
    let lhs = Expr::plus(
        Expr::times(completed.clone(), completed),
        Expr::minus(Expr::product(vec![z.clone(), z, b.clone(), b])),
    );
    let statement=IdentityStatement {
        name:"literal target-completed norm evaluation".into(),
        source:"NormMembershipElimination.completionRow/normAtTarget; substitute target row and f(q)=Z^2; literal polynomial expansion only".into(),
        variables:vec!["D".into(),"B".into(),"Z".into()],degree_bound:4,lhs,rhs:Expr::Const(0),
    };
    let cert=issue(statement,0x2026_1113_0000_0002,32,
        "unsigned producer invocation of unchanged canonical issuer/checker; extra stream uses separate fixed seed; no independent signer; Lean pointer checked separately",
        Some(ProofRef{kind:"lean4".into(),path:"research/prime_norm_rank_20261112/lean/NormMembershipElimination.lean".into(),sha256:args[1].clone()}))?;
    let verdict = check_with_extra(&cert, Some((0x4e4f_524d_5441_5247, 16)))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}").into());
    }
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    out.write_all(serde_json::to_string_pretty(&cert)?.as_bytes())?;
    println!("{}: {verdict:?}; literal expansion only, unsigned", cert.id);
    Ok(())
}
