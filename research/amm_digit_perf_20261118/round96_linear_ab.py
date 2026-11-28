#!/usr/bin/env python3
"""Round96 source-exact Rust A/B: linear quotient-ring powering and full solve+verify.
Both variants compile a single original prime_root_trace.rs source at frozen refs.
"""
import csv
import os
import pathlib
import statistics
import subprocess
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
SOURCE = "src/cryptanalysis/prime_root_trace.rs"
BASE = os.environ.get("ROOTFIND_BASELINE_REF", "2b5100f506001e3947658886731b9b4dbad4738e")
OUTPUT = pathlib.Path(os.environ.get("ROOTFIND_AB_OUTPUT", "research/amm_digit_perf_20261118/round96_native_ab.csv"))
CARGO = """[package]
name = "rootfind-round96-ab"
version = "0.1.0"
edition = "2021"
[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
"""
MAIN = r'''
#![allow(dead_code)]
mod cryptanalysis {
    pub mod residual_walk {
        pub fn is_prime_u64(p:u64)->bool {
            matches!(p,7|13|65521|1000000007|2305843009213693951)
        }
    }
    pub mod prime_root_trace;
}
use cryptanalysis::prime_root_trace::{benchmark_linear_power,solve,verify,RootLimits,MAX_WORK,MAX_EVENTS,MAX_DEGREE};
use std::hint::black_box;
use std::time::Instant;

fn limits() -> RootLimits {
    RootLimits{field_work:MAX_WORK,split_attempts:1000,events:MAX_EVENTS,degree:MAX_DEGREE}
}
fn next(s:&mut u64)->u64 {
    *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *s
}
#[derive(Clone)]
struct Case {
    base:Vec<u64>,
    modulus:Vec<u64>,
    exponent:u64,
    polynomial:Vec<u64>,
}
fn value(kind:&str,c:&Case,p:u64)->u64 {
    if kind=="power" {
        let result=benchmark_linear_power(
            black_box(&c.base),
            black_box(c.exponent),
            black_box(&c.modulus),
            black_box(p)).unwrap();
        result.iter().fold(17u64,|hash,&x|hash.wrapping_mul(131).wrapping_add(x))
    } else {
        let report=solve(black_box(&c.polynomial),black_box(p),7,limits(),&mut |_| Ok(())).unwrap();
        verify(&report).unwrap();
        assert!(report.complete());
        report.roots.iter().fold(report.events.len() as u64,|hash,&x|hash.wrapping_mul(131).wrapping_add(x))
    }
}
fn main(){
    let args:Vec<String>=std::env::args().collect();
    let p=args[1].parse::<u64>().unwrap();
    let kind=&args[2];
    let degree=args[3].parse::<usize>().unwrap();
    let mut seed=0x5296_2026_1121_u64;
    let mut cases=Vec::new();
    for index in 0..12u64 {
        let root=index+1;
        let constant=(p-root)%p;
        let lead=if index%2==0 {1} else {2+next(&mut seed)%(p-2)};
        let modulus=vec![next(&mut seed)%p,lead];
        let mut base=vec![next(&mut seed)%p,1];
        if index%4==0 {base=vec![next(&mut seed)%p,next(&mut seed)%p,0,1];}
        let exponent=if index%7==0 {0} else if index%3==0 {p} else {(p-1)/2};
        let polynomial=if degree==1 {
            vec![constant,1]
        } else {
            let next_root=root+1;
            vec![(root*next_root)%p,(p-((root+next_root)%p))%p,1]
        };
        cases.push(Case{base,modulus,exponent,polynomial});
    }
    let mut checksum=0u64;
    for c in &cases {
        checksum=checksum.wrapping_mul(109).wrapping_add(value(kind,c,p));
    }
    const REPEATS:usize=32;
    let mut result=0u64;
    let start=Instant::now();
    for _ in 0..REPEATS {
        for c in &cases {
            result=result.wrapping_mul(109).wrapping_add(black_box(value(kind,black_box(c),p)));
        }
    }
    black_box(result);
    let ns=start.elapsed().as_nanos() as f64/(REPEATS*cases.len()) as f64;
    println!("{p},{kind},{degree},{ns:.3},{checksum},{result}");
}
'''
ACCESSOR = """
#[doc(hidden)] pub fn benchmark_linear_power(b:&[u64],e:u64,m:&[u64],p:u64)->Result<Vec<u64>,String>{
    vpower(b,e,m,p)
}
"""
try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"CPU pinned to logical core {cpu}",flush=True)
except (AttributeError,OSError) as e:
    print(f"affinity unavailable: {e}",flush=True)

with tempfile.TemporaryDirectory(prefix="rootfind-round96-") as temp:
    temp=pathlib.Path(temp)
    before=subprocess.check_output(["git","show",BASE+":"+SOURCE],cwd=ROOT,text=True)
    after=(ROOT/SOURCE).read_text()
    executables={}
    for label,source in [("before",before),("after",after)]:
        folder=temp/label
        (folder/"src/cryptanalysis").mkdir(parents=True)
        (folder/"Cargo.toml").write_text(CARGO)
        (folder/"src/main.rs").write_text(MAIN)
        (folder/"src/cryptanalysis/prime_root_trace.rs").write_text(source+ACCESSOR)
        (folder/"src/cryptanalysis/prime_root_bounded.rs").write_text("// bounded is out of scope\n")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=folder,check=True)
        if label=="after":
            for test in [
                "round96_linear_modulus_scalar_power_matches_independent_polynomial_arithmetic",
                "round96_linear_root_certificate_replay",
                "round95_nonmonic_power_reduction_matches_legacy_reference",
                "round95_verify_nonmonic_root_certificates",
                "certificate_tampering_rejected",
            ]:
                subprocess.run(["cargo","test","--release","--quiet",test,"--","--nocapture"],cwd=folder,check=True)
        executables[label]=folder/"target/release/rootfind-round96-ab"
    OUTPUT.parent.mkdir(parents=True,exist_ok=True)
    with OUTPUT.open("w",newline="") as fd:
        writer=csv.DictWriter(fd,fieldnames=["p","kind","degree","trial","before_ns","after_ns","speedup","checksum"])
        writer.writeheader()
        for p in [65521,1000000007,(1<<61)-1]:
            for kind,degree in [("power",1),("complete",1),("complete",2)]:
                samples={"before":[],"after":[]}
                for trial in range(13):
                    result={}
                    order=["before","after"] if trial%2==0 else ["after","before"]
                    for label in order:
                        output=subprocess.check_output([str(executables[label]),str(p),kind,str(degree)],text=True).strip().split(",")
                        assert len(output)==6,output
                        result[label]=(float(output[3]),output[4],output[5])
                        samples[label].append(float(output[3]))
                    assert result["before"][1:]==result["after"][1:],(p,kind,degree,trial,result)
                    a,b=result["before"][0],result["after"][0]
                    writer.writerow(dict(p=p,kind=kind,degree=degree,trial=trial,before_ns=a,after_ns=b,
                                         speedup=a/b,checksum=result["after"][1]))
                a=statistics.median(samples["before"])
                b=statistics.median(samples["after"])
                print(f"p={p} kind={kind} degree={degree}: {a:.1f} -> {b:.1f} ns ({a/b:.3f}x), checksum OK",flush=True)
print(f"saved {OUTPUT}",flush=True)
