#!/usr/bin/env python3
"""Round 64: source-exact before/after of monic-linear GCD and nonmonic
linear squaring in the bounded prime-field root finder and independent replay."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
baseline = source
for block in ["PRODUCER_MONIC_LINEAR_GCD", "REPLAY_MONIC_LINEAR_GCD",
              "PRODUCER_NONMONIC_LINEAR_SQUARE", "REPLAY_NONMONIC_LINEAR_SQUARE"]:
    pat = rf"(?ms)^ *// R64_{block}_BEGIN\n.*?^ *// R64_{block}_END\n"
    baseline, count = re.subn(pat, "", baseline, count=1)
    assert count == 1, "expected unique round64 marker pair " + block

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
pub fn round64_kernel(p: u64, degree: usize, mode: u8, divisible: bool,
                      seed: u64, repeats: usize) -> (u64, u64) {
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
    let c = if p < 173 { 17 } else { 173 };
    let divisor = Poly17 { coefficients: {
            let mut arr = [0u64; 17]; arr[0] = c; arr[1] = 1; arr
        }, len: 2 };
    for iteration in 0..repeats {
        let mut input = Poly17::ZERO;
        let mut state = black_box(seed.wrapping_add(iteration as u64))
            .wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(p);
        if mode <= 1 {
            input.len = (degree + 1) as u8;
            for i in 0..=degree {
                state = state.wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                input.coefficients[i] = state % p;
            }
            input.coefficients[degree] = 1;
            if divisible {
                // Force f(-c)=0 using independent u128 Horner arithmetic.
                let x = p - c;
                let mut value = 0u64;
                for i in (0..=degree).rev() {
                    value = ((value as u128 * x as u128
                           + input.coefficients[i] as u128)
                           % p as u128) as u64;
                }
                input.coefficients[0] =
                    (input.coefficients[0] + p - value) % p;
            }
        } else {
            input.len = 2;
            state = state.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            input.coefficients[0] = 1 + state % (p - 1);
            state = state.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            input.coefficients[1] = 2 + state % (p - 3);
        }
        let input = black_box(input);
        let output = match mode {
            0 => producer.gcd(input, black_box(divisor)).unwrap(),
            1 => replay.monic_gcd(input, black_box(divisor)).unwrap(),
            2 => producer.square(&input).unwrap(),
            3 => replay.square_diagonals(&input).unwrap(),
            _ => panic!("invalid mode"),
        };
        let mut fingerprint = output.len as u64;
        for k in 0..17 {
            fingerprint = fingerprint.wrapping_mul(37)
                .wrapping_add(output.coefficients[k]);
        }
        digest = digest.wrapping_mul(0x9e3779b9)
            .wrapping_add(black_box(fingerprint));
        let gate = if mode % 2 == 0 { &mut producer.gate } else { &mut replay.gate };
        cost += gate.metrics.field_work;
        gate.metrics.field_work = 0;
        gate.metrics.work = 0;
    }
    (digest, cost)
}

pub fn round64_whole(p: u64, degree: usize) ->
    (Stop, [u64; 8], u8, Vec<BoundedRootEvent>) {
    use std::time::Duration;
    let limits = RootBudget { work: MAX_WORK, field_work: MAX_FIELD_WORK,
        bytes: STORAGE_BYTES, events: MAX_EVENTS, split_attempts: 1000,
        degree: 8 };
    let field = super::super::fp_backend::Fp64Arithmetic { p };
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
mod round64 {
    use super::cryptanalysis::bounded::{before, after};
    use std::{hint::black_box, time::Instant};

    fn median(mut x: Vec<f64>) -> f64 {
        x.sort_by(f64::total_cmp); x[x.len() / 2]
    }

    #[test]
    fn exact_source_equivalence() {
        let mut compared = 0usize;
        for p in [101u64, 1_000_000_007, 1_000_000_009, (1u64 << 61) - 1] {
            for degree in [2usize, 4, 8, 16] {
                for divisible in [false, true] {
                    for mode in 0..4u8 {
                        let old = before::round64_kernel(
                            p, degree, mode, divisible, 4242, 128);
                        let new = after::round64_kernel(
                            p, degree, mode, divisible, 4242, 128);
                        assert_eq!(old.0, new.0, "result mismatch p={p} d={degree} mode={mode} divisible={divisible}");
                        compared += 128;
                    }
                }
            }
            for degree in [4usize, 8] {
                assert_eq!(
                    format!("{:?}", before::round64_whole(p, degree)),
                    format!("{:?}", after::round64_whole(p, degree)),
                    "root/event/certificate stream p={p} d={degree}");
            }
        }
        println!("ROUND64_EQ kernel_comparisons={compared} complete_streams=8");
    }

    #[test]
    #[ignore = "13 alternating A/B trials of compiled production Rust"]
    fn bench_gcd_square_kernel() {
        for p in [1_000_000_007u64, (1u64 << 61) - 1] {
            for mode in 0..4u8 {
                for degree in [4usize, 8, 16] {
                    if mode >= 2 && degree != 4 { continue; }
                    for divisible in [false, true] {
                        if mode >= 2 && divisible { continue; }
                        let reps = 384usize;
                        let mut old = Vec::new();
                        let mut new = Vec::new();
                        for trial in 0..13 {
                            let bench = |optimized: bool| {
                                let now = Instant::now();
                                if optimized {
                                    black_box(after::round64_kernel(
                                        p, degree, mode, divisible, 98765, reps));
                                } else {
                                    black_box(before::round64_kernel(
                                        p, degree, mode, divisible, 98765, reps));
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
                        println!("ROUND64_NATIVE p={p} degree={degree} divisible={divisible} mode={mode} before_ns={a:.2} after_ns={b:.2} speedup={:.3}", a/b);
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "13 alternating complete root extraction and certificate replay"]
    fn bench_complete_root_and_replay() {
        for p in [1_000_000_007u64, (1u64 << 61) - 1] {
            for degree in [4usize, 8] {
                let reps = 8usize;
                let mut old = Vec::new();
                let mut new = Vec::new();
                for trial in 0..13 {
                    let bench = |optimized: bool| {
                        let now = Instant::now();
                        for _ in 0..reps {
                            if optimized { black_box(after::round64_whole(p, degree)); }
                            else { black_box(before::round64_whole(p, degree)); }
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
                println!("ROUND64_WHOLE p={p} degree={degree} before_ns={a:.2} after_ns={b:.2} speedup={:.3}", a/b);
            }
        }
    }
}
"""

with tempfile.TemporaryDirectory() as tmp:
    path = Path(tmp) / "round64.rs"
    executable = Path(tmp) / "round64"
    path.write_text(head + source + helper + middle + baseline + helper + tail + tests)
    subprocess.run(["rustc", "--edition=2021", "-O", "--test",
                    str(path), "-o", str(executable)], check=True)
    for name in ["exact_source_equivalence", "bench_gcd_square_kernel",
                 "bench_complete_root_and_replay"]:
        args = [str(executable), name, "--nocapture"]
        if name.startswith("bench"): args.append("--ignored")
        subprocess.run(args, check=True)
