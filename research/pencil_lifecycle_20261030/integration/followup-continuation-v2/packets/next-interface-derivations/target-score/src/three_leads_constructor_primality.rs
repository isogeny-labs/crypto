//! Observed copy of engine::is_prime, preserving its u64 deterministic algorithm.
//! This is a separate cold-validation ledger, never the symbolic constructor meter.
//! M and S include their product reduction. product_reductions and loop/check
//! counts are diagnostics, not additional priced operations. Standalone reductions
//! exclude those product reductions. Branches, comparisons and loop bookkeeping
//! are not a complete CPU ledger and must not be inferred from these diagnostics.

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct Counts {
    pub modular_muls: u64,
    pub modular_squares: u64,
    pub product_reductions: u64,
    pub standalone_modular_reductions: u64,
    pub divisibility_checks: u64,
    pub small_divisor_iterations: u64,
    pub witness_iterations: u64,
    pub exponent_iterations: u64,
    pub witness_square_iterations: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Observation {
    pub modulus: u64,
    pub prime: bool,
    pub counts: Counts,
}
fn product(a: u64, b: u64, p: u64, square: bool, c: &mut Counts) -> u64 {
    if square {
        c.modular_squares += 1;
    } else {
        c.modular_muls += 1;
    }
    c.product_reductions += 1;
    (a as u128 * b as u128 % p as u128) as u64
}
fn reduce(a: u64, p: u64, c: &mut Counts) -> u64 {
    c.standalone_modular_reductions += 1;
    a % p
}
fn power(mut a: u64, mut e: u64, p: u64, c: &mut Counts) -> u64 {
    let mut r = 1;
    while e != 0 {
        c.exponent_iterations += 1;
        if e & 1 != 0 {
            r = product(r, a, p, false, c);
        }
        e >>= 1;
        if e != 0 {
            a = product(a, a, p, true, c);
        }
    }
    r
}
fn check(p: u64, c: &mut Counts) -> bool {
    if p < 2 {
        return false;
    }
    for q in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        c.small_divisor_iterations += 1;
        c.divisibility_checks += 1;
        if reduce(p, q, c) == 0 {
            return p == q;
        }
    }
    let s = (p - 1).trailing_zeros();
    let d = (p - 1) >> s;
    'a: for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        c.witness_iterations += 1;
        if reduce(a, p, c) == 0 {
            continue;
        }
        let mut x = power(reduce(a, p, c), d, p, c);
        if x == 1 || x == p - 1 {
            continue;
        }
        for _ in 1..s {
            c.witness_square_iterations += 1;
            x = product(x, x, p, true, c);
            if x == p - 1 {
                continue 'a;
            }
        }
        return false;
    }
    true
}
pub fn observe(p: u64) -> Observation {
    let mut counts = Counts::default();
    let prime = check(p, &mut counts);
    Observation {
        modulus: p,
        prime,
        counts,
    }
}
