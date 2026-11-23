#!/usr/bin/env python3
"""Round 71: source-exact native Rust A/B for sparse Fp2 arithmetic.
The frozen baseline is obtained by deleting ONLY the three guarded fast paths
from the production source. Both variants compile as ordinary Rust modules.
"""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]

fast_paths = [
"""        // Multiplication by an Fp scalar takes two base-field products.
        // Preserve the fast dense Karatsuba formula for generic inputs.
        if x.1 == 0 {
            return (f.mul(x.0, y.0), f.mul(x.0, y.1));
        }
        if y.1 == 0 {
            return (f.mul(x.0, y.0), f.mul(x.1, y.0));
        }
""",
"""        // Sparse squares lie in Fp. Avoid three full products.
        if x.1 == 0 {
            return (f.sqr(x.0), 0);
        }
        if x.0 == 0 {
            return (self.mul_nonresidue(f.sqr(x.1)), 0);
        }
""",
"""        // Pure Fp elements cube with only one square and one product.
        // Do not branch on imaginary-only elements: local dense controls
        // showed worse throughput with that additional check.
        if x.1 == 0 {
            return (f.mul(f.sqr(x.0), x.0), 0);
        }
""",
]
before = source
for patch in fast_paths:
    assert before.count(patch) == 1, "source changed: expected exactly one fast path"
    before = before.replace(patch, "")

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
checks = r"""
#[cfg(test)]
mod checks {
use crate::cryptanalysis::{before,after};
use std::{hint::black_box,time::Instant};

fn next(s:&mut u64)->u64 {
 *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
 *s
}
fn pair(p:u64,n:u64)->(before::Fq2,after::Fq2) {
 (before::Fq2{f:before::Fp::new(p),n},after::Fq2{f:after::Fp::new(p),n})
}
fn baseline(q:&before::Fq2,x:(u64,u64),y:(u64,u64),op:usize)->(u64,u64){
 match op {
  0=>before::Ring::mul(q,x,y),
  1=>before::Ring::sqr(q,x),
  _=>before::Ring::cube(q,x),
 }
}
fn candidate(q:&after::Fq2,x:(u64,u64),y:(u64,u64),op:usize)->(u64,u64){
 match op {
  0=>after::Ring::mul(q,x,y),
  1=>after::Ring::sqr(q,x),
  _=>after::Ring::cube(q,x),
 }
}
fn inputs(p:u64,kind:usize,count:usize)->Vec<((u64,u64),(u64,u64))> {
 let mut state=p^0x710fb7a4fe87a0c1u64^((kind as u64)<<32);
 (0..count).map(|i| {
  let mut x=(1+next(&mut state)%(p-1),1+next(&mut state)%(p-1));
  let mut y=(1+next(&mut state)%(p-1),1+next(&mut state)%(p-1));
  match kind {
   1=>x.1=0,
   2=>y.1=0,
   3=>x.0=0,
   4=>y.0=0,
   5=>match i%4 {0=>x.1=0,1=>y.1=0,2=>x.0=0,_=>y.0=0},
   6=>{x.1=0;y.1=0},
   7=>{x.0=0;y.0=0},
   _=>{}
  }
  (x,y)
 }).collect()
}
fn median(mut x:Vec<f64>)->f64 {x.sort_by(|a,b|a.total_cmp(b));x[x.len()/2]}
#[test]
fn exact_arithmetic_equivalence() {
 let mut count=0;
 // Correct extension nonsquares: first for p=7 is 3, for p=2^61-1 is 3.
 for (p,n) in [(5,2),(7,3),(11,2),(17,3),(97,5),
    (1_000_000_007u64,5),((1u64<<61)-1,3)] {
  let (qb,qa)=pair(p,n);
  for kind in 0..8 {
   for (x,y) in inputs(p,kind,256) {
    for op in 0..3 {
     let a=baseline(&qb,x,y,op);
     let b=candidate(&qa,x,y,op);
     assert_eq!(a,b,"p={p} n={n} kind={kind} op={op} x={x:?} y={y:?}");
     count+=1;
    }
   }
  }
 }
 println!("ROUND71_EQ count={count}");
}
#[test]
fn full_cubic_equivalence() {
 let mut count=0usize;
 for p in [1_000_000_007u64,(1u64<<61)-1] {
  let cb=before::RadicalRootCtx::new(p).unwrap();
  let ca=after::RadicalRootCtx::new(p).unwrap();
  let mut s=p^0x7171742a5f3a89be;
  for _ in 0..128 {
   let a=next(&mut s)%p;let b=next(&mut s)%p;
   let mut old=cb.cubic_depressed(a,b);
   let mut new=ca.cubic_depressed(a,b);
   old.sort_unstable();old.dedup();new.sort_unstable();new.dedup();
   assert_eq!(old,new,"p={p} a={a} b={b}");
   count+=1;
  }
 }
 println!("ROUND71_FULL_EQ count={count}");
}
#[test]
#[ignore="13 alternating A/B trials on production Rust source"]
fn bench_kernels() {
 for (p,n) in [(1_000_000_007u64,5u64),((1u64<<61)-1,3)] {
  let (qb,qa)=pair(p,n);
  for op in 0..3 {
   for kind in [0usize,1,2,3,5,6,7] {
    let v=inputs(p,kind,256);
    let mut before_trials=Vec::new();
    let mut after_trials=Vec::new();
    for trial in 0..13 {
     let measure=|is_new:bool| {
      let t=Instant::now();let mut checksum=0u64;
      for _ in 0..64 {
       for &(x,y) in &v {
        let out=if is_new {candidate(&qa,black_box(x),black_box(y),op)}
                 else {baseline(&qb,black_box(x),black_box(y),op)};
        checksum ^= out.0.wrapping_add(out.1);
       }
      }
      black_box(checksum);
      t.elapsed().as_nanos() as f64/(v.len()*64) as f64
     };
     if trial%2==0 {
      before_trials.push(measure(false));
      after_trials.push(measure(true));
     } else {
      after_trials.push(measure(true));
      before_trials.push(measure(false));
     }
    }
    let a=median(before_trials);let b=median(after_trials);
    println!("ROUND71_RUST p={p} op={op} kind={kind} before_ns={a:.3} after_ns={b:.3} speedup={:.3}x",a/b);
   }
  }
 }
}
#[test]
#[ignore="paired full cubic medians"]
fn bench_full_cubic() {
 for p in [1_000_000_007u64,(1u64<<61)-1] {
  let cb=before::RadicalRootCtx::new(p).unwrap();
  let ca=after::RadicalRootCtx::new(p).unwrap();
  let mut s=p^0x7171742a5f3a89be;
  let v:Vec<_>=(0..128).map(|_|(next(&mut s)%p,next(&mut s)%p)).collect();
  let mut old=Vec::new();let mut new=Vec::new();
  for trial in 0..13 {
   let measure=|is_new:bool| {
    let t=Instant::now();
    for _ in 0..3 {
     for &(a,b) in &v {
      let roots=if is_new {ca.cubic_depressed(black_box(a),black_box(b))}
        else {cb.cubic_depressed(black_box(a),black_box(b))};
      black_box(roots);
     }
    }
    t.elapsed().as_nanos() as f64/(v.len()*3) as f64
   };
   if trial%2==0 {old.push(measure(false));new.push(measure(true));}
   else {new.push(measure(true));old.push(measure(false));}
  }
  let a=median(old);let b=median(new);
  println!("ROUND71_FULL p={p} before_ns={a:.3} after_ns={b:.3} speedup={:.3}x",a/b);
 }
}
}
"""

with tempfile.TemporaryDirectory() as directory:
    path = Path(directory) / "round71.rs"
    path.write_text(head + before + "\n}\npub mod after {\n" + source + "\n}\n}\n" + checks)
    exe = Path(directory) / "round71-tests"
    subprocess.run(["rustc", "--edition=2021", "-O", "--test", str(path), "-o", str(exe)], check=True)
    subprocess.run([str(exe), "--nocapture", "--test-threads=1"], check=True)
    subprocess.run([str(exe), "--ignored", "--nocapture", "--test-threads=1"], check=True)
