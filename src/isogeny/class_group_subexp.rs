//! Subexponential class-group computation for imaginary quadratic orders
//! (Hafner–McCurley / Buchmann style), in native Rust.
//!
//! Given a negative discriminant `D` (fundamental in all callers here) this
//! module computes, under GRH, the class number `h(D)`, the invariant factors
//! of `Cl(D)` and the discrete logarithm (in the Smith-normal-form
//! coordinates) of every factor-base prime ideal.  It replaces PARI's
//! `bnfinit` for the isogeny-loop sweeper (`examples/isogeny_loop_sweeper.rs`).
//!
//! # Method
//!
//! 1. **Factor base.**  Every prime `p ≤ B` with `(D/p) ≠ −1`.  The default
//!    `B` is Bach's GRH bound `6 log²|D|` for quadratic fields (E. Bach,
//!    *Explicit bounds for primality testing and related problems*, Math.
//!    Comp. 55 (1990), 355–380): under GRH the prime ideals of norm at most
//!    `6 log²|D|` generate `Cl(D)`, so the factor base generates and the
//!    lattice spanned by any set of true relations has determinant a positive
//!    multiple of `h` once it has full rank.  Prime `p` is represented by
//!    the prime form `(p, b_p, c_p)` with `b_p² ≡ D (mod 4p)`, i.e. the
//!    ideal `𝔭 = (p, (−b_p + √D)/2)`.
//! 2. **Relations.**  Random products `F = ∏ g_i^{±1}` of a few factor-base
//!    forms are composed with one more factor-base form `g_j^{±1}`, the result
//!    is reduced to `(a, b, c)` and `a` is trial-divided over the factor base.
//!    A `B`-smooth `a = ∏ p^{v_p}` gives the relation
//!    `Σ e_i [𝔭_i] − Σ_p ±v_p [𝔭_p] = 0`, the sign of `p` being read from
//!    `b mod p` (`b mod 4` for `p = 2`): the reduced form `(a, b, c)` is the
//!    ideal `(a, (−b + √D)/2)`, whose `p`-part is `𝔭^{v_p}` when `b ≡ b_p`
//!    and `𝔭̄^{v_p}` when `b ≡ −b_p`.  One large prime `P ≤ lp_factor · B`
//!    is allowed; two partial relations sharing `P` are combined.  Ramified
//!    primes get the explicit relation `2[𝔭] = 0`.
//! 3. **Linear algebra.**  Structured Gaussian elimination over `Z` with
//!    `±1` pivots only (unimodular, so the quotient `Z^n / L` is unchanged),
//!    then an exact Hermite normal form (BigInt) of the remaining dense block,
//!    `h* = det`, and a Smith normal form with the column transform tracked
//!    modulo `h*`, which gives the invariant factors and the discrete logs of
//!    the surviving columns.  Eliminated columns are recovered by
//!    back-substitution through their pivot rows.
//! 4. **Stopping and verification.**  Batches of new relations are added until
//!    `h*` has not changed for `stable_rounds` consecutive batches *and* the
//!    analytic estimate agrees: `h_approx = w √|D| L_P(1, χ_D) / (2π)` with
//!    the truncated Euler product `L_P = ∏_{p ≤ P} (1 − χ_D(p)/p)^{-1}`, and
//!    acceptance requires `|log(h*/h_approx)| < log √2`.  Because `h*` is a
//!    multiple of `h`, a lattice of index 2 or more would show up as a ratio
//!    near 2 or above.  **The error bound used is this tolerance
//!    (`log √2 ≈ 0.347`), which is not a proven bound**: a proven GRH bound
//!    on `|log L(1,χ) − log L_P(1,χ)|` (Bach 1995, *Improved approximations
//!    for Euler products*) is not implemented here.  The observed deviations
//!    are recorded by the callers (tests: all fundamental `|D| < 20000`;
//!    sweeper: every curve).  Finally, freshly generated held-out relations
//!    must have discrete-log sums `≡ 0` modulo the invariant factors.
//!
//! Assumption level: GRH (the same as PARI's `bnfinit` without
//! `bnfcertify`), plus the analytic tolerance stated above.

use super::class_group::BinaryQuadraticForm;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

type Bqf = BinaryQuadraticForm;
/// A sparse relation: sorted `(column, coefficient)` pairs.
pub type Relation = Vec<(u32, i64)>;

// ───────────────────────────── small number theory ─────────────────────────────

/// Primes `≤ n` by the sieve of Eratosthenes.
pub fn primes_up_to(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }
    let n = n as usize;
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 2;
    while i * i <= n {
        if sieve[i] {
            let mut j = i * i;
            while j <= n {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    sieve
        .iter()
        .enumerate()
        .filter(|(_, &b)| b)
        .map(|(k, _)| k as u64)
        .collect()
}

/// `x mod p` in `[0, p)` for a BigInt `x`.
pub fn mod_u64(x: &BigInt, p: u64) -> u64 {
    x.mod_floor(&BigInt::from(p)).to_u64().expect("residue fits")
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, m);
        }
        b = mulmod(b, b, m);
        e >>= 1;
    }
    r
}

/// Jacobi symbol `(a/n)` for odd `n > 0`.
pub fn jacobi(mut a: u64, mut n: u64) -> i32 {
    debug_assert!(n % 2 == 1);
    a %= n;
    let mut t = 1;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let r = n % 8;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
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

/// Kronecker symbol `(D/p)` for a prime `p`.
pub fn kronecker_prime(d: &BigInt, p: u64) -> i32 {
    if p == 2 {
        let r = mod_u64(d, 8);
        return match r {
            1 | 7 => 1,
            3 | 5 => -1,
            _ => 0,
        };
    }
    jacobi(mod_u64(d, p), p)
}

/// Square root of a quadratic residue `a` modulo an odd prime `p`
/// (Tonelli–Shanks).
pub fn sqrt_mod_prime(a: u64, p: u64) -> u64 {
    let a = a % p;
    if a == 0 {
        return 0;
    }
    if p % 4 == 3 {
        return powmod(a, (p + 1) / 4, p);
    }
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while jacobi(z, p) != -1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(a, q, p);
    let mut r = powmod(a, q.div_ceil(2), p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulmod(tt, tt, p);
            i += 1;
        }
        let b = powmod(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        r = mulmod(r, b, p);
    }
    r
}

/// Extended gcd: `(g, x, y)` with `x a + y b = g ≥ 0`.
pub fn ext_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut r0, mut r1) = (a.clone(), b.clone());
    let (mut s0, mut s1) = (BigInt::one(), BigInt::zero());
    let (mut t0, mut t1) = (BigInt::zero(), BigInt::one());
    while !r1.is_zero() {
        let q = r0.div_floor(&r1);
        let r2 = &r0 - &q * &r1;
        r0 = std::mem::replace(&mut r1, r2);
        let s2 = &s0 - &q * &s1;
        s0 = std::mem::replace(&mut s1, s2);
        let t2 = &t0 - &q * &t1;
        t0 = std::mem::replace(&mut t1, t2);
    }
    if r0.is_negative() {
        (-r0, -s0, -t0)
    } else {
        (r0, s0, t0)
    }
}

// ───────────────────────────── form arithmetic ─────────────────────────────

/// Gauss composition (Cohen, GTM 138, Algorithm 5.4.7) **without** the final
/// reduction.  For invertible ideals `𝔞₁ = (a₁, (−b₁+√D)/2)`,
/// `𝔞₂ = (a₂, (−b₂+√D)/2)` the result is the form of `𝔞₁𝔞₂ / d` with
/// `d = gcd(a₁, a₂, (b₁+b₂)/2)`; when no rational prime divides both ideals
/// (`d = 1`) it is exactly the product ideal.
pub fn compose_unreduced(f: &Bqf, g: &Bqf, disc: &BigInt) -> Bqf {
    let (f1, f2) = if f.a <= g.a { (f, g) } else { (g, f) };
    let (a1, b1) = (&f1.a, &f1.b);
    let (a2, b2, c2) = (&f2.a, &f2.b, &f2.c);
    let s: BigInt = (b1 + b2) / 2;
    let n: BigInt = b2 - &s;
    let (d, y1) = if a2.mod_floor(a1).is_zero() {
        (a1.clone(), BigInt::zero())
    } else {
        let (d, u, _v) = ext_gcd(a2, a1);
        (d, u)
    };
    let (d1, x2, y2) = if s.mod_floor(&d).is_zero() {
        (d.clone(), BigInt::zero(), BigInt::from(-1))
    } else {
        let (d1, x2, y2) = ext_gcd(&s, &d);
        (d1, x2, -y2)
    };
    let v1 = a1 / &d1;
    let v2 = a2 / &d1;
    let r = (&y1 * &y2 * &n - &x2 * c2).mod_floor(&v1);
    let b3: BigInt = b2 + &v2 * &r * 2;
    let a3 = &v1 * &v2;
    let num = &b3 * &b3 - disc;
    let den = &a3 * 4;
    debug_assert!(num.mod_floor(&den).is_zero());
    let c3 = num / den;
    Bqf {
        a: a3,
        b: b3,
        c: c3,
    }
}

/// Composition followed by reduction.
pub fn compose_reduce(f: &Bqf, g: &Bqf, disc: &BigInt) -> Bqf {
    compose_unreduced(f, g, disc).reduce()
}

