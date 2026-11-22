#!/usr/bin/env python3
"""Round115: source-exact native Rust A/B, frozen pre-Barrett versus current file.
Runs with bare rustc, no Cargo, network, or unrelated repository dependencies.
The two module bodies differ ONLY in the cached reciprocal and mul reduction.
"""
from pathlib import Path
import os
import subprocess
import tempfile

# Pin all source-exact A/B subprocesses to one allowed CPU.
try:
    allowed = os.sched_getaffinity(0)
    if allowed:
        cpu = min(allowed)
        os.sched_setaffinity(0, {cpu})
        print(f"ROUND115_CPU_PIN cpu={cpu}", flush=True)
except (AttributeError, OSError):
    print("ROUND115_CPU_PIN unavailable", flush=True)

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
new_struct = """pub(crate) struct Fp {
    pub p: u64,
    // floor(2^64 / p): quotient estimate for canonical 32-bit products.
    // Calculated once per field rather than dividing on every multiply.
    reciprocal_64: u64,
}
impl Fp {
    pub fn new(p: u64) -> Self {
        let reciprocal_64 = if p >= 2 && p <= u32::MAX as u64 {
            ((1u128 << 64) / p as u128) as u64
        } else {
            0
        };
        Fp { p, reciprocal_64 }
    }"""
old_struct = """pub(crate) struct Fp {
    pub p: u64,
}
impl Fp {
    pub fn new(p: u64) -> Self {
        Fp { p }
    }"""
new_mul = """        } else if self.p <= u32::MAX as u64 && a < self.p && b < self.p {
            // Division-free 64-bit Barrett reduction. Let x=a*b < 2^64
            // and mu=floor(2^64/p). Then floor(x*mu/2^64) differs
            // from floor(x/p) by at most one (and never exceeds it).
            // The remainder is in [0,2p), so one subtraction suffices.
            // This uses a u128 widening multiply instead of integer div.
            let product = a * b;
            let quotient = ((product as u128 * self.reciprocal_64 as u128) >> 64) as u64;
            let remainder = product - quotient * self.p;
            if remainder >= self.p { remainder - self.p } else { remainder }
        } else {"""
old_mul = """        } else if self.p <= u32::MAX as u64 && a < self.p && b < self.p {
            // Canonical 32-bit operands fit in a native 64-bit product.
            (a * b) % self.p
        } else {"""
baseline = source
for new, old in [(new_struct, old_struct), (new_mul, old_mul)]:
    assert baseline.count(new) == 1, "baseline reconstruction failed"
    baseline = baseline.replace(new, old)

