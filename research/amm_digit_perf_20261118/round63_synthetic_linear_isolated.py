#!/usr/bin/env python3
"""Round 63. Compile exact production Rust before/after, verify four
synthetic-linear paths against the preceding branch implementation, and
measure kernel and full certified-root performance. No C++ mirror is used."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
baseline = source
for block in ["PRODUCER_SYNTHETIC_LINEAR", "PRODUCER_SYNTHETIC_REDUCE",
              "REPLAY_HORNER_REMAINDER", "REPLAY_HORNER_CANCEL"]:
    pat = rf"(?ms)^ *// R63_{block}_BEGIN\n.*?^ *// R63_{block}_END\n"
    baseline, count = re.subn(pat, "", baseline, count=1)
    assert count == 1, "missing one complete source marker pair " + block

head = """#![allow(dead_code)]
mod cryptanalysis {
 pub mod fp_backend {
  pub struct Fp64Arithmetic { pub p: u64 }
  impl Fp64Arithmetic { pub fn p(&self)->u64 { self.p } }
 }
 pub mod bounded { pub mod after {
"""
middle = "\n} pub mod before {\n"
tail = "\n} } }\n"

helper = r"""
pub fn round63_kernel(p: u64, degree: usize, c: u64, mode: u8,
                      seed: u64, repeats: usize, sparse: bool) -> (u64, u64) {
    use std::{hint::black_box, time::Duration};
    let limits = RootBudget { work: MAX_WORK, field_work: MAX_FIELD_WORK,
        bytes: STORAGE_BYTES, events: MAX_EVENTS, split_attempts: 1000,
        degree: 8 };
    let deadline = Instant::now() + Duration::from_secs(120);
    let field = super::super::fp_backend::Fp64Arithmetic { p };
    let mut observer = |_: &BoundedRootEvent| Ok(());
    let mut producer = Producer { p, gate: Gate::new(limits, deadline),
        observer: &mut observer, unacknowledged: None,
        pending: [Poly17::ZERO; 8], pending_len: 0, roots: [0; 8],
        root_count: 0 };
    producer.gate.admit().unwrap();
    let mut replay = Replay::new(&field, &[1, 1], 42, limits, limits,
                                 deadline).unwrap();
    let mut digest = 0u64;
    let mut cost = 0u64;
    let divisor = Poly17 { coefficients: {
            let mut arr = [0u64; 17]; arr[0] = c; arr[1] = 1; arr
        }, len: 2 };
    for iteration in 0..repeats {
        let mut input = Poly17::ZERO;
        input.len = (degree + 1) as u8;
        let mut state = black_box(seed + iteration as u64)
            .wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(p);
        for i in 0..=degree {
            state = state.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            input.coefficients[i] =
                if sparse && i % 3 == 0 { 0 } else { state % p };
        }
        input.coefficients[degree] = 1;
        let input = black_box(input);
        let modulus = black_box(divisor);
        let (q, r) = match mode {
            0 => producer.division(&input, &modulus).unwrap(),
            1 => { let mut r = input;
                producer.reduce_monic(&mut r, &modulus).unwrap();
                (Poly17::ZERO, r)
            },
            2 => { let r = replay.remainder(input, &modulus).unwrap();
                (Poly17::ZERO, r)
            },
            3 => { let mut r = input;
                replay.cancel_monic(&mut r, &modulus).unwrap();
                (Poly17::ZERO, r)
            },
            _ => panic!("invalid mode"),
        };
        let mut fingerprint = (q.len as u64).wrapping_add((r.len as u64) << 8);
        for k in 0..17 {
            fingerprint = fingerprint.wrapping_mul(37)
                .wrapping_add(q.coefficients[k]);
            fingerprint = fingerprint.wrapping_mul(37)
                .wrapping_add(r.coefficients[k]);
        }
        digest = digest.wrapping_mul(0x9e3779b9)
            .wrapping_add(black_box(fingerprint));
        let gate = if mode < 2 { &mut producer.gate } else { &mut replay.gate };
        cost += gate.metrics.field_work;
        gate.metrics.field_work = 0;
        gate.metrics.work = 0;
    }
    (digest, cost)
}

pub fn round63_whole(p: u64, degree: usize) ->
    (Stop, [u64; 8], u8, Vec<BoundedRootEvent>) {
    use std::time::Duration;
    let limits = RootBudget { work: MAX_WORK, field_work: MAX_FIELD_WORK,
        bytes: STORAGE_BYTES, events: MAX_EVENTS, split_attempts: 1000,
        degree: 8 };
    let field = super::super::fp_backend::Fp64Arithmetic { p };
    // f(x) = product of (x-r), with d distinct nonzero roots.
    let mut values = vec![1u64];
    for root in 1..=degree {
        let mut next = vec![0u64; values.len() + 1];
        let neg = p - root as u64;
        for (i, &v) in values.iter().enumerate() {
            next[i] = ((next[i] as u128 + v as u128 * neg as u128)
                       % p as u128) as u64;
            next[i + 1] = (next[i + 1] + v) % p;
        }
        values = next;
    }
    let mut events = Vec::new();
    let report = solve_bounded(&field, &values, 42, limits,
        Instant::now() + Duration::from_secs(120),
        &mut |e| { events.push(*e); Ok(()) });
    if report.stop == Stop::Complete {
        let mut replay = Replay::new(&field, &values, 42, limits, limits,
            Instant::now() + Duration::from_secs(120)).unwrap();
        for event in &events { replay.push(event).unwrap(); }
        assert!(replay.finish(&report).unwrap().finite_complete);
    }
    (report.stop, report.roots, report.root_count, events)
}
"""

tests = r"""
#[cfg(test)]
mod round63 {
    use super::cryptanalysis::bounded::{before, after};
    use std::{hint::black_box, time::Instant};

