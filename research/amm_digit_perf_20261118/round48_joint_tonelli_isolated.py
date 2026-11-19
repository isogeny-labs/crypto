#!/usr/bin/env python3
"""Round 48: compare cached joint Tonelli powers to R47 and measure complete root extraction and replay."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source
for label, replacement in [
    ("PRODUCER", """            let q = (self.p - 1) / 8;
            let mut c = self.split_scalar_pow(11, q)?;
            let mut t = self.split_scalar_pow(discriminant, q)?;
            let mut r = self.split_scalar_pow(discriminant, (q + 1) / 2)?;
"""),
    ("REPLAY", """            let odd = (self.p - 1) >> 3;
            let mut root = self.split_character_power(discriminant, (odd + 1) / 2)?;
            let mut residue = self.split_character_power(discriminant, odd)?;
            let mut nonresidue = self.split_character_power(11, odd)?;
"""),
]:
    before, count = re.subn(
        r"(?m)^ *// R48_TONELLI_" + label + r"_BEGIN\n.*?^ *// R48_TONELLI_" + label + r"_END\n",
        replacement, before, count=1, flags=re.S,
    )
    assert count == 1, f"missing R48 marker {label}"
assert "R48_TONELLI_" not in before
assert "R48_TONELLI_" in source

header = """#![allow(dead_code)]
mod cryptanalysis {
pub mod fp_backend {
 pub struct Fp64Arithmetic { pub p:u64 }
 impl Fp64Arithmetic { pub fn p(&self)->u64 {self.p} }
}
pub mod bounded {
pub mod after {
"""
middle = "\n} pub mod before {\n"
footer = "\n}\n}\n}\n"
tests = r"""
#[cfg(test)]
mod round48_tests {
use super::cryptanalysis::{fp_backend::Fp64Arithmetic,bounded::{after,before}};
use std::{hint::black_box,time::{Instant,Duration}};
fn field(p:u64)->Fp64Arithmetic { Fp64Arithmetic{p} }
fn budget_after()->after::RootBudget {
 after::RootBudget{work:after::MAX_WORK,field_work:after::MAX_FIELD_WORK,
 bytes:after::STORAGE_BYTES,events:after::MAX_EVENTS,split_attempts:1000,degree:8}
}
fn budget_before()->before::RootBudget {
 before::RootBudget{work:before::MAX_WORK,field_work:before::MAX_FIELD_WORK,
 bytes:before::STORAGE_BYTES,events:before::MAX_EVENTS,split_attempts:1000,degree:8}
}
fn polynomial(p:u64, roots:&[u64], leading:u64)->Vec<u64>{
 let mut poly=vec![leading];
 for &root in roots {
  let mut next=vec![0;poly.len()+1];
  for (j,&c) in poly.iter().enumerate(){
   next[j]=((next[j] as u128+c as u128*((p-root)%p) as u128)%p as u128) as u64;
   next[j+1]=(next[j+1]+c)%p;
  }
  poly=next;
 }
 poly
}
fn old(p:u64, coeff:&[u64], seed:u64)->(Vec<u64>,Vec<String>,u64,u64){
 let f=field(p);let deadline=Instant::now()+Duration::from_secs(120);
 let b=budget_before();let mut events=Vec::new();
 let report=before::solve_bounded(&f,black_box(coeff),seed,b,deadline,&mut |e|{events.push(*e);Ok(())});
 assert_eq!(report.stop,before::Stop::Complete,"baseline p={p} degree={} stop={:?}",coeff.len()-1,report.stop);
 let mut replay=before::Replay::new(&f,coeff,seed,b,b,deadline).unwrap();
 for e in &events {replay.push(e).unwrap();}
 let checked=replay.finish(&report).unwrap();assert!(checked.finite_complete);
 (report.roots[..report.root_count as usize].to_vec(),
  events.iter().map(|e|format!("{e:?}")).collect(),
  report.metrics.field_work,checked.replay_metrics.field_work)
}
fn new(p:u64, coeff:&[u64], seed:u64)->(Vec<u64>,Vec<String>,u64,u64){
 let f=field(p);let deadline=Instant::now()+Duration::from_secs(120);
 let b=budget_after();let mut events=Vec::new();
 let report=after::solve_bounded(&f,black_box(coeff),seed,b,deadline,&mut |e|{events.push(*e);Ok(())});
 assert_eq!(report.stop,after::Stop::Complete,"optimized p={p} degree={} stop={:?}",coeff.len()-1,report.stop);
 let mut replay=after::Replay::new(&f,coeff,seed,b,b,deadline).unwrap();
 for e in &events {replay.push(e).unwrap();}
 let checked=replay.finish(&report).unwrap();assert!(checked.finite_complete);
 (report.roots[..report.root_count as usize].to_vec(),
  events.iter().map(|e|format!("{e:?}")).collect(),
  report.metrics.field_work,checked.replay_metrics.field_work)
}
#[test]
fn frozen_event_and_certificate_comparisons() {
 let mut count=0;
 for p in [1_000_000_007u64,1_000_000_009,1_000_000_021,(1u64<<61)-1] {
  for degree in [2usize,3,4,6,8] {
   let roots:Vec<u64>=(1..=degree as u64).collect();
   let f=polynomial(p,&roots,1);
   for seed in [0,42,0x5291ae77] {
    let x=old(p,&f,seed);let y=new(p,&f,seed);
    assert_eq!(x.0,y.0,"roots p={p} degree={degree} seed={seed}");
    assert_eq!(x.1,y.1,"events p={p} degree={degree} seed={seed}");
    count+=1;
   }
  }
 }
 println!("ROUND48_EXACT_COMPLETE count={count}");
}
#[test]
fn randomized_split_quadratics_match_baseline(){
 let mut comparisons=0;
 for p in [1_000_000_009u64]{
  for j in 1..=80u64{
   let a=(j*17+5)%p;let b=(j*77+13)%p;
   if a==b {continue;}
   for leading in [1,p-2] {
    let f=polynomial(p,&[a,b],leading);
    for seed in [0,42,0x5291ae77] {
     let x=old(p,&f,seed);let y=new(p,&f,seed);
     assert_eq!(x.0,y.0,"p={p} r={a},{b}");
     assert_eq!(x.1,y.1,"event stream p={p} r={a},{b}");
     comparisons+=1;
    }
   }
  }
 }
 println!("ROUND48_RANDOM_QUADRATICS checked={comparisons}");
}
fn median(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);v[v.len()/2]}
fn timed(p:u64,f:&[u64], baseline:bool, reps:usize)->f64{
 let start=Instant::now();let mut digest=0u64;
 for _ in 0..reps {
  let v=if baseline {old(p,f,42)} else {new(p,f,42)};
  digest^=v.2^v.3^v.0.first().copied().unwrap_or(0);
 }
 black_box(digest);
 start.elapsed().as_nanos() as f64/reps as f64
}
#[test]
#[ignore="13 A/B trials, R47 baseline vs R48, complete solve plus independent replay"]
fn bench_complete_root_extraction_and_replay(){
 for p in [1_000_000_009u64,1_000_000_007,1_000_000_021]{
  for degree in [2usize,4,6,8]{
   let roots:Vec<u64>=(1..=degree as u64).collect();
   let f=polynomial(p,&roots,1);
   let a=old(p,&f,42);let b=new(p,&f,42);
   assert_eq!(a.0,b.0);assert_eq!(a.1,b.1);
   let mut prior=Vec::new();let mut optimized=Vec::new();
   for trial in 0..13{
    if trial%2==0{
     prior.push(timed(p,&f,true,2));optimized.push(timed(p,&f,false,2));
    }else{
     optimized.push(timed(p,&f,false,2));prior.push(timed(p,&f,true,2));
    }
   }
   let x=median(prior);let y=median(optimized);
   println!("ROUND48_NATIVE_COMPLETE p={p} degree={degree} before_ns={x:.1} after_ns={y:.1} speedup={:.4}x producer_field={}:{} replay_field={}:{}",x/y,a.2,b.2,a.3,b.3);
  }
 }
}
}
"""
with tempfile.TemporaryDirectory() as tmp:
    p=Path(tmp)/"round48.rs"; exe=Path(tmp)/"round48"
    p.write_text(header+source+middle+before+footer+tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(p),"-o",str(exe)],check=True)
    for name in ["frozen_event_and_certificate_comparisons","randomized_split_quadratics_match_baseline"]:
        subprocess.run([str(exe),name,"--nocapture"],check=True)
    subprocess.run([str(exe),"bench_complete_root_extraction_and_replay","--ignored","--nocapture"],check=True)
