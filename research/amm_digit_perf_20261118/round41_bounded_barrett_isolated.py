#!/usr/bin/env python3
"""Round 41: compile the exact production bounded rootfinder twice, comparing
the cached 32-bit Barrett reductions to the immediate pre-round41 implementation.
The standalone crate imports the actual Rust module, not a C++ model.
"""
from pathlib import Path
import subprocess
import tempfile

src = Path("src/cryptanalysis/prime_root_bounded.rs").read_text()
after = src.split("\n#[cfg(test)]\n", 1)[0]

def once(s, old, new):
    n = s.count(old)
    if n != 1:
        raise RuntimeError(f"expected one baseline replacement, got {n}: {old[:80]!r}")
    return s.replace(old, new)

before = after
before = once(before,
    "    // Cached floor(2^64/p) for all canonical 32-bit field products.\n    barrett32: u64,\n", "")
before = once(before, "            barrett32: 0,\n", "")
start = "    // The quotient estimate floor(x * floor(2^64/p) / 2^64)"
end = "    fn clock(&self)"
if before.count(start) != 1:
    raise RuntimeError("missing reciprocal32 predecessor")
a, b = before.index(start), before.index(end, before.index(start))
before = before[:a] + before[b:]
before = once(before,
    """            // Avoid runtime division: the precomputed reciprocal yields
            // q=floor(ab/p) or q=floor(ab/p)-1. A single correction suffices.
            let product = a * b;
            let reciprocal = self.gate.reciprocal32(p);
            let q = ((u128::from(product) * u128::from(reciprocal)) >> 64) as u64;
            let remainder = product - q * p;
            if remainder >= p { remainder - p } else { remainder }
""",
    """            // (p-1)^2 fits in u64, including p=2^32-5.
            (a * b) % p
""")
before = once(before,
    """            // Independent verifier reduction, with the same proven
            // one-correction bound but a distinct arithmetic expression.
            let product = x * y;
            let reciprocal = self.gate.reciprocal32(p);
            let quotient = ((u128::from(reciprocal) * u128::from(product)) >> 64) as u64;
            let residual = product - quotient * p;
            if residual < p { residual } else { residual - p }
""",
    """            (x * y) % p
""")

header = """#![allow(dead_code)]
mod cryptanalysis {
 pub mod fp_backend {
  pub struct Fp64Arithmetic {pub p:u64}
  impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
 }
 pub mod prime_root_trace {
  pub mod after {
"""
middle = "\n  } pub mod before {\n"
tail = "\n  }\n }\n}\n"
kernel = r"""
pub fn round41_mul_kernel(p:u64,pairs:&[(u64,u64)],replay_path:bool)->u64 {
    let field=Fp64Arithmetic{p};
    let deadline=Instant::now()+std::time::Duration::from_secs(120);
    let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
        bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8};
    let mut digest=0u64;
    if replay_path {
        let mut replay=Replay::new(&field,&[1],42,limits,limits,deadline).unwrap();
        for &(x,y) in pairs {
            digest=digest.wrapping_add(replay.times(x,y).unwrap());
        }
    } else {
        let mut observer=|_:&BoundedRootEvent|Ok(());
        let mut producer=Producer{
            p,gate:Gate::new(limits,deadline),observer:&mut observer,
            unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
            roots:[0;8],root_count:0
        };
        for &(x,y) in pairs {
            digest=digest.wrapping_add(producer.mul(x,y).unwrap());
        }
    }
    digest
}
"""

