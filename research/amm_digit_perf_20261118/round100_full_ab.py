#!/usr/bin/env python3
"""Paired, full source-file solve+verify A/B, with a stub for bounded API and a
primality whitelist limited to known prime benchmark inputs.

The entire producer and verifier in prime_root_trace.rs are copied verbatim
from the frozen Round97 commit and Round98 candidate, respectively, into two temporary Cargo
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
baseline_ref = os.environ.get("ROOTFIND_BASELINE_REF", "06d01cfbf7f4df79f9f247240124eea80e8e6bf4")
baseline = subprocess.check_output(
    ["git", "show", baseline_ref + ":" + source_path],
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
            matches!(p,3|5|7|11|13|17|101|65521|1000000007|2305843009213693951)
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
   // Main currently reaches the fixed 100k event cap on p61 octet.
   // Compare completed workloads instead of treating censored reports as roots.
   if p==MAX_PRIME && case=="octet" {continue;}
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
  }
 }
 // Source-exact traced Engine::mul with identical observer and trace
 // accounting in frozen Round99 and candidate Round100.
 for p in [65_521u64, 1_000_000_007, MAX_PRIME] {
  for width in [4usize, 8, 12] {
   for shape in ["dense", "sparse", "unit", "mixed"] {
    let mut seed=0x726f_6f74_3130_3030u64 ^ p ^ width as u64;
    let mut pairs=Vec::new();
    for _ in 0..384usize {
     let mut a=vec![0u64;width]; let mut b=vec![0u64;width];
     for i in 0..width {
      seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
      a[i]=seed%p;
      seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
      b[i]=seed%p;
      match shape {
       "sparse" => {if i%3!=0{a[i]=0;}if i%2==1{b[i]=0;}},
       "unit" => {if i%3==1{a[i]=1;}if i%4==2{b[i]=1;}},
       "mixed" => {if i%3==1{a[i]=1;}if i%5<3{b[i]=0;}},
       _ => {}
      }
     }
     a[width-1]=1;b[width-1]=1;
     pairs.push((a,b));
    }
    let mut checksum=0u64;
    let t=Instant::now();
    let repeats=3usize;
    for _ in 0..repeats { for (a,b) in &pairs {
     let out=cryptanalysis::prime_root_trace::benchmark_traced_product(
        black_box(a),black_box(b),black_box(p));
     checksum=checksum.wrapping_mul(131).wrapping_add(out.len() as u64);
     for &v in &out {checksum=checksum.wrapping_mul(33).wrapping_add(v);}
     black_box(out);
    }}
    let completed=repeats*pairs.len();
    let ns=t.elapsed().as_nanos() as f64 / completed as f64;
    println!("{p},traced_{shape}_{width},{ns:.3},{checksum},{completed}");
   }
  }
 }
 // Direct source-exact verifier polynomial product microbenchmark. The only
 // source addition in temporary crates is a thin accessor for private vproduct.
 for p in [65521u64, 1_000_000_007, 4_294_967_291, MAX_PRIME] {
  for sparse in [false,true] {
   for degree in [4usize,8,12] {
    let mut seed=0x55e0_119a_8897_ab32u64 ^ p ^ (degree as u64);
    let mut inputs=Vec::new();
    for _ in 0..2048 {
     let mut a=vec![0u64;degree];let mut b=vec![0u64;degree];
     for x in a.iter_mut().chain(b.iter_mut()) {
       seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
       *x=if sparse && ((seed>>27)&1)==0 {0} else {seed%p};
     }
     if sparse {a[degree-1]=1;b[degree-1]=1;}
     inputs.push((a,b));
    }
    let start=Instant::now();let mut checksum=0u64;let mut count=0usize;
    for _ in 0..2 {
     for (a,b) in &inputs {
      let result=cryptanalysis::prime_root_trace::benchmark_product(
       black_box(a),black_box(b),black_box(p));
      checksum=checksum.wrapping_mul(33).wrapping_add(result[degree-1]);
      black_box(result);count+=1;
     }
    }
    let ns=start.elapsed().as_nanos() as f64 / count as f64;
    let name=if sparse {format!("product_sparse_{degree}")}
         else {format!("product_{degree}")};
    println!("{p},{name},{ns:.3},{checksum},{count}");
   }
  }
  for degree in [4usize,8,12] {
    let mut seed=0x76cf_7211_a929_0045u64 ^ p ^ (degree as u64);
    let mut inputs=Vec::new();
    for _ in 0..2048 {
      let mut a=vec![0u64;degree];
      for x in a.iter_mut() {
        seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *x=(seed%p).max(1);
      }
      inputs.push(a);
    }
    let start=Instant::now();let mut checksum=0u64;let mut count=0usize;
    for _ in 0..2 { for a in &inputs {
      let result=cryptanalysis::prime_root_trace::benchmark_square(black_box(a),black_box(p));
      checksum=checksum.wrapping_mul(33).wrapping_add(result[degree-1]);
      black_box(result);count+=1;
    }}
    let ns=start.elapsed().as_nanos() as f64 / count as f64;
    println!("{p},square_{degree},{ns:.3},{checksum},{count}");
  }
 }
 // Constructed binomial multiplication is a separate root-certificate
 // workload. The source-exact before/after checksums must match.
 for p in [65_521u64, 1_000_000_007, MAX_PRIME] {
  for degree in [6usize,8,12,16] {
   let mut seed=0x2026_1121_0091u64 ^ p ^ degree as u64;
   let mut inputs=Vec::new();
   for index in 0..1024usize {
    let mut a=Vec::with_capacity(degree);
    for _ in 0..degree {
     seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
     a.push((seed % (p-1))+1);
    }
    let mut b=vec![0;degree];
    b[0]=((seed>>13)%(p-1))+1;
    b[degree-1]=if index%2==0 {1} else {((seed>>7)%(p-1))+1};
    inputs.push((a,b));
   }
   let start=Instant::now();let mut checksum=0u64;let mut calls=0;
   for _ in 0..3 {
    for (a,b) in &inputs {
     let result=cryptanalysis::prime_root_trace::benchmark_verified_product(
         black_box(a),black_box(b),black_box(p));
     checksum=checksum.wrapping_mul(33).wrapping_add(result[degree-1]);
     black_box(result);calls+=1;
    }
   }
   let ns=start.elapsed().as_nanos() as f64 / calls as f64;
   println!("{p},product_binomial_{degree},{ns:.3},{checksum},{calls}");
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

with tempfile.TemporaryDirectory(prefix="root100-full-") as tmp:
    tmp = Path(tmp)
    bins = {}
    for label, source in [("before",baseline),("after",candidate)]:
        folder = tmp / label
        (folder / "src/cryptanalysis").mkdir(parents=True)
        (folder / "Cargo.toml").write_text(cargo)
        (folder / "src/main.rs").write_text(main)
        (folder / "src/cryptanalysis/prime_root_trace.rs").write_text(
            source + "\n\n#[doc(hidden)] pub(crate) fn benchmark_traced_product(a:&[u64],b:&[u64],p:u64)->Vec<u64>{\n let mut observer=|e:&RootEvent| -> Result<(), String> {std::hint::black_box(e);Ok(())};\n let report=RootReport {\n  schema:\"prime-root-trace/v1\".into(),\n  p,polynomial:vec![],seed:0,\n  limits:RootLimits {field_work:MAX_WORK,split_attempts:MAX_EVENTS,events:MAX_EVENTS,degree:MAX_DEGREE},\n  stop:RootStop::WorkLimit,counters:RootCounters::default(),\n  events:vec![],unacknowledged:None,sink_error:None,roots:vec![]\n };\n let mut engine=Engine{report,sink:&mut observer,next_id:0};\n let out=engine.mul(a,b).expect(\"isolated trace product exceeds limits\");\n std::hint::black_box(engine.report.counters);\n out\n}\n" + "\n#[doc(hidden)] pub(crate) fn benchmark_product(a:&[u64],b:&[u64],p:u64)->Vec<u64>{vproduct(a,b,p)} \n#[doc(hidden)] pub(crate) fn benchmark_square(a:&[u64],p:u64)->Vec<u64>{vsquare(a,p)}\n" +
            ("#[doc(hidden)] pub(crate) fn benchmark_verified_product(a:&[u64],b:&[u64],p:u64)->Vec<u64>{vverified_product(a,b,p)}\n"
             if label == "after" else
             "#[doc(hidden)] pub(crate) fn benchmark_verified_product(a:&[u64],b:&[u64],p:u64)->Vec<u64>{vverified_product(a,b,p)}\n")
        )
        (folder / "src/cryptanalysis/prime_root_bounded.rs").write_text(
            "// The bounded API is out of scope for this exact producer+verifier benchmark.\n"
        )
        subprocess.run(["cargo","build","--release","--quiet"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "round100_traced_multiplication", "--", "--nocapture"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "round99_quadratic_power_step_replay_matches_generic_polynomials",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "round98_quadratic_power_matches_independent_long_division",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "column_convolution_and_squaring_match_u128_reference",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "certificate_replay_matches_solver_after_clone_elimination",
                            "--","--nocapture"],cwd=folder,check=True)
            subprocess.run(["cargo","test","--release","--quiet",
                            "certificate_tampering_rejected",
                            "--","--nocapture"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "binomial_products_match_independent_arithmetic_oracle",
                            "--","--nocapture"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "round93_traced_division_matches_independent_quotient_remainder",
                            "--","--nocapture"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "exhaustive_quadratics_over_f7_match_direct_evaluation",
                            "--","--nocapture"],cwd=folder,check=True)
        if label == "after":
            subprocess.run(["cargo","test","--release","--quiet",
                            "round94_noop_reductions",
                            "--","--nocapture"],cwd=folder,check=True)
        bins[label] = folder / "target/release/rootfind-isolated-ab"
    rows = []
    for trial in range(17):
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
    out = repo / os.environ.get(
        "ROOTFIND_AB_OUTPUT",
        "research/amm_digit_perf_20261118/round100_full_ab_results.csv")
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
