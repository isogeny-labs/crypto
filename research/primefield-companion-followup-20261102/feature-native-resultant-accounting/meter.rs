//! Single-thread instrument. No allocation, panic, or logging in allocator callbacks.
use std::alloc::{GlobalAlloc,Layout,System};
use std::sync::atomic::{AtomicBool,AtomicU64,Ordering::Relaxed};
include!("names.rs");
static ACTIVE:AtomicBool=AtomicBool::new(false);
static OPS:[AtomicU64;NAMES.len()]=[const{AtomicU64::new(0)};NAMES.len()];
// alloc, zeroed, realloc, dealloc, requested, live, peak, failures, invalid, overflow
static HEAP:[AtomicU64;10]=[const{AtomicU64::new(0)};10];
pub const HEAP_NAMES:[&str;10]=["alloc_calls","zeroed_calls","realloc_calls","dealloc_calls","requested_bytes","live_bytes","peak_live_bytes","allocation_failures","accounting_invalid","counter_overflow"];
fn add(cell:&AtomicU64,n:u64){
 let _=cell.fetch_update(Relaxed,Relaxed,|v|Some(match v.checked_add(n){Some(x)=>x,None=>{HEAP[9].store(1,Relaxed);u64::MAX}}));
}
pub fn hit(i:usize){if ACTIVE.load(Relaxed){add(&OPS[i],1);}}
fn grow(n:usize){add(&HEAP[5],n as u64);HEAP[6].fetch_max(HEAP[5].load(Relaxed),Relaxed);}
fn shrink(n:usize){let _=HEAP[5].fetch_update(Relaxed,Relaxed,|v|Some(match v.checked_sub(n as u64){Some(x)=>x,None=>{HEAP[8].store(1,Relaxed);0}}));}
fn allocated(kind:usize,n:usize,ok:bool){if ACTIVE.load(Relaxed){add(&HEAP[kind],1);add(&HEAP[4],n as u64);if ok{grow(n)}else{add(&HEAP[7],1);}}}
fn resized(old:usize,new:usize,ok:bool){if ACTIVE.load(Relaxed){add(&HEAP[2],1);add(&HEAP[4],new as u64);if ok{shrink(old);grow(new)}else{add(&HEAP[7],1);}}}
pub struct CountingAllocator;
// SAFETY: all pointer/layout operations are delegated unchanged to System.
// Tracking uses only atomics and arithmetic with explicit overflow handling.
unsafe impl GlobalAlloc for CountingAllocator{
 unsafe fn alloc(&self,l:Layout)->*mut u8{let p=unsafe{System.alloc(l)};allocated(0,l.size(),!p.is_null());p}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{let p=unsafe{System.alloc_zeroed(l)};allocated(1,l.size(),!p.is_null());p}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){if ACTIVE.load(Relaxed){add(&HEAP[3],1);shrink(l.size());}unsafe{System.dealloc(p,l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{let q=unsafe{System.realloc(p,l,n)};resized(l.size(),n,!q.is_null());q}
}
#[global_allocator]static ALLOCATOR:CountingAllocator=CountingAllocator;
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Snapshot{pub ops:[u64;NAMES.len()],pub heap:[u64;10]}
struct Span(std::marker::PhantomData<std::rc::Rc<()>>);
impl Span{
 fn start()->Result<Self,&'static str>{
  if ACTIVE.compare_exchange(false,true,Relaxed,Relaxed).is_err(){return Err("nested accounting span");}
  for x in OPS.iter().chain(HEAP.iter()){x.store(0,Relaxed);}
  Ok(Self(std::marker::PhantomData))
 }
 fn finish(self)->Snapshot{drop(self);Snapshot{ops:std::array::from_fn(|i|OPS[i].load(Relaxed)),heap:std::array::from_fn(|i|HEAP[i].load(Relaxed))}}
}
impl Drop for Span{fn drop(&mut self){ACTIVE.store(false,Relaxed);}}
/// Only a scalar may escape; the caller must retain outside allocations until after finish.
pub fn measure(f:impl FnOnce()->Result<u64,&'static str>)->Result<(u64,Snapshot),&'static str>{
 let span=Span::start()?;let value=f()?;let snapshot=span.finish();
 if snapshot.heap[5]!=0||snapshot.heap[8]!=0||snapshot.heap[9]!=0{return Err("invalid allocation boundary or counter overflow");}
 Ok((value,snapshot))
}
pub fn project(mut a:u64,p:u64)->u64{
 hit(7);let mut e=(p-1)/(1u64<<(p-1).trailing_zeros());let mut v=1;
 while e!=0{if e&1!=0{hit(0);v=((v as u128*a as u128)%p as u128)as u64;}hit(0);a=((a as u128*a as u128)%p as u128)as u64;e>>=1;}v
}
pub fn self_test(){
 // Invoke the trait API directly and pair it only with the same trait API.
 let(_,s)=measure(||{unsafe{
  let a=Layout::from_size_align(16,8).unwrap();let p=ALLOCATOR.alloc(a);assert!(!p.is_null());
  let q=ALLOCATOR.realloc(p,a,24);assert!(!q.is_null());ALLOCATOR.dealloc(q,Layout::from_size_align(24,8).unwrap());
  let z=ALLOCATOR.alloc_zeroed(a);assert!(!z.is_null());assert_eq!(*z,0);ALLOCATOR.dealloc(z,a);
 }Ok(0)}).unwrap();
 assert_eq!(s.heap,[1,1,1,2,56,0,24,0,0,0]);
 let span=Span::start().unwrap();assert!(Span::start().is_err());allocated(0,16,false);allocated(0,16,true);resized(16,24,false);shrink(16);let s=span.finish();
 assert_eq!(s.heap[5],0);assert_eq!(s.heap[7],2);
 let span=Span::start().unwrap();shrink(1);assert_eq!(span.finish().heap[8],1);
 let span=Span::start().unwrap();OPS[0].store(u64::MAX,Relaxed);hit(0);let s=span.finish();assert_eq!(s.heap[9],1);assert_eq!(s.ops[0],u64::MAX);
 assert!(measure(||Err("synthetic error")).is_err());assert!(!ACTIVE.load(Relaxed));
 hit(0);let(_,s)=measure(||Ok(0)).unwrap();assert_eq!(s.ops,[0;NAMES.len()]);assert_eq!(s.heap,[0;10]);
 let(v,s)=measure(||Ok(project(2,13))).unwrap();assert_eq!(v,8);assert_eq!(s.ops[0],4);assert_eq!(s.ops[7],1);
}
