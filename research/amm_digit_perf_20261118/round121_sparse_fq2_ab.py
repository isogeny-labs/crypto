#!/usr/bin/env python3
"""Round 121 source-exact Rust Fp2 multiplication A/B harness.

Compiles the current production prime_radical_root.rs with a self-contained
stub module; compares Ring::mul against the exact pre-round121 Karatsuba body.
Benchmarks use black_box, alternating A/B trials and checksum parity.
"""
from pathlib import Path
import os
import subprocess
import tempfile

try:
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    print(f"ROUND121_CPU={cpu}", flush=True)
except (AttributeError, OSError):
    print("ROUND121_CPU=unavailable", flush=True)

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
header = r'''
#![allow(dead_code, unused_imports, private_interfaces)]
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
    matches!(p,5|7|11|101|257|65_521|1_000_000_007|
      4_294_967_291|1_000_000_021|4_294_967_197|
      2_305_843_009_213_693_693|2_305_843_009_213_693_951)
 }
}
pub mod candidate {
'''
expose = r'''
pub fn q(p:u64) -> Fq2 {Fq2{f:Fp::new(p),n:2}}
pub fn optimized(q:&Fq2,x:(u64,u64),y:(u64,u64))->(u64,u64) {
    q.mul(x,y)
}
pub fn frozen(q:&Fq2,x:(u64,u64),y:(u64,u64))->(u64,u64) {
    // Frozen pre-round121 Fq2::mul, identical arithmetic for dense and sparse.
    let f = &q.f;
    let ac = f.mul(x.0, y.0);
    let bd = f.mul(x.1, y.1);
    let nbd = match q.n {
        2 => f.add(bd, bd),
        3 => f.add(f.add(bd, bd), bd),
        _ => f.mul(q.n, bd),
    };
    let s = f.mul(f.add(x.0, x.1), f.add(y.0, y.1));
    (f.add(ac, nbd), f.sub(f.sub(s, ac), bd))
}
pub fn oracle(p:u64,x:(u64,u64),y:(u64,u64))->(u64,u64) {
    let w=|a:u64,b:u64| -> u64 {
      ((a as u128*b as u128)%(p as u128)) as u64
    };
    (((w(x.0,y.0)+2*w(x.1,y.1)) as u128 % (p as u128)) as u64,
     ((w(x.0,y.1)+w(x.1,y.0)) as u128 % (p as u128)) as u64)
}
pub fn disc(p:u64,a:u64)->Option<u64> {RadicalRootCtx::new(p).unwrap().sqrt_discriminant(a)}
pub fn sqrt(p:u64,a:u64)->Option<u64> {RadicalRootCtx::new(p).unwrap().sqrt(a)}
}
}
'''
tests = r'''
#[cfg(test)]
mod tests {
 use super::cryptanalysis::candidate;
 use std::{hint::black_box,time::Instant};
 fn next(x:&mut u64)->u64 {*x^=*x<<13;*x^=*x>>7;*x^=*x<<17;*x}
 fn median(mut v:Vec<f64>)->f64{v.sort_by(|a,b|a.total_cmp(b));v[v.len()/2]}
 fn checksum(a:(u64,u64))->u64{a.0.wrapping_mul(0x9e37_79b1_85eb_ca87)^a.1}

 #[test]
 fn equivalence() {
    let mut seed=0x1211_2026_1122_b377u64;
    let mut comparisons=0;
    for p in [65_521u64,1_000_000_007,(1u64<<61)-1] {
        let q=candidate::q(p);
        for i in 0..100_000 {
            let mut x=(next(&mut seed)%p,next(&mut seed)%p);
            let mut y=(next(&mut seed)%p,next(&mut seed)%p);
            if i%2==0{x.1=0;}
            if i%5==0{y.1=0;}
            let a=candidate::optimized(&q,x,y);
            let b=candidate::frozen(&q,x,y);
            let c=candidate::oracle(p,x,y);
            assert_eq!(a,b,"p={p} i={i}");
            assert_eq!(a,c,"p={p} i={i}");
            comparisons+=1;
        }
    }
    // Includes every new narrow-Jacobi modulus and a 61-bit control.
    for p in [101u64,1_000_000_021,4_294_967_197,4_294_967_291,
              2_305_843_009_213_693_693] {
        let mut a=3;
        for i in 0..512 {
            a=next(&mut seed)%p;
            let square=((a as u128*a as u128)%(p as u128)) as u64;
            assert_eq!(candidate::disc(p,square),candidate::sqrt(p,square),"p={p} i={i}");
            comparisons+=1;
        }
    }
    println!("ROUND121_EQ comparisons={comparisons}");
 }
 #[test]
 #[ignore="Source-exact paired benchmark"]
 fn benchmark() {
    let mut seed=0x1211_2026_1122_a65bu64;
    for p in [65_521u64,1_000_000_007,(1u64<<61)-1] {
        let q=candidate::q(p);
        for shape in ["dense","left_sparse","right_sparse","both_sparse","half_sparse"] {
            let mut xs=Vec::new();
            for i in 0..256 {
                let mut x=(next(&mut seed)%p,next(&mut seed)%p);
                let mut y=(next(&mut seed)%p,next(&mut seed)%p);
                if shape=="left_sparse" || shape=="both_sparse" || shape=="half_sparse" && i%2==0 {x.1=0;}
                if shape=="right_sparse" || shape=="both_sparse" {y.1=0;}
                xs.push((x,y));
            }
            for &(x,y) in &xs {
                assert_eq!(candidate::optimized(&q,x,y),candidate::frozen(&q,x,y));
            }
            let mut b=Vec::new();let mut a=Vec::new();let mut last=None;
            for trial in 0..13 {
                let measure=|new:bool|->(f64,u64) {
                    let start=Instant::now();
                    let mut sum=0x1211_ffff_2026_1122u64;
                    for _ in 0..160 {
                        for &(x,y) in &xs {
                            let r=if new {
                                candidate::optimized(black_box(&q),black_box(x),black_box(y))
                            } else {
                                candidate::frozen(black_box(&q),black_box(x),black_box(y))
                            };
                            sum=sum.rotate_left(7).wrapping_mul(0x9e37_79b1_85eb_ca87)
                                ^ black_box(checksum(r));
                        }
                    }
                    (start.elapsed().as_nanos()as f64/(160*xs.len())as f64,sum)
                };
                let (old,new)=if trial%2==0 {
                    (measure(false),measure(true))
                } else {
                    let n=measure(true);let o=measure(false);(o,n)
                };
                assert_eq!(old.1,new.1,"p={p} shape={shape}");
                if let Some(prev)=last{assert_eq!(prev,new.1);}else{last=Some(new.1);}
                b.push(old.0);a.push(new.0);
            }
            let bv=median(b);let av=median(a);
            println!("ROUND121_RUST p={p} shape={shape} before_ns={bv:.3} after_ns={av:.3} speedup={:.4} checksum={}",bv/av,last.unwrap());
        }
    }
 }
}
'''
content = header + source + expose + tests
with tempfile.TemporaryDirectory() as tmp:
    file=Path(tmp)/"round121.rs"
    exe=Path(tmp)/"round121"
    file.write_text(content)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(file),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
