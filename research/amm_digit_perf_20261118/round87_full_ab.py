#!/usr/bin/env python3
"""Paired, full source-file solve+verify A/B, with a stub for bounded API and a
primality whitelist limited to known prime benchmark inputs.

The entire producer and verifier in prime_root_trace.rs are copied verbatim
from the Round82 base and Round87 candidate, respectively, into two temporary Cargo
crates. Only the unrelated bounded.rs module and prime validation dependency
are stubbed. Does not measure actual index-calculus relation generation.
"""
from pathlib import Path
import csv
import subprocess
import tempfile
import statistics
import os

repo = Path(__file__).resolve().parents[2]
source_path = "src/cryptanalysis/prime_root_trace.rs"
baseline = subprocess.check_output(
    ["git", "show", "359e1140f5315feac52e116bd5ea96ab49b22389:" + source_path],
    cwd=repo, text=True
)
candidate = (repo / source_path).read_text()
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
  for case in ["linear","quadratic","quartic","split_quartic","octet"]{
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
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    print(f"benchmark affinity pinned to logical CPU {cpu}", flush=True)
except (AttributeError, OSError) as exc:
    print(f"CPU affinity unavailable: {exc}", flush=True)

with tempfile.TemporaryDirectory(prefix="root87-full-") as tmp:
    tmp = Path(tmp)
    bins = {}
    for label, source in [("before",baseline),("after",candidate)]:
        folder = tmp / label
        (folder / "src/cryptanalysis").mkdir(parents=True)
        (folder / "Cargo.toml").write_text(cargo)
        (folder / "src/main.rs").write_text(main)
        (folder / "src/cryptanalysis/prime_root_trace.rs").write_text(source)
        (folder / "src/cryptanalysis/prime_root_bounded.rs").write_text(
            "// The bounded API is out of scope for this exact producer+verifier benchmark.\n"
        )
        subprocess.run(["cargo","build","--release","--quiet"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "round87_remainder_only_replay_matches_independent_division",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "round82_full_split_frobenius_certificate_and_tampering",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "round81_linear_and_quartic_certificates_still_verify",
                            "--","--nocapture"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "smallest_odd_primes_and_constant_modulus_preserve_certificates",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "limited_work_and_sink_failures_remain_verifiable",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "terminal_base_elision_preserves_results_and_legacy_certificates",
                            "--","--nocapture"],cwd=folder,check=True)
        bins[label] = folder / "target/release/rootfind-isolated-ab"
    rows = []
    for trial in range(15):
        order = ("before","after") if trial % 2 == 0 else ("after","before")
        trial_data = {}
        for label in order:
            output = subprocess.check_output([str(bins[label])],text=True)
            trial_data[label] = {(r["prime"],r["case"]):r for r in csv.DictReader(output.splitlines())}
        assert trial_data["before"].keys()==trial_data["after"].keys()
        for key in trial_data["before"]:
            a,b=trial_data["before"][key],trial_data["after"][key]
            assert (a["checksum"],a["completed"])==(b["checksum"],b["completed"]),key
            rows.append({
                "trial":trial,"prime":key[0],"case":key[1],
                "before_ns":a["ns_per_solve_verify"],
                "after_ns":b["ns_per_solve_verify"]
            })
    out = repo / "research/amm_digit_perf_20261118/round87_full_ab_results.csv"
    with out.open("w",newline="") as stream:
        w=csv.DictWriter(stream,fieldnames=list(rows[0]))
        w.writeheader();w.writerows(rows)
    print("prime,case,before_ns,after_ns,speedup")
    for key in sorted(set((r["prime"],r["case"]) for r in rows)):
        series = [r for r in rows if (r["prime"],r["case"])==key]
        a = statistics.median(float(r["before_ns"]) for r in series)
        b = statistics.median(float(r["after_ns"]) for r in series)
        print(f"{key[0]},{key[1]},{a:.3f},{b:.3f},{a/b:.4f}")
    print(f"verified {len(rows)} before/after result-matched full solve+verify trials, 128 instances (8 for degree 8) each")
