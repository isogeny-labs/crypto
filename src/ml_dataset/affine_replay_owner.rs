//! Caller-owned row replay. Producer authority is never recreated here.
use super::*;
use koblitz_m3::sat_original_owner::affine_central as central;
#[derive(Clone,Copy,PartialEq,Eq)] pub(in crate::ml_dataset) enum AffineReplayVerdict{CheckedRow,Incomplete,Unknown,Rejected}
enum Owner{Row(central::RowOutcome),Failed(central::Failed)}
pub(in crate::ml_dataset) struct AffineReplayCustody{owner:Owner,verdict:AffineReplayVerdict}
impl AffineReplayCustody {
 pub(in crate::ml_dataset) fn verdict(&self)->AffineReplayVerdict{self.verdict}
 pub(in crate::ml_dataset) fn retained_slot_bytes()->usize{std::mem::size_of::<Self>()}
 pub(in crate::ml_dataset) fn charged(&self)->(u64,u64){match &self.owner{
  Owner::Row(central::RowOutcome::Verified(v))=>v.usage(),
  Owner::Row(central::RowOutcome::UnknownOriginalMath(f)|central::RowOutcome::Incomplete(f))=>f.usage(),
  Owner::Failed(f)=>f.frame.usage()}}
 pub(in crate::ml_dataset) fn revalidate(&mut self,dir:&Path,config:&CaseConfig,outcome:&Value,artifacts:&[crate::ml_dataset::types::Artifact])->AffineReplayVerdict{
  if self.verdict!=AffineReplayVerdict::CheckedRow{return self.verdict}
  match &mut self.owner{Owner::Row(central::RowOutcome::Verified(v))=>{if v.revalidate(dir,config,outcome,artifacts).is_err(){self.verdict=AffineReplayVerdict::Rejected}},_=>self.verdict=AffineReplayVerdict::Rejected};self.verdict
 }
 pub(in crate::ml_dataset) fn project_identity(&mut self)->Result<Value,String>{
  if self.verdict!=AffineReplayVerdict::CheckedRow{return Err("affine projection lacks verified retained owner".into())}
  match &mut self.owner{Owner::Row(central::RowOutcome::Verified(v))=>match v.project_identity(){Ok(v)=>Ok(v),Err(_)=>{self.verdict=AffineReplayVerdict::Rejected;Err("affine projection admission refused; owner retained".into())}},_=>Err("affine projection owner mismatch".into())}
 }
}
pub(in crate::ml_dataset) fn needs_affine_row_replay(c:&CaseConfig,o:&Value,a:&[crate::ml_dataset::types::Artifact])->bool{
 (c.channels.as_ref().is_some_and(|c|c.affine_boolean_row.is_some()) && koblitz_m3::sat_original_owner::affine_worker::supported(c))||o["adapter_id"]=="original-affine-unit-row-v1"||o["strategy"]=="original-affine-unit-row-v1"
 ||a.iter().any(|a|a.role=="boolean-original-affine-unit-row"||a.path.contains("boolean-affine-original-pattern-"))
}
pub(in crate::ml_dataset) fn verify_affine_row_cell(dir:&Path,c:&CaseConfig,o:&Value,a:&[crate::ml_dataset::types::Artifact],policy:SatReplayPolicy)->AffineReplayCustody{
 use crate::cryptanalysis::binary_semaev_chain_sat::provenance::boolean_equations::{self as e,affine_domain::matrix_rows as rows};
 let cap=e::Limits{variables:4096,generators:4096,terms:4096,degree:64,work:policy.work.min(4_000_000),bytes:policy.bytes.min(8_000_000)};
 let limits=central::RowLimits{expansion:cap,audit:cap,checker:rows::CheckerLimits{variables:4096,generators:4096,polynomial_terms:4096,proof_terms:4096,accumulator_entries:4096,caps:[65536,262144,262144,262144,262144,65536,1024,1048576,262144]}};
 let result=central::verify_affine_row(dir,c,o,a,central::Policy{work:policy.work,bytes:policy.bytes,deadline:policy.deadline,stream_bytes:policy.stream_bytes,payload_bytes:policy.payload_bytes},limits);
 match result{Ok(v)=>{let verdict=match &v{central::RowOutcome::Verified(_)=>AffineReplayVerdict::CheckedRow,central::RowOutcome::UnknownOriginalMath(_)=>AffineReplayVerdict::Unknown,central::RowOutcome::Incomplete(_)=>AffineReplayVerdict::Incomplete};AffineReplayCustody{owner:Owner::Row(v),verdict}},Err(f)=>AffineReplayCustody{owner:Owner::Failed(f),verdict:AffineReplayVerdict::Rejected}}
}
