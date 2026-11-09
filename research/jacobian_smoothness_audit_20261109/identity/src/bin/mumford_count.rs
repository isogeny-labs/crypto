// Only a scalar quotient/remainder supplement; Lean supplies the actual
// F3 model completeness/count/splitting proofs. No curve population is run.
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
    let a=E::var("a");let b=E::var("b");let cc=E::var("c");
    let d=E::var("d");let x=E::var("x");
    let u=add(add(pow(&x,2),mul(a.clone(),x.clone())),b.clone());
    let v=add(mul(cc.clone(),x.clone()),d.clone());
    let qf=add(add(sub(pow(&x,3),mul(a.clone(),pow(&x,2))),
        mul(sub(pow(&a,2),b.clone()),x.clone())),
        add(E::minus(pow(&a,3)),mul(c(2),mul(a.clone(),b.clone()))));
    let q=sub(pow(&cc,2),qf);
    let rx=add(sub(add(sub(add(E::minus(mul(a.clone(),pow(&cc,2))),
        mul(c(2),mul(cc.clone(),d.clone()))),pow(&a,4)),
        mul(c(3),mul(pow(&a,2),b.clone()))),pow(&b,2)),c(1));
    let rc=add(sub(sub(pow(&d,2),mul(b.clone(),pow(&cc,2))),
        mul(pow(&a,3),b.clone())),mul(c(2),mul(a.clone(),pow(&b,2))));
    let statement=IdentityStatement {
        name:"Degree-five Mumford quadratic quotient/remainder rewrite".into(),
        source:"MumfordSmoothnessAudit.quadratic_remainder: scalar residue+u*quotient=v^2-(X^5-X); this finite supplement does not prove the F3 cardinality, splitting classification, Cantor group law or geometric Picard identification.".into(),
        variables:["a","b","c","d","x"].iter().map(|s|s.to_string()).collect(),
        degree_bound:5,
        lhs:add(add(mul(rx,x.clone()),rc),mul(u,q)),
        rhs:sub(pow(&v,2),sub(pow(&x,5),x)),
    };
    let bytes=std::fs::read("../lean/MumfordSmoothness.lean").expect("read frozen Lean source");
    let digest=sha256::sha256(&bytes).iter().map(|b|format!("{b:02x}")).collect();
    let cert=issue(statement,0x2026_1109_1950,32,
        "Producer-issued on one host with unchanged repository checker. Only the scalar quotient/remainder rewrite is sampled; the actual F3 reduced-pair completeness, cardinalities and splitting statements rest on Lean. No signed or independent-host standing; geometric Jacobian and subgroup claims remain separate.",
        Some(ProofRef{kind:"lean4".into(),
            path:"research/jacobian_smoothness_audit_20261109/lean/MumfordSmoothness.lean".into(),sha256:digest}))
        .expect("identity must verify before sealing");
    assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e095,11))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string(&cert).unwrap());
}
