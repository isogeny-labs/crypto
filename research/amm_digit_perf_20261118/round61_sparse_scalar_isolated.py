#!/usr/bin/env python3
"""Round 61: source-exact A/B for sparse polynomial arithmetic and scalar powers."""
from pathlib import Path
import re
import subprocess
import tempfile

source=Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n",1)[0]
baseline=source
replacements={
 "PRODUCT_LINEAR_ZERO": "if a.coefficients[0] == 0 || b.coefficients[0] == 0 {",
 "REPLAY_LINEAR_ZERO": "let sparse = x.coefficients[0] == 0 || y.coefficients[0] == 0;",
 "PRODUCER_SCALAR_IDENTITY": """let mut acc = 1u64;
        let mut factor = base;
        while exponent != 0 {
            if exponent & 1 != 0 {
                acc = self.mul(acc, factor)?;
            }
            exponent >>= 1;
            if exponent != 0 {
                factor = self.mul(factor, factor)?;
            }
        }
        Ok(acc)""",
 "REPLAY_SCALAR_IDENTITY": """let mut acc = 1u64;
        for bit in (0..64 - exponent.leading_zeros()).rev() {
            acc = self.times(acc, acc)?;
            if exponent & (1u64 << bit) != 0 {
                acc = self.times(acc, x)?;
            }
        }
        Ok(acc)"""
}
for key,old in replacements.items():
    regex=r"(?s)// R61_"+key+r"_BEGIN.*?// R61_"+key+r"_END"
    baseline,n=re.subn(regex,lambda _:old,baseline,count=1)
    assert n==1,key

head="""#![allow(dead_code)]
mod cryptanalysis {
  pub mod fp_backend {
    pub struct Fp64Arithmetic {pub p:u64}
    impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
  }
  pub mod bounded {
    pub mod after {
"""
mid="\n} pub mod before {\n"
tail="\n}\n}\n}\n"

