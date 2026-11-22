#!/usr/bin/env python3
"""Round114 source-exact Rust A/B harness, no Cargo or network dependency.

Reconstruct the exact prior implementation by reversing only three tagged
changes in prime_radical_root.rs. All other source is byte-for-byte identical.
"""
from pathlib import Path
import subprocess
import tempfile

path = Path("src/cryptanalysis/prime_radical_root.rs")
new = path.read_text().split("\n#[cfg(test)]\n", 1)[0]
old = new
changes = [
("""            // For p = 2^61-1, sqrt(a) = a^((p+1)/4) = a^(2^59).
            // Avoid binary-power bit tests and the identity multiply.
            // This is public root-finding arithmetic, never secret data.
            if e == (1u64 << 59) {
                let mut root = if a < MAX_PRIME { a } else { a % MAX_PRIME };
                for _ in 0..59 {
                    root = self.sqr(root);
                }
                return root;
            }
""", ""),
("""        // Unit/half-unit inverses recur in monic normalization and radicals.
        // The formulas hold in every supported odd prime field.
        if r1 == 1 { return 1; }
        if r1 == self.p - 1 { return self.p - 1; }
        if r1 == 2 { return (self.p + 1) / 2; }
        if r1 == self.p - 2 { return (self.p - 1) / 2; }
""", ""),
("""        // For p = 2^61-1, the square-root power is 2^59.
        // Squaring 59 times and checking r^2=a beats running the
        // variable-time Jacobi prefilter first, especially for residues.
        // Noncanonical inputs retain the prior standalone sqrt behavior.
        if p == MAX_PRIME && a < p {
            return self.sqrt(a);
        }
""", ""),
]
for key, replacement in changes:
    assert old.count(key) == 1, f"cannot isolate exactly one new change: {key[:65]}"
    old = old.replace(key, replacement)

