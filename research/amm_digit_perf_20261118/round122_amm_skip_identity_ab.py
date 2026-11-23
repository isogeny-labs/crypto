#!/usr/bin/env python3
"""Round 122b, source-exact native Rust A/B for first AMM digit identity elimination.

Both modules compile the same production Rust module, with baseline reverting
only the digit-one correction multiplication by the identity.
"""
import os
import subprocess
import tempfile
from pathlib import Path

try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"ROUND122_IDENTITY_CPU={cpu}",flush=True)
except (AttributeError,OSError):
    print("ROUND122_IDENTITY_CPU=unavailable",flush=True)
source=Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n",1)[0]
candidate="            let mut z = if j == 1 { h } else { r.mul(h, correction) };"
baseline="            let mut z = r.mul(h, correction);"
assert source.count(candidate)==1
frozen=source.replace(candidate,baseline,1)
assert frozen!=source
header=r'''
#![allow(dead_code,unused_imports,private_interfaces)]
mod cryptanalysis {
pub mod prime_root_trace {
 pub const MAX_PRIME:u64=(1u64<<61)-1;
 pub const MAX_WORK:u64=1_000_000;
 pub struct RootLimits{pub field_work:u64,pub split_attempts:u64,pub events:u64,pub degree:usize}
 #[derive(Debug,PartialEq,Eq)] pub enum RootStop{Complete}
 pub struct RootReport{pub stop:RootStop,pub roots:Vec<u64>}
 pub struct RootEvent;
 pub fn solve(_: &[u64],_:u64,_:u64,_:RootLimits,_:&mut dyn FnMut(&RootEvent)->Result<(),String>)
  ->Result<RootReport,String>{panic!("Unexpected Frobenius fallback")}
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool{
  matches!(p,5|7|11|31|97|109|487|101|257|65_521|1_000_000_007|1_000_000_009|4_294_967_291|
           1_000_000_021|4_294_967_197|2_305_843_009_213_693_693|
           2_305_843_009_213_693_951)
 }
}
pub mod baseline {
'''

expose=r'''
pub fn context(p:u64)->RadicalRootCtx{RadicalRootCtx::new(p).unwrap()}
pub fn fp(c:&RadicalRootCtx,x:u64)->Option<u64>{c.cube_p.root(x)}
pub fn fq(c:&RadicalRootCtx,x:(u64,u64))->Option<(u64,u64)>{c.cube_q.root(x)}
pub fn cubic(c:&RadicalRootCtx,b:u64,d:u64,e:u64)->Vec<u64>{c.cubic_monic(b,d,e)}
}
'''
tests=r'''
#[cfg(test)]
mod checks {
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64 {
  *s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s
 }
 fn med(mut x:Vec<f64>)->f64{x.sort_by(f64::total_cmp);x[x.len()/2]}
 fn fields()->[u64;8]{[97,109,487,65_521,1_000_000_007,1_000_000_009,4_294_967_291,(1u64<<61)-1]}
 fn data(p:u64,seed:&mut u64)->Vec<(u64,u64,u64)>{
  let mut v=Vec::new();
  for _ in 0..96{v.push((next(seed)%p,next(seed)%p,next(seed)%p));}
  v
 }
 fn digest(acc:u64,a:u64)->u64{acc.rotate_left(7).wrapping_mul(0x9e37_79b1_85eb_ca87)^a}
 fn take_fp(v:Option<u64>)->u64{v.unwrap_or(0)}
 fn take_fq(v:Option<(u64,u64)>)->u64{match v{Some((x,y))=>digest(x,y),None=>0}}
 #[test]fn equivalence(){
   let mut seed=0x1220_2026_1122_0f12u64;
   let mut count=0;
   for p in fields(){
     let old=baseline::context(p);let new=candidate::context(p);
     for (a,b,c) in data(p,&mut seed){
       assert_eq!(baseline::fp(&old,a),candidate::fp(&new,a),"Fp p={p} a={a}");
       assert_eq!(baseline::fq(&old,(a,b)),candidate::fq(&new,(a,b)),"Fp2 p={p} a={a} b={b}");
       assert_eq!(baseline::cubic(&old,a,b,c),candidate::cubic(&new,a,b,c),"cubic p={p}");
       count+=3;
     }
   }
   println!("ROUND122_IDENTITY_EQ comparisons={count}");
 }
 #[test]
 #[ignore="13 paired optimized Rust A/B trials"]
 fn benchmark(){
   let mut seed=0x1220_2026_1122_0f12u64;
   for p in fields(){
     let old=baseline::context(p);let new=candidate::context(p);
     let inputs=data(p,&mut seed);
     for kind in ["fp","fp2","cubic"] {
       for &(a,b,c) in &inputs {
         match kind{
           "fp"=>assert_eq!(baseline::fp(&old,a),candidate::fp(&new,a)),
           "fp2"=>assert_eq!(baseline::fq(&old,(a,b)),candidate::fq(&new,(a,b))),
           _=>assert_eq!(baseline::cubic(&old,a,b,c),candidate::cubic(&new,a,b,c))
         }
       }
       let mut before=Vec::new();let mut after=Vec::new();
       let mut expected_hash=None;
       for trial in 0..13 {
         let measure=|optimized:bool|->(f64,u64){
           let start=Instant::now();
           let mut hash=0x1220_2026_111e_cafeu64;
           for _ in 0..16 {
             for &(a,b,c) in &inputs {
               let item= if optimized {
                 match kind{
                   "fp"=>take_fp(candidate::fp(black_box(&new),black_box(a))),
                   "fp2"=>take_fq(candidate::fq(black_box(&new),black_box((a,b)))),
                   _=>candidate::cubic(black_box(&new),black_box(a),black_box(b),black_box(c)).iter().fold(0u64,|h,&x|digest(h,x)),
                 }
               } else {
                 match kind{
                   "fp"=>take_fp(baseline::fp(black_box(&old),black_box(a))),
                   "fp2"=>take_fq(baseline::fq(black_box(&old),black_box((a,b)))),
                   _=>baseline::cubic(black_box(&old),black_box(a),black_box(b),black_box(c)).iter().fold(0u64,|h,&x|digest(h,x)),
                 }
               };
               hash=digest(hash,black_box(item));
             }
           }
           (start.elapsed().as_nanos() as f64/(16*inputs.len()) as f64,hash)
         };
         let (b,a)=if trial%2==0 {(measure(false),measure(true))}
           else {let a=measure(true);let b=measure(false);(b,a)};
         assert_eq!(b.1,a.1,"p={p} kind={kind}");
         if let Some(prior)=expected_hash {assert_eq!(prior,a.1)}
           else {expected_hash=Some(a.1);}
         before.push(b.0);after.push(a.0);
       }
       let b=med(before);let a=med(after);
       println!("ROUND122_IDENTITY_RUST p={p} kind={kind} baseline_ns={b:.3} optimized_ns={a:.3} speedup={:.4} checksum={}",b/a,expected_hash.unwrap());
     }
   }
 }
}
'''
text=header+frozen+expose+"pub mod candidate{\n"+source+expose+"}\n"+tests
with tempfile.TemporaryDirectory() as temp:
    root=Path(temp)
    src=root/"round122_identity.rs"
    exe=root/"round122_identity"
    src.write_text(text)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
