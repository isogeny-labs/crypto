use std::{cell::RefCell,collections::BTreeMap};
pub type Counts=BTreeMap<u64,[u64;11]>;
thread_local!{static COUNTS:RefCell<Counts>=RefCell::new(BTreeMap::new());}
pub fn hit(q:u64,i:usize,n:u64){COUNTS.with(|c|c.borrow_mut().entry(q).or_insert([0;11])[i]+=n)}
pub fn snapshot()->Counts{COUNTS.with(|c|c.borrow().clone())}
pub fn reset(){COUNTS.with(|c|c.borrow_mut().clear())}
pub fn delta(before:&Counts)->Counts{let mut out=snapshot();for(q,v)in out.iter_mut(){let b=before.get(q).copied().unwrap_or([0;11]);for i in 0..11{v[i]-=b[i];}}out.retain(|_,v|v.iter().any(|&x|x!=0));out}
