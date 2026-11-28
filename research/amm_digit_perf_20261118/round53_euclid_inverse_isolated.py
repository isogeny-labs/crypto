#!/usr/bin/env python3
"""Round 53: native Rust before/after bounded root-finder Euclid and constant division."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
old_inverse = """    fn inverse(&mut self, value: u64) -> Result<u64> {
        if value == 0 { return Err(Stop::InvalidInput); }
        self.gate.charge(32)?;
        if value == 1 { return Ok(1); }
        let (mut base, mut result, mut exponent) = (value, 1, self.p - 2);
        while exponent != 0 {
            self.gate.charge(32)?;
            if exponent & 1 != 0 { result = self.mul(result, base)?; }
            exponent >>= 1;
            if exponent != 0 { base = self.mul(base, base)?; }
        }
        Ok(result)
    }
"""
old_reciprocal = """    fn reciprocal(&mut self, x: u64) -> Result<u64> {
        if x == 0 { return Err(Stop::InvalidCertificate); }
        self.gate.charge(32)?;
        if x == 1 { return Ok(1); }
        let exponent = self.p - 2;
        let mut acc = 1;
        for bit in (0..64 - exponent.leading_zeros()).rev() {
            self.gate.charge(32)?;
            acc = self.times(acc, acc)?;
            if exponent & (1u64 << bit) != 0 { acc = self.times(acc, x)?; }
        }
        Ok(acc)
    }
