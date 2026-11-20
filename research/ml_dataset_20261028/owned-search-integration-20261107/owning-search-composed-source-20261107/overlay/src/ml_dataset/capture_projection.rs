//! Read-only retained owner projection, not a wire format or custody receipt.
//! Install as a child of original_boolean_search so construction can read its
//! private fields while writers receive no mutation capability or owned clone.
use super::{OwnedAttempt,LiftRecord,Failure,Stop,equations,continuation,exclusions,capture};
#[path="borrowed_page.rs"] mod paging;
pub(in crate::ml_dataset::solvers) use paging::{Page,Error as PageError,page as page_retained};
/// Exact original hashes retained by admission. These are not synthesized from
/// labels, nor a replacement for the CaptureEmitter's full AttemptBinding.
pub(in crate::ml_dataset::solvers) struct SourceIdentity<'a>{
 pub public_sha256:&'a [u8;32],pub factor_sha256:&'a [u8;32],
 pub launch_sha256:&'a [u8;32],pub binding_sha256:&'a [u8;32],
 pub map_bytes:&'a [u8],pub map_ack:Option<&'a capture::CommittedEventPosition>,
 pub publication_attempted:bool,pub map_published:bool,pub pattern:Option<u32>,
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub(in crate::ml_dataset::solvers) enum Status{
 Incomplete,
 AcceptedSignedRelation,
 ConditionalBooleanCoverage,
 /// Authentic owner's replay gate succeeded. Does not claim all patterns,
 /// generator keys, or curves exhausted, and is not a persisted receipt.
 RetainedModelExclusionsReplayed,
}
/// Fixed-size immutable owner borrow. Every page borrows existing storage;
/// no vector, serialization, payload hash, allocation or scan is performed here.
pub(in crate::ml_dataset::solvers) struct View<'a>{owner:&'a OwnedAttempt}
impl<'a> View<'a>{
 pub fn of(owner:&'a OwnedAttempt)->Self{Self{owner}}
 pub fn identity(&self)->SourceIdentity<'a>{let o=&self.owner.original;SourceIdentity{
  public_sha256:&o.public_sha256,factor_sha256:&o.factor_sha256,launch_sha256:&o.launch_sha256,binding_sha256:&o.binding,
  map_bytes:&o.map_bytes,map_ack:o.map_ack.as_ref(),publication_attempted:o.publication_attempted,map_published:o.map_published,pattern:o.pattern,
 }}
 /// Signed domain and target are borrowed from authenticated original Math;
 /// no caller-selected substitute can enter this projection.
 pub fn signed_domain(&self,start:usize,limit:usize)->Result<Page<'a,(u64,u64)>,PageError>{paging::page(&self.owner.original.math.signed,start,limit)}
 pub fn signed_target(&self)->Option<(u64,u64)>{self.owner.original.math.signed_target}
 pub fn failure(&self)->Option<Failure>{self.owner.failure()}
 pub fn first_stop(&self)->Option<Stop>{self.owner.first_stop()}
 pub fn status(&self)->Status{
  // A retained successful sub-audit cannot mask a later owner refusal.
  if self.failure().is_some()||self.first_stop().is_some(){return Status::Incomplete}
  if self.owner.accepted_relation().is_some(){return Status::AcceptedSignedRelation}
  if self.owner.verified_model_exclusions().is_some(){return Status::RetainedModelExclusionsReplayed}
  if self.owner.conditional_coverage().is_some(){return Status::ConditionalBooleanCoverage}
  Status::Incomplete
 }
 /// Raw failed expansion and normalization cursors stay inspectable.
 pub fn expansion(&self)->Option<&'a equations::OwnedExpansion>{self.owner.expansion.as_ref()}
 pub fn normalization_cursor(&self)->(usize,usize,usize){(self.owner.normalization_generator,self.owner.normalization_term,self.owner.normalization_sort)}
 pub fn normalized(&self,start:usize,limit:usize)->Result<Page<'a,equations::Polynomial>,PageError>{paging::page(&self.owner.normalized,start,limit)}
 /// Retains nested search basis/proof objects in each borrowed partition.
 pub fn partitions(&self,start:usize,limit:usize)->Result<Page<'a,continuation::Partition>,PageError>{paging::page(self.owner.continuation.as_ref().map_or(&[],|x|x.partitions.as_slice()),start,limit)}
 /// None differs from Some(empty): search was not entered versus no partitions.
 pub fn continuation(&self)->Option<&'a continuation::Attempt>{self.owner.continuation.as_ref()}
 pub fn lifts(&self,start:usize,limit:usize)->Result<Page<'a,LiftRecord>,PageError>{paging::page(&self.owner.lift_records,start,limit)}
 pub fn coverage(&self)->Option<&'a continuation::CoverageAudit>{self.owner.coverage.as_ref()}
 pub fn exclusion_audit(&self)->Option<&'a exclusions::Audit>{self.owner.exclusion_audit()}
 pub fn replays(&self,start:usize,limit:usize)->Result<Page<'a,exclusions::Replay>,PageError>{paging::page(self.owner.exclusion_audit().map_or(&[],|x|x.replays.as_slice()),start,limit)}
}
