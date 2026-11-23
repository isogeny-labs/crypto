#!/usr/bin/env python3
"""Round123 source-exact production Rust A/B: quadratic specializations.

The frozen baseline is the *same checked-out production module* with only
the three Round123 changes reverted, so upstream optimizations are shared.
"""
import os
import subprocess
import tempfile
from pathlib import Path

try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"ROUND123_CPU={cpu}",flush=True)
except (OSError,AttributeError):
    print("ROUND123_CPU=unavailable",flush=True)

s=Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n",1)[0]
def remove_block(text, begin, end):
    a=text.index(begin)
    b=text.index(end,a)+len(end)
    return text[:a]+text[b:]
frozen=remove_block(s,"        // R123_MONIC_BZERO_BEGIN\n",
                    "        // R123_MONIC_BZERO_END\n")
frozen=remove_block(frozen,"        // R123_SMALL_LEADING_BZERO_BEGIN\n",
                    "        // R123_SMALL_LEADING_BZERO_END\n")
old="vec![0, f.mul(f.neg(b), self.inv_leading(a))]"
new="vec![0, f.mul(f.neg(b), f.inv(a))]"
assert frozen.count(old)==1
frozen=frozen.replace(old,new,1)
assert frozen!=s

header=r"""
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
  matches!(p,5|7|11|17|31|97|101|109|487|65_521|1_000_000_007|4_294_967_291|2_305_843_009_213_693_951)
 }
}
pub mod baseline{
"""
expose=r"""
pub fn ctx(p:u64)->RadicalRootCtx{RadicalRootCtx::new(p).unwrap()}
pub fn quad(c:&RadicalRootCtx,a:u64,b:u64,d:u64)->Vec<u64>{
 let mut v=c.quad_roots(a,b,d);v.sort_unstable();v.dedup();v
}
}
"""
tests=r"""
#[cfg(test)]
mod checks {
use super::cryptanalysis::{baseline,candidate};
use std::{hint::black_box,time::Instant};
#[derive(Clone,Copy)]
struct Q{a:u64,b:u64,c:u64}
fn next(s:&mut u64)->u64 {
 *s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s
}
fn median(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);v[v.len()/2]}
fn hash(h:u64,x:u64)->u64{h.rotate_left(7).wrapping_mul(0x9e37_79b1_85eb_ca87)^x}
fn make(p:u64,kind:usize,n:usize)->Vec<Q>{
 let mut state=0x123b_2026_1122_99e1u64^p;
 let f=baseline::ctx(p);
 let m=&f.f;
 let mut v=Vec::with_capacity(n);
 for i in 0..n {
  let r=next(&mut state)%p;
  let t=next(&mut state)%p;
  let a=match kind{
    0|1=>1,
    2|3|4=>2+(i%3) as u64,
    _=>5+next(&mut state)%(p-5),
  };
  let (b,c)=match kind {
    0|2|5=>(0,m.neg(m.mul(a,m.sqr(r)))),
    1|3|6=>(0,next(&mut state)%p),
    4=>{
      let b=m.neg(m.mul(a,m.add(r,t)));
      (b,m.mul(a,m.mul(r,t)))
    },
    _=>(next(&mut state)%p,next(&mut state)%p),
  };
  v.push(Q{a,b,c});
 }
 v
}
fn digest(v:&[u64])->u64{
 v.iter().fold(v.len() as u64,|h,&x|hash(h,x))
}
#[test]
fn correctness(){
 let mut count=0u64;
 for p in [5u64,7,11,97,65_521,1_000_000_007,4_294_967_291,(1u64<<61)-1]{
  let old=baseline::ctx(p);let new=candidate::ctx(p);
  for kind in 0..7 {
   for q in make(p,kind,256){
    let x=baseline::quad(&old,q.a,q.b,q.c);
    let y=candidate::quad(&new,q.a,q.b,q.c);
    assert_eq!(x,y,"p={p} kind={kind} a={} b={} c={}",q.a,q.b,q.c);
    for r in y{let f=&new.f;
      let value=f.add(f.add(f.mul(q.a,f.sqr(r)),f.mul(q.b,r)),q.c);
      assert_eq!(value,0,"invalid root p={p}");
    }
    count+=1;
   }
  }
 }
 println!("ROUND123_EQ_COMPARISONS={count}");
}
#[test]
#[ignore="13 paired optimized production Rust benchmarks"]
fn benchmark(){
 for p in [97u64,65_521,1_000_000_007,4_294_967_291]{
  let old=baseline::ctx(p);let new=candidate::ctx(p);
  for kind in 0..7{
   if p<=5 && kind>=5 {continue;}
   let inputs=make(p,kind,128);
   let mut before=Vec::new();let mut after=Vec::new();
   let mut checksum=None;
   let measure=|optimized:bool|->(f64,u64){
    let start=Instant::now();
    let mut h=0x1234_5654u64;
    for _ in 0..24{
     for &q in &inputs{
      let v=if optimized{
       candidate::quad(black_box(&new),black_box(q.a),black_box(q.b),black_box(q.c))
      } else {
       baseline::quad(black_box(&old),black_box(q.a),black_box(q.b),black_box(q.c))
      };
      h=hash(h,black_box(digest(&v)));
     }
    }
    (start.elapsed().as_nanos() as f64/(24*inputs.len()) as f64,h)
   };
   for trial in 0..13 {
    let (b,a)=if trial%2==0{
      (measure(false),measure(true))
    }else{
      let a=measure(true);let b=measure(false);(b,a)
    };
    assert_eq!(a.1,b.1,"p={p} kind={kind}");
    if let Some(h)=checksum{assert_eq!(a.1,h)}
      else{checksum=Some(a.1);}
    before.push(b.0);after.push(a.0);
   }
   let b=median(before);let a=median(after);
   println!("ROUND123_RUST p={p} kind={kind} before_ns={b:.3} after_ns={a:.3} speedup={:.4} checksum={}",b/a,checksum.unwrap());
  }
 }
}
}
"""
source=header+frozen+expose+"pub mod candidate{\n"+s+expose+"}\n"+tests
with tempfile.TemporaryDirectory() as temp:
    path=Path(temp)/"round123.rs"
    exe=Path(temp)/"round123"
    path.write_text(source)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(path),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"correctness","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
