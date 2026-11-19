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
# Reconstruct the pre-round31 full module directly from the frozen original
# method bodies; this permits end-to-end compiled-Rust solve_bounded A/B with
# identical surrounding code, event accounting and field setup stub.
def method(code, name):
    marker = "    fn " + name + "("
    at = code.index(marker)
    open_brace = code.index("{", at)
    depth = 0
    for i in range(open_brace, len(code)):
        if code[i] == "{":
            depth += 1
        elif code[i] == "}":
            depth -= 1
            if depth == 0:
                return code[at:i+1]
    raise ValueError("unterminated method " + name)

old_production = production
for current, frozen in [
    ("scalar", "before_scalar"),
    ("add", "before_add"),
    ("sub", "before_sub"),
    ("mul", "before_mul"),
    ("times", "before_times"),
    ("plus", "before_plus"),
    ("minus", "before_minus"),
]:
    current_code = method(old_production, current)
    frozen_code = method(baseline, frozen)
    frozen_code = frozen_code.replace("fn " + frozen, "fn " + current, 1)
    frozen_code = frozen_code.replace("self.gate.before_scalar()", "self.gate.scalar()")
    old_production = old_production.replace(current_code, frozen_code, 1)

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
    // Full production root extraction, not a microbench or arithmetic mirror.
    // Both variants are assembled from the exact production module, with only
    // the seven changed method bodies replaced by frozen pre-round31 code.
    #[test]
    fn whole_rootfinder_before_after_exact() {
        use crate::cryptanalysis::prime_root_trace::bounded_before as old;
        for p in [17u64,487,1_000_000_007,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            for f in [vec![p-1,1],vec![2,p-3,1]] {
                let mut actual_events=Vec::new();
                let actual=solve_bounded(&field,&f,42,budget(),deadline(),&mut |e| {
                    actual_events.push(format!("{e:?}"));Ok(())
                });
                let old_limits=old::RootBudget {
                    work:old::MAX_WORK,field_work:old::MAX_FIELD_WORK,
                    bytes:old::STORAGE_BYTES,events:old::MAX_EVENTS,
                    split_attempts:1000,degree:8
                };
                let mut old_events=Vec::new();
                let expected=old::solve_bounded(&field,&f,42,old_limits,deadline(),&mut |e| {
                    old_events.push(format!("{e:?}"));Ok(())
                });
                assert_eq!(format!("{:?}",actual.stop),format!("{:?}",expected.stop));
                assert_eq!(actual.roots,expected.roots);
                assert_eq!(actual.root_count,expected.root_count);
                assert_eq!(actual.metrics.work,expected.metrics.work);
                assert_eq!(actual.metrics.field_work,expected.metrics.field_work);
                assert_eq!(actual.metrics.splits,expected.metrics.splits);
                assert_eq!(actual.metrics.attempted_events,expected.metrics.attempted_events);
                assert_eq!(actual_events,old_events);
                assert_eq!(actual.stop,Stop::Complete,"p={p} f={f:?}");
            }
        }
    }

    fn whole_trial(p:u64,degree:usize,fast:bool,repeats:usize)->f64 {
        use crate::cryptanalysis::prime_root_trace::bounded_before as old;
        let field=Fp64Arithmetic{p};
        let coeffs=if degree==1 {vec![p-1,1]} else {vec![2,p-3,1]};
        let old_limits=old::RootBudget {
            work:old::MAX_WORK,field_work:old::MAX_FIELD_WORK,
            bytes:old::STORAGE_BYTES,events:old::MAX_EVENTS,
            split_attempts:1000,degree:8
        };
        let mut checksum=0u64;
        let start=Instant::now();
        for _ in 0..repeats {
            if fast {
                let result=solve_bounded(&field,black_box(&coeffs),42,
                    budget(),deadline(),&mut |_|Ok(()));
                checksum=checksum.wrapping_add(result.metrics.field_work)
                    .wrapping_add(result.roots[0]);
                assert_eq!(result.stop,Stop::Complete);
            } else {
                let result=old::solve_bounded(&field,black_box(&coeffs),42,
                    old_limits,deadline(),&mut |_|Ok(()));
                checksum=checksum.wrapping_add(result.metrics.field_work)
                    .wrapping_add(result.roots[0]);
                assert_eq!(result.stop,old::Stop::Complete);
            }
        }
        black_box(checksum);
        start.elapsed().as_nanos() as f64/repeats as f64
    }

    #[test]
    #[ignore="paired actual compiled Rust whole solve_bounded root extraction"]
    fn whole_rootfinder_paired_timings() {
        for p in [487u64,1_000_000_007,(1u64<<61)-1] {
            for degree in [1usize,2] {
                let (mut before,mut after)=(Vec::new(),Vec::new());
                for i in 0..13 {
                    if i%2==0 {
                        before.push(whole_trial(p,degree,false,64));
                        after.push(whole_trial(p,degree,true,64));
                    } else {
                        after.push(whole_trial(p,degree,true,64));
                        before.push(whole_trial(p,degree,false,64));
                    }
                }
                before.sort_by(f64::total_cmp);
                after.sort_by(f64::total_cmp);
                println!("WHOLE_RUST p={p} degree={degree}: {:.3} -> {:.3} ns ({:.3}x)",
                    before[6],after[6],before[6]/after[6]);
            }
        }
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
tail = "\n" + baseline + "\n" + isolated + "\n} pub mod bounded_before {\n" + old_production + "\n}}}\n"
# The production comment at file top uses //!, legal as module-level inner docs.
rust = header + production + tail
with tempfile.TemporaryDirectory() as tmp:
    file=Path(tmp)/"round31.rs"
    out=Path(tmp)/"round31"
    file.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(file),"-o",str(out)],check=True)
    subprocess.run([str(out),"exact_comparisons","--nocapture"],check=True)
    subprocess.run([str(out),"full_bounded_solver_and_replay_smoke","--nocapture"],check=True)
    subprocess.run([str(out),"whole_rootfinder_before_after_exact","--nocapture"],check=True)
    subprocess.run([str(out),"whole_rootfinder_paired_timings","--ignored","--nocapture"],check=True)
    subprocess.run([str(out),"paired_rust_timings","--ignored","--nocapture"],check=True)
