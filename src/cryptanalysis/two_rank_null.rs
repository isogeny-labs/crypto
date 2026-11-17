//! Null distribution of the number of distinct primes dividing the CM
//! discriminant of a prime-order curve (class-group 2-rank = ω(D_K) − 1).
//!
//! Genus theory: for a fundamental discriminant `D`, the 2-rank of `Cl(D)`
//! is `ω(D) − 1`, with `ω` the number of distinct prime divisors of `D`.
//! For a curve with prime group order `N`, the Frobenius trace `t = p+1−N` is
//! odd, `Δ = t² − 4p ≡ 1 (mod 4)` is negative, and `D_K = −d` where `d` is the
//! product of the primes that divide `|Δ|` to an odd power.  So
//! `ω(D_K)` = the number of primes with odd exponent in `|Δ|`.
//!
//! Factoring 224-bit numbers completely is out of reach, so the metric is
//! reported as bounds `[lo, hi]` (plus a `mid` imputation) around whatever
//! `p224_hypotheses::factor` leaves unsplit, and as the exact small-prime part
//! `omega_small`.  Protocol: `research/p224_two_rank_null_20261022/PROTOCOL.md`.

use crate::cryptanalysis::isogeny_walk::field::is_probable_prime;
use crate::cryptanalysis::p224_hypotheses::{bits_f64, factor, Factored};
use num_bigint::{BigInt, BigUint, RandBigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// `ω(D_K)` measured on one discriminant, as bounds.
#[derive(Clone, Debug, PartialEq)]
pub struct OmegaBounds {
    /// Rigorous lower bound: odd-exponent primes found, plus 1 for each
    /// unsplit composite cofactor that is not a perfect square.
    pub lo: u32,
    /// Imputation: each such cofactor counts as exactly two primes.
    pub mid: u32,
    /// Rigorous upper bound: each such cofactor, all of whose prime factors
    /// exceed the trial bound, counts `⌊log c / log B⌋` primes.
    pub hi: u32,
    /// Odd-exponent primes `≤` the trial bound.  Exact, never censored.
    pub small: u32,
    /// `log₂` of each unsplit cofactor (squares excluded).
    pub cofactor_bits: Vec<f64>,
    /// A cofactor shares a prime with a found prime or another cofactor, so
    /// the bounds above may be off by the overlap.  Counted, not corrected.
    pub overlap: bool,
}

impl OmegaBounds {
    /// No cofactor was left, so `lo = mid = hi`.
    pub fn exact(&self) -> bool {
        self.cofactor_bits.is_empty()
    }
}

fn is_square(c: &BigUint) -> bool {
    let s = c.sqrt();
    &(&s * &s) == c
}

/// `ω` bounds from a budgeted factorisation made with `trial_bound`.
pub fn omega_bounds(f: &Factored, trial_bound: u32) -> OmegaBounds {
    let (mut lo, mut mid, mut hi, mut small) = (0u32, 0u32, 0u32, 0u32);
    let log_b = (trial_bound as f64 + 1.0).log2();
    for (q, &(e, _)) in &f.primes {
        if e % 2 == 1 {
            lo += 1;
            mid += 1;
            hi += 1;
            if q <= &BigUint::from(trial_bound) {
                small += 1;
            }
        }
    }
    let mut cofactor_bits = Vec::new();
    let mut live: Vec<&BigUint> = Vec::new();
    for c in &f.composite {
        if is_square(c) {
            continue;
        }
        let bits = bits_f64(c);
        cofactor_bits.push(bits);
        lo += 1;
        mid += 2;
        hi += ((bits / log_b).floor() as u32).max(2);
        live.push(c);
    }
    let mut overlap = false;
    for (i, c) in live.iter().enumerate() {
        for q in f.primes.keys().filter(|q| *q > &BigUint::from(trial_bound)) {
            if (*c % q).is_zero() {
                overlap = true;
            }
        }
        for d in &live[i + 1..] {
            if c.gcd(d) != BigUint::one() {
                overlap = true;
            }
        }
    }
    OmegaBounds {
        lo,
        mid,
        hi,
        small,
        cofactor_bits,
        overlap,
    }
}

/// Factor `n` under the instrument's budget and return `ω` bounds.
pub fn omega_of(n: &BigUint, trial_bound: u32, rho_budget: u64) -> (Factored, OmegaBounds) {
    let f = factor(n, trial_bound, rho_budget);
    let b = omega_bounds(&f, trial_bound);
    (f, b)
}

/// `|t² − 4p|`.
pub fn abs_delta(p: &BigUint, t: &BigInt) -> BigUint {
    let d = t * t - BigInt::from(p * 4u8);
    d.magnitude().clone()
}

/// The smallest `k` primes `≥ 5` that do not divide `n`, multiplied into `n`.
/// The planted number has `k` more odd-exponent primes than `n`.
pub fn plant_small_primes(n: &BigUint, k: usize) -> BigUint {
    let mut out = n.clone();
    let mut planted = 0;
    let mut q = 5u32;
    while planted < k {
        let qb = BigUint::from(q);
        if (2..q).all(|d| !q.is_multiple_of(d)) && !(n % &qb).is_zero() {
            out *= &qb;
            planted += 1;
        }
        q += 2;
    }
    out
}

/// How the trace is drawn once `p + 1 − t` is required to be prime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceScheme {
    /// `t` uniform on the Hasse interval (the protocol's primary null).
    Uniform,
    /// `t` weighted by `√(1 − t²/4p)`, the Sato–Tate / Deuring density of
    /// the trace over random curves (the protocol's sensitivity null).
    Semicircle,
}

