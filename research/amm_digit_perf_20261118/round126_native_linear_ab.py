#!/usr/bin/env python3
"""Round 126: exact production Rust A/B for small-coefficient linear roots.
The baseline restores only the original linear-root match arm. Field arithmetic
and all other compiler options are identical. Unused tracing dependencies are
stubbed in the isolated harness; full root operations use the actual Rust source.
The chosen polynomial inputs do not invoke the Frobenius fallback.
"""
import os
import subprocess
import tempfile
from pathlib import Path

try:
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    print(f"ROUND126_CPU={cpu}", flush=True)
except (OSError, AttributeError):
    print("ROUND126_CPU=unavailable", flush=True)

s = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
new_linear = """                // The context already holds inverses of 3 and 4 for
                // Cardano/Ferrari. Cache hits beat Euclid at large primes;
                // native Rust tests rejected the halving shortcut at a=2,
                // and showed regressions on smaller prime fields.
                let root = if c[0] == 0 { 0 }
                    else if c[1] == 1 { fp.neg(c[0]) }
                    else {
                        let neg_c0 = fp.neg(c[0]);
                        let inverse = if fp.p >= (1u64 << 20) {
                            match c[1] {
                                3 => self.inv3,
                                4 => self.inv4,
                                _ => fp.inv(c[1]),
                            }
                        } else {
                            fp.inv(c[1])
                        };
                        fp.mul(neg_c0, inverse)
                    };"""
old_linear = """                let root = if c[0] == 0 { 0 }
                    else if c[1] == 1 { fp.neg(c[0]) }
                    else { fp.mul(fp.neg(c[0]), fp.inv(c[1])) };"""
assert s.count(new_linear) == 1, "source no longer matches Round126 optimization"
frozen = s.replace(new_linear, old_linear, 1)
assert frozen != s

header = r"""
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
 ->Result<RootReport,String>{panic!("unexpected Frobenius fallback in benchmark")}
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool {
  matches!(p,5|7|65_521|1_000_000_007|4_294_967_291|2_305_843_009_213_693_951)
 }
}
pub mod baseline {
"""
expose = r"""
pub fn ctx(p:u64)->RadicalRootCtx{RadicalRootCtx::new(p).unwrap()}
pub fn roots(c:&RadicalRootCtx,p:&[u64])->RadicalRoots{c.roots(p).unwrap()}
}
"""
tests = r"""
#[cfg(test)]
mod checks {
use super::cryptanalysis::{baseline,candidate};
use std::{hint::black_box,time::Instant};

fn next(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
fn median(mut x:Vec<f64>)->f64{x.sort_by(f64::total_cmp);x[x.len()/2]}
fn hash(h:u64,x:u64)->u64{h.rotate_left(7).wrapping_mul(0x9e37_79b1_85eb_ca87)^x}
fn inputs(p:u64,kind:usize,n:usize)->Vec<Vec<u64>>{
 let mut state=0x125a_2026_1122_7654u64^p^kind as u64;
 let c=baseline::ctx(p);
 let f=&c.f;
 (0..n).map(|i|{
  let r=next(&mut state)%p;
  let t=next(&mut state)%p;
  let sum=f.neg(f.add(r,t));
  let prod=f.mul(r,t);
  let mut poly=match kind%5{
   0=>vec![f.neg(r),2], // linear: half shortcut
   1=>vec![f.neg(r),3], // linear: cached inv3
   2=>vec![f.neg(r),4], // linear: cached inv4
   3=>vec![f.neg(r),(r%(p-1))+1], // random leading control
   _=>vec![prod,sum,1], // full quadratic control
  };
  if (kind%10)>=5 {
   for x in &mut poly {*x+=p;}
  }
  if kind>=10 {poly.extend([0,p,2*p,0]);}
  if i%29==0 && kind>=10 {poly.push(0);}
  poly
 }).collect()
}
fn digest(roots:&[u64])->u64{roots.iter().fold(roots.len() as u64,|h,&x|hash(h,x))}
#[test]
fn correctness(){
 let mut count=0;
 for p in [5,7,65_521,1_000_000_007,4_294_967_291,(1u64<<61)-1]{
  let x=baseline::ctx(p);let y=candidate::ctx(p);
  for kind in 0..20 {
   for q in inputs(p,kind,256){
    let before=baseline::roots(&x,&q);
    let after=candidate::roots(&y,&q);
    assert_eq!(before.roots,after.roots,"p={p} kind={kind} q={q:?}");
    assert_eq!(before.frobenius_fallback,after.frobenius_fallback);
    let reduced:Vec<_>=q.iter().map(|&v|v%p).collect();
    for root in after.roots {
     let eval=reduced.iter().rev().fold(0u64,|acc,&v|y.f.add(y.f.mul(acc,root),v));
     assert_eq!(eval,0,"p={p} kind={kind} q={q:?} root={root}");
    }
    count+=1;
   }
  }
 }
 println!("ROUND126_EQ_COMPARISONS={count}");
}
#[test]
#[ignore="13 alternating source-exact Rust trials"]
fn benchmark(){
 for p in [65_521u64,1_000_000_007,4_294_967_291,(1u64<<61)-1]{
  let x=baseline::ctx(p);let y=candidate::ctx(p);
  for kind in [0usize,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19]{
   let q=inputs(p,kind,128);
   let mut before=Vec::new();let mut after=Vec::new();
   let measure=|opt:bool|->(f64,u64){
    let start=Instant::now();let mut h=0x125abc_u64;
    for _ in 0..20 {
     for poly in &q {
      let roots=if opt{candidate::roots(black_box(&y),black_box(poly)).roots}
                else {baseline::roots(black_box(&x),black_box(poly)).roots};
      h=hash(h,black_box(digest(&roots)));
     }
    }
    (start.elapsed().as_nanos() as f64/(20*q.len()) as f64,h)
   };
   for trial in 0..13 {
    let (b,a)=if trial%2==0{(measure(false),measure(true))}
              else{let a=measure(true);let b=measure(false);(b,a)};
    assert_eq!(a.1,b.1,"benchmark mismatch");
    before.push(b.0);after.push(a.0);
   }
   let b=median(before);let a=median(after);
   println!("ROUND126_RUST p={p} kind={kind} before_ns={b:.3} after_ns={a:.3} speedup={:.4}",b/a);
  }
 }
}
}
"""
source=header+frozen+expose+"pub mod candidate {\n"+s+expose+"}\n"+tests
with tempfile.TemporaryDirectory() as temp:
    path=Path(temp)/"round126.rs"
    exe=Path(temp)/"round126"
    path.write_text(source)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(path),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"correctness","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
