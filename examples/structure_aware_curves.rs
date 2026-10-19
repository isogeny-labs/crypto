//! Curve selection for `research/structure_aware_ic_20261019/`.
//!
//! Prints `ecbench` workload curve entries (JSON, one per line) so the curves
//! of both tests are fixed by a seed and a rule before any timing:
//!
//! * `j0 <bits> <count> <seed>`: `y² = x³ + b` over `F_p`, `p ≡ 1 (mod 3)`
//!   of exactly `bits` bits, `#E` prime (so `ζ₃` acts on the whole group and
//!   `Aut` has order 6).
//! * `rand <bits> <count> <seed>`: `y² = x³ + a x + b`, `a ≠ 0`, the same
//!   `p ≡ 1 (mod 3)` rule, `#E` prime, `j ∉ {0, 1728}`: `Aut = {±1}`.
//! * `bin <n> <count> <seed0> [max_cofactor]`: binary random curves from
//!   `random_binary_instance` with the first seeds ≥ `seed0` that return a
//!   curve with `a = 1`.
//!
//! No result is computed here; only the instances are chosen.

use crypto_lib::cryptanalysis::ic_boundary::{
    random_binary_instance, PrimeCurve, PrimePoint,
};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn prime_with_bits(rng: &mut StdRng, bits: u32) -> u64 {
    loop {
        let cand = (rng.gen::<u64>() & ((1u64 << bits) - 1)) | (1u64 << (bits - 1)) | 1;
        if cand % 3 == 1 && is_prime_u64(cand) {
            return cand;
        }
    }
}

fn point(curve: &PrimeCurve) -> PrimePoint {
    (0..curve.p)
        .find_map(|x| curve.sqrt(curve.rhs(x)).map(|y| PrimePoint::affine(x, y)))
        .expect("a point exists")
}

fn emit(curve: &PrimeCurve, order: u64) {
    let g = point(curve);
    println!(
        "{{\"kind\":\"prime_explicit\",\"p\":{},\"a\":{},\"b\":{},\"group_order\":{order},\"r\":{order},\"gx\":{},\"gy\":{}}}",
        curve.p, curve.a, curve.b, g.x, g.y
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("");
    let n: u32 = args.get(2).and_then(|s| s.parse().ok()).expect("bits or n");
    let count: usize = args.get(3).and_then(|s| s.parse().ok()).expect("count");
    let seed: u64 = args.get(4).and_then(|s| s.parse().ok()).expect("seed");
    match mode {
        "j0" | "rand" => {
            let mut rng = StdRng::seed_from_u64(seed ^ (u64::from(n) << 40) ^ 0x5A17);
            let mut found = 0;
            while found < count {
                let p = prime_with_bits(&mut rng, n);
                for _ in 0..16 {
                    let b = rng.gen_range(1..p);
                    let a = if mode == "j0" { 0 } else { rng.gen_range(1..p) };
                    let curve = PrimeCurve { p, a, b };
                    let disc = (4 * mulmod(mulmod(a, a, p), a, p) % p
                        + 27 * mulmod(b, b, p) % p)
                        % p;
                    if disc == 0 {
                        continue;
                    }
                    if mode == "rand" {
                        // j = 1728 b = 0 curves are excluded by b ≥ 1; j = 0 by a ≥ 1.
                    }
                    if let Some(order) = curve.prime_order() {
                        if order == p {
                            continue; // anomalous
                        }
                        emit(&curve, order);
                        found += 1;
                        break;
                    }
                }
            }
        }
        "bin" => {
            let max_cof: u64 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(2);
            let mut s = seed;
            let mut found = 0;
            while found < count {
                if let Some(i) = random_binary_instance(n, s, max_cof) {
                    if i.a == 1 {
                        println!(
                            "{{\"kind\":\"binary_random\",\"n\":{n},\"seed\":{s},\"max_cofactor\":{max_cof}}}  // a={} b={:#x} #E={} r={} h={}",
                            i.a, i.b, i.group_order, i.r, i.cofactor
                        );
                        found += 1;
                    }
                }
                s += 1;
            }
        }
        _ => panic!("mode j0 | rand | bin"),
    }
}

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
