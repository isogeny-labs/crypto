// Supplement the kernel proof; this finite checker does not replace it.
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
    let a=E::var("a"); let b=E::var("b"); let cc=E::var("c");
    let d=E::var("d"); let e=E::var("e"); let f=E::var("f");
    let lhs=add(sub(add(sub(sub(mul(sq(a.clone()),sq(f.clone())),
        mul(mul(mul(a.clone(),b.clone()),e.clone()),f.clone())),
        mul(c(2),mul(mul(mul(a.clone(),cc.clone()),d.clone()),f.clone()))),
        mul(mul(a.clone(),cc.clone()),sq(e.clone()))),
        mul(mul(mul(b.clone(),cc.clone()),d.clone()),e.clone())),
        add(mul(mul(sq(b.clone()),d.clone()),f.clone()),mul(sq(cc.clone()),sq(d.clone()))));
    let rhs=sub(sq(sub(mul(a.clone(),f.clone()),mul(cc.clone(),d.clone()))),
        mul(sub(mul(a,e.clone()),mul(b.clone(),d)),sub(mul(b,f),mul(cc,e))));
    let statement=IdentityStatement {
        name:"Degree2 Sylvester resultant coefficient identity".into(),
        source:"GeneralS4Audit.quadratic_resultant: actual Mathlib determinant, including degree drops".into(),
        variables:["a","b","c","d","e","f"].iter().map(|s|s.to_string()).collect(),
        degree_bound:4,lhs,rhs,
    };
    let bytes=std::fs::read("../lean/GeneralS4.lean").expect("read fixed Lean source");
    let digest=sha256::sha256(&bytes).iter().map(|b|format!("{b:02x}")).collect();
    let cert=issue(statement,0x2026_1109_1614,32,
        "Producer-issued on one host using unchanged repository checker; universal determinant and group conclusions independently checked by Lean; no signed or separate-host standing.",
        Some(ProofRef {kind:"lean4".into(),
            path:"research/theorem_census_20261109/lean/GeneralS4.lean".into(),sha256:digest}))
        .expect("identity must verify before sealing");
    assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e004,11))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string_pretty(&cert).unwrap());
}
