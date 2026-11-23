//! Exact ordered retained evidence encoding; encoding completeness is not solver completeness.
use super::{OwnedAttempt,Budget,capture,equations,continuation,exclusions,scheduler,check,bridge,bqi,checked_model,LiftRecord,Failure as OwnerFailure,Stop,capture_projection};
use std::time::Instant;
#[path="capture_wire.rs"] mod wire;
pub(in crate::ml_dataset::solvers) use wire::{AdmissionSink,Limits,Failure,Attempt};
use wire::Stream;
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
macro_rules! object{($s:ident; $($key:literal=>$value:expr),* $(,)?)=>{{ $s.map([$($key),*].len())?;$( $s.text($key)?;$value?; )* Ok(()) }}}
fn charge<T:AdmissionSink>(s:&mut Stream<'_,T>,v:equations::Charge)->Result<(),Failure>{s.list(2)?;s.number(v.work)?;s.number(v.bytes)}
fn poly<T:AdmissionSink>(s:&mut Stream<'_,T>,p:&equations::Polynomial)->Result<(),Failure>{s.array(p,|s,t|s.array(t,|s,v|s.number(*v as u64)))}
fn polys<T:AdmissionSink>(s:&mut Stream<'_,T>,p:&[equations::Polynomial])->Result<(),Failure>{s.array(p,poly)}
fn point<T:AdmissionSink>(s:&mut Stream<'_,T>,p:&(u64,u64))->Result<(),Failure>{s.list(2)?;s.number(p.0)?;s.number(p.1)}
fn row<T:AdmissionSink>(s:&mut Stream<'_,T>,r:&scheduler::Row)->Result<(),Failure>{object!(s;"polynomial"=>poly(s,&r.polynomial),"proof"=>s.array(&r.proof,|s,p|{s.list(2)?;s.usize(p.generator)?;s.array(&p.multiplier,|s,v|s.number(*v as u64))}))}
fn task<T:AdmissionSink>(s:&mut Stream<'_,T>,t:&scheduler::Task)->Result<(),Failure>{object!(s;"kind"=>s.debug(&t.kind),"degree"=>s.usize(t.degree),"serial"=>s.number(t.serial))}
fn batch<T:AdmissionSink>(s:&mut Stream<'_,T>,b:&scheduler::Batch)->Result<(),Failure>{object!(s;
 "basis_before"=>s.usize(b.basis_before),"degree"=>s.usize(b.degree),"tasks"=>s.array(&b.tasks,task),"initial"=>s.array(&b.initial,row),"matrix"=>s.array(&b.matrix,row),
 "reducers"=>s.array(&b.reducers,|s,r|object!(s;"basis"=>s.usize(r.basis),"multiplier"=>s.array(&r.multiplier,|s,v|s.number(*v as u64)),"row"=>s.usize(r.row))),
 "operations"=>s.array(&b.operations,|s,p|{s.list(2)?;s.usize(p.target)?;s.usize(p.source)}),"stage"=>s.debug(&b.stage),"row_cursor"=>s.usize(b.row_cursor),"term_cursor"=>s.usize(b.term_cursor),"basis_cursor"=>s.usize(b.basis_cursor),"order"=>s.array(&b.order,|s,v|s.usize(*v)),"pivots"=>s.array(&b.pivots,|s,v|s.usize(*v)),"staged_basis"=>s.array(&b.staged_basis,row),"staged_tasks"=>s.array(&b.staged_tasks,task),"additions"=>s.array(&b.additions,|s,p|{s.list(2)?;s.usize(p.0)?;s.usize(p.1)}))}
fn native<T:AdmissionSink>(s:&mut Stream<'_,T>,a:&scheduler::Attempt)->Result<(),Failure>{object!(s;
 "batches"=>s.array(&a.batches,batch),"result"=>s.debug(&a.result),"complete"=>s.boolean(a.complete),"basis"=>s.array(&a.basis,row),"pending"=>s.array(&a.pending,task),"active"=>s.option(a.active.as_ref(),task),"current"=>row(s,&a.current),"charge"=>charge(s,a.charge),"completed"=>s.number(a.completed),"row_xors"=>s.number(a.row_xors),"symbolic_reducers"=>s.number(a.symbolic_reducers),"field_tasks"=>s.number(a.field_tasks),"ordinary_tasks"=>s.number(a.ordinary_tasks),"last_degree"=>s.usize(a.last_degree))}
fn search<T:AdmissionSink>(s:&mut Stream<'_,T>,a:&scheduler::search::SearchAttempt)->Result<(),Failure>{object!(s;
 "result"=>s.debug(&a.result),"verdict"=>s.debug(&a.verdict),"nodes"=>s.array(&a.nodes,|s,n|object!(s;
 "parent"=>s.opt_usize(n.parent),"origins"=>s.array(&n.origins,|s,v|s.debug(v)),"equations"=>polys(s,&n.equations),"decisions"=>s.array(&n.decisions,|s,p|{s.list(2)?;s.number(p.0 as u64)?;s.boolean(p.1)}),"status"=>s.debug(&n.status),"native_disposition"=>s.option(n.native_disposition.as_ref(),|s,v|s.debug(v)),"native"=>s.option(n.native.as_ref(),native),"candidate"=>s.array(&n.candidate,|s,v|s.boolean(*v)),"candidate_valid"=>s.option(n.candidate_valid.as_ref(),|s,v|s.boolean(*v)),"children"=>s.array(&n.children,|s,v|s.opt_usize(*v)),"refutation_row"=>s.opt_usize(n.refutation_row))),
 "queue"=>s.array(&a.queue,|s,v|s.usize(*v)),"active"=>s.opt_usize(a.active),"witness_node"=>s.opt_usize(a.witness_node),"charge"=>charge(s,a.charge))}
fn certificate<T:AdmissionSink>(s:&mut Stream<'_,T>,c:&bridge::Certificate)->Result<(),Failure>{object!(s;
 "raw"=>s.bytes(&c.raw),"charge"=>charge(s,c.charge),"audit"=>s.option(c.audit.as_ref(),|s,a|object!(s;
 "result"=>match &a.result{Ok(d)=>object!(s;"target"=>s.opt_usize(d.target),"max_quotient_summand"=>s.opt_usize(d.max_quotient_summand),"max_before_quotient_summand"=>s.opt_usize(d.max_before_quotient_summand)),Err(e)=>s.debug(e)},"charges"=>s.array(&a.charges.used,|s,v|s.number(*v)),"source_authenticated"=>s.boolean(a.source_authenticated),"native_operation_witness"=>s.boolean(a.native_operation_witness))),"result"=>s.debug(&c.result))}
fn audit<T:AdmissionSink>(s:&mut Stream<'_,T>,a:&check::Audit)->Result<(),Failure>{object!(s;
 "result"=>s.debug(&a.result),"charge"=>charge(s,a.charge),"checked_nodes"=>s.usize(a.checked_nodes),"certificates"=>s.array(&a.certificates,certificate),"source_authenticated"=>s.boolean(a.source_authenticated))}
fn coverage<T:AdmissionSink>(s:&mut Stream<'_,T>,a:&continuation::CoverageAudit)->Result<(),Failure>{object!(s;
 "result"=>s.debug(&a.result),"charge"=>charge(s,a.charge),"partitions"=>s.usize(a.partitions),"exclusions"=>s.usize(a.exclusions),"checks"=>s.array(&a.checks,audit))}
