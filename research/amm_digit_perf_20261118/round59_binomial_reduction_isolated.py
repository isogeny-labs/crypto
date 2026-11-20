#!/usr/bin/env python3
"""Round 59: source-exact before/after binomial reduction/division and replay."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
baseline = source
for block in ("BINOMIAL_DIVISION", "BINOMIAL_REDUCTION", "BINOMIAL_REMAINDER", "BINOMIAL_CANCEL"):
    baseline, count = re.subn(
        rf"(?m)^ *// R59_{block}_BEGIN\n.*?^ *// R59_{block}_END\n",
        "", baseline, count=1, flags=re.S,
    )
    assert count == 1, "missing R59 baseline marker " + block

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
pub fn round58_case(p:u64, degree:usize, constant:u64, delta:u64,
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
 for iteration in 0..repeats{
    let m=black_box(modulus);let b=black_box(base);
    let result=if verify{replay.modular_power(b,p,&m).unwrap()}
               else{producer.power(&b,p,&m).unwrap()};
    let mut fingerprint=0u64;
    for j in 0..17{fingerprint=fingerprint.wrapping_mul(37)
                                   .wrapping_add(result.coefficients[j]);}
    digest=digest.wrapping_add(black_box(fingerprint).wrapping_add(iteration as u64));
    let gate=if verify{&mut replay.gate}else{&mut producer.gate};
    gate.metrics.work=0;gate.metrics.field_work=0;
 }
 let metric=if verify{replay.gate.metrics.field_work}else{producer.gate.metrics.field_work};
 (digest,metric)
}
pub fn round58_whole(p:u64,degree:usize,constant:u64,leading:u64) -> (Stop,[u64;8],u8,Vec<BoundedRootEvent>) {
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
 
helper += r"""
pub fn round59_case(p:u64, degree:usize, constant:u64, mode:u8,
                    seed:u64, repeats:usize) -> (u64,u64) {
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
 modulus.coefficients[0]=constant;
 modulus.coefficients[degree]=1;
 let mut digest=0u64;
 let mut charged=0u64;
 for iteration in 0..repeats {
     let mut input=Poly17::ZERO;
     let n=(2*degree+1).min(17);
     input.len=n as u8;
     let mut r=black_box(seed.wrapping_add(iteration as u64))
         .wrapping_mul(0x9E3779B97F4A7C15);
     for j in 0..n {
         r=r.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
         input.coefficients[j]=r%p;
     }
     input.coefficients[n-1]=1;
     let m=black_box(modulus);
     let input=black_box(input);
     let (a,b)=match mode {
       0=>{let (q,r)=producer.division(&input,&m).unwrap();(q,r)},
       1=>{let mut r=input;producer.reduce_monic(&mut r,&m).unwrap();(Poly17::ZERO,r)},
       2=>{let r=replay.remainder(input,&m).unwrap();(Poly17::ZERO,r)},
       3=>{let mut r=input;replay.cancel_monic(&mut r,&m).unwrap();(Poly17::ZERO,r)},
       _=>panic!("unsupported mode")
     };
     let mut sample=(a.len as u64).wrapping_add((b.len as u64)<<8);
     for j in 0..17 {
         sample=sample.wrapping_mul(37).wrapping_add(a.coefficients[j]);
         sample=sample.wrapping_mul(37).wrapping_add(b.coefficients[j]);
     }
     digest=digest.wrapping_mul(0x9E3779B9).wrapping_add(black_box(sample));
     let gate=if mode<2{&mut producer.gate}else{&mut replay.gate};
     charged+=gate.metrics.field_work;
     gate.metrics.field_work=0;gate.metrics.work=0;
 }
 (digest,charged)
}
"""

tests = r"""
#[cfg(test)]
mod round58 {
 use super::cryptanalysis::bounded::{before,after};
 use std::{time::Instant,hint::black_box};
 fn median(mut a:Vec<f64>)->f64{a.sort_by(f64::total_cmp);a[a.len()/2]}
 #[test]
 fn binomial_frobenius_and_certificate_streams() {
   let mut count=0usize;
   for p in [3u64,5,7,101,1_000_000_007,1_000_000_009,(1u64<<61)-1]{
     for degree in 3..=8{
       for leading in [1,p-1]{
         for constant in [1,2%p,p-1,17%p]{
           for delta in [0,1,p-1,23%p]{
             for verify in [false,true]{
               let b=before::round58_case(p,degree,constant,delta,leading,verify,1);
               let a=after::round58_case(p,degree,constant,delta,leading,verify,1);
               assert_eq!(b.0,a.0,"p={p} d={degree} c={constant} delta={delta} verify={verify}");
               count+=1;
             }
           }
         }
       }
     }
   }
   for p in [101u64,1_000_000_007,(1u64<<61)-1] {
     for degree in [4,8] {
       for c in [1,17] {
         for leading in [1,p-1] {
           let before=before::round58_whole(p,degree,c,leading);
           let after=after::round58_whole(p,degree,c,leading);
           assert_eq!(format!("{before:?}"),format!("{after:?}"),"complete stream p={p} d={degree} c={c} leading={leading}");
         }
       }
     }
   }
   println!("ROUND58_NATIVE_EQ comparisons={count} plus 24 full root/certificate streams");
 }
 #[test]
 #[ignore="13 alternating production Rust before/after trials"]
 fn benchmark_binomial_frobenius() {
   for p in [1_000_000_007u64,(1u64<<61)-1] {
     for degree in [4usize,8] {
       for verify in [false,true] {
         let reps=96usize;
         let mut old=Vec::new();let mut new=Vec::new();
         for trial in 0..13 {
           let bench=|optimized:bool|{
             let start=Instant::now();
             if optimized {
               black_box(after::round58_case(p,degree,17,7,1,verify,reps));
             } else {
               black_box(before::round58_case(p,degree,17,7,1,verify,reps));
             }
             start.elapsed().as_nanos() as f64 / reps as f64
           };
           if trial%2==0 {old.push(bench(false));new.push(bench(true));}
           else {new.push(bench(true));old.push(bench(false));}
         }
         let a=median(old);let b=median(new);
         println!("ROUND58_NATIVE p={p} degree={degree} verify={verify} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
       }
     }
   }
 }
 #[test]
 #[ignore="13 paired source-exact complete root plus replay timings"]
 fn benchmark_complete_binomial_roots() {
   for p in [1_000_000_007u64,(1u64<<61)-1] {
     for degree in [4usize,8] {
       for leading in [1u64,p-1] {
         let mut old=Vec::new(); let mut new=Vec::new();
         let reps=8usize;
         for trial in 0..13 {
           let bench=|optimized:bool| {
             let start=Instant::now();
             for _ in 0..reps {
               if optimized {
                 black_box(after::round58_whole(p,degree,17,leading));
               } else {
                 black_box(before::round58_whole(p,degree,17,leading));
               }
             }
             start.elapsed().as_nanos() as f64 / reps as f64
           };
           if trial%2==0 { old.push(bench(false));new.push(bench(true)); }
           else { new.push(bench(true));old.push(bench(false)); }
         }
         let a=median(old);let b=median(new);
         println!("ROUND58_WHOLE p={p} degree={degree} leading={leading} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
       }
     }
   }
 }

}
"""

tests += r"""
#[cfg(test)]
mod round59 {
 use super::cryptanalysis::bounded::{before,after};
 use std::{time::Instant,hint::black_box};
 fn median(mut a:Vec<f64>)->f64{a.sort_by(f64::total_cmp);a[a.len()/2]}
 #[test]
 fn binomial_reduction_and_certificates() {
   let mut count=0usize;
   for p in [3u64,5,7,101,1_000_000_007,1_000_000_009,(1u64<<61)-1]{
     for degree in 2..=8 {
       for c0 in [1,2,17,p-1] {
         let c=c0%p;if c==0 {continue}
         for seed in 0..32u64 {
           for mode in 0..4u8 {
             let b=before::round59_case(p,degree,c,mode,seed,1);
             let a=after::round59_case(p,degree,c,mode,seed,1);
             assert_eq!(b.0,a.0,"p={p} degree={degree} c={c} mode={mode} seed={seed}");
             assert!(a.1<=b.1,"expected no higher field cost");
             count+=1;
           }
         }
       }
     }
   }
   for p in [101u64,1_000_000_007,(1u64<<61)-1]{
     for degree in [4usize,8]{
       for c in [1u64,17] {
         for leading in [1u64,p-1] {
           let before=before::round58_whole(p,degree,c,leading);
           let after=after::round58_whole(p,degree,c,leading);
           assert_eq!(format!("{before:?}"),format!("{after:?}"),
               "complete event/root/certificate stream p={p} degree={degree} c={c}");
         }
       }
     }
   }
   println!("ROUND59_EQ comparisons={count} plus 24 complete root/certificate streams");
 }
 #[test]
 #[ignore="13 alternating source-exact Rust kernel A/B trials"]
 fn benchmark_binomial_reduction() {
   for p in [1_000_000_007u64,(1u64<<61)-1] {
     for degree in [4usize,8] {
       for c in [1u64,p-1,17]{
         for mode in 0..4u8 {
           let repeats=512usize;
           let mut old=Vec::new();let mut new=Vec::new();
           for trial in 0..13 {
             let bench=|optimized:bool|{
               let now=Instant::now();
               if optimized {
                 black_box(after::round59_case(p,degree,c,mode,12345,repeats));
               }else {
                 black_box(before::round59_case(p,degree,c,mode,12345,repeats));
               }
               now.elapsed().as_nanos() as f64 / repeats as f64
             };
             if trial%2==0{old.push(bench(false));new.push(bench(true));}
             else{new.push(bench(true));old.push(bench(false));}
           }
           let a=median(old);let b=median(new);
           println!("ROUND59_NATIVE p={p} degree={degree} c={c} mode={mode} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
         }
       }
     }
   }
 }
}
"""

with tempfile.TemporaryDirectory() as tmp:
    path = Path(tmp) / "round59.rs"
    binary = Path(tmp) / "round59"
    path.write_text(head + source + helper + middle + baseline + helper + tail + tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"binomial_reduction_and_certificates","--nocapture"],check=True)
    subprocess.run([str(binary),"benchmark_binomial_reduction","--ignored","--nocapture"],check=True)
    
