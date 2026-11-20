#!/usr/bin/env python3
"""Round 66: exact production Rust A/B for four unit-linear polynomial paths."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
baseline = source
paths = [
    "PRODUCER_UNIT_LINEAR_DIVISION",
    "PRODUCER_UNIT_LINEAR_REDUCE",
    "REPLAY_UNIT_LINEAR_REMAINDER",
    "REPLAY_UNIT_LINEAR_CANCEL",
]
for name in paths:
    pattern = rf"(?ms)^ *// R66_{name}_BEGIN\n.*?^ *// R66_{name}_END\n"
    baseline, count = re.subn(pattern, "", baseline, count=1)
    assert count == 1, f"expected one marked R66 block for {name}"

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
pub fn round66_case(p:u64,c:u64,n:usize,sparse:bool,op:usize,repeats:usize,collect:bool)
    -> (Vec<([u64;17],u8,[u64;17],u8)>,u64,u64,u64) {
 use std::time::Duration;
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
 let mut divisor=Poly17::ZERO;divisor.len=2;divisor.coefficients[0]=c;
 divisor.coefficients[1]=1;
 let mut outputs=Vec::new();
 if collect { outputs.reserve(repeats); }
 let mut digest=0u64;
 for iteration in 0..repeats {
     let mut value=Poly17::ZERO;value.len=n as u8;
     let mut seed=(iteration as u64).wrapping_add(0x6a09e667f3bcc909 ^ p);
     for i in 0..n {
         seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
         value.coefficients[i]=if sparse && (i+iteration)%3==0 {0} else {seed%p};
     }
     value.coefficients[n-1]=1+(seed%(p-1));
     let (quotient,residue)=match op {
         0=>producer.division(&value,&divisor).unwrap(),
         1=>{
             let mut r=value;
             producer.reduce_monic(&mut r,&divisor).unwrap();
             (Poly17::ZERO,r)
         },
         2=>(Poly17::ZERO,replay.remainder(value,&divisor).unwrap()),
         3=>{
             let mut r=value;
             replay.cancel_monic(&mut r,&divisor).unwrap();
             (Poly17::ZERO,r)
         },
         _=>panic!("operation out of range"),
     };
     digest=digest.wrapping_add(residue.coefficients[0])
         .wrapping_add(quotient.coefficients[0])
         .wrapping_add(u64::from(residue.len))
         .wrapping_add(u64::from(quotient.len));
     if collect {
         outputs.push((quotient.coefficients,quotient.len,residue.coefficients,residue.len));
     }
 }
 let m=if op<2 { producer.gate.metrics } else { replay.gate.metrics };
 (outputs,digest,m.work,m.field_work)
}

pub fn round66_whole(p:u64, polynomial:&[u64]) -> String {
 use std::time::Duration;
 let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,bytes:STORAGE_BYTES,
     events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(120);
 let f=super::super::fp_backend::Fp64Arithmetic{p};
 let mut stream=Vec::new();
 let report=solve_bounded(&f,polynomial,29,limits,deadline,
     &mut |event| {stream.push(*event);Ok(())});
 let mut replay=Replay::new(&f,polynomial,29,limits,limits,deadline).unwrap();
 for event in &stream { replay.push(event).unwrap(); }
 let checked=replay.finish(&report).unwrap();
 format!("{:?} {:?} {:?}",report,stream,checked)
}
"""
tests = r"""
#[cfg(test)]
mod rounds {
 use crate::cryptanalysis::bounded::{before,after};
 use std::{hint::black_box,time::Instant};
 fn median(mut values:Vec<f64>)->f64 {
     values.sort_by(|a,b|a.total_cmp(b));values[values.len()/2]
 }
 #[test]
 fn native_exact_kernel_and_stream_equivalence() {
     let mut cases=0usize;
     for p in [101u64,1_000_000_007,(1u64<<61)-1] {
         for c in [1,p-1] {
             for n in [2usize,5,9,17] {
                 for sparse in [false,true] {
                     for op in 0..4 {
                         let a=after::round66_case(p,c,n,sparse,op,32,true);
                         let b=before::round66_case(p,c,n,sparse,op,32,true);
                         assert_eq!(a,b,"p={p} c={c} n={n} sparse={sparse} op={op}");
                         cases+=32;
                     }
                 }
             }
         }
     }
     let mut streams=0usize;
     for p in [101u64,1_000_000_007,(1u64<<61)-1] {
         for polynomial in [vec![p-1,0,1],vec![1,1],vec![p-1,1],vec![1,0,0,1]] {
             let a=after::round66_whole(p,&polynomial);
             let b=before::round66_whole(p,&polynomial);
             assert_eq!(a,b,"p={p} polynomial={polynomial:?}");
             streams+=1;
         }
     }
     println!("ROUND66_EQ kernel_cases={cases} complete_root_streams={streams}");
 }
 #[test]
 #[ignore="timed source-exact Rust A/B"]
 fn benchmark_unit_linear_kernels() {
     for p in [1_000_000_007u64,(1u64<<61)-1] {
         for c in [1,p-1] {
             for sparse in [false,true] {
                 for op in 0..4 {
                     let reps=256usize; let n=17usize;
                     let mut old=Vec::new();let mut new=Vec::new();
                     for trial in 0..13 {
                         let bench=|optimized:bool|{
                             let start=Instant::now();
                             let result=if optimized {
                                 after::round66_case(p,c,n,sparse,op,reps,false)
                             } else {
                                 before::round66_case(p,c,n,sparse,op,reps,false)
                             };
                             black_box(result);
                             start.elapsed().as_nanos() as f64/reps as f64
                         };
                         if trial%2==0 {old.push(bench(false));new.push(bench(true));}
                         else {new.push(bench(true));old.push(bench(false));}
                     }
                     let a=median(old);let b=median(new);
                     println!("ROUND66_KERNEL p={p} c={c} sparse={sparse} op={op} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
                 }
             }
         }
     }
 }
 #[test]
 #[ignore="timed complete root extraction and replay"]
 fn benchmark_complete_root_and_replay() {
     for p in [1_000_000_007u64,(1u64<<61)-1] {
         for polynomial in [vec![p-1,0,1],vec![1,1],vec![p-1,1],vec![1,0,0,1]] {
             let reps=64usize;let mut old=Vec::new();let mut new=Vec::new();
             for trial in 0..13 {
                 let bench=|optimized:bool|{
                     let start=Instant::now();
                     for _ in 0..reps {
                         if optimized {black_box(after::round66_whole(p,&polynomial));}
                         else {black_box(before::round66_whole(p,&polynomial));}
                     }
                     start.elapsed().as_nanos() as f64/reps as f64
                 };
                 if trial%2==0 {old.push(bench(false));new.push(bench(true));}
                 else {new.push(bench(true));old.push(bench(false));}
             }
             let a=median(old);let b=median(new);
             println!("ROUND66_WHOLE p={p} polynomial={polynomial:?} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
         }
     }
 }
}
"""

with tempfile.TemporaryDirectory() as tmp:
    path=Path(tmp)/"round66.rs"
    binary=Path(tmp)/"round66"
    path.write_text(head + source + helper + middle + baseline + helper + tail + tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    for name in ["native_exact_kernel_and_stream_equivalence",
                 "benchmark_unit_linear_kernels","benchmark_complete_root_and_replay"]:
        command=[str(binary),name,"--nocapture"]
        if name.startswith("benchmark"):command.append("--ignored")
        subprocess.run(command,check=True)
