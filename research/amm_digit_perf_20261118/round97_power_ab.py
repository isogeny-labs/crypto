#!/usr/bin/env python3
"""Source-exact Round97 nonmonic-modulus vpower paired A/B (native Rust).
Isolates the actual verifier vpower implementation from two frozen source refs.
This does NOT claim index-calculus throughput or local PMU counters.
"""
import csv, os, pathlib, statistics, subprocess, tempfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
SOURCE = "src/cryptanalysis/prime_root_trace.rs"
BASE = os.environ.get("ROOTFIND_BASELINE_REF", "baf8f8621b7a764f0f5705100f36af1090615db7")
OUTPUT = pathlib.Path(os.environ.get("ROOTFIND_AB_OUTPUT", "research/amm_digit_perf_20261118/round97_native_ab.csv"))
CARGO = """[package]
name="rootfind-round97-ab"
version="0.1.0"
edition="2021"
[dependencies]
rand="0.8.5"
serde={version="1.0",features=["derive"]}
"""
MAIN = r'''
#![allow(dead_code)]
mod cryptanalysis {
    pub mod residual_walk {
        pub fn is_prime_u64(p:u64)->bool{
            matches!(p,3|5|7|11|13|17|65521|1000000007|2305843009213693951)
        }
    }
    pub mod prime_root_trace;
}
use cryptanalysis::prime_root_trace::benchmark_power;
use std::hint::black_box;
use std::time::Instant;
fn next(s:&mut u64)->u64{
    *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*s
}
fn main(){
    let args:Vec<String>=std::env::args().collect();
    let p=args[1].parse::<u64>().unwrap();
    let degree=args[2].parse::<usize>().unwrap();
    let monic=args[3].parse::<bool>().unwrap();
    let mut seed=0x3e9914542661795u64;
    let mut cases=Vec::new();
    for index in 0..12 {
        let mut m=vec![0;degree+1];
        for coefficient in &mut m {*coefficient=next(&mut seed)%p;}
        m[degree]=if monic {1} else {2+next(&mut seed)%(p-2)};
        if index%3==0 {
            for coefficient in &mut m[1..degree] {*coefficient=0;}
        }
        let b=vec![next(&mut seed)%p,1];
        let e=if p==(1u64<<61)-1 {(p-1)/2} else {p};
        cases.push((b,e,m));
    }
    let mut checksum=0u64;
    for (b,e,m) in &cases {
        for &value in &benchmark_power(b,*e,m,p).unwrap(){
            checksum=checksum.wrapping_mul(131).wrapping_add(value);
        }
    }
    // Measured interval excludes fixture construction and stdout; avoid
    // constant-folding input values and preserve observable result checksums.
    let start=Instant::now();
    let mut result=0u64;
    for _ in 0..24 {
        for (b,e,m) in &cases {
            let values=black_box(benchmark_power(black_box(b),black_box(*e),
                black_box(m),black_box(p)).unwrap());
            for &value in &values {
                result=result.wrapping_mul(33).wrapping_add(value);
            }
        }
    }
    black_box(result);
    let ns=start.elapsed().as_nanos() as f64/(24*cases.len()) as f64;
    println!("{p},{degree},{monic},{ns:.3},{checksum},{result}");
}
'''
ACCESSOR = """
#[doc(hidden)] pub fn benchmark_power(b:&[u64],e:u64,m:&[u64],p:u64)->Result<Vec<u64>,String>{
    vpower(b,e,m,p)
}
"""
try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print(f"CPU pinned to logical core {cpu}",flush=True)
except (AttributeError,OSError) as e:
    print(f"affinity unavailable: {e}",flush=True)

with tempfile.TemporaryDirectory(prefix="rootfind-round97-") as tmp:
    tmp=pathlib.Path(tmp)
    old=subprocess.check_output(["git","show",BASE+":"+SOURCE],cwd=ROOT,text=True)
    new=(ROOT/SOURCE).read_text()
    executables={}
    for label,source in [("before",old),("after",new)]:
        folder=tmp/label
        (folder/"src/cryptanalysis").mkdir(parents=True)
        (folder/"Cargo.toml").write_text(CARGO)
        (folder/"src/main.rs").write_text(MAIN)
        (folder/"src/cryptanalysis/prime_root_trace.rs").write_text(source+ACCESSOR)
        (folder/"src/cryptanalysis/prime_root_bounded.rs").write_text("// bounded is out of scope\n")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=folder,check=True)
        if label=="after":
            for test in [
                "round97_nonmonic_power_reduction_matches_legacy_reference",
                "round97_verify_nonmonic_root_certificates",
                "round93_traced_division_matches_independent_quotient_remainder",
                "round97_quotient_free_power_reduction_matches_division_remainder",
                "certificate_tampering_rejected",
            ]:
                subprocess.run(["cargo","test","--release","--quiet",test,"--","--nocapture"],cwd=folder,check=True)
        executables[label]=folder/"target/release/rootfind-round97-ab"
    OUTPUT.parent.mkdir(parents=True,exist_ok=True)
    with OUTPUT.open("w",newline="") as fd:
        w=csv.DictWriter(fd,fieldnames=["p","degree","monic","trial","before_ns","after_ns","speedup","checksum"])
        w.writeheader()
        for p in [65521,1000000007,(1<<61)-1]:
            for degree in [2,3,4,8]:
                for monic in [False,True]:
                    timing={"before":[],"after":[]}
                    checksums=set()
                    for trial in range(13):
                        results={}
                        order=["before","after"] if trial%2==0 else ["after","before"]
                        for label in order:
                            line=subprocess.check_output(
                                [str(executables[label]),str(p),str(degree),str(monic).lower()],
                                text=True).strip().split(",")
                            assert len(line)==6,line
                            ns=float(line[3])
                            timing[label].append(ns)
                            results[label]=(ns,line[4],line[5])
                            checksums.add((line[4],line[5]))
                        assert len(checksums)==1,(p,degree,monic,checksums)
                        a,b=results["before"][0],results["after"][0]
                        w.writerow(dict(p=p,degree=degree,monic=int(monic),trial=trial,
                                        before_ns=a,after_ns=b,speedup=a/b,checksum=results["after"][1]))
                    before=statistics.median(timing["before"])
                    after=statistics.median(timing["after"])
                    print(f"p={p} d={degree} monic={monic}: {before:.1f} -> {after:.1f} ns "
                          f"({before/after:.3f}x) checksum=OK",flush=True)
print(f"saved {OUTPUT}",flush=True)
