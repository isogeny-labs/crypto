//! Issue a literal polynomial-identity certificate; no fixture search or experiment.
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
fn twice(x: &Expr) -> Expr {
    Expr::times(Expr::Const(2), x.clone())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: weighted_leading PROOF_SHA256 NEW_OUTPUT_FILE".into());
    }
    let [a1, a2, c0, c1, x] = ["a1", "a2", "c0", "c1", "x"].map(Expr::var);
    let d = Expr::sum(vec![
        pow(&x, 3),
        Expr::times(a2.clone(), pow(&x, 2)),
        Expr::times(a1.clone(), x.clone()),
    ]);
    let b = Expr::plus(c0.clone(), Expr::times(c1.clone(), x.clone()));
    let u4 = sub(twice(&a2), pow(&c1, 2));
    let u3 = sub(
        Expr::plus(pow(&a2, 2), twice(&a1)),
        Expr::product(vec![Expr::Const(2), c0.clone(), c1]),
    );
    let u2 = sub(Expr::times(twice(&a1), a2), pow(&c0, 2));
    let u = Expr::sum(vec![
        pow(&x, 5),
        Expr::times(u4, pow(&x, 4)),
        Expr::times(u3, pow(&x, 3)),
        Expr::times(u2, pow(&x, 2)),
        Expr::times(pow(&a1, 2), x.clone()),
    ]);
    let statement = IdentityStatement {
        name: "literal special-fiber norm expansion".into(),
        source: "NormWeightedLeading.leadingNormIdentity; literal expansion only, no remainder/weighted-initial-form or regularity assertion".into(),
        variables: ["a1", "a2", "c0", "c1", "x"].map(str::to_owned).to_vec(),
        degree_bound: 6,
        lhs: sub(pow(&d, 2), Expr::times(pow(&x, 3), pow(&b, 2))),
        rhs: Expr::times(x, u),
    };
    let cert=issue(statement,0x2026_1113_0000_0003,32,
        "unsigned producer invocation of unchanged canonical issuer/checker; literal algebra certificate construction, no independent signer or experiment",
        Some(ProofRef{kind:"lean4".into(),path:"research/prime_norm_rank_20261112/lean/NormWeightedLeading.lean".into(),sha256:args[1].clone()}))?;
    let verdict = check_with_extra(&cert, Some((0x4e4f_524d_5745_4947, 16)))?;
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
