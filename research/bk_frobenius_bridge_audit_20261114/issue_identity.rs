//! Scalar supplement; official checker, no geometric or timing computation.
mod hash {
    #[path = "/workspace/crypto-audit/src/hash/sha256.rs"]
    pub mod sha256;
}
#[path = "/workspace/crypto-audit/src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, ProofRef, Verdict};
fn c(n: i64) -> E { E::Const(n) }
fn v(n: &str) -> E { E::var(n) }
fn add(xs: Vec<E>) -> E { E::sum(xs) }
fn mul(xs: Vec<E>) -> E { E::product(xs) }
fn pow(x: E, n: usize) -> E { mul(vec![x;n]) }
fn neg(x: E) -> E { E::minus(x) }
fn main() {
    let proof_hash = std::env::args().nth(1).expect("Lean source SHA256");
    let (x,y)=(v("x"),v("y"));
    let lhs=add(vec![pow(y.clone(),4),neg(y.clone()),neg(pow(x.clone(),6)),neg(pow(x.clone(),3))]);
    let equation=add(vec![pow(y.clone(),2),y.clone(),neg(pow(x.clone(),3)),c(-1)]);
    let factor=add(vec![pow(y.clone(),2),y.clone(),pow(x,3)]);
    let rhs=add(vec![mul(vec![equation,factor]),neg(mul(vec![c(2),pow(y,3)]))]);
    let statement=IdentityStatement{
        name:"Binary curve fourth-power reduction over the integers".into(),
        source:"BKFrobeniusBridgeAudit.Binary.fourth_power_identity. Generic Lean CommRing proof. Scalar supplement does not certify Frobenius, determinant, torsion or descent.".into(),
        variables:vec!["x".into(),"y".into()],degree_bound:6,lhs,rhs,
    };
    let cert=identity::issue(statement,20261114,32,
        "Producer-issued and same-host official checker;32extra domain-separated points. No independent-host or signed verification.",
        Some(ProofRef{kind:"lean4".into(),path:"research/bk_frobenius_bridge_audit_20261114/lean/BinaryFrobenius.lean".into(),sha256:proof_hash})).expect("identity holds");
    let verdict=identity::check_with_extra(&cert,Some((831989,32))).expect("well formed");
    assert!(matches!(verdict,Verdict::Accept{points:32,extra_points:32,..}));
    eprintln!("{verdict:?}");println!("{}",serde_json::to_string(&cert).unwrap());
}
