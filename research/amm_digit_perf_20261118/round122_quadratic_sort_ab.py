#!/usr/bin/env python3
"""Round122 source-exact Rust A/B: nonmonic quadratic root ordering without sorting."""
import os
import subprocess
import tempfile
from pathlib import Path

try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"ROUND122_QUAD_CPU={cpu}",flush=True)
except (AttributeError,OSError):
    print("ROUND122_QUAD_CPU=unavailable",flush=True)
source=Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n",1)[0]
begin="        // A nonzero discriminant yields two distinct roots when a != 0."
end="    }\n    /// F_p-roots of the depressed cubic"
assert source.count(begin)==1 and source.count(end)==1
a=source.index(begin);b=source.index(end,a)
old='''        let mut roots = if s == 0 {
            vec![first]
        } else {
            vec![first, f.mul(f.sub(nb, s), inv_2a)]
        };
        roots.sort_unstable();
        roots.dedup();
        roots
'''
frozen=source[:a]+old+source[b:]
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
  matches!(p,5|7|11|31|97|109|487|101|257|65_521|1_000_000_007|4_294_967_291|
           1_000_000_021|4_294_967_197|2_305_843_009_213_693_693|
           2_305_843_009_213_693_951)
 }
}
pub mod baseline {
'''

expose=r'''
pub fn context(p:u64)->RadicalRootCtx{RadicalRootCtx::new(p).unwrap()}
pub fn roots(c:&RadicalRootCtx,a:u64,b:u64,d:u64)->Vec<u64>{c.quad_roots(a,b,d)}
pub fn complete(c:&RadicalRootCtx,a:u64,b:u64,d:u64)->Vec<u64>{
 c.roots(&[d,b,a]).unwrap().roots
}
}
'''
tests=r'''
#[cfg(test)]
mod tests{
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
 fn median(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);v[v.len()/2]}
 fn fields()->[u64;7]{[5,97,109,65_521,1_000_000_007,4_294_967_291,(1u64<<61)-1]}
 fn fmul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%(p as u128))as u64}
 fn inputs(p:u64,known:bool,s:&mut u64)->Vec<(u64,u64,u64)>{
  let mut out=Vec::new();
  for i in 0..128{
   let a=2+next(s)%(p-2);
   let b=next(s)%p;let c=next(s)%p;
   if known {
     let x=b;let y=if i%11==0 {b} else {c};
     let sum=((x as u128+y as u128)%(p as u128))as u64;
     let bb=if sum==0 {0}else{p-fmul(a,sum,p)};
     let cc=fmul(a,fmul(x,y,p),p);
     out.push((a,bb,cc));
   }else{out.push((a,b,c));}
  }
  out
 }
 fn digest(rs:&[u64])->u64{rs.iter().fold(0x1220_2026_ddd0_1111u64,|h,&v|
   h.rotate_left(7).wrapping_mul(0x9e37_79b1_85eb_ca87)^v)}
 #[test]fn equivalence(){
  let mut seed=0x1222_2026_1314_1516u64;
  let mut count=0usize;
  for p in fields(){
   let old=baseline::context(p);let new=candidate::context(p);
   for &known in &[false,true]{
    for (a,b,c) in inputs(p,known,&mut seed){
     let before=baseline::roots(&old,a,b,c);let after=candidate::roots(&new,a,b,c);
     assert_eq!(before,after,"p={p} a={a} b={b} c={c}");
     assert_eq!(baseline::complete(&old,a,b,c),candidate::complete(&new,a,b,c));
     for &x in &after{
      let res=(fmul(a,fmul(x,x,p),p)+fmul(b,x,p))%(p as u64);
      let res=((res as u128+c as u128)%(p as u128))as u64;
      assert_eq!(res,0,"p={p} root={x}");
     }
     count+=2;
    }
   }
   for b in 0..p.min(20){
    for c in 0..p.min(20){
     assert_eq!(baseline::roots(&old,0,b,c),candidate::roots(&new,0,b,c),
       "legacy a=0 p={p} b={b} c={c}");
     count+=1;
    }
   }
  }
  println!("ROUND122_QUAD_EQ comparisons={count}");
 }
 #[test]
 #[ignore="13 CPU-pinned A/B trials"]
 fn benchmark(){
  let mut seed=0x1220_2026_3031_3233u64;
  for p in fields(){
    let old=baseline::context(p);let new=candidate::context(p);
    for &known in &[false,true]{
      let inputs=inputs(p,known,&mut seed);
      let mut before=Vec::new();let mut after=Vec::new();let mut checksum=None;
      for trial in 0..13{
        let measure=|optimized:bool|->(f64,u64){
          let start=Instant::now();let mut hash=0x1211_0f22_0c0d_0e0fu64;
          for _ in 0..24{
            for &(a,b,c) in &inputs{
              let roots=if optimized{
                candidate::complete(black_box(&new),black_box(a),black_box(b),black_box(c))
              }else{
                baseline::complete(black_box(&old),black_box(a),black_box(b),black_box(c))
              };
              hash=hash.rotate_left(3).wrapping_mul(0x9e37_79b1_85eb_ca87)^black_box(digest(&roots));
            }
          }
          (start.elapsed().as_nanos()as f64/(24*inputs.len())as f64,hash)
        };
        let (b,a)=if trial%2==0{(measure(false),measure(true))}
         else{let a=measure(true);let b=measure(false);(b,a)};
        assert_eq!(b.1,a.1,"p={p} known={known}");
        if let Some(prev)=checksum{assert_eq!(prev,a.1);}
         else{checksum=Some(a.1);}
        before.push(b.0);after.push(a.0);
      }
      let b=median(before);let a=median(after);
      println!("ROUND122_QUAD_RUST p={p} shape={} before_ns={b:.3} after_ns={a:.3} speedup={:.4} checksum={}",if known{"known"}else{"uniform"},b/a,checksum.unwrap());
    }
  }
 }
}
'''
text=header+frozen+expose+"pub mod candidate{\n"+source+expose+"}\n"+tests
with tempfile.TemporaryDirectory() as temp:
    root=Path(temp)
    src=root/"round122_quad.rs"
    exe=root/"round122_quad"
    src.write_text(text)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
