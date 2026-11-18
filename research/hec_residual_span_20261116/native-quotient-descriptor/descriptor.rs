use hec_native_ic::counted::Counts;
use std::cell::RefCell;
#[derive(Clone,Debug,PartialEq)]pub struct Doc{pub admitted:usize,pub a:Vec<u64>,pub b:Vec<u64>,pub grounded:Vec<bool>,pub rank:usize,pub witness:Option<(usize,usize)>,pub raw_checked:usize,pub counts:Counts}
thread_local!{static DOC:RefCell<Option<Doc>>=const{RefCell::new(None)};}
pub fn clear(){DOC.with(|d|*d.borrow_mut()=None);}
pub fn store(doc:Doc){DOC.with(|d|{let d=&mut*d.borrow_mut();assert!(d.is_none());*d=Some(doc);});}
pub fn take()->Option<Doc>{DOC.with(|d|d.borrow_mut().take())}
