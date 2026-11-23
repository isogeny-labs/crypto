use std::{cell::RefCell,collections::BTreeMap};
use hec_native_ic::counted::{self,Counts};
#[derive(Default)]struct State{last:Counts,label:Option<&'static str>,phases:BTreeMap<&'static str,Counts>}
thread_local!{static STATE:RefCell<State>=RefCell::new(State::default());}
pub fn reset(){STATE.with(|s|*s.borrow_mut()=State::default());}
pub fn set(label:&'static str){let now=counted::snapshot();STATE.with(|s|{let s=&mut*s.borrow_mut();if let Some(previous)=s.label{let acc=s.phases.entry(previous).or_default();for(&p,v)in &now{let out=acc.entry(p).or_insert([0;16]);let before=s.last.get(&p).copied().unwrap_or([0;16]);for j in 0..16{out[j]+=v[j].checked_sub(before[j]).unwrap();}}}s.last=now;s.label=Some(label);});}
pub fn finish()->BTreeMap<&'static str,Counts>{set("finished");STATE.with(|s|s.borrow().phases.clone())}