checks = r"""
#[cfg(test)]
mod round41 {
    use super::cryptanalysis::{
        fp_backend::Fp64Arithmetic,
        prime_root_trace::{after, before},
    };
    use std::{hint::black_box, time::{Duration,Instant}};

    fn polynomial(p:u64,degree:usize)->Vec<u64> {
        let mut a=vec![1u64];
        for k in 1..=degree {
            let mut b=vec![0u64;a.len()+1];
            for (i,&c) in a.iter().enumerate() {
                let v=(c as u128*k as u128%p as u128) as u64;
                b[i]=((b[i] as u128+p as u128-v as u128)%p as u128) as u64;
                b[i+1]=(b[i+1]+c)%p;
            }
            a=b;
        }
        a
    }
    fn limits_after()->after::RootBudget {
        after::RootBudget{work:after::MAX_WORK,field_work:after::MAX_FIELD_WORK,
            bytes:after::STORAGE_BYTES,events:after::MAX_EVENTS,
            split_attempts:1000,degree:8}
    }
    fn limits_before()->before::RootBudget {
        before::RootBudget{work:before::MAX_WORK,field_work:before::MAX_FIELD_WORK,
            bytes:before::STORAGE_BYTES,events:before::MAX_EVENTS,
            split_attempts:1000,degree:8}
    }
    fn run_after(p:u64,f:&[u64],seed:u64,compare:bool)->(String,u64) {
        let field=Fp64Arithmetic{p};
        let deadline=Instant::now()+Duration::from_secs(120);
        let lim=limits_after();
        let mut ev=Vec::new();
        let report=after::solve_bounded(&field,black_box(f),seed,lim,deadline,
            &mut |e|{ev.push(*e);Ok(())});
        assert!(report.root_set_complete()||report.all_field_elements(),
                "after: {:?}",report.stop);
        let mut replay=after::Replay::new(&field,f,seed,lim,lim,deadline).unwrap();
        for e in &ev {replay.push(e).unwrap()}
        let checked=replay.finish(&report).unwrap();
        (if compare {format!("{:?} {:?} {:?}",report,ev,checked)} else {String::new()},
            black_box(report.roots[0]+u64::from(report.root_count)+ev.len() as u64))
    }
    fn run_before(p:u64,f:&[u64],seed:u64,compare:bool)->(String,u64) {
        let field=Fp64Arithmetic{p};
        let deadline=Instant::now()+Duration::from_secs(120);
        let lim=limits_before();
        let mut ev=Vec::new();
        let report=before::solve_bounded(&field,black_box(f),seed,lim,deadline,
            &mut |e|{ev.push(*e);Ok(())});
        assert!(report.root_set_complete()||report.all_field_elements(),
                "before: {:?}",report.stop);
        let mut replay=before::Replay::new(&field,f,seed,lim,lim,deadline).unwrap();
        for e in &ev {replay.push(e).unwrap()}
        let checked=replay.finish(&report).unwrap();
        (if compare {format!("{:?} {:?} {:?}",report,ev,checked)} else {String::new()},
            black_box(report.roots[0]+u64::from(report.root_count)+ev.len() as u64))
    }
    #[test]
    fn same_complete_root_report_events_and_replay() {
        let mut n=0usize;
        for p in [17u64,487,1_000_000_007,1_000_000_009,4_294_967_291,(1u64<<61)-1] {
            for degree in [1usize,2,3,4,6,8] {
                let f=polynomial(p,degree);
                for seed in [1u64,42,0x5291ae77] {
                    let a=run_after(p,&f,seed,true);
                    let b=run_before(p,&f,seed,true);
                    assert_eq!(a,b,"p={p} degree={degree} seed={seed}");
                    n+=1;
                }
            }
        }
        println!("ROUND41_EQ whole_solver_and_replay_cases={n}");
    }
    fn time(p:u64,f:&[u64],is_old:bool,reps:usize)->f64 {
        let begin=Instant::now();
        let mut digest=0;
        for _ in 0..reps {
            digest+=if is_old {run_before(p,f,42,false).1} else {run_after(p,f,42,false).1};
        }
        black_box(digest);
        begin.elapsed().as_nanos() as f64/reps as f64
    }

    #[test]
    #[ignore="Compile actual producer/replay scalar operations and compare versions"]
    fn bench_producer_replay_kernels() {
        let mut seed=0x903e_e50e_2026_0041u64;
        for p in [487u64,1_000_000_007,1_000_000_009,4_294_967_291,(1u64<<61)-1] {
            let mut pairs=Vec::new();
            for _ in 0..16384 {
                seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a=seed%p;
                seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                pairs.push((a,seed%p));
            }
            for replay_path in [false,true] {
                let mut old=Vec::new(); let mut new=Vec::new();
                for trial in 0..13 {
                    for after_first in [trial%2==1,trial%2==0] {
                        let begin=Instant::now();
                        let digest=if after_first {
                            after::round41_mul_kernel(p,black_box(&pairs),replay_path)
                        } else {
                            before::round41_mul_kernel(p,black_box(&pairs),replay_path)
                        };
                        black_box(digest);
                        let ns=begin.elapsed().as_nanos() as f64 / pairs.len() as f64;
                        if after_first {new.push(ns)} else {old.push(ns)}
                    }
                }
                old.sort_by(f64::total_cmp);
                new.sort_by(f64::total_cmp);
                let name=if replay_path {"replay"}else{"producer"};
                println!("ROUND41_KERNEL kind={name} p={p} before_ns={:.3} after_ns={:.3} speedup={:.4}x",
                    old[6],new[6],old[6]/new[6]);
            }
        }
    }

    #[test]
    #[ignore="Compare full compiled Rust producer/replay against exact preceding version"]
    fn bench_complete_solver() {
        for p in [487u64,1_000_000_007,(1u64<<61)-1] {
            for degree in [3usize,6,8] {
                let f=polynomial(p,degree);
                let mut old=Vec::new();let mut new=Vec::new();
                for i in 0..13 {
                    if i%2==0 {
                        old.push(time(p,&f,true,3));
                        new.push(time(p,&f,false,3));
                    } else {
                        new.push(time(p,&f,false,3));
                        old.push(time(p,&f,true,3));
                    }
                }
                old.sort_by(f64::total_cmp);
                new.sort_by(f64::total_cmp);
                println!("ROUND41_WHOLE p={p} degree={degree} before_ns={:.1} after_ns={:.1} speedup={:.4}x",
                    old[6],new[6],old[6]/new[6]);
            }
        }
    }
}
"""

