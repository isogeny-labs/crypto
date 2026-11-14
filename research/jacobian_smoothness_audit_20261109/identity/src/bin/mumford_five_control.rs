// Scalar quadratic factorization only; complete F5 counts rest on Lean.
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
#[path = "../../../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash { pub use crate::sha256; }
use identity_certificate::{Expr as E,IdentityStatement,ProofRef,Verdict,issue,check_with_extra};
fn add(a:E,b:E)->E { E::plus(a,b) }
fn mul(a:E,b:E)->E { E::times(a,b) }
fn sub(a:E,b:E)->E { add(a,E::minus(b)) }
fn main() {
    let x=E::var("x");let a=E::var("a");let r=E::var("r");
    let lhs=mul(sub(x.clone(),r.clone()),add(add(x.clone(),a.clone()),r.clone()));
    let rhs=sub(sub(add(mul(x.clone(),x),mul(a.clone(),E::var("x"))),mul(r.clone(),r.clone())),mul(a,r));
    let statement=IdentityStatement {
        name:"Quadratic factorization from a supplied root".into(),
        source:"MumfordFiveControlAudit.quadratic_splits_of_root uses b=-r^2-a*r and (x-r)(x+a+r)=x^2+a*x-r^2-a*r. This scalar supplement does not prove the F5 complete-pair cardinality16, all16splitting, separability or geometry.".into(),
        variables:["x","a","r"].iter().map(|s|s.to_string()).collect(),degree_bound:2,lhs,rhs,
    };
    let bytes=std::fs::read("../lean/MumfordFiveControl.lean").expect("read frozen Lean source");
    let digest=sha256::sha256(&bytes).iter().map(|b|format!("{b:02x}")).collect();
    let cert=issue(statement,0x2026_1109_2120,32,
        "Producer-issued scalar factorization supplement only; the complete F5model/count/splitting proof rests on Lean. No independent-host or signed standing, no geometric identification.",
        Some(ProofRef{kind:"lean4".into(),
            path:"research/jacobian_smoothness_audit_20261109/lean/MumfordFiveControl.lean".into(),sha256:digest}))
        .expect("scalar identity must verify before sealing");
    assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e097,11))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string(&cert).unwrap());
}
