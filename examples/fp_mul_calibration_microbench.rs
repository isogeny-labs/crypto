//! Primitive microbenchmark for pinning `ns_per_fp_mul` on prime table rows.
//!
//! 400_000 dependent field multiplications (`u128` product mod `p`) per rep,
//! matching `docs/ic/calibration-solver-units.md` tooling item 2.

use crypto_lib::cryptanalysis::ic_boundary::{
    calibrate_group, calibrate_row_ops, fp_p31_subexp_mersenne_instance, roster_prime_instance,
    Calibration, CountedGroup, GroupOps, PrimeInstance, PrimePoint,
};
use serde_json::json;
use std::time::Instant;

fn field_mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn bench_fp_mul(inst: &PrimeInstance, samples: u64, seed: u64) -> f64 {
    let p = inst.curve.p;
    let mut a = 1u64.max(seed % p);
    let mut b = 2u64.max((seed >> 17) % p);
    let start = Instant::now();
    for i in 0..samples {
        a = field_mul(a, b, p);
        b = field_mul(b, a.wrapping_add(i + 1), p);
    }
    let wall = start.elapsed().as_nanos() as f64;
    std::hint::black_box((a, b));
    wall / samples as f64
}

fn sample_points(inst: &PrimeInstance) -> Vec<PrimePoint> {
    let g = inst.generator_point();
    let mut ops = GroupOps::default();
    (1..=8u64)
        .map(|k| inst.curve.mul(&mut ops, g, k))
        .collect()
}

fn main() {
    let bits: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);
    let reps: u32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let samples: u64 = 400_000;

    let (inst, legacy, table_row) = if bits == 31 {
        let inst = fp_p31_subexp_mersenne_instance();
        (
            inst,
            "generated-31bit-2147483647".to_string(),
            "prime/generated-31bit-2147483647".to_string(),
        )
    } else {
        let inst = roster_prime_instance(bits).expect("roster prime instance");
        let legacy = format!("bench-{bits}bit");
        let table_row = format!("prime/{legacy}");
        (inst, legacy, table_row)
    };

    let mut calib = Calibration::default();
    calibrate_group(&inst.curve, &sample_points(&inst), &mut calib);
    calibrate_row_ops(inst.r, &mut calib);
    calib.pin("prime", &legacy);
    let ns_per_add = calib.ns_per_add;

    let mut ratios = Vec::new();
    for seed in 1..=reps {
        let ns_per_mul = bench_fp_mul(&inst, samples, u64::from(seed));
        ratios.push(ns_per_mul / ns_per_add);
    }
    let mut sorted = ratios.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = sorted[sorted.len() / 2];
    println!(
        "{}",
        serde_json::to_string(&json!({
            "table_row": table_row,
            "instance": inst.name,
            "bits": bits,
            "samples_per_rep": samples,
            "reps": reps,
            "ns_per_add": ns_per_add,
            "ns_per_fp_mul_median_ratio": (median * 1e6).round() / 1e6,
            "ratios_per_rep": ratios,
        }))
        .expect("json")
    );
}