/// Splitmix64, to derive independent per-sample seeds.
pub fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A uniformly random prime with exactly `bits` bits.
pub fn random_prime(bits: u64, rng: &mut SmallRng) -> BigUint {
    loop {
        let mut c = rng.gen_biguint(bits);
        c |= BigUint::one() << (bits - 1);
        c |= BigUint::one();
        if is_probable_prime(&c) {
            return c;
        }
    }
}

fn ratio_f64(a: &BigUint, b: &BigUint) -> f64 {
    let shift = a.bits().max(b.bits()).saturating_sub(60);
    let x = (a >> shift).iter_u64_digits().next().unwrap_or(0) as f64;
    let y = (b >> shift).iter_u64_digits().next().unwrap_or(0) as f64;
    x / y
}

/// A trace `t` such that `p + 1 − t` is a prime `N`, drawn from the Hasse
/// interval under `scheme`.  Returns `(N, t)`.
pub fn sample_prime_order_trace(
    p: &BigUint,
    scheme: TraceScheme,
    rng: &mut SmallRng,
) -> (BigUint, BigInt) {
    let s = (p * 4u8).sqrt();
    let lo = p + 1u8 - &s;
    let width = &s * 2u8 + 1u8;
    loop {
        let n = &lo + rng.gen_biguint_below(&width);
        if n.is_even() || &n == p || !is_probable_prime(&n) {
            continue;
        }
        let t = BigInt::from(p + 1u8) - BigInt::from(n.clone());
        if scheme == TraceScheme::Semicircle {
            let r = ratio_f64(t.magnitude(), &s);
            if rng.gen::<f64>() >= (1.0 - r * r).max(0.0).sqrt() {
                continue;
            }
        }
        return (n, t);
    }
}

// ── Statistics ──────────────────────────────────────────────────────────────

fn ln_choose_table(n: usize) -> Vec<f64> {
    let mut lf = vec![0.0f64; n + 1];
    for i in 1..=n {
        lf[i] = lf[i - 1] + (i as f64).ln();
    }
    lf
}

/// `P(X ≤ k)` for `X ~ Binomial(n, q)`, summed in log space.
pub fn binom_cdf(k: usize, n: usize, q: f64) -> f64 {
    if q <= 0.0 {
        return 1.0;
    }
    if q >= 1.0 {
        return if k >= n { 1.0 } else { 0.0 };
    }
    let lf = ln_choose_table(n);
    let (lq, l1q) = (q.ln(), (1.0 - q).ln());
    (0..=k.min(n))
        .map(|i| (lf[n] - lf[i] - lf[n - i] + i as f64 * lq + (n - i) as f64 * l1q).exp())
        .sum::<f64>()
        .min(1.0)
}

