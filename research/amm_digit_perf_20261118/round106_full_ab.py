#!/usr/bin/env python3
"""Round 106 source-exact Rust A/B for repeated nonmonic root-power reductions.

The two temporary crates each contain the unmodified full production source
(frozen pre-round106 and current candidate), plus the same private benchmark
accessor and minimal unrelated module stubs. Checksum-equal results are
required before reporting any timing speedup. Does not benchmark ECC2K.
"""
from pathlib import Path
import csv
import os
import statistics
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[2]
source_path = "src/cryptanalysis/prime_root_trace.rs"
baseline_ref = os.environ.get("ROOTFIND_BASELINE_REF", "759c1202aa66ca5cbca1e56b058ac4f7b650399c")
baseline = subprocess.check_output(["git", "show", f"{baseline_ref}:{source_path}"], cwd=repo, text=True)
candidate = (repo / source_path).read_text()
# The frozen source predates the nonmonic-specific method. Add only a thin
# benchmark alias so both snapshots invoke the same actual producer dispatch
# in their identical private accessor. No baseline arithmetic is modified.
baseline += """
impl Engine<'_> {
    fn pow_nonmonic(
        &mut self, base: &[u64], exponent: u64, modulus: &[u64],
    ) -> Work<Vec<u64>> {
        self.pow(base, exponent, modulus)
    }
}
"""

helper = r"""
#[doc(hidden)]
pub(crate) fn benchmark_engine_power(
    base: &[u64], exponent: u64, modulus: &[u64], p: u64,
) -> Vec<u64> {
    let mut sink = |event: &RootEvent| -> Result<(), String> {
        std::hint::black_box(event); Ok(())
    };
    let report = RootReport {
        schema: "prime-root-trace/v1".into(), p,
        polynomial: vec![], seed: 7,
        limits: RootLimits {field_work: MAX_WORK,
            split_attempts: MAX_EVENTS, events: MAX_EVENTS, degree: MAX_DEGREE},
        stop: RootStop::WorkLimit, counters: RootCounters::default(),
        events: vec![], unacknowledged: None, sink_error: None, roots: vec![],
    };
    let mut engine = Engine {report, sink: &mut sink, next_id: 0};
    let out = if modulus.last() != Some(&1) {
        engine.pow_nonmonic(base, exponent, modulus)
    } else {
        engine.pow(base, exponent, modulus)
    }.expect("power cap");
    std::hint::black_box(&engine.report.counters);
    out
}
"""
cargo = """[package]
name = "round106-source-exact"
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
use cryptanalysis::prime_root_trace::{
    solve,verify,RootEvent,RootLimits,MAX_WORK,MAX_EVENTS,MAX_DEGREE,MAX_PRIME,
};
use std::hint::black_box;
use std::time::Instant;

fn product(a:&[u64],b:&[u64],p:u64)->Vec<u64>{
    let mut r=vec![0;a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate(){
        for (j,&y) in b.iter().enumerate(){
            r[i+j]=((r[i+j] as u128+x as u128*y as u128)%p as u128) as u64;
        }
    }
    r
}
fn polynomial(p:u64,degree:usize,scale:u64,index:u64)->Vec<u64>{
    let mut r=vec![1];
    for i in 0..degree {
        let x=3+(index*13+i as u64*7)%103;
        r=product(&r,&[p-x,1],p);
    }
    r.into_iter().map(|x|((x as u128*scale as u128)%p as u128) as u64).collect()
}
fn checksum(mut h:u64,r:&[u64])->u64 {
    h=h.wrapping_mul(131).wrapping_add(r.len() as u64);
    for &x in r {h=h.wrapping_mul(33).wrapping_add(x);}
    h
}
fn main(){
    println!("prime,case,ns,checksum,completed");
    // Direct traced producer modular power, with identical input and sink.
    for p in [65_521u64,1_000_000_007,MAX_PRIME] {
        for degree in [2usize,4,8] {
            for scale in [1u64,2] {
                for exponent_kind in [0usize,1] {
                    let e=if exponent_kind==0 {p} else {(p-1)/2};
                    let mut inputs=Vec::new();
                    for i in 0..8u64 {
                        let m=polynomial(p,degree,scale,i);
                        let b=vec![(i*17+13)%p,1u64];
                        inputs.push((b,m));
                    }
                    let mut h=0u64;let mut n=0usize;
                    let start=Instant::now();
                    for _ in 0..3 {
                        for (b,m) in &inputs {
                            let r=cryptanalysis::prime_root_trace::benchmark_engine_power(
                                black_box(b),black_box(e),black_box(m),black_box(p));
                            h=checksum(h,&r);
                            black_box(r);
                            n+=1;
                        }
                    }
                    println!("{p},power_d{degree}_scale{scale}_e{exponent_kind},{:.3},{h},{n}",
                        start.elapsed().as_nanos() as f64/n as f64);
                }
            }
        }
    }
    // Completed root extraction and independent certificate verification.
    for p in [65_521u64,1_000_000_007,MAX_PRIME] {
        for degree in [2usize,4] {
            for scale in [1u64,2] {
                let mut h=0u64;let mut n=0usize;
                let start=Instant::now();
                for _ in 0..2 {
                    for i in 0..4u64 {
                        let f=polynomial(p,degree,scale,i);
                        let mut sink=|e:&RootEvent|->Result<(),String>{black_box(e);Ok(())};
                        let report=solve(black_box(&f),p,7+i,
                            RootLimits{field_work:MAX_WORK,split_attempts:1000,
                                events:MAX_EVENTS,degree:MAX_DEGREE},
                            &mut sink).unwrap();
                        assert!(report.complete(),"p={p} degree={degree} scale={scale} stop={:?}",report.stop);
                        verify(&report).unwrap();
                        assert_eq!(report.roots.len(),degree);
                        h=checksum(h,&report.roots);
                        black_box(report);n+=1;
                    }
                }
                println!("{p},solve_verify_d{degree}_scale{scale},{:.3},{h},{n}",
                    start.elapsed().as_nanos() as f64/n as f64);
            }
        }
    }
}
"""
try:
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    print("CPU affinity pinned:",cpu,flush=True)
except (OSError,AttributeError) as exc:
    print("CPU affinity unavailable:",exc,flush=True)
