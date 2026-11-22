#!/usr/bin/env python3
"""Round 117 source-exact Rust equivalence and paired end-to-end root timing.

Reconstruct the frozen pre-round117 source in memory, compile both together
with rustc, compare results, and time 13 alternating A/B trials. No Cargo or
network dependency and no binary root fallback used by measured workloads.
"""
import os
from pathlib import Path
import subprocess
import tempfile

try:
    cpus = os.sched_getaffinity(0)
    os.sched_setaffinity(0, {min(cpus)})
    print(f"ROUND117_PIN_CPU={min(cpus)}", flush=True)
except (AttributeError, OSError):
    print("ROUND117_PIN_CPU=unavailable", flush=True)

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
chain_start = source.index("    // For p=2^32-5, sqrt exponent")
chain_end = source.index("    #[inline]\n    pub fn half", chain_start)
frozen = source[:chain_start] + source[chain_end:]

new_sqrt = """        if p % 4 == 3 {
            let r = if p == 4_294_967_291 && a < p {
                f.sqrt_power_4294967291(a)
            } else {
                f.powm(a, (p + 1) / 4)
            };
            return if f.sqr(r) == a { Some(r) } else { None };
        }"""
old_sqrt = """        if p % 4 == 3 {
            let r = f.powm(a, (p + 1) / 4);
            return if f.sqr(r) == a { Some(r) } else { None };
        }"""
