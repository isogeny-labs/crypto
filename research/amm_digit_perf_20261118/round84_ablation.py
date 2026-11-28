#!/usr/bin/env python3
"""Ablate Round83's event-capacity shortcut vs degree-eight factor construction.
The sources under test are whole prime_root_trace.rs versions, not C++ analogues.
Only the unrelated bounded API and primality admission are replaced with known-prime
benchmark stubs. Full repository integration and relations/sec are not measured.
"""
from pathlib import Path
import csv
import subprocess
import tempfile
import statistics
import os

repo = Path(__file__).resolve().parents[2]
path = "src/cryptanalysis/prime_root_trace.rs"
source82 = subprocess.check_output(["git","show","359e1140f5315feac52e116bd5ea96ab49b22389:"+path],cwd=repo,text=True)
source83 = subprocess.check_output(["git","show","58b92cd77fed6a1ccc9ed892132de148d0fbf6dc:"+path],cwd=repo,text=True)
def method_block(source):
    start = source.index("    fn event(&mut self, event: RootEvent) -> Work<()> {")
    end = source.index("    fn field(&mut self, op: FieldOp",start)
    return source[start:end]
old,fast = method_block(source82),method_block(source83)
assert "event_after_room" not in old
assert "event_after_room" in fast
def toggle_event(source,enable):
    if enable:
        assert old in source
        source=source.replace(old,fast,1)
        before = "self.event(RootEvent::Field { op, a, b, value })?;"
        after = "self.event_after_room(RootEvent::Field { op, a, b, value })?;"
    else:
        assert fast in source
        source=source.replace(fast,old,1)
        before = "self.event_after_room(RootEvent::Field { op, a, b, value })?;"
        after = "self.event(RootEvent::Field { op, a, b, value })?;"
    assert source.count(before)==1
    return source.replace(before,after,1)
sources={
    "round82":source82,
    "event_only":toggle_event(source82,True),
    "factor_only":toggle_event(source83,False),
    "round83":source83,
}
cargo = """[package]
name="rootfind-isolated-ab"
version="0.1.0"
edition="2021"
[dependencies]
rand="0.8.5"
serde={version="1.0", features=["derive"]}
"""

main = r"""
#![allow(dead_code)]
mod cryptanalysis {
    pub mod residual_walk {
        // The benchmark and regression-test p values are known primes; admission/primality cost
        // is deliberately excluded from this arithmetic throughput study.
        pub fn is_prime_u64(p: u64) -> bool {
            matches!(p,3|5|7|11|17|101|65521|1000000007|2305843009213693951)
        }
    }
    pub mod prime_root_trace;
}
use cryptanalysis::prime_root_trace::{
    solve,verify,RootEvent,RootLimits,MAX_WORK,MAX_EVENTS,MAX_DEGREE,MAX_PRIME
};
use std::hint::black_box;
use std::time::Instant;

fn product(a:&[u64],b:&[u64],p:u64)->Vec<u64>{
 let mut r=vec![0;a.len()+b.len()-1];
 for (i,&x) in a.iter().enumerate(){for (j,&y) in b.iter().enumerate(){
  r[i+j]=((r[i+j] as u128+x as u128*y as u128)%p as u128) as u64;
 }}r
}
fn linear(root:u64,p:u64)->Vec<u64>{vec![p-root,1]}
fn poly(p:u64,case:&str,index:u64)->Vec<u64>{
 let a=2+((index*13+17)%131);
 let b=a+7;
 match case{
  "linear"=>linear(a,p),
  "quadratic"=>product(&linear(a,p),&linear(b,p),p),
  "quartic"=>{
   let q=product(&linear(a,p),&linear(b,p),p);
   product(&q,&[1,0,1],p)
  },
  "split_quartic" | "octet" => {
   let count=if case=="octet" {8} else {4};
   let mut result=vec![1];
   for j in 0..count {result=product(&result,&linear(a+7*j,p),p);}
   result
  },
  _=>unreachable!()
 }
}
fn main(){
 println!("prime,case,ns_per_solve_verify,checksum,completed");
 for p in [65521,1000000007,MAX_PRIME]{
  for case in ["quadratic","quartic","octet"]{
   let t=Instant::now();
   let mut checksum=0u64;
   let mut completed=0;
   let repeats=if case=="octet" {2} else {16};
   let input_count=if case=="octet" {4} else {8};
   for _repeat in 0..repeats { for i in 0..input_count {
    let f=poly(p,case,i);
    let mut sink=|event:&RootEvent|{black_box(event);Ok(())};
    let limits=RootLimits{field_work:MAX_WORK,split_attempts:1000,events:MAX_EVENTS,degree:MAX_DEGREE};
    let report=solve(black_box(&f),p,7+i,limits,&mut sink).unwrap();
    assert!(report.complete(),"incomplete p={p} case={case} i={i} stop={:?}",report.stop);
    verify(&report).unwrap();
    let a=2+((i*13+17)%131);
    assert!(report.roots.contains(&a));
    if case!="linear"{assert!(report.roots.contains(&(a+7)));}
    if case=="split_quartic" || case=="octet" {
     let count=if case=="octet" {8} else {4};
     for j in 0..count {assert!(report.roots.contains(&(a+7*j)), "missing root {j} for {case}");}
    }
    checksum=checksum.wrapping_mul(131).wrapping_add(p);
    for &root in &report.roots { checksum=checksum.wrapping_mul(33).wrapping_add(root); }
    completed+=1;
    black_box(report);
   } }
   let ns=t.elapsed().as_nanos() as f64/completed as f64;
   println!("{p},{case},{ns:.3},{checksum},{completed}");
   // Separately measure independent certificate replay, without including
   // the producer. Compare its checksum and completion with the baseline.
   let mut prepared = Vec::new();
   for i in 0..input_count {
      let f = poly(p,case,i);
      let limits = RootLimits{field_work:MAX_WORK,split_attempts:1000,events:MAX_EVENTS,degree:MAX_DEGREE};
      let mut sink = |event:&RootEvent|{black_box(event);Ok(())};
      let report = solve(&f,p,7+i,limits,&mut sink).unwrap();
      assert!(report.complete());
      verify(&report).unwrap();
      prepared.push(report);
   }
   let started = Instant::now();
   let mut checksum=0u64;
   let mut verified=0u64;
   for _ in 0..repeats {
      for report in &prepared {
         verify(black_box(report)).unwrap();
         checksum=checksum.wrapping_mul(131).wrapping_add(report.p);
         for &root in &report.roots {
             checksum=checksum.wrapping_mul(33).wrapping_add(root);
         }
         verified += 1;
      }
   }
   let ns=started.elapsed().as_nanos() as f64/verified as f64;
   println!("{p},verify_only_{case},{ns:.3},{checksum},{verified}");
  }
 }
}
"""

