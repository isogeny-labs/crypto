#!/usr/bin/env python3
"""Source-exact A/B regression and native Rust bench of Cardano Fp2 finish."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
new_chunk = """                // Fq2 inverse and Cardano scaling: m*conj(u)/N(u).
                // Scale once before multiplying the two conjugate components.
                // The Sylow context already caches w^2; avoid a fresh Fq2 multiply.
                let nrm = f.sub(f.sqr(u.0), k.r.mul_nonresidue(f.sqr(u.1)));
                let scaled = f.mul(m, f.inv(nrm));
                let v = (f.mul(u.0, scaled), f.mul(f.neg(u.1), scaled));
                let w = self.w2;
                // Cube caches w² only when v3(|Fp²*|)>=2; small fields
                // with s=1 still require one explicit square.
                let w_2 = if k.s >= 2 { k.w_squared } else { k.r.sqr(w) };"""
old_chunk = """                // 1/u = conj(u)/N(u)
                let nrm = f.sub(f.sqr(u.0), f.mul(self.n, f.sqr(u.1)));
                let ni = f.inv(nrm);
                let uinv = (f.mul(u.0, ni), f.mul(f.neg(u.1), ni));
                let v = (f.mul(m, uinv.0), f.mul(m, uinv.1));
                let w = self.w2;
                let w_2 = k.r.mul(w, w);"""
assert source.count(new_chunk) == 1, "source mismatch, audit before benchmarking"
before = source.replace(new_chunk, old_chunk, 1)

# Exercise the literal production code inside both versions as an isolated
# inner-stage measurement, not a hand-translated approximation.
def expose_finish(body, marker):
    pos = body.index(marker)
    end = body.index("                out\n", pos) + len("                out\n")
    fragment = body[pos:end]
    wrapper = """
impl RadicalRootCtx {
 pub fn finish_for_test(&self, u:(u64,u64), m:u64)->Vec<u64>{
  let f = &self.f;
  let k = &self.cube_q;
""" + fragment + """
 }
}
"""
    return body + "\n" + wrapper

before = expose_finish(before, "                // 1/u = conj(u)/N(u)")
after = expose_finish(source, "                // Fq2 inverse and Cardano scaling:")

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
  ->Result<RootReport,String> {panic!("unreachable: benchmark degree <= 3, no quartic fallback")}
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool {
     matches!(p, 7|11|31|101|1_000_000_007|2_305_843_009_213_693_951)
 }
}
pub mod before {
"""
middle = "\n}\npub mod after {\n"
tail = "\n}\n}\n"

