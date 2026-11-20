#!/usr/bin/env python3
"""Round 57: native source-exact Rust A/B for monomial Frobenius and complete replay."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source
for name in ["MONOMIAL_FROBENIUS_PRODUCER", "MONOMIAL_FROBENIUS_REPLAY"]:
    before, count = re.subn(
        rf"(?m)^ *// R57_{name}_BEGIN\n.*?^ *// R57_{name}_END\n",
        "", before, count=1, flags=re.S,
    )
    assert count == 1, f"Missing exact baseline marker: {name}"

header = """#![allow(dead_code)]
mod cryptanalysis {
pub mod fp_backend {
 pub struct Fp64Arithmetic {pub p:u64}
 impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
}
pub mod bounded {
pub mod after {
"""
middle = "\n} pub mod before {\n"
footer = "\n}\n}\n}\n"

helper = r"""
pub fn round57_power_case(p:u64, degree:usize, delta:u64, leading:u64,
                          verify:bool, repeats:usize) -> (u64,u64,u64) {
 use std::{time::Duration,hint::black_box};
 let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
     bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(120);
 let field=super::super::fp_backend::Fp64Arithmetic{p};
 let mut observer=|_:&BoundedRootEvent|Ok(());
 let mut producer=Producer{p,gate:Gate::new(limits,deadline),observer:&mut observer,
     unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
     roots:[0;8],root_count:0};
 producer.gate.admit().unwrap();
 let mut replay=Replay::new(&field,&[1,1],42,limits,limits,deadline).unwrap();
 let mut modulus=Poly17::ZERO;
 modulus.len=(degree+1) as u8;modulus.coefficients[degree]=leading;
 let mut base=Poly17::ZERO;base.len=2;base.coefficients[0]=delta;base.coefficients[1]=1;
 let mut digest=0u64;
 for iteration in 0..repeats {
    let m=black_box(modulus);
    let b=black_box(base);
    let result=if verify {
       replay.modular_power(b,p,&m).unwrap()
    } else {
       producer.power(&b,p,&m).unwrap()
    };
    let mut fingerprint=0u64;
    for j in 0..17 {fingerprint=fingerprint.wrapping_mul(37).wrapping_add(result.coefficients[j]);}
    digest=digest.wrapping_add(black_box(fingerprint).wrapping_add(iteration as u64));
 }
 let metrics=if verify{replay.gate.metrics}else{producer.gate.metrics};
 (digest,metrics.work,metrics.field_work)
}
pub fn round57_whole(p:u64,degree:usize,leading:u64,repeats:usize)
    -> (u64,usize,usize) {
 use std::{time::Duration,hint::black_box};
 let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
     bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8};
 let f=super::super::fp_backend::Fp64Arithmetic{p};
 let mut coefficients=[0u64;9];coefficients[degree]=leading;
 let mut digest=0u64;let mut total_events=0usize;
 for i in 0..repeats {
   let mut events=Vec::new();
   let report=solve_bounded(&f,black_box(&coefficients[..degree+1]),42,limits,
       Instant::now()+Duration::from_secs(120),&mut |e|{events.push(*e);Ok(())});
   assert_eq!(report.stop,Stop::Complete);
   assert_eq!(report.root_count,1);
   assert_eq!(report.roots[0],0);
   let mut replay=Replay::new(&f,&coefficients[..degree+1],42,limits,limits,
       Instant::now()+Duration::from_secs(120)).unwrap();
   for event in &events {replay.push(event).unwrap();}
   let result=replay.finish(&report).unwrap();
   assert!(result.finite_complete);
   total_events+=events.len();
   digest=digest.wrapping_add(black_box(report.roots[0] ^ report.root_count as u64 ^ i as u64));
 }
 (digest,total_events,repeats)
}
pub fn round57_oracle(p:u64,degree:usize,delta:u64,leading:u64)->u64 {
 use std::time::Duration;
 let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
     bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(120);
 let field=super::super::fp_backend::Fp64Arithmetic{p};
 let mut observer=|_:&BoundedRootEvent|Ok(());
 let mut producer=Producer{p,gate:Gate::new(limits,deadline),observer:&mut observer,
     unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
     roots:[0;8],root_count:0};
 producer.gate.admit().unwrap();
 let mut replay=Replay::new(&field,&[1,1],42,limits,limits,deadline).unwrap();
 let mut modulus=Poly17::ZERO;modulus.len=(degree+1) as u8;modulus.coefficients[degree]=leading;
 let mut base=Poly17::ZERO;base.len=2;base.coefficients[0]=delta;base.coefficients[1]=1;
 let mut expected=Poly17::constant(delta);
 if p<degree as u64{
    expected.coefficients[p as usize]=1;
    expected.len=(p+1) as u8;
 }
 let a=producer.power(&base,p,&modulus).unwrap();
 let b=replay.modular_power(base,p,&modulus).unwrap();
 assert_eq!(a,expected,"producer p={p} d={degree} delta={delta} lead={leading}");
 assert_eq!(b,expected,"replay p={p} d={degree} delta={delta} lead={leading}");
 (a.len as u64).wrapping_add(a.coefficients[0]).wrapping_add(b.len as u64)
}
"""
tests = r"""
#[cfg(test)]
mod round57 {
 use super::cryptanalysis::bounded::{after,before};
 use std::{time::Instant,hint::black_box};
 fn median(mut data:Vec<f64>)->f64{data.sort_by(f64::total_cmp);data[data.len()/2]}
 #[test]
 fn frobenius_exact_oracle_and_complete_certificate_replay() {
   let mut checks=0;
   for p in [3u64,5,7,1_000_000_007,1_000_000_009,(1u64<<61)-1] {
     for degree in 2..=8usize{
       for leading in [1,p-1] {
         for delta in [0,1,p-1,17%p] {
           assert_eq!(before::round57_oracle(p,degree,delta,leading),
                      after::round57_oracle(p,degree,delta,leading));
           let old=before::round57_power_case(p,degree,delta,leading,false,1);
           let new=after::round57_power_case(p,degree,delta,leading,false,1);
           assert_eq!(old.0,new.0);
           let old=before::round57_power_case(p,degree,delta,leading,true,1);
           let new=after::round57_power_case(p,degree,delta,leading,true,1);
           assert_eq!(old.0,new.0);
           checks+=3;
         }
         if p==3 || p==7 || p==1_000_000_007 || p==(1u64<<61)-1 {
            assert_eq!(before::round57_whole(p,degree,leading,1),
                       after::round57_whole(p,degree,leading,1));
         }
       }
     }
   }
   println!("ROUND57_NATIVE_EQ comparisons={checks}; plus 56 complete root/certificate streams");
 }
 #[test]
 #[ignore="13 alternating source-exact native Rust before/after timings"]
 fn benchmark_frobenius_and_whole_root() {
   for p in [1_000_000_007u64,(1u64<<61)-1]{
     for degree in [4usize,8]{
       for kind in 0..4{
         let reps=if kind<2{64}else{16};
         let mut old=Vec::new();let mut new=Vec::new();
         for trial in 0..13{
           let bench=|optimized:bool| {
             let start=Instant::now();
             if optimized{
               if kind==0{black_box(after::round57_power_case(p,degree,0,1,false,reps));}
               if kind==1{black_box(after::round57_power_case(p,degree,3,1,true,reps));}
               if kind==2{black_box(after::round57_whole(p,degree,1,reps));}
               if kind==3{black_box(after::round57_whole(p,degree,p-1,reps));}
             }else{
               if kind==0{black_box(before::round57_power_case(p,degree,0,1,false,reps));}
               if kind==1{black_box(before::round57_power_case(p,degree,3,1,true,reps));}
               if kind==2{black_box(before::round57_whole(p,degree,1,reps));}
               if kind==3{black_box(before::round57_whole(p,degree,p-1,reps));}
             }
             start.elapsed().as_nanos() as f64 / reps as f64
           };
           if trial%2==0 {old.push(bench(false));new.push(bench(true));}
           else {new.push(bench(true));old.push(bench(false));}
         }
         let a=median(old);let b=median(new);
         println!("ROUND57_NATIVE p={p} degree={degree} kind={kind} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
       }
     }
   }
 }
}
"""
with tempfile.TemporaryDirectory() as tmp:
    path = Path(tmp) / "round57.rs"
    binary = Path(tmp) / "round57"
    path.write_text(header + source + helper + middle + before + helper + footer + tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"frobenius_exact_oracle_and_complete_certificate_replay","--nocapture"],check=True)
    subprocess.run([str(binary),"benchmark_frobenius_and_whole_root","--ignored","--nocapture"],check=True)
