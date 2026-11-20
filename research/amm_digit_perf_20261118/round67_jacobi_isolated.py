#!/usr/bin/env python3
"""Source-exact Rust A/B: Jacobi prefilter for public radical discriminants."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source
begin = before.index("    // Binary Jacobi prefilter:")
end = before.index("    /// Distinct roots of monic y^2 + b y + c.", begin)
before = before[:begin] + before[end:]
assert before.count("self.sqrt_discriminant(") == 3
before = before.replace("self.sqrt_discriminant(", "self.sqrt(")

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
 use crate::cryptanalysis::{before,after};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64 {
   *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*s
 }
 fn median(mut a:Vec<f64>)->f64{a.sort_by(|x,y|x.total_cmp(y));a[a.len()/2]}
 fn roots_both(p:u64,a:u64,b:u64,c:u64,op:usize)->(Vec<u64>,Vec<u64>){
   let x=before::RadicalRootCtx::new(p).unwrap();
   let y=after::RadicalRootCtx::new(p).unwrap();
   let old=match op {
     0=>x.quad_monic(b,c),
     1=>x.quad_roots(a,b,c),
     _=>x.cubic_depressed(b,c)
   };
   let new=match op {
     0=>y.quad_monic(b,c),
     1=>y.quad_roots(a,b,c),
     _=>y.cubic_depressed(b,c)
   };
   (old,new)
 }
 #[test]
 fn exact_roots() {
   let mut checks=0usize;
   for p in [7u64,11,31,101,1_000_000_007,(1u64<<61)-1]{
      let mut s=p^0x57c1_daef_6120_8829;
      let trials=if p>101{48}else{128};
      for _ in 0..trials {
        let a=1+(next(&mut s)%(p-1));
        let b=next(&mut s)%p;
        let c=next(&mut s)%p;
        for op in 0..3 {
          let (mut x,mut y)=roots_both(p,a,b,c,op);
          x.sort_unstable();y.sort_unstable();
          assert_eq!(x,y,"p={p} op={op} a={a} b={b} c={c}");
          checks+=1;
        }
      }
   }
   println!("ROUND67_EQ exact_root_comparisons={checks}");
 }
 #[test]
 #[ignore="source-exact native Rust A/B, alternating 13 trials"]
 fn bench_discriminant_cases(){
   for p in [1_000_000_007u64,(1u64<<61)-1]{
     let x=before::RadicalRootCtx::new(p).unwrap();
     let y=after::RadicalRootCtx::new(p).unwrap();
     let mut s=p^0xda17_a37e_501c_9871;
     for op in [0usize,1]{
       for known_root in [false,true]{
         let mut inputs=Vec::new();
         for _ in 0..256{
           let a=1+(next(&mut s)%(p-1));
           let b=next(&mut s)%p;
           let r=next(&mut s)%p;
           let c=if known_root{
              if op==0 {
                let term=x.f.mul(r,x.f.add(r,b));
                x.f.neg(term)
              }else{
                let sq=x.f.mul(a,x.f.sqr(r));
                x.f.neg(x.f.add(sq,x.f.mul(b,r)))
              }
           }else{next(&mut s)%p};
           inputs.push((a,b,c));
         }
         let mut old=Vec::new();let mut new=Vec::new();
         for trial in 0..13{
           let measure=|optimized:bool|{
             let start=Instant::now();
             for _ in 0..4{
               for &(a,b,c) in &inputs{
                 if optimized {
                   let roots=if op==0{y.quad_monic(b,c)}else{y.quad_roots(a,b,c)};
                   black_box(roots);
                 }else{
                   let roots=if op==0{x.quad_monic(b,c)}else{x.quad_roots(a,b,c)};
                   black_box(roots);
                 }
               }
             }
             start.elapsed().as_nanos() as f64/(4*inputs.len()) as f64
           };
           if trial%2==0{old.push(measure(false));new.push(measure(true));}
           else{new.push(measure(true));old.push(measure(false));}
         }
         let a=median(old);let b=median(new);
         println!("ROUND67_RUST p={p} op={op} known_root={known_root} before_ns={a:.3} after_ns={b:.3} speedup={:.4}",a/b);
       }
     }
   }
 }
}
"""
with tempfile.TemporaryDirectory() as folder:
 p=Path(folder)/"round67.rs"
 binary=Path(folder)/"round67"
 p.write_text(head+before+middle+source+tail+tests)
 subprocess.run(["rustc","--edition=2021","-O","--test",str(p),"-o",str(binary)],check=True)
 subprocess.run([str(binary),"exact_roots","--nocapture"],check=True)
 subprocess.run([str(binary),"bench_discriminant_cases","--ignored","--nocapture"],check=True)