"""
before = source
for section, replacement in (
    ("PRODUCER_INVERSE", old_inverse),
    ("REPLAY_RECIPROCAL", old_reciprocal),
    ("CONSTANT_DIVISION", ""),
):
    before, count = re.subn(
        rf"(?m)^ *// R53_{section}_BEGIN\n.*?^ *// R53_{section}_END\n",
        lambda match: replacement,
        before, count=1, flags=re.S)
    assert count == 1, f"missing R53 {section} markers"
header = """#![allow(dead_code)]
mod cryptanalysis {
pub mod fp_backend {
 pub struct Fp64Arithmetic { pub p: u64 }
 impl Fp64Arithmetic { pub fn p(&self)->u64 { self.p } }
}
pub mod bounded { pub mod after {
"""
middle = "\n} pub mod before {\n"
footer = "\n} } }\n"
helper = r"""
pub fn inverse_kernel(p:u64, kind:u8, repetitions:usize)->(u64,u64) {
 use std::time::Duration;
 let limits=RootBudget {work:MAX_WORK,field_work:MAX_FIELD_WORK,bytes:STORAGE_BYTES,
    events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(180);
 let field=super::super::fp_backend::Fp64Arithmetic{p};
 let mut observer=|_:&BoundedRootEvent|Ok(());
 let mut producer=Producer{p,gate:Gate::new(limits,deadline),observer:&mut observer,
    unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
    roots:[0;8],root_count:0};
 producer.gate.admit().unwrap();
 let mut replay=Replay::new(&field,&[1,1],42,limits,limits,deadline).unwrap();
 let mut digest=0u64;
 for i in 0..repetitions {
    // Odd/even and fixed-value controls exercise a mix of Euclid paths.
    let value=((i as u64).wrapping_mul(1597334677).wrapping_add(37) % (p-1))+1;
    let v=match kind {
      0=>producer.inverse(value).unwrap(),
      1=>replay.reciprocal(value).unwrap(),
      2=>{
       let mut numerator=Poly17::ZERO;
       numerator.len=9;
       for j in 0..9 {numerator.coefficients[j]=(value+j as u64+1)%p;}
       let divisor=Poly17::constant(value);
       let (q,r)=producer.division(&numerator,&divisor).unwrap();
       assert_eq!(r,Poly17::ZERO);
       q.coefficients[0]
      },
      _=>panic!("kind"),
    };
    digest=digest.wrapping_add(v);
 }
 let fops=if kind==1 {replay.gate.metrics.field_work}
          else {producer.gate.metrics.field_work};
 (digest,fops)
}
"""
tests = r"""
#[cfg(test)]
mod round53 {
 use super::cryptanalysis::{fp_backend::Fp64Arithmetic,bounded::{after,before}};
 use std::{hint::black_box,time::{Instant,Duration}};
 fn limit_after()->after::RootBudget {after::RootBudget{
   work:after::MAX_WORK,field_work:after::MAX_FIELD_WORK,bytes:after::STORAGE_BYTES,
   events:after::MAX_EVENTS,split_attempts:1000,degree:8}}
 fn limit_before()->before::RootBudget {before::RootBudget{
   work:before::MAX_WORK,field_work:before::MAX_FIELD_WORK,bytes:before::STORAGE_BYTES,
   events:before::MAX_EVENTS,split_attempts:1000,degree:8}}
 fn polynomial(p:u64,degree:usize,leading:u64)->Vec<u64>{
  let mut result=vec![leading];
  for root in 1..=degree as u64 {
   let mut next=vec![0u64;result.len()+1];
   for (i,&value) in result.iter().enumerate(){
     next[i]=((u128::from(next[i])+u128::from(value)*u128::from(p-root))%u128::from(p)) as u64;
     next[i+1]=(next[i+1]+value)%p;
   }
   result=next;
  }
  result
 }
 fn before_full(p:u64,polynomial:&[u64])->(Vec<u64>,Vec<String>,u64,u64){
  let field=Fp64Arithmetic{p};let limits=limit_before();let deadline=Instant::now()+Duration::from_secs(120);
  let mut events=Vec::new();
  let report=before::solve_bounded(&field,black_box(polynomial),42,limits,deadline,&mut |e|{events.push(*e);Ok(())});
  assert_eq!(report.stop,before::Stop::Complete,"before p={p}");
  let mut verifier=before::Replay::new(&field,polynomial,42,limits,limits,deadline).unwrap();
  for e in &events {verifier.push(e).unwrap();}
  let checked=verifier.finish(&report).unwrap();assert!(checked.finite_complete);
  (report.roots[..report.root_count as usize].to_vec(),
   events.iter().map(|e|format!("{e:?}")).collect(),
   report.metrics.field_work,checked.replay_metrics.field_work)
 }
 fn after_full(p:u64,polynomial:&[u64])->(Vec<u64>,Vec<String>,u64,u64){
  let field=Fp64Arithmetic{p};let limits=limit_after();let deadline=Instant::now()+Duration::from_secs(120);
  let mut events=Vec::new();
  let report=after::solve_bounded(&field,black_box(polynomial),42,limits,deadline,&mut |e|{events.push(*e);Ok(())});
  assert_eq!(report.stop,after::Stop::Complete,"after p={p}");
  let mut verifier=after::Replay::new(&field,polynomial,42,limits,limits,deadline).unwrap();
  for e in &events {verifier.push(e).unwrap();}
  let checked=verifier.finish(&report).unwrap();assert!(checked.finite_complete);
  (report.roots[..report.root_count as usize].to_vec(),
   events.iter().map(|e|format!("{e:?}")).collect(),
   report.metrics.field_work,checked.replay_metrics.field_work)
 }
 fn median(mut x:Vec<f64>)->f64{x.sort_by(f64::total_cmp);x[x.len()/2]}
 fn timed_kernel(p:u64,kind:u8,old:bool)->f64{
  let t=Instant::now();
  let result=if old{before::inverse_kernel(p,kind,128)}else{after::inverse_kernel(p,kind,128)};
  black_box(result);
  t.elapsed().as_nanos() as f64/128.0
 }
 #[test]
 fn inverse_and_complete_event_stream_equivalence(){
  let mut count=0;
  for p in [5u64,7,17,1_000_000_007,1_000_000_009,1_000_000_021,(1u64<<61)-1] {
    for kind in 0..3 {
      let old=before::inverse_kernel(p,kind,128);
      let new=after::inverse_kernel(p,kind,128);
      assert_eq!(old.0,new.0,"p={p} kind={kind}");
      count+=128;
    }
    for degree in [2usize,4,8] {
      if degree >= p as usize {continue;}
      for leading in [1u64,3] {
        let polynomial=polynomial(p,degree,leading);
        let old=before_full(p,&polynomial);
        let new=after_full(p,&polynomial);
        assert_eq!(old.0,new.0,"roots p={p} degree={degree}");
        assert_eq!(old.1,new.1,"events p={p} degree={degree}");
        count+=1;
      }
    }
  }
  println!("ROUND53_NATIVE_EQUIVALENCE checks={count}");
 }
 #[test]
 #[ignore="13 alternating native Rust A/B timings"]
 fn bench_inversions_and_complete_roots(){
  for p in [1_000_000_007u64,1_000_000_009,(1u64<<61)-1] {
   for kind in 0..3 {
    let old=before::inverse_kernel(p,kind,128).1;
    let new=after::inverse_kernel(p,kind,128).1;
    let mut prev=Vec::new();let mut opt=Vec::new();
    for i in 0..13 {
      if i%2==0 {
        prev.push(timed_kernel(p,kind,true));
        opt.push(timed_kernel(p,kind,false));
      } else {
        opt.push(timed_kernel(p,kind,false));
        prev.push(timed_kernel(p,kind,true));
      }
    }
    let x=median(prev);let y=median(opt);
    println!("ROUND53_NATIVE_KERNEL p={p} kind={kind} before_ns={x:.2} after_ns={y:.2} speedup={:.3} field_before={old} field_after={new}",x/y);
   }
   for degree in [4usize,8] {
    for leading in [1u64,3] {
     let f=polynomial(p,degree,leading);
     let a=before_full(p,&f);let b=after_full(p,&f);
     assert_eq!(a.0,b.0);assert_eq!(a.1,b.1);
     let mut prev=Vec::new();let mut opt=Vec::new();
     for i in 0..13 {
      if i%2==0 {
       let t=Instant::now();black_box(before_full(p,&f));prev.push(t.elapsed().as_nanos() as f64);
       let t=Instant::now();black_box(after_full(p,&f));opt.push(t.elapsed().as_nanos() as f64);
      } else {
       let t=Instant::now();black_box(after_full(p,&f));opt.push(t.elapsed().as_nanos() as f64);
       let t=Instant::now();black_box(before_full(p,&f));prev.push(t.elapsed().as_nanos() as f64);
      }
     }
     let x=median(prev);let y=median(opt);
     println!("ROUND53_NATIVE_COMPLETE p={p} degree={degree} leading={leading} before_ns={x:.1} after_ns={y:.1} speedup={:.3} producer_field={}:{} replay_field={}:{}",x/y,a.2,b.2,a.3,b.3);
    }
   }
  }
 }
}
"""
with tempfile.TemporaryDirectory() as tmp:
    rust = Path(tmp)/"round53.rs"
    exe = Path(tmp)/"round53"
    rust.write_text(header+source+helper+middle+before+helper+footer+tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(rust),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"inverse_and_complete_event_stream_equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"bench_inversions_and_complete_roots","--ignored","--nocapture"],check=True)
