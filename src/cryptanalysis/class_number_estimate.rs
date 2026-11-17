//! Class-number estimates and genus theory for imaginary quadratic
//! discriminants too large for a class-group computation (the CM fields of
//! NIST P-224 and P-256, `|D| ~ 2^222` and `2^258`).
//!
//! Protocol: `research/p224_class_number_20261022/PROTOCOL.md`.
//!
//! Evidence labels used in the docs below:
//!
//! * **exact**: a theorem plus exact integer arithmetic (genus theory, the
//!   Rédei 4-rank, the reduced-form count used as a test oracle);
//! * **GRH-conditional**: holds if the generalised Riemann hypothesis holds
//!   and, where stated, if an explicit constant quoted from the literature
//!   shape is the right one;
//! * **heuristic**: a random-sign model, no proof.
//!
//! Nothing here computes `h(D)` exactly for a large `D`.
//!
//! The analytic class number formula for `D < -4` is
//! `h(D) = sqrt|D| L(1, chi_D) / pi`, with `chi_D(n) = (D/n)`.

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Jacobi symbol `(a/n)` for odd `n > 0`.
pub fn jacobi_u64(a: u64, n: u64) -> i32 {
    debug_assert!(n & 1 == 1);
    let (mut a, mut n) = (a % n, n);
    let mut t = 1;
    while a != 0 {
        let tz = a.trailing_zeros();
        a >>= tz;
        if tz & 1 == 1 {
            let r = n & 7;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a & 3 == 3 && n & 3 == 3 {
            t = -t;
        }
        a %= n;
    }
    if n == 1 {
        t
    } else {
        0
    }
}

/// Jacobi symbol `(a/n)` for odd `n > 0` with big operands.
pub fn jacobi_big(a: &BigUint, n: &BigUint) -> i32 {
    debug_assert!(n.is_odd());
    let mut a = a % n;
    let mut n = n.clone();
    let mut t = 1;
    while !a.is_zero() {
        let tz = a.trailing_zeros().unwrap_or(0);
        a >>= tz;
        if tz & 1 == 1 {
            let r = (&n % 8u32).to_u32().unwrap_or(0);
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if (&a % 4u32) == BigUint::from(3u32) && (&n % 4u32) == BigUint::from(3u32) {
            t = -t;
        }
        a = &a % &n;
    }
    if n.is_one() {
        t
    } else {
        0
    }
}

/// Kronecker symbol `(d/n)` for `n >= 1`.
pub fn kronecker_i64(d: i64, n: u64) -> i32 {
    assert!(n >= 1);
    let k = n.trailing_zeros();
    let m = n >> k;
    let mut r = 1;
    if k > 0 {
        let c = if d & 1 == 0 {
            0
        } else {
            match d.rem_euclid(8) {
                1 | 7 => 1,
                _ => -1,
            }
        };
        for _ in 0..k {
            r *= c;
        }
    }
    if m > 1 {
        r *= jacobi_u64(d.rem_euclid(m as i64) as u64, m);
    }
    r
}

/// Is `d < 0` a fundamental discriminant?
pub fn is_fundamental_negative(d: i64) -> bool {
    if d >= 0 {
        return false;
    }
    let sqfree = |mut m: u64| {
        let mut p = 2u64;
        while p * p <= m {
            if m.is_multiple_of(p * p) {
                return false;
            }
            if m.is_multiple_of(p) {
                m /= p;
            }
            p += 1;
        }
        true
    };
    let a = d.unsigned_abs();
    match d.rem_euclid(4) {
        1 => sqfree(a),
        0 => {
            let m = d / 4;
            matches!(m.rem_euclid(4), 2 | 3) && sqfree(m.unsigned_abs())
        }
        _ => false,
    }
}

/// Exact `h(D)` for a negative fundamental `D` by counting reduced primitive
/// forms. `O(|D|)`; a test oracle, not a method for large `D`.
pub fn exact_class_number(d: i64) -> u64 {
    assert!(d < 0 && d.rem_euclid(4) <= 1);
    let abs = d.unsigned_abs();
    let par = abs & 1; // b has the parity of D
    let mut count = 0u64;
    let mut a = 1u64;
    while 3 * a * a <= abs {
        // -a < b <= a, b ≡ par (mod 2)
        let mut b = -(a as i64) + 1;
        if (b.rem_euclid(2) as u64) != par {
            b += 1;
        }
        while b <= a as i64 {
            let num = (b * b + abs as i64) as u64; // b^2 - D = 4ac
            if num.is_multiple_of(4 * a) {
                let c = num / (4 * a);
                if c >= a && (c > a || b >= 0) && gcd3(a, b.unsigned_abs(), c) == 1 {
                    count += 1;
                }
            }
            b += 2;
        }
        a += 1;
    }
    count
}

fn gcd3(a: u64, b: u64, c: u64) -> u64 {
    a.gcd(&b).gcd(&c)
}

/// Complementary error function, accurate to about 1e-14 relative for
/// `0 <= x <= 7`.
pub fn erfc(x: f64) -> f64 {
    assert!(x >= 0.0);
    if x < 2.0 {
        // erf(x) = 2/sqrt(pi) * sum (-1)^k x^(2k+1) / (k! (2k+1))
        let mut term = x;
        let mut sum = x;
        let mut k = 0.0f64;
        loop {
            k += 1.0;
            term *= -x * x / k;
            let add = term / (2.0 * k + 1.0);
            sum += add;
            if add.abs() < 1e-18 * sum.abs().max(1e-300) || k > 200.0 {
                break;
            }
        }
        1.0 - 2.0 / std::f64::consts::PI.sqrt() * sum
    } else {
        // erfc(x) = e^{-x^2}/sqrt(pi) / (x + (1/2)/(x + 1/(x + (3/2)/(x + ...))))
        let mut f = x;
        for k in (1..=80).rev() {
            f = x + (k as f64 / 2.0) / f;
        }
        (-x * x).exp() / std::f64::consts::PI.sqrt() / f
    }
}

/// `L(1, chi_D)` for a small negative fundamental `D` from the rapidly
/// convergent series (functional-equation / theta form, the series Cohen
/// uses for class numbers):
/// `L(1,chi) = sum chi(n) e^{-pi n^2/|D|}/n + (pi/sqrt|D|) sum chi(n) erfc(n sqrt(pi/|D|))`.
/// The terms are below `1e-17` once `n > sqrt(12.4 |D|)`, so the cost is
/// `O(sqrt|D|)` terms: **exact to double precision, but only for small `D`**.
/// Returns `(L, number of terms)`.
pub fn l1_theta_series(d: i64) -> (f64, u64) {
    assert!(d < 0);
    let abs = d.unsigned_abs() as f64;
    let a = std::f64::consts::PI / abs;
    let nmax = (12.4 * abs).sqrt().ceil() as u64 + 1;
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    for n in 1..=nmax {
        let c = kronecker_i64(d, n) as f64;
        if c == 0.0 {
            continue;
        }
        let nf = n as f64;
        s1 += c * (-a * nf * nf).exp() / nf;
        s2 += c * erfc(nf * a.sqrt());
    }
    (s1 + std::f64::consts::PI / abs.sqrt() * s2, nmax)
}

/// `h` from `L(1, chi)` for `D < -4` (`w = 2`).
pub fn h_from_l1(abs_d: f64, l1: f64) -> f64 {
    abs_d.sqrt() * l1 / std::f64::consts::PI
}

/// Natural log of a big unsigned integer.
pub fn ln_big(x: &BigUint) -> f64 {
    let bits = x.bits();
    if bits <= 63 {
        return (x.to_u64().unwrap_or(0) as f64).ln();
    }
    let shift = bits - 64;
    let top = (x >> shift).to_u64().unwrap_or(0);
    (top as f64).ln() + shift as f64 * std::f64::consts::LN_2
}

/// A negative fundamental discriminant `D = -abs`, prepared for the sweep.
#[derive(Clone, Debug)]
pub struct NegDisc {
    pub abs: BigUint,
    pub ln_abs: f64,
    /// `chi(2)`: 0 for even `D`, else by `D mod 8`.
    pub chi2: i32,
}

impl NegDisc {
    /// `abs` must be `3 (mod 4)` (so `D ≡ 1 mod 4`) or `0 mod 4`.
    pub fn new(abs: BigUint) -> Result<Self, String> {
        let m4 = (&abs % 4u32).to_u32().unwrap_or(9);
        if m4 == 1 || m4 == 2 {
            return Err(format!("-abs is not a discriminant (abs mod 4 = {m4})"));
        }
        let chi2 = if m4 == 0 {
            0
        } else {
            // D = -abs mod 8
            let r = (&abs % 8u32).to_u32().unwrap_or(0);
            match (8 - r) % 8 {
                1 => 1,
                5 => -1,
                x => return Err(format!("odd D with D mod 8 = {x}")),
            }
        };
        Ok(NegDisc {
            ln_abs: ln_big(&abs),
            abs,
            chi2,
        })
    }

    pub fn from_i64(d: i64) -> Self {
        assert!(d < 0);
        Self::new(BigUint::from(d.unsigned_abs())).expect("discriminant")
    }

    /// `ln h(D) = (1/2) ln|D| - ln pi + ln L(1,chi)` (`w = 2`).
    pub fn ln_h(&self, ln_l: f64) -> f64 {
        0.5 * self.ln_abs - std::f64::consts::PI.ln() + ln_l
    }
}

/// One checkpoint of the Euler-product sweep (cumulative over `p <= bound`).
#[derive(Clone, Debug)]
pub struct Checkpoint {
    pub bound: u64,
    /// `ln prod_{p<=bound} (1 - chi(p)/p)^{-1}`.
    pub ln_l: f64,
    /// `sum_{p<=bound} chi(p) ln p`.
    pub s_chi: f64,
    pub n_split: u64,
    pub n_inert: u64,
    pub n_ramified: u64,
}

#[derive(Clone, Copy, Default)]
struct Acc {
    ln_l: f64,
    s_chi: f64,
    np: u64,
    nm: u64,
    nz: u64,
}

const SEG: u64 = 1 << 22;

fn base_primes(limit: u64) -> Vec<u64> {
    let n = limit as usize + 1;
    let mut sieve = vec![true; n.max(2)];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 2usize;
    while i * i < n {
        if sieve[i] {
            let mut j = i * i;
            while j < n {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..n).filter(|&i| sieve[i]).map(|i| i as u64).collect()
}

/// The Euler-product sweep: `chi_D(p)` for every prime `p <= bounds.last()`,
/// cumulative values at each of `bounds` (ascending, first `>= 2`). The result
/// is deterministic (segments are merged in order).
pub fn euler_sweep(d: &NegDisc, bounds: &[u64], threads: usize) -> Vec<Checkpoint> {
    assert!(!bounds.is_empty() && bounds[0] >= 2 && bounds.windows(2).all(|w| w[0] < w[1]));
    let bmax = *bounds.last().unwrap();
    let nb = bounds.len();
    let bits_b = 64 - bmax.leading_zeros();
    let w = (63 - bits_b).clamp(8, 32);
    // limbs of |D| in base 2^w, little endian
    let limbs: Vec<u64> = {
        let mask = (BigUint::one() << w) - 1u32;
        let mut x = d.abs.clone();
        let mut v = Vec::new();
        while !x.is_zero() {
            v.push((&x & &mask).to_u64().unwrap_or(0));
            x >>= w;
        }
        v
    };
    let base = base_primes(((bmax as f64).sqrt() as u64) + 2);
    let nseg = (bmax + 1).div_ceil(SEG) as usize;
    let next = AtomicUsize::new(0);

    let work = || {
        let mut out: Vec<(usize, Vec<Acc>)> = Vec::new();
        let mut bitmap: Vec<u8> = Vec::new();
        loop {
            let k = next.fetch_add(1, Ordering::Relaxed);
            if k >= nseg {
                break;
            }
            let lo = k as u64 * SEG;
            let hi = ((k as u64 + 1) * SEG).min(bmax + 1);
            let cnt = ((hi - lo) / 2) as usize;
            bitmap.clear();
            bitmap.resize(cnt, 0);
            for &q in base.iter().skip(1) {
                if q * q >= hi {
                    break;
                }
                let mut s = (lo.div_ceil(q) * q).max(q * q);
                if s & 1 == 0 {
                    s += q;
                }
                let mut i = ((s - lo - 1) / 2) as usize;
                while i < cnt {
                    bitmap[i] = 1;
                    i += q as usize;
                }
            }
            let mut acc = vec![Acc::default(); nb];
            let mut bk = 0usize;
            for (i, &flag) in bitmap.iter().enumerate() {
                if flag != 0 {
                    continue;
                }
                let n = lo + 2 * i as u64 + 1;
                if n == 1 {
                    continue;
                }
                while n > bounds[bk] {
                    bk += 1;
                }
                // |D| mod n
                let mut r = 0u64;
                for &limb in limbs.iter().rev() {
                    r = ((r << w) | limb) % n;
                }
                let a = &mut acc[bk];
                if r == 0 {
                    a.nz += 1;
                    continue;
                }
                let chi = jacobi_u64(n - r, n);
                let inv = 1.0 / n as f64;
                if chi == 1 {
                    a.ln_l -= (-inv).ln_1p();
                    a.np += 1;
                    a.s_chi += (n as f64).ln();
                } else {
                    a.ln_l -= inv.ln_1p();
                    a.nm += 1;
                    a.s_chi -= (n as f64).ln();
                }
            }
            out.push((k, acc));
        }
        out
    };

    let mut parts: Vec<(usize, Vec<Acc>)> = if threads <= 1 {
        work()
    } else {
        std::thread::scope(|s| {
            let hs: Vec<_> = (0..threads).map(|_| s.spawn(work)).collect();
            hs.into_iter()
                .flat_map(|h| h.join().expect("sweep thread"))
                .collect()
        })
    };
    parts.sort_by_key(|p| p.0);

    // prime 2 sits in bucket 0
    let mut total = vec![Acc::default(); nb];
    let ln2 = std::f64::consts::LN_2;
    match d.chi2 {
        1 => {
            total[0].ln_l += ln2;
            total[0].np += 1;
            total[0].s_chi += ln2;
        }
        -1 => {
            total[0].ln_l -= (0.5f64).ln_1p();
            total[0].nm += 1;
            total[0].s_chi -= ln2;
        }
        _ => total[0].nz += 1,
    }
    let mut comp = vec![0.0f64; nb];
    for (_, acc) in parts {
        for (b, a) in acc.iter().enumerate() {
            // Kahan-compensated accumulation of the per-segment partials
            let y = a.ln_l - comp[b];
            let t = total[b].ln_l + y;
            comp[b] = (t - total[b].ln_l) - y;
            total[b].ln_l = t;
            total[b].s_chi += a.s_chi;
            total[b].np += a.np;
            total[b].nm += a.nm;
            total[b].nz += a.nz;
        }
    }
    let mut out = Vec::with_capacity(nb);
    let mut c = Acc::default();
    for (b, t) in total.iter().enumerate() {
        c.ln_l += t.ln_l;
        c.s_chi += t.s_chi;
        c.np += t.np;
        c.nm += t.nm;
        c.nz += t.nz;
        out.push(Checkpoint {
            bound: bounds[b],
            ln_l: c.ln_l,
            s_chi: c.s_chi,
            n_split: c.np,
            n_inert: c.nm,
            n_ramified: c.nz,
        });
    }
    out
}

/// **Heuristic** one-sigma size of the tail `ln L - ln L_B`: the signs
/// `chi(p)` for `p > B` modelled as independent fair signs, so the sum
/// `sum_{p>B} chi(p)/p` has variance `sum_{p>B} p^-2 ~ 1/(B ln B)`.
pub fn heuristic_tail_sigma(bound: f64) -> f64 {
    1.0 / (bound * bound.ln()).sqrt()
}

/// **GRH-conditional** bound on `|ln L(1,chi) - ln L_B|`.
///
/// Assumption (named, not proved here): for all `x >= bound`,
/// `|sum_{n<=x} Lambda(n) chi(n)| <= c sqrt(x) ln^2(|D| x)`. Writing
/// `F(t) = c sqrt(t) ln^2(|D| t) + 1.2 sqrt(t)` (the second term covers
/// prime powers, using `psi(t) - theta(t) < 1.2 sqrt(t)` for `t >= 1000`),
/// partial summation gives
/// `|sum_{p>B} chi(p)/p| <= F(B)/(B ln B) + int_B^inf F(t)(ln t + 1)/(t^2 ln^2 t) dt`
/// (evaluated here by Simpson's rule in `u = ln(t/B)` out to `u = 120`, where
/// the integrand is below `e^-60`), and the Euler-factor remainder
/// `sum_{p>B} |-ln(1-u) - u| <= 1/B`.
pub fn grh_tail_bound(ln_abs_d: f64, bound: f64, c: f64) -> f64 {
    assert!(bound >= 1000.0);
    let f = |t: f64| {
        let l = ln_abs_d + t.ln();
        (c * l * l + 1.2) * t.sqrt()
    };
    let boundary = f(bound) / (bound * bound.ln());
    let g = |u: f64| {
        let t = bound * u.exp();
        let lt = t.ln();
        f(t) * (lt + 1.0) / (t * lt * lt)
    };
    let (u_max, steps) = (120.0f64, 24000usize);
    let hstep = u_max / steps as f64;
    let mut s = g(0.0) + g(u_max);
    for i in 1..steps {
        s += g(i as f64 * hstep) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    boundary + s * hstep / 3.0 + 1.0 / bound
}

/// `(mantissa, exponent)` with `x = mantissa * 10^exponent` for `x = e^ln_x`.
pub fn sci_from_ln(ln_x: f64) -> (f64, i64) {
    let l10 = ln_x / std::f64::consts::LN_10;
    let e = l10.floor();
    (10f64.powf(l10 - e), e as i64)
}

// ---------------------------------------------------------------------------
// Genus theory and the Rédei 4-rank (exact)
// ---------------------------------------------------------------------------

/// Genus-theory data of a negative fundamental discriminant (exact).
#[derive(Clone, Debug)]
pub struct GenusData {
    /// Number of prime discriminants (distinct primes dividing `D`, the prime 2
    /// counted when `D` is even).
    pub t: usize,
    /// 2-rank of the class group, `t - 1`.
    pub two_rank: usize,
    /// Rank of the Rédei matrix over F2.
    pub redei_rank: usize,
    /// 4-rank, `t - 1 - redei_rank`.
    pub four_rank: usize,
}

fn kron_disc_prime(dj: &BigInt, p: &BigUint) -> i32 {
    if *p == BigUint::from(2u32) {
        let r = dj.mod_floor(&BigInt::from(8)).to_i32().unwrap_or(0);
        return match r {
            1 | 7 => 1,
            3 | 5 => -1,
            _ => 0,
        };
    }
    let r = dj.mod_floor(&BigInt::from_biguint(Sign::Plus, p.clone()));
    jacobi_big(&r.to_biguint().unwrap_or_default(), p)
}

/// Genus data of `D = -abs` given its distinct odd prime factors (and `D`
/// itself, which fixes the even part). Errors if `D` is not the product of
/// prime discriminants times one of `1, -4, 8, -8`.
pub fn genus_data(d: &BigInt, odd_primes: &[BigUint]) -> Result<GenusData, String> {
    let mut discs: Vec<(BigUint, BigInt)> = Vec::new();
    let mut prod = BigInt::one();
    for p in odd_primes {
        let sign = if (p % 4u32) == BigUint::one() { 1 } else { -1 };
        let pd = BigInt::from_biguint(Sign::Plus, p.clone()) * sign;
        prod *= &pd;
        discs.push((p.clone(), pd));
    }
    let (q, r) = d.div_rem(&prod);
    if !r.is_zero() {
        return Err("prime list does not divide D".into());
    }
    let e = q.to_i32().unwrap_or(0);
    match e {
        1 => {}
        -4 | 8 | -8 => discs.push((BigUint::from(2u32), q.clone())),
        _ => {
            return Err(format!(
                "D / prod(prime discriminants) = {q}, not 1, -4, ±8"
            ))
        }
    }
    let t = discs.len();
    let mut rows = vec![0u128; t];
    assert!(t <= 128);
    for i in 0..t {
        let mut diag = 0u32;
        for j in 0..t {
            if i == j {
                continue;
            }
            let k = kron_disc_prime(&discs[j].1, &discs[i].0);
            if k == 0 {
                return Err("non-coprime prime discriminants".into());
            }
            if k == -1 {
                rows[i] |= 1u128 << j;
                diag ^= 1;
            }
        }
        if diag == 1 {
            rows[i] |= 1u128 << i;
        }
    }
    let rank = f2_rank(&mut rows);
    let two_rank = t.saturating_sub(1);
    Ok(GenusData {
        t,
        two_rank,
        redei_rank: rank,
        four_rank: two_rank.saturating_sub(rank),
    })
}

fn f2_rank(rows: &mut [u128]) -> usize {
    let mut rank = 0;
    for bit in 0..128 {
        let Some(pivot) = (rank..rows.len()).find(|&i| rows[i] >> bit & 1 == 1) else {
            continue;
        };
        rows.swap(rank, pivot);
        for i in 0..rows.len() {
            if i != rank && rows[i] >> bit & 1 == 1 {
                rows[i] ^= rows[rank];
            }
        }
        rank += 1;
    }
    rank
}

/// Distinct odd prime factors of a small `|d|` (trial division).
pub fn odd_prime_factors_small(d: i64) -> Vec<BigUint> {
    let mut m = d.unsigned_abs();
    while m.is_multiple_of(2) {
        m /= 2;
    }
    let mut out = Vec::new();
    let mut p = 3u64;
    while p * p <= m {
        if m.is_multiple_of(p) {
            out.push(BigUint::from(p));
            while m.is_multiple_of(p) {
                m /= p;
            }
        }
        p += 2;
    }
    if m > 1 {
        out.push(BigUint::from(m));
    }
    out
}

// ---------------------------------------------------------------------------
// CM-field inputs
// ---------------------------------------------------------------------------

/// A CM field input: `t^2 - 4p = -conductor^2 * prod(factors)` is enforced.
#[derive(Clone, Debug)]
pub struct CmInput {
    pub name: &'static str,
    pub p: BigUint,
    pub t: BigUint,
    pub conductor: u32,
    pub factors: Vec<BigUint>,
}

/// Checks the identity, the discriminant shape and the primality (strong
/// probable-prime, 12 bases) of every factor; returns `|D|`.
pub fn verify_cm_input(c: &CmInput) -> Result<BigUint, String> {
    let abs = c.factors.iter().fold(BigUint::one(), |a, f| a * f);
    let f2 = BigUint::from(c.conductor) * BigUint::from(c.conductor);
    let lhs = BigInt::from_biguint(Sign::Plus, &c.t * &c.t)
        - BigInt::from_biguint(Sign::Plus, BigUint::from(4u32) * &c.p);
    let rhs = -BigInt::from_biguint(Sign::Plus, &f2 * &abs);
    if lhs != rhs {
        return Err(format!("{}: t^2 - 4p != -f^2 * prod(factors)", c.name));
    }
    for f in &c.factors {
        if !super::curve_construction::weil::is_probable_prime(f) {
            return Err(format!(
                "{}: factor {f} fails the probable-prime test",
                c.name
            ));
        }
    }
    let mut seen = c.factors.clone();
    seen.sort();
    seen.dedup();
    if seen.len() != c.factors.len() {
        return Err("repeated factor: D not squarefree".into());
    }
    Ok(abs)
}

/// Absolute value helper for signed big integers.
pub fn big_abs(x: &BigInt) -> BigUint {
    x.abs().to_biguint().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_class_numbers_by_forms() {
        let table: &[(i64, u64)] = &[
            (-3, 1),
            (-4, 1),
            (-7, 1),
            (-8, 1),
            (-11, 1),
            (-15, 2),
            (-20, 2),
            (-23, 3),
            (-31, 3),
            (-39, 4),
            (-47, 5),
            (-71, 7),
            (-79, 5),
            (-84, 4),
            (-87, 6),
            (-95, 8),
            (-103, 5),
            (-127, 5),
            (-151, 7),
            (-163, 1),
            (-199, 9),
            (-239, 15),
            (-427, 2),
        ];
        for &(d, h) in table {
            assert!(is_fundamental_negative(d), "{d}");
            assert_eq!(exact_class_number(d), h, "h({d})");
        }
    }

    #[test]
    fn theta_series_matches_exact_h() {
        for d in (-3000i64..-4).filter(|&d| is_fundamental_negative(d)) {
            let (l, _) = l1_theta_series(d);
            let h = h_from_l1(d.unsigned_abs() as f64, l);
            let hx = exact_class_number(d) as f64;
            assert!((h - hx).abs() < 1e-8 * hx, "D={d}: theta {h} vs {hx}");
        }
    }

    #[test]
    fn erfc_values() {
        assert!((erfc(0.0) - 1.0).abs() < 1e-15);
        assert!((erfc(0.5) - 0.4795001221869535).abs() < 1e-14);
        assert!((erfc(1.0) - 0.15729920705028513).abs() < 1e-14);
        assert!((erfc(2.0) - 0.004677734981047266).abs() < 1e-15);
        assert!((erfc(3.0) - 2.209049699858544e-5).abs() < 1e-18);
    }

    #[test]
    fn sweep_agrees_with_direct_kronecker() {
        for d in [-23i64, -47, -163, -419, -1_000_003, -8 * 5] {
            if !is_fundamental_negative(d) {
                continue;
            }
            let nd = NegDisc::from_i64(d);
            let bound = 5000u64;
            let cp = euler_sweep(&nd, &[bound], 1);
            let mut ln_l = 0.0;
            for p in base_primes(bound) {
                let c = kronecker_i64(d, p) as f64;
                ln_l -= (1.0 - c / p as f64).ln();
            }
            assert!((cp[0].ln_l - ln_l).abs() < 1e-12, "D={d}");
        }
    }

    #[test]
    fn sweep_is_thread_count_independent() {
        let nd = NegDisc::from_i64(-1_000_003);
        let b = [1000u64, 10_000, 3_000_000];
        let a = euler_sweep(&nd, &b, 1);
        let c = euler_sweep(&nd, &b, 3);
        for (x, y) in a.iter().zip(&c) {
            assert_eq!(x.ln_l, y.ln_l);
            assert_eq!((x.n_split, x.n_inert), (y.n_split, y.n_inert));
        }
    }

    #[test]
    fn euler_product_close_to_h() {
        for d in [-4003i64, -20_011, -100_003, -999_983] {
            if !is_fundamental_negative(d) {
                continue;
            }
            let nd = NegDisc::from_i64(d);
            let cp = euler_sweep(&nd, &[1_000_000], 1).remove(0);
            let est = nd.ln_h(cp.ln_l).exp();
            let hx = exact_class_number(d) as f64;
            assert!((est / hx - 1.0).abs() < 0.015, "D={d}: {est} vs {hx}");
        }
    }

    #[test]
    fn intervals_cover_exact_h() {
        // positive control for the stated intervals: GRH band (c = 1/pi) and
        // heuristic 3-sigma band must contain the exact h
        let mut tested = 0;
        let mut d = -200_001i64;
        while tested < 40 {
            d -= 2;
            if !is_fundamental_negative(d) {
                continue;
            }
            tested += 1;
            let nd = NegDisc::from_i64(d);
            let bound = 100_000u64;
            let cp = euler_sweep(&nd, &[bound], 1).remove(0);
            let hx = (exact_class_number(d) as f64).ln();
            let mid = nd.ln_h(cp.ln_l);
            let g = grh_tail_bound(nd.ln_abs, bound as f64, 1.0 / std::f64::consts::PI);
            assert!((hx - mid).abs() <= g, "D={d} GRH band");
            assert!(
                (hx - mid).abs() <= 3.0 * heuristic_tail_sigma(bound as f64),
                "D={d}: {hx} vs {mid}"
            );
        }
    }

    #[test]
    fn genus_two_rank_matches_class_number_parity() {
        let mut positive_four_rank = 0;
        for d in (-3000i64..-4).filter(|&d| is_fundamental_negative(d)) {
            let g = genus_data(&BigInt::from(d), &odd_prime_factors_small(d)).expect("genus");
            let h = exact_class_number(d);
            let v2 = h.trailing_zeros() as usize;
            assert!(v2 >= g.two_rank, "D={d}: 2^(t-1) must divide h");
            if g.four_rank == 0 {
                assert_eq!(v2, g.two_rank, "D={d}: 4-rank 0 => elementary 2-part");
            } else {
                positive_four_rank += 1;
                assert!(v2 >= g.two_rank + g.four_rank, "D={d}: 4-rank bound");
            }
        }
        assert!(positive_four_rank > 20, "control would be vacuous");
    }

    #[test]
    fn jacobi_big_matches_small() {
        for n in (3u64..200).step_by(2) {
            for a in 0u64..300 {
                assert_eq!(
                    jacobi_big(&BigUint::from(a), &BigUint::from(n)),
                    jacobi_u64(a, n)
                );
            }
        }
    }

    #[test]
    fn rejects_bad_cm_input() {
        let c = CmInput {
            name: "bad",
            p: BigUint::from(7u32),
            t: BigUint::from(1u32),
            conductor: 1,
            factors: vec![BigUint::from(3u32)],
        };
        assert!(verify_cm_input(&c).is_err());
        // t^2 - 4p = 1 - 28 = -27 = -3^2 * 3: conductor 3, factor 3
        let ok = CmInput { conductor: 3, ..c };
        assert_eq!(verify_cm_input(&ok).unwrap(), BigUint::from(3u32));
    }
}