header = r'''
#![allow(dead_code, unused_imports)]
mod cryptanalysis {
pub mod prime_root_trace {
 pub const MAX_PRIME:u64=(1u64<<61)-1;
 pub const MAX_WORK:u64=1_000_000;
 pub struct RootLimits {pub field_work:u64,pub split_attempts:u64,pub events:u64,pub degree:usize}
 #[derive(Debug,PartialEq,Eq)] pub enum RootStop {Complete}
 pub struct RootReport {pub stop:RootStop,pub roots:Vec<u64>}
 pub struct RootEvent;
 pub fn solve(_: &[u64],_:u64,_:u64,_:RootLimits,_:&mut dyn FnMut(&RootEvent)->Result<(),String>)
 -> Result<RootReport,String> { panic!("unexpected quartic Frobenius fallback") }
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool {
   matches!(p,5|7|11|31|101|257|65_521|1_000_000_007|4_294_967_291|2_305_843_009_213_693_951)
 }
}
pub mod baseline {
'''
expose = r'''
pub fn f(p:u64)->Fp { Fp::new(p) }
pub fn mul(f:&Fp,a:u64,b:u64)->u64{f.mul(a,b)}
pub fn power(f:&Fp,a:u64,e:u64)->u64{f.powm(a,e)}
pub fn fq2(f:Fp,n:u64,x:(u64,u64),y:(u64,u64))->(u64,u64){
 use self::Ring;
 Fq2{f,n}.mul(x,y)
}
pub fn ctx(p:u64)->RadicalRootCtx { RadicalRootCtx::new(p).unwrap() }
}
pub mod candidate {
'''
footer = expose.replace("}\npub mod candidate {", "}\n}\n")
tests = r'''
#[cfg(test)]
mod tests {
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64{
   *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *s
 }
 fn med(mut a:Vec<f64>)->f64 {a.sort_by(|x,y|x.total_cmp(y));a[a.len()/2]}
 #[test]
 fn exact_equivalence(){
   let mut seed=0xbd8e_5419_c07a_236f_u64;
   let mut count=0;
   for p in [5u64,7,11,31,257,65_521,1_000_000_007,4_294_967_291,(1u64<<61)-1]{
     let b=baseline::f(p);let c=candidate::f(p);
     for _ in 0..10000{
       let a=next(&mut seed)%p;let x=next(&mut seed)%p;let e=next(&mut seed)&0xffff;
       let oracle=((a as u128*x as u128)%p as u128)as u64;
       assert_eq!(baseline::mul(&b,a,x),oracle,"baseline mul");
       assert_eq!(candidate::mul(&c,a,x),oracle,"candidate mul");
       count+=2;
       if count%37==0 {
         assert_eq!(baseline::power(&b,a,e),candidate::power(&c,a,e));
         count+=1;
       }
     }
     for _ in 0..1024{
       let x=(next(&mut seed)%p,next(&mut seed)%p);
       let y=(next(&mut seed)%p,next(&mut seed)%p);
       let n=if p%8==3||p%8==5 {2}else{3};
       assert_eq!(baseline::fq2(b,n,x,y),candidate::fq2(c,n,x,y));
       count+=1;
     }
     if p==65_521 || p==1_000_000_007 || p==(1u64<<61)-1{
       let old=baseline::ctx(p);let new=candidate::ctx(p);
       for _ in 0..160{
         let a=next(&mut seed)%p;let x=next(&mut seed)%p;
         let mut r=old.quad_monic(a,x);r.sort_unstable();
         let mut s=new.quad_monic(a,x);s.sort_unstable();
         assert_eq!(r,s);
         let mut r=old.cubic_depressed(a,x);r.sort_unstable();
         let mut s=new.cubic_depressed(a,x);s.sort_unstable();
         assert_eq!(r,s);
         count+=2;
       }
     }
   }
   println!("ROUND115_EQ checks={count}");
 }
 #[test]
 #[ignore="13 alternating A/B trials per workload"]
 fn benchmark() {
  let mut seed=0xa047_587c_199e_21bd_u64;
  for p in [65_521u64,1_000_000_007,4_294_967_291,(1u64<<61)-1]{
   let b=baseline::f(p);let c=candidate::f(p);
   let n=if p%8==3||p%8==5 {2}else{3};
   let old_ctx=if p!=4_294_967_291{Some(baseline::ctx(p))}else{None};
   let new_ctx=if p!=4_294_967_291{Some(candidate::ctx(p))}else{None};
   let mut inputs=Vec::new();
   for _ in 0..512{inputs.push((next(&mut seed)%p,next(&mut seed)%p,next(&mut seed)%p));}
   for kind in ["mul","fq2_mul","power","quad","cubic"] {
    if kind=="cubic" && p==4_294_967_291 {continue;}
    if kind=="quad" && p==4_294_967_291 {continue;}
    // Multi-millisecond measurement windows avoid per-microbenchmark timer noise.
    let loops=match kind {
      "mul"=>2048, "fq2_mul"=>1024, "power"=>64,
      "quad"=>96, "cubic"=>32, _=>unreachable!(),
    };
    let mut before=Vec::new();let mut after=Vec::new();
    let mut expected=None;
    for trial in 0..13 {
      let measure=|candidate_mode:bool|->(f64,u64){
        let start=Instant::now();let mut digest=0x2e2d_6bea_3849_19df_u64;
        for _ in 0..loops {for &(a,x,z) in &inputs {
          let v=match kind {
           "mul"=>if candidate_mode{candidate::mul(&c,a,x)}else{baseline::mul(&b,a,x)},
           "fq2_mul"=>{
             let y=(x,z);
             let out=if candidate_mode{candidate::fq2(c,n,(a,x),y)}else{baseline::fq2(b,n,(a,x),y)};
             out.0.wrapping_mul(131).wrapping_add(out.1)
           },
           "power"=>if candidate_mode{candidate::power(&c,a,p/2)}else{baseline::power(&b,a,p/2)},
           "quad"=>{
             let mut v=if candidate_mode{new_ctx.as_ref().unwrap().quad_monic(a,z)}
                 else{old_ctx.as_ref().unwrap().quad_monic(a,z)};
             v.sort_unstable();v.into_iter().fold(0u64,|h,q|h.wrapping_mul(131).wrapping_add(q))
           },
           "cubic"=>{
             let mut v=if candidate_mode{new_ctx.as_ref().unwrap().cubic_depressed(a,z)}
                 else{old_ctx.as_ref().unwrap().cubic_depressed(a,z)};
             v.sort_unstable();v.into_iter().fold(0u64,|h,q|h.wrapping_mul(131).wrapping_add(q))
           },
           _=>unreachable!()
          };
          digest=digest.rotate_left(7).wrapping_add(black_box(v));
        }}
        (start.elapsed().as_nanos() as f64/(inputs.len()*loops)as f64,digest)
      };
      let (bv,av)=if trial%2==0{(measure(false),measure(true))}else{let a=measure(true);let b=measure(false);(b,a)};
      assert_eq!(bv.1,av.1,"kind={kind} p={p} trial={trial}");
      if let Some(w)=expected {assert_eq!(w,av.1,"nondeterministic checksum");}else{expected=Some(av.1);}
      before.push(bv.0);after.push(av.0);
    }
    let b=med(before);let a=med(after);
    println!("ROUND115_RUST p={p} kind={kind} before_ns={b:.3} after_ns={a:.3} speedup={:.4} checksum={}",b/a,expected.unwrap());
   }
  }
 }
}
'''
assembled = header + baseline + expose + source + footer + tests
with tempfile.TemporaryDirectory() as tmp:
    root=Path(tmp)
    rs=root/"round115.rs"
    exe=root/"round115"
    rs.write_text(assembled)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(rs),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"exact_equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
