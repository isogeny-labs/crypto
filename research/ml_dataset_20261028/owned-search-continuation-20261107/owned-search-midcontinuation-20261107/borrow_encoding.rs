use super::*;
#[derive(Clone,Copy)]
struct ExpansionView<'a>{variables:u32,equations:&'a [equations::Polynomial],status:Result<(),equations::Failure>,owner_failure:Option<super::provenance::Error>,charge:equations::Charge,partial:&'a equations::Polynomial,scratch:&'a equations::Polynomial,stage:equations::Stage,row:usize,literal:usize}
pub(in crate::ml_dataset::solvers) struct EncodingSource<'a>{
 identity:capture_projection::SourceIdentity<'a>,signed:&'a [(u64,u64)],target:Option<(u64,u64)>,failure:Option<OwnerFailure>,stop:Option<Stop>,diagnostic:Option<&'a str>,cursor:(usize,usize,usize),expansion:Option<ExpansionView<'a>>,normalized:&'a [equations::Polynomial],continuation:Option<&'a continuation::Attempt>,lifts:&'a [LiftRecord],coverage:Option<&'a continuation::CoverageAudit>,exclusions:Option<&'a exclusions::Audit>,accepted:Option<usize>,status:capture_projection::Status,deadline:Instant,
}
impl EncodingSource<'_>{
 pub fn map_ack(&self)->Option<&capture::CommittedEventPosition>{self.identity.map_ack}
 pub fn pattern(&self)->Option<u32>{self.identity.pattern}
 pub fn deadline(&self)->Instant{self.deadline}
 pub fn binding_sha256(&self)->&[u8;32]{self.identity.binding_sha256}
}
/// Constructs immutable disjoint references before lending the original ledger
/// and capture sink. No external equations/target/identity constructor exists.
pub(in crate::ml_dataset::solvers) fn with_parts<R>(owner:&mut OwnedAttempt,f:impl FnOnce(&EncodingSource<'_>,&mut Budget,&mut capture::CaptureEmitter)->R)->R{
 let status=capture_projection::View::of(owner).status();let o=&mut owner.original;
 let source=EncodingSource{identity:capture_projection::SourceIdentity{public_sha256:&o.public_sha256,factor_sha256:&o.factor_sha256,launch_sha256:&o.launch_sha256,binding_sha256:&o.binding,map_bytes:&o.map_bytes,map_ack:o.map_ack.as_ref(),publication_attempted:o.publication_attempted,map_published:o.map_published,pattern:o.pattern},signed:&o.math.signed,target:o.math.signed_target,failure:owner.failure,stop:o.budget.stopped,diagnostic:owner.diagnostic.as_deref(),cursor:(owner.normalization_generator,owner.normalization_term,owner.normalization_sort),expansion:owner.expansion.as_ref().map(|e|{let(partial,scratch,stage,row,literal)=e.retained();ExpansionView{variables:e.variables(),equations:e.equations(),status:e.status(),owner_failure:e.owner_failure(),charge:e.charged(),partial,scratch,stage,row,literal}}),normalized:&owner.normalized,continuation:owner.continuation.as_ref(),lifts:&owner.lift_records,coverage:owner.coverage.as_ref(),exclusions:owner.exclusion_audit.as_ref(),accepted:owner.accepted_record,status,deadline:owner.original_deadline};
 f(&source,&mut o.budget,&mut owner.sink)
}
/// Only this private child can split the actual owner. The template excludes
/// mutable live records/state; each checkpoint borrows those from the Session.
pub(super) fn with_continuation_parts<R>(owner:&mut OwnedAttempt,f:impl FnOnce(&EncodingSource<'_>,&equations::OwnedExpansion,&mut Budget,&mut Vec<LiftRecord>,&mut super::publication::Publication,&mut capture::CaptureEmitter)->R)->R{
 let o=&mut owner.original;
 let source=EncodingSource{identity:capture_projection::SourceIdentity{public_sha256:&o.public_sha256,factor_sha256:&o.factor_sha256,launch_sha256:&o.launch_sha256,binding_sha256:&o.binding,map_bytes:&o.map_bytes,map_ack:o.map_ack.as_ref(),publication_attempted:o.publication_attempted,map_published:o.map_published,pattern:o.pattern},signed:&o.math.signed,target:o.math.signed_target,failure:owner.failure,stop:o.budget.stopped,diagnostic:owner.diagnostic.as_deref(),cursor:(owner.normalization_generator,owner.normalization_term,owner.normalization_sort),expansion:owner.expansion.as_ref().map(|e|{let(partial,scratch,stage,row,literal)=e.retained();ExpansionView{variables:e.variables(),equations:e.equations(),status:e.status(),owner_failure:e.owner_failure(),charge:e.charged(),partial,scratch,stage,row,literal}}),normalized:&owner.normalized,continuation:None,lifts:&[],coverage:owner.coverage.as_ref(),exclusions:owner.exclusion_audit.as_ref(),accepted:owner.accepted_record,status:capture_projection::Status::Incomplete,deadline:owner.original_deadline};
 f(&source,owner.expansion.as_ref().expect("admitted expansion"),&mut o.budget,&mut owner.lift_records,&mut owner.publication,&mut owner.sink)
}
impl EncodingSource<'_>{
 pub(super) fn normalized(&self)->&[equations::Polynomial]{self.normalized}
 pub(super) fn signed_domain(&self)->&[(u64,u64)]{self.signed}
 pub(super) fn signed_target(&self)->Option<(u64,u64)>{self.target}
 pub(super) fn checkpoint_active(&self,attempt:&continuation::Attempt,lifts:&[LiftRecord],budget:&mut Budget,publication:&mut super::publication::Publication,sink:&mut capture::CaptureEmitter,limits:&capture::CaptureLimits)->Result<(),Stop>{
  let i=&self.identity;
  let source=EncodingSource{identity:capture_projection::SourceIdentity{public_sha256:i.public_sha256,factor_sha256:i.factor_sha256,launch_sha256:i.launch_sha256,binding_sha256:i.binding_sha256,map_bytes:i.map_bytes,map_ack:i.map_ack,publication_attempted:i.publication_attempted,map_published:i.map_published,pattern:i.pattern},signed:self.signed,target:self.target,failure:self.failure,stop:budget.stopped,diagnostic:self.diagnostic,cursor:self.cursor,expansion:self.expansion,normalized:self.normalized,continuation:Some(attempt),lifts,coverage:None,exclusions:None,accepted:None,status:capture_projection::Status::Incomplete,deadline:self.deadline};
  super::publication::checkpoint_parts(&source,budget,sink,publication,limits)
 }
}

#[cfg(test)]pub(super) fn state(s:&EncodingSource)->(usize,usize){(s.continuation.map_or(0,|a|a.rows),s.lifts.len())}
