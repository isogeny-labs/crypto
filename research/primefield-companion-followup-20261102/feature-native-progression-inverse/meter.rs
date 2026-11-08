//! Single-thread software instrument; requests only, no live/peak heap claim.
use std::alloc::{GlobalAlloc,Layout,System};
use std::sync::atomic::{AtomicBool,AtomicU64,AtomicUsize,Ordering::Relaxed};
static ARMED:AtomicBool=AtomicBool::new(false);
static DEPTH:AtomicUsize=AtomicUsize::new(0);
static OVERFLOW:AtomicBool=AtomicBool::new(false);
pub const NAMES:[&str;10]=["scalar_products","polynomial_products","eea_inverse_calls","pow_r_minus_2_calls","alloc_calls","zeroed_calls","realloc_calls","requested_bytes","allocation_failures","dealloc_calls"];
static COUNTS:[AtomicU64;10]=[const{AtomicU64::new(0)};10];
fn add(i:usize,n:u64){let old=COUNTS[i].load(Relaxed);match old.checked_add(n){Some(v)=>COUNTS[i].store(v,Relaxed),None=>{OVERFLOW.store(true,Relaxed);COUNTS[i].store(u64::MAX,Relaxed);}}}
fn active()->bool{ARMED.load(Relaxed)&&DEPTH.load(Relaxed)>0}
pub fn hit(i:usize){if active(){add(i,1)}}
fn allocation(i:usize,size:usize,ok:bool){if active(){add(i,1);add(7,size as u64);if !ok{add(8,1)}}}
pub struct CountingAllocator;
// SAFETY: forward the unchanged pointers/layouts to System; callbacks only use
// nonallocating, nonpanicking atomic bookkeeping. No pointer ownership changes.
unsafe impl GlobalAlloc for CountingAllocator{
 unsafe fn alloc(&self,l:Layout)->*mut u8{let p=unsafe{System.alloc(l)};allocation(4,l.size(),!p.is_null());p}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{let p=unsafe{System.alloc_zeroed(l)};allocation(5,l.size(),!p.is_null());p}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{let q=unsafe{System.realloc(p,l,n)};allocation(6,n,!q.is_null());q}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){hit(9);unsafe{System.dealloc(p,l)}}
}
#[global_allocator]static ALLOCATOR:CountingAllocator=CountingAllocator;
pub struct Region{entered:bool,_single:std::marker::PhantomData<std::rc::Rc<()>>}
impl Region{pub fn enter()->Self{let entered=ARMED.load(Relaxed);if entered{DEPTH.fetch_add(1,Relaxed);}Self{entered,_single:std::marker::PhantomData}}}
impl Drop for Region{fn drop(&mut self){if self.entered{DEPTH.fetch_sub(1,Relaxed);}}}
struct Session;
impl Drop for Session{fn drop(&mut self){ARMED.store(false,Relaxed);DEPTH.store(0,Relaxed);}}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Snapshot{pub counts:[u64;10],pub overflow:bool}
/// One process thread only. Callback may return retained allocations. Counting
/// occurs only in Regions (IntervalProducts::products); output/outer work excluded.
pub fn capture<T>(f:impl FnOnce()->T)->Result<(T,Snapshot),&'static str>{
 if ARMED.compare_exchange(false,true,Relaxed,Relaxed).is_err(){return Err("nested capture");}
 let session=Session;for n in &COUNTS{n.store(0,Relaxed)}OVERFLOW.store(false,Relaxed);
 let value=f();let snapshot=Snapshot{counts:std::array::from_fn(|i|COUNTS[i].load(Relaxed)),overflow:OVERFLOW.load(Relaxed)};
 drop(session);Ok((value,snapshot))
}
pub fn self_test(){
 let (_,s)=capture(||{let _g=Region::enter();unsafe{
  let l=Layout::from_size_align(16,8).unwrap();let p=ALLOCATOR.alloc(l);assert!(!p.is_null());
  let q=ALLOCATOR.realloc(p,l,24);assert!(!q.is_null());ALLOCATOR.dealloc(q,Layout::from_size_align(24,8).unwrap());
  let z=ALLOCATOR.alloc_zeroed(l);assert!(!z.is_null());assert_eq!(*z,0);ALLOCATOR.dealloc(z,l);
 }}).unwrap();assert_eq!(s.counts,[0,0,0,0,1,1,1,56,0,2]);assert!(!s.overflow);
 let(_,s)=capture(||{hit(0);{let _g=Region::enter();hit(0);{let _n=Region::enter();hit(0);}hit(0);}hit(0);assert!(capture(||()).is_err());}).unwrap();assert_eq!(s.counts[0],3);
 let(_,s)=capture(||{let _g=Region::enter();allocation(4,16,false);COUNTS[0].store(u64::MAX,Relaxed);hit(0);}).unwrap();assert!(s.overflow);assert_eq!(s.counts[8],1);assert_eq!(s.counts[7],16);
 let hook=std::panic::take_hook();std::panic::set_hook(Box::new(|_|{}));
 let p=std::panic::catch_unwind(||capture(||{let _g=Region::enter();panic!("synthetic unwind")}));
 std::panic::set_hook(hook);assert!(p.is_err());assert!(!ARMED.load(Relaxed));assert_eq!(DEPTH.load(Relaxed),0);
 let(_,s)=capture(||()).unwrap();assert_eq!(s.counts,[0;10]);assert!(!s.overflow);
}
