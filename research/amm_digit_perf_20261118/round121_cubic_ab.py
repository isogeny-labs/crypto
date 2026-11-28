#!/usr/bin/env python3
"""Round 121 source-exact native Rust complete cubic root A/B.

Duplicate the *current* production Rust source, and remove precisely the
round121 Fq2 sparse branch from the frozen copy. Both use identical Fp and
Jacobi implementations. No external dependencies; Frobenius fallback panics.
"""
from pathlib import Path
import os
import subprocess
import tempfile

try:
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    print(f"ROUND121_CUBIC_CPU={cpu}", flush=True)
except (AttributeError, OSError):
    print("ROUND121_CUBIC_CPU=unavailable", flush=True)

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
new = """        // The AMM root solver frequently multiplies by elements of Fp.
        // Avoid Karatsuba's extra base-field product when the left operand
        // lies in the embedded prime field. Keep dense arithmetic unchanged.
        // Public-input, variable-time fast path; never use for secret data.
        if x.1 == 0 {
            return (f.mul(x.0, y.0), f.mul(x.0, y.1));
        }
"""
assert source.count(new) == 1, "source-exact Fq2 branch changed unexpectedly"
frozen=source.replace(new, "", 1)
assert frozen!=source

header=r'''
#![allow(dead_code, unused_imports, private_interfaces)]
mod cryptanalysis {
pub mod prime_root_trace {
 pub const MAX_PRIME:u64=(1u64<<61)-1;
 pub const MAX_WORK:u64=1_000_000;
 pub struct RootLimits{pub field_work:u64,pub split_attempts:u64,pub events:u64,pub degree:usize}
 #[derive(Debug,PartialEq,Eq)]pub enum RootStop{Complete}
 pub struct RootReport{pub stop:RootStop,pub roots:Vec<u64>}
 pub struct RootEvent;
 pub fn solve(_: &[u64],_:u64,_:u64,_:RootLimits,_:&mut dyn FnMut(&RootEvent)->Result<(),String>)
   ->Result<RootReport,String>{panic!("unexpected Frobenius fallback")}
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool{
  matches!(p,5|7|11|31|101|257|65_521|1_000_000_007|4_294_967_291|
            1_000_000_021|4_294_967_197|2_305_843_009_213_693_693|
            2_305_843_009_213_693_951)
 }
}
pub mod baseline{
'''
expose=r'''
pub fn context(p:u64)->RadicalRootCtx {RadicalRootCtx::new(p).unwrap()}
pub fn roots(c:&RadicalRootCtx,f:&[u64])->Vec<u64>{c.roots(f).unwrap().roots}
}
'''
tests=r'''
#[cfg(test)]
mod tests{
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
 fn mul(p:u64,a:u64,b:u64)->u64{((a as u128*b as u128)%(p as u128)) as u64}
 fn cubic(p:u64,r:u64,s:u64,t:u64)->Vec<u64>{
  let sum=(r+s+t)%p;
  let pair=(mul(p,r,s)+mul(p,r,t)+mul(p,s,t))%p;
  let product=mul(p,mul(p,r,s),t);
  vec![if product==0{0}else{p-product},pair,if sum==0{0}else{p-sum},1]
 }
 fn digest(xs:&[u64])->u64{xs.iter().fold(0x1212_2026_1122_9001u64,|a,&b|
     a.rotate_left(5).wrapping_mul(0x9e37_79b1_85eb_ca87)^b)}
 fn median(mut a:Vec<f64>)->f64{a.sort_by(|x,y|x.total_cmp(y));a[a.len()/2]}
 fn workload(p:u64,shape:&str,seed:&mut u64)->Vec<Vec<u64>>{
  let mut out=Vec::new();
  for _ in 0..128{
   if shape=="known_three_roots"{
    let r=next(seed)%p;
    let mut s=next(seed)%p;
    if s==r{s=(s+1)%p;}
    let mut t=next(seed)%p;
    while t==r||t==s{t=(t+1)%p;}
    out.push(cubic(p,r,s,t));
   }else{
    out.push(vec![next(seed)%p,next(seed)%p,next(seed)%p,1]);
   }
  }
  out
 }
 #[test]fn equivalence(){
  let mut seed=0x121e_2026_1122_f00du64;
  let mut checks=0usize;
  for p in [65_521u64,1_000_000_007,(1u64<<61)-1]{
   let old=baseline::context(p);let new=candidate::context(p);
   for shape in ["known_three_roots","uniform"]{
    for poly in workload(p,shape,&mut seed){
     let before=baseline::roots(&old,&poly);
     let after=candidate::roots(&new,&poly);
     assert_eq!(before,after,"p={p} shape={shape} poly={poly:?}");
     if shape=="known_three_roots"{assert_eq!(after.len(),3);}
     checks+=1;
    }
   }
  }
  println!("ROUND121_CUBIC_EQ comparisons={checks}");
 }
 #[test]
 #[ignore="Source-exact complete cubic root paired A/B"]
 fn benchmark(){
  let mut seed=0x121b_2026_1122_fabc;
  for p in [65_521u64,1_000_000_007,(1u64<<61)-1]{
   let old=baseline::context(p);let new=candidate::context(p);
   for shape in ["known_three_roots","uniform"]{
    let inputs=workload(p,shape,&mut seed);
    for poly in &inputs{
      assert_eq!(baseline::roots(&old,poly),candidate::roots(&new,poly));
    }
    let mut bv=Vec::new();let mut av=Vec::new();let mut last=None;
    for trial in 0..13{
     let measure=|optimized:bool|->(f64,u64){
      let begin=Instant::now();let mut sum=0x1211_fff0_2026_1122u64;
      for _ in 0..8{
       for poly in &inputs{
        let roots=if optimized{
          candidate::roots(black_box(&new),black_box(poly))
        }else{
          baseline::roots(black_box(&old),black_box(poly))
        };
        sum=sum.wrapping_mul(0x9e37_79b1_85eb_ca87)^black_box(digest(&roots));
       }
      }
      (begin.elapsed().as_nanos() as f64/(8*inputs.len()) as f64,sum)
     };
     let (before,after)=if trial%2==0{(measure(false),measure(true))}
       else{let a=measure(true);let b=measure(false);(b,a)};
     assert_eq!(before.1,after.1,"p={p} shape={shape}");
     if let Some(prev)=last{assert_eq!(prev,after.1);}else{last=Some(after.1);}
     bv.push(before.0);av.push(after.0);
    }
    let b=median(bv);let a=median(av);
    println!("ROUND121_CUBIC_RUST p={p} shape={shape} before_ns={b:.3} after_ns={a:.3} speedup={:.4} checksum={}",b/a,last.unwrap());
   }
  }
 }
}
'''
code=header+frozen+expose+"pub mod candidate{\n"+source+expose+"}\n"+tests
with tempfile.TemporaryDirectory() as tmp:
    f=Path(tmp)/"round121_cubic.rs"
    exe=Path(tmp)/"round121_cubic"
    f.write_text(code)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(f),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
