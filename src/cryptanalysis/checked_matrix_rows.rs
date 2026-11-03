//! Child of checked_boolean_equations; source authority remains the borrowed CheckedMap.
//! Integration source only: wrapper requires full library compilation and original-owner controls.
use super::{Admission, Error, Expansion, Quote, sparse};
#[path="matrix_rows.rs"] mod rows;
#[path="boolean_identity_bqi.rs"] mod bqi;
pub use rows::{Failure, Limits};
pub use bqi::{Limits as CheckerLimits, Audit};
use std::time::Instant;
struct Bridge<'a,T:Admission>{owner:&'a mut T,last:Option<Error>}
impl<T:Admission> sparse::Owner for Bridge<'_,T>{
 fn reserve(&mut self,q:sparse::Charge)->Result<(),()>{self.owner.reserve(Quote{work:q.work,bytes:q.bytes}).map_err(|e|{self.last=Some(e);})}
 fn check(&mut self)->Result<(),()>{self.owner.check().map_err(|e|{self.last=Some(e);})}
}
/// Private fields bind every row to the actual complete expansion and exact window.
pub struct BoundRows<'a,'w>{source:Expansion<'a>,window:&'w[sparse::Monomial],attempt:Option<rows::Attempt>,failure:Option<Failure>,owner_failure:Option<Error>}
pub fn reduce<'a,'w>(source:Expansion<'a>,window:&'w[sparse::Monomial],limits:Limits,owner:&mut impl Admission)->BoundRows<'a,'w>{
 if source.status().is_err(){return BoundRows{source,window,attempt:None,failure:Some(Failure::Incomplete),owner_failure:None}}
 let mut bridge=Bridge{owner,last:None};
 let attempt=rows::reduce(source.variables(),source.equations(),window,limits,&mut bridge);
 let failure=attempt.result.err();BoundRows{source,window,attempt:Some(attempt),failure,owner_failure:bridge.last}
}
impl BoundRows<'_,'_>{
 pub fn source(&self)->&Expansion<'_>{&self.source}
 pub fn failure(&self)->Option<Failure>{self.failure}
 pub fn owner_failure(&self)->Option<Error>{self.owner_failure}
 pub fn rows(&self)->usize{self.attempt.as_ref().map_or(0,|a|a.basis.len())}
}
/// Full source/row/wire/audit owners survive both serialization and checker refusal.
/// This is not a capture, exporter, native-search or original-ledger authority token.
pub struct Certificate<'a,'w>{rows:BoundRows<'a,'w>,wire:Option<rows::Encoded>,audit:Option<bqi::Audit>,failure:Option<Failure>,owner_failure:Option<Error>}
pub fn certify<'a,'w>(rows:BoundRows<'a,'w>,row:usize,limits:Limits,checker:CheckerLimits,deadline:Instant,owner:&mut impl Admission)->Certificate<'a,'w>{
 let mut out=Certificate{rows,wire:None,audit:None,failure:None,owner_failure:None};
 if let Some(f)=out.rows.failure{out.failure=Some(f);return out}
 let mut bridge=Bridge{owner,last:None};
 let wire=rows::encode_retained(out.rows.source.variables(),out.rows.source.equations(),out.rows.window,out.rows.attempt.as_ref().unwrap(),row,limits,&mut bridge);
 out.failure=wire.result.err();out.wire=Some(wire);
 if out.failure.is_none(){let checked=rows::independent_check_retained(&out.wire.as_ref().unwrap().raw,checker,deadline,&mut bridge);out.failure=checked.failure;out.audit=checked.audit;if out.failure.is_none()&&out.audit.as_ref().is_some_and(|a|a.result.is_err()){out.failure=Some(Failure::Checker)}}
 out.owner_failure=bridge.last;out
}
impl Certificate<'_,'_>{
 pub fn failure(&self)->Option<Failure>{self.failure}
 pub fn rows(&self)->&BoundRows<'_,'_>{&self.rows}
 pub fn raw(&self)->Option<&[u8]>{self.wire.as_ref().map(|w|w.raw.as_slice())}
 pub fn audit(&self)->Option<&bqi::Audit>{self.audit.as_ref()}
 pub fn owner_failure(&self)->Option<Error>{self.owner_failure}
}
#[cfg(test)]
mod integration_tests {
 use super::*;
 use crate::binary_ecc::{F2mElement,IrreduciblePoly};
 use num_bigint::BigUint;
 use super::super::super::{build_original,Original,Limits as MapLimits};
 struct Ledger{work:u64,bytes:u64,deadline:Instant}
 impl Admission for Ledger{
  fn reserve(&mut self,q:Quote)->Result<(),Error>{self.check()?;let w=self.work.checked_add(q.work).ok_or(Error::Admission)?;let b=self.bytes.checked_add(q.bytes).ok_or(Error::Admission)?;if w>2_000_000_000||b>256_000_000{return Err(Error::Admission)}self.work=w;self.bytes=b;Ok(())}
  fn check(&mut self)->Result<(),Error>{if Instant::now()>=self.deadline{Err(Error::Admission)}else{Ok(())}}
 }
 #[test]
 fn genuine_checked_map_matrix_certificate_both_twists_requires_nontrivial_row(){
  for twist in [false,true]{
   let irr=IrreduciblePoly{degree:2,low_terms:vec![0,1]};let a=F2mElement::one(2);let b=F2mElement::from_bit_positions(&[1],2);let target=F2mElement::one(2);let codes=vec![BigUint::from(0u8),BigUint::from(1u8),BigUint::from(3u8)];
   let original=Original{binding:[19;32],a:&a,b:&b,irr:&irr,twist,m:4,target:Some(&target),pattern:0,codes:&codes};
   let deadline=Instant::now()+std::time::Duration::from_secs(30);let mut owner=Ledger{work:0,bytes:0,deadline};
   let map=build_original(&original,MapLimits{variables:10000,codes:16,domain_clauses:10000,work:32_000_000,bytes:16_000_000},&mut owner).unwrap();
   let source=super::super::expand(&map,sparse::Limits{variables:10000,generators:10000,terms:64,degree:64,work:32_000_000,bytes:16_000_000},&mut owner);assert!(source.status().is_ok());
   let window=vec![vec![]];let limits=Limits{variables:10000,generators:10000,window:1,candidates:10000,basis:10000,terms:1024,degree:64,proof:10000,wire:4_000_000,work:100_000_000,bytes:64_000_000};
   let before=owner.work;let rows=reduce(source,&window,limits,&mut owner);assert!(owner.work>before);assert_eq!(rows.failure(),None);
   let target=rows.attempt.as_ref().unwrap().basis.iter().position(|row|row.proof.len()>1&&!row.polynomial.is_empty()).expect("real map must yield nontrivial reduced row");
   let result=certify(rows,target,limits,CheckerLimits::software(),deadline,&mut owner);assert_eq!(result.failure(),None);assert!(result.audit().unwrap().result.is_ok());assert!(std::ptr::eq(result.rows().source().source(),&map));
  }
 }
}
