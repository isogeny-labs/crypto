//! Seal a literal normalization step, separate from conditional inverse-pair transport.
#[path = "../../../../../src/hash/sha256.rs"]
pub mod canonical_sha256;
pub mod hash {
    pub use crate::canonical_sha256 as sha256;
}
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;
use identity_certificate::{check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict};
use std::{fs::OpenOptions, io::Write};

fn pow(x: &Expr, n: usize) -> Expr {
    Expr::product(vec![x.clone(); n])
}
fn sub(a: Expr, b: Expr) -> Expr {
    Expr::plus(a, Expr::minus(b))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: pole_transport PROOF_SHA256 NEW_OUTPUT_FILE".into());
    }
    let [e, a, f, b] = ["e", "A", "F", "B"].map(Expr::var);
    let statement = IdentityStatement {
        name: "literal pole-normalized norm factorization".into(),
        source: "NormPoleTransport.normValueScale intermediate unconditional ring step; does not encode the inverse-pair premise or polynomial/remainder transport".into(),
        variables: ["e", "A", "F", "B"].map(str::to_owned).to_vec(),
        degree_bound: 15,
        lhs: Expr::times(pow(&e,12), sub(pow(&a,2),Expr::times(f.clone(),pow(&b,2)))),
        rhs: sub(pow(&Expr::times(pow(&e,6),a),2),Expr::times(Expr::times(pow(&e,6),f),pow(&Expr::times(pow(&e,3),b),2))),
    };
    let cert=issue(statement,0x2026_1114_0000_0001,32,
        "unsigned finite literal algebra certificate; no independent signer, experiment or proof of conditional transport",
        Some(ProofRef{kind:"lean4".into(),path:"research/prime_norm_rank_20261112/lean/NormPoleTransport.lean".into(),sha256:args[1].clone()}))?;
    let verdict = check_with_extra(&cert, Some((0x504f_4c45_5452_414e, 16)))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}").into());
    }
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    out.write_all(serde_json::to_string_pretty(&cert)?.as_bytes())?;
    println!("{}: {verdict:?}; literal step only, unsigned", cert.id);
    Ok(())
}
