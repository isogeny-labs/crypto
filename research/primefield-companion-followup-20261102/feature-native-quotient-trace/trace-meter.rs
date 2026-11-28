//! Audit-only, same-thread source quotient trace. No timing or global counters.
use std::cell::RefCell;
#[derive(Clone,Debug,PartialEq)]pub struct Event{pub p:u64,pub n:usize,pub d:usize,pub support:usize,pub weight:u128,pub route:u8,pub products:u64,pub inversions:u64,pub ok:bool}
#[derive(Debug,Default)]pub struct Capture{pub rows:Vec<Event>,pub dropped:u64,pub invalid:bool}
#[derive(Default)]struct State{active:bool,cap:usize,counts:[u64;2],route:u8,capture:Capture}
thread_local!{static STATE:RefCell<State>=RefCell::new(State::default());}
pub fn begin(cap:usize)->Result<(), &'static str>{STATE.with(|s|{let mut s=s.borrow_mut();if s.active{return Err("already active");}if cap>100000{return Err("cap exceeds 100000");}*s=State{active:true,cap,..State::default()};Ok(())})}
pub fn finish()->Result<Capture,&'static str>{STATE.with(|s|{let mut s=s.borrow_mut();if !s.active{return Err("not active");}s.active=false;Ok(std::mem::take(&mut s.capture))})}
pub(super) fn bump(i:usize){STATE.with(|s|{let mut s=s.borrow_mut();if s.active{match s.counts[i].checked_add(1){Some(v)=>s.counts[i]=v,None=>s.capture.invalid=true}}})}
pub(super) fn route(route:u8){STATE.with(|s|{let mut s=s.borrow_mut();if s.active{s.route=route;}})}
pub(super) fn start(p:u64,a:&[u64],b:&[u64])->Option<(Event,[u64;2])>{STATE.with(|s|{let mut s=s.borrow_mut();if !s.active{return None;}s.route=255;
 let n=a.len()-b.len()+1;let d=b.len()-1;let mut support=0;let mut weight=0u128;
 for j in 1..=d.min(n-1){if b[d-j]!=0{support+=1;weight+=(n-j)as u128;}}
 Some((Event{p,n,d,support,weight,route:255,products:0,inversions:0,ok:false},s.counts))
})}
pub(super) fn end(before:Option<(Event,[u64;2])>,ok:bool){let Some((mut event,counts))=before else{return;};STATE.with(|s|{let mut s=s.borrow_mut();event.route=s.route;event.products=s.counts[0]-counts[0];event.inversions=s.counts[1]-counts[1];event.ok=ok;if event.route==255{s.capture.invalid=true;}if s.capture.rows.len()<s.cap{s.capture.rows.push(event)}else{s.capture.invalid=true;s.capture.dropped=s.capture.dropped.saturating_add(1);}})}
#[cfg(trace_tests)]pub fn force_overflow(){STATE.with(|s|s.borrow_mut().counts[0]=u64::MAX);bump(0);}
