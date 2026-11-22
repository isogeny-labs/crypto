#!/usr/bin/env python3
"""Round 118 small-leading inverse reuse source-exact Rust A/B.

Reconstruct the pre-small-leading source in memory, compile both together
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
    print(f"ROUND118_PIN_CPU={min(cpus)}", flush=True)
except (AttributeError, OSError):
    print("ROUND118_PIN_CPU=unavailable", flush=True)

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
# Reconstruct pre-small-leading source, retaining the new u32 Euclid path.
start = source.index("    // Cardano/Ferrari setup already caches these small inverses.")
end = source.index("    /// Distinct roots of monic y^2 + b y + c.",start)
frozen = source[:start] + source[end:]
old_quad = "        let inv_2a = f.inv(f.add(a, a));"
new_quad = """        let inv_2a = match a {
            2 => self.inv4,
            3 => f.half(self.inv3),
            4 => f.half(self.inv4),
            _ => f.inv(f.add(a, a)),
        };"""
assert frozen.count(new_quad) == 1
frozen = frozen.replace(new_quad, old_quad)
assert frozen.count("let ia = self.inv_leading(a);") == 2
frozen = frozen.replace("let ia = self.inv_leading(a);","let ia = f.inv(a);")
assert frozen.count("let ia = self.inv_leading(c[3]);") == 1
frozen = frozen.replace("let ia = self.inv_leading(c[3]);","let ia = fp.inv(c[3]);")
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
pub fn product(ctx:&RadicalRootCtx,a:u64,b:u64)->u64 {ctx.f.mul(a,b)}
pub fn inverse(ctx:&RadicalRootCtx,a:u64)->u64 {ctx.f.inv(a)}
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
 fn inverse_equivalence() {
  let mut seed=0x118e_2026_1122_88c3u64;
  let mut count=0usize;
  for p in [5u64,7,11,31,101,257,65_521,1_000_000_007,4_294_967_291,2_305_843_009_213_693_951] {
   let b=baseline::ctx(p);let c=candidate::ctx(p);
   let mut xs=vec![0,1,2,p-1,p-2,p/2,p+1,u64::MAX];
   for _ in 0..8_192 {xs.push(next(&mut seed)%p);}
   for a in xs {
    let old=baseline::inverse(&b,a);
    let new=candidate::inverse(&c,a);
    assert_eq!(old,new,"inverse p={p} a={a}");
    let reduced=a%p;
    if reduced != 0 {
     assert_eq!(((new as u128 * reduced as u128)%(p as u128)) as u64,1,
       "inverse oracle p={p} a={a}");
    } else {assert_eq!(new,0);}
    count+=1;
   }
  }
  println!("ROUND118_SMALL_EQ inverse_cases={count}");
 }
 #[test]
 fn polynomial_equivalence() {
  let mut seed=0x8873_118e_2026_1122u64;
  let mut count=0usize;
  for p in [5u64,65_521,1_000_000_007,4_294_967_291] {
   let b=baseline::ctx(p);let c=candidate::ctx(p);
   for j in 0..768 {
    let x=next(&mut seed)%p;
    let y=next(&mut seed)%p;
    let z=next(&mut seed)%p;
    let leading=[2,3,4][j%3];
    let quadratic=[x,y,leading];
    assert_eq!(baseline::roots(&b,&quadratic),candidate::roots(&c,&quadratic),
      "quad p={p} i={j}");
    let cubic=[x,y,z,leading];
    assert_eq!(baseline::roots(&b,&cubic),candidate::roots(&c,&cubic),
      "cubic p={p} i={j}");
    count+=2;
   }
  }
  println!("ROUND118_SMALL_EQ polynomial_cases={count}");
 }
 #[test]
 #[ignore="Native source-exact paired A/B performance measurements"]
 fn benchmark() {
  let mut seed=0x118d_0257_8a6e_1829u64;
  for p in [65_521u64,1_000_000_007,4_294_967_291,2_305_843_009_213_693_951] {
   let b=baseline::ctx(p);let c=candidate::ctx(p);
   for kind in ["inverse_uniform","quadratic_leading2","quadratic_leading3",
                "quadratic_leading4","cubic_leading3","quartic_zero_leading3",
                "quadratic_nonmonic","cubic_nonmonic"] {
    if p>u32::MAX as u64 && kind != "inverse_uniform" {continue;}
    let mut inputs=Vec::new();
    for _ in 0..256 {
     let a=next(&mut seed)%p;
     let x=next(&mut seed)%p;
     let y=next(&mut seed)%p;
     let leading=2+next(&mut seed)%(p-3);
     let v=match kind {
      "inverse_uniform"=>vec![a],
      "quadratic_leading2"=>vec![a,x,2],
      "quadratic_leading3"=>vec![a,x,3],
      "quadratic_leading4"=>vec![a,x,4],
      "cubic_leading3"=>vec![a,x,y,3],
      "quartic_zero_leading3"=>vec![0,a,x,y,3],
      "quadratic_nonmonic"=>vec![a,x,leading],
      "cubic_nonmonic"=>vec![a,x,y,leading],
      _=>unreachable!(),
     };
     inputs.push(v);
    }
    let before_fn=|x:&Vec<u64>| -> u64 {
     if kind.starts_with("inverse") { baseline::inverse(&b,x[0]) }
     else { hash(&baseline::roots(&b,x)) }
    };
    let after_fn=|x:&Vec<u64>| -> u64 {
     if kind.starts_with("inverse") { candidate::inverse(&c,x[0]) }
     else { hash(&candidate::roots(&c,x)) }
    };
    for x in &inputs {assert_eq!(before_fn(x),after_fn(x),"p={p} kind={kind} x={x:?}");}
    let loops=match kind {
     "inverse_uniform"=>96,
     "quadratic_leading2"|"quadratic_leading3"|"quadratic_leading4"|"quadratic_nonmonic"=>48,
     "cubic_leading3"|"cubic_nonmonic"=>8,
     "quartic_zero_leading3"=>4,
     _=>unreachable!()
    };
    let mut before=Vec::new();let mut after=Vec::new();
    let mut reference=None;
    for trial in 0..13 {
     let measure=|new:bool| -> (f64,u64) {
      let start=Instant::now();
      let mut digest=0x8e1a_118d_9022_1027u64;
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
    println!("ROUND118_SMALL_RUST p={p} kind={kind} before_ns={bv:.3} after_ns={av:.3} speedup={:.4} checksum={}",bv/av,reference.unwrap());
   }
  }
 }
}
'''
source_out = header + frozen + expose + "pub mod candidate {\n" + source + expose + "}\n" + tests
with tempfile.TemporaryDirectory() as tmp:
    f = Path(tmp) / "round118.rs"
    exe = Path(tmp) / "round118"
    f.write_text(source_out)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3",
                    "--test",str(f),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
