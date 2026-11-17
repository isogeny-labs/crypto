use hec_native_ic::counted::Counts;use std::cell::RefCell;
thread_local!{static CONTROL:RefCell<Counts>=RefCell::new(Counts::new());}
pub fn record(before:Counts,after:Counts){let mut d=after;for(p,v)in &mut d{for j in 0..16{v[j]-=before.get(p).map_or(0,|x|x[j]);}}CONTROL.with(|x|*x.borrow_mut()=d);}
pub fn snapshot()->Counts{CONTROL.with(|x|x.borrow().clone())}
