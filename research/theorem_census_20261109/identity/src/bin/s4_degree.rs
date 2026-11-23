// Supplement the Lean leading-coefficient proof; finite checks are not its proof.
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
    let a=E::var("a"); let b=E::var("b"); let cc=E::var("c"); let z=E::var("z");
    let z2=sq(z.clone()); let e=mul(c(-2),z.clone());
    let lhs=sub(sq(sub(mul(a.clone(),z2.clone()),cc.clone())),
        mul(sub(mul(a.clone(),e.clone()),b.clone()),
            sub(mul(b.clone(),z2.clone()),mul(cc.clone(),e))));
    let rhs=sq(add(add(mul(a,z2),mul(b,z)),cc));
    let statement=IdentityStatement {
        name:"S4 leading coefficient perfect-square resultant rewrite".into(),
        source:"S4DegreeAudit.quartic_coeff_four and s4_degree_audit_certificate; res22(a,b,c,1,-2z,z^2)=(az^2+bz+c)^2".into(),
        variables:["a","b","c","z"].iter().map(|s|s.to_string()).collect(),
        degree_bound:6,lhs,rhs,
    };
    let bytes=std::fs::read("../lean/S4Degree.lean").expect("read fixed Lean source");
    let digest=sha256::sha256(&bytes).iter().map(|b|format!("{b:02x}")).collect();
    let cert=issue(statement,0x2026_1109_1710,32,
        "Producer-issued on one host with unchanged repository checker. Lean proves the actual determinant/coefficient/degree statements; this finite rewrite supplement supplies no all-characteristic, signed or independent-host inference.",
        Some(ProofRef {kind:"lean4".into(),
            path:"research/theorem_census_20261109/lean/S4Degree.lean".into(),sha256:digest}))
        .expect("identity must verify before sealing");
    assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e044,11))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string_pretty(&cert).unwrap());
}
