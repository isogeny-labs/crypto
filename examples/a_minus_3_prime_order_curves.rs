//! Find curves `y² = x³ − 3x + b` over `F_p` with `#E = h·r`, `r` prime,
//! for the generator-effect study (research/generator_effect_ic_20261019).
//!
//! `cargo run --release --example a_minus_3_prime_order_curves -- BITS COUNT SEED [H]`
//!
//! Draws `p` with exactly `BITS` bits and `b` at random, keeps curves with
//! `#E = H·r` for a prime `r > H` (default `H = 1`, prime `#E`), and prints
//! one `ecbench` `prime_explicit` curve per line.  At `H = 1` the generator
//! is the smallest-abscissa point, "the standard's G" of the plan, and the
//! order is counted by `prime_order` (the E1 population, unchanged).  At
//! `H > 1` (experiment E2's cofactor ladder) `#E` is the single multiple
//! of a point's order in the Hasse interval, and the generator is `[H]P`
//! for the smallest-abscissa `P` with `[H]P ≠ ∞`, of order `r`.
use crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let args: Vec<u64> = std::env::args()
        .skip(1)
        .map(|a| a.parse().expect("integer argument"))
        .collect();
    let (bits, count, seed) = (args[0] as u32, args[1] as usize, args[2]);
    let h = args.get(3).copied().unwrap_or(1);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut found = 0;
    while found < count {
        let p = loop {
            let cand = (rng.gen::<u64>() & ((1u64 << bits) - 1)) | (1u64 << (bits - 1)) | 1;
            if is_prime_u64(cand) {
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
        let (order, r) = if h == 1 {
            let Some(r) = curve.prime_order() else {
                continue;
            };
            (r, r)
        } else {
            let Some(n) = hasse_order(&curve) else {
                continue;
            };
            if n % h != 0 || n / h <= h || !is_prime_u64(n / h) {
                continue;
            }
            (n, n / h)
        };
        let mut ops = GroupOps::default();
        let g: PrimePoint = (0..p)
            .find_map(|x| {
                let y = curve.sqrt(curve.rhs(x))?;
                let pt = PrimePoint::affine(x, y.min(p - y));
                let g = curve.mul(&mut ops, pt, h);
                (curve.legendre(curve.rhs(x)) >= 0 && !g.infinity).then_some(g)
            })
            .expect("a point");
        println!(
            "{{\"kind\": \"prime_explicit\", \"p\": {p}, \"a\": {}, \"b\": {b}, \"group_order\": {order}, \"r\": {r}, \"gx\": {}, \"gy\": {}}}",
            p - 3,
            g.x,
            g.y
        );
        found += 1;
    }
}

/// `#E(F_p)` as the one multiple of a point's order in the Hasse interval
/// `[p+1−t, p+1+t]`, `t = ⌊2√p⌋`; `None` when the first point's order is
/// small enough to leave several multiples there (the curve is skipped).
fn hasse_order(curve: &PrimeCurve) -> Option<u64> {
    let p = curve.p;
    let pt = (0..p).find_map(|x| Some(PrimePoint::affine(x, curve.sqrt(curve.rhs(x))?)))?;
    let t = (4 * p).isqrt();
    let mut ops = GroupOps::default();
    let mut acc = curve.mul(&mut ops, pt, p + 1 - t);
    let mut hit = None;
    for j in 0..=2 * t {
        if acc.infinity {
            if hit.is_some() {
                return None;
            }
            hit = Some(p + 1 - t + j);
        }
        acc = curve.add(&mut ops, acc, pt);
    }
    hit
}
