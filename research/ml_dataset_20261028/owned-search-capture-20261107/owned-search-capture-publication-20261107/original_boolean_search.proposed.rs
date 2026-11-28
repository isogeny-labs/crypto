//! Consuming transport proposal: actual admitted original parts, no borrowed token.
use super::*;
use provenance::boolean_equations as equations;
use provenance::{checked_model, signed_lift, Admission, CheckedMap, Error, Quote};
use std::mem::size_of;
#[path="owned_search_capture_publication.rs"] pub(in crate::ml_dataset::solvers) mod publication;
#[path="capture_projection.rs"] pub(in crate::ml_dataset::solvers) mod capture_projection;
#[path="original_boolean_search_policy.rs"] mod policy;
pub(in crate::ml_dataset::solvers) use policy::ProposedPolicy;
mod sparse {
    pub use super::equations::{Charge, Monomial, Polynomial};
    pub trait Owner {fn reserve(&mut self,q:Charge)->Result<(),()>;fn check(&mut self)->Result<(),()>;}
}
#[path="dynamic_boolean_search_scheduler.rs"] mod scheduler;
#[path="../cryptanalysis/boolean_identity_bqi.rs"] mod bqi;
#[path="dynamic_boolean_search_bqi.rs"] mod bridge;
#[path="dynamic_boolean_search_check.rs"] mod check;
#[path="dynamic_boolean_continuation.rs"] pub(in crate::ml_dataset::solvers) mod continuation;
#[path="retained_exclusion_verifier.rs"] pub(in crate::ml_dataset::solvers) mod exclusions;
pub(in crate::ml_dataset::solvers) use continuation::Limits as ContinuationLimits;
pub(in crate::ml_dataset::solvers) use scheduler::search::SearchLimits;
pub(in crate::ml_dataset::solvers) use check::Limits as AuditLimits;
// Charge incremental algorithm quotes against the ORIGINAL cumulative phase
// counter. observe also enforces global headroom and the phase's absolute clock.
// This is reservation accounting, not a retired-instruction measurement.
fn reserve_phase(budget:&mut Budget,work:u64,bytes:u64)->Result<(),Stop>{
    budget.check(Instant::now())?;
    let Some(work)=budget.phase_native.work.checked_add(work)else{return budget.fail(Stop::Overflow)};
    let Some(bytes)=budget.phase_native.bytes.checked_add(bytes)else{return budget.fail(Stop::Overflow)};
    budget.observe(super::super::Reservation{work,bytes},Instant::now())
}
struct MatrixAdmission<'a>(&'a mut Budget);
impl provenance::Admission for MatrixAdmission<'_>{
    fn reserve(&mut self,q:provenance::Quote)->Result<(),provenance::Error>{reserve_phase(self.0,q.work,q.bytes).map_err(|_|provenance::Error::Admission)}
    fn check(&mut self)->Result<(),provenance::Error>{self.0.check(Instant::now()).map_err(|_|provenance::Error::Admission)}
}
impl sparse::Owner for MatrixAdmission<'_>{
    fn reserve(&mut self,q:sparse::Charge)->Result<(),()>{reserve_phase(self.0,q.work,q.bytes).map_err(|_|())}
    fn check(&mut self)->Result<(),()>{self.0.check(Instant::now()).map_err(|_|())}
}
impl checked_model::VerificationAdmission for MatrixAdmission<'_>{
    fn addition(&mut self)->Result<(),provenance::Error>{self.0.reserve_matrix_candidate_addition(Instant::now()).map_err(|_|provenance::Error::Admission)}
}
struct LiftAdmission<'a>(&'a mut Budget);
impl provenance::Admission for LiftAdmission<'_>{
    fn reserve(&mut self,q:provenance::Quote)->Result<(),provenance::Error>{self.0.reserve_checking(q.work,q.bytes,Instant::now()).map_err(|_|provenance::Error::Admission)}
    fn check(&mut self)->Result<(),provenance::Error>{self.0.check(Instant::now()).map_err(|_|provenance::Error::Admission)}
}
impl checked_model::VerificationAdmission for LiftAdmission<'_>{
    fn addition(&mut self)->Result<(),provenance::Error>{self.0.reserve_matrix_candidate_addition(Instant::now()).map_err(|_|provenance::Error::Admission)}
}
#[derive(Clone,Copy)]
pub(in crate::ml_dataset::solvers) struct Caps{pub expansion:equations::Limits,pub continuation:ContinuationLimits}
#[derive(Clone,Copy,Debug)]
pub(in crate::ml_dataset::solvers) enum Failure{Admission(Stop),Expansion(equations::Failure),Search(scheduler::Failure),Audit(check::Failure),LiftStopped(provenance::Error),Continuation(continuation::Failure),Coverage(continuation::Failure),Exclusions(exclusions::Failure),Publication(Stop)}
/// Each record belongs to one exact retained partition and model. Returned
/// callback values are diagnostics until callback_result is successful.
pub(in crate::ml_dataset::solvers) struct LiftRecord {
    pub partition:usize,pub record:u64,pub model:Vec<bool>,
    pub progress:checked_model::Progress,
    pub callback_result:Option<Result<(),continuation::Failure>>,
    pub returned:Option<Result<signed_lift::Outcome,Stop>>,
}
struct RetainedRecords<'a>(&'a [LiftRecord]);
impl exclusions::Records for RetainedRecords<'_>{
    fn len(&self)->usize{self.0.len()}
    fn get(&self,index:usize)->Option<exclusions::RecordView<'_>>{
        let r=self.0.get(index)?;
        Some(exclusions::RecordView{partition:r.partition,record:r.record,model:&r.model,
            callback_result:r.callback_result,returned_outcome:r.returned.and_then(Result::ok)})
    }
}
struct OriginalParts{
    public_sha256:[u8;32],factor_sha256:[u8;32],budget:Budget,binding:[u8;32],launch_sha256:[u8;32],math:Math,
    checked:Option<provenance::CheckedMap>,unchecked:Option<provenance::Map>,pattern:Option<u32>,
    map_bytes:Vec<u8>,map_ack:Option<capture::CommittedEventPosition>,publication_attempted:bool,map_published:bool,
    capture_limits:capture::CaptureLimits,boolean_attempted:bool,
}
/// Contains no lifetime parameters, borrowed maps or borrowed execution tokens.
/// Actual sink, original ledger, full expansion and search/audit survive errors.
pub(in crate::ml_dataset::solvers) struct OwnedAttempt{
    original:OriginalParts,sink:capture::CaptureEmitter,diagnostic:Option<String>,
    expansion:Option<equations::OwnedExpansion>,normalized:Vec<equations::Polynomial>,
    normalization_generator:usize,normalization_term:usize,normalization_sort:usize,
    continuation:Option<continuation::Attempt>,coverage:Option<continuation::CoverageAudit>,
    exclusion_audit:Option<exclusions::Audit>,
    publication:publication::Publication,
    original_deadline:Instant,failure:Option<Failure>,
    lift_records:Vec<LiftRecord>,accepted_record:Option<usize>,
}
impl OwnedAttempt{
    pub(in crate::ml_dataset::solvers) fn failure(&self)->Option<Failure>{self.failure}
    pub(in crate::ml_dataset::solvers) fn first_stop(&self)->Option<Stop>{self.original.budget.stopped}
    pub(in crate::ml_dataset::solvers) fn charged(&self)->(u64,u64){(self.original.budget.native.work+self.original.budget.checking_work,self.original.budget.native.bytes+self.original.budget.adapter_bytes)}
    pub(in crate::ml_dataset::solvers) fn lift_records(&self)->&[LiftRecord]{&self.lift_records}
    pub(in crate::ml_dataset::solvers) fn accepted_relation(&self)->Option<&LiftRecord>{
        if self.failure.is_some(){return None}
        let r=self.lift_records.get(self.accepted_record?)?;
        if r.callback_result!=Some(Ok(()))||!matches!(r.returned,Some(Ok(signed_lift::Outcome::VerifiedSignedRelation))){return None}Some(r)
    }
    /// Conditional Boolean coverage only; exclusions still require independent
    /// authentic classified replay before any pattern-negative conclusion.
    pub(in crate::ml_dataset::solvers) fn conditional_coverage(&self)->Option<&continuation::CoverageAudit>{
        if self.failure.is_some(){return None}self.coverage.as_ref().filter(|v|v.result.is_ok())
    }
    /// Raw replay diagnostics survive failed admission or arithmetic replay.
    pub(in crate::ml_dataset::solvers) fn exclusion_audit(&self)->Option<&exclusions::Audit>{self.exclusion_audit.as_ref()}
    /// Complete Boolean coverage with fresh per-model exhaustion for this exact
    /// authenticated expansion and original signed domain, not all patterns.
    pub(in crate::ml_dataset::solvers) fn verified_model_exclusions(&self)->Option<&exclusions::Audit>{
        if self.failure.is_some(){return None}self.exclusion_audit.as_ref().filter(|v|v.result.is_ok())
    }
    fn fail(&mut self,f:Failure,s:Stop){self.failure=Some(f);let _:Result<(),Stop>=self.original.budget.fail(s);}
    fn pay(&mut self,w:u64,b:usize)->Result<(),Stop>{reserve_phase(&mut self.original.budget,w,u64::try_from(b).map_err(|_|Stop::Overflow)?)}
    fn normalize(&mut self)->Result<(),Stop>{
        let count=self.expansion.as_ref().ok_or(Stop::Phase)?.equations().len();
        self.pay(1,count.checked_mul(size_of::<equations::Polynomial>()).and_then(|n|n.checked_add(64)).ok_or(Stop::Overflow)?)?;
        self.normalized.try_reserve_exact(count).map_err(|_|Stop::Allocation)?;
        while self.normalization_generator<count {
            let g=self.normalization_generator;
            let terms=self.expansion.as_ref().ok_or(Stop::Phase)?.equations()[g].len();
            self.pay(1,terms.checked_mul(size_of::<equations::Monomial>()).and_then(|n|n.checked_add(64)).ok_or(Stop::Overflow)?)?;
            // Store every growing allocation before the next fallible operation.
            self.normalized.push(Vec::new());
            self.normalized[g].try_reserve_exact(terms).map_err(|_|Stop::Allocation)?;
            self.normalization_term=0;
            while self.normalization_term<terms {
                let t=self.normalization_term;
                let len=self.expansion.as_ref().ok_or(Stop::Phase)?.equations()[g][t].len();
                self.pay(u64::try_from(len).map_err(|_|Stop::Overflow)?.checked_add(1).ok_or(Stop::Overflow)?,len.checked_mul(size_of::<u32>()).and_then(|n|n.checked_add(64)).ok_or(Stop::Overflow)?)?;
                self.normalized[g].push(Vec::new());
                self.normalized[g][t].try_reserve_exact(len).map_err(|_|Stop::Allocation)?;
                self.normalized[g][t].extend_from_slice(&self.expansion.as_ref().ok_or(Stop::Phase)?.equations()[g][t]);
                self.normalization_term+=1;
            }
            // Expansion uses lex vectors; scheduler requires degree then lex.
            // Insertion swaps allocate nothing; all comparisons/swaps are prepaid.
            self.normalization_sort=1;
            while self.normalization_sort<terms {
                let mut j=self.normalization_sort;
                while j>0 {
                    let lens=self.normalized[g][j].len().checked_add(self.normalized[g][j-1].len()).ok_or(Stop::Overflow)?;
                    self.pay(u64::try_from(lens).map_err(|_|Stop::Overflow)?.checked_add(2).ok_or(Stop::Overflow)?,0)?;
                    let a=&self.normalized[g][j-1];let b=&self.normalized[g][j];
                    if a.len().cmp(&b.len()).then_with(||a.cmp(b))!=std::cmp::Ordering::Greater {break}
                    self.normalized[g].swap(j-1,j);j-=1;
                }
                self.normalization_sort+=1;
            }
            self.normalization_generator+=1;
        }
        self.original.budget.check(Instant::now())
    }
    fn continue_models(&mut self,limits:ContinuationLimits){
        // Capacity is bounded and prepaid once before the session borrows the
        // actual original ledger. Every growing record stays in this owner.
        let bytes=match limits.partitions.checked_mul(size_of::<LiftRecord>()).and_then(|n|n.checked_add(64)){
            Some(v)=>v,None=>{self.fail(Failure::Admission(Stop::Overflow),Stop::Overflow);return}
        };
        if let Err(e)=self.pay(1,bytes){self.fail(Failure::Admission(e),e);return}
        if self.lift_records.try_reserve_exact(limits.partitions).is_err(){self.fail(Failure::Admission(Stop::Allocation),Stop::Allocation);return}
        let Some(expansion)=self.expansion.as_ref()else{self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return};
        let variables=expansion.variables();let map=expansion.source();
        let signed=&self.original.math.signed;let target=self.original.math.signed_target;
        let records=&mut self.lift_records;
        let mut owner=MatrixAdmission(&mut self.original.budget);
        let mut session=continuation::Session::new(variables,&self.normalized,limits,self.original_deadline,&mut owner);
        let mut terminal_error=None;let mut accepted=None;let mut unclassified=false;
        loop {
            match session.next(){
                Err(e)=>{terminal_error=Some(Failure::Continuation(e));break},
                Ok(continuation::Outcome::RemainingBooleanModelsExhaustedUnderRecordedExclusions)=>break,
                Ok(continuation::Outcome::CandidateRetained)=>{},
                _=>{terminal_error=Some(Failure::Admission(Stop::Phase));break},
            }
            let Some(partition)=session.attempt.active else{terminal_error=Some(Failure::Admission(Stop::Phase));break};
            if records.len()>=limits.partitions{terminal_error=Some(Failure::Admission(Stop::Phase));break}
            let index=records.len();let id=match u64::try_from(index){Ok(v)=>v,Err(_)=>{terminal_error=Some(Failure::Admission(Stop::Overflow));break}};
            // Capacity above guarantees this push cannot allocate. The exact
            // model buffer and verifier progress are retained before fallibility.
            records.push(LiftRecord{partition,record:id,model:Vec::new(),progress:checked_model::Progress::default(),callback_result:None,returned:None});
            let record=&mut records[index];
            let call=session.with_candidate_owner(|model,deadline,owner|->Result<signed_lift::Outcome,Stop>{
                let bytes=model.len().checked_mul(size_of::<bool>()).and_then(|n|n.checked_add(64)).ok_or(Stop::Overflow)?;
                reserve_phase(owner.0,u64::try_from(model.len()).map_err(|_|Stop::Overflow)?.checked_add(1).ok_or(Stop::Overflow)?,u64::try_from(bytes).map_err(|_|Stop::Overflow)?)?;
                record.model.try_reserve_exact(model.len()).map_err(|_|Stop::Allocation)?;
                record.model.extend_from_slice(model);
                Ok(signed_lift::verify_candidate(map,&record.model,signed,target,&mut record.progress,deadline,&mut LiftAdmission(&mut *owner.0)))
            });
            // No intervening fallible action: even a failed postcheck returns
            // owned data that must survive in this authentic owning attempt.
            record.returned=call.value;record.callback_result=Some(call.result);
            if let Err(e)=call.result{terminal_error=Some(Failure::Continuation(e));break}
            match record.returned{
                Some(Ok(signed_lift::Outcome::VerifiedSignedRelation))=>{accepted=Some(index);break},
                Some(Ok(signed_lift::Outcome::ExhaustedForModel))=>{
                    // The model was copied from this exact callback, no next()
                    // or other candidate mutation occurred before exclusion.
                    if let Err(e)=session.exclude(id){terminal_error=Some(Failure::Continuation(e));break}
                },
                Some(Ok(signed_lift::Outcome::RejectedUnclassified(_)))=>{unclassified=true;break},
                Some(Ok(signed_lift::Outcome::Deadline))=>{terminal_error=Some(Failure::Admission(Stop::Deadline));break},
                Some(Ok(signed_lift::Outcome::Stopped(e)))=>{terminal_error=Some(Failure::LiftStopped(e));break},
                Some(Err(e))=>{terminal_error=Some(Failure::Admission(e));break},
                None=>{terminal_error=Some(Failure::Admission(Stop::Phase));break},
            }
        }
        self.continuation=Some(session.into_attempt());
        self.accepted_record=accepted;
        if let Some(e)=terminal_error{
            let stop=match e{Failure::Admission(s)=>s,Failure::Continuation(continuation::Failure::Deadline)=>Stop::Deadline,_=>Stop::NativeFailure};
            self.fail(e,stop);return
        }
        if accepted.is_some(){
            if let Err(e)=self.original.budget.finish_native(Instant::now()){self.fail(Failure::Admission(e),e);return}
            if let Err(e)=self.original.budget.begin_extraction(true,Instant::now()){self.fail(Failure::Admission(e),e)}
            return
        }
        if unclassified{return} // Retain Matrix and candidate; never exclude.
        let audit=continuation::audit(variables,&self.normalized,self.continuation.as_ref().expect("retained continuation"),limits,self.original_deadline,&mut MatrixAdmission(&mut self.original.budget));
        let result=audit.result;self.coverage=Some(audit);
        if let Err(e)=result{let stop=if e==continuation::Failure::Deadline{Stop::Deadline}else{Stop::NativeFailure};self.fail(Failure::Coverage(e),stop);return}
        // Both equation order and checked map originate in this retained owned
        // expansion; no caller-supplied formula/map/domain enters this boundary.
        let expansion=self.expansion.as_ref().expect("retained authentic expansion");
        let replay=exclusions::verify(expansion.variables(),&self.normalized,expansion.source(),
            &self.original.math.signed,self.original.math.signed_target,
            self.continuation.as_ref().expect("retained continuation"),&RetainedRecords(&self.lift_records),
            exclusions::Limits{records:limits.partitions,work:limits.work,bytes:limits.bytes,coverage:limits},
            self.original_deadline,&mut MatrixAdmission(&mut self.original.budget));
        let result=replay.result;self.exclusion_audit=Some(replay);
        if let Err(e)=result{
            let stop=if e==exclusions::Failure::Deadline{Stop::Deadline}else{Stop::NativeFailure};
            self.fail(Failure::Exclusions(e),stop);return
        }
        // Stay Matrix. This proves only this source model partition's exhaustion;
        // exceptional-pattern completeness and public negative transport are separate.
    }
    fn run(&mut self,mut caps:Caps){
        let phase=match self.original.budget.begin_native(false,Instant::now()){Ok(v)=>v,Err(s)=>{self.fail(Failure::Admission(s),s);return}};
        caps.expansion.work=caps.expansion.work.min(phase.work);caps.expansion.bytes=caps.expansion.bytes.min(phase.bytes);
        let Some(map)=self.original.checked.take()else{self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return};
        // Existing source-authoritative owning API; raw maps cannot enter.
        let expanded=equations::expand_owned(map,caps.expansion,&mut MatrixAdmission(&mut self.original.budget));
        let status=expanded.status();self.expansion=Some(expanded);
        if let Err(f)=status{self.fail(Failure::Expansion(f),Stop::NativeFailure);return}
        if let Err(s)=self.normalize(){self.fail(Failure::Admission(s),s);return}
        // Persist authenticated equations before Matrix work can exhaust supply.
        if self.checkpoint_capture().is_err(){return}
        if let Err(s)=self.original.budget.finish_native(Instant::now()){self.fail(Failure::Admission(s),s);return}
        let phase=match self.original.budget.begin_native(true,Instant::now()){Ok(v)=>v,Err(s)=>{self.fail(Failure::Admission(s),s);return}};
        caps.continuation.work=caps.continuation.work.min(phase.work);caps.continuation.bytes=caps.continuation.bytes.min(phase.bytes);
        caps.continuation.search.native.work=caps.continuation.search.native.work.min(phase.work);caps.continuation.search.native.bytes=caps.continuation.search.native.bytes.min(phase.bytes);
        self.continue_models(caps.continuation);
    }
}
impl OriginalOwner<'_>{
    /// Move, never clone or reconstruct. Both successful and failed admission
    /// return custody of every preexisting owner field and the actual sink.
    pub(super) fn into_boolean_search(mut self,sink:capture::CaptureEmitter,choice:ProposedPolicy)->OwnedAttempt{
        let repeated=self.boolean_attempted;self.boolean_attempted=true;let mut diagnostic=None;
        let admission=(||->Result<Caps,Stop>{
            if repeated||!self.map_published||self.map_ack.is_none()||self.checked.is_none(){return self.budget.fail(Stop::Phase)}
            self.budget.reserve_checking(8192,size_of::<OwnedAttempt>() as u64+65536,Instant::now())?;
            let Some(binding)=self.original.binding else{return self.budget.fail(Stop::InvalidLimits)};
            if let Err(e)=sink.verify_directory(&self.original.root.join("live")){diagnostic=Some(e);return self.budget.fail(Stop::Observer)}
            if let Err(e)=sink.verify_attempt_limits(binding,&self.capture_limits){diagnostic=Some(e);return self.budget.fail(Stop::Observer)}
            let caps=policy::for_owner(&self,choice)?;
            self.budget.check(Instant::now())?;
            Ok(caps)
        })();
        let OriginalOwner{original,budget,binding,launch_sha256,math,checked,retained_unchecked,pattern,retained_bytes,map_ack,publication_attempted,map_published,boolean_attempted,capture_limits}=self;
        let original_deadline=budget.deadline;
        let mut out=OwnedAttempt{
            original:OriginalParts{public_sha256:original.public_sha256,factor_sha256:original.factor_base_sha256,budget,binding,launch_sha256,math,checked,unchecked:retained_unchecked,pattern,map_bytes:retained_bytes,map_ack,publication_attempted,map_published,capture_limits,boolean_attempted},
            sink,diagnostic,expansion:None,normalized:Vec::new(),normalization_generator:0,normalization_term:0,normalization_sort:0,
            continuation:None,coverage:None,exclusion_audit:None,publication:publication::Publication::default(),original_deadline,failure:None,
            lift_records:Vec::new(),accepted_record:None,
        };
        let caps=match admission{Ok(c)=>c,Err(s)=>{out.fail(Failure::Admission(s),s);return out}};
        out.run(caps);out
    }
}
