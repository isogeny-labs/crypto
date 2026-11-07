//! Candidate selection only, after bounded wire/identity classification.
//! A candidate is not mathematical admission or authority to export rows.
use std::time::Instant;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum State { Complete, BoundTruncatedPrefix, Unavailable, Invalid }
#[derive(Clone,Copy,Debug)]
pub struct Snapshot {pub index:u64,pub state:State}
pub trait Admission {fn reserve(&mut self,work:u64)->Result<(),()>;fn check(&mut self)->Result<(),()>;}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Failure { Empty,Cap,Owner,Deadline,Order,Unavailable,Invalid,TailNotTerminal,NoComplete }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Candidate {pub selected:usize,pub tail:Option<usize>}
#[derive(Debug)]pub struct Attempt {pub result:Result<Candidate,Failure>,pub inspected:usize,pub charged_work:u64}
/// State::BoundTruncatedPrefix requires the strict prefix parser plus the same
/// source/map binding as complete snapshots. Callers must classify EVERY input;
/// no parse, custody, identity, or resource error may be replaced with that state.
/// After this helper, independently replay descent and mathematics of selected.
pub fn select(snapshots:&[Snapshot],cap:usize,deadline:Instant,owner:&mut impl Admission)->Attempt {
 let mut out=Attempt{result:Err(Failure::Empty),inspected:0,charged_work:0};
 out.result=(||{
  if cap==0||cap>4096||snapshots.len()>cap{return Err(Failure::Cap)}
  if snapshots.is_empty(){return Err(Failure::Empty)}
  let mut selected=None;let mut tail=None;
  for(i,s)in snapshots.iter().enumerate(){
   if Instant::now()>=deadline{return Err(Failure::Deadline)}
   owner.reserve(1).map_err(|_|Failure::Owner)?;out.charged_work+=1;
   owner.check().map_err(|_|Failure::Owner)?;
   if Instant::now()>=deadline{return Err(Failure::Deadline)}
   out.inspected+=1;
   if s.index!=i as u64{return Err(Failure::Order)}
   match s.state {
    State::Complete=>selected=Some(i),
    State::BoundTruncatedPrefix=>{if i+1!=snapshots.len(){return Err(Failure::TailNotTerminal)}tail=Some(i)},
    State::Unavailable=>return Err(Failure::Unavailable),
    State::Invalid=>return Err(Failure::Invalid),
   }
  }
  owner.check().map_err(|_|Failure::Owner)?;
  if Instant::now()>=deadline{return Err(Failure::Deadline)}
  Ok(Candidate{selected:selected.ok_or(Failure::NoComplete)?,tail})
 })();out
}
#[cfg(test)]mod tests{
 use super::*;use std::time::Duration;
 #[derive(Default)]struct Owner{work:u64,stop:bool,stop_after:Option<u64>}
 impl Admission for Owner{fn reserve(&mut self,w:u64)->Result<(),()>{if self.stop{return Err(())}self.work+=w;Ok(())}fn check(&mut self)->Result<(),()>{if self.stop_after==Some(self.work){self.stop=true}if self.stop{Err(())}else{Ok(())}}}
 fn run(states:&[State])->Attempt{let s:Vec<_>=states.iter().enumerate().map(|(i,&state)|Snapshot{index:i as u64,state}).collect();select(&s,4096,Instant::now()+Duration::from_secs(10),&mut Owner::default())}
 #[test]fn latest_complete_wins(){assert_eq!(run(&[State::Complete,State::Complete]).result,Ok(Candidate{selected:1,tail:None}));}
 #[test]fn exactly_one_bound_terminal_prefix_preserves_predecessor(){assert_eq!(run(&[State::Complete,State::Complete,State::BoundTruncatedPrefix]).result,Ok(Candidate{selected:1,tail:Some(2)}));}
 #[test]fn malformed_tail_never_salvages_earlier_rows(){assert_eq!(run(&[State::Complete,State::Invalid]).result,Err(Failure::Invalid));}
 #[test]fn resource_or_identity_unavailable_never_salvages(){assert_eq!(run(&[State::Complete,State::Unavailable]).result,Err(Failure::Unavailable));}
 #[test]fn cannot_skip_malformed_middle(){assert_eq!(run(&[State::Complete,State::Invalid,State::Complete]).result,Err(Failure::Invalid));}
 #[test]fn cannot_continue_after_truncated_snapshot(){assert_eq!(run(&[State::Complete,State::BoundTruncatedPrefix,State::Complete]).result,Err(Failure::TailNotTerminal));}
 #[test]fn multiple_tails_are_rejected(){assert_eq!(run(&[State::Complete,State::BoundTruncatedPrefix,State::BoundTruncatedPrefix]).result,Err(Failure::TailNotTerminal));}
 #[test]fn only_partial_capture_is_not_a_candidate(){assert_eq!(run(&[State::BoundTruncatedPrefix]).result,Err(Failure::NoComplete));}
 #[test]fn empty_is_not_a_candidate(){assert_eq!(run(&[]).result,Err(Failure::Empty));}
 #[test]fn sequence_gaps_or_duplicates_are_rejected(){for index in [0,2,u64::MAX]{let s=[Snapshot{index:0,state:State::Complete},Snapshot{index,state:State::Complete}];assert_eq!(select(&s,3,Instant::now()+Duration::from_secs(1),&mut Owner::default()).result,Err(Failure::Order));}}
 #[test]fn cap_refuses_before_owner_charge(){let mut o=Owner::default();let s=[Snapshot{index:0,state:State::Complete}];assert_eq!(select(&s,0,Instant::now()+Duration::from_secs(1),&mut o).result,Err(Failure::Cap));assert_eq!(o.work,0);}
 #[test]fn original_deadline_refuses_before_charge(){let mut o=Owner::default();let s=[Snapshot{index:0,state:State::Complete}];assert_eq!(select(&s,1,Instant::now(),&mut o).result,Err(Failure::Deadline));assert_eq!(o.work,0);}
 #[test]fn shared_owner_refusal_has_no_candidate_or_retry(){let s=[Snapshot{index:0,state:State::Complete},Snapshot{index:1,state:State::Complete}];let mut o=Owner{stop_after:Some(2),..Owner::default()};let d=Instant::now()+Duration::from_secs(1);let a=select(&s,2,d,&mut o);assert_eq!(a.result,Err(Failure::Owner));assert_eq!(a.inspected,1);assert_eq!(a.charged_work,2);assert_eq!(select(&s,2,d,&mut o).result,Err(Failure::Owner));assert_eq!(o.work,2);}
}
