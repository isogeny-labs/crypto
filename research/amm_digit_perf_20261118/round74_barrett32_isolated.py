#!/usr/bin/env python3
"""Round 73: source-exact native Rust A/B for single-fold Mersenne Fp.
The frozen baseline restores ONLY the old extra Mersenne reduction fold
in production Fp::mul. Both variants compile as ordinary Rust modules.
"""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]

# Freeze only the old 32-bit modulo division; keep the shared initialization.
new_code = """            let product = a * b;
            let quotient = ((product as u128 * self.reciprocal32 as u128) >> 64) as u64;
            let remainder = product - quotient * self.p;
            if remainder >= self.p { remainder - self.p } else { remainder }"""
old_code = """            (a * b) % self.p"""
assert source.count(new_code) == 1
before = source.replace(new_code, old_code)

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
fn median(mut x:Vec<f64>)->f64{x.sort_by(|a,b|a.total_cmp(b));x[x.len()/2]}
#[test]
fn scalar_equivalence() {
 let mut count=0;
 for p in [5,7,17,65537,1_000_000_007,4_294_967_291,(1u64<<61)-1] {
  let old=before::Fp::new(p);let new=after::Fp::new(p);
  for a in [0,1,2,p/2,p-2,p-1,p,p+1] {
   for b in [0,1,2,p/2,p-2,p-1,p,p+1] {
    let want=((a as u128*b as u128)%p as u128) as u64;
    assert_eq!(old.mul(a,b),want);
    assert_eq!(new.mul(a,b),want,"p={p} a={a} b={b}");
    count+=1;
   }
  }
  let mut s=p^0xa74b_2026_0047_1021;
  for _ in 0..8192 {
   let a=next(&mut s)%p;let b=next(&mut s)%p;
   let want=((a as u128*b as u128)%p as u128) as u64;
   assert_eq!(old.mul(a,b),want);
   assert_eq!(new.mul(a,b),want,"p={p} a={a} b={b}");
   count+=1;
  }
 }
 println!("ROUND74_SCALAR_EQ count={count}");
}
fn make_poly(p:u64,roots:&[u64])->Vec<u64> {
 let mut a=vec![1u64];
 for &r in roots {
  let mut b=vec![0u64;a.len()+1];
  for (i,&v) in a.iter().enumerate() {
   let t=((v as u128*r as u128)%p as u128) as u64;
   b[i]=(b[i]+p-t)%p;
   b[i+1]=(b[i+1]+v)%p;
  }
  a=b;
 }
 a
}
fn samples(p:u64,degree:usize,count:usize)->Vec<(Vec<u64>,Vec<u64>)> {
 let mut s=p^0x77b2_98cf_74ac_eb03u64^(degree as u64);
 (0..count).map(|_| {
  let mut rr:Vec<u64>=(0..degree).map(|_|1+next(&mut s)%(p-1)).collect();
  rr.sort_unstable();rr.dedup();
  let poly=make_poly(p,&rr);
  (poly,rr)
 }).collect()
}
#[test]
fn complete_radical_equivalence() {
 let mut count=0;
 for p in [7,1_000_000_007u64,4_294_967_291,(1u64<<61)-1] {
  let cb=before::RadicalRootCtx::new(p).unwrap();
  let ca=after::RadicalRootCtx::new(p).unwrap();
  for degree in 1..=4 {
   for (poly,expected) in samples(p,degree,128) {
    let mut a=cb.roots(&poly).unwrap().roots;
    let mut b=ca.roots(&poly).unwrap().roots;
    a.sort_unstable();a.dedup();b.sort_unstable();b.dedup();
    assert_eq!(a,b,"p={p} degree={degree} poly={poly:?}");
    assert_eq!(b,expected,"p={p} degree={degree} poly={poly:?}");
    count+=1;
   }
  }
 }
 println!("ROUND74_ROOT_EQ count={count}");
}
#[test]
#[ignore="paired production Rust modular multiplication"]
fn bench_fp_mul() {
 for p in [1_000_000_007u64,4_294_967_291,(1u64<<61)-1] {
  let cb=before::Fp::new(p);let ca=after::Fp::new(p);
  let mut s=p^0xf1edb2a4;
  let pairs:Vec<_>=(0..8192).map(|_|(next(&mut s)%p,next(&mut s)%p)).collect();
  let mut a=Vec::new();let mut b=Vec::new();
  for trial in 0..13 {
   let measure=|optimized:bool| {
    let now=Instant::now();let mut checksum=0u64;
    for _ in 0..64 {for &(x,y) in &pairs {
     checksum ^= if optimized {ca.mul(black_box(x),black_box(y))}
       else {cb.mul(black_box(x),black_box(y))};
    }}
    black_box(checksum);
    now.elapsed().as_nanos() as f64/(pairs.len()*64) as f64
   };
   if trial%2==0 {a.push(measure(false));b.push(measure(true));}
   else {b.push(measure(true));a.push(measure(false));}
  }
  let old=median(a);let new=median(b);
  println!("ROUND74_RUST_MUL p={p} before_ns={old:.3} after_ns={new:.3} speedup={:.3}x",old/new);
 }
}
#[test]
#[ignore="paired complete low-degree root-extraction"]
fn bench_full_roots() {
 for p in [1_000_000_007u64,4_294_967_291,(1u64<<61)-1] {
  let cb=before::RadicalRootCtx::new(p).unwrap();
  let ca=after::RadicalRootCtx::new(p).unwrap();
  for degree in [2,3,4] {
   let cases=samples(p,degree,64);
   let mut a=Vec::new();let mut b=Vec::new();
   for trial in 0..13 {
    let measure=|optimized:bool| {
     let now=Instant::now();let mut checksum=0usize;
     for _ in 0..4 {for (poly,_) in &cases {
      let count=if optimized {ca.roots(black_box(poly)).unwrap().roots.len()}
        else {cb.roots(black_box(poly)).unwrap().roots.len()};
      checksum+=count;
     }}
     black_box(checksum);
     now.elapsed().as_nanos() as f64/(cases.len()*4) as f64
    };
    if trial%2==0 {a.push(measure(false));b.push(measure(true));}
    else {b.push(measure(true));a.push(measure(false));}
   }
   let old=median(a);let new=median(b);
   println!("ROUND74_RUST_ROOT p={p} degree={degree} before_ns={old:.3} after_ns={new:.3} speedup={:.3}x",old/new);
  }
 }
}
}
"""
with tempfile.TemporaryDirectory() as directory:
    path = Path(directory) / "round74.rs"
    path.write_text(head + before + "\n}\npub mod after {\n" + source + "\n}\n}\n" + checks)
    exe = Path(directory) / "round74-tests"
    subprocess.run(["rustc", "--edition=2021", "-O", "--test", str(path), "-o", str(exe)], check=True)
    subprocess.run([str(exe), "--nocapture", "--test-threads=1"], check=True)
    subprocess.run([str(exe), "--ignored", "--nocapture", "--test-threads=1"], check=True)
