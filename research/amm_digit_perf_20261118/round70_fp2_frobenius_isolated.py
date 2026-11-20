#!/usr/bin/env python3
"""Native Rust source-exact A/B validation of Fp2 Frobenius powering."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
start = source.index("        // In Fp2, x^p = conjugate(x)")
end = source.index("        if e < p ||", start)
before = source[:start] + source[end:]
start = before.index("        // x^lo * conjugate(x)^hi =")
end = before.index("        // The top nonzero joint bit", start)
before = before[:start] + before[end:]
assert before != source and before.count("let difference = lo.abs_diff(hi)") == 0

head = r"""
#![allow(dead_code)]
mod cryptanalysis {
pub mod prime_root_trace {
 pub const MAX_PRIME:u64=(1u64<<61)-1;
 pub const MAX_WORK:u64=1_000_000;
 pub struct RootLimits {pub field_work:u64,pub split_attempts:u64,pub events:u64,pub degree:usize}
 #[derive(Debug, PartialEq, Eq)] pub enum RootStop {Complete}
 pub struct RootReport {pub stop:RootStop,pub roots:Vec<u64>}
 pub struct RootEvent;
 pub fn solve(_: &[u64],_:u64,_:u64,_:RootLimits,_:&mut dyn FnMut(&RootEvent)->Result<(),String>)
  ->Result<RootReport,String> {panic!("quartic fallback unused")}
}
pub mod residual_walk {pub fn is_prime_u64(_:u64)->bool {true}}
pub mod before {
"""
middle = "\n}\npub mod after {\n"
tail = "\n}\n}\n"
checks = r"""
#[cfg(test)]
mod checks {
use crate::cryptanalysis::{before,after};
use std::{hint::black_box,time::Instant};
fn next(s:&mut u64)->u64 {
 *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*s
}
fn pair(p:u64,n:u64)->(before::Fq2,after::Fq2) {
 (before::Fq2{f:before::Fp::new(p),n},after::Fq2{f:after::Fp::new(p),n})
}
fn call_old(q:&before::Fq2,x:(u64,u64),e:u128)->(u64,u64){
 use before::Ring;q.pow(x,e)
}
fn call_new(q:&after::Fq2,x:(u64,u64),e:u128)->(u64,u64){
 use after::Ring;q.pow(x,e)
}
fn median(mut v:Vec<f64>)->f64 {v.sort_by(|a,b|a.total_cmp(b));v[v.len()/2]}
#[test]
fn equivalence() {
 let mut count=0usize;
 for (p,n) in [(7u64,3u64),(11,2),(17,3),(1_000_000_007,5),((1u64<<61)-1,3)] {
  let (qb,qa)=pair(p,n);let mut state=p^0x70139efab328c210;
  for i in 0..128 {
   let a=next(&mut state)%p;let b=next(&mut state)%p;
   let x=match i%5 {0=>(0,0),1=>(a,0),2=>(0,b),_=>(a,b)};
   let hi=1+next(&mut state)%(p-1);
   let delta=1+(next(&mut state)%256);
   let lo=hi.saturating_add(delta).min(p-1);
   let hi2=lo.saturating_sub(delta).max(1);
   for e in [
    0u128,1,p as u128-1,2*(p as u128-1),
    (hi as u128)*(p as u128)+lo as u128,
    (hi2 as u128)*(p as u128)+lo as u128,
    ((p-1) as u128)*(p as u128),
   ] {
     let old=call_old(&qb,x,e);let new=call_new(&qa,x,e);
     assert_eq!(new,old,"p={p} x={x:?} e={e}");
     assert_eq!(new,after::rpow(&qa,x,e),"independent binary exponent p={p} e={e}");
     count+=1;
   }
  }
 }
 println!("ROUND70_EQ count={count}");
}
#[test]
#[ignore="13 alternating native Rust A/B trials"]
fn benchmark() {
 for (p,n) in [(1_000_000_007u64,5u64),((1u64<<61)-1,3)] {
  let (qb,qa)=pair(p,n);let mut state=p^0x70139efab328c210;
  for kind in 0..4usize {
   let mut inputs=Vec::new();
   for _ in 0..256 {
     let x=(1+next(&mut state)%(p-1),1+next(&mut state)%(p-1));
     let hi=1025+next(&mut state)%(p-2000);
     let delta=1+next(&mut state)%128;
     let e=match kind {
       0=>(p-1) as u128,
       1=>2*(p as u128-1),
       2=>(hi as u128)*(p as u128)+(hi+delta) as u128,
       _=>(next(&mut state) as u128 % (p as u128-1))*(p as u128)
         +(next(&mut state) as u128 % p as u128),
     };
     inputs.push((x,e));
   }
   let mut old=Vec::new();let mut new=Vec::new();
   for trial in 0..13 {
     let measure=|optimized:bool| {
       let begin=Instant::now();let mut check=0u64;
       for _ in 0..4 {
        for &(x,e) in &inputs {
         let v=if optimized {call_new(&qa,black_box(x),black_box(e))}
                     else {call_old(&qb,black_box(x),black_box(e))};
         check^=v.0.wrapping_add(v.1);
        }
       }
       black_box(check);
       begin.elapsed().as_nanos() as f64 / (inputs.len()*4) as f64
     };
     if trial%2==0 {old.push(measure(false));new.push(measure(true));}
     else {new.push(measure(true));old.push(measure(false));}
   }
   let a=median(old);let b=median(new);
   println!("ROUND70_RUST p={p} kind={kind} before_ns={a:.3} after_ns={b:.3} speedup={:.3}x",a/b);
  }
 }
}
}
"""
with tempfile.TemporaryDirectory() as directory:
 path = Path(directory) / "main.rs"
 path.write_text(head + before + middle + source + tail + checks)
 executable = Path(directory) / "root70-tests"
 subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(executable)],check=True)
 subprocess.run([str(executable),"--nocapture","--test-threads=1"],check=True)
 subprocess.run([str(executable),"--ignored","--nocapture","--test-threads=1"],check=True)