/// Inverse class: `(a, −b, c)`, reduced.
pub fn form_inverse(f: &Bqf) -> Bqf {
    Bqf {
        a: f.a.clone(),
        b: -f.b.clone(),
        c: f.c.clone(),
    }
    .reduce()
}

/// Reduce a positive definite form and return the reduced form `g` together
/// with `M ∈ SL₂(Z)` (row-major `[m11, m12, m21, m22]`) such that
/// `g(X, Y) = f(m11 X + m12 Y, m21 X + m22 Y)`.  In particular
/// `f(m11, m21) = g.a`.
pub fn reduce_tracking(f: &Bqf) -> (Bqf, [BigInt; 4]) {
    let (mut a, mut b, mut c) = (f.a.clone(), f.b.clone(), f.c.clone());
    let mut m = [BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one()];
    // right-multiply m by [[1, k], [0, 1]]
    fn shear(m: &mut [BigInt; 4], k: &BigInt) {
        m[1] = &m[1] + &m[0] * k;
        m[3] = &m[3] + &m[2] * k;
    }
    // right-multiply m by [[0, -1], [1, 0]]
    fn rot(m: &mut [BigInt; 4]) {
        let (m0, m1, m2, m3) = (m[0].clone(), m[1].clone(), m[2].clone(), m[3].clone());
        m[0] = m1;
        m[1] = -m0;
        m[2] = m3;
        m[3] = -m2;
    }
    loop {
        let two_a = &a * 2;
        let mut r = b.mod_floor(&two_a);
        if r > a {
            r -= &two_a;
        }
        let q = (&b - &r) / &two_a;
        if !q.is_zero() {
            // f(x - q y, y)
            c = &c + &q * &q * &a - &q * &b;
            b = r;
            shear(&mut m, &(-&q));
        }
        if a > c {
            // f(-y, x) = (c, -b, a)
            std::mem::swap(&mut a, &mut c);
            b = -b;
            rot(&mut m);
            continue;
        }
        if a == c && b.is_negative() {
            b = -b;
            rot(&mut m);
        } else if a == -&b {
            // f(x + y, y) = (a, b + 2a, ...) = (a, a, c)
            b = -b;
            shear(&mut m, &BigInt::one());
        }
        break;
    }
    (Bqf { a, b, c }, m)
}

// ───────────────────────────── factor base ─────────────────────────────

/// One factor-base prime: the ideal `𝔭 = (p, (−b + √D)/2)`.
#[derive(Clone, Debug)]
pub struct FbPrime {
    pub p: u64,
    /// `b` of the prime form `(p, b, c)`, `0 ≤ b < 2p`, `b ≡ D (mod 2)`.
    pub b: BigInt,
    /// `b mod p` (odd `p`) or `b mod 4` (`p = 2`): orientation key.
    pub key: u64,
    pub ramified: bool,
    /// The prime form `(p, b, c)` (not necessarily reduced).
    pub prime_form: Bqf,
    /// Reduced form of `𝔭` and of `𝔭̄`.
    pub form: Bqf,
    pub form_inv: Bqf,
}

/// Prime form `(p, b, c)` of the ideal above `p` with the given `b`.
fn prime_form_with_b(disc: &BigInt, p: u64, b: BigInt) -> Bqf {
    let pa = BigInt::from(p);
    let c = (&b * &b - disc) / (&pa * 4);
    Bqf { a: pa, b, c }
}

/// The canonical prime ideal above a non-inert prime `p`: `b ∈ [0, 2p)`,
/// `b ≡ D (mod 2)`, `b² ≡ D (mod 4p)`, and the smallest such `b`.
/// Returns `None` for inert `p`.
pub fn fb_prime(disc: &BigInt, p: u64) -> Option<FbPrime> {
    let k = kronecker_prime(disc, p);
    if k == -1 {
        return None;
    }
    let dpar = mod_u64(disc, 2);
    let b: u64 = if p == 2 {
        let d8 = mod_u64(disc, 8);
        match d8 {
            1 => 1,
            0 => 0,
            4 => 2,
            _ => return None,
        }
    } else {
        let s = sqrt_mod_prime(mod_u64(disc, p), p);
        // candidates s, p - s, s + p, 2p - s with the right parity; smallest
        let mut best = u64::MAX;
        for cand in [s, (p - s) % p, s + p, 2 * p - s] {
            if cand < 2 * p && cand % 2 == dpar {
                let cb = BigInt::from(cand);
                if (&cb * &cb - disc).mod_floor(&BigInt::from(4 * p)).is_zero() {
                    best = best.min(cand);
                }
            }
        }
        assert!(best != u64::MAX, "no square root of D mod 4p");
        best
    };
    let bb = BigInt::from(b);
    let pf = prime_form_with_b(disc, p, bb.clone());
    let form = pf.reduce();
    let form_inv = form_inverse(&form);
    let key = if p == 2 { b % 4 } else { b % p };
    Some(FbPrime {
        p,
        b: bb,
        key,
        ramified: k == 0,
        prime_form: pf,
        form,
        form_inv,
    })
}

/// Exact-division divisibility test data for an odd prime (Granlund–Montgomery):
/// `x ≡ 0 (mod p)` iff `x · p⁻¹ mod 2¹²⁸ ≤ ⌊(2¹²⁸ − 1)/p⌋`, and then the
/// product is the exact quotient.
#[derive(Clone, Copy, Debug)]
struct Divisor {
    p: u64,
    inv: u128,
    lim: u128,
}

impl Divisor {
    fn new(p: u64) -> Self {
        let p128 = p as u128;
        let mut inv: u128 = 1;
        // Newton iteration for the inverse modulo 2^128 (p odd)
        for _ in 0..7 {
            inv = inv.wrapping_mul(2u128.wrapping_sub(p128.wrapping_mul(inv)));
        }
        debug_assert_eq!(p128.wrapping_mul(inv), 1);
        Divisor {
            p,
            inv,
            lim: u128::MAX / p128,
        }
    }
}

/// Factor-base context for one discriminant.
pub struct FactorBase {
    pub disc: BigInt,
    pub primes: Vec<FbPrime>,
    pub bound: u64,
    index: HashMap<u64, usize>,
    divisors: Vec<Option<Divisor>>,
    two_index: Option<usize>,
}

impl FactorBase {
    pub fn new(disc: &BigInt, bound: u64) -> Self {
        let mut primes = Vec::new();
        for p in primes_up_to(bound) {
            if let Some(fp) = fb_prime(disc, p) {
                primes.push(fp);
            }
        }
        let index = primes.iter().enumerate().map(|(i, f)| (f.p, i)).collect();
        let divisors = primes
            .iter()
            .map(|f| if f.p == 2 { None } else { Some(Divisor::new(f.p)) })
            .collect();
        let two_index = primes.iter().position(|f| f.p == 2);
        FactorBase {
            disc: disc.clone(),
            primes,
            bound,
            index,
            divisors,
            two_index,
        }
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.primes.is_empty()
    }

    pub fn index_of(&self, p: u64) -> Option<usize> {
        self.index.get(&p).copied()
    }

    /// Orientation sign of the `p`-part of the reduced form with middle
    /// coefficient `b` (as `i128`): `+1` for `𝔭`, `−1` for `𝔭̄`.
    fn orientation(&self, idx: usize, b: i128) -> i64 {
        let fp = &self.primes[idx];
        if fp.ramified {
            return 1;
        }
        let m = if fp.p == 2 { 4 } else { fp.p as i128 };
        let r = b.rem_euclid(m) as u64;
        if r == fp.key {
            1
        } else {
            debug_assert_eq!((r + fp.key) % (m as u64), 0, "orientation mismatch");
            -1
        }
    }

    /// Factor the leading coefficient of a reduced form over the factor
    /// base.  Returns the ideal factorization as `(index, signed exponent)`
    /// and the cofactor (1 when smooth).  `None` if `a` does not fit `u128`.
    fn factor_form(&self, f: &Bqf) -> Option<(Vec<(u32, i64)>, u128)> {
        let mut x = f.a.to_u128()?;
        let b = f.b.to_i128()?;
        let mut out = Vec::new();
        if let Some(i2) = self.two_index {
            let tz = x.trailing_zeros();
            if tz > 0 {
                x >>= tz;
                out.push((i2 as u32, self.orientation(i2, b) * tz as i64));
            }
        }
        for (i, dv) in self.divisors.iter().enumerate() {
            let Some(dv) = dv else { continue };
            let p = dv.p as u128;
            if p * p > x {
                break;
            }
            let mut e = 0i64;
            loop {
                let q = x.wrapping_mul(dv.inv);
                if q <= dv.lim {
                    x = q;
                    e += 1;
                } else {
                    break;
                }
            }
            if e > 0 {
                out.push((i as u32, self.orientation(i, b) * e));
            }
        }
        if x > 1 {
            if x <= self.bound as u128 {
                if let Some(&i) = self.index.get(&(x as u64)) {
                    out.push((i as u32, self.orientation(i, b)));
                    x = 1;
                }
            }
        }
        out.sort_by_key(|e| e.0);
        Some((out, x))
    }
}