tests=r"""
#[cfg(test)]
mod checks {
 use crate::cryptanalysis::{before, after};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64{
  *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
  *s
 }
 fn median(mut values:Vec<f64>)->f64{
  values.sort_by(f64::total_cmp);values[values.len()/2]
 }
 #[test]
 fn exact_finish_and_full_roots(){
  let mut stages=0usize;let mut complete=0usize;
  for p in [7u64,11,31,101,1_000_000_007,(1u64<<61)-1] {
   let a=before::RadicalRootCtx::new(p).unwrap();
   let b=after::RadicalRootCtx::new(p).unwrap();
   let mut s=p^0xfde6c3a202611201;
   for i in 0..256 {
    let u=(1+next(&mut s)%(p-1),1+next(&mut s)%(p-1));
    let m=next(&mut s)%p;
    assert_eq!(a.finish_for_test(u,m),b.finish_for_test(u,m),
      "finisher p={p} i={i}");
    stages+=1;
    if i<64 {
      let pp=next(&mut s)%p;let q=next(&mut s)%p;
      let mut x=a.cubic_depressed(pp,q);
      let mut y=b.cubic_depressed(pp,q);
      x.sort_unstable();y.sort_unstable();
      assert_eq!(x,y,"full cubic p={p} i={i} pp={pp} q={q}");
      complete+=1;
    }
   }
  }
  println!("ROUND69_EQ source_finish={stages} full_cubic={complete}");
 }
 fn paired<F,G>(label:&str,input_len:usize,measure_before:F,measure_after:G)
 where F:Fn()->u64,G:Fn()->u64 {
  let mut before=Vec::new();let mut after=Vec::new();
  for i in 0..13 {
   if i%2==0{
    let start=Instant::now();black_box(measure_before());before.push(start.elapsed().as_nanos() as f64/input_len as f64);
    let start=Instant::now();black_box(measure_after());after.push(start.elapsed().as_nanos() as f64/input_len as f64);
   }else{
    let start=Instant::now();black_box(measure_after());after.push(start.elapsed().as_nanos() as f64/input_len as f64);
    let start=Instant::now();black_box(measure_before());before.push(start.elapsed().as_nanos() as f64/input_len as f64);
   }
  }
  let a=median(before);let b=median(after);
  println!("ROUND69_RUST {label} before_ns={a:.3} after_ns={b:.3} speedup={:.4}",a/b);
 }
 #[test]
 #[ignore="compiled native Rust 13 alternating A/B trials"]
 fn bench_casus_finish_and_complete_cubic(){
  for p in [1_000_000_007u64,(1u64<<61)-1]{
   let a=before::RadicalRootCtx::new(p).unwrap();
   let b=after::RadicalRootCtx::new(p).unwrap();
   let mut s=p^0x69201128ecafee01u64;
   let mut raw=Vec::new();let mut cubics=Vec::new();
   for _ in 0..128{
     raw.push(((1+next(&mut s)%(p-1),1+next(&mut s)%(p-1)),next(&mut s)%p));
     // Deliberately generate a depressed cubic with three known Fp roots.
     let r1=next(&mut s)%p;let r2=next(&mut s)%p;
     let r3=a.f.sub(0,a.f.add(r1,r2));
     let pp=a.f.add(a.f.add(a.f.mul(r1,r2),a.f.mul(r2,r3)),a.f.mul(r1,r3));
     let q=a.f.sub(0,a.f.mul(a.f.mul(r1,r2),r3));
     cubics.push((pp,q));
   }
   paired(&format!("isolated_casus_finish p={p}"),128*128,
    ||{let mut hash=0;for _ in 0..128{for &(u,m) in &raw{
      let out=black_box(a.finish_for_test(black_box(u),black_box(m)));
      hash^=out.iter().fold(0u64,|h,&x|h.wrapping_mul(33).wrapping_add(x));
    }}hash},
    ||{let mut hash=0;for _ in 0..128{for &(u,m) in &raw{
      let out=black_box(b.finish_for_test(black_box(u),black_box(m)));
      hash^=out.iter().fold(0u64,|h,&x|h.wrapping_mul(33).wrapping_add(x));
    }}hash});
   paired(&format!("complete_three_root_cubic p={p}"),128*8,
    ||{let mut hash=0;for _ in 0..8{for &(pp,q) in &cubics{
      let out=black_box(a.cubic_depressed(black_box(pp),black_box(q)));
      hash^=out.iter().fold(0u64,|h,&x|h.wrapping_mul(33).wrapping_add(x));
    }}hash},
    ||{let mut hash=0;for _ in 0..8{for &(pp,q) in &cubics{
      let out=black_box(b.cubic_depressed(black_box(pp),black_box(q)));
      hash^=out.iter().fold(0u64,|h,&x|h.wrapping_mul(33).wrapping_add(x));
    }}hash});
  }
 }
}
"""

with tempfile.TemporaryDirectory() as folder:
    p = Path(folder)/"round69.rs"
    binary = Path(folder)/"round69"
    p.write_text(head + before + middle + after + tail + tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(p),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"exact_finish_and_full_roots","--nocapture"],check=True)
    subprocess.run([str(binary),"bench_casus_finish_and_complete_cubic","--ignored","--nocapture"],check=True)
