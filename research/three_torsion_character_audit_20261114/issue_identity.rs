//! Scalar supplement using the unchanged repository checker.
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
fn pow(x: E,n: usize) -> E { mul(vec![x;n]) }
fn neg(x: E) -> E { E::minus(x) }
fn main() {
    let proof_hash=std::env::args().nth(1).expect("Lean source SHA256");
    let (a,b,x,y)=(v("a"),v("b"),v("x"),v("y"));
    let d=add(vec![mul(vec![c(2),y.clone()]),mul(vec![a.clone(),x.clone()]),b.clone()]);
    let t=add(vec![y.clone(),mul(vec![a.clone(),x.clone()]),mul(vec![c(2),b.clone()])]);
    let lhs=add(vec![pow(d.clone(),2),neg(mul(vec![d,t.clone()])),pow(t,2)]);
    let cubic=add(vec![mul(vec![c(3),pow(x.clone(),3)]),mul(vec![pow(a.clone(),2),pow(x.clone(),2)]),mul(vec![c(3),a.clone(),b.clone(),x.clone()]),mul(vec![c(3),pow(b.clone(),2)])]);
    let equation=add(vec![pow(y.clone(),2),mul(vec![a,x.clone(),y.clone()]),mul(vec![b,y]),neg(pow(x,3))]);
    let rhs=add(vec![cubic,mul(vec![c(3),equation])]);
    let statement=IdentityStatement{
        name:"Order-three normal-form quadratic character identity".into(),
        source:"ThreeTorsionCharacterAudit.quadratic_identity. Universal Field Lean identity. This scalar supplement does not certify actual character, group law, determinant or field descent.".into(),
        variables:vec!["a","b","x","y"].into_iter().map(String::from).collect(),degree_bound:4,lhs,rhs,
    };
    let cert=identity::issue(statement,20261114,32,
        "Producer-issued and same-host official checker,32extra domain-separated points. No independent-host or signed verification.",
        Some(ProofRef{kind:"lean4".into(),path:"research/three_torsion_character_audit_20261114/lean/ThreeCharacterScalar.lean".into(),sha256:proof_hash})).expect("identity holds");
    let verdict=identity::check_with_extra(&cert,Some((831991,32))).expect("well formed");
    assert!(matches!(verdict,Verdict::Accept{points:32,extra_points:32,..}));
    eprintln!("{verdict:?}");println!("{}",serde_json::to_string(&cert).unwrap());
}