/// Exact two-sided Clopper–Pearson interval for `k` successes in `n` at
/// confidence `1 − alpha`.
pub fn clopper_pearson(k: usize, n: usize, alpha: f64) -> (f64, f64) {
    let bisect = |f: &dyn Fn(f64) -> f64| {
        let (mut a, mut b) = (0.0f64, 1.0f64);
        for _ in 0..80 {
            let m = 0.5 * (a + b);
            if f(m) > 0.0 {
                a = m;
            } else {
                b = m;
            }
        }
        0.5 * (a + b)
    };
    let lower = if k == 0 {
        0.0
    } else {
        // P(X ≥ k | q) = alpha/2, increasing in q.
        bisect(&|q| alpha / 2.0 - (1.0 - binom_cdf(k - 1, n, q)))
    };
    let upper = if k >= n {
        1.0
    } else {
        // P(X ≤ k | q) = alpha/2, decreasing in q.
        bisect(&|q| binom_cdf(k, n, q) - alpha / 2.0)
    };
    (lower, upper)
}

/// Tail counts of a target against a control sample, with the add-one rule.
#[derive(Clone, Copy, Debug)]
pub struct Tail {
    /// Controls with `ω ≥ x`.
    pub ge: usize,
    /// Controls with `ω ≤ x`.
    pub le: usize,
    /// Controls with `ω < x`.
    pub lt: usize,
    pub n: usize,
}

impl Tail {
    pub fn new(controls: &[u32], x: u32) -> Tail {
        Tail {
            ge: controls.iter().filter(|&&c| c >= x).count(),
            le: controls.iter().filter(|&&c| c <= x).count(),
            lt: controls.iter().filter(|&&c| c < x).count(),
            n: controls.len(),
        }
    }

    /// Two-sided add-one p: `min(1, 2·min((1+ge)/(N+1), (1+le)/(N+1)))`.
    pub fn p_two_sided(&self) -> f64 {
        let d = (self.n + 1) as f64;
        (2.0 * ((1 + self.ge) as f64 / d).min((1 + self.le) as f64 / d)).min(1.0)
    }

    /// Mid-rank percentile, `(#{c < x} + ½·#{c = x}) / N`.
    pub fn midrank(&self) -> f64 {
        let eq = self.le - self.lt;
        (self.lt as f64 + 0.5 * eq as f64) / self.n as f64
    }
}

/// Mann–Whitney U test (normal approximation, tie-corrected), two-sided.
/// Returns `(z, p)`; `z > 0` when `b` is stochastically larger than `a`.
pub fn mann_whitney(a: &[u32], b: &[u32]) -> (f64, f64) {
    let (n1, n2) = (a.len() as f64, b.len() as f64);
    let mut all: Vec<(u32, bool)> = a.iter().map(|&x| (x, false)).collect();
    all.extend(b.iter().map(|&x| (x, true)));
    all.sort();
    let (mut rank_b, mut tie) = (0.0, 0.0);
    let mut i = 0;
    while i < all.len() {
        let mut j = i;
        while j < all.len() && all[j].0 == all[i].0 {
            j += 1;
        }
        let avg = (i + 1 + j) as f64 / 2.0;
        let t = (j - i) as f64;
        tie += t * t * t - t;
        rank_b += avg * all[i..j].iter().filter(|x| x.1).count() as f64;
        i = j;
    }
    let u_b = rank_b - n2 * (n2 + 1.0) / 2.0;
    let n = n1 + n2;
    let mean = n1 * n2 / 2.0;
    let var = n1 * n2 / 12.0 * ((n + 1.0) - tie / (n * (n - 1.0)));
    let z = (u_b - mean) / var.sqrt();
    (
        z,
        crate::cryptanalysis::class_number_estimate::erfc(z.abs() / 2f64.sqrt()),
    )
}

/// Sign of a trace as `+1`, `−1` or `0`.
pub fn sign_of(t: &BigInt) -> i8 {
    match t.sign() {
        Sign::Plus => 1,
        Sign::Minus => -1,
        Sign::NoSign => 0,
    }
}