with tempfile.TemporaryDirectory() as folder:
    path = Path(folder) / "round41.rs"
    binary = Path(folder) / "round41"
    path.write_text(header + after + kernel + middle + before + kernel + tail + checks)
    subprocess.run(["rustc", "--edition=2021", "-O", "--test",
                    str(path), "-o", str(binary)], check=True)
    subprocess.run([str(binary), "same_complete_root_report_events_and_replay",
                    "--nocapture"], check=True)
    subprocess.run([str(binary), "bench_complete_solver",
                    "--ignored", "--nocapture"], check=True)
    subprocess.run([str(binary), "bench_producer_replay_kernels",
                    "--ignored", "--nocapture"], check=True)

    # Compile the unmodified production #[cfg(test)] module too. This
    # validates every pair against the u128 oracle, including 2^32-5.
    full_header = """#![allow(dead_code)]
mod cryptanalysis {
 pub mod fp_backend {
  pub struct Fp64Arithmetic {pub p:u64}
  pub struct Admission {pub field:Fp64Arithmetic}
  impl Fp64Arithmetic {
   pub fn p(&self)->u64{self.p}
   pub fn try_new<F:FnMut(&())->Result<(),()>>(
      p:u64,_work:u64,_bytes:u64,_deadline:std::time::Instant,_callback:&mut F
   )->Result<Admission,()> {Ok(Admission{field:Self{p}})}
  }
 }
 pub mod prime_root_trace { pub mod after {
"""
    full_tail = "\n } } }\n"
    full_path = Path(folder) / "round41_full_tests.rs"
    full_binary = Path(folder) / "round41_full_tests"
    full_path.write_text(full_header + src + full_tail)
    subprocess.run(["rustc", "--edition=2021", "-O", "--test", str(full_path),
                    "-o", str(full_binary)], check=True)
    subprocess.run([str(full_binary),
                    "round41_cached_barrett_matches_reference_and_replay",
                    "--nocapture"], check=True)
