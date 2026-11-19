#!/usr/bin/env python3
"""Compile the *actual* bounded root producer/replay with frozen pre-round31 methods.

This standalone Rust test harness stubs only the field setup's p() accessor,
avoiding unrelated full-library dependency and workflow failures.
"""
from pathlib import Path
import subprocess
import tempfile

production = Path("src/cryptanalysis/prime_root_bounded.rs").read_text()
production = production.split("\n#[cfg(test)]\n", 1)[0]
baseline = Path("research/amm_digit_perf_20261118/round31_bounded_baseline.rs").read_text()
isolated = r'''
#[cfg(test)]
mod round31 {
    use super::*;
    use std::hint::black_box;
    use std::time::{Instant, Duration};

    fn budget() -> RootBudget {
        RootBudget {work: MAX_WORK, field_work: MAX_FIELD_WORK,
            bytes: STORAGE_BYTES, events: MAX_EVENTS, split_attempts: 1000, degree: 8}
    }
    fn deadline() -> Instant { Instant::now() + Duration::from_secs(60) }
    fn inputs(p:u64)->Vec<(u64,u64)> {
        let mut s=0x2026_1119_3131_0101u64;
        let mut out=Vec::new();
        for i in 0..1536 {
            s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let mut a=(s^(s>>29))%p;
            s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let mut b=(s^(s>>29))%p;
            match i % 43 {
                0=>{a=p-1;b=p-1;},
                1=>{a=0;b=p-1;},
                2=>{a=p-1;b=0;},
                3=>{a=0;b=0;},
                _=>{}
            }
            out.push((a,b));
        }
        out
    }
    fn run_op(which:usize,fast:bool, producer:&mut Producer<'_>,
              replay:&mut Replay,a:u64,b:u64)->u64 {
        match (which,fast) {
            (0,false)=>producer.before_add(a,b).unwrap(),
            (0,true)=>producer.add(a,b).unwrap(),
            (1,false)=>producer.before_sub(a,b).unwrap(),
            (1,true)=>producer.sub(a,b).unwrap(),
            (2,false)=>producer.before_mul(a,b).unwrap(),
            (2,true)=>producer.mul(a,b).unwrap(),
            (3,false)=>replay.before_plus(a,b).unwrap(),
            (3,true)=>replay.plus(a,b).unwrap(),
            (4,false)=>replay.before_minus(a,b).unwrap(),
            (4,true)=>replay.minus(a,b).unwrap(),
            (5,false)=>replay.before_times(a,b).unwrap(),
            (5,true)=>replay.times(a,b).unwrap(),
            _=>unreachable!()
        }
    }
    #[test]
    fn exact_comparisons() {
        let mut count=0usize;
        for p in [3u64,7,17,487,17497,1_000_000_007,
                  4_294_967_291,140_737_488_355_213,(1u64<<61)-1] {
            let field=Fp64Arithmetic { p };
            let mut observe=|_:&BoundedRootEvent| Ok(());
            let mut producer=Producer { p, gate:Gate::new(budget(),deadline()),
                observer:&mut observe, unacknowledged:None,
                pending:[Poly17::ZERO;8], pending_len:0,roots:[0;8],root_count:0 };
            producer.gate.admit().unwrap();
            let mut replay=Replay::new(&field,&[1,1],42,budget(),budget(),deadline()).unwrap();
            for (a,b) in inputs(p) {
                let wide_add=((a as u128+b as u128)%p as u128)as u64;
                let wide_sub=((a as u128+p as u128-b as u128)%p as u128)as u64;
                let wide_mul=((a as u128*b as u128)%p as u128)as u64;
                for op in 0..6 {
                    let ref_value=match op {0|3=>wide_add,1|4=>wide_sub,_=>wide_mul};
                    assert_eq!(run_op(op,false,&mut producer,&mut replay,a,b),ref_value,
                        "before p={p} op={op} a={a} b={b}");
                    assert_eq!(run_op(op,true,&mut producer,&mut replay,a,b),ref_value,
                        "after p={p} op={op} a={a} b={b}");
                    count+=2;
                }
            }
        }
        println!("round31 actual Rust producer/replay comparisons={count}");
        assert_eq!(count,9*1536*12);
    }

    #[test]
    fn full_bounded_solver_and_replay_smoke() {
        for p in [17u64,1_000_000_007,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            // f(x)=x-1 is monic and has exactly one root over every Fp.
            let input=[p-1,1];
            let limits=budget();
            let mut stream=Vec::new();
            let report=solve_bounded(&field,&input,42,limits,deadline(),&mut |e| {
                stream.push(*e);Ok(())
            });
            assert_eq!(report.stop,Stop::Complete,"p={p}");
            assert_eq!(&report.roots[..1],&[1]);
            let mut replay=Replay::new(&field,&input,42,limits,budget(),deadline()).unwrap();
            for event in &stream { replay.push(event).unwrap(); }
            assert!(replay.finish(&report).unwrap().finite_complete);
        }
    }

    fn sample(which:usize,fast:bool,p:u64,cases:&[(u64,u64)],repeats:usize)->f64 {
        let field=Fp64Arithmetic{p};
        let mut observe=|_:&BoundedRootEvent|Ok(());
        let mut producer=Producer{p,gate:Gate::new(budget(),deadline()),
            observer:&mut observe,unacknowledged:None,
            pending:[Poly17::ZERO;8],pending_len:0,roots:[0;8],root_count:0};
        producer.gate.admit().unwrap();
        let mut replay=Replay::new(&field,&[1,1],42,budget(),budget(),deadline()).unwrap();
        let mut checksum=0u64;
        let start=Instant::now();
        for _ in 0..repeats {
            for &(a,b) in cases {
                checksum=checksum.wrapping_add(run_op(which,fast,&mut producer,
                    &mut replay,black_box(a),black_box(b)));
            }
        }
        black_box(checksum);
        start.elapsed().as_nanos() as f64/(repeats*cases.len()) as f64
    }
    #[test]
    #[ignore="13 paired before/after native Rust comparisons"]
    fn paired_rust_timings() {
        let names=["producer_add","producer_sub","producer_mul",
                   "replay_plus","replay_minus","replay_times"];
        for p in [487u64,1_000_000_007,4_294_967_291,
                  140_737_488_355_213,(1u64<<61)-1] {
            let data=inputs(p);
            for (which,name) in names.iter().enumerate() {
                let (mut old,mut new)=(Vec::new(),Vec::new());
                for iteration in 0..13 {
                    if iteration%2==0 {
                        old.push(sample(which,false,p,&data,70));
                        new.push(sample(which,true,p,&data,70));
                    } else {
                        new.push(sample(which,true,p,&data,70));
                        old.push(sample(which,false,p,&data,70));
                    }
                }
                old.sort_by(f64::total_cmp);
                new.sort_by(f64::total_cmp);
                println!("RUST p={p} {name}: {:.3} -> {:.3} ns/op ({:.3}x)",
                    old[6],new[6],old[6]/new[6]);
            }
        }
    }
}
'''
header = """
#![allow(dead_code)]
mod cryptanalysis {
    pub mod fp_backend {
        pub struct Fp64Arithmetic { pub p: u64 }
        impl Fp64Arithmetic { pub fn p(&self) -> u64 { self.p } }
    }
    pub mod prime_root_trace {
        pub mod bounded {
"""
tail = "\n" + baseline + "\n" + isolated + "\n}}}\n"
# The production comment at file top uses //!, legal as module-level inner docs.
rust = header + production + tail
with tempfile.TemporaryDirectory() as tmp:
    file=Path(tmp)/"round31.rs"
    out=Path(tmp)/"round31"
    file.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(file),"-o",str(out)],check=True)
    subprocess.run([str(out),"exact_comparisons","--nocapture"],check=True)
    subprocess.run([str(out),"full_bounded_solver_and_replay_smoke","--nocapture"],check=True)
    subprocess.run([str(out),"paired_rust_timings","--ignored","--nocapture"],check=True)