with tempfile.TemporaryDirectory(prefix="rootfind106-") as temp:
    root=Path(temp)
    binaries={}
    for label,source in [("before",baseline),("after",candidate)]:
        d=root/label
        (d/"src/cryptanalysis").mkdir(parents=True)
        (d/"Cargo.toml").write_text(cargo)
        (d/"src/main.rs").write_text(main)
        (d/"src/cryptanalysis/prime_root_trace.rs").write_text(source+"\n"+helper)
        (d/"src/cryptanalysis/prime_root_bounded.rs").write_text(
            "// unrelated bounded API stubbed only in isolated A/B harness\n")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=d,check=True)
        if label=="after":
            subprocess.run(["cargo","test","--release","--quiet"],cwd=d,check=True)
        binaries[label]=d/"target/release/round106-source-exact"
    rows=[]
    trials=int(os.environ.get("ROOTFIND_AB_TRIALS","13"))
    for trial in range(trials):
        order=["before","after"] if trial%2==0 else ["after","before"]
        result={}
        for label in order:
            out=subprocess.check_output([str(binaries[label])],text=True)
            result[label]={(r["prime"],r["case"]):r for r in csv.DictReader(out.splitlines())}
        assert result["before"].keys()==result["after"].keys()
        for key in result["before"]:
            a=result["before"][key];b=result["after"][key]
            assert (a["checksum"],a["completed"])==(b["checksum"],b["completed"]),key
            rows.append(dict(trial=trial,prime=key[0],case=key[1],
                before_ns=a["ns"],after_ns=b["ns"]))
    output=repo/os.environ.get("ROOTFIND_AB_OUTPUT",
            "research/amm_digit_perf_20261118/round106_full_results.csv")
    output.parent.mkdir(parents=True,exist_ok=True)
    with output.open("w",newline="") as fh:
        writer=csv.DictWriter(fh,fieldnames=list(rows[0]))
        writer.writeheader();writer.writerows(rows)
    print("prime,case,before_ns,after_ns,speedup")
    for key in sorted({(r["prime"],r["case"]) for r in rows}):
        samples=[r for r in rows if (r["prime"],r["case"])==key]
        a=statistics.median(float(r["before_ns"]) for r in samples)
        b=statistics.median(float(r["after_ns"]) for r in samples)
        print(f"{key[0]},{key[1]},{a:.3f},{b:.3f},{a/b:.4f}")
    print("PASS",len(rows),"paired source-exact checksum comparisons")
