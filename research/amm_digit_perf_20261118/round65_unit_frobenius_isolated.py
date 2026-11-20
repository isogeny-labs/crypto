#!/usr/bin/env python3
"""Round 65 source-exact Rust: four Frobenius specializations with A/B timing."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
baseline = source
originals = {
    "PRODUCER_UNIT_BINOMIAL": """            let scaled = if quotient == 0 {
                1
            } else {
                let neg_c = self.sub(0, monic.coefficients[0])?;
                self.split_scalar_pow(neg_c, quotient)?
            };
""",
    "REPLAY_UNIT_BINOMIAL": """            let scaled = if quotient == 0 {
                1
            } else {
                let neg_c = self.minus(0, monic.coefficients[0])?;
                self.split_character_power(neg_c, quotient)?
            };
""",
    "PRODUCER_AFFINE_LINEAR": """        // Over Fp, r^p == r. In Fp[x]/(x-r), x^p is therefore r.
        // The canonical input x is the only base eligible for this path.
        if monic.len == 2 && exponent == self.p
            && base.len == 2 && base.coefficients[0] == 0
            && base.coefficients[1] == 1
        {
            return Ok(Poly17::constant(self.sub(0, monic.coefficients[0])?));
        }
""",
    "REPLAY_AFFINE_LINEAR": """        // Evaluate x^p at the unique root of the linear modulus using
        // Frobenius on Fp; do not call the producer's arithmetic.
        if monic.len == 2 && exponent == self.p
            && base.len == 2 && base.coefficients[0] == 0
            && base.coefficients[1] == 1
        {
            return Ok(Poly17::constant(self.minus(0, monic.coefficients[0])?));
        }
