use std::cell::{RefCell,Cell};
use std::alloc::{GlobalAlloc,Layout,System};
#[derive(Clone,Copy)]pub enum Stage{Off,Compile,Calibration,Prepare,Query,InitialGeometry,InitialTransport,InitialCharacter,Decode,Extension,ExtraGeometry,ExtraTransport,ExtraCharacter,Verify}
#[derive(Clone,Copy)]pub enum Component{Geometry,Phase,Value,Polynomial,Source,Decoder,Compiler,Target,Recurrence,Verifier,Allocator,Transport}
#[derive(Clone,Copy)]pub enum Op{Product,Inverse,ScalarInverse,EuclidStep,Popcount,RuleCompare,Character,Alloc,ZeroAlloc,Realloc,Free,RequestedBytes,FreedBytes}
pub const STAGES:[&str;14]=["off","compile","calibration","prepare","query","initial_geometry","initial_transport","initial_character","decode","extension","extra_geometry","extra_transport","extra_character","verify"];
pub const COMPONENTS:[&str;12]=["geometry","phase","value","polynomial","source","decoder","compiler","target","recurrence","verifier","allocator","transport"];
pub const OPS:[&str;13]=["products","inversions","scalar_inversions","euclid_steps","popcounts","rule_comparisons","characters","alloc_calls","zero_alloc_calls","realloc_calls","free_calls","requested_bytes","freed_bytes"];
pub type Counts=[[[u64;13];12];14];
struct State{stage:usize,counts:Counts,overflow:bool}
thread_local!{static MISSED:Cell<u64>=const{Cell::new(0)};static STATE:RefCell<State>=const{RefCell::new(State{stage:0,counts:[[[0;13];12];14],overflow:false})};}
pub fn add(c:Component,o:Op,n:u64){let _=STATE.try_with(|state|{if let Ok(mut s)=state.try_borrow_mut(){if s.stage!=0{let stage=s.stage;let cell=&mut s.counts[stage][c as usize][o as usize];match cell.checked_add(n){Some(v)=>*cell=v,None=>s.overflow=true}}}else{let _=MISSED.try_with(|m|m.set(m.get().saturating_add(1)));}});}
pub fn hit(c:Component,o:Op){add(c,o,1)}
pub fn value<T>(c:Component,o:Op,v:T)->T{hit(c,o);v}
pub struct Guard(usize);
pub fn enter(stage:Stage)->Guard{STATE.with(|s|{let mut s=s.borrow_mut();let old=s.stage;s.stage=stage as usize;Guard(old)})}
impl Drop for Guard{fn drop(&mut self){STATE.with(|s|s.borrow_mut().stage=self.0);}}
pub fn reset(){MISSED.with(|m|m.set(0));STATE.with(|s|{let mut s=s.borrow_mut();assert_eq!(s.stage,0);s.counts=[[[0;13];12];14];s.overflow=false;});}
pub fn snapshot()->Counts{MISSED.with(|m|assert_eq!(m.get(),0));STATE.with(|s|{let s=s.borrow();assert!(!s.overflow);s.counts})}
pub struct Counting;
unsafe impl GlobalAlloc for Counting{
 unsafe fn alloc(&self,l:Layout)->*mut u8{hit(Component::Allocator,Op::Alloc);add(Component::Allocator,Op::RequestedBytes,l.size()as u64);unsafe{System.alloc(l)}}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{hit(Component::Allocator,Op::ZeroAlloc);add(Component::Allocator,Op::RequestedBytes,l.size()as u64);unsafe{System.alloc_zeroed(l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{hit(Component::Allocator,Op::Realloc);add(Component::Allocator,Op::RequestedBytes,n as u64);unsafe{System.realloc(p,l,n)}}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){hit(Component::Allocator,Op::Free);add(Component::Allocator,Op::FreedBytes,l.size()as u64);unsafe{System.dealloc(p,l)}}
}

// Nested source phases retain the outer public setup bucket.
pub fn granular(stage:Stage)->Guard{let old=STATE.with(|s|s.borrow().stage);let stage=if old==Stage::Calibration as usize{Stage::Calibration}else if old==Stage::Prepare as usize{Stage::Prepare}else if old==Stage::Compile as usize{Stage::Compile}else{stage};enter(stage)}
