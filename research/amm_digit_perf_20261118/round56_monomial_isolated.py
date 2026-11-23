#!/usr/bin/env python3
"""Round 56: compile source-exact production Rust before/after x^d kernels."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source
for name in [
    "PRODUCER_MONOMIAL_DIVISION", "PRODUCER_MONOMIAL_REDUCTION",
    "REPLAY_MONOMIAL_REMAINDER", "REPLAY_MONOMIAL_CANCEL",
]:
    before, count = re.subn(
        rf"(?m)^ *// R56_{name}_BEGIN\n.*?^ *// R56_{name}_END\n",
        "", before, count=1, flags=re.S,
    )
    assert count == 1, f"R56_{name}: baseline removal failed"

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
pub fn round56_kernel(p:u64,degree:usize,kind:u8,repeats:usize)->(u64,u64,u64) {
 use std::{hint::black_box,time::Duration};
 let budget=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
     bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(120);
 let mut observer=|_:&BoundedRootEvent|Ok(());
 let mut producer=Producer{p,gate:Gate::new(budget,deadline),observer:&mut observer,
     unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
     roots:[0;8],root_count:0};
 producer.gate.admit().unwrap();
 let f=super::super::fp_backend::Fp64Arithmetic{p};
 let mut replay=Replay::new(&f,&[1,1],42,budget,budget,deadline).unwrap();
 let mut modulus=Poly17::ZERO;modulus.len=(degree+1) as u8;modulus.coefficients[degree]=1;
 let mut input=Poly17::ZERO;input.len=16;
 for i in 0..16 {
   input.coefficients[i]=((i as u64+7)*97+31)%p;
   if i<15 && i%5==3 {input.coefficients[i]=0;}
 }
 let mut digest=0u64;
 for i in 0..repeats {
   let x=black_box(input);
   let result=match kind {
     0 => {let (q,r)=producer.division(&x,&modulus).unwrap();
           black_box(q);r},
     1 => {let mut y=x;producer.reduce_monic(&mut y,&modulus).unwrap();y},
     2 => replay.remainder(x,&modulus).unwrap(),
     3 => {let mut y=x;replay.cancel_monic(&mut y,&modulus).unwrap();y},
     _ => panic!("unknown kernel"),
   };
   digest=digest.wrapping_add(black_box(result.coefficients[i%degree]));
 }
 let metrics=if kind>=2 {replay.gate.metrics} else {producer.gate.metrics};
 (digest,metrics.work,metrics.field_work)
}
pub fn round56_oracle(p:u64,degree:usize)->u64{
 use std::time::Duration;
 let budget=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
    bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8};
 let deadline=Instant::now()+Duration::from_secs(120);
 let mut observer=|_:&BoundedRootEvent|Ok(());
 let mut producer=Producer{p,gate:Gate::new(budget,deadline),observer:&mut observer,
    unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
    roots:[0;8],root_count:0};
 producer.gate.admit().unwrap();
 let f=super::super::fp_backend::Fp64Arithmetic{p};
 let mut replay=Replay::new(&f,&[1,1],42,budget,budget,deadline).unwrap();
 let mut modulus=Poly17::ZERO;modulus.len=(degree+1) as u8;modulus.coefficients[degree]=1;
 let mut input=Poly17::ZERO;input.len=16;
 for i in 0..16 {
   input.coefficients[i]=((i as u64+7)*97+31)%p;
   if i<15 && i%5==3 {input.coefficients[i]=0;}
 }
 let mut expected_q=Poly17::ZERO;expected_q.len=(16-degree) as u8;
 expected_q.coefficients[..16-degree].copy_from_slice(&input.coefficients[degree..16]);
 expected_q.trim();
 let mut expected_r=Poly17::ZERO;expected_r.len=degree as u8;
 expected_r.coefficients[..degree].copy_from_slice(&input.coefficients[..degree]);
 expected_r.trim();
 let (q,r)=producer.division(&input,&modulus).unwrap();
 assert_eq!(q,expected_q);assert_eq!(r,expected_r);
 let mut reduced=input;producer.reduce_monic(&mut reduced,&modulus).unwrap();
 assert_eq!(reduced,expected_r);
 let rem=replay.remainder(input,&modulus).unwrap();assert_eq!(rem,expected_r);
 let mut canceled=input;replay.cancel_monic(&mut canceled,&modulus).unwrap();
 assert_eq!(canceled,expected_r);
 u64::from(q.len)+u64::from(r.len)+u64::from(rem.len)+u64::from(canceled.len)
}
"""
tests = r"""
#[cfg(test)]
mod round56 {
 use super::cryptanalysis::bounded::{after,before};
 use std::{hint::black_box,time::Instant};
 fn median(mut data:Vec<f64>)->f64{data.sort_by(f64::total_cmp);data[data.len()/2]}
 #[test]
 fn monomial_all_four_paths_match_independent_oracle() {
    for p in [7u64,1_000_000_007,(1u64<<61)-1] {
      for degree in [2usize,3,4,8] {
        assert_eq!(before::round56_oracle(p,degree),after::round56_oracle(p,degree));
        for kind in 0..4u8 {
          let old=before::round56_kernel(p,degree,kind,128);
          let new=after::round56_kernel(p,degree,kind,128);
          assert_eq!(old,new,"p={p} degree={degree} kind={kind}");
        }
      }
    }
    println!("ROUND56_NATIVE_ORACLE 24 independent oracle invocations and 48 before/after comparisons");
 }
 #[test]
 #[ignore="13 alternating Rust before/after trials"]
 fn benchmark_monomial_kernels() {
    for p in [1_000_000_007u64,(1u64<<61)-1] {
      for degree in [2usize,4,8] {
        for kind in 0..4u8 {
          let mut old=Vec::new();let mut new=Vec::new();
          for trial in 0..13 {
            if trial%2==0 {
              let t=Instant::now();
              black_box(before::round56_kernel(p,degree,kind,256));
              old.push(t.elapsed().as_nanos() as f64/256.0);
              let t=Instant::now();
              black_box(after::round56_kernel(p,degree,kind,256));
              new.push(t.elapsed().as_nanos() as f64/256.0);
            } else {
              let t=Instant::now();
              black_box(after::round56_kernel(p,degree,kind,256));
              new.push(t.elapsed().as_nanos() as f64/256.0);
              let t=Instant::now();
              black_box(before::round56_kernel(p,degree,kind,256));
              old.push(t.elapsed().as_nanos() as f64/256.0);
            }
          }
          let a=median(old);let b=median(new);
          println!("ROUND56_NATIVE p={p} degree={degree} kind={kind} before_ns={a:.2} after_ns={b:.2} speedup={:.3}",a/b);
        }
      }
    }
 }
}
"""
with tempfile.TemporaryDirectory() as tmp:
    path = Path(tmp) / "round56.rs"
    binary = Path(tmp) / "round56"
    path.write_text(header + source + helper + middle + before + helper + footer + tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"monomial_all_four_paths_match_independent_oracle","--nocapture"],check=True)
    subprocess.run([str(binary),"benchmark_monomial_kernels","--ignored","--nocapture"],check=True)