/// Sort, merge duplicate columns and drop zeros.
pub fn normalize_relation(mut rel: Vec<(u32, i64)>) -> Relation {
    rel.sort_by_key(|e| e.0);
    let mut out: Relation = Vec::with_capacity(rel.len());
    for (c, v) in rel {
        if let Some(last) = out.last_mut() {
            if last.0 == c {
                last.1 += v;
                continue;
            }
        }
        out.push((c, v));
    }
    out.retain(|e| e.1 != 0);
    out
}

#[derive(Clone, Debug)]
struct Partial {
    rel: Relation,
    large: u64,
    bmod: u64,
}

/// One deterministic batch of relation trials.
fn trial_batch(
    fb: &FactorBase,
    seed: u64,
    trials: usize,
    targets: &[u32],
    lp_bound: u64,
) -> (Vec<Relation>, Vec<Partial>, u64) {
    let mut rng = SmallRng::seed_from_u64(seed);
    let n = fb.len();
    let disc = &fb.disc;
    let mut rels = Vec::new();
    let mut partials = Vec::new();
    // a varying number of base terms: with a fixed count every relation of a
    // class-number-one discriminant would have an even coefficient sum
    let k_max = 3.min(n);
    let inner = 16;
    let mut done = 0usize;
    let mut count = 0u64;
    while done < trials {
        // base product F = prod g_i^{±1}
        let mut exps: Vec<(u32, i64)> = Vec::new();
        let mut base: Option<Bqf> = None;
        let k_base = rng.gen_range(1..=k_max);
        for _ in 0..k_base {
            let i = rng.gen_range(0..n);
            let s: i64 = if rng.gen_bool(0.5) { 1 } else { -1 };
            let g = if s > 0 {
                &fb.primes[i].form
            } else {
                &fb.primes[i].form_inv
            };
            base = Some(match base {
                None => g.clone(),
                Some(f) => compose_reduce(&f, g, disc),
            });
            exps.push((i as u32, s));
        }
        let base = base.expect("nonempty factor base");
        for _ in 0..inner {
            done += 1;
            count += 1;
            let j = if !targets.is_empty() && rng.gen_bool(0.5) {
                targets[rng.gen_range(0..targets.len())] as usize
            } else {
                rng.gen_range(0..n)
            };
            let s: i64 = if rng.gen_bool(0.5) { 1 } else { -1 };
            let g = if s > 0 {
                &fb.primes[j].form
            } else {
                &fb.primes[j].form_inv
            };
            let f = compose_reduce(&base, g, disc);
            let Some((fac, cof)) = fb.factor_form(&f) else {
                continue;
            };
            if cof != 1 && cof > lp_bound as u128 {
                continue;
            }
            let mut rel = exps.clone();
            rel.push((j as u32, s));
            for (c, v) in fac {
                rel.push((c, -v));
            }
            let rel = normalize_relation(rel);
            if cof == 1 {
                if !rel.is_empty() {
                    rels.push(rel);
                }
            } else {
                let p = cof as u64;
                let bmod = f.b.mod_floor(&BigInt::from(p)).to_u64().unwrap_or(0);
                partials.push(Partial {
                    rel,
                    large: p,
                    bmod,
                });
            }
        }
    }
    (rels, partials, count)
}

// ───────────────────────────── linear algebra ─────────────────────────────

fn row_get(row: &[(u32, i64)], c: u32) -> i64 {
    match row.binary_search_by_key(&c, |e| e.0) {
        Ok(k) => row[k].1,
        Err(_) => 0,
    }
}

