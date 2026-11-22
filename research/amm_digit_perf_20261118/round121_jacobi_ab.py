#!/usr/bin/env python3
"""Round 121 source-exact native Rust Jacobi narrow A/B with full quadratics."""
from pathlib import Path
import os
import subprocess
import tempfile

try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"ROUND121_JACOBI_CPU={cpu}",flush=True)
except (AttributeError,OSError):
    print("ROUND121_JACOBI_CPU=unavailable",flush=True)

source=Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n",1)[0]
begin="            // Use narrow Euclidean remainders for every 32-bit modulus,"
end="            // For p == 5 (mod 8), a^(p+3)/8 squares to a or -a"
assert source.count(begin)==1 and source.count(end)==1
start=source.index(begin); finish=source.index(end,start)
frozen_body="""            // A selected 32-bit modulus: use 32-bit Euclidean remainders
            // rather than 64-bit hardware divisions in the Jacobi prefilter.
            // Public, canonical discriminants only.
            if p == 4_294_967_291 {
                let (mut v, mut n) = (a as u32, p as u32);
                let mut negative = false;
                while v != 0 {
                    let twos = v.trailing_zeros();
                    v >>= twos;
                    if twos & 1 != 0 && (n & 7 == 3 || n & 7 == 5) {
                        negative = !negative;
                    }
                    if v & 3 == 3 && n & 3 == 3 {
                        negative = !negative;
                    }
                    let next = n % v;
                    n = v;
                    v = next;
                }
                // The Jacobi test already proves that a is a square. Use the
                // 2^30-1 addition chain here as in sqrt(), not generic rpow.
                // This saves 24 modular multiplications for each accepted
                // canonical discriminant over p = 2^32-5.
                if n != 1 || negative { return None; }
                return Some(self.f.sqrt_power_4294967291(a));
            }
            let (mut v, mut n) = (a, p);
            let mut negative = false;
            while v != 0 {
                let twos = v.trailing_zeros();
                v >>= twos;
                if twos & 1 != 0 && (n & 7 == 3 || n & 7 == 5) {
                    negative = !negative;
                }
                if v & 3 == 3 && n & 3 == 3 {
                    negative = !negative;
                }
                let next = n % v;
                n = v;
                v = next;
            }
            if n != 1 || negative {
                return None;
            }
"""
frozen=source[:start]+frozen_body+source[finish:]
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
  matches!(p,5|7|11|31|101|257|65_521|1_000_000_007|4_294_967_291|
           1_000_000_021|4_294_967_197|2_305_843_009_213_693_693|
           2_305_843_009_213_693_951)
 }
}
pub mod baseline {
'''
expose=r'''
pub fn context(p:u64)->RadicalRootCtx{RadicalRootCtx::new(p).unwrap()}
pub fn disc(c:&RadicalRootCtx,a:u64)->Option<u64>{c.sqrt_discriminant(a)}
pub fn roots(c:&RadicalRootCtx,b:u64,d:u64)->Vec<u64>{
 let mut roots=c.quad_monic(b,d);
 roots.sort_unstable();roots.dedup();roots
}
}
'''
tests=r'''
#[cfg(test)]
mod tests {
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
 fn med(mut v:Vec<f64>)->f64{v.sort_by(|a,b|a.total_cmp(b));v[v.len()/2]}
 fn hash(xs:&[u64])->u64{xs.iter().fold(0x1211_2026_fff0_0121u64,|h,&v|
    h.rotate_left(5).wrapping_mul(0x9e37_79b1_85eb_ca87)^v)}
 fn fields()->[u64;4]{[101,1_000_000_021,4_294_967_197,2_305_843_009_213_693_693]}
 fn data(p:u64,shape:&str,s:&mut u64)->Vec<(u64,u64)>{
  let mut out=Vec::new();
  for _ in 0..256{
   let x=next(s)%p;let y=next(s)%p;
   let (b,c)=if shape=="known_roots" {
    let sum=(x+y)%p;
    (if sum==0{0}else{p-sum},((x as u128*y as u128)%(p as u128))as u64)
   }else{(x,y)};
   out.push((b,c));
  }
  out
 }
 #[test]fn equivalence(){
  let mut seed=0x121e_0f01_2026_d58bu64;
  let mut count=0;
  for p in fields(){
   let old=baseline::context(p);let new=candidate::context(p);
   for i in 0..4096{
    let x=next(&mut seed)%p;
    let a=if i%2==0{((x as u128*x as u128)%(p as u128))as u64}else{x};
    let before=baseline::disc(&old,a);
    let after=candidate::disc(&new,a);
    assert_eq!(before,after,"p={p} a={a}");
    if let Some(root)=after{
     assert_eq!(((root as u128*root as u128)%(p as u128))as u64,a);
    }
    count+=1;
   }
   for kind in ["uniform","known_roots"]{
    for (b,c) in data(p,kind,&mut seed){
     assert_eq!(baseline::roots(&old,b,c),candidate::roots(&new,b,c),
         "p={p} kind={kind} b={b} c={c}");
     count+=1;
    }
   }
  }
  println!("ROUND121_JACOBI_EQ comparisons={count}");
 }
 #[test]
 #[ignore="Source-exact paired quadratic root benchmark"]
 fn benchmark(){
  let mut seed=0x121b_0f01_2026_1122u64;
  for p in fields(){
   let old=baseline::context(p);let new=candidate::context(p);
   for shape in ["uniform","known_roots"]{
    let inputs=data(p,shape,&mut seed);
    for &(b,c) in &inputs{
     assert_eq!(baseline::roots(&old,b,c),candidate::roots(&new,b,c));
    }
    let mut before=Vec::new();let mut after=Vec::new();let mut hash_ref=None;
    for trial in 0..13{
     let measure=|optimized:bool|->(f64,u64){
      let start=Instant::now();let mut digest=0x1211_2026_f00d_abcd;
      for _ in 0..64{
       for &(b,c) in &inputs {
        let roots=if optimized{
         candidate::roots(black_box(&new),black_box(b),black_box(c))
        }else{
         baseline::roots(black_box(&old),black_box(b),black_box(c))
        };
        digest=digest.wrapping_mul(0x9e37_79b1_85eb_ca87)^black_box(hash(&roots));
       }
      }
      (start.elapsed().as_nanos() as f64/(64*inputs.len())as f64,digest)
     };
     let (b,a)=if trial%2==0{(measure(false),measure(true))}
       else{let a=measure(true);let b=measure(false);(b,a)};
     assert_eq!(b.1,a.1);
     if let Some(prev)=hash_ref{assert_eq!(prev,a.1);}else{hash_ref=Some(a.1);}
     before.push(b.0);after.push(a.0);
    }
    let b=med(before);let a=med(after);
    println!("ROUND121_JACOBI_RUST p={p} shape={shape} before_ns={b:.3} after_ns={a:.3} speedup={:.4} checksum={}",b/a,hash_ref.unwrap());
   }
  }
 }
}
'''
text=header+frozen+expose+"pub mod candidate{\n"+source+expose+"}\n"+tests
with tempfile.TemporaryDirectory() as temp:
    path=Path(temp)/"round121_jacobi.rs"
    binary=Path(temp)/"round121_jacobi"
    path.write_text(text)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"equivalence","--nocapture"],check=True)
    subprocess.run([str(binary),"benchmark","--ignored","--nocapture"],check=True)