""",
}
for name, old in originals.items():
    pattern = rf"(?ms)^ *// R65_{name}_BEGIN\n.*?^ *// R65_{name}_END\n"
    baseline, count = re.subn(pattern, lambda _m: old, baseline, count=1)
    assert count == 1, f"expected exactly one marked R65 block for {name}"

head = """#![allow(dead_code)]
mod cryptanalysis {
pub mod fp_backend {
 pub struct Fp64Arithmetic {pub p:u64}
 impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
}
pub mod bounded {
pub mod after {
"""
middle = "\n} pub mod before {\n"
tail = "\n}\n}\n}\n"

helper = r"""
pub fn round65_case(p:u64, degree:usize, constant:u64, delta:u64,
                    leading:u64, verify:bool, repeats:usize) -> (u64,u64) {
 use std::{hint::black_box,time::Duration};
 let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,bytes:STORAGE_BYTES,
    events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(120);
 let field=super::super::fp_backend::Fp64Arithmetic{p};
 let mut observer=|_:&BoundedRootEvent|Ok(());
 let mut producer=Producer{p,gate:Gate::new(limits,deadline),observer:&mut observer,
    unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
    roots:[0;8],root_count:0};
 producer.gate.admit().unwrap();
 let mut replay=Replay::new(&field,&[1,1],42,limits,limits,deadline).unwrap();
 let mut modulus=Poly17::ZERO;
 modulus.len=(degree+1) as u8;
 modulus.coefficients[0]=((constant as u128*leading as u128)%p as u128) as u64;
 modulus.coefficients[degree]=leading;
 let mut base=Poly17::ZERO;base.len=2;base.coefficients[0]=delta;base.coefficients[1]=1;
 let mut digest=0u64;
 let mut cost=0u64;
 for iteration in 0..repeats {
    let m=black_box(modulus);let b=black_box(base);
    let result=if verify{replay.modular_power(b,p,&m).unwrap()}
               else{producer.power(&b,p,&m).unwrap()};
    let mut fingerprint=0u64;
    for j in 0..17{fingerprint=fingerprint.wrapping_mul(37)
                                   .wrapping_add(result.coefficients[j]);}
    digest=digest.wrapping_mul(0x9e3779b97f4a7c15)
                 .wrapping_add(black_box(fingerprint).wrapping_add(iteration as u64));
    let gate=if verify{&mut replay.gate}else{&mut producer.gate};
    cost+=gate.metrics.field_work;
    gate.metrics.work=0;gate.metrics.field_work=0;
 }
 (digest,cost)
}

pub fn round65_whole(p:u64,degree:usize,constant:u64,leading:u64) ->
    (Stop,[u64;8],u8,Vec<BoundedRootEvent>) {
 use std::time::Duration;
 let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,bytes:STORAGE_BYTES,
     events:MAX_EVENTS,split_attempts:1000,degree:8};
 let field=super::super::fp_backend::Fp64Arithmetic{p};
 let mut coefficients=[0u64;9];
 coefficients[0]=((constant as u128*leading as u128)%p as u128) as u64;
 coefficients[degree]=leading;
 let coeffs=&coefficients[..degree+1];
 let mut events=Vec::new();
 let report=solve_bounded(&field,coeffs,42,limits,Instant::now()+Duration::from_secs(120),
     &mut |e|{events.push(*e);Ok(())});
 if report.stop==Stop::Complete {
    let mut replay=Replay::new(&field,coeffs,42,limits,limits,
                               Instant::now()+Duration::from_secs(120)).unwrap();
    for e in &events{replay.push(e).unwrap();}
    assert!(replay.finish(&report).unwrap().finite_complete);
 }
 (report.stop,report.roots,report.root_count,events)
}
"""

tests = r"""
#[cfg(test)]
mod round65 {
 use super::cryptanalysis::bounded::{before,after};
 use std::{time::Instant,hint::black_box};
 fn median(mut a:Vec<f64>)->f64{a.sort_by(f64::total_cmp);a[a.len()/2]}
 #[test]
 fn native_four_path_output_and_complete_stream_equivalence() {
   let mut count=0usize;
   for p in [3u64,5,7,101,1_000_000_007,1_000_000_009,(1u64<<61)-1] {
     for degree in [1usize,3,4,8] {
       for leading in [1,p-1] {
         for c in [1,p-1] {
           for delta in [0,1,p-1,23%p] {
             for verify in [false,true] {
               let a=after::round65_case(p,degree,c,delta,leading,verify,1);
               let b=before::round65_case(p,degree,c,delta,leading,verify,1);
               assert_eq!(a.0,b.0,"p={p} d={degree} c={c} delta={delta} lead={leading} replay={verify}");
               assert!(a.1<=b.1,"increased field work");
               count+=1;
             }
           }
         }
       }
     }
   }
   let mut streams=0usize;
   for p in [101u64,1_000_000_007,(1u64<<61)-1] {
     for degree in [4usize,8] {
       for c in [1,p-1] {
         for leading in [1,p-1] {
           let a=after::round65_whole(p,degree,c,leading);
           let b=before::round65_whole(p,degree,c,leading);
           assert_eq!(format!("{a:?}"),format!("{b:?}"),"stream p={p} d={degree} c={c} lead={leading}");
           streams+=1;
         }
       }
     }
   }
   println!("ROUND65_NATIVE_EQ comparisons={count} complete_root_streams={streams}");
 }
 #[test]
 #[ignore="13 alternating compiled Rust A/B trials"]
 fn benchmark_frobenius_kernels() {
   for p in [1_000_000_007u64,(1u64<<61)-1] {
     for degree in [1usize,4,8] {
       for c in [1,p-1] {
         for verify in [false,true] {
           let reps=128usize;
           let mut old=Vec::new();let mut new=Vec::new();
           for trial in 0..13 {
             let bench=|optimized:bool|{
               let start=Instant::now();
               if optimized {black_box(after::round65_case(p,degree,c,7,1,verify,reps));}
               else {black_box(before::round65_case(p,degree,c,7,1,verify,reps));}
               start.elapsed().as_nanos() as f64 / reps as f64
             };
             if trial%2==0 {old.push(bench(false));new.push(bench(true));}
             else {new.push(bench(true));old.push(bench(false));}
           }
           let a=median(old);let b=median(new);
           println!("ROUND65_NATIVE p={p} degree={degree} c={c} replay={verify} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
         }
       }
     }
   }
 }
 #[test]
 #[ignore="paired complete root extraction plus certificate replay"]
 fn benchmark_complete_root_and_replay() {
   for p in [1_000_000_007u64,(1u64<<61)-1] {
     for degree in [4usize,8] {
       for c in [1,p-1] {
         let reps=2usize;
         let mut old=Vec::new();let mut new=Vec::new();
         for trial in 0..13 {
           let bench=|optimized:bool|{
             let start=Instant::now();
             for _ in 0..reps {
               if optimized {black_box(after::round65_whole(p,degree,c,1));}
               else {black_box(before::round65_whole(p,degree,c,1));}
             }
             start.elapsed().as_nanos() as f64 / reps as f64
           };
           if trial%2==0 {old.push(bench(false));new.push(bench(true));}
           else {new.push(bench(true));old.push(bench(false));}
         }
         let a=median(old);let b=median(new);
         println!("ROUND65_WHOLE p={p} degree={degree} c={c} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
       }
     }
   }
 }
}
"""

with tempfile.TemporaryDirectory() as tmp:
    path=Path(tmp)/"round65.rs"
    binary=Path(tmp)/"round65"
    path.write_text(head + source + helper + middle + baseline + helper + tail + tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    for name in ["native_four_path_output_and_complete_stream_equivalence",
                 "benchmark_frobenius_kernels","benchmark_complete_root_and_replay"]:
        command=[str(binary),name,"--nocapture"]
        if name.startswith("benchmark"):command.append("--ignored")
        subprocess.run(command,check=True)