/// `s − m·r` for sparse rows; `None` on coefficient overflow past `2^40`.
fn row_axpy(s: &[(u32, i64)], m: i64, r: &[(u32, i64)]) -> Option<Relation> {
    const CAP: i64 = 1 << 40;
    let mut out = Vec::with_capacity(s.len() + r.len());
    let (mut i, mut j) = (0, 0);
    while i < s.len() || j < r.len() {
        let take_s = j >= r.len() || (i < s.len() && s[i].0 < r[j].0);
        let take_r = i >= s.len() || (j < r.len() && r[j].0 < s[i].0);
        if take_s {
            out.push(s[i]);
            i += 1;
        } else if take_r {
            let v = r[j].1.checked_mul(m)?.checked_neg()?;
            if v.abs() > CAP {
                return None;
            }
            out.push((r[j].0, v));
            j += 1;
        } else {
            let v = s[i].1.checked_sub(r[j].1.checked_mul(m)?)?;
            if v.abs() > CAP {
                return None;
            }
            if v != 0 {
                out.push((s[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    Some(out)
}

struct Elimination {
    /// (column, pivot row at elimination time, pivot value ±1), in order.
    log: Vec<(u32, Relation, i64)>,
    rows: Vec<Relation>,
    alive: Vec<u32>,
}

/// Structured Gaussian elimination with ±1 pivots (Markowitz ordering).
fn eliminate(ncols: usize, input: &[Relation], markowitz_limit: usize) -> Elimination {
    let mut rows: Vec<Relation> = input.to_vec();
    let mut active = vec![true; rows.len()];
    let mut col_rows: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    for (ri, r) in rows.iter().enumerate() {
        for &(c, _) in r {
            col_rows[c as usize].push(ri as u32);
        }
    }
    let mut col_alive = vec![true; ncols];
    let mut col_blocked = vec![false; ncols];
    let mut log = Vec::new();
    loop {
        let mut best: Option<(usize, usize, usize)> = None; // (cost, col, row)
        for c in 0..ncols {
            if !col_alive[c] || col_blocked[c] {
                continue;
            }
            let list = &mut col_rows[c];
            list.sort_unstable();
            list.dedup();
            list.retain(|&r| active[r as usize] && row_get(&rows[r as usize], c as u32) != 0);
            let k = list.len();
            if k == 0 {
                continue;
            }
            for &r in list.iter() {
                let row = &rows[r as usize];
                if row_get(row, c as u32).abs() == 1 {
                    let cost = (k - 1) * (row.len() - 1);
                    if best.is_none_or(|b| cost < b.0) {
                        best = Some((cost, c, r as usize));
                    }
                }
            }
            if let Some(b) = best {
                if b.0 == 0 {
                    break;
                }
            }
        }
        let Some((cost, c, pr)) = best else { break };
        if cost > markowitz_limit {
            break;
        }
        let prow = rows[pr].clone();
        let pv = row_get(&prow, c as u32);
        let others: Vec<u32> = col_rows[c]
            .iter()
            .copied()
            .filter(|&r| r as usize != pr)
            .collect();
        let mut updates = Vec::with_capacity(others.len());
        let mut ok = true;
        for &s in &others {
            let v = row_get(&rows[s as usize], c as u32);
            match row_axpy(&rows[s as usize], v * pv, &prow) {
                Some(nr) => updates.push((s, nr)),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            col_blocked[c] = true;
            continue;
        }
        for (s, nr) in updates {
            for &(cc, _) in &nr {
                col_rows[cc as usize].push(s);
            }
            if nr.is_empty() {
                active[s as usize] = false;
            }
            rows[s as usize] = nr;
        }
        active[pr] = false;
        col_alive[c] = false;
        log.push((c as u32, prow, pv));
    }
    let alive: Vec<u32> = (0..ncols as u32).filter(|&c| col_alive[c as usize]).collect();
    let rows_left: Vec<Relation> = rows
        .into_iter()
        .zip(active)
        .filter(|(r, a)| *a && !r.is_empty())
        .map(|(r, _)| r)
        .collect();
    Elimination {
        log,
        rows: rows_left,
        alive,
    }
}

/// Incremental Hermite normal form (row style, upper triangular, positive
/// pivots, entries right of a pivot reduced into `[0, pivot)`).
pub(crate) struct Hnf {
    n: usize,
    pub(crate) rows: Vec<Option<Vec<BigInt>>>,
    inserted: usize,
}

impl Hnf {
    pub(crate) fn new(n: usize) -> Self {
        Hnf {
            n,
            rows: vec![None; n],
            inserted: 0,
        }
    }

    pub(crate) fn full_rank(&self) -> bool {
        self.rows.iter().all(|r| r.is_some())
    }

    pub(crate) fn det(&self) -> Option<BigInt> {
        let mut d = BigInt::one();
        for (i, r) in self.rows.iter().enumerate() {
            d *= &r.as_ref()?[i];
        }
        Some(d)
    }

    fn size_reduce_row(&mut self, i: usize) {
        let n = self.n;
        for j in i + 1..n {
            let Some(rj) = self.rows[j].clone() else {
                continue;
            };
            let ri = self.rows[i].as_mut().unwrap();
            let q = ri[j].div_floor(&rj[j]);
            if !q.is_zero() {
                for k in j..n {
                    ri[k] -= &q * &rj[k];
                }
            }
        }
    }

    pub(crate) fn insert(&mut self, mut v: Vec<BigInt>) {
        self.inserted += 1;
        let n = self.n;
        if let Some(d) = self.det() {
            for x in v.iter_mut() {
                *x = x.mod_floor(&d);
            }
        }
        for i in 0..n {
            if v[i].is_zero() {
                continue;
            }
            match self.rows[i].take() {
                None => {
                    if v[i].is_negative() {
                        for x in v.iter_mut() {
                            *x = -&*x;
                        }
                    }
                    self.rows[i] = Some(v);
                    self.size_reduce_row(i);
                    // rows above may need reduction against the new pivot
                    for k in (0..i).rev() {
                        if self.rows[k].is_some() {
                            self.size_reduce_row(k);
                        }
                    }
                    return;
                }
                Some(h) => {
                    if v[i].mod_floor(&h[i]).is_zero() {
                        let q = &v[i] / &h[i];
                        for k in i..n {
                            v[k] -= &q * &h[k];
                        }
                        self.rows[i] = Some(h);
                    } else {
                        let (g, x, y) = ext_gcd(&h[i], &v[i]);
                        let hi = &h[i] / &g;
                        let vi = &v[i] / &g;
                        let mut nh = vec![BigInt::zero(); n];
                        let mut nv = vec![BigInt::zero(); n];
                        for k in i..n {
                            nh[k] = &x * &h[k] + &y * &v[k];
                            nv[k] = &vi * &h[k] - &hi * &v[k];
                        }
                        self.rows[i] = Some(nh);
                        self.size_reduce_row(i);
                        v = nv;
                    }
                }
            }
        }
    }

    pub(crate) fn finish(&mut self) {
        for i in (0..self.n).rev() {
            if self.rows[i].is_some() {
                self.size_reduce_row(i);
            }
        }
    }
}

// ── modular determinant and modular HNF ──

fn is_prime_u32(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // deterministic for n < 4_759_123_141
    'outer: for a in [2u64, 7, 61] {
        if a % n == 0 {
            continue;
        }
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// Primes just below `2^31`, descending.
fn crt_primes(count: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(count);
    let mut c = (1u64 << 31) - 1;
    while out.len() < count {
        if is_prime_u32(c) {
            out.push(c);
        }
        c -= 2;
    }
    out
}

fn residue(x: i64, p: u64) -> u64 {
    x.rem_euclid(p as i64) as u64
}

/// Indices of `n` rows that are linearly independent modulo `p` (greedy, in
/// order), or `None` when the rows have rank `< n` modulo `p`.
fn select_independent_rows(rows: &[Vec<(usize, i64)>], n: usize, p: u64) -> Option<Vec<usize>> {
    if n == 0 {
        return Some(Vec::new());
    }
    let mut piv: Vec<Option<Vec<u64>>> = vec![None; n]; // pivot row normalised to 1 at its column
    let mut chosen = Vec::new();
    for (ri, r) in rows.iter().enumerate() {
        let mut v = vec![0u64; n];
        for &(c, x) in r {
            v[c] = residue(x, p);
        }
        for c in 0..n {
            if v[c] == 0 {
                continue;
            }
            match &piv[c] {
                Some(pr) => {
                    let f = v[c];
                    for k in c..n {
                        if pr[k] != 0 {
                            v[k] = (v[k] + p - mulmod(f, pr[k], p)) % p;
                        }
                    }
                }
                None => {
                    let inv = powmod(v[c], p - 2, p);
                    for x in v.iter_mut().skip(c) {
                        *x = mulmod(*x, inv, p);
                    }
                    piv[c] = Some(v);
                    chosen.push(ri);
                    break;
                }
            }
        }
        if chosen.len() == n {
            return Some(chosen);
        }
    }
    None
}

fn det_mod_p(mat: &[Vec<u64>], p: u64) -> u64 {
    let n = mat.len();
    let mut a: Vec<Vec<u64>> = mat.iter().map(|r| r.iter().map(|x| x % p).collect()).collect();
    let mut det = 1u64;
    for c in 0..n {
        let Some(pr) = (c..n).find(|&r| a[r][c] != 0) else {
            return 0;
        };
        if pr != c {
            a.swap(pr, c);
            det = (p - det) % p;
        }
        det = mulmod(det, a[c][c], p);
        let inv = powmod(a[c][c], p - 2, p);
        let (top, bottom) = a.split_at_mut(c + 1);
        let prow = &top[c];
        for row in bottom.iter_mut() {
            if row[c] == 0 {
                continue;
            }
            let f = mulmod(row[c], inv, p);
            for k in c..n {
                if prow[k] != 0 {
                    row[k] = (row[k] + p - mulmod(f, prow[k], p)) % p;
                }
            }
        }
    }
    det
}

/// `|det|` of the square matrix formed by `rows[idx]`, by CRT over 31-bit
/// primes up to twice the Hadamard bound.
fn exact_abs_det(rows: &[Vec<(usize, i64)>], idx: &[usize], n: usize) -> BigInt {
    let mut log2_bound = 1.0f64;
    for &i in idx {
        let s: f64 = rows[i].iter().map(|&(_, x)| (x as f64) * (x as f64)).sum();
        log2_bound += 0.5 * s.max(1.0).log2();
    }
    let count = (log2_bound / 30.0).ceil() as usize + 2;
    let primes = crt_primes(count);
    let residues: Vec<u64> = primes
        .par_iter()
        .map(|&p| {
            let mat: Vec<Vec<u64>> = idx
                .iter()
                .map(|&i| {
                    let mut v = vec![0u64; n];
                    for &(c, x) in &rows[i] {
                        v[c] = residue(x, p);
                    }
                    v
                })
                .collect();
            det_mod_p(&mat, p)
        })
        .collect();
    // CRT
    let mut acc = BigInt::zero();
    let mut modulus = BigInt::one();
    for (&p, &r) in primes.iter().zip(&residues) {
        let pb = BigInt::from(p);
        let m_mod_p = mod_u64(&modulus, p);
        let a_mod_p = mod_u64(&acc, p);
        let diff = (r + p - a_mod_p) % p;
        let t = mulmod(diff, powmod(m_mod_p, p - 2, p), p);
        acc += &modulus * t;
        modulus *= pb;
    }
    let half = &modulus / 2;
    if acc > half {
        acc -= &modulus;
    }
    acc.abs()
}

/// Hermite basis (row `i` has its pivot at column `i`, zeros before it,
/// entries after it reduced modulo the later pivots) of the lattice spanned
/// by `gens` **plus** `D·Z^n`, where `D` must be a multiple of the
/// determinant of the lattice spanned by `gens` (which has full rank).
/// Arithmetic is modulo a shrinking `R` (Domich–Kannan–Trotter; Cohen,
/// GTM 138, Algorithm 2.4.8): if `L ⊇ R·Z^n` and `det L | R` and the
/// projection of `L` to the current column is `dZ`, then
/// `L = Z w ⊕ L'` with `det L' | R/d`, so `L' ⊇ (R/d)·Z^{n−i−1}`.
fn hnf_modular(mut gens: Vec<Vec<BigInt>>, n: usize, d_mod: &BigInt) -> Vec<Vec<BigInt>> {
    let mut r = d_mod.clone();
    let mut w_rows: Vec<Vec<BigInt>> = Vec::with_capacity(n);
    for g in gens.iter_mut() {
        for x in g.iter_mut() {
            *x = x.mod_floor(&r);
        }
    }
    for i in 0..n {
        let mut acc: Option<Vec<BigInt>> = None;
        for g in gens.iter_mut() {
            if g[i].is_zero() {
                continue;
            }
            match acc.as_mut() {
                None => {
                    acc = Some(std::mem::replace(g, vec![BigInt::zero(); n]));
                }
                Some(a) => {
                    let (gg, x, y) = ext_gcd(&a[i], &g[i]);
                    let ai = &a[i] / &gg;
                    let gi = &g[i] / &gg;
                    for k in i..n {
                        let na = (&x * &a[k] + &y * &g[k]).mod_floor(&r);
                        let ng = (&ai * &g[k] - &gi * &a[k]).mod_floor(&r);
                        a[k] = na;
                        g[k] = ng;
                    }
                }
            }
        }
        gens.retain(|g| g.iter().any(|x| !x.is_zero()));
        let (d, w) = match acc {
            None => {
                let mut w = vec![BigInt::zero(); n];
                w[i] = r.clone();
                (r.clone(), w)
            }
            Some(a) => {
                let (d, u, _) = ext_gcd(&a[i], &r);
                let mut w: Vec<BigInt> = a.iter().map(|x| (&u * x).mod_floor(&r)).collect();
                w[i] = d.clone();
                // a - (a_i/d) w has zero in column i and lies in the lattice
                let q = &a[i] / &d;
                let extra: Vec<BigInt> = a
                    .iter()
                    .zip(&w)
                    .map(|(x, y)| (x - &q * y).mod_floor(&r))
                    .collect();
                if extra.iter().any(|x| !x.is_zero()) {
                    gens.push(extra);
                }
                (d, w)
            }
        };
        w_rows.push(w);
        r = &r / &d;
        if r.is_one() {
            // the rest of the lattice is Z^{n-i-1}
            for j in i + 1..n {
                let mut e = vec![BigInt::zero(); n];
                e[j] = BigInt::one();
                w_rows.push(e);
            }
            break;
        }
        for g in gens.iter_mut() {
            for x in g.iter_mut().skip(i + 1) {
                *x = x.mod_floor(&r);
            }
        }
    }
    // exact size reduction (unimodular, lattice unchanged)
    for i in (0..n).rev() {
        for j in i + 1..n {
            let q = w_rows[i][j].div_floor(&w_rows[j][j]);
            if !q.is_zero() {
                let wj = w_rows[j].clone();
                for k in j..n {
                    w_rows[i][k] -= &q * &wj[k];
                }
            }
        }
    }
    w_rows
}

/// Smith normal form of a square nonsingular integer matrix `a` (rows are
/// lattice generators).  Returns the diagonal `s_1 | s_2 | …` and the column
/// transform `V` (reduced modulo `modulus`) with `U a V = diag(s)`.
fn smith_with_v(mut a: Vec<Vec<BigInt>>, modulus: &BigInt) -> (Vec<BigInt>, Vec<Vec<BigInt>>) {
    let n = a.len();
    let mut v: Vec<Vec<BigInt>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| if i == j { BigInt::one() } else { BigInt::zero() })
                .collect()
        })
        .collect();
    for k in 0..n {
        loop {
            // pivot: prefer (k,k) when it is ±1, else the smallest |entry|
            let mut piv: Option<(usize, usize)> = None;
            if a[k][k].abs().is_one() {
                piv = Some((k, k));
            } else {
                let mut best: Option<BigInt> = None;
                for (i, row) in a.iter().enumerate().skip(k) {
                    for (j, x) in row.iter().enumerate().skip(k) {
                        if !x.is_zero() {
                            let ax = x.abs();
                            if best.as_ref().is_none_or(|b| &ax < b) {
                                best = Some(ax);
                                piv = Some((i, j));
                            }
                        }
                    }
                }
            }
            let Some((pi, pj)) = piv else {
                panic!("singular matrix in smith_with_v");
            };
            if pi != k {
                a.swap(pi, k);
            }
            if pj != k {
                for row in a.iter_mut() {
                    row.swap(pj, k);
                }
                for row in v.iter_mut() {
                    row.swap(pj, k);
                }
            }
            let p = a[k][k].clone();
            let mut clean = true;
            // clear column k below
            for i in k + 1..n {
                if a[i][k].is_zero() {
                    continue;
                }
                let q = &a[i][k] / &p;
                if !q.is_zero() {
                    let rk = a[k].clone();
                    for j in k..n {
                        let t = &q * &rk[j];
                        a[i][j] -= t;
                    }
                }
                if !a[i][k].is_zero() {
                    clean = false;
                }
            }
            // clear row k to the right
            let nz_rows: Vec<usize> = (0..n).filter(|&i| !a[i][k].is_zero()).collect();
            for j in k + 1..n {
                if a[k][j].is_zero() {
                    continue;
                }
                let q = &a[k][j] / &p;
                if !q.is_zero() {
                    for &i in &nz_rows {
                        let t = &q * &a[i][k];
                        a[i][j] -= t;
                    }
                    for row in v.iter_mut() {
                        let t = &q * &row[k];
                        row[j] = (&row[j] - t).mod_floor(modulus);
                    }
                }
                if !a[k][j].is_zero() {
                    clean = false;
                }
            }
            if !clean {
                continue;
            }
            if !p.abs().is_one() {
                let mut bad = None;
                'outer: for (i, row) in a.iter().enumerate().skip(k + 1) {
                    for x in row.iter().skip(k + 1) {
                        if !x.mod_floor(&p).is_zero() {
                            bad = Some(i);
                            break 'outer;
                        }
                    }
                }
                if let Some(i) = bad {
                    let ri = a[i].clone();
                    for j in k..n {
                        a[k][j] += &ri[j];
                    }
                    continue;
                }
            }
            break;
        }
    }
    let s = (0..n).map(|k| a[k][k].abs()).collect();
    (s, v)
}

/// Group structure of `Z^ncols / ⟨rels⟩`, if the relations have full rank.
pub struct Structure {
    pub h: BigInt,
    /// Invariant factors `> 1`, descending (PARI `cyc` order).
    pub invariants: Vec<BigInt>,
    /// Discrete log of each column, in the coordinates of `invariants`.
    pub dlogs: Vec<Vec<BigInt>>,
    pub dense_dim: usize,
    pub eliminated: usize,
}

/// Does the relation map to zero under the discrete logs of `s`?
pub fn relation_in_lattice(s: &Structure, rel: &[(u32, i64)]) -> bool {
    (0..s.invariants.len()).all(|t| {
        let mut acc = BigInt::zero();
        for &(c, v) in rel {
            acc += &s.dlogs[c as usize][t] * v;
        }
        acc.mod_floor(&s.invariants[t]).is_zero()
    })
}

/// Compute the structure of `Z^ncols / L` where `L` is spanned by `rels`.
///
/// Linear algebra runs on a primary subset of about `1.1·ncols` relations;
/// every other relation is then checked against the resulting discrete logs
/// and any relation outside the lattice is moved into the primary set and
/// the computation repeated, so the returned structure is that of the
/// lattice spanned by **all** relations.
pub fn structure_from_relations(
    ncols: usize,
    rels: &[Relation],
    markowitz_limit: usize,
) -> Option<Structure> {
    let cap = ncols + ncols / 10 + 64;
    let mut primary: Vec<Relation> = Vec::new();
    let mut rest: Vec<Relation> = Vec::new();
    for (i, r) in rels.iter().enumerate() {
        if i < cap {
            primary.push(r.clone());
        } else {
            rest.push(r.clone());
        }
    }
    loop {
        let s = match structure_core(ncols, &primary, markowitz_limit) {
            Some(s) => s,
            None => {
                if rest.is_empty() {
                    return None;
                }
                // rank deficient: move a block of further relations in
                let k = rest.len().min(ncols / 4 + 16);
                primary.extend(rest.drain(..k));
                continue;
            }
        };
        let (bad, good): (Vec<Relation>, Vec<Relation>) =
            rest.into_iter().partition(|r| !relation_in_lattice(&s, r));
        if bad.is_empty() {
            return Some(s);
        }
        primary.extend(bad);
        rest = good;
    }
}

fn structure_core(
    ncols: usize,
    rels: &[Relation],
    markowitz_limit: usize,
) -> Option<Structure> {
    let t_e = Instant::now();
    let el = eliminate(ncols, rels, markowitz_limit);
    let t_el = t_e.elapsed().as_secs_f64();
    let t_h = Instant::now();
    let nd = el.alive.len();
    let pos: HashMap<u32, usize> = el.alive.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let dense: Vec<Vec<(usize, i64)>> = el
        .rows
        .iter()
        .map(|r| r.iter().map(|&(c, x)| (pos[&c], x)).collect())
        .collect();
    let to_vec = |r: &[(usize, i64)]| -> Vec<BigInt> {
        let mut v = vec![BigInt::zero(); nd];
        for &(c, x) in r {
            v[c] = BigInt::from(x);
        }
        v
    };
    // Choose `nd` rows independent modulo a 31-bit prime (two primes, so a
    // prime dividing the determinant cannot fake rank deficiency), take the
    // exact |det| D of that square block by CRT, and compute the HNF of the
    // generators modulo D (valid: D·Z^nd lies in the lattice of the block).
    // The remaining rows are then *checked* against the discrete logs and
    // only rows outside the lattice are added (with the previous h* as the
    // modulus, which the enlarged lattice's determinant divides).
    let primes = crt_primes(2);
    let sel = select_independent_rows(&dense, nd, primes[0])
        .or_else(|| select_independent_rows(&dense, nd, primes[1]))?;
    let mut d_mod = exact_abs_det(&dense, &sel, nd);
    if d_mod.is_zero() {
        return None;
    }
    let mut inserted = vec![false; dense.len()];
    let mut gens: Vec<Vec<BigInt>> = Vec::new();
    for &i in &sel {
        inserted[i] = true;
        gens.push(to_vec(&dense[i]));
    }
    // Further independent blocks from shuffled row orders: the lattice of all
    // their rows has a determinant dividing every block determinant, so the
    // gcd is a valid (and usually much smaller) modulus.
    let mut rng = SmallRng::seed_from_u64(0xb10c ^ dense.len() as u64);
    for _ in 0..3 {
        if d_mod.bits() <= 2 * (nd as u64 + 64) / 4 {
            break;
        }
        let mut order: Vec<usize> = (0..dense.len()).collect();
        for k in (1..order.len()).rev() {
            order.swap(k, rng.gen_range(0..=k));
        }
        let shuffled: Vec<Vec<(usize, i64)>> = order.iter().map(|&i| dense[i].clone()).collect();
        let Some(sel2) = select_independent_rows(&shuffled, nd, primes[0]) else {
            break;
        };
        let d2 = exact_abs_det(&shuffled, &sel2, nd);
        let g = d_mod.gcd(&d2);
        if g < d_mod {
            d_mod = g;
            for &j in &sel2 {
                let i = order[j];
                if !inserted[i] {
                    inserted[i] = true;
                    gens.push(to_vec(&dense[i]));
                }
            }
        }
    }
    let mut extra_added = 0usize;
    for i in 0..dense.len() {
        if extra_added >= 24 {
            break;
        }
        if !inserted[i] {
            inserted[i] = true;
            gens.push(to_vec(&dense[i]));
            extra_added += 1;
        }
    }
    let mut modulus = d_mod.clone();
    let mut basis = hnf_modular(gens, nd, &modulus);
    let t_hnf = t_h.elapsed().as_secs_f64();
    let (h, s, v) = loop {
        let mut h = BigInt::one();
        for (i, row) in basis.iter().enumerate() {
            h *= &row[i];
        }
        let t_s = Instant::now();
        let (s, v) = if nd > 0 {
            smith_with_v(basis.clone(), &h)
        } else {
            (Vec::new(), Vec::new())
        };
        // check the rows not yet used
        let mut failing = Vec::new();
        for (i, r) in dense.iter().enumerate() {
            if inserted[i] {
                continue;
            }
            let bad = (0..nd).any(|k| {
                if s[k].is_one() {
                    return false;
                }
                let mut acc = BigInt::zero();
                for &(c, x) in r {
                    acc += &v[c][k] * x;
                }
                !acc.mod_floor(&s[k]).is_zero()
            });
            if bad {
                failing.push(i);
            }
        }
        if std::env::var_os("CG_TIMING").is_some() {
            eprintln!(
                "    [linalg] elim {:.2}s hnf {:.2}s snf+check {:.2}s nd {} rows {} |det block| {} bits, h* {} bits, failing {}",
                t_el,
                t_hnf,
                t_s.elapsed().as_secs_f64(),
                nd,
                dense.len(),
                d_mod.bits(),
                h.bits(),
                failing.len()
            );
        }
        if failing.is_empty() {
            break (h, s, v);
        }
        modulus = h.clone();
        let mut gens = basis.clone();
        for i in failing {
            inserted[i] = true;
            gens.push(to_vec(&dense[i]));
        }
        basis = hnf_modular(gens, nd, &modulus);
    };
    // invariant positions (s > 1), ascending in s; output descending
    let inv_pos: Vec<usize> = (0..nd).filter(|&k| s[k] > BigInt::one()).rev().collect();
    let invariants: Vec<BigInt> = inv_pos.iter().map(|&k| s[k].clone()).collect();
    let m = invariants.len();
    let mut dlogs: Vec<Option<Vec<BigInt>>> = vec![None; ncols];
    for (i, &c) in el.alive.iter().enumerate() {
        let d: Vec<BigInt> = inv_pos
            .iter()
            .map(|&k| v[i][k].mod_floor(&s[k]))
            .collect();
        dlogs[c as usize] = Some(d);
    }
    for (c, row, pv) in el.log.iter().rev() {
        // pv x_c + sum_{j != c} r_j x_j = 0  =>  x_c = -pv * sum r_j x_j
        let mut acc = vec![BigInt::zero(); m];
        for &(j, rj) in row {
            if j == *c {
                continue;
            }
            let dj = dlogs[j as usize]
                .as_ref()
                .expect("back-substitution order");
            for t in 0..m {
                acc[t] += &dj[t] * rj;
            }
        }
        let d: Vec<BigInt> = (0..m)
            .map(|t| (-&acc[t] * pv).mod_floor(&invariants[t]))
            .collect();
        dlogs[*c as usize] = Some(d);
    }
    Some(Structure {
        h,
        invariants,
        dlogs: dlogs.into_iter().map(|d| d.unwrap()).collect(),
        dense_dim: nd,
        eliminated: el.log.len(),
    })
}

// ───────────────────────────── analytic estimate ─────────────────────────────

/// Number of units `w` of the maximal order of discriminant `D`.
pub fn units(disc: &BigInt) -> u32 {
    if *disc == BigInt::from(-3) {
        6
    } else if *disc == BigInt::from(-4) {
        4
    } else {
        2
    }
}

/// Natural logarithm of a positive BigInt.
pub fn ln_bigint(x: &BigInt) -> f64 {
    let bits = x.bits();
    let shift = bits.saturating_sub(64);
    let top = (x >> shift).to_f64().unwrap_or(f64::NAN);
    top.ln() + shift as f64 * std::f64::consts::LN_2
}

/// `log h_approx` from the truncated Euler product of `L(1, χ_D)` over the
/// given primes: `h ≈ w √|D| L(1,χ) / (2π)`.
pub fn analytic_log_h(disc: &BigInt, primes: &[u64]) -> f64 {
    let mut log_l = 0.0f64;
    for &p in primes {
        let chi = kronecker_prime(disc, p) as f64;
        log_l -= (1.0 - chi / p as f64).ln();
    }
    (units(disc) as f64).ln() + 0.5 * ln_bigint(&disc.abs()) + log_l
        - (2.0 * std::f64::consts::PI).ln()
}

// ───────────────────────────── driver ─────────────────────────────

/// Parameters of [`compute_class_group`].
#[derive(Clone, Debug)]
pub struct SubexpParams {
    /// Factor-base bound as a multiple of Bach's `6 log²|D|`.
    pub fb_factor: f64,
    /// The factor base contains every prime up to at least this bound.
    pub min_fb_bound: u64,
    pub seed: u64,
    /// Relation trials per parallel task.
    pub trials_per_task: usize,
    /// Extra relations per verification batch (0 = automatic).
    pub batch_extra: usize,
    /// Consecutive batches without change of `h*` required to stop.
    pub stable_rounds: usize,
    /// Euler product over primes up to this bound.
    pub euler_bound: u64,
    /// Held-out relations checked against the discrete logs.
    pub heldout: usize,
    /// Large-prime bound as a multiple of the factor-base bound (0 = off).
    pub lp_factor: u64,
    pub markowitz_limit: usize,
    pub max_rounds: usize,
    /// Run trial batches in parallel with rayon.
    pub parallel: bool,
    pub verbose: bool,
}

impl Default for SubexpParams {
    fn default() -> Self {
        SubexpParams {
            fb_factor: 1.0,
            min_fb_bound: 2,
            seed: 0x5eed_c1a5_5600_0001,
            trials_per_task: 4096,
            batch_extra: 0,
            stable_rounds: 2,
            euler_bound: 1 << 22,
            heldout: 32,
            lp_factor: 64,
            markowitz_limit: 20_000,
            max_rounds: 400,
            parallel: true,
            verbose: false,
        }
    }
}

/// Result of [`compute_class_group`].
#[derive(Clone, Debug)]
pub struct SubexpClassGroup {
    pub disc: BigInt,
    pub h: BigInt,
    /// Invariant factors `> 1`, descending (PARI `cyc` order).
    pub invariants: Vec<BigInt>,
    pub fb: Vec<FbPrime>,
    /// Discrete log of each factor-base prime ideal `𝔭_i = (p, (−b+√D)/2)`.
    pub dlogs: Vec<Vec<BigInt>>,
    pub fb_bound: u64,
    pub bach_bound: u64,
    pub relations_used: usize,
    pub full_relations_found: usize,
    pub combined_large_prime_relations: usize,
    pub trials: u64,
    pub dense_dim: usize,
    pub eliminated: usize,
    /// `h*` after each verification batch.
    pub h_history: Vec<BigInt>,
    pub euler_bound: u64,
    pub log_h_approx: f64,
    /// `log(h / h_approx)`.
    pub log_ratio: f64,
    pub analytic_ok: bool,
    pub heldout_checked: usize,
    pub heldout_ok: bool,
    pub seconds: f64,
    pub seconds_relations: f64,
    pub seconds_linalg: f64,
}

impl SubexpClassGroup {
    pub fn index_of(&self, p: u64) -> Option<usize> {
        self.fb.iter().position(|f| f.p == p)
    }

    /// `Σ e_i dlog_i` reduced modulo the invariants.
    pub fn combine(&self, rel: &[(u32, i64)]) -> Vec<BigInt> {
        let m = self.invariants.len();
        let mut acc = vec![BigInt::zero(); m];
        for &(c, v) in rel {
            for t in 0..m {
                acc[t] += &self.dlogs[c as usize][t] * v;
            }
        }
        acc.iter()
            .zip(&self.invariants)
            .map(|(x, d)| x.mod_floor(d))
            .collect()
    }

    /// Order of the class of factor-base prime `i`.
    pub fn order_of(&self, i: usize) -> BigInt {
        let mut o = BigInt::one();
        for (x, d) in self.dlogs[i].iter().zip(&self.invariants) {
            let g = x.gcd(d);
            o = o.lcm(&(d / g));
        }
        o
    }
}

/// Bach's GRH bound `6 log²|D|` (rounded up).
pub fn bach_bound(disc: &BigInt) -> u64 {
    let ln = ln_bigint(&disc.abs());
    (6.0 * ln * ln).ceil() as u64
}

/// Compute `Cl(D)` (GRH) with the subexponential method described in the
/// module documentation.
pub fn compute_class_group(disc: &BigInt, params: &SubexpParams) -> Result<SubexpClassGroup, String> {
    let t0 = Instant::now();
    if !disc.is_negative() {
        return Err("discriminant must be negative".into());
    }
    let d4 = mod_u64(disc, 4);
    if d4 != 0 && d4 != 1 {
        return Err("discriminant must be 0 or 1 mod 4".into());
    }
    let bach = bach_bound(disc);
    let fb_bound = ((bach as f64 * params.fb_factor).ceil() as u64)
        .max(params.min_fb_bound)
        .max(bach)
        .max(2);
    let fb = FactorBase::new(disc, fb_bound);
    let n = fb.len();
    if n == 0 {
        return Err("empty factor base".into());
    }
    // a cofactor below B² with no prime factor ≤ B is prime
    let lp_bound = if params.lp_factor == 0 {
        0
    } else {
        fb_bound
            .saturating_mul(params.lp_factor)
            .min(fb_bound.saturating_mul(fb_bound) - 1)
    };
    let mut rels: Vec<Relation> = Vec::new();
    // ramified primes: 2[p] = 0
    for (i, f) in fb.primes.iter().enumerate() {
        if f.ramified {
            rels.push(vec![(i as u32, 2)]);
        }
    }
    let mut partial_map: HashMap<u64, Partial> = HashMap::new();
    let mut full_found = 0usize;
    let mut combined = 0usize;
    let mut trials = 0u64;
    let tasks = if params.parallel {
        rayon::current_num_threads().max(1) * 2
    } else {
        1
    };
    let mut round = 0u64;
    let mut t_rel = 0.0f64;
    let mut t_lin = 0.0f64;
    let batch_extra = if params.batch_extra > 0 {
        params.batch_extra
    } else {
        (n / 20).max(8)
    };
    let mut target_count = n + n / 8 + 16;
    let mut history: Vec<BigInt> = Vec::new();
    let mut last: Option<Structure>;
    let euler_primes = primes_up_to(params.euler_bound);
    let log_h_approx = analytic_log_h(disc, &euler_primes);
    let tol = std::f64::consts::SQRT_2.ln();
    let collect = |rels: &mut Vec<Relation>,
                       want: usize,
                       partial_map: &mut HashMap<u64, Partial>,
                       full_found: &mut usize,
                       combined: &mut usize,
                       trials: &mut u64,
                       round: &mut u64|
     -> Result<(), String> {
        let tr = Instant::now();
        let mut guard = 0;
        while rels.len() < want {
            guard += 1;
            if guard > 100_000 {
                return Err("relation collection made no progress".into());
            }
            // targets: columns that occur in fewer than 2 relations
            let mut occ = vec![0u32; n];
            for r in rels.iter() {
                for &(c, _) in r {
                    occ[c as usize] += 1;
                }
            }
            let targets: Vec<u32> = (0..n as u32).filter(|&c| occ[c as usize] < 2).collect();
            let seeds: Vec<u64> = (0..tasks as u64)
                .map(|t| {
                    params
                        .seed
                        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                        .wrapping_add((*round << 20) ^ t)
                })
                .collect();
            *round += 1;
            let per = params.trials_per_task;
            let results: Vec<(Vec<Relation>, Vec<Partial>, u64)> = if params.parallel {
                seeds
                    .par_iter()
                    .map(|&s| trial_batch(&fb, s, per, &targets, lp_bound))
                    .collect()
            } else {
                seeds
                    .iter()
                    .map(|&s| trial_batch(&fb, s, per, &targets, lp_bound))
                    .collect()
            };
            for (rs, ps, c) in results {
                *trials += c;
                *full_found += rs.len();
                rels.extend(rs);
                for p in ps {
                    if let Some(first) = partial_map.get(&p.large) {
                        // same ideal if b ≡ b' (mod P), conjugate if b ≡ −b'
                        let same = p.bmod == first.bmod;
                        let m: i64 = if same { 1 } else { -1 };
                        let mut r = first.rel.clone();
                        for &(c, v) in &p.rel {
                            r.push((c, -m * v));
                        }
                        let r = normalize_relation(r);
                        if !r.is_empty() {
                            rels.push(r);
                            *combined += 1;
                        }
                    } else {
                        partial_map.insert(p.large, p);
                    }
                }
            }
            if params.verbose {
                eprintln!(
                    "  [classgroup] round {} relations {}/{} trials {} partials {}",
                    round,
                    rels.len(),
                    want,
                    trials,
                    partial_map.len()
                );
            }
        }
        let _ = tr.elapsed();
        Ok(())
    };
    let mut rounds_done = 0usize;
    loop {
        rounds_done += 1;
        if rounds_done > params.max_rounds {
            return Err(format!(
                "no stable class group after {} verification rounds (h* history {:?})",
                params.max_rounds,
                history.iter().map(|h| h.to_string()).collect::<Vec<_>>()
            ));
        }
        let tr = Instant::now();
        collect(
            &mut rels,
            target_count,
            &mut partial_map,
            &mut full_found,
            &mut combined,
            &mut trials,
            &mut round,
        )?;
        t_rel += tr.elapsed().as_secs_f64();
        let tl = Instant::now();
        let st = structure_from_relations(n, &rels, params.markowitz_limit);
        t_lin += tl.elapsed().as_secs_f64();
        match st {
            None => {
                target_count = rels.len() + batch_extra.max(rels.len() / 4);
                if params.verbose {
                    eprintln!("  [classgroup] rank deficient with {} relations", rels.len());
                }
            }
            Some(s) => {
                history.push(s.h.clone());
                let log_ratio = ln_bigint(&s.h) - log_h_approx;
                if params.verbose {
                    eprintln!(
                        "  [classgroup] h* = {} cyc {:?} dense {} log(h*/h_approx) = {:.4} ({} relations, linalg so far {:.2} s)",
                        s.h,
                        s.invariants.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                        s.dense_dim,
                        log_ratio,
                        rels.len(),
                        t_lin
                    );
                }
                let analytic_ok = log_ratio.abs() < tol;
                last = Some(s);
                // Verification without recomputation: fresh relations must all
                // lie in the current lattice (zero discrete-log sum).  A
                // relation outside it would map to a nonzero element.
                let mut stable = 0usize;
                let mut failed = false;
                while stable < params.stable_rounds {
                    rounds_done += 1;
                    if rounds_done > params.max_rounds {
                        return Err(format!(
                            "no stable class group after {} rounds (h* history {:?})",
                            params.max_rounds,
                            history.iter().map(|h| h.to_string()).collect::<Vec<_>>()
                        ));
                    }
                    let before = rels.len();
                    let tr = Instant::now();
                    collect(
                        &mut rels,
                        before + batch_extra,
                        &mut partial_map,
                        &mut full_found,
                        &mut combined,
                        &mut trials,
                        &mut round,
                    )?;
                    t_rel += tr.elapsed().as_secs_f64();
                    let cur = last.as_ref().unwrap();
                    if rels[before..].iter().any(|r| !relation_in_lattice(cur, r)) {
                        failed = true;
                        break;
                    }
                    stable += 1;
                }
                if !failed && analytic_ok {
                    break;
                }
                target_count = if failed {
                    rels.len()
                } else {
                    rels.len() + 4 * batch_extra
                };
                if params.verbose {
                    eprintln!(
                        "  [classgroup] verification {}; recomputing",
                        if failed {
                            "found a relation outside the lattice"
                        } else {
                            "passed but the analytic check did not"
                        }
                    );
                }
            }
        }
    }
    let s = last.unwrap();
    let log_ratio = s
        .h
        .to_f64()
        .map(|x| x.ln())
        .unwrap_or(f64::NAN)
        - log_h_approx;
    let mut cg = SubexpClassGroup {
        disc: disc.clone(),
        h: s.h.clone(),
        invariants: s.invariants.clone(),
        fb: fb.primes.clone(),
        dlogs: s.dlogs.clone(),
        fb_bound,
        bach_bound: bach,
        relations_used: rels.len(),
        full_relations_found: full_found,
        combined_large_prime_relations: combined,
        trials,
        dense_dim: s.dense_dim,
        eliminated: s.eliminated,
        h_history: history,
        euler_bound: params.euler_bound,
        log_h_approx,
        log_ratio,
        analytic_ok: log_ratio.abs() < tol,
        heldout_checked: 0,
        heldout_ok: true,
        seconds: 0.0,
        seconds_relations: t_rel,
        seconds_linalg: t_lin,
    };
    // held-out relations: fresh seeds, never used above
    let tr = Instant::now();
    let mut held = Vec::new();
    let mut hround = 0u64;
    while held.len() < params.heldout && hround < 1000 {
        let seed = params.seed ^ 0xA5A5_5A5A_0000_0000 ^ (hround.wrapping_mul(0x1000_0000_01B3));
        let (rs, _, c) = trial_batch(&fb, seed, params.trials_per_task.min(4096), &[], 0);
        cg.trials += c;
        held.extend(rs);
        hround += 1;
    }
    held.truncate(params.heldout);
    for r in &held {
        if cg.combine(r).iter().any(|x| !x.is_zero()) {
            cg.heldout_ok = false;
        }
    }
    cg.heldout_checked = held.len();
    cg.seconds_relations += tr.elapsed().as_secs_f64();
    cg.seconds = t0.elapsed().as_secs_f64();
    Ok(cg)
}

// ───────────────────────────── brute force (tests, cross-checks) ─────────────────────────────

/// Class number and invariant factors (descending, `> 1`) of `Cl(D)` by
/// enumerating reduced forms and counting `p^k`-torsion.  `O(h · |D|^{1/2})`.
pub fn bruteforce_structure(disc: &BigInt) -> (u64, Vec<u64>) {
    let forms = super::class_group::enumerate_reduced_forms(disc);
    let h = forms.len() as u64;
    let id = Bqf::principal(disc).unwrap().reduce();
    let mut fac = Vec::new();
    let mut m = h;
    let mut p = 2;
    while p * p <= m {
        if m % p == 0 {
            let mut e = 0;
            while m % p == 0 {
                m /= p;
                e += 1;
            }
            fac.push((p, e));
        }
        p += 1;
    }
    if m > 1 {
        fac.push((m, 1));
    }
    // per prime: number of invariant factors divisible by p^k
    let mut invariants: Vec<u64> = Vec::new();
    let mut parts: Vec<Vec<u32>> = Vec::new(); // per prime, exponents of p-parts
    for &(p, e) in &fac {
        let mut counts = vec![1u64]; // |G[p^0]| = 1
        let mut pk = 1u64;
        for _ in 0..e {
            pk *= p;
            let c = forms.iter().filter(|f| f.pow(pk).reduce() == id).count() as u64;
            counts.push(c);
            if c == h {
                break;
            }
        }
        // r_k = log_p(counts[k]/counts[k-1]) = #invariants with p-exponent >= k
        let mut r: Vec<u32> = Vec::new();
        for k in 1..counts.len() {
            let mut q = counts[k] / counts[k - 1];
            let mut l = 0;
            while q > 1 {
                q /= p;
                l += 1;
            }
            r.push(l);
        }
        // exponents: number of invariants with exponent exactly k = r_k - r_{k+1}
        let mut exps = Vec::new();
        for k in 0..r.len() {
            let next = if k + 1 < r.len() { r[k + 1] } else { 0 };
            for _ in 0..(r[k] - next) {
                exps.push((k + 1) as u32);
            }
        }
        exps.sort_unstable_by(|a, b| b.cmp(a));
        parts.push(exps);
    }
    let maxlen = parts.iter().map(|v| v.len()).max().unwrap_or(0);
    for i in 0..maxlen {
        let mut d = 1u64;
        for (k, &(p, _)) in fac.iter().enumerate() {
            if let Some(&e) = parts[k].get(i) {
                d *= p.pow(e);
            }
        }
        invariants.push(d);
    }
    (h, invariants)
}

/// Is `D` a fundamental discriminant?  (Trial division; small `|D|` only.)
pub fn is_fundamental_small(d: i64) -> bool {
    if d >= 0 {
        return false;
    }
    let squarefree = |mut m: i64| -> bool {
        let mut p = 2;
        while p * p <= m {
            if m % (p * p) == 0 {
                return false;
            }
            while m % p == 0 {
                m /= p;
            }
            p += 1;
        }
        true
    };
    let r = d.rem_euclid(4);
    if r == 1 {
        return squarefree(-d);
    }
    if r == 0 {
        let m = d / 4;
        let mr = m.rem_euclid(4);
        return (mr == 2 || mr == 3) && squarefree(-m);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_params() -> SubexpParams {
        SubexpParams {
            trials_per_task: 256,
            euler_bound: 1 << 16,
            heldout: 8,
            parallel: false,
            ..Default::default()
        }
    }

    #[test]
    fn sqrt_mod_and_kronecker() {
        for &p in &[3u64, 5, 7, 13, 17, 41, 97, 1009, 65537] {
            for a in 1..p.min(200) {
                if jacobi(a, p) == 1 {
                    let s = sqrt_mod_prime(a, p);
                    assert_eq!(mulmod(s, s, p), a % p);
                }
            }
        }
        assert_eq!(kronecker_prime(&BigInt::from(-7), 2), 1);
        assert_eq!(kronecker_prime(&BigInt::from(-3), 2), -1);
        assert_eq!(kronecker_prime(&BigInt::from(-4), 2), 0);
    }

    #[test]
    fn tracking_reduction_represents_leading_coefficient() {
        let d = BigInt::from(-1_000_003i64 * 4 + 1);
        let f = Bqf {
            a: BigInt::from(1_234_567u64),
            b: BigInt::from(1_234_567u64 * 2 + 1),
            c: BigInt::zero(),
        };
        // fix c so that the discriminant is d
        let num = &f.b * &f.b - &d;
        let den = &f.a * 4;
        if num.mod_floor(&den).is_zero() {
            let f = Bqf {
                c: num / den,
                ..f
            };
            let (g, m) = reduce_tracking(&f);
            let val = &f.a * &m[0] * &m[0] + &f.b * &m[0] * &m[2] + &f.c * &m[2] * &m[2];
            assert_eq!(val, g.a);
            assert_eq!(g, f.reduce());
        }
        // generic: a random composed form
        let fb = FactorBase::new(&d, 200);
        let mut f = fb.primes[0].prime_form.clone();
        for p in fb.primes.iter().take(8) {
            f = compose_unreduced(&f, &p.prime_form, &d);
        }
        let (g, m) = reduce_tracking(&f);
        let val = &f.a * &m[0] * &m[0] + &f.b * &m[0] * &m[2] + &f.c * &m[2] * &m[2];
        assert_eq!(val, g.a);
        assert_eq!(&m[0] * &m[3] - &m[1] * &m[2], BigInt::one());
        assert_eq!(g, f.reduce());
    }

    #[test]
    fn small_known_class_groups() {
        // (D, h, invariants) for values that are textbook facts; the rest are
        // cross-checked against brute force only.
        let cases: &[(i64, u64, &[u64])] = &[
            (-3, 1, &[]),
            (-4, 1, &[]),
            (-23, 3, &[3]),
            (-56, 4, &[4]),
            (-84, 4, &[2, 2]),
            (-619, 5, &[5]),
        ];
        for &(d, h, inv) in cases {
            let disc = BigInt::from(d);
            let (hb, ib) = bruteforce_structure(&disc);
            assert_eq!(hb, h, "bruteforce h({d})");
            assert_eq!(ib, inv.to_vec(), "bruteforce cyc({d})");
            let cg = compute_class_group(&disc, &small_params()).unwrap();
            assert_eq!(cg.h, BigInt::from(h), "subexp h({d})");
            let got: Vec<u64> = cg.invariants.iter().map(|x| x.to_u64().unwrap()).collect();
            assert_eq!(got, inv.to_vec(), "subexp cyc({d})");
            assert!(cg.heldout_ok);
        }
    }

    /// Every fundamental discriminant with |D| < 20000: class number and
    /// invariant factors agree with brute-force form enumeration, and the
    /// discrete logs are a homomorphism (checked on held-out relations and on
    /// explicit products of factor-base forms).
    #[test]
    fn all_fundamental_discriminants_below_20000() {
        let ds: Vec<i64> = (3..20000).map(|x| -x).filter(|&d| is_fundamental_small(d)).collect();
        let worst = std::sync::Mutex::new(0.0f64);
        let failures: Vec<String> = ds
            .par_iter()
            .filter_map(|&d| {
                let disc = BigInt::from(d);
                let (h, inv) = bruteforce_structure(&disc);
                let cg = match compute_class_group(&disc, &small_params()) {
                    Ok(c) => c,
                    Err(e) => return Some(format!("{d}: error {e}")),
                };
                {
                    let mut w = worst.lock().unwrap();
                    *w = w.max(cg.log_ratio.abs());
                }
                let got: Vec<u64> = cg.invariants.iter().map(|x| x.to_u64().unwrap()).collect();
                if cg.h != BigInt::from(h) || got != inv {
                    return Some(format!("{d}: subexp {} {:?} vs brute {} {:?}", cg.h, got, h, inv));
                }
                if !cg.heldout_ok {
                    return Some(format!("{d}: held-out relation failed"));
                }
                if let Err(e) = check_dlog_homomorphism(&cg, d as u64 ^ 0x77) {
                    return Some(format!("{d}: {e}"));
                }
                None
            })
            .collect();
        eprintln!(
            "checked {} fundamental discriminants, max |log(h/h_approx)| = {:.4}",
            ds.len(),
            *worst.lock().unwrap()
        );
        assert!(failures.is_empty(), "{:?}", &failures[..failures.len().min(20)]);
    }

    /// Random products of factor-base forms: the product form is principal
    /// iff the dlog sum vanishes, and equal forms have equal dlog sums.
    fn check_dlog_homomorphism(cg: &SubexpClassGroup, seed: u64) -> Result<(), String> {
        let mut rng = SmallRng::seed_from_u64(seed);
        let d = &cg.disc;
        let n = cg.fb.len();
        let mut seen: HashMap<(BigInt, BigInt), Vec<BigInt>> = HashMap::new();
        for _ in 0..40 {
            let mut f = Bqf::principal(d).unwrap().reduce();
            let mut rel = Vec::new();
            for _ in 0..4 {
                let i = rng.gen_range(0..n);
                let e: i64 = rng.gen_range(-3..=3);
                for _ in 0..e.abs() {
                    let g = if e > 0 {
                        &cg.fb[i].form
                    } else {
                        &cg.fb[i].form_inv
                    };
                    f = compose_reduce(&f, g, d);
                }
                rel.push((i as u32, e));
            }
            let dl = cg.combine(&normalize_relation(rel));
            let principal = f.a.is_one();
            let zero = dl.iter().all(|x| x.is_zero());
            if principal != zero {
                return Err(format!("principal {principal} but dlog zero {zero}"));
            }
            let key = (f.a.clone(), f.b.clone());
            if let Some(prev) = seen.get(&key) {
                if prev != &dl {
                    return Err("same class, different dlogs".into());
                }
            } else {
                seen.insert(key, dl);
            }
        }
        // distinct classes must have distinct dlogs
        let mut inv: HashMap<Vec<BigInt>, (BigInt, BigInt)> = HashMap::new();
        for (k, v) in seen {
            if let Some(prev) = inv.insert(v, k.clone()) {
                if prev != k {
                    return Err("different classes with equal dlogs".into());
                }
            }
        }
        Ok(())
    }
}