fn continued<T:AdmissionSink>(s:&mut Stream<'_,T>,a:&continuation::Attempt)->Result<(),Failure>{object!(s;
 "partitions"=>s.array(&a.partitions,|s,p|object!(s;"parent"=>s.opt_usize(p.parent),"prefix"=>s.array(&p.prefix,|s,v|s.boolean(*v)),"equations"=>polys(s,&p.equations),"status"=>s.debug(&p.status),"search"=>s.option(p.search.as_ref(),search),"audit"=>s.option(p.audit.as_ref(),audit),"children"=>s.array(&p.children,|s,v|s.usize(*v)),"exclusion_record"=>s.opt_u64(p.exclusion_record))),"queue"=>s.array(&a.queue,|s,v|s.usize(*v)),"active"=>s.opt_usize(a.active),"outcome"=>s.debug(&a.outcome),"failure"=>s.option(a.failure.as_ref(),|s,v|s.debug(v)),"charge"=>charge(s,a.charge))}
fn progress<T:AdmissionSink>(s:&mut Stream<'_,T>,p:&checked_model::Progress)->Result<(),Failure>{object!(s;
 "formula_rows"=>s.usize(p.formula_rows),"blocks"=>s.array(&p.blocks,|s,v|s.number(*v)),"chain_steps"=>s.usize(p.chain_steps),"lift_choices"=>s.array(&p.lift_choices,|s,p|s.array(p,|s,v|s.option(v.as_ref(),point))),"path"=>s.array(&p.path,point),"sum"=>s.option(p.sum.as_ref(),point),"sign_mask"=>s.number(p.sign_mask as u64),"signed_relation"=>s.boolean(p.signed_relation))}