/// A fixed-seed RNG for sample `idx` of arm `arm`, independent of thread
/// scheduling.
pub fn sample_rng(base_seed: u64, arm: u64, idx: u64) -> SmallRng {
    SmallRng::seed_from_u64(splitmix(splitmix(base_seed ^ arm) ^ idx))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big(s: &str) -> BigUint {
        s.parse().unwrap()
    }

    #[test]
    fn omega_counts_odd_exponent_primes_only() {
        // 3 · 5² · 7 · 11³ : odd exponents at 3, 7, 11.
        let n = big("3") * big("25") * big("7") * big("1331");
        let (f, b) = omega_of(&n, 1000, 1000);
        assert!(b.exact());
        assert_eq!((b.lo, b.mid, b.hi, b.small), (3, 3, 3, 3));
        assert!(f.complete());
    }

    #[test]
    fn omega_of_a_perfect_square_is_zero() {
        let q = big("1000003") * big("1000033");
        let n = &q * &q;
        let (_, b) = omega_of(&n, 1000, 0);
        assert_eq!((b.lo, b.mid, b.hi), (0, 0, 0));
    }

    #[test]
    fn unsplit_semiprime_gives_bounds() {
        // Two primes above the trial bound and no rho budget.
        let n = big("3") * big("1000003") * big("1000033");
        let (_, b) = omega_of(&n, 1000, 0);
        assert_eq!(b.small, 1);
        assert_eq!((b.lo, b.mid), (2, 3));
        assert!(b.hi >= b.mid && b.cofactor_bits.len() == 1);
        // With budget the same input is exact.
        let (_, e) = omega_of(&n, 1000, 1 << 20);
        assert_eq!((e.lo, e.hi), (3, 3));
    }

    #[test]
    fn nist_p224_has_omega_seven() {
        // Δ = −9·D_K with D_K = −3·29·79·7523·40927·11549194661·P45.
        let p = big("26959946667150639794667015087019630673557916260026308143510066298881");
        let t = BigInt::from(big("4733100108545601916421827343930821"));
        let abs = abs_delta(&p, &t);
        let (_, b) = omega_of(&abs, 1_000_000, 1 << 22);
        assert!(b.exact(), "P-224 discriminant must factor under the budget");
        assert_eq!((b.lo, b.hi), (7, 7));
        assert_eq!(b.small, 5);
    }

    #[test]
    fn planting_adds_exactly_k_primes() {
        let n = big("3") * big("5") * big("7") * big("1000003");
        let planted = plant_small_primes(&n, 3);
        let (_, b0) = omega_of(&n, 1000, 100);
        let (_, b1) = omega_of(&planted, 1000, 100);
        // 3, 7 and 5 divide n; planted primes are 11, 13, 17.
        assert_eq!(b1.lo, b0.lo + 3);
        assert_eq!(planted, &n * big("11") * big("13") * big("17"));
    }

    #[test]
    fn sampled_traces_give_prime_order_in_the_hasse_interval() {
        let mut rng = SmallRng::seed_from_u64(7);
        let p = random_prime(64, &mut rng);
        for scheme in [TraceScheme::Uniform, TraceScheme::Semicircle] {
            for _ in 0..20 {
                let (n, t) = sample_prime_order_trace(&p, scheme, &mut rng);
                assert!(is_probable_prime(&n));
                assert_eq!(BigInt::from(&p + 1u8) - &t, BigInt::from(n));
                assert!(&t * &t < BigInt::from(&p * 4u8));
                assert!(t.magnitude().is_odd());
            }
        }
        assert_eq!(random_prime(64, &mut rng).bits(), 64);
    }

    #[test]
    fn clopper_pearson_matches_known_values() {
        // 0 of 10: upper = 1 − 0.025^(1/10).
        let (lo, hi) = clopper_pearson(0, 10, 0.05);
        assert_eq!(lo, 0.0);
        assert!((hi - (1.0 - 0.025f64.powf(0.1))).abs() < 1e-9);
        // 5 of 10: (0.1871, 0.8129).
        let (lo, hi) = clopper_pearson(5, 10, 0.05);
        assert!((lo - 0.18705).abs() < 1e-4 && (hi - 0.81295).abs() < 1e-4);
    }

    #[test]
    fn tail_and_mann_whitney() {
        let c = [1u32, 2, 2, 3, 4];
        let t = Tail::new(&c, 2);
        assert_eq!((t.ge, t.le, t.lt), (4, 3, 1));
        assert!((t.midrank() - 0.4).abs() < 1e-12);
        let a: Vec<u32> = (0..200).map(|i| i % 5).collect();
        let b: Vec<u32> = a.iter().map(|x| x + 2).collect();
        let (z, p) = mann_whitney(&a, &b);
        assert!(z > 5.0 && p < 1e-6);
        let (z0, p0) = mann_whitney(&a, &a);
        assert!(z0.abs() < 1e-9 && p0 > 0.99);
    }
}
