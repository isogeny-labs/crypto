//! Dynamic gate-ANF variable index w-1 is already the checked SAT model index.
//! No coordinate conversion, inferred signs, replacement curve, or new budget.
use super::{checked_model::{self,Progress,VerificationAdmission},Admission,CheckedMap,Error,Quote};
use std::time::Instant;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Outcome{VerifiedSignedRelation,ExhaustedForModel,RejectedUnclassified(Error),Stopped(Error),Deadline}
struct Bound<'a,T:VerificationAdmission>{owner:&'a mut T,deadline:Instant,expired:bool,stop:Option<Error>}
impl<T:VerificationAdmission> Bound<'_,T>{fn time(&mut self)->Result<(),Error>{if Instant::now()>=self.deadline{self.expired=true;Err(Error::Admission)}else{Ok(())}}}
impl<T:VerificationAdmission> Admission for Bound<'_,T>{
 fn reserve(&mut self,q:Quote)->Result<(),Error>{self.time()?;self.owner.reserve(q).map_err(|e|{self.stop=Some(e);e})?;self.time()}
 fn check(&mut self)->Result<(),Error>{self.time()?;self.owner.check().map_err(|e|{self.stop=Some(e);e})?;self.time()}
}
impl<T:VerificationAdmission> VerificationAdmission for Bound<'_,T>{fn addition(&mut self)->Result<(),Error>{self.time()?;self.owner.addition().map_err(|e|{self.stop=Some(e);e})?;self.time()}}
/// Caller supplies its retained checked map, actual signed domain/target, original
/// absolute deadline and original owner. Progress is caller-owned before entry.
/// Only the classified terminal sign-mask verdict is ExhaustedForModel.
/// Early semantic errors remain unclassified; neither implies pattern refutation.
pub fn verify_candidate(map:&CheckedMap,model:&[bool],signed_domain:&[(u64,u64)],target:Option<(u64,u64)>,progress:&mut Progress,deadline:Instant,owner:&mut impl VerificationAdmission)->Outcome{
 let mut bound=Bound{owner,deadline,expired:false,stop:None};
 let result=checked_model::verify_classified(map,model,signed_domain,target,progress,&mut bound);
 if bound.expired{return Outcome::Deadline}
 if let Some(e)=bound.stop{return Outcome::Stopped(e)}
 match result{Ok(checked_model::LiftVerdict::SignedRelation)=>Outcome::VerifiedSignedRelation,Ok(checked_model::LiftVerdict::ExhaustedForModel)=>Outcome::ExhaustedForModel,Err(Error::Admission|Error::Cap)=>Outcome::Stopped(result.unwrap_err()),Err(e)=>Outcome::RejectedUnclassified(e)}
}