assert frozen.count(new_sqrt) >= 1
frozen = frozen.replace(new_sqrt, old_sqrt, 1)
assert frozen != source

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
 -> Result<RootReport,String> { panic!("unexpected Frobenius fallback"); }
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool {
   matches!(p,5|7|11|31|101|257|65_521|1_000_000_007|4_294_967_291|2_305_843_009_213_693_951)
 }
}
pub mod baseline {
'''
expose = r'''
pub fn ctx(p:u64)->RadicalRootCtx {RadicalRootCtx::new(p).unwrap()}
pub fn disc(ctx:&RadicalRootCtx,a:u64)->Option<u64> {ctx.sqrt_discriminant(a)}
pub fn sqrt(ctx:&RadicalRootCtx,a:u64)->Option<u64> {ctx.sqrt(a)}
pub fn square(ctx:&RadicalRootCtx,a:u64)->u64 {ctx.f.sqr(a)}
pub fn character(ctx:&RadicalRootCtx,a:u64)->bool {
 a==0 || ctx.f.powm(a,(ctx.f.p-1)/2)==1
}
pub fn roots(ctx:&RadicalRootCtx, x:&[u64])->Vec<u64>{
 ctx.roots(x).unwrap().roots
}
}
'''
tests = r'''
#[cfg(test)]
mod tests {
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(seed:&mut u64)->u64 {
  *seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;*seed
 }
 fn med(mut a:Vec<f64>)->f64 {a.sort_by(|x,y|x.total_cmp(y));a[a.len()/2]}
 fn hash(xs:&[u64])->u64 {
  xs.iter().fold(0xd215_b391_29a7_04e1u64,|h,&v|h.rotate_left(7).wrapping_mul(0x9e3779b185ebca87)^v)
 }
 #[test]
 fn equivalence() {
  let mut seed=0x117e_2026_1122_88c3u64;
  let mut count=0usize;
  for p in [1_000_000_007u64,4_294_967_291] {
   let b=baseline::ctx(p);let c=candidate::ctx(p);
   for j in 0..8_192 {
    let x=next(&mut seed)%p;
    let a=if j%2==0 {baseline::square(&b,x)} else {x};
    let old=baseline::disc(&b,a);let new=candidate::disc(&c,a);
    assert_eq!(old,new,"discriminant p={p} a={a}");
    assert_eq!(new.is_some(),baseline::character(&b,a),"Euler p={p} a={a}");
    if let Some(v)=new {assert_eq!(baseline::square(&b,v),a,"p={p} a={a}");}
    assert_eq!(baseline::sqrt(&b,a),candidate::sqrt(&c,a));
    count+=1;
    if j%8==0 {
      let poly=[next(&mut seed)%p,next(&mut seed)%p,1u64];
      assert_eq!(baseline::roots(&b,&poly),candidate::roots(&c,&poly));
      count+=1;
    }
   }
   for a in [0u64,1,2,p-1,p-2,p,p+1,p+2,u64::MAX] {
    assert_eq!(baseline::disc(&b,a),candidate::disc(&c,a),"p={p} a={a}");
    count+=1;
   }
  }
  println!("ROUND117_EQ comparisons={count}");
 }
 #[test]
 #[ignore="Native source-exact paired A/B performance measurements"]
 fn benchmark() {
  let mut seed=0x117d_0257_8a6e_1829u64;
  for p in [1_000_000_007u64,4_294_967_291] {
   let b=baseline::ctx(p);let c=candidate::ctx(p);
   for kind in ["sqrt_power_squares","discriminant_uniform",
                "discriminant_known_squares","quadratic_uniform",
                "quadratic_known_roots","cubic_uniform"] {
    let mut inputs=Vec::new();
    for _ in 0..256 {
     let a=next(&mut seed)%p;
     let x=next(&mut seed)%p;
     let y=next(&mut seed)%p;
     let v=match kind {
      "sqrt_power_squares"|"discriminant_known_squares" => vec![baseline::square(&b,a)],
      "discriminant_uniform" =>vec![a],
      "quadratic_uniform" => vec![a,x,1],
      "quadratic_known_roots" => {
       let sum=(a+x)%p;
       let negative_sum=if sum==0 {0} else {p-sum};
       vec![baseline::square(&b,a)%p,negative_sum,1]
      },
      "cubic_uniform" => vec![a,x,y,1],
      _=>unreachable!(),
     };
     inputs.push(v);
    }
    let before_fn=|x:&Vec<u64>| -> u64 {
     if kind=="sqrt_power_squares" {
      baseline::sqrt(&b,x[0]).unwrap_or(u64::MAX)
     } else if kind.starts_with("discriminant") {
      baseline::disc(&b,x[0]).unwrap_or(u64::MAX)
     } else {
      hash(&baseline::roots(&b,x))
     }
    };
    let after_fn=|x:&Vec<u64>| -> u64 {
     if kind=="sqrt_power_squares" {
      candidate::sqrt(&c,x[0]).unwrap_or(u64::MAX)
     } else if kind.starts_with("discriminant") {
      candidate::disc(&c,x[0]).unwrap_or(u64::MAX)
     } else {
      hash(&candidate::roots(&c,x))
     }
    };
    for x in &inputs {assert_eq!(before_fn(x),after_fn(x),"p={p} kind={kind} x={x:?}");}
    let loops=match kind {
     "sqrt_power_squares"|"discriminant_uniform"|"discriminant_known_squares"=>96,
     "quadratic_uniform"|"quadratic_known_roots"=>48,
     "cubic_uniform"=>8,
     _=>unreachable!()
    };
    let mut before=Vec::new();let mut after=Vec::new();
    let mut reference=None;
    for trial in 0..13 {
     let measure=|new:bool| -> (f64,u64) {
      let start=Instant::now();
      let mut digest=0x8e1a_117d_9022_1027u64;
      for _ in 0..loops {for input in &inputs {
       let r=if new {after_fn(black_box(input))} else {before_fn(black_box(input))};
       digest=digest.wrapping_mul(0x9e3779b185ebca87).wrapping_add(black_box(r));
      }}
      (start.elapsed().as_nanos()as f64/(loops*inputs.len())as f64,digest)
     };
     let (bv,av)=if trial%2==0 {(measure(false),measure(true))}
       else {let a=measure(true);let b=measure(false);(b,a)};
     assert_eq!(bv.1,av.1,"p={p} kind={kind} trial={trial}");
     if let Some(prev)=reference {assert_eq!(prev,av.1);} else {reference=Some(av.1);}
     before.push(bv.0);after.push(av.0);
    }
    let bv=med(before);let av=med(after);
    println!("ROUND117_RUST p={p} kind={kind} before_ns={bv:.3} after_ns={av:.3} speedup={:.4} checksum={}",bv/av,reference.unwrap());
   }
  }
 }
}
'''
source_out = header + frozen + expose + "pub mod candidate {\n" + source + expose + "}\n" + tests
with tempfile.TemporaryDirectory() as tmp:
    f = Path(tmp) / "round117.rs"
    exe = Path(tmp) / "round117"
    f.write_text(source_out)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3",
                    "--test",str(f),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
