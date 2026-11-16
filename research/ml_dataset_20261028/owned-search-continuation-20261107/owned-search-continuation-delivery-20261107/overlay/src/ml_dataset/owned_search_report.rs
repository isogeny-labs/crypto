//! Fixed-size, original-budget terminal report. A report is scoped to one
//! pattern and does not promote producer classifications to independent replay.
use super::{OwnedAttempt,Failure,Stop};
use std::time::Instant;
const REPORT_CAP:u64=8192;
fn failure_kind(f:Option<Failure>)->Option<&'static str>{
 f.map(|f|match f{Failure::Admission(_)=>"admission",Failure::Expansion(_)=>"expansion",
 Failure::Search(_)=>"search",Failure::Audit(_)=>"audit",Failure::LiftStopped(_)=>"lift",
 Failure::Continuation(_)=>"continuation",Failure::Coverage(_)=>"coverage",
 Failure::Exclusions(_)=>"exclusions",Failure::Publication(_)=>"publication"})
}
impl OwnedAttempt{
 /// One attempt only. No report is manufactured if the original ledger is
 /// stopped or its clock expired. All custody remains in this owner on refusal.
 pub(in crate::ml_dataset::solvers) fn outcome(&mut self)->Option<serde_json::Value>{
  if self.report_taken{return None}
  self.report_taken=true;
  match self.report_inner(){Ok(v)=>Some(v),Err(s)=>{if self.failure.is_none(){self.fail(Failure::Publication(s),s)}else{let _:Result<(),Stop>=self.original.budget.fail(s);}None}}
 }
 fn report_inner(&mut self)->Result<serde_json::Value,Stop>{
  self.original.budget.check(Instant::now())?;
  if !self.original.map_published||self.original.pattern.is_none(){return Err(Stop::Phase)}
  // The summary refers only to the latest completely acknowledged snapshot.
  // No scan, encoding, retry or new capture sink is hidden in this report path.
  let snapshot=self.publication.snapshots.last().ok_or(Stop::Phase)?;
  if !snapshot.complete||snapshot.failure.is_some()||self.publication.failure.is_some(){return Err(Stop::Observer)}
  let chunk=snapshot.chunks.last().ok_or(Stop::Observer)?;
  let ack=chunk.ack.as_ref().ok_or(Stop::Observer)?;
  if !chunk.exact||ack.last_event_sha256.len()!=64{return Err(Stop::Observer)}
  let quote=super::super::super::quote_parent_storage_length(REPORT_CAP,true,&self.original.capture_limits)?;
  // Fixed object, fixed hashes and scalar fields: reserve encoding and parent
  // report storage before allocating Value nodes or a copied capture head.
  self.original.budget.reserve_checking(quote.work.checked_add(16*REPORT_CAP).ok_or(Stop::Overflow)?,quote.bytes.checked_add(262144).ok_or(Stop::Overflow)?,Instant::now())?;
  self.original.budget.reserve_trace(quote.trace,Instant::now())?;
  if self.sink.committed_event_position()!=*ack{return Err(Stop::Observer)}
  let b=&self.original.budget;
  let work=b.native.work.checked_add(b.checking_work).ok_or(Stop::Overflow)?;
  let bytes=b.native.bytes.checked_add(b.adapter_bytes).ok_or(Stop::Overflow)?;
  let value=serde_json::json!({
   "adapter_id":"original-owned-boolean-search-v1","strategy":"original-owned-boolean-search-v1",
   "schema":"crypto.ml-dataset.owned-search-report/v1","status":"partial",
   "scope":"single-pattern-retained-search","pattern":self.original.pattern,
   "all_patterns_covered":false,"solver_completed":false,"ecdlp_pipeline_completed":false,
   "scalar_recovered_without_witness":false,"full_span_search":false,
   "first_fall_degree":null,"last_fall_degree":null,"degree_of_regularity":null,
   "independent_original_source_replay":false,"s3_custody_verified":false,
   "capture_snapshot_index":snapshot.index,"capture_snapshot_complete":true,
   "capture_chunk_count":snapshot.chunks.len(),"capture_head_event_count":ack.event_count,
   "capture_head_sha256":ack.last_event_sha256,
   "public_sha256_bytes":self.original.public_sha256,"factor_base_sha256_bytes":self.original.factor_sha256,
   "launch_sha256_bytes":self.original.launch_sha256,"binding_sha256_bytes":self.original.binding,
   "retained_snapshot_count":self.publication.snapshots.len(),"retained_lift_record_count":self.lift_records.len(),
   "producer_signed_relation_accepted":self.accepted_relation().is_some(),
   "producer_conditional_coverage_checked":self.conditional_coverage().is_some(),
   "producer_model_exclusions_checked":self.verified_model_exclusions().is_some(),
   "failure_stage":failure_kind(self.failure),
   "accounting_kind":"original-ledger-reservations-not-retired-instructions",
   "reserved_work":work,"reserved_bytes":bytes,"native_reserved_work":b.native.work,
   "checking_reserved_work":b.checking_work,"checking_performed_work":b.checking_performed_work,
   "trace_reserved_bytes":b.trace_bytes,"curve_additions":b.additions,
   "first_stop":null
  });
  // The complete fixed report is paid above. This final check refuses delivery
  // if constructing it crossed the original deadline; it grants no extension.
  self.original.budget.check(Instant::now())?;
  Ok(value)
 }
}
