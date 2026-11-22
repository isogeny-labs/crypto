#!/usr/bin/env python3
"""Source-exact production Rust A/B for p61 high-degree quotient powering.

Reads frozen Round112 and this branch's unmodified prime_root_trace.rs.
Adds identical private benchmark accessors to both temporary crates.
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
    "d4598fdb83ba6fa64bcc65ac1faf6ee67bc40b34")
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
#[doc(hidden)]
pub(crate) fn benchmark_verifier_inverse(a: u64, p: u64) -> u64 {
    vinv(a, p)
}
#[doc(hidden)]
pub(crate) fn benchmark_verifier_quadratic_mul(
    a: (u64,u64), b: (u64,u64), c: u64, d: u64, p: u64,
) -> (u64,u64) {
    quadratic_mul(a, b, c, d, p)
}
"""
cargo = """[package]
name = "round113-source-exact"
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
    for p in [MAX_PRIME,1_000_000_007u64,65_521u64] {
        for degree in [2usize,4,5,6,8] {
            for shape in 0..3usize {
                for exponent_kind in 0..3usize {
                    if p != MAX_PRIME && degree != 2 && (degree != 8 || shape != 0) {continue;}
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
                    for _ in 0..(if degree == 2 { 150 } else { 9 }) {
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

    // Independently measure the Karatsuba multiplier over all three fields,
    // including the untouched 61-bit control and zero/unit moduli.
    for p in [MAX_PRIME, 1_000_000_007u64, 65_521u64] {
        for shape in 0..3u64 {
            let mut inputs=Vec::new();
            for i in 0..64u64 {
                let a=((123+i*97)%p, (257+i*47)%p);
                let b=((541+i*137)%p,(811+i*109)%p);
                let (c,d)=match shape {
                    1=>(0,0),2=>(1,0),_=>( (37+i*17)%p, (19+i*23)%p )
                };
                inputs.push((a,b,c,d));
            }
            let mut h=0u64;
            let mut count=0u64;
            let start=Instant::now();
            for _ in 0..4_096 {
                for &(a,b,c,d) in &inputs {
                    let (x,y)=cryptanalysis::prime_root_trace::benchmark_verifier_quadratic_mul(
                        black_box(a),black_box(b),black_box(c),black_box(d),black_box(p),
                    );
                    h=h.wrapping_mul(131).wrapping_add(x);
                    h=h.wrapping_mul(131).wrapping_add(y);
                    count+=1;
                    black_box((x,y));
                }
            }
            println!("{p},quadratic_mul_shape{shape},{:.3},{h},{count}",
                     start.elapsed().as_nanos() as f64/count as f64);
        }
        for special in [true,false] {
            let inputs:Vec<u64>=if special {
                vec![1,2,p-1,p-2]
            } else {
                (0..64u64).map(|i| 3+i*79).collect()
            };
            let mut h=0u64;
            let mut count=0u64;
            let start=Instant::now();
            for _ in 0..4_096 {
                for &x in &inputs {
                    let y=cryptanalysis::prime_root_trace::benchmark_verifier_inverse(
                        black_box(x),black_box(p));
                    h=h.wrapping_mul(131).wrapping_add(y);
                    count+=1;
                    black_box(y);
                }
            }
            println!("{p},inverse_{},{:.3},{h},{count}",
                     if special {"special"} else {"general"},
                     start.elapsed().as_nanos() as f64/count as f64);
        }
    }

    // Complete root extraction with independent certificate verification.
    // The polynomial (X-r1)(X-r2) is split over each chosen prime.
    for p in [MAX_PRIME, 1_000_000_007u64, 65_521u64] {
        let mut inputs=Vec::new();
        for i in 0..8u64 {
            let r1=(3+i*7)%p;
            let r2=(19+i*13)%p;
            let product=((r1 as u128*r2 as u128)%(p as u128)) as u64;
            let sum=(r1+r2)%p;
            inputs.push(vec![product,if sum==0 {0} else {p-sum},1]);
        }
        let limits=cryptanalysis::prime_root_trace::RootLimits {
            field_work: cryptanalysis::prime_root_trace::MAX_WORK,
            split_attempts: 1000,
            events: cryptanalysis::prime_root_trace::MAX_EVENTS,
            degree: cryptanalysis::prime_root_trace::MAX_DEGREE,
        };
        let mut h=0u64;
        let mut count=0u64;
        let start=Instant::now();
        for _ in 0..4 {
            for f in &inputs {
                let report=cryptanalysis::prime_root_trace::solve(
                    black_box(f),black_box(p),7,limits.clone(),&mut |_| Ok(())
                ).expect("root solve");
                assert!(report.complete(), "quadratic root solve was censored");
                cryptanalysis::prime_root_trace::verify(&report).expect("independent verification");
                h=hash(h,&report.roots);
                h=h.wrapping_mul(131).wrapping_add(report.events.len() as u64);
                count+=1;
            }
        }
        println!("{p},complete_quadratic_solve_verify,{:.3},{h},{count}",
                 start.elapsed().as_nanos() as f64 / count as f64);
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
        # The benchmark entry point selects the new production multiplier only
        # for the candidate. The frozen baseline keeps the old implementation.
        accessor = helper if label == "before" else helper.replace(
            "    quadratic_mul(a, b, c, d, p)",
            "    if p <= u32::MAX as u64 { quadratic_mul_karatsuba(a, b, c, d, p) } else { quadratic_mul(a, b, c, d, p) }",
        )
        (d/"src/cryptanalysis/prime_root_trace.rs").write_text(source+"\n"+accessor)
        (d/"src/cryptanalysis/prime_root_bounded.rs").write_text(
            "// isolated bench stub of unrelated bounded API\n")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=d,check=True)
        if label=="after":
            subprocess.run(["cargo","test","--release","--quiet"],cwd=d,check=True)
        binaries[label]=d/"target/release/round113-source-exact"
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
    dest=repo/"research/amm_digit_perf_20261118/round113_rust_ab.csv"
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
