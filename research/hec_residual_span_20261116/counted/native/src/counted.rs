use std::{cell::RefCell,collections::BTreeMap};
pub type Counts=BTreeMap<u64,[u64;16]>;
thread_local!{static COUNTS:RefCell<Counts>=RefCell::new(BTreeMap::new());}
pub fn hit(p:u64,j:usize,n:u64){COUNTS.with(|c|c.borrow_mut().entry(p).or_insert([0;16])[j]+=n)}
pub fn reset(){COUNTS.with(|c|c.borrow_mut().clear())}
pub fn snapshot()->Counts{COUNTS.with(|c|c.borrow().clone())}
