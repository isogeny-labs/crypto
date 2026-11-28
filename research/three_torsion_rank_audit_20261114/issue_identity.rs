//! Scalar identity supplement using the repository's unchanged official checker.
mod hash {
    #[path = "/workspace/crypto-audit/src/hash/sha256.rs"]
    pub mod sha256;
}
#[path = "/workspace/crypto-audit/src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, ProofRef, Verdict};
fn c(n: i64) -> E { E::Const(n) }
fn v(n: &str) -> E { E::var(n) }
fn sum(xs: Vec<E>) -> E { E::sum(xs) }
fn mul(xs: Vec<E>) -> E { E::product(xs) }
fn pow(x: E, n: usize) -> E { mul(vec![x;n]) }
fn neg(x: E) -> E { E::minus(x) }
fn main() {
    let proof_hash = std::env::args().nth(1).expect("Lean source SHA256 argument");
    let (a1,a2,a3,a4,a6,x,y)=(v("a1"),v("a2"),v("a3"),v("a4"),v("a6"),v("x"),v("y"));
    let b2=sum(vec![pow(a1.clone(),2),mul(vec![c(4),a2.clone()])]);
    let b4=sum(vec![mul(vec![a1.clone(),a3.clone()]),mul(vec![c(2),a4.clone()])]);
    let b6=sum(vec![pow(a3.clone(),2),mul(vec![c(4),a6.clone()])]);
    let b8=sum(vec![mul(vec![pow(a1.clone(),2),a6.clone()]),mul(vec![c(4),a2.clone(),a6.clone()]),neg(mul(vec![a1.clone(),a3.clone(),a4.clone()])),mul(vec![a2.clone(),pow(a3.clone(),2)]),neg(pow(a4.clone(),2))]);
    let d=sum(vec![mul(vec![c(2),y.clone()]),mul(vec![a1.clone(),x.clone()]),a3.clone()]);
    let n=sum(vec![mul(vec![c(3),pow(x.clone(),2)]),mul(vec![c(2),a2.clone(),x.clone()]),a4.clone(),neg(mul(vec![a1.clone(),y.clone()]))]);
    let eq=sum(vec![pow(y.clone(),2),mul(vec![a1.clone(),x.clone(),y.clone()]),mul(vec![a3,y]),neg(pow(x.clone(),3)),neg(mul(vec![a2.clone(),pow(x.clone(),2)])),neg(mul(vec![a4,x.clone()])),neg(a6)]);
    let lhs=sum(vec![mul(vec![c(3),pow(x.clone(),4)]),mul(vec![b2.clone(),pow(x.clone(),3)]),mul(vec![c(3),b4,pow(x.clone(),2)]),mul(vec![c(3),b6,x.clone()]),b8]);
    let rhs=sum(vec![mul(vec![sum(vec![mul(vec![c(3),x.clone()]),a2]),pow(d.clone(),2)]),neg(mul(vec![a1,n.clone(),d])),neg(pow(n,2)),neg(mul(vec![sum(vec![mul(vec![c(12),x]),b2]),eq]))]);
    let statement=IdentityStatement{
        name:"General Weierstrass three-division coordinate identity".into(),
        source:"ThreeTorsionRankAudit.General.psi_coordinate_identity; literal Mathlib Psi3/invariants/equation. Scalar supplement, not geometric point count or basis proof.".into(),
        variables:vec!["a1","a2","a3","a4","a6","x","y"].into_iter().map(String::from).collect(),
        degree_bound:5,lhs,rhs,
    };
    let cert=identity::issue(statement,20261114,32,
        "Producer-issued and same-host official checker,32extra domain-separated points. No independent-host/signed standing. Generic geometry rests on Lean.",
        Some(ProofRef{kind:"lean4".into(),path:"research/three_torsion_rank_audit_20261114/lean/GeneralTriplePoint.lean".into(),sha256:proof_hash})).expect("true identity");
    let verdict=identity::check_with_extra(&cert,Some((831987,32))).expect("well formed");
    assert!(matches!(verdict,Verdict::Accept{points:32,extra_points:32,..}));
    eprintln!("{verdict:?}");println!("{}",serde_json::to_string(&cert).unwrap());
}
