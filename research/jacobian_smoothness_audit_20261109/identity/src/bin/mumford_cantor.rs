// Scalar supplement only. The complete F3 Cantor law and subgroup
// cardinalities, if proved, rest on the Lean module, not these samples.
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
#[path = "../../../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash { pub use crate::sha256; }
use identity_certificate::{Expr as E,IdentityStatement,ProofRef,Verdict,issue,check_with_extra};
fn c(n:i64)->E { E::Const(n) }
fn add(a:E,b:E)->E { E::plus(a,b) }
fn mul(a:E,b:E)->E { E::times(a,b) }
fn sub(a:E,b:E)->E { add(a,E::minus(b)) }
fn pow(a:&E,n:usize)->E { E::product((0..n).map(|_|a.clone()).collect()) }
fn main() {
    let d=E::var("d");let a=E::var("a");let b=E::var("b");
    let x=E::var("x");let t=E::var("t");
    let factors=E::product(vec![x.clone(),sub(x.clone(),c(1)),
        add(x.clone(),c(1)),add(pow(&x,2),c(1))]);
    let statement=IdentityStatement {
        name:"Zero-v Cantor composition cancellation and quintic factorization".into(),
        source:"Scalar d^2*(a*b)=(d*a)*(d*b) and x*(x-1)*(x+1)*(x^2+1)=x^5-x, combined with an independent variable t. Does not prove gcd selection, all64 reductions, group structure, subgroup counts or Picard identification.".into(),
        variables:["d","a","b","x","t"].iter().map(|s|s.to_string()).collect(),
        degree_bound:6,
        lhs:add(mul(pow(&d,2),mul(a.clone(),b.clone())),mul(t.clone(),factors)),
        rhs:add(mul(mul(d.clone(),a),mul(d,b)),mul(t,sub(pow(&x,5),x))),
    };
    let bytes=std::fs::read("../lean/MumfordCantor.lean").expect("read frozen Lean source");
    let digest=sha256::sha256(&bytes).iter().map(|b|format!("{b:02x}")).collect();
    let cert=issue(statement,0x2026_1109_2040,32,
        "Producer-issued on one host with unchanged repository checker. Only scalar cancellation/factorization identities are sampled. Actual F3 gcd, all-pair reduction, group and subgroup statements require the cited Lean proofs; geometric Picard identification remains separate. No signed or independent-host standing.",
        Some(ProofRef{kind:"lean4".into(),
            path:"research/jacobian_smoothness_audit_20261109/lean/MumfordCantor.lean".into(),sha256:digest}))
        .expect("identity must verify before sealing");
    assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e096,11))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string(&cert).unwrap());
}
