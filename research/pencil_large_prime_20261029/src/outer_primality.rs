//! Observed copy of engine::is_prime, preserving its u64 deterministic algorithm.
//! This is a separate cold-validation ledger, never the symbolic constructor meter.
//! M and S include their product reduction. product_reductions and loop/check
//! counts are diagnostics, not additional priced operations. Standalone reductions
//! exclude those product reductions. Branches, comparisons and loop bookkeeping
//! are not a complete CPU ledger and must not be inferred from these diagnostics.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
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
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
pub struct Observation {
    pub modulus: u64,
    pub prime: bool,
    pub counts: Counts,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
pub struct OuterValidation {
    /// None means not invoked, including a preceding validation short circuit.
    pub base_modulus: Option<Observation>,
    pub scalar_modulus: Option<Observation>,
}
impl OuterValidation {
    pub fn check_base(&mut self, p: u64) -> bool {
        let observation = observe(p);
        let prime = observation.prime;
        self.base_modulus = Some(observation);
        prime
    }
    pub fn check_scalar(&mut self, r: u64) -> bool {
        let observation = observe(r);
        let prime = observation.prime;
        self.scalar_modulus = Some(observation);
        prime
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_small_divisor_and_noninvocation() {
        assert_eq!(observe(0).counts, Counts::default());
        assert_eq!(observe(1).counts, Counts::default());
        let mut expected = Counts::default();
        expected.small_divisor_iterations = 1;
        expected.divisibility_checks = 1;
        expected.standalone_modular_reductions = 1;
        assert_eq!(observe(2).counts, expected);
        assert!(observe(2).prime);
        assert!(!observe(4).prime);
        assert_eq!(observe(4).counts, expected);
        let mut v = OuterValidation::default();
        assert!(!v.check_base(4));
        assert!(v.scalar_modulus.is_none());
        assert!(v.base_modulus.is_some());
    }
    #[test]
    fn exact_prime_and_composite_powers() {
        // 47-1=2*23: seven five-bit powers, popcount(23)=4,
        // no Miller--Rabin inner squaring when s=1.
        let p = observe(47);
        assert!(p.prime);
        assert_eq!(
            p.counts,
            Counts {
                modular_muls: 28,
                modular_squares: 28,
                product_reductions: 56,
                standalone_modular_reductions: 26,
                divisibility_checks: 12,
                small_divisor_iterations: 12,
                witness_iterations: 7,
                exponent_iterations: 35,
                witness_square_iterations: 0
            }
        );
        // 41*43=1763: base 2 rejects; d=881 has ten bits and six set bits.
        let composite = observe(1763);
        assert!(!composite.prime);
        assert_eq!(
            composite.counts,
            Counts {
                modular_muls: 6,
                modular_squares: 9,
                product_reductions: 15,
                standalone_modular_reductions: 14,
                divisibility_checks: 12,
                small_divisor_iterations: 12,
                witness_iterations: 1,
                exponent_iterations: 10,
                witness_square_iterations: 0
            }
        );
    }
    #[test]
    fn large_prime_exact_and_zero_power() {
        // 2^61-1 is prime; d=2^60-1, seven 60-bit all-one powers.
        let p = observe((1u64 << 61) - 1);
        assert!(p.prime);
        assert_eq!(p.counts.modular_muls, 420);
        assert_eq!(p.counts.modular_squares, 413);
        assert_eq!(p.counts.product_reductions, 833);
        assert_eq!(p.counts.standalone_modular_reductions, 26);
        let mut c = Counts::default();
        assert_eq!(power(5, 0, 0, &mut c), 1);
        assert_eq!(c, Counts::default());
    }
    #[cfg(feature = "producer")]
    #[test]
    fn differential_actual_engine() {
        for p in (0..10000).chain([
            341550071728321,
            3825123056546413051,
            (1u64 << 61) - 1,
            u64::MAX - 58,
            u64::MAX - 1,
            u64::MAX,
        ]) {
            assert_eq!(observe(p).prime, crate::engine::is_prime(p), "{p}");
        }
    }
}
