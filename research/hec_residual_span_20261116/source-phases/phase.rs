use hec_native_ic::counted::{self,Counts};
use std::{cell::RefCell,collections::BTreeMap};
thread_local!{static LAST:RefCell<Counts>=RefCell::new(BTreeMap::new());static PHASES:RefCell<BTreeMap<(String,u64),[u64;16]>>=RefCell::new(BTreeMap::new());}
pub fn reset(){LAST.with(|x|x.borrow_mut().clear());PHASES.with(|x|x.borrow_mut().clear());}
pub fn tap(name:&str){let now=counted::snapshot();LAST.with(|last|{let mut last=last.borrow_mut();PHASES.with(|phases|{let mut phases=phases.borrow_mut();for(&p,v)in &now{let prior=last.get(&p).copied().unwrap_or([0;16]);let out=phases.entry((name.to_owned(),p)).or_default();for j in 0..16{out[j]+=v[j]-prior[j];}}});*last=now;});}
pub fn snapshot()->BTreeMap<(String,u64),[u64;16]>{PHASES.with(|x|x.borrow().clone())}
