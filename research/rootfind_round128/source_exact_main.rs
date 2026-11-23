#![allow(dead_code)]
mod cryptanalysis {
    pub mod ic_boundary {
        #[derive(Clone)]
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
fn run_baseline(op: &str, p: u64, n: usize) -> (u64, f64) {
    let t = Instant::now();
    let checksum = baseline::round128_benchmark(op, p, n);
    (checksum, t.elapsed().as_secs_f64() * 1e9 / (n * 32) as f64)
}
fn run_optimized(op: &str, p: u64, n: usize) -> (u64, f64) {
    let t = Instant::now();
    let checksum = optimized::round128_benchmark(op, p, n);
    (checksum, t.elapsed().as_secs_f64() * 1e9 / (n * 32) as f64)
}
fn main() {
    println!("compiler: {:?}", option_env!("RUSTC_VERSION"));
    for p in [65_521u64, 1_000_000_007, 4_294_967_291] {
        for op in ["mul", "add", "sub", "pow", "rem", "powmod", "roots"] {
            let reps = match op { "mul" | "add" | "sub" => 20_000, "pow" => 300,
                "rem" => 1_000, "powmod" => 40, _ => 10 };
            let (a, _) = run_baseline(op, p, 1);
            let (b, _) = run_optimized(op, p, 1);
            assert_eq!(a, b, "checksum mismatch on {p} {op}");
            let mut before = Vec::new();
            let mut after = Vec::new();
            let mut ratios = Vec::new();
            for trial in 0..11 {
                let (ra, ta, rb, tb) = if trial % 2 == 0 {
                    let (ra, ta) = run_baseline(op, p, reps);
                    let (rb, tb) = run_optimized(op, p, reps);
                    (ra, ta, rb, tb)
                } else {
                    let (rb, tb) = run_optimized(op, p, reps);
                    let (ra, ta) = run_baseline(op, p, reps);
                    (ra, ta, rb, tb)
                };
                assert_eq!(ra, rb, "result mismatch on {p} {op} trial {trial}");
                before.push(ta);
                after.push(tb);
                ratios.push(ta / tb);
            }
            println!("p={p} op={op} before_ns={:.3} after_ns={:.3} median_paired_speedup={:.3}x", median(before), median(after), median(ratios));
        }
    }
}
