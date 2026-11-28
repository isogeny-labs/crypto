//! Consuming transport proposal: actual admitted original parts, no borrowed token.
use super::*;
use provenance::boolean_equations as equations;
use provenance::{checked_model, signed_lift};
use std::mem::size_of;
mod sparse {
    pub use super::equations::{Charge, Monomial, Polynomial};
    pub trait Owner {fn reserve(&mut self,q:Charge)->Result<(),()>;fn check(&mut self)->Result<(),()>;}
}
#[path="dynamic_boolean_search_scheduler.rs"] mod scheduler;
#[path="../cryptanalysis/boolean_identity_bqi.rs"] mod bqi;
#[path="dynamic_boolean_search_bqi.rs"] mod bridge;
#[path="dynamic_boolean_search_check.rs"] mod check;
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
struct LiftAdmission<'a>(&'a mut Budget);
impl provenance::Admission for LiftAdmission<'_>{
    fn reserve(&mut self,q:provenance::Quote)->Result<(),provenance::Error>{self.0.reserve_checking(q.work,q.bytes,Instant::now()).map_err(|_|provenance::Error::Admission)}
    fn check(&mut self)->Result<(),provenance::Error>{self.0.check(Instant::now()).map_err(|_|provenance::Error::Admission)}
}
impl checked_model::VerificationAdmission for LiftAdmission<'_>{
    fn addition(&mut self)->Result<(),provenance::Error>{self.0.reserve_addition(Instant::now()).map_err(|_|provenance::Error::Admission)}
}
#[derive(Clone,Copy)]
pub(in crate::ml_dataset::solvers) struct Caps{pub expansion:equations::Limits,pub search:SearchLimits,pub audit:AuditLimits}
#[derive(Clone,Copy,Debug)]
pub(in crate::ml_dataset::solvers) enum Failure{Admission(Stop),Expansion(equations::Failure),Search(scheduler::Failure),Audit(check::Failure),LiftStopped(provenance::Error)}
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
    search:Option<scheduler::search::SearchAttempt>,audit:Option<check::Audit>,
    original_deadline:Instant,failure:Option<Failure>,
    lift_progress:checked_model::Progress,lift_outcome:Option<signed_lift::Outcome>,lift_attempted:bool,
}
impl OwnedAttempt{
    pub(in crate::ml_dataset::solvers) fn failure(&self)->Option<Failure>{self.failure}
    pub(in crate::ml_dataset::solvers) fn first_stop(&self)->Option<Stop>{self.original.budget.stopped}
    pub(in crate::ml_dataset::solvers) fn charged(&self)->(u64,u64){(self.original.budget.native.work+self.original.budget.checking_work,self.original.budget.native.bytes+self.original.budget.adapter_bytes)}
    pub(in crate::ml_dataset::solvers) fn boolean_candidate(&self)->Option<&[bool]>{
        if self.failure.is_some() || self.audit.as_ref()?.result.is_err(){return None}
        let s=self.search.as_ref()?;if s.result.is_err()||s.verdict!=scheduler::search::Verdict::Witness{return None}
        let node=s.nodes.get(s.witness_node?)?;if node.candidate_valid!=Some(true){return None}Some(&node.candidate)
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
    /// A first-candidate outcome only; never a whole-pattern no-lift verdict.
    pub(in crate::ml_dataset::solvers) fn lift_outcome(&self)->Option<signed_lift::Outcome>{
        if self.failure.is_some() && matches!(self.lift_outcome,Some(signed_lift::Outcome::VerifiedSignedRelation)){return None}
        self.lift_outcome
    }
    fn lift_candidate(&mut self){
        if self.failure.is_some(){return}
        let Some(search)=self.search.as_ref()else{self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return};
        if search.verdict!=scheduler::search::Verdict::Witness{return}
        if self.lift_attempted{self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return}
        self.lift_attempted=true;
        // Audit admission precedes native model consumption; source/model are
        // borrowed from this exact owned attempt, with no externally supplied map.
        if self.audit.as_ref().is_none_or(|a|a.result.is_err()){
            self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return
        }
        let Some(index)=search.witness_node else{self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return};
        let Some(node)=search.nodes.get(index)else{self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return};
        if node.candidate_valid!=Some(true){self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return}
        let Some(expansion)=self.expansion.as_ref()else{self.fail(Failure::Admission(Stop::Phase),Stop::Phase);return};
        if let Err(s)=self.original.budget.begin_extraction(true,Instant::now()){self.fail(Failure::Admission(s),s);return}
        let result=signed_lift::verify_candidate(
            expansion.source(),&node.candidate,&self.original.math.signed,self.original.math.signed_target,
            &mut self.lift_progress,self.original_deadline,&mut LiftAdmission(&mut self.original.budget));
        self.lift_outcome=Some(result);
        match result{
            signed_lift::Outcome::Deadline=>self.fail(Failure::Admission(Stop::Deadline),Stop::Deadline),
            signed_lift::Outcome::Stopped(e)=>self.fail(Failure::LiftStopped(e),Stop::NativeFailure),
            signed_lift::Outcome::RejectedUnclassified(_)=>{
                // Keep this exact candidate, remaining search nodes and lift
                // progress. Further model search is NOT implemented here.
                // No pattern refutation or global no-lift result is inferred.
            },
            signed_lift::Outcome::VerifiedSignedRelation=>{
                // Stay in Extraction for prepaid capture/publication. Completing
                // the budget here would prohibit later same-owner writes.
                if let Err(s)=self.original.budget.check(Instant::now()){self.fail(Failure::Admission(s),s)}
            },
        }
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
        if let Err(s)=self.original.budget.finish_native(Instant::now()){self.fail(Failure::Admission(s),s);return}
        let phase=match self.original.budget.begin_native(true,Instant::now()){Ok(v)=>v,Err(s)=>{self.fail(Failure::Admission(s),s);return}};
        caps.search.native.work=caps.search.native.work.min(phase.work);caps.search.native.bytes=caps.search.native.bytes.min(phase.bytes);
        let variables=self.expansion.as_ref().expect("owned expansion").variables();
        let search=scheduler::search::solve(variables,&self.normalized,caps.search,self.original_deadline,&mut MatrixAdmission(&mut self.original.budget));
        let status=search.result;self.search=Some(search);
        if let Err(f)=status{let stop=if f==scheduler::Failure::Deadline{Stop::Deadline}else{Stop::NativeFailure};self.fail(Failure::Search(f),stop);return}
        let audit=check::verify(variables,&self.normalized,self.search.as_ref().expect("retained search"),caps.audit,self.original_deadline,&mut MatrixAdmission(&mut self.original.budget));
        let status=audit.result;self.audit=Some(audit);
        if let Err(f)=status{let stop=if f==check::Failure::Deadline{Stop::Deadline}else{Stop::NativeFailure};self.fail(Failure::Audit(f),stop);return}
        if let Err(s)=self.original.budget.finish_native(Instant::now()){self.fail(Failure::Admission(s),s);return}
        self.lift_candidate();
    }
}
impl OriginalOwner<'_>{
    /// Move, never clone or reconstruct. Both successful and failed admission
    /// return custody of every preexisting owner field and the actual sink.
    pub(super) fn into_boolean_search(mut self,sink:capture::CaptureEmitter,caps:Caps)->OwnedAttempt{
        let repeated=self.boolean_attempted;self.boolean_attempted=true;let mut diagnostic=None;
        let admission=(||->Result<(),Stop>{
            if repeated||!self.map_published||self.map_ack.is_none()||self.checked.is_none(){return self.budget.fail(Stop::Phase)}
            self.budget.reserve_checking(8192,size_of::<OwnedAttempt>() as u64+65536,Instant::now())?;
            let Some(binding)=self.original.binding else{return self.budget.fail(Stop::InvalidLimits)};
            if let Err(e)=sink.verify_directory(&self.original.root.join("live")){diagnostic=Some(e);return self.budget.fail(Stop::Observer)}
            if let Err(e)=sink.verify_attempt_limits(binding,&self.capture_limits){diagnostic=Some(e);return self.budget.fail(Stop::Observer)}
            self.budget.check(Instant::now())
        })();
        let OriginalOwner{original,budget,binding,launch_sha256,math,checked,retained_unchecked,pattern,retained_bytes,map_ack,publication_attempted,map_published,boolean_attempted,capture_limits}=self;
        let original_deadline=budget.deadline;
        let mut out=OwnedAttempt{
            original:OriginalParts{public_sha256:original.public_sha256,factor_sha256:original.factor_base_sha256,budget,binding,launch_sha256,math,checked,unchecked:retained_unchecked,pattern,map_bytes:retained_bytes,map_ack,publication_attempted,map_published,capture_limits,boolean_attempted},
            sink,diagnostic,expansion:None,normalized:Vec::new(),normalization_generator:0,normalization_term:0,normalization_sort:0,
            search:None,audit:None,original_deadline,failure:None,
            lift_progress:checked_model::Progress::default(),lift_outcome:None,lift_attempted:false,
        };
        if let Err(s)=admission{out.fail(Failure::Admission(s),s);return out}
        out.run(caps);out
    }
}