head = r"""
#![allow(dead_code, unused_imports)]
mod cryptanalysis {
pub mod prime_root_trace {
 pub const MAX_PRIME:u64=(1u64<<61)-1;
 pub const MAX_WORK:u64=1_000_000;
 pub struct RootLimits { pub field_work:u64,pub split_attempts:u64,pub events:u64,pub degree:usize }
 #[derive(Debug, PartialEq, Eq)] pub enum RootStop {Complete}
 pub struct RootReport {pub stop:RootStop,pub roots:Vec<u64>}
 pub struct RootEvent;
 pub fn solve(_: &[u64],_:u64,_:u64,_:RootLimits,_:&mut dyn FnMut(&RootEvent)->Result<(),String>)
 -> Result<RootReport,String> {panic!("Unexpected quartic Frobenius fallback in benchmark")}
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool {
   matches!(p,5|7|11|31|101|65_521|1_000_000_007|2_305_843_009_213_693_951)
 }
}
pub mod baseline {
"""
between = r"""
pub fn test_inverse(p:u64,a:u64)->u64 {Fp::new(p).inv(a)}
pub fn test_power(p:u64,a:u64,e:u64)->u64 {Fp::new(p).powm(a,e)}
pub fn test_discriminant(p:u64,a:u64)->Option<u64> {
 RadicalRootCtx::new(p).unwrap().sqrt_discriminant(a)
}
pub fn test_quad(p:u64,b:u64,c:u64)->Vec<u64>{
 RadicalRootCtx::new(p).unwrap().quad_monic(b,c)
}
pub fn test_cubic(p:u64,b:u64,c:u64)->Vec<u64>{
 RadicalRootCtx::new(p).unwrap().cubic_depressed(b,c)
}
}
pub mod candidate {
"""
tail = between.replace("}\npub mod candidate {", "}\n}\n")
tests = r"""
}
#[cfg(test)]
mod checks {
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64 {
  *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*s
 }
 fn median(mut xs:Vec<f64>)->f64{xs.sort_by(|a,b|a.total_cmp(b));xs[xs.len()/2]}
 fn isqrt_equal(x:Option<u64>,y:Option<u64>,p:u64,a:u64){
  assert_eq!(x.is_some(),y.is_some(),"sqrt existence p={p} a={a}");
  if let Some(r)=x {assert_eq!((r as u128*r as u128)%(p as u128), (a%p) as u128);}
  if let Some(r)=y {assert_eq!((r as u128*r as u128)%(p as u128), (a%p) as u128);}
 }
 #[test]
 fn exact_equivalence() {
  let mut checks=0usize;
  for p in [5u64,7,11,31,101,65_521,1_000_000_007,(1u64<<61)-1] {
   let mut s=p^0x517f_90dd_1021_3829;
   for a in [0,1,2,p-2,p-1,p,p+1,u64::MAX] {
    assert_eq!(baseline::test_inverse(p,a), candidate::test_inverse(p,a));
    assert_eq!(baseline::test_power(p,a,1u64<<59), candidate::test_power(p,a,1u64<<59));
    isqrt_equal(baseline::test_discriminant(p,a),candidate::test_discriminant(p,a),p,a);
    checks+=3;
   }
   let n=if p >= (1u64<<50) {80} else {128};
   for _ in 0..n {
    let a=next(&mut s)%p;let b=next(&mut s)%p;
    assert_eq!(baseline::test_inverse(p,a),candidate::test_inverse(p,a));
    assert_eq!(baseline::test_power(p,a,1u64<<59),candidate::test_power(p,a,1u64<<59));
    isqrt_equal(baseline::test_discriminant(p,a),candidate::test_discriminant(p,a),p,a);
    let mut x=baseline::test_quad(p,a,b);let mut y=candidate::test_quad(p,a,b);
    x.sort_unstable();y.sort_unstable();assert_eq!(x,y);
    // Cubic cases exercise AMM/FP2 without assuming identical root ordering.
    let mut x=baseline::test_cubic(p,a,b);let mut y=candidate::test_cubic(p,a,b);
    x.sort_unstable();y.sort_unstable();assert_eq!(x,y);
    checks+=5;
   }
  }
  println!("ROUND114_EQ checks={checks}");
 }
 #[test]
 #[ignore="13 alternating A/B trials, optimized Rust"]
 fn benchmark() {
  let p=(1u64<<61)-1;
  let mut seed=p^0x8745_32ad_0117_63bc;
  for kind in ["sqrt_known","sqrt_uniform","pow_sqrt","inv_special","inv_general","quad_known","quad_uniform","cubic_uniform"] {
   let mut inputs=Vec::new();
   for i in 0..256 {
    let a=if kind=="inv_special" {[1u64,2,p-2,p-1][i%4]} else {next(&mut seed)%p};
    let b=next(&mut seed)%p;
    let c=if kind=="sqrt_known" || kind=="quad_known" {
      ((b as u128*b as u128)%(p as u128)) as u64
    } else {next(&mut seed)%p};
    inputs.push((a,b,c));
   }
   let mut before=Vec::new();let mut after=Vec::new();
   let mut expected_checksum=None;
   for trial in 0..13 {
    let measure = |opt:bool| -> (f64,u64) {
     let now=Instant::now();let mut digest=0x9e37_79b9_7f4a_7c15u64;
     for _ in 0..4 {
      for &(a,b,c) in &inputs {
       let v:u64=match kind {
        "sqrt_known"|"sqrt_uniform"=> {
         let discriminant=if kind=="sqrt_known"{c}else{a};
         let root=if opt{candidate::test_discriminant(p,discriminant)}else{baseline::test_discriminant(p,discriminant)};
         root.map(|r| ((r as u128*r as u128)%(p as u128)) as u64).unwrap_or(p)
        }
        "pow_sqrt"=>if opt{candidate::test_power(p,a,1u64<<59)}else{baseline::test_power(p,a,1u64<<59)},
        "inv_special"|"inv_general"=>if opt{candidate::test_inverse(p,a)}else{baseline::test_inverse(p,a)},
        "quad_known"|"quad_uniform"=> {
          let cc=if kind=="quad_known" {
            let x=((b as u128*b as u128 + (a as u128)*(b as u128))%(p as u128)) as u64;
            if x==0{0}else{p-x}
          }else{c};
          let mut roots=if opt{candidate::test_quad(p,a,cc)}else{baseline::test_quad(p,a,cc)};
          roots.sort_unstable();roots.into_iter().fold(0u64,|h,r|h.wrapping_mul(31).wrapping_add(r))
        }
        "cubic_uniform"=> {
          let mut roots=if opt{candidate::test_cubic(p,a,c)}else{baseline::test_cubic(p,a,c)};
          roots.sort_unstable();roots.into_iter().fold(0u64,|h,r|h.wrapping_mul(31).wrapping_add(r))
        }
        _=>unreachable!(),
       };
       digest=digest.rotate_left(7).wrapping_add(black_box(v));
      }
     }
     (now.elapsed().as_nanos() as f64/(4*inputs.len()) as f64,digest)
    };
    let (b,a)=if trial%2==0{(measure(false),measure(true))}else{
      let a=measure(true);let b=measure(false);(b,a)
    };
    assert_eq!(a.1,b.1,"checksum divergence kind={kind} trial={trial}");
    if let Some(previous)=expected_checksum{assert_eq!(a.1,previous)}else{expected_checksum=Some(a.1);}
    before.push(b.0);after.push(a.0);
   }
   let b=median(before);let a=median(after);
   println!("ROUND114_RUST kind={kind} before_ns={b:.4} after_ns={a:.4} speedup={:.4} checksum={}",b/a,expected_checksum.unwrap());
  }
 }
}
"""
# Both modules expose the same private wrappers and source-exact implementation.
source = head + old + between + new + tail + tests
with tempfile.TemporaryDirectory() as tmp:
    src=Path(tmp)/"round114.rs"
    exe=Path(tmp)/"round114"
    src.write_text(source)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"exact_equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
