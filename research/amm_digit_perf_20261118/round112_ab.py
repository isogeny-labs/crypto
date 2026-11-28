#!/usr/bin/env python3
"""Source-exact production Rust A/B for p61 high-degree quotient powering.

Reads frozen Round111 and this branch's unmodified prime_root_trace.rs.
Adds only the same private benchmark accessor to both temporary crates.
CPU-pinned, alternating execution order; checksum equality required.
"""
import csv
import os
from pathlib import Path
import statistics
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[2]
path = "src/cryptanalysis/prime_root_trace.rs"
frozen = os.environ.get("ROOTFIND_BASELINE_REF",
    "35559c0c6eaff1526f21ae592d6717873f17a757")
baseline = subprocess.check_output(["git", "show", f"{frozen}:{path}"],
                                   cwd=repo, text=True)
candidate = (repo/path).read_text()
helper = r"""
#[doc(hidden)]
pub(crate) fn benchmark_verifier_power(
    base: &[u64], exponent: u64, modulus: &[u64], p: u64,
) -> Vec<u64> {
    vpower(base, exponent, modulus, p).expect("valid quotient modulus")
}
"""
cargo = """[package]
name = "round112-source-exact"
version = "0.1.0"
edition = "2021"
[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
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
use cryptanalysis::prime_root_trace::MAX_PRIME;
use std::hint::black_box;
use std::time::Instant;

fn hash(mut h:u64, xs:&[u64]) -> u64 {
    h=h.wrapping_mul(131).wrapping_add(xs.len() as u64);
    for &v in xs { h=h.wrapping_mul(33).wrapping_add(v); }
    h
}
fn main() {
    println!("prime,case,ns,checksum,completed");
    for p in [MAX_PRIME,1_000_000_007u64] {
        for degree in [4usize,5,6,8] {
            for shape in 0..3usize {
                for exponent_kind in 0..3usize {
                    if p != MAX_PRIME && (degree != 8 || shape != 0) {continue;}
                    if degree == 4 && shape != 0 {continue;}
                    if exponent_kind == 2 && (shape != 0 || degree != 8) {continue;}
                    let exponent = match exponent_kind {
                        0 => p, 1 => (p-1)/2, _ => 175_921,
                    };
                    let mut inputs=Vec::new();
                    for i in 0..24u64 {
                        let mut m=(0..degree).map(|j|
                            (17+i*193+(j as u64)*59+((i*j as u64)%19))%p
                        ).collect::<Vec<u64>>();
                        let mut a=(0..degree).map(|j|
                            (29+i*277+j as u64*137)%p
                        ).collect::<Vec<u64>>();
                        if shape == 1 { // sparse unit binomial and sparse base
                            for j in 1..degree {m[j]=0;a[j]=0;}
                            m[0]=if i%2==0 {1} else {p-1};
                        }
                        let lead=if shape==2 {2} else {1};
                        m.push(lead);
                        if i==0 {a.clear();}
                        inputs.push((a,m));
                    }
                    let mut h=0u64;
                    let mut count=0u64;
                    let start=Instant::now();
                    for _ in 0..9 {
                        for (a,m) in &inputs {
                            let result=cryptanalysis::prime_root_trace::benchmark_verifier_power(
                                black_box(a),black_box(exponent),black_box(m),black_box(p),
                            );
                            h=hash(h,&result);count+=1;
                            black_box(result);
                        }
                    }
                    println!("{p},d{degree}_shape{shape}_e{exponent_kind},{:.3},{h},{count}",
                        start.elapsed().as_nanos() as f64 / count as f64);
                }
            }
        }
    }
}
"""
try:
    cpu=min(os.sched_getaffinity(0))
    os.sched_setaffinity(0,{cpu})
    print("pinned CPU:",cpu,flush=True)
except (OSError,AttributeError) as e:
    print("CPU affinity unavailable:",e,flush=True)

with tempfile.TemporaryDirectory(prefix="rootfind112-") as scratch:
    binaries={}
    for label,source in [("before",baseline),("after",candidate)]:
        d=Path(scratch)/label
        (d/"src/cryptanalysis").mkdir(parents=True)
        (d/"Cargo.toml").write_text(cargo)
        (d/"src/main.rs").write_text(main)
        (d/"src/cryptanalysis/prime_root_trace.rs").write_text(source+"\n"+helper)
        (d/"src/cryptanalysis/prime_root_bounded.rs").write_text(
            "// isolated bench stub of unrelated bounded API\n")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=d,check=True)
        if label=="after":
            subprocess.run(["cargo","test","--release","--quiet"],cwd=d,check=True)
        binaries[label]=d/"target/release/round112-source-exact"
    trials=int(os.environ.get("ROOTFIND_AB_TRIALS","13"))
    rows=[]
    for trial in range(trials):
        order=["before","after"] if trial%2==0 else ["after","before"]
        result={}
        for label in order:
            output=subprocess.check_output([str(binaries[label])],text=True)
            result[label]={(r["prime"],r["case"]):r for r in csv.DictReader(output.splitlines())}
        assert result["before"].keys()==result["after"].keys()
        for key in result["before"]:
            a=result["before"][key]
            b=result["after"][key]
            assert (a["checksum"],a["completed"])==(b["checksum"],b["completed"]),key
            rows.append(dict(trial=trial,prime=key[0],case=key[1],
                             before_ns=a["ns"],after_ns=b["ns"]))
    dest=repo/"research/amm_digit_perf_20261118/round112_rust_ab.csv"
    with dest.open("w",newline="") as f:
        out=csv.DictWriter(f,fieldnames=list(rows[0]))
        out.writeheader();out.writerows(rows)
    print("prime,case,before_ns,after_ns,speedup")
    for key in sorted({(r["prime"],r["case"]) for r in rows}):
        pairs=[r for r in rows if (r["prime"],r["case"])==key]
        b=statistics.median(float(r["before_ns"]) for r in pairs)
        a=statistics.median(float(r["after_ns"]) for r in pairs)
        print(f"{key[0]},{key[1]},{b:.3f},{a:.3f},{b/a:.4f}")
    print("PASS",len(rows),"source-exact checksum-matched A/B comparisons")
