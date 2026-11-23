//! Run60: measure the minimum affine public-shift phase of the Prime31
//! two-summand idealized target-query pipeline, using original Rust arithmetic.
//! This deliberately performs NO index calculus extraction, lookup, or DLP.
//! Reproduce: cargo run --release --example prime31_run60_shift_floor
use prime31_algebraic_ic::{check_curve, Arithmetic, PRIME31};
use std::hint::black_box;
use std::time::Instant;

const ITERATIONS: u64 = 250_000;
const REPETITIONS: usize = 5;
const OLD_BSGS_MS: f64 = 170.113;
const MODEL_CALLS_FOR_1024: f64 = 4_699_949.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arithmetic = Arithmetic::default();
    check_curve(PRIME31, &mut arithmetic)?;
    let g = Some(PRIME31.g);
    let mut q = g;
    let mut cumulative = 1_u64;
    let mut digest = 0_u64;
    let mut samples_ns = Vec::with_capacity(REPETITIONS);
    for rep in 0..REPETITIONS {
        let before = arithmetic.counts;
        let began = Instant::now();
        for _ in 0..ITERATIONS {
            q = arithmetic.add(PRIME31, q, g);
            digest ^= q.map_or(0, |(x, y)| x ^ y);
            black_box(q);
        }
        let ns = began.elapsed().as_nanos() as f64 / ITERATIONS as f64;
        let additions = arithmetic.counts.group_additions
            - before.group_additions;
        // The scalar cross-check below is excluded from the timed shift loop.
        assert_eq!(additions, ITERATIONS,
                   "shift loop counted unexpected EC additions");
        cumulative += ITERATIONS;
        assert_eq!(q, arithmetic.scalar(PRIME31, cumulative, g),
                   "public shift drift from original scalar multiplication");
        println!("run60 rep={} iterations={} affine_shift_ns_per_call={:.3}",
                 rep, ITERATIONS, ns);
        samples_ns.push(ns);
    }
    black_box(digest);
    samples_ns.sort_by(|a, b| a.total_cmp(b));
    let median_ns = samples_ns[REPETITIONS / 2];
    let threshold_ns = OLD_BSGS_MS * 1_000_000.0 / MODEL_CALLS_FOR_1024;
    // IMPORTANT: old 170.113-ms result is from another run and different
    // scope; this is an admission sensitivity only, NOT a same-host win/loss.
    println!("run60 original-rust Prime31 p={} n={} median_affine_shift_ns={:.3} \
              historical_bsgs_sensitivity_ns={:.3} ratio={:.3} \
              predicted_1024_shift_only_ms={:.3} digest={} \
              classification=affine_primitive_only_no_blind_extraction",
             PRIME31.p, PRIME31.n, median_ns, threshold_ns,
             median_ns / threshold_ns, median_ns * MODEL_CALLS_FOR_1024 / 1e6,
             digest);
    Ok(())
}
