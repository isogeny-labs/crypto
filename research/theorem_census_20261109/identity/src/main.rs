// Use the repository issuer/checker unchanged, rather than inventing a format.
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash { pub use crate::sha256; }
use identity_certificate::{Expr as E,IdentityStatement,ProofRef,Verdict,issue,check_with_extra};
fn c(v:i64)->E { E::Const(v) }
fn add(x:E,y:E)->E { E::plus(x,y) }
fn mul(x:E,y:E)->E { E::times(x,y) }
fn sub(x:E,y:E)->E { add(x,E::minus(y)) }
fn sq(x:E)->E { mul(x.clone(),x) }
fn main(){
 let x=E::var("x");let y=E::var("y");let sum=add(x.clone(),y.clone());let xy=mul(x.clone(),y.clone());
 let lhs=add(sub(mul(sq(sub(x,y)),c(4)),mul(c(4),add(mul(sum.clone(),add(xy.clone(),c(1))),c(2)))),
   sub(sq(sub(xy.clone(),c(1))),mul(c(4),sum.clone())));
 let rhs=add(sub(mul(sub(sq(sum.clone()),mul(c(4),xy.clone())),c(4)),
   mul(c(4),add(mul(sum.clone(),add(xy.clone(),c(1))),c(2)))),
   sub(sq(sub(xy,c(1))),mul(c(4),sum)));
 let statement=IdentityStatement{name:"S3(a=b=1,z=2) in elementary symmetric coordinates".into(),
   source:"FGHR2014 symmetric substitution; SkepticalAudit.symmetrization_universal".into(),
   variables:vec!["x".into(),"y".into()],degree_bound:4,lhs,rhs};
 let path="research/theorem_census_20261109/lean/DescentCounterexample.lean";
 let bytes=std::fs::read("../lean/DescentCounterexample.lean").unwrap();
 let digest=sha256::sha256(&bytes).iter().map(|b|format!("{b:02x}")).collect();
 let cert=issue(statement,0x2026_1109,32,
   "Producer-issued and checked by unchanged repository code on one host; no independent-host or signed validation claim.",
   Some(ProofRef{kind:"lean4".into(),path:path.into(),sha256:digest})).unwrap();
 assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e001,11))).unwrap(),Verdict::Accept{..}));
 println!("{}",serde_json::to_string_pretty(&cert).unwrap());
}
