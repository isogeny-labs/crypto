//! Source-shaped PKM selected-map arithmetic probe. One process prices one prime and path.
//! The Jacobi workload requires exact selector operands and is intentionally absent.

use std::hint::black_box;
use std::time::Instant;

#[inline(always)]
fn curve_word_mul(a: u64, b: u64, p: u64) -> u64 {
    // The p < 2^32, reduced-operand branch of ic_boundary::mulmod on arm64.
    let prod = a * b;
    prod % p
}

#[inline(always)]
fn residual_pow_mul(a: u64, b: u64, p: u64) -> u64 {
    // The small-product branch of residual_walk::mul_mod.
    let wide = a as u128 * b as u128;
    match u64::try_from(wide) {
        Ok(w) => w % p,
        Err(_) => (wide % p as u128) as u64,
    }
}

fn ref_pow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut out = 1u64;
    while e != 0 {
        if e & 1 != 0 {
            out = ((out as u128 * a as u128) % p as u128) as u64;
        }
        a = ((a as u128 * a as u128) % p as u128) as u64;
        e >>= 1;
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 5,
        "usage: fp_selected_primitives PATH P REPS ITERS"
    );
    let path = args[1].as_str();
    assert!(matches!(path, "curve_word_mul" | "residual_pow_mul"));
    let p: u64 = args[2].parse().expect("p is u64");
    let reps: usize = args[3].parse().expect("reps is usize");
    let iters: u64 = args[4].parse().expect("iters is u64");
    assert!(p >= 5 && p < (1u64 << 32) && p % 2 == 1);
    assert!(reps >= 3 && reps % 2 == 1 && iters > 0);
    let operations = iters.checked_mul(8).expect("operation count overflow");
    let mut samples = Vec::with_capacity(reps);
    let mut checks = Vec::with_capacity(reps);
    for rep in 0..reps {
        let modulus = black_box(p);
        let b = black_box(1 + ((rep as u64 + 2) % (p - 1)));
        let a0 = black_box(1 + ((rep as u64 + 1) % (p - 1)));
        let mut a = a0;
        let start = Instant::now();
        for _ in 0..iters {
            for _ in 0..8 {
                a = match path {
                    "curve_word_mul" => curve_word_mul(a, b, modulus),
                    _ => residual_pow_mul(a, b, modulus),
                };
            }
        }
        let elapsed_ns = start.elapsed().as_nanos();
        let expected = ((a0 as u128 * ref_pow(b, operations, p) as u128) % p as u128) as u64;
        assert_eq!(a, expected, "independent modular reference differs");
        checks.push(black_box(a));
        samples.push(elapsed_ns as f64 / operations as f64);
    }
    let mut sorted = samples.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let series = samples
        .iter()
        .map(|x| format!("{x:.9}"))
        .collect::<Vec<_>>()
        .join(",");
    let check_series = checks
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    println!("{{\"schema\":\"pkm-selected-primitive/v1\",\"path\":\"{path}\",\"p\":{p},\"reps\":{reps},\"iters\":{iters},\"field_mults_per_rep\":{operations},\"ns_per_mul_per_rep\":[{series}],\"ns_per_mul_median\":{:.9},\"result_checks\":[{check_series}]}}", sorted[reps / 2]);
}