    fn median(mut samples: Vec<f64>) -> f64 {
        samples.sort_by(f64::total_cmp); samples[samples.len() / 2]
    }

    #[test]
    fn exact_source_equivalence() {
        let mut compared = 0usize;
        for p in [101u64, 1_000_000_007, 1_000_000_009, (1u64 << 61) - 1] {
            for degree in [4usize, 8, 16] {
                for c in [0, 1, 2, 17, 173, p - 1] {
                    for sparse in [false, true] {
                        for mode in 0..4u8 {
                            let reference = before::round63_kernel(
                                p, degree, c, mode, 4242, 32, sparse);
                            let result = after::round63_kernel(
                                p, degree, c, mode, 4242, 32, sparse);
                            assert_eq!(reference, result,
                                "p={p} degree={degree} c={c} mode={mode} sparse={sparse}");
                            compared += 32;
                        }
                    }
                }
            }
            for degree in [4usize, 8] {
                assert_eq!(
                    format!("{:?}", before::round63_whole(p, degree)),
                    format!("{:?}", after::round63_whole(p, degree)),
                    "root/event/certificate stream p={p} d={degree}");
            }
        }
        println!("ROUND63_EQ kernel_comparisons={compared} complete_streams=8");
    }

    #[test]
    #[ignore = "13 alternating A/B trials of actual production Rust functions"]
    fn bench_synthetic_linear_kernel() {
        for p in [1_000_000_007u64, (1u64 << 61) - 1] {
            for degree in [4usize, 8, 16] {
                for mode in 0..4u8 {
                    for sparse in [false, true] {
                        let reps = 512usize;
                        let mut old = Vec::new();
                        let mut new = Vec::new();
                        for trial in 0..13 {
                            let bench = |optimized: bool| {
                                let start = Instant::now();
                                if optimized {
                                    black_box(after::round63_kernel(
                                        p, degree, 173, mode, 987654, reps, sparse));
                                } else {
                                    black_box(before::round63_kernel(
                                        p, degree, 173, mode, 987654, reps, sparse));
                                }
                                start.elapsed().as_nanos() as f64 / reps as f64
                            };
                            if trial % 2 == 0 {
                                old.push(bench(false)); new.push(bench(true));
                            } else {
                                new.push(bench(true)); old.push(bench(false));
                            }
                        }
                        let a = median(old);
                        let b = median(new);
                        println!("ROUND63_NATIVE p={p} degree={degree} sparse={sparse} mode={mode} before_ns={a:.2} after_ns={b:.2} speedup={:.3}", a/b);
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "13 alternating whole bounded root plus replay trials"]
    fn bench_synthetic_linear_whole() {
        for p in [1_000_000_007u64, (1u64 << 61) - 1] {
            for degree in [4usize, 8] {
                let reps = 8usize;
                let mut old = Vec::new();
                let mut new = Vec::new();
                for trial in 0..13 {
                    let bench = |optimized: bool| {
                        let now = Instant::now();
                        for _ in 0..reps {
                            if optimized {
                                black_box(after::round63_whole(p, degree));
                            } else {
                                black_box(before::round63_whole(p, degree));
                            }
                        }
                        now.elapsed().as_nanos() as f64 / reps as f64
                    };
                    if trial % 2 == 0 {
                        old.push(bench(false)); new.push(bench(true));
                    } else {
                        new.push(bench(true)); old.push(bench(false));
                    }
                }
                let a = median(old);
                let b = median(new);
                println!("ROUND63_WHOLE p={p} degree={degree} before_ns={a:.2} after_ns={b:.2} speedup={:.3}", a/b);
            }
        }
    }
}
"""

with tempfile.TemporaryDirectory() as tmp:
    src = Path(tmp) / "round63.rs"
    exe = Path(tmp) / "round63"
    src.write_text(head + source + helper + middle + baseline + helper + tail + tests)
    subprocess.run(["rustc", "--edition=2021", "-O", "--test",
                    str(src), "-o", str(exe)], check=True)
    for name in ["exact_source_equivalence", "bench_synthetic_linear_kernel",
                 "bench_synthetic_linear_whole"]:
        argv = [str(exe), name, "--nocapture"]
        if name.startswith("bench"): argv.append("--ignored")
        subprocess.run(argv, check=True)
