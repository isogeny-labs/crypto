//! Proposed borrowed-record checker, not registered in production.
//! Caller owns authenticated capture bytes, strict typed decode, original map,
//! signed public domain and one cumulative VerificationAdmission throughout.
//! This checks custody joins and a model relation; not CDCL transition semantics.
use crate::cryptanalysis::binary_semaev_chain_sat::provenance::{
    checked_model::{self, Progress, VerificationAdmission}, Admission, CheckedMap, Quote,
};
use crate::cryptanalysis::sat::SolverEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error { Admission, Identity, Phase, Cap, Model, Claim }
#[derive(Clone, Copy)]
pub(super) struct Expected {
    pub original_binding: [u8;32],
    pub pattern: u32,
    pub checkpoint_sequence: u64,
    pub checkpoint_sha256: [u8;32],
    pub event_limit: usize,
}
/// Produced only by the existing authenticated capture traversal, not wire fields
/// accepted on their own. Digest includes the exact newline-terminated raw line.
pub(super) struct Link {
    pub sequence: u64,
    pub previous_sha256: [u8;32],
    pub actual_line_sha256: [u8;32],
}
pub(super) struct Event<'a> {
    pub original_binding: [u8;32], pub pattern: u32,
    pub initial_checkpoint_sequence: u64, pub initial_checkpoint_sha256: [u8;32],
    pub native_index: usize, pub event: &'a SolverEvent,
}
pub(super) struct ResultRecord<'a> {
    pub original_binding: [u8;32], pub pattern: u32,
    pub initial_checkpoint_sequence: u64, pub initial_checkpoint_sha256: [u8;32],
    pub search_end_sequence: u64, pub search_end_sha256: [u8;32],
    pub model: &'a [bool], pub claimed_verification: &'a Progress,
    pub scalar_recovered: bool, pub all_patterns_negative: bool,
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase { Fresh, Searching, SatEnd, UncertifiedUnsatEnd, UnknownEnd, ModelChecked }
/// Retained by the central replay Frame on success AND every failure. It never
/// owns a new policy/deadline and grants no terminal/export capability itself.
pub(super) struct Segment {
    expected: Expected, phase: Phase, count: usize, decisions: usize,
    sequence: u64, head: [u8;32], first_error: Option<Error>,
    pub verification: Progress,
}
fn pay(owner:&mut impl VerificationAdmission, work:u64)->Result<(),Error> {
    owner.reserve(Quote{work,bytes:0}).map_err(|_|Error::Admission)?;
    owner.check().map_err(|_|Error::Admission)
}
impl Segment {
    pub(super) fn new(expected:Expected, owner:&mut impl VerificationAdmission)->Result<Self,Error> {
        pay(owner,64)?;
        if expected.event_limit==0 {return Err(Error::Cap)}
        Ok(Self{sequence:expected.checkpoint_sequence,head:expected.checkpoint_sha256,
            expected,phase:Phase::Fresh,count:0,decisions:0,first_error:None,
            verification:Progress::default()})
    }
    fn identity(&self,binding:[u8;32],pattern:u32,sequence:u64,head:[u8;32])->Result<(),Error> {
        if binding!=self.expected.original_binding || pattern!=self.expected.pattern ||
            sequence!=self.expected.checkpoint_sequence || head!=self.expected.checkpoint_sha256 {
            return Err(Error::Identity)
        }
        Ok(())
    }
    fn link(&self,link:&Link)->Result<(),Error> {
        if self.sequence.checked_add(1)!=Some(link.sequence) || self.head!=link.previous_sha256 {
            return Err(Error::Identity)
        }
        Ok(())
    }
    fn retain_failure(&mut self,result:Result<(),Error>)->Result<(),Error> {
        if let Err(e)=result {self.first_error.get_or_insert(e);}
        result
    }
    pub(super) fn event(&mut self,link:&Link,row:&Event<'_>,owner:&mut impl VerificationAdmission)->Result<(),Error> {
        if let Some(e)=self.first_error{return Err(e)}
        let result=(||{
            pay(owner,128)?;self.link(link)?;
            self.identity(row.original_binding,row.pattern,row.initial_checkpoint_sequence,row.initial_checkpoint_sha256)?;
            if row.native_index!=self.count || self.count>=self.expected.event_limit{return Err(Error::Cap)}
            match (self.phase,row.event) {
                (Phase::Fresh,SolverEvent::SolveBegin{..})=>self.phase=Phase::Searching,
                (Phase::Searching,SolverEvent::SolveBegin{..})=>return Err(Error::Phase),
                (Phase::Searching,SolverEvent::SolveEnd{result})=>{
                    self.phase=match result.as_str(){"Sat"=>Phase::SatEnd,"Unsat"=>Phase::UncertifiedUnsatEnd,"Unknown"=>Phase::UnknownEnd,_=>return Err(Error::Phase)};
                },
                (Phase::Searching,SolverEvent::Snapshot{..})=>return Err(Error::Phase),
                (Phase::Searching,SolverEvent::Assignment{initial:true,..})=>return Err(Error::Phase),
                (Phase::Searching,SolverEvent::Decision{..})=>self.decisions=self.decisions.checked_add(1).ok_or(Error::Cap)?,
                (Phase::Searching,_)=>{}, // retained diagnostic transition, not semantically certified
                _=>return Err(Error::Phase),
            }
            self.count=self.count.checked_add(1).ok_or(Error::Cap)?;
            self.sequence=link.sequence;self.head=link.actual_line_sha256;
            pay(owner,1)
        })();self.retain_failure(result)
    }
    pub(super) fn model_result(&mut self,link:&Link,row:&ResultRecord<'_>,map:&CheckedMap,
        signed_domain:&[(u64,u64)],target:Option<(u64,u64)>,owner:&mut impl VerificationAdmission)->Result<(),Error> {
        if let Some(e)=self.first_error{return Err(e)}
        let result=(||{
            pay(owner,256)?;self.link(link)?;
            self.identity(row.original_binding,row.pattern,row.initial_checkpoint_sequence,row.initial_checkpoint_sha256)?;
            if self.phase!=Phase::SatEnd || row.search_end_sequence!=self.sequence || row.search_end_sha256!=self.head ||
                row.scalar_recovered || row.all_patterns_negative || map.original_binding()!=self.expected.original_binding ||
                map.pattern()!=self.expected.pattern {return Err(Error::Identity)}
            // Fresh central Progress, actual authenticated map/model/full signed domain.
            // No native Solver, producer Progress reuse, allowance reset or new deadline.
            checked_model::verify(map,row.model,signed_domain,target,&mut self.verification,owner).map_err(|_|Error::Model)?;
            let a=&self.verification;let b=row.claimed_verification;
            let units=a.blocks.len().checked_add(b.blocks.len()).and_then(|n|n.checked_add(a.lift_choices.len())).and_then(|n|n.checked_add(b.lift_choices.len())).and_then(|n|n.checked_add(a.path.len())).and_then(|n|n.checked_add(b.path.len())).ok_or(Error::Cap)?;
            pay(owner,(units as u64).checked_mul(32).and_then(|n|n.checked_add(64)).ok_or(Error::Cap)?)?;
            if !a.signed_relation || a.formula_rows!=b.formula_rows || a.blocks!=b.blocks || a.chain_steps!=b.chain_steps ||
                a.lift_choices!=b.lift_choices || a.path!=b.path || a.sum!=b.sum || a.sign_mask!=b.sign_mask ||
                a.signed_relation!=b.signed_relation {return Err(Error::Claim)}
            pay(owner,1)?;
            self.sequence=link.sequence;self.head=link.actual_line_sha256;self.phase=Phase::ModelChecked;
            Ok(())
        })();self.retain_failure(result)
    }
    pub(super) fn model_relation_checked(&self)->bool {self.first_error.is_none() && self.phase==Phase::ModelChecked}
    pub(super) fn uncertified_native_unsat(&self)->bool {self.first_error.is_none() && self.phase==Phase::UncertifiedUnsatEnd}
    pub(super) fn observed_decisions(&self)->usize {self.decisions}
    pub(super) fn endpoint(&self)->(u64,[u8;32]) {(self.sequence,self.head)}
}
