#!/usr/bin/env python3
"""Source-exact before/after native Rust benchmark for terminal-square omission.

The full producer/verifier Rust file is identical to the frozen Round91 commit
or the current branch. Only bounded.rs and prime admission are stubbed.
"""
import csv
import os
from pathlib import Path
import statistics
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[2]
source_path = "src/cryptanalysis/prime_root_trace.rs"
old_ref = os.environ.get("ROOTFIND_BASELINE_REF", "d13f9f17589af06a90fd74978165ad916dbe0e7b")
baseline = subprocess.check_output(["git", "show", old_ref+":"+source_path], cwd=repo, text=True)
candidate = (repo/source_path).read_text()
cargo = """[package]
name="round92-isolated"
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
        pub fn is_prime_u64(p:u64)->bool {
            matches!(p, 3|5|7|11|13|17|101|65521|1000000007|2305843009213693951)
        }
    }
    pub mod prime_root_trace;
}
use cryptanalysis::prime_root_trace::{solve,verify,RootEvent,RootLimits,MAX_WORK,MAX_EVENTS,MAX_DEGREE,MAX_PRIME};
use std::hint::black_box;
use std::time::Instant;

fn product(a:&[u64],b:&[u64],p:u64)->Vec<u64>{
    let mut out=vec![0;a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate(){
        for (j,&y) in b.iter().enumerate(){
            out[i+j]=((out[i+j] as u128+x as u128*y as u128)%p as u128) as u64;
        }
    }
    out
}
fn polynomial(p:u64,deg:usize,i:u64)->Vec<u64> {
    let mut out=vec![1u64];
    for j in 0..deg {
        out=product(&out,&[p-(2+i*17+j as u64*7),1],p);
    }
    out
}
fn main() {
    println!("prime,case,ns_per_op,checksum,completed");
    for p in [65521u64,1_000_000_007,MAX_PRIME]{
        for degree in [3usize,5usize] {
            for width in [8usize,12usize,16usize] {
                let mut seed=0x925c_2026_1121_1234u64 ^ p ^ degree as u64 ^ width as u64;
                let mut data=Vec::with_capacity(1024);
                for _ in 0..1024 {
                    let mut a=vec![0u64;width];
                    let mut b=vec![0u64;degree+1];
                    for x in a.iter_mut().chain(b.iter_mut()) {
                        seed=seed.wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        *x=1+(seed % (p-1));
                    }
                    b[degree]=1;
                    data.push((a,b));
                }
                let start=Instant::now();
                let mut checksum=0u64;
                let mut completed=0usize;
                for _ in 0..4 {
                    for (a,b) in &data {
                        let (q,r)=cryptanalysis::prime_root_trace::round92_benchmark_division(
                            black_box(a),black_box(b),black_box(p));
                        checksum=checksum.wrapping_mul(131)
                            .wrapping_add(q.len() as u64)
                            .wrapping_add(r.len() as u64)
                            .wrapping_add(*q.first().unwrap_or(&0))
                            .wrapping_add(*r.first().unwrap_or(&0));
                        completed+=1;
                        black_box((q,r));
                    }
                }
                let ns=start.elapsed().as_nanos() as f64 / completed as f64;
                let shape=if degree==3 {"cubic"} else {"quintic"};
                println!("{p},division_{shape}_w{width},{ns:.3},{checksum},{completed}");
            }
        }
        for degree in [2usize,4usize] {
            let mut checksum=0u64;
            let start=Instant::now();
            for i in 0..8 {
                let f=polynomial(p,degree,i);
                let limits=RootLimits{field_work:MAX_WORK,split_attempts:1000,events:MAX_EVENTS,degree:MAX_DEGREE};
                let report=solve(&f,p,7+i,limits,&mut |event:&RootEvent|{
                    black_box(event);Ok(())
                }).unwrap();
                assert!(report.complete(), "incomplete p={p} degree={degree}, stop={:?}",report.stop);
                verify(&report).unwrap();
                for &r in &report.roots {checksum=checksum.wrapping_mul(131).wrapping_add(r);}
                checksum=checksum.wrapping_add(report.roots.len() as u64);
                black_box(report);
            }
            let ns=start.elapsed().as_nanos() as f64/8.0;
            println!("{p},solve_verify_deg{degree},{ns:.3},{checksum},8");
        }
    }
}
"""
try:
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"pinned to CPU {cpu}",flush=True)
except (OSError,AttributeError) as exc:
    print(f"affinity not available: {exc}",flush=True)

with tempfile.TemporaryDirectory(prefix="root92-rust-") as temp:
    temp = Path(temp)
    binaries={}
    for label,source in [("before",baseline),("after",candidate)]:
        directory=temp/label
        (directory/"src/cryptanalysis").mkdir(parents=True)
        (directory/"Cargo.toml").write_text(cargo)
        (directory/"src/main.rs").write_text(main)
        (directory/"src/cryptanalysis/prime_root_trace.rs").write_text(
            source+"\n#[doc(hidden)] pub(crate) fn round92_benchmark_division(a:&[u64],b:&[u64],p:u64)->(Vec<u64>,Vec<u64>){vdiv(a,b,p).unwrap()}\n"
        )
        (directory/"src/cryptanalysis/prime_root_bounded.rs").write_text("")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=directory,check=True)
        if label=="after":
            subprocess.run([
                "cargo","test","--release","--quiet",
                "round92_skips_terminal_power_squares_but_preserves_complete_certificates",
                "--","--nocapture"
            ],cwd=directory,check=True)
            subprocess.run([
                "cargo","test","--release","--quiet","certificate_tampering_rejected",
                "--","--nocapture"
            ],cwd=directory,check=True)
        binaries[label]=directory/"target/release/round92-isolated"
    rows=[]
    for trial in range(13):
        order=("before","after") if trial%2==0 else ("after","before")
        results={}
        for label in order:
            output=subprocess.check_output([str(binaries[label])],text=True)
            results[label]={(r["prime"],r["case"]):r for r in csv.DictReader(output.splitlines())}
        assert results["before"].keys()==results["after"].keys()
        for key in results["before"]:
            a,b=results["before"][key],results["after"][key]
            assert (a["checksum"],a["completed"])==(b["checksum"],b["completed"]),key
            rows.append({"trial":trial,"prime":key[0],"case":key[1],
                         "before_ns":a["ns_per_op"],"after_ns":b["ns_per_op"]})
    out=repo/"research/amm_digit_perf_20261118/round92_ab_results.csv"
    with out.open("w",newline="") as f:
        writer=csv.DictWriter(f,fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    print("prime,case,before_ns,after_ns,speedup")
    for key in sorted({(r["prime"],r["case"]) for r in rows}):
        arr=[r for r in rows if (r["prime"],r["case"])==key]
        a=statistics.median(float(r["before_ns"]) for r in arr)
        b=statistics.median(float(r["after_ns"]) for r in arr)
        print(f"{key[0]},{key[1]},{a:.3f},{b:.3f},{a/b:.4f}")
    print(f"PASS: {len(rows)} checksum-matched A/B case-trials")