fn lift<T:AdmissionSink>(s:&mut Stream<'_,T>,r:&LiftRecord)->Result<(),Failure>{object!(s;
 "partition"=>s.usize(r.partition),"record"=>s.number(r.record),"model"=>s.array(&r.model,|s,v|s.boolean(*v)),"progress"=>progress(s,&r.progress),"callback_result"=>s.option(r.callback_result.as_ref(),|s,v|s.debug(v)),"returned"=>s.option(r.returned.as_ref(),|s,v|s.debug(v)))}
fn exclusion<T:AdmissionSink>(s:&mut Stream<'_,T>,a:&exclusions::Audit)->Result<(),Failure>{object!(s;
 "result"=>s.debug(&a.result),"charge"=>charge(s,a.charge),"coverage"=>s.option(a.coverage.as_ref(),coverage),"replays"=>s.array(&a.replays,|s,r|object!(s;"record"=>s.usize(r.record),"partition"=>s.usize(r.partition),"progress"=>progress(s,&r.progress),"outcome"=>s.option(r.outcome.as_ref(),|s,v|s.debug(v)))),"paired"=>s.array(&a.paired,|s,v|s.boolean(*v)))}
fn expansion<T:AdmissionSink>(s:&mut Stream<'_,T>,e:&ExpansionView<'_>)->Result<(),Failure>{object!(s;
 "variables"=>s.number(e.variables as u64),"equations"=>polys(s,e.equations),"status"=>s.debug(&e.status),"owner_failure"=>s.option(e.owner_failure.as_ref(),|s,v|s.debug(v)),"charge"=>charge(s,e.charge),"partial"=>poly(s,e.partial),"scratch"=>poly(s,e.scratch),"stage"=>s.debug(&e.stage),"row"=>s.usize(e.row),"literal"=>s.usize(e.literal))}

pub(in crate::ml_dataset::solvers) fn encode<T:AdmissionSink>(source:&EncodingSource<'_>,snapshot_index:u64,limits:Limits,io:&mut T)->Attempt{
 wire::run(limits,source.deadline,io,|s|{let i=&source.identity;object!(s;
 "schema"=>s.text("owned-search-retained-evidence/v1"),"snapshot_index"=>s.number(snapshot_index),"identity"=>object!(s;
 "public_sha256"=>s.bytes(i.public_sha256),"factor_sha256"=>s.bytes(i.factor_sha256),"launch_sha256"=>s.bytes(i.launch_sha256),"binding_sha256"=>s.bytes(i.binding_sha256),"map_bytes"=>s.bytes(i.map_bytes),"map_ack"=>s.option(i.map_ack,|s,a|object!(s;"binding_sha256"=>s.text(&a.binding_sha256),"event_count"=>s.number(a.event_count),"byte_end"=>s.number(a.byte_end),"last_event_sha256"=>s.text(&a.last_event_sha256))),"publication_attempted"=>s.boolean(i.publication_attempted),"map_published"=>s.boolean(i.map_published),"pattern"=>s.opt_u64(i.pattern.map(u64::from))),
 "signed_domain"=>s.array(source.signed,point),"signed_target"=>s.option(source.target.as_ref(),point),"failure"=>s.option(source.failure.as_ref(),|s,v|s.debug(v)),"first_stop"=>s.option(source.stop.as_ref(),|s,v|s.debug(v)),"diagnostic"=>match source.diagnostic{Some(v)=>s.text(v),None=>s.null()},"normalization_cursor"=>{s.list(3)?;s.usize(source.cursor.0)?;s.usize(source.cursor.1)?;s.usize(source.cursor.2)},"expansion"=>s.option(source.expansion.as_ref(),expansion),"normalized"=>polys(s,source.normalized),"continuation"=>s.option(source.continuation,continued),"lift_records"=>s.array(source.lifts,lift),"coverage"=>s.option(source.coverage,coverage),"exclusion_audit"=>s.option(source.exclusions,exclusion),"accepted_record"=>s.opt_usize(source.accepted),"status"=>s.debug(&source.status),"encoding_complete"=>s.boolean(true))})
}