helper=r"""
pub fn case(p:u64,mode:u8,sparse:bool,seed:u64,reps:usize)->(u64,u64){
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
 let mut digest=0u64;
 let mut field_count=0u64;
 for iteration in 0..reps {
   let mut a=Poly17::ZERO;let mut b=Poly17::ZERO;
   a.len=9;b.len=9;
   let mut x=black_box(seed.wrapping_add(iteration as u64));
   for i in 0..9 {
     x=x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
     a.coefficients[i]=1+x%(p-1);
     x=x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
     b.coefficients[i]=1+x%(p-1);
   }
   a.coefficients[8]=1;b.coefficients[8]=1;
   if sparse {
     for i in (1..8).step_by(2){a.coefficients[i]=0;b.coefficients[i]=0;}
   }
   let a=black_box(a);let b=black_box(b);
   let mut sample=0u64;
   if mode<=1 {
     let result=if mode==0{producer.product(&a,&b).unwrap()}
                else{replay.convolution(&a,&b).unwrap()};
     for c in result.coefficients {sample=sample.wrapping_mul(37).wrapping_add(c);}
   }else{
     let exponent=black_box((p-1)/4).wrapping_add(iteration as u64%7);
     sample=if mode==2{producer.split_scalar_pow(a.coefficients[0],exponent).unwrap()}
            else{replay.split_character_power(a.coefficients[0],exponent).unwrap()};
   }
   digest=digest.wrapping_add(black_box(sample));
   let gate=if mode==0||mode==2 {&mut producer.gate}else{&mut replay.gate};
   field_count+=gate.metrics.field_work;
   gate.metrics.field_work=0;gate.metrics.work=0;
 }
 (digest,field_count)
}

pub fn complete(p:u64,degree:usize,sparse:bool,leading:u64) ->
    (Stop,[u64;8],u8,Vec<BoundedRootEvent>) {
 use std::time::Duration;
 let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,bytes:STORAGE_BYTES,
     events:MAX_EVENTS,split_attempts:1000,degree:8};
 let field=super::super::fp_backend::Fp64Arithmetic{p};
 let mut coefficients=[0u64;9];
 let len=if sparse{
    coefficients[0]=17;
    coefficients[degree/2]=3;
    coefficients[degree]=1;
    degree+1
 }else{
    coefficients[0]=1;
    let mut n=1usize;
    for root in 1..=degree{
       let neg=p-root as u64;
       let mut next=[0u64;9];
       for i in 0..n {
         next[i]=((u128::from(next[i])+u128::from(coefficients[i])*u128::from(neg))%u128::from(p)) as u64;
         next[i+1]=((u128::from(next[i+1])+u128::from(coefficients[i]))%u128::from(p)) as u64;
       }
       coefficients=next;n+=1;
    }
    n
 };
 for c in &mut coefficients[..len] {*c=((u128::from(*c)*u128::from(leading))%u128::from(p)) as u64;}
 let coeffs=&coefficients[..len];
 let mut events=Vec::new();
 let report=solve_bounded(&field,coeffs,42,limits,Instant::now()+Duration::from_secs(120),
     &mut |e|{events.push(*e);Ok(())});
 if report.stop==Stop::Complete {
   let mut replay=Replay::new(&field,coeffs,42,limits,limits,
                             Instant::now()+Duration::from_secs(120)).unwrap();
   for event in &events {replay.push(event).unwrap();}
   assert!(replay.finish(&report).unwrap().finite_complete);
 }
 (report.stop,report.roots,report.root_count,events)
}
"""
tests=r"""
#[cfg(test)]
mod round61{
 use super::cryptanalysis::bounded::{before,after};
 use std::{hint::black_box,time::Instant};
 fn median(mut a:Vec<f64>)->f64{a.sort_by(f64::total_cmp);a[a.len()/2]}
 #[test]
 fn exact_output_and_event_comparison(){
   let mut comparisons=0usize;
   for p in [3u64,5,7,101,1_000_000_007,1_000_000_009,(1u64<<61)-1] {
     for mode in 0..4u8 {
       for sparse in [false,true] {
         for seed in 0..64u64 {
           let old=before::case(p,mode,sparse,seed,1);
           let new=after::case(p,mode,sparse,seed,1);
           assert_eq!(old.0,new.0,"prime={p} mode={mode} sparse={sparse} seed={seed}");
           assert!(new.1<=old.1,"field ops increase at p={p} mode={mode} sparse={sparse}");
           comparisons+=1;
         }
       }
     }
   }
   for p in [101u64,1_000_000_007,1_000_000_009,(1u64<<61)-1] {
     for degree in [4usize,8] {
       for sparse in [false,true] {
         for leading in [1u64,p-1] {
           let old=before::complete(p,degree,sparse,leading);
           let new=after::complete(p,degree,sparse,leading);
           assert_eq!(format!("{old:?}"),format!("{new:?}"),"complete prime={p} degree={degree} sparse={sparse} leading={leading}");
         }
       }
     }
   }
   println!("ROUND61_EQ comparisons={} complete_streams=32",comparisons);
 }
 #[test]
 #[ignore="13 alternating exact-source Rust before/after trials"]
 fn native_kernel_bench(){
   for p in [1_000_000_007u64,(1u64<<61)-1] {
     for mode in 0..4u8 {
       for sparse in [false,true] {
         if mode>=2&&sparse{continue;}
         let reps=if mode>=2{96}else{256};
         let mut old=Vec::new();let mut new=Vec::new();
         for trial in 0..13 {
           let bench=|optimized:bool|{
             let start=Instant::now();
             if optimized {black_box(after::case(p,mode,sparse,1234567,reps));}
             else {black_box(before::case(p,mode,sparse,1234567,reps));}
             start.elapsed().as_nanos() as f64/reps as f64
           };
           if trial%2==0{old.push(bench(false));new.push(bench(true));}
           else{new.push(bench(true));old.push(bench(false));}
         }
         let before_ns=median(old);let after_ns=median(new);
         println!("ROUND61_NATIVE p={} mode={} sparse={} before_ns={:.2} after_ns={:.2} speedup={:.3}",
             p,mode,sparse,before_ns,after_ns,before_ns/after_ns);
       }
     }
   }
 }
 #[test]
 #[ignore="13 alternating complete extraction and independent certificate replay trials"]
 fn native_whole_bench(){
   for p in [1_000_000_007u64,(1u64<<61)-1]{
     for degree in [4usize,8]{
       for sparse in [false,true]{
         let reps=4;
         let mut old=Vec::new();let mut new=Vec::new();
         for trial in 0..13{
           let bench=|optimized:bool|{
             let start=Instant::now();
             for _ in 0..reps {
               if optimized{black_box(after::complete(p,degree,sparse,1));}
               else{black_box(before::complete(p,degree,sparse,1));}
             }
             start.elapsed().as_nanos() as f64/reps as f64
           };
           if trial%2==0{old.push(bench(false));new.push(bench(true));}
           else{new.push(bench(true));old.push(bench(false));}
         }
         let before_ns=median(old);let after_ns=median(new);
         println!("ROUND61_WHOLE p={} degree={} sparse={} before_ns={:.2} after_ns={:.2} speedup={:.3}",
             p,degree,sparse,before_ns,after_ns,before_ns/after_ns);
       }
     }
   }
 }
}
"""
with tempfile.TemporaryDirectory() as tmp:
 path=Path(tmp)/"round61.rs"
 binary=Path(tmp)/"round61"
 path.write_text(head+source+helper+mid+baseline+helper+tail+tests)
 subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
 subprocess.run([str(binary),"exact_output_and_event_comparison","--nocapture"],check=True)
 subprocess.run([str(binary),"native_kernel_bench","--ignored","--nocapture"],check=True)
 subprocess.run([str(binary),"native_whole_bench","--ignored","--nocapture"],check=True)
