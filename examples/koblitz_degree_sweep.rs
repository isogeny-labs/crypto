//! Every prime degree `m` in a range where a Koblitz curve
//! `K_a : y² + xy = x³ + a x² + 1` over `GF(2^m)` has an almost-prime order
//! (`#K_0 = 4n`, `#K_1 = 2n`, `n` prime), for the census in
//! `research/nist_binary_skipped_20261020/`.
//!
//! ```bash
//! cargo run --release --example koblitz_degree_sweep -- --from 100 --to 600
//! ```
//!
//! Prints `m a h bits(n) gnb_type fips` per candidate, then for each FIPS
//! security level `s` the smallest candidate with `bits(n) ≥ 2s` beside the
//! degree FIPS 186-2 chose.  Orders by the Lucas recurrence
//! ([`koblitz_order`]), primality by Miller–Rabin with twenty bases.

use crypto_lib::cryptanalysis::ecc2k130_guard::is_probable_prime;
use crypto_lib::cryptanalysis::koblitz_wide::koblitz_order;
use num_bigint::BigUint;

/// FIPS 186-2 K-curves: (security level, m, a).
const FIPS: [(u32, u32, u8); 5] = [
    (80, 163, 1),
    (112, 233, 0),
    (128, 283, 0),
    (192, 409, 0),
    (256, 571, 0),
];

fn arg(name: &str, default: u32) -> u32 {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn is_prime_u32(n: u32) -> bool {
    n >= 2
        && (2..)
            .take_while(|d| d * d <= n)
            .all(|d| !n.is_multiple_of(d))
}

fn order_mod(a: u64, p: u64) -> u64 {
    let (mut x, mut k) = (a % p, 1);
    while x != 1 {
        x = x * a % p;
        k += 1;
    }
    k
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least type `T` of a Gaussian normal basis of `GF(2^m)`.
fn gnb_type(m: u32) -> u32 {
    let m = m as u64;
    (1..)
        .find(|&t| {
            let p = t * m + 1;
            is_prime_u32(p as u32) && gcd(t * m / order_mod(2, p), m) == 1
        })
        .unwrap() as u32
}

fn main() {
    let (from, to) = (arg("--from", 100), arg("--to", 600));
    let mut found: Vec<(u32, u8, u64)> = Vec::new();
    println!("# m a h bits(n) gnb_type fips");
    for m in (from..=to).filter(|&m| is_prime_u32(m)) {
        for a in [0u8, 1] {
            let h = if a == 0 { 4u32 } else { 2 };
            let order = koblitz_order(a, m);
            if &order % h != BigUint::from(0u8) {
                continue;
            }
            let n = &order / h;
            if is_probable_prime(&n) {
                let fips = FIPS.iter().any(|&(_, d, fa)| d == m && fa == a);
                println!(
                    "{m} {a} {h} {} {} {}",
                    n.bits(),
                    gnb_type(m),
                    if fips { "fips" } else { "-" }
                );
                found.push((m, a, n.bits()));
            }
        }
    }
    println!("# level need first_m first_a bits(n) fips_m");
    for (s, d, _) in FIPS {
        let first = found.iter().find(|&&(_, _, b)| b >= 2 * s as u64);
        match first {
            Some(&(m, a, b)) => println!("{s} {} {m} {a} {b} {d}", 2 * s),
            None => println!("{s} {} - - - {d}", 2 * s),
        }
    }
}
