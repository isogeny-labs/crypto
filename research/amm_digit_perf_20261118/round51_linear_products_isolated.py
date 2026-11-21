#!/usr/bin/env python3
"""Round 51: compile exact Rust root producer and replay, compare against R50 code."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source
for section in ("PRODUCER_PRODUCT", "REPLAY_CONVOLUTION"):
    before, n = re.subn(
        rf"(?m)^ *// R51_{section}_BEGIN\n.*?^ *// R51_{section}_END\n",
        "", before, count=1, flags=re.S,
    )
    assert n == 1, f"missing R51_{section} markers"

header = """#![allow(dead_code)]
mod cryptanalysis {
pub mod fp_backend {
 pub struct Fp64Arithmetic {pub p:u64}
 impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
}
pub mod bounded {
pub mod after {
"""
middle = "\n} pub mod before {\n"
footer = "\n}\n}\n}\n"
helper = r"""
/// Exercise only the exact production module's polynomial operations.
pub fn kernel(p:u64, size:usize, mode:u8, repeats:usize, verifier:bool)->(u64,u64){
 use std::time::Duration;
 let limit=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,bytes:STORAGE_BYTES,
  events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(120);
 let mut observer=|_:&BoundedRootEvent|Ok(());
 let mut producer=Producer{p,gate:Gate::new(limit,deadline),observer:&mut observer,
  unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
  roots:[0;8],root_count:0};
 producer.gate.admit().unwrap();
 let field=super::super::fp_backend::Fp64Arithmetic{p};
 let mut replay=Replay::new(&field,&[1,1],42,limit,limit,deadline).unwrap();
 let mut poly=Poly17::ZERO;
 poly.len=size as u8;
 for k in 0..size{poly.coefficients[k]=((k as u64)*37+11)%p;}
 poly.coefficients[size-1]=1;
 let mut factor=Poly17::ZERO;
 if mode==2 || mode==3 {
  factor.len=1;factor.coefficients[0]=if mode==2{1}else{2};
 }else{
  factor.len=2;factor.coefficients[0]=if mode==1{0}else{37};
  factor.coefficients[1]=1;
 }
 let mut digest=0u64;
 for i in 0..repeats{
  let value=if verifier{replay.convolution(&poly,&factor).unwrap()}
                else{producer.product(&poly,&factor).unwrap()};
  digest=digest.wrapping_add(value.coefficients[i%(value.len as usize)]);
 }
 (digest,if verifier{replay.gate.metrics.field_work}else{producer.gate.metrics.field_work})
}
"""
tests = r"""
#[cfg(test)]
mod round51 {
 use super::cryptanalysis::{fp_backend::Fp64Arithmetic,bounded::{after,before}};
 use std::{hint::black_box,time::{Instant,Duration}};
 fn budget_after()->after::RootBudget {
  after::RootBudget{work:after::MAX_WORK,field_work:after::MAX_FIELD_WORK,
    bytes:after::STORAGE_BYTES,events:after::MAX_EVENTS,split_attempts:1000,degree:8}
 }
 fn budget_before()->before::RootBudget {
  before::RootBudget{work:before::MAX_WORK,field_work:before::MAX_FIELD_WORK,
    bytes:before::STORAGE_BYTES,events:before::MAX_EVENTS,split_attempts:1000,degree:8}
 }
 fn polynomial(p:u64,degree:usize)->Vec<u64>{
  let mut result=vec![1u64];
  for root in 1..=degree as u64 {
   let mut next=vec![0;result.len()+1];
   for (i,&value) in result.iter().enumerate(){
    next[i]=((next[i] as u128+value as u128*((p-root)%p) as u128)%p as u128) as u64;
    next[i+1]=(next[i+1]+value)%p;
   }
   result=next;
  }
  result
 }
 fn old(p:u64,f:&[u64])->(Vec<u64>,Vec<String>,u64,u64){
  let field=Fp64Arithmetic{p};let limits=budget_before();
  let deadline=Instant::now()+Duration::from_secs(120);
  let mut events=Vec::new();
  let report=before::solve_bounded(&field,black_box(f),42,limits,deadline,&mut |e|{
   events.push(*e);Ok(())});
  assert_eq!(report.stop,before::Stop::Complete);
  let mut replay=before::Replay::new(&field,f,42,limits,limits,deadline).unwrap();
  for e in &events{replay.push(e).unwrap();}
  let checked=replay.finish(&report).unwrap();assert!(checked.finite_complete);
  (report.roots[..report.root_count as usize].to_vec(),
   events.iter().map(|e|format!("{e:?}")).collect(),
   report.metrics.field_work,checked.replay_metrics.field_work)
 }
 fn new(p:u64,f:&[u64])->(Vec<u64>,Vec<String>,u64,u64){
  let field=Fp64Arithmetic{p};let limits=budget_after();
  let deadline=Instant::now()+Duration::from_secs(120);
  let mut events=Vec::new();
  let report=after::solve_bounded(&field,black_box(f),42,limits,deadline,&mut |e|{
   events.push(*e);Ok(())});
  assert_eq!(report.stop,after::Stop::Complete);
  let mut replay=after::Replay::new(&field,f,42,limits,limits,deadline).unwrap();
  for e in &events{replay.push(e).unwrap();}
  let checked=replay.finish(&report).unwrap();assert!(checked.finite_complete);
  (report.roots[..report.root_count as usize].to_vec(),
   events.iter().map(|e|format!("{e:?}")).collect(),
   report.metrics.field_work,checked.replay_metrics.field_work)
 }
 fn median(mut data:Vec<f64>)->f64{data.sort_by(f64::total_cmp);data[data.len()/2]}
 fn timed_kernel(p:u64,mode:u8,verify:bool,old_version:bool)->f64{
  let start=Instant::now();
  let result=if old_version{before::kernel(p,8,mode,384,verify)}
             else{after::kernel(p,8,mode,384,verify)};
  black_box(result);
  start.elapsed().as_nanos() as f64/384.0
 }
 #[test]
 fn arithmetic_and_complete_event_certificate_equivalence(){
  let mut checked=0;
  for p in [1_000_000_007u64,1_000_000_009,(1u64<<61)-1]{
   for size in [4usize,8]{
    for mode in 0..4u8{
     for verify in [false,true]{
      let a=before::kernel(p,size,mode,384,verify);
      let b=after::kernel(p,size,mode,384,verify);
      assert_eq!(a.0,b.0,"p={p} size={size} mode={mode} replay={verify}");
      checked+=1;
     }
    }
   }
   for degree in [2usize,3,4,6,8]{
    let f=polynomial(p,degree);let a=old(p,&f);let b=new(p,&f);
    assert_eq!(a.0,b.0,"p={p} degree={degree}");
    assert_eq!(a.1,b.1,"p={p} degree={degree}");
    checked+=1;
   }
  }
  println!("ROUND51_NATIVE_EQUIVALENCE checked={checked}");
 }
 #[test]
 #[ignore="13 alternating Rust A/B trials: production producer and verifier"]
 fn bench_kernels_and_complete_root_extraction(){
  for p in [1_000_000_007u64,1_000_000_009,(1u64<<61)-1]{
   for mode in 0..4u8{
    for verify in [false,true]{
     let before_count=before::kernel(p,8,mode,384,verify).1;
     let after_count=after::kernel(p,8,mode,384,verify).1;
     let mut prev=Vec::new();let mut opt=Vec::new();
     for trial in 0..13{
      if trial%2==0{
       prev.push(timed_kernel(p,mode,verify,true));
       opt.push(timed_kernel(p,mode,verify,false));
      }else{
       opt.push(timed_kernel(p,mode,verify,false));
       prev.push(timed_kernel(p,mode,verify,true));
      }
     }
     let x=median(prev);let y=median(opt);
     println!("ROUND51_NATIVE_KERNEL p={p} mode={mode} verifier={verify} before_ns={x:.2} after_ns={y:.2} speedup={:.3} field_before={before_count} field_after={after_count}",x/y);
    }
   }
   for degree in [4usize,8]{
    let f=polynomial(p,degree);let a=old(p,&f);let b=new(p,&f);
    assert_eq!(a.0,b.0);assert_eq!(a.1,b.1);
    let mut prev=Vec::new();let mut opt=Vec::new();
    for trial in 0..13{
     if trial%2==0{
      let t=Instant::now();black_box(old(p,&f));prev.push(t.elapsed().as_nanos() as f64);
      let t=Instant::now();black_box(new(p,&f));opt.push(t.elapsed().as_nanos() as f64);
     }else{
      let t=Instant::now();black_box(new(p,&f));opt.push(t.elapsed().as_nanos() as f64);
      let t=Instant::now();black_box(old(p,&f));prev.push(t.elapsed().as_nanos() as f64);
     }
    }
    let x=median(prev);let y=median(opt);
    println!("ROUND51_NATIVE_COMPLETE p={p} degree={degree} before_ns={x:.1} after_ns={y:.1} speedup={:.3} producer_field={}:{} replay_field={}:{}",x/y,a.2,b.2,a.3,b.3);
   }
  }
 }
}
"""
with tempfile.TemporaryDirectory() as tmp:
 p=Path(tmp)/"round51.rs";exe=Path(tmp)/"round51"
 p.write_text(header+source+helper+middle+before+helper+footer+tests)
 subprocess.run(["rustc","--edition=2021","-O","--test",str(p),"-o",str(exe)],check=True)
 subprocess.run([str(exe),"arithmetic_and_complete_event_certificate_equivalence","--nocapture"],check=True)
 subprocess.run([str(exe),"bench_kernels_and_complete_root_extraction","--ignored","--nocapture"],check=True)
