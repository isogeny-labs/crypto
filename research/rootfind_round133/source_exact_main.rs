#![allow(dead_code)]
mod cryptanalysis {
    pub mod ic_boundary {
        #[derive(Clone, Debug)]
        pub struct PrimeCurve { pub p: u64, pub a: u64, pub b: u64 }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct PrimePoint { pub x: u64, pub y: u64, pub infinity: bool }
        impl PrimePoint {
            pub const INFINITY: Self = Self { x: 0, y: 0, infinity: true };
            pub fn affine(x: u64, y: u64) -> Self { Self { x, y, infinity: false } }
        }
    }
}
#[path = "baseline.rs"] mod baseline;
#[path = "optimized.rs"] mod optimized;
use std::time::Instant;

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    values[values.len()/2]
}
fn baseline_run(op: &str, p: u64, reps: usize) -> (u64, f64) {
    let start = Instant::now();
    let checksum = baseline::round129_benchmark(op, p, reps);
    (checksum, start.elapsed().as_secs_f64() * 1e9 / (32 * reps) as f64)
}
fn optimized_run(op: &str, p: u64, reps: usize) -> (u64, f64) {
    let start = Instant::now();
    let checksum = optimized::round129_benchmark(op, p, reps);
    (checksum, start.elapsed().as_secs_f64() * 1e9 / (32 * reps) as f64)
}
fn main() {
    baseline::round129_independent_oracle();
    optimized::round129_independent_oracle();
    baseline::round131_quadratic_oracle();
    optimized::round131_quadratic_oracle();
    baseline::round132_low_degree_oracle();
    optimized::round132_low_degree_oracle();
    baseline::round133_independent_oracle();
    optimized::round133_independent_oracle();
    println!("independent correctness oracle: PASS, both production Rust sources, 5 fields");
    for p in [65_521u64, 1_000_000_009, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
        for op in ["isogeny2", "isogeny3", "psi3",
            "linear_monic", "linear_scaled3", "linear_padded",
            "quadratic_double_root", "quadratic_x2_plus_1",
            "quadratic_monic", "quadratic_scaled3",
            "cubic_monic", "cubic_scaled3",
            "quartic_monic", "quartic_scaled3", "quartic_random"] {
            let repetitions = 10usize;
            let (a, _) = baseline_run(op, p, 1);
            let (b, _) = optimized_run(op, p, 1);
            assert_eq!(a, b, "A/B checksum mismatch: p={p}, {op}");
            let mut before = Vec::new();
            let mut after = Vec::new();
            let mut ratios = Vec::new();
            for trial in 0..13 {
                let (ba, ta, bb, tb) = if trial % 2 == 0 {
                    let (ba, ta) = baseline_run(op, p, repetitions);
                    let (bb, tb) = optimized_run(op, p, repetitions);
                    (ba, ta, bb, tb)
                } else {
                    let (bb, tb) = optimized_run(op, p, repetitions);
                    let (ba, ta) = baseline_run(op, p, repetitions);
                    (ba, ta, bb, tb)
                };
                assert_eq!(ba, bb, "checksum mismatch p={p}, {op}, trial={trial}");
                before.push(ta);
                after.push(tb);
                ratios.push(ta / tb);
            }
            println!("p={p} op={op} before_ns={:.2} after_ns={:.2} median_paired_speedup={:.3}x",
                median(before), median(after), median(ratios));
        }
    }
}