#!/usr/bin/env python3
"""Round 116: source-exact Rust A/B for root coefficient normalization.

Reconstructs the frozen pre-round116 implementation from the current source,
then compiles both implementations in one test binary. Both call the exact
same field/radical logic. No Cargo, network, or unrelated repo dependencies.
"""
from pathlib import Path
import os
import subprocess
import tempfile

try:
    allowed = os.sched_getaffinity(0)
    if allowed:
        cpu = min(allowed)
        os.sched_setaffinity(0, {cpu})
        print(f"ROUND116_CPU_PIN cpu={cpu}", flush=True)
except (AttributeError, OSError):
    print("ROUND116_CPU_PIN unavailable", flush=True)

source = Path("src/cryptanalysis/prime_radical_root.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
tag = "    pub fn roots(&self, f: &[u64]) -> Result<RadicalRoots, String>"
start = source.index("        let mut n = f.len();", source.index(tag))
end = source.index("        let ok = |roots:", start)
optimized = source[start:end]
assert "let mut leading = 0u64;" in optimized
assert "n.saturating_sub(1)" in optimized
frozen = """        let mut n = f.len();
        while n > 0 && f[n - 1] % fp.p == 0 {
            n -= 1;
        }
        if n > 5 {
            return Err("radical root finder supports degree <= 4".into());
        }
        let mut c = [0u64; 5];
        for j in 0..n {
            c[j] = f[j] % fp.p;
        }
"""
baseline = source[:start] + frozen + source[end:]
# The second Round116 optimization specializes monic linear polynomials.
# Reconstruct the frozen pre-Round116 root entry point too.
fast_start = baseline.index("        // Monic linear inputs occur repeatedly")
fast_end = baseline.index("        // Polynomials of degree", fast_start)
baseline = baseline[:fast_start] + baseline[fast_end:]
assert baseline != source and baseline.count("let mut leading = 0u64;") == 0
assert "Monic linear inputs occur repeatedly" not in baseline

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
 -> Result<RootReport,String> { panic!("unplanned Frobenius fallback"); }
}
pub mod residual_walk {
 pub fn is_prime_u64(p:u64)->bool {
   matches!(p,5|7|11|31|101|257|65_521|1_000_000_007|2_305_843_009_213_693_951)
 }
}
pub mod baseline {
'''
expose = r'''
pub fn ctx(p:u64)->RadicalRootCtx { RadicalRootCtx::new(p).unwrap() }
pub fn roots(ctx:&RadicalRootCtx, input:&[u64])->Result<(Vec<u64>,bool),String>{
 ctx.roots(input).map(|r|(r.roots,r.frobenius_fallback))
}
}
pub mod candidate {
'''
footer = expose.replace("}\npub mod candidate {", "}\n}\n")
tests = r'''
#[cfg(test)]
mod tests {
 use super::cryptanalysis::{baseline,candidate};
 use std::{hint::black_box,time::Instant};
 fn next(s:&mut u64)->u64 {
  *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *s
 }
 fn med(mut a:Vec<f64>)->f64 {a.sort_by(|x,y|x.total_cmp(y));a[a.len()/2]}
 fn hash(xs: &[u64])->u64 {
   xs.iter().fold(0x92d0_8dbe_12a7_a770u64,|h,&x|h.rotate_left(7).wrapping_mul(0x9e3779b185ebca87)^x)
 }
 #[test]
 fn equivalence() {
  let mut seed=0xa99e_89db_62ce_5153u64;
  let mut total=0usize;
  for p in [5u64,7,101,65_521,1_000_000_007,(1u64<<61)-1] {
   let b=baseline::ctx(p);let c=candidate::ctx(p);
   let cases=if p<1024{5000}else if p==65_521{2048}else if p==1_000_000_007{1024}else{512};
   for k in 0..cases {
    let len=(next(&mut seed)%9)as usize;
    let mut v=Vec::new();
    for j in 0..len {
     let x=next(&mut seed)%p;
     v.push(match (k+j)%8 {
      0=>0,1=>p,2=>2*p,3=>3*p,4=>x,5=>p+x,6=>next(&mut seed),_=>2*p+x,
     });
    }
    // The independent quartic Frobenius solver is deliberately stubbed
    // in this isolated harness. Keep valid normalized degrees <= 3;
    // n > 5 still exercises the exact error path.
    if len>=5 { v[4]=p; }
    let old=baseline::roots(&b,&v);
    let new=candidate::roots(&c,&v);
    assert_eq!(old,new,"p={p} k={k} polynomial={v:?}");
    total+=1;
   }
   // Independently check canonical and noncanonical coefficient inputs
   // for every supported linear and quadratic degree against both versions.
   for _ in 0..512 {
    let a=next(&mut seed)%p;let x=next(&mut seed)%p;let v=[a,x,1];
    assert_eq!(baseline::roots(&b,&v),candidate::roots(&c,&v));
    let n=[p+a,p+x,p+1,p,0];
    assert_eq!(baseline::roots(&b,&n),candidate::roots(&c,&n));
    total+=2;
   }
   println!("ROUND116_EQ p={p} comparisons={total}");
  }
  println!("ROUND116_EQ_ALL comparisons={total}");
 }
 #[test]
 #[ignore="Native source-exact 13 trial A/B measurements"]
 fn benchmark() {
  let mut seed=0xe481_f9bb_056a_8ca7u64;
  for p in [65_521u64,1_000_000_007,(1u64<<61)-1] {
   let b=baseline::ctx(p);let c=candidate::ctx(p);
   for kind in ["linear","quadratic","cubic","padded_cubic","noncanonical_quadratic"] {
    let mut inputs=Vec::new();
    for _ in 0..128 {
     let a=next(&mut seed)%p;
     let x=next(&mut seed)%p;
     let y=next(&mut seed)%p;
     let v=match kind {
      "linear"=>vec![a,1],
      "quadratic"=>vec![a,x,1],
      "cubic"=>vec![a,x,y,1],
      "padded_cubic"=>vec![a,x,y,1,p,0,2*p,p],
      "noncanonical_quadratic"=>vec![2*p+a,p+x,p+1],
      _=>unreachable!(),
     };
     assert_eq!(baseline::roots(&b,&v),candidate::roots(&c,&v));
     inputs.push(v);
    }
    let loops=match kind {
     "linear"=>1024,
     "quadratic" | "noncanonical_quadratic"=>if p>u32::MAX as u64 {48} else {128},
     "cubic" | "padded_cubic"=>if p>u32::MAX as u64 {8} else {24},
     _=>unreachable!(),
    };
    let mut before=Vec::new();let mut after=Vec::new();
    let mut expected=None;
    for trial in 0..13 {
     let measure=|new:bool|->(f64,u64) {
       let start=Instant::now();let mut digest=0x4af5_9851_923a_d072u64;
       for _ in 0..loops {for input in &inputs {
         let r=if new{candidate::roots(&c,black_box(input))}else{baseline::roots(&b,black_box(input))}.unwrap();
         digest=digest.wrapping_mul(0x9e37_79b1_85eb_ca87).wrapping_add(black_box(hash(&r.0)));
       }}
       (start.elapsed().as_nanos()as f64/(loops*inputs.len())as f64,digest)
     };
     let (bv,av)=if trial%2==0 { (measure(false),measure(true)) }
       else {let a=measure(true);let b=measure(false);(b,a)};
     assert_eq!(bv.1,av.1,"p={p},kind={kind},trial={trial}");
     if let Some(prev)=expected {assert_eq!(prev,av.1);}else{expected=Some(av.1);}
     before.push(bv.0);after.push(av.0);
    }
    let bmed=med(before);let amed=med(after);
    println!("ROUND116_RUST p={p} kind={kind} before_ns={bmed:.3} after_ns={amed:.3} speedup={:.4} checksum={}",bmed/amed,expected.unwrap());
   }
  }
 }
}
'''
assembled = header + baseline + expose + source + footer + tests
with tempfile.TemporaryDirectory() as tmp:
    root=Path(tmp)
    rs=root/"round116.rs"
    exe=root/"round116"
    rs.write_text(assembled)
    subprocess.run(["rustc","--edition=2021","-C","opt-level=3","--test",str(rs),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"equivalence","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark","--ignored","--nocapture"],check=True)