try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"affinity pinned to CPU {cpu}",flush=True)
except (AttributeError,OSError) as exc:
    print(f"affinity unavailable: {exc}",flush=True)

with tempfile.TemporaryDirectory(prefix="round84-ablation-") as tmp:
    tmp=Path(tmp)
    binaries={}
    for label,source in sources.items():
        d=tmp/label
        (d/"src/cryptanalysis").mkdir(parents=True)
        (d/"Cargo.toml").write_text(cargo)
        (d/"src/main.rs").write_text(main)
        (d/path).write_text(source)
        (d/"src/cryptanalysis/prime_root_bounded.rs").write_text("// intentionally excluded bounded API\n")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=d,check=True)
        binaries[label]=d/"target/release/rootfind-isolated-ab"
    rows=[]
    labels=list(sources)
    for trial in range(9):
        ordered=labels[trial%4:]+labels[:trial%4]
        results={}
        for label in ordered:
            output=subprocess.check_output([str(binaries[label])],text=True)
            results[label]={(r["prime"],r["case"]):r for r in csv.DictReader(output.splitlines())}
        assert all(results[k].keys()==results["round82"].keys() for k in labels)
        for key in results["round82"]:
            expected=results["round82"][key]
            for label in labels:
                cur=results[label][key]
                assert (cur["checksum"],cur["completed"])==(expected["checksum"],expected["completed"]),(label,key)
                rows.append(dict(trial=trial,label=label,prime=key[0],case=key[1],ns=cur["ns_per_solve_verify"]))
    output=repo/"research/amm_digit_perf_20261118/round84_event_factor_ablation.csv"
    with output.open("w",newline="") as file:
        writer=csv.DictWriter(file,fieldnames=["trial","label","prime","case","ns"])
        writer.writeheader();writer.writerows(rows)
    print("prime,case,round82_ns,event_only_speedup,factor_only_speedup,round83_speedup")
    for key in sorted(results["round82"]):
        medians={}
        for label in labels:
            medians[label]=statistics.median(float(x["ns"]) for x in rows if x["label"]==label and (x["prime"],x["case"])==key)
        p,c=key
        print(f'{p},{c},{medians["round82"]:.3f},{medians["round82"]/medians["event_only"]:.4f},{medians["round82"]/medians["factor_only"]:.4f},{medians["round82"]/medians["round83"]:.4f}')
    print(f"validated {len(rows)} case/variant/trial checksum-equivalent measurements across 4 source variants")
