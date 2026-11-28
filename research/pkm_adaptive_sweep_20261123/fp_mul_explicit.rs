//! Measure the PKM oracle's exact `mulm` primitive for explicit prime workloads.
//! One invocation measures one `p`; the caller freezes its prime list and host gate.

use std::hint::black_box;
use std::time::Instant;

#[inline(always)]
fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 4, "usage: fp_mul_explicit P REPS ITERS");
    let p: u64 = args[1].parse().expect("p is u64");
    let reps: usize = args[2].parse().expect("reps is usize");
    let iters: u64 = args[3].parse().expect("iters is u64");
    assert!(p >= 5 && p % 2 == 1, "p must be odd and at least five");
    assert!(reps >= 3 && reps % 2 == 1 && iters > 0, "odd reps >= 3 and positive iters required");
    let operations = iters.checked_mul(8).expect("operation count overflow");
    let mut samples = Vec::with_capacity(reps);
    let mut checks = Vec::with_capacity(reps);
    for rep in 0..reps {
        let modulus = black_box(p);
        let b = black_box(1 + ((rep as u64 + 2) % (p - 1)));
        let mut a = black_box(1 + ((rep as u64 + 1) % (p - 1)));
        let start = Instant::now();
        for _ in 0..iters {
            a = mulm(a, b, modulus);
            a = mulm(a, b, modulus);
            a = mulm(a, b, modulus);
            a = mulm(a, b, modulus);
            a = mulm(a, b, modulus);
            a = mulm(a, b, modulus);
            a = mulm(a, b, modulus);
            a = mulm(a, b, modulus);
        }
        let elapsed_ns = start.elapsed().as_nanos();
        checks.push(black_box(a));
        samples.push(elapsed_ns as f64 / operations as f64);
    }
    let mut sorted = samples.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let series = samples.iter().map(|x| format!("{x:.9}"))
        .collect::<Vec<_>>().join(",");
    let check_series = checks.iter().map(u64::to_string)
        .collect::<Vec<_>>().join(",");
    println!("{{\"p\":{p},\"reps\":{reps},\"iters\":{iters},\"field_mults_per_rep\":{operations},\"ns_per_mul_per_rep\":[{series}],\"ns_per_mul_median\":{:.9},\"result_checks\":[{check_series}]}}", sorted[reps / 2]);
}
