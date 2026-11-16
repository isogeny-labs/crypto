// Issue the required identity record with the unchanged repository verifier.
// This is a verification supplement to the Lean proof, not a curve census.
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
#[path = "../../../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash { pub use crate::sha256; }
use identity_certificate::{Expr as E,IdentityStatement,ProofRef,Verdict,issue,check_with_extra};
fn c(v:i64)->E { E::Const(v) }
fn add(x:E,y:E)->E { E::plus(x,y) }
fn mul(x:E,y:E)->E { E::times(x,y) }
fn sub(x:E,y:E)->E { add(x,E::minus(y)) }
fn sq(x:E)->E { mul(x.clone(),x) }
fn main() {
    let a1=E::var("a1"); let a2=E::var("a2"); let a3=E::var("a3");
    let a4=E::var("a4"); let a6=E::var("a6");
    let x=E::var("x"); let y=E::var("y"); let z=E::var("z");
    let b2=add(sq(a1.clone()),mul(c(4),a2.clone()));
    let b4=add(mul(c(2),a4.clone()),mul(a1.clone(),a3.clone()));
    let b6=add(sq(a3.clone()),mul(c(4),a6.clone()));
    let b8=sub(add(sub(add(mul(sq(a1.clone()),a6.clone()),
        mul(c(4),mul(a2.clone(),a6))),mul(mul(a1,a3.clone()),a4.clone())),
        mul(a2,sq(a3))),sq(a4));
    let xy=mul(x.clone(),y.clone()); let sumxy=add(x.clone(),y.clone());
    let linear=add(add(add(mul(c(2),mul(xy.clone(),sumxy.clone())),
        mul(b2.clone(),xy.clone())),mul(b4.clone(),sumxy.clone())),b6.clone());
    let lhs=sub(sub(sub(add(sub(mul(sq(sub(x.clone(),y.clone())),sq(z.clone())),
        mul(linear,z.clone())),sq(xy.clone())),mul(b4.clone(),xy.clone())),
        mul(b6.clone(),sumxy.clone())),b8.clone());
    let s1=add(sumxy,z.clone());
    let s2=add(add(xy.clone(),mul(x,z.clone())),mul(y,z.clone()));
    let s3=mul(xy,z);
    let rhs=sub(sub(sub(sq(s2.clone()),mul(add(mul(c(4),s1.clone()),b2),s3)),
        mul(b4,s2)),add(mul(b6,s1),b8));
    let statement=IdentityStatement {
        name:"General Weierstrass S3 quadratic and Vieta forms".into(),
        source:"Kosters-Yeo2015 Section2; GeneralS3Audit.generalS3_vieta, actual b-invariants".into(),
        variables:["a1","a2","a3","a4","a6","x","y","z"].iter().map(|s|s.to_string()).collect(),
        degree_bound:5,lhs,rhs,
    };
    let bytes=std::fs::read("../lean/GeneralS3.lean").expect("read fixed Lean source");
    let digest=sha256::sha256(&bytes).iter().map(|b|format!("{b:02x}")).collect();
    let cert=issue(statement,0x2026_1109_1532,32,
        "Producer-issued and extra-checked by the unchanged repository implementation on one host. Formal proof checked separately; no independent-host or signed standing.",
        Some(ProofRef { kind:"lean4".into(),
            path:"research/theorem_census_20261109/lean/GeneralS3.lean".into(),sha256:digest }))
        .expect("the polynomial identity must verify before sealing");
    assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e002,11))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string_pretty(&cert).unwrap());
}
