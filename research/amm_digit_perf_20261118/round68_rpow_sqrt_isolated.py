#!/usr/bin/env python3
"""Source-exact Rust A/B: Jacobi prefilter for public radical discriminants."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source
new_window = """        // The most-significant window is nonzero. Initialize directly
        // from its precomputed odd power instead of squaring and multiplying
        // the identity up to four times.
        let mut high = bits as i32 - 1;
        let mut low = (high - 3).max(0);
        while ((e >> (low as u32)) & 1) == 0 {
            low += 1;
        }
        let width = (high - low + 1) as u32;
        let digit = ((e >> (low as u32)) & ((1u128 << width) - 1)) as usize;
        let mut acc = odd[(digit - 1) / 2];
        high = low - 1;
        while high >= 0 {"""
old_window = """        let mut acc = r.one();
        let mut high = bits as i32 - 1;
        while high >= 0 {"""
new_sqrt = """            // The prime-field Jacobi character +1 proves that a is a
            // quadratic residue. For p = 3 (mod 4), r=a^((p+1)/4) is
            // necessarily a square root. Skip sqrt()'s redundant r*r check.
            // Only public, canonical inputs take this variable-time branch.
            return Some(self.f.powm(a, (p + 1) / 4));
"""
assert before.count(new_window) == 1
assert before.count(new_sqrt) == 1
before = before.replace(new_window, old_window).replace(new_sqrt, "")

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
   println!("ROUND68_EQ exact_root_comparisons={checks}");
 }
 #[test]
 #[ignore="source-exact native Rust A/B, alternating 13 trials"]
 fn bench_discriminant_cases(){
   for p in [1_000_000_007u64,(1u64<<61)-1]{
     let x=before::RadicalRootCtx::new(p).unwrap();
     let y=after::RadicalRootCtx::new(p).unwrap();
     let mut s=p^0xda17_a37e_501c_9871;
     for op in [0usize,1,2]{
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
              }else if op==1{
                let sq=x.f.mul(a,x.f.sqr(r));
                x.f.neg(x.f.add(sq,x.f.mul(b,r)))
              }else{
                let cubic=x.f.mul(r,x.f.sqr(r));
                x.f.neg(x.f.add(cubic,x.f.mul(b,r)))
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
                   let roots=match op{0=>y.quad_monic(b,c),1=>y.quad_roots(a,b,c),_=>y.cubic_depressed(b,c)};
                   black_box(roots);
                 }else{
                   let roots=match op{0=>x.quad_monic(b,c),1=>x.quad_roots(a,b,c),_=>x.cubic_depressed(b,c)};
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
 #[test]
 fn exact_rpow(){
   let mut checks=0;
   for p in [7u64,11,31,101,1_000_000_007,(1u64<<61)-1]{
     let n=if p==1_000_000_007{5}else if p==(1u64<<61)-1{3}else{2};
     let qb=before::Fq2{f:before::Fp::new(p),n};
     let qa=after::Fq2{f:after::Fp::new(p),n};
     let mut s=p^0x989ab17c313feae7;
     for k in 0..128 {
       let x=(next(&mut s)%p,next(&mut s)%p);
       let random=((next(&mut s) as u128)<<64)|(next(&mut s) as u128);
       let e=match k%8 {0=>0,1=>1,2=>2,3=>3,
         4=>(1u128<<56)+(1u128<<20)+1,
         5=>random,6=>random & ((1u128<<63)-1),
         _=>(1u128<<20)-1};
       let a=before::rpow(&qb,x,e);
       let b=after::rpow(&qa,x,e);
       assert_eq!(a,b,"p={p} e={e} x={x:?}");
       checks+=1;
     }
   }
   println!("ROUND68_EQ exact_rpow={checks}");
 }
 #[test]
 #[ignore="alternating 13-trial actual Rust Fp2 powering benchmark"]
 fn bench_rpow() {
  for p in [1_000_000_007u64,(1u64<<61)-1] {
    let n=if p==1_000_000_007{5}else{3};
    let qb=before::Fq2{f:before::Fp::new(p),n};
    let qa=after::Fq2{f:after::Fp::new(p),n};
    let mut s=p^0x68e15ee10fda1714;
    for mode in 0..3 {
      let mut inputs=Vec::new();
      for _ in 0..128 {
        let x=(next(&mut s)%p,next(&mut s)%p);
        let random=((next(&mut s) as u128)<<64)|(next(&mut s) as u128);
        let e=match mode {
          0=>(1u128<<56)+(1u128<<20)+1,
          1=>random,
          _=>random&((1u128<<63)-1)
        };
        inputs.push((x,e));
      }
      let mut old=Vec::new();let mut new=Vec::new();
      for trial in 0..13 {
        let measure=|optimized:bool|{
          let start=Instant::now();
          for _ in 0..8 {for &(x,e) in &inputs{
            let r=if optimized {after::rpow(&qa,x,e)}
              else {before::rpow(&qb,x,e)};
            black_box(r);
          }}
          start.elapsed().as_nanos() as f64/(8*inputs.len()) as f64
        };
        if trial%2==0{old.push(measure(false));new.push(measure(true));}
        else{new.push(measure(true));old.push(measure(false));}
      }
      let a=median(old);let b=median(new);
      println!("ROUND68_RUST rpow p={p} mode={mode} before_ns={a:.3} after_ns={b:.3} speedup={:.4}",a/b);
    }
  }
 }
}
"""
with tempfile.TemporaryDirectory() as folder:
 p=Path(folder)/"round68.rs"
 binary=Path(folder)/"round68"
 p.write_text(head+before+middle+source+tail+tests)
 subprocess.run(["rustc","--edition=2021","-O","--test",str(p),"-o",str(binary)],check=True)
 subprocess.run([str(binary),"exact_roots","--nocapture"],check=True)
 subprocess.run([str(binary),"exact_rpow","--nocapture"],check=True)
 subprocess.run([str(binary),"bench_discriminant_cases","--ignored","--nocapture"],check=True)
 subprocess.run([str(binary),"bench_rpow","--ignored","--nocapture"],check=True)
