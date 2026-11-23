use std::{cell::RefCell,collections::BTreeMap};
pub type Counts=BTreeMap<u64,[u64;16]>;
type Phases=BTreeMap<(&'static str,u64),[u64;16]>;
thread_local!{static TOTAL:RefCell<Counts>=RefCell::new(BTreeMap::new());static PHASES:RefCell<Phases>=RefCell::new(BTreeMap::new());static STAGE:RefCell<&'static str>=const{RefCell::new("entry")};}
pub fn stage(s:&'static str){STAGE.with(|c|*c.borrow_mut()=s)}
pub fn hit(p:u64,j:usize,n:u64){TOTAL.with(|c|c.borrow_mut().entry(p).or_insert([0;16])[j]+=n);let s=STAGE.with(|c|*c.borrow());PHASES.with(|c|c.borrow_mut().entry((s,p)).or_insert([0;16])[j]+=n)}
pub fn reset(){TOTAL.with(|c|c.borrow_mut().clear());PHASES.with(|c|c.borrow_mut().clear());stage("entry")}
pub fn snapshot()->Counts{TOTAL.with(|c|c.borrow().clone())}
pub fn phases()->Phases{PHASES.with(|c|c.borrow().clone())}
