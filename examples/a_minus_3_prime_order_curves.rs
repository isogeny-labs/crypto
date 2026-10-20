//! Find prime-order curves `y² = x³ − 3x + b` over `F_p` for the
//! generator-effect study (research/generator_effect_ic_20261019).
//!
//! `cargo run --release --example a_minus_3_prime_order_curves -- BITS COUNT SEED [MIN_V2]`
//!
//! With `MIN_V2`, only primes with `2^MIN_V2 | p − 1` are kept, so that the
//! `prime-kummer` tower of height up to `MIN_V2` exists (experiment E3).
//! The draws are those of the plain search, filtered: without the argument
//! the output is unchanged.
//!
//! Draws `p` with exactly `BITS` bits and `b` at random, keeps curves with
//! prime `#E` (cofactor 1), and prints one `ecbench` `prime_explicit` curve
//! per line.  The generator is the smallest-abscissa point, "the
//! standard's G" of the plan; the order is counted by `prime_order`.
use crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
use crypto_lib::cryptanalysis::ic_boundary::{PrimeCurve, PrimePoint};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let args: Vec<u64> = std::env::args()
        .skip(1)
        .map(|a| a.parse().expect("integer argument"))
        .collect();
    let (bits, count, seed) = (args[0] as u32, args[1] as usize, args[2]);
    let min_v2 = args.get(3).copied().unwrap_or(0) as u32;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut found = 0;
    while found < count {
        let p = loop {
            let cand = (rng.gen::<u64>() & ((1u64 << bits) - 1)) | (1u64 << (bits - 1)) | 1;
            if is_prime_u64(cand) && (cand - 1).trailing_zeros() >= min_v2 {
                break cand;
            }
        };
        let b = rng.gen_range(1..p);
        let curve = PrimeCurve { p, a: p - 3, b };
        let disc = (4 * (p - 3) as u128 * (p - 3) as u128 % p as u128 * (p - 3) as u128
            + 27 * b as u128 * b as u128)
            % p as u128;
        if disc == 0 {
            continue;
        }
        let Some(r) = curve.prime_order() else {
            continue;
        };
        let g: PrimePoint = (0..p)
            .find_map(|x| {
                let y = curve.sqrt(curve.rhs(x))?;
                (curve.legendre(curve.rhs(x)) >= 0).then(|| PrimePoint::affine(x, y.min(p - y)))
            })
            .expect("a point");
        println!(
            "{{\"kind\": \"prime_explicit\", \"p\": {p}, \"a\": {}, \"b\": {b}, \"group_order\": {r}, \"r\": {r}, \"gx\": {}, \"gy\": {}}}",
            p - 3,
            g.x,
            g.y
        );
        found += 1;
    }
}
