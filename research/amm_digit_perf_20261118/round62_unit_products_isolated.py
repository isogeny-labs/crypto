#!/usr/bin/env python3
"""Round 62: source-exact A/B for sparse polynomial arithmetic and scalar powers."""
from pathlib import Path
import re
import subprocess
import tempfile

source=Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n",1)[0]
baseline=source
# The immediate baseline is this exact production source with only R62
# fast paths removed. All earlier rootfinding improvements are retained.
for key in ["PRODUCER_UNIT_SCALAR","PRODUCER_UNIT_LINEAR","PRODUCER_BINOMIAL_SQUARE",
            "REPLAY_UNIT_SCALAR","REPLAY_UNIT_LINEAR","REPLAY_BINOMIAL_SQUARE"]:
    pattern=r"(?s)// R62_"+key+r"_BEGIN.*?// R62_"+key+r"_END"
    baseline,n=re.subn(pattern,"",baseline,count=1)
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
pub fn case(p:u64,mode:u8,control:bool,seed:u64,reps:usize)->(u64,u64){
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
   let mut poly=Poly17::ZERO;
   poly.len=if mode>=6{5}else{8};
   let mut x=black_box(seed.wrapping_add(iteration as u64));
   for i in 0..poly.len as usize {
     x=x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
     poly.coefficients[i]=1+x%(p-1);
   }
   poly.coefficients[poly.len as usize-1]=1;
   let mut linear=Poly17::ZERO;
   linear.len=2;
   linear.coefficients[0]=if control{17%p}else if mode==4||mode==5{p-1}else{1};
   linear.coefficients[1]=1;
   let scalar=Poly17::constant(if control{17%p}else{p-1});
   if mode>=6 {
     poly.coefficients[0]=if control{17%p}else if iteration&1==0{1}else{p-1};
     if !control {for i in 1..4 {poly.coefficients[i]=0;}}
   }
   let poly=black_box(poly);
   let mut sample=0u64;
   let result=match mode{
      0=>producer.product(&poly,&scalar).unwrap(),
      1=>replay.convolution(&poly,&scalar).unwrap(),
      2|4=>producer.product(&poly,&linear).unwrap(),
      3|5=>replay.convolution(&poly,&linear).unwrap(),
      6=>producer.square(&poly).unwrap(),
      7=>replay.square_diagonals(&poly).unwrap(),
      _=>panic!("unexpected mode"),
   };
   for c in result.coefficients {sample=sample.wrapping_mul(37).wrapping_add(c);}
   digest=digest.wrapping_add(black_box(sample));
   let gate=if mode%2==0 {&mut producer.gate}else{&mut replay.gate};
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
mod round62{
 use super::cryptanalysis::bounded::{before,after};
 use std::{hint::black_box,time::Instant};
 fn median(mut a:Vec<f64>)->f64{a.sort_by(f64::total_cmp);a[a.len()/2]}
 #[test]
 fn exact_output_and_event_comparison(){
   let mut comparisons=0usize;
   for p in [3u64,5,7,101,1_000_000_007,1_000_000_009,(1u64<<61)-1] {
     for mode in 0..8u8 {
       for control in [false,true] {
         for seed in 0..64u64 {
           let old=before::case(p,mode,control,seed,1);
           let new=after::case(p,mode,control,seed,1);
           assert_eq!(old.0,new.0,"prime={p} mode={mode} control={control} seed={seed}");
           assert!(new.1<=old.1,"field ops increase at p={p} mode={mode} control={control}");
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
   println!("ROUND62_EQ comparisons={} complete_streams=32",comparisons);
 }
 #[test]
 #[ignore="13 alternating exact-source native Rust A/B kernel trials"]
 fn native_kernel_bench(){
   for p in [1_000_000_007u64,(1u64<<61)-1] {
     for mode in 0..8u8 {
       for control in [false,true] {
         let reps=256;
         let mut old=Vec::new();let mut new=Vec::new();
         for trial in 0..13 {
           let bench=|optimized:bool|{
             let start=Instant::now();
             if optimized {black_box(after::case(p,mode,control,1234567,reps));}
             else {black_box(before::case(p,mode,control,1234567,reps));}
             start.elapsed().as_nanos() as f64/reps as f64
           };
           if trial%2==0{old.push(bench(false));new.push(bench(true));}
           else{new.push(bench(true));old.push(bench(false));}
         }
         let before_ns=median(old);let after_ns=median(new);
         println!("ROUND62_NATIVE p={} mode={} control={} before_ns={:.2} after_ns={:.2} speedup={:.3}",
             p,mode,control,before_ns,after_ns,before_ns/after_ns);
       }
     }
   }
 }
 #[test]
 #[ignore="13 alternating full extraction and independent replay"]
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
         println!("ROUND62_WHOLE p={} degree={} sparse={} before_ns={:.2} after_ns={:.2} speedup={:.3}",
             p,degree,sparse,before_ns,after_ns,before_ns/after_ns);
       }
     }
   }
 }
}
"""
with tempfile.TemporaryDirectory() as tmp:
 path=Path(tmp)/"round62.rs"
 binary=Path(tmp)/"round62"
 path.write_text(head+source+helper+mid+baseline+helper+tail+tests)
 subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
 subprocess.run([str(binary),"exact_output_and_event_comparison","--nocapture"],check=True)
 subprocess.run([str(binary),"native_kernel_bench","--ignored","--nocapture"],check=True)
 subprocess.run([str(binary),"native_whole_bench","--ignored","--nocapture"],check=True)
