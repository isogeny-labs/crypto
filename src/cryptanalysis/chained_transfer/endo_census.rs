//! Census of the class-wide minimal endomorphism degree of ordinary `E/F_p`.
//!
//! For an ordinary curve with trace `t`, the Frobenius discriminant is
//! `Δ = t² − 4p = d_K · f²` with `d_K` fundamental (the discriminant of
//! `Q(π)`) and `f` the conductor of `Z[π]`. `End(E)` is an order of
//! discriminant `d_K c²` with `c | f`. A non-rational endomorphism of any
//! order inside `O_K` has norm at least the minimum non-rational norm of
//! `O_K` itself ([`min_degree`]), attained on the crater (`c = 1`).
//!
//! Testing `|d_K| ≤ B` needs no factorisation: for each fundamental
//! discriminant `d` with `|d| ≤ B`, `Δ/d` is either a perfect square integer
//! (then `d_K = d`) or it is not. The fundamental `d` with `Δ = d f²` is
//! unique, so at most one test passes.
//!
//! [`expected_fraction`] is the exact probability, for a uniformly random
//! `(a, b)` over `F_p`, that `|d_K| ≤ B`. By Deuring and Lenstra the number
//! of `(a, b)` with trace `t` is `(p − 1) H(4p − t²) / 2`, `H` the
//! Hurwitz–Kronecker class number, so
//! `P(d_K = d) = (1/2p) Σ_{t² + |d| f² = 4p} H(|d| f²)` with
//! `H(|d| f²) = Σ_{c | f} h(d) c Π_{q | c}(1 − (d/q)/q) / (w(d)/2)`.
//! (`predict.py` in the research directory is an independent implementation;
//! a unit test checks this function against an exhaustive enumeration.)

use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};

/// Negative fundamental discriminants `d` with `|d| ≤ max_abs`, ascending in `|d|`.
pub fn fundamental_discriminants(max_abs: i64) -> Vec<i64> {
    fn squarefree(n: i64) -> bool {
        let mut q = 2;
        while q * q <= n {
            if n % (q * q) == 0 {
                return false;
            }
            q += 1;
        }
        true
    }
    let mut out = Vec::new();
    for n in 3..=max_abs {
        let d = -n;
        if d.rem_euclid(4) == 1 {
            if squarefree(n) {
                out.push(d);
            }
        } else if d % 4 == 0 {
            let m = d / 4;
            if matches!(m.rem_euclid(4), 2 | 3) && squarefree(-m) {
                out.push(d);
            }
        }
    }
    out
}

/// Class number `h(d)` of a fundamental discriminant `d < 0` (reduced forms).
pub fn class_number(d: i64) -> u64 {
    let mut cnt = 0;
    let mut a = 1i64;
    while 3 * a * a <= -d {
        for b in (-a + 1)..=a {
            if (b - d) % 2 != 0 {
                continue;
            }
            let num = b * b - d;
            if num % (4 * a) != 0 {
                continue;
            }
            let c = num / (4 * a);
            if c < a || (a == c && b < 0) {
                continue;
            }
            cnt += 1;
        }
        a += 1;
    }
    cnt
}

/// Minimal degree (norm) of a non-rational endomorphism in the quadratic order
/// of discriminant `disc < 0` (`disc ≡ 0, 1 mod 4`): `|disc|/4` for even
/// `disc`, `(|disc| + 1)/4` for odd. A degree-1 element is an automorphism
/// (`disc = −3, −4`).
pub fn min_degree(disc: i64) -> u64 {
    let a = disc.unsigned_abs();
    if disc.rem_euclid(4) == 0 {
        a / 4
    } else {
        (a + 1) / 4
    }
}

/// Kronecker symbol `(d / q)` for a prime `q` and a discriminant `d`.
pub fn kronecker(d: i64, q: u64) -> i64 {
    if q == 2 {
        if d % 2 == 0 {
            return 0;
        }
        return if matches!(d.rem_euclid(8), 1 | 7) {
            1
        } else {
            -1
        };
    }
    let mut r = 1u128;
    let mut b = (d.rem_euclid(q as i64)) as u128;
    let mut e = (q - 1) / 2;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % q as u128;
        }
        b = b * b % q as u128;
        e >>= 1;
    }
    match r {
        0 => 0,
        1 => 1,
        _ => -1,
    }
}

/// Weighted class count of orders `⊇ Z[π]` for `Δ = d f²`:
/// `Σ_{c | f} h(d) c Π_{q | c}(1 − (d/q)/q) / (w(d)/2)`.
fn hurwitz_part(d: i64, f: u64, h: u64) -> f64 {
    let w2 = match d {
        -3 => 3.0,
        -4 => 2.0,
        _ => 1.0,
    };
    let mut n = f;
    let mut tot = 1.0f64;
    let mut q = 2u64;
    let mut facs: Vec<(u64, u32)> = Vec::new();
    while q * q <= n {
        if n.is_multiple_of(q) {
            let mut e = 0;
            while n.is_multiple_of(q) {
                n /= q;
                e += 1;
            }
            facs.push((q, e));
        }
        q += 1;
    }
    if n > 1 {
        facs.push((n, 1));
    }
    for (q, e) in facs {
        let mut s = 1.0;
        for k in 1..=e {
            s += (q as f64).powi(k as i32) * (1.0 - kronecker(d, q) as f64 / q as f64);
        }
        tot *= s;
    }
    h as f64 * tot / w2
}

/// Exact `P(d_K = d)` for each `d` in `discs`, for uniform `(a, b)` over `F_p`
/// (`p > 3` prime). The sum of the returned vector is `P(|d_K| ≤ max |d|)`.
pub fn expected_by_d(p: u64, discs: &[i64]) -> Vec<f64> {
    let four_p = 4 * p as u128;
    discs
        .iter()
        .map(|&d| {
            let h = class_number(d);
            let ad = d.unsigned_abs() as u128;
            let mut tot = 0.0;
            let mut f = 1u128;
            while ad * f * f < four_p {
                let s = four_p - ad * f * f;
                let t = isqrt_u128(s);
                if t * t == s && t > 0 {
                    tot += 2.0 * hurwitz_part(d, f as u64, h);
                }
                f += 1;
            }
            tot / (2.0 * p as f64)
        })
        .collect()
}

/// `P(|d_K| ≤ B)` for `p`, `B` given by the discriminant list.
pub fn expected_fraction(p: u64, discs: &[i64]) -> f64 {
    expected_by_d(p, discs).iter().sum()
}

/// Floor square root of a `u128`.
pub fn isqrt_u128(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut s = (n as f64).sqrt() as u128;
    while s * s > n {
        s -= 1;
    }
    while (s + 1) * (s + 1) <= n {
        s += 1;
    }
    s
}

/// What the test found for one `(p, t)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Analysis {
    /// `p | t`: supersingular for `p > 3` (`t = 0`); `d_K` is not defined.
    pub supersingular: bool,
    /// The fundamental discriminant of `Δ`, when `|d_K|` is within the search bound.
    pub d_k: Option<i64>,
    /// `f` with `Δ = d_K f²`, when `d_k` is found.
    pub f: Option<BigInt>,
    /// The search bound `B` actually used (`|d| ≤ B`).
    pub bound: i64,
}

impl Analysis {
    /// Minimal non-rational endomorphism degree over the isogeny class (the crater).
    pub fn class_min_degree(&self) -> Option<u64> {
        self.d_k.map(min_degree)
    }
    /// `|d_K| ≤ 44` (the degree-≤-11 test).
    pub fn within_44(&self) -> bool {
        matches!(self.d_k, Some(d) if d.abs() <= 44)
    }
}

/// Test `Δ = t² − 4p` against the fundamental discriminants in `discs`
/// (ascending in `|d|`; `bound` is `max |d|`). `discs` restricted to `|d| ≤ 44`
/// is used when `Δ` does not fit an `i128`.
pub fn analyse(p: &BigInt, t: &BigInt, discs: &[i64]) -> Analysis {
    let delta = t * t - BigInt::from(4) * p;
    let supersingular = (t % p).is_zero();
    let bound_full = discs.last().map(|d| d.abs()).unwrap_or(0);
    if supersingular || !delta.is_negative() {
        return Analysis {
            supersingular,
            d_k: None,
            f: None,
            bound: bound_full,
        };
    }
    if let Some(dl) = delta.to_i128() {
        for &d in discs {
            let dd = d as i128;
            if dl % dd == 0 {
                let q = (dl / dd) as u128;
                let r = isqrt_u128(q);
                if r * r == q {
                    return Analysis {
                        supersingular,
                        d_k: Some(d),
                        f: Some(BigInt::from(r)),
                        bound: bound_full,
                    };
                }
            }
        }
        return Analysis {
            supersingular,
            d_k: None,
            f: None,
            bound: bound_full,
        };
    }
    let mut bound = 0;
    for &d in discs.iter().take_while(|d| d.abs() <= 44) {
        bound = d.abs();
        let dd = BigInt::from(d);
        if (&delta % &dd).is_zero() {
            let q = &delta / &dd;
            let r = q.sqrt();
            if &r * &r == q {
                return Analysis {
                    supersingular,
                    d_k: Some(d),
                    f: Some(r),
                    bound,
                };
            }
        }
    }
    Analysis {
        supersingular,
        d_k: None,
        f: None,
        bound,
    }
}

/// `j(E) mod p` for `y² = x³ + ax + b` over `F_p` (`None` when singular).
pub fn j_mod_p(p: &BigInt, a: &BigInt, b: &BigInt) -> Option<BigInt> {
    let a3 = a.modpow(&BigInt::from(3), p);
    let num = (BigInt::from(1728) * BigInt::from(4) * &a3).modpow(&BigInt::from(1), p);
    let den = (BigInt::from(4) * &a3 + BigInt::from(27) * b * b).modpow(&BigInt::from(1), p);
    if den.is_zero() {
        return None;
    }
    let inv = den.modpow(&(p - BigInt::from(2)), p);
    Some((num * inv).modpow(&BigInt::from(1), p))
}

/// Class-number-one fundamental discriminants with their CM `j`-invariants.
pub const CLASS_NUMBER_ONE: [(i64, i64); 7] = [
    (-3, 0),
    (-4, 1_728),
    (-7, -3_375),
    (-8, 8_000),
    (-11, -32_768),
    (-19, -884_736),
    (-43, -884_736_000),
];

/// Whether `End(E)` is the maximal order, decided exactly when `h(d_K) = 1`
/// (`j(E) ≡ j_{d_K} mod p`); `None` for other `d_K` (class bound only).
pub fn on_crater(p: &BigInt, a: &BigInt, b: &BigInt, d_k: i64) -> Option<bool> {
    let (_, jd) = CLASS_NUMBER_ONE.iter().find(|(d, _)| *d == d_k)?;
    let j = j_mod_p(p, a, b)?;
    let jd = BigInt::from(*jd).modpow(&BigInt::from(1), p);
    Some(j == jd)
}

/// `(a, b)` with `j(E) = j mod p` for `j ∉ {0, 1728}`: `y² = x³ + 3kx + 2k`,
/// `k = j/(1728 − j)`.
pub fn curve_with_j(p: u64, j: i64) -> Option<(u64, u64)> {
    let pp = p as i128;
    let jm = (j as i128).rem_euclid(pp);
    let den = (1728 - jm).rem_euclid(pp);
    if den == 0 || jm == 0 {
        return None;
    }
    let mut inv = 1i128;
    let (mut base, mut e) = (den, pp - 2);
    while e > 0 {
        if e & 1 == 1 {
            inv = inv * base % pp;
        }
        base = base * base % pp;
        e >>= 1;
    }
    let k = jm * inv % pp;
    if (k + 1) % pp == 0 {
        return None;
    }
    Some(((3 * k % pp) as u64, (2 * k % pp) as u64))
}

/// Frobenius trace of `y² = x³ + ax + b` over `F_p`, `p` prime `< 2^62`,
/// by the crate's Shanks–Mestre counter in [`crate::isogeny::cm`]. Below `2^20`
/// the crate's `frobenius_trace` (with its Legendre-sum fallback) is used; above,
/// BSGS only, retried once on a quadratic twist (`t → −t`), `None` if both fail.
pub fn count_trace(p: u64, a: u64, b: u64) -> Option<i64> {
    use crate::isogeny::{cm, SmallCurve};
    let c = SmallCurve {
        name: "census",
        p,
        a,
        b,
    };
    if p < (1 << 20) {
        return Some(cm::frobenius_trace(&c));
    }
    if let Some(t) = cm::frobenius_trace_bsgs(&c) {
        return Some(t);
    }
    // Twist by the least non-residue d: (a d², b d³) has trace −t.
    let mut d = 2u64;
    while cm::legendre_u64(d, p) != -1 {
        d += 1;
    }
    let pp = p as u128;
    let d2 = d as u128 * d as u128 % pp;
    let d3 = d2 * d as u128 % pp;
    let tw = SmallCurve {
        name: "census-twist",
        p,
        a: (a as u128 * d2 % pp) as u64,
        b: (b as u128 * d3 % pp) as u64,
    };
    cm::frobenius_trace_bsgs(&tw).map(|t| -t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big(x: i128) -> BigInt {
        BigInt::from(x)
    }

    #[test]
    fn fundamental_discriminant_list_up_to_44() {
        assert_eq!(
            fundamental_discriminants(44),
            vec![-3, -4, -7, -8, -11, -15, -19, -20, -23, -24, -31, -35, -39, -40, -43]
        );
    }

    #[test]
    fn class_numbers_and_min_degrees() {
        for (d, h) in [
            (-3, 1),
            (-4, 1),
            (-7, 1),
            (-8, 1),
            (-11, 1),
            (-15, 2),
            (-23, 3),
            (-39, 4),
            (-43, 1),
        ] {
            assert_eq!(class_number(d), h, "h({d})");
        }
        // Hand-checked: omega = (1+sqrt d)/2 has norm (1-d)/4; sqrt(d/4) has norm |d|/4.
        let want = [
            (-3, 1),
            (-4, 1),
            (-7, 2),
            (-8, 2),
            (-11, 3),
            (-15, 4),
            (-19, 5),
            (-20, 5),
            (-43, 11),
        ];
        for (d, m) in want {
            assert_eq!(min_degree(d), m, "min degree of {d}");
        }
    }

    /// Hand-checked: p = 13, t = 4: Δ = 16 − 52 = −36 = (−4)·9, so d_K = −4, f = 3,
    /// and -36 / -3 = 12 is not a square, so only d = −4 passes.
    #[test]
    fn delta_over_d_square_hand_example() {
        let discs = fundamental_discriminants(44);
        let a = analyse(&big(13), &big(4), &discs);
        assert_eq!(a.d_k, Some(-4));
        assert_eq!(a.f, Some(big(3)));
        assert_eq!(a.class_min_degree(), Some(1));
        // p = 7, t = 1: Δ = −27 = −3 · 9.
        let a = analyse(&big(7), &big(1), &discs);
        assert_eq!((a.d_k, a.f), (Some(-3), Some(big(3))));
        // p = 101, t = 5: Δ = −379 prime ⇒ d_K = −379 (> 44).
        assert_eq!(analyse(&big(101), &big(5), &discs).d_k, None);
    }

    #[test]
    fn supersingular_is_flagged() {
        let discs = fundamental_discriminants(44);
        assert!(analyse(&big(101), &big(0), &discs).supersingular);
    }

    /// Positive controls: CM curves for D = −3, −4, −7, −8, −11, −19, −43 are
    /// detected with d_K = D on the crater, over several primes and twists.
    #[test]
    fn cm_controls_are_detected() {
        let discs = fundamental_discriminants(44);
        for &(dk, j) in CLASS_NUMBER_ONE.iter() {
            let mut seen = 0;
            let mut p = 1_000_003u64;
            while seen < 3 {
                p += 2;
                if !crate::cryptanalysis::chained_transfer::field::is_prime(p)
                    || kronecker(dk, p) != 1
                {
                    continue;
                }
                let (a, b) = match dk {
                    -3 => (0, 7),
                    -4 => (5, 0),
                    _ => curve_with_j(p, j).expect("nondegenerate"),
                };
                let t = count_trace(p, a, b).expect("count");
                let an = analyse(&big(p as i128), &big(t as i128), &discs);
                assert_eq!(an.d_k, Some(dk), "p={p} t={t}");
                assert_eq!(
                    on_crater(&big(p as i128), &big(a as i128), &big(b as i128), dk),
                    Some(true)
                );
                seen += 1;
            }
        }
    }

    /// Non-maximal CM curves (D = −12 has d_K = −3, conductor 2) are not on the crater.
    #[test]
    fn non_maximal_control_is_not_crater() {
        let discs = fundamental_discriminants(44);
        let mut p = 1_000_003u64;
        loop {
            p += 2;
            if crate::cryptanalysis::chained_transfer::field::is_prime(p) && kronecker(-3, p) == 1 {
                break;
            }
        }
        let (a, b) = curve_with_j(p, 54_000).unwrap();
        let t = count_trace(p, a, b).unwrap();
        let an = analyse(&big(p as i128), &big(t as i128), &discs);
        assert_eq!(an.d_k, Some(-3));
        assert_eq!(
            on_crater(&big(p as i128), &big(a as i128), &big(b as i128), -3),
            Some(false)
        );
    }

    fn legendre_trace(p: u64, a: u64, b: u64) -> i64 {
        let chi = |v: u64| -> i64 {
            if v == 0 {
                return 0;
            }
            let (mut r, mut x, mut e) = (1u128, v as u128, (p - 1) / 2);
            while e > 0 {
                if e & 1 == 1 {
                    r = r * x % p as u128;
                }
                x = x * x % p as u128;
                e >>= 1;
            }
            if r == 1 {
                1
            } else {
                -1
            }
        };
        let mut s = 0i64;
        for x in 0..p {
            let v = ((x as u128 * x as u128 % p as u128 * x as u128
                + a as u128 * x as u128
                + b as u128)
                % p as u128) as u64;
            s += chi(v);
        }
        -s
    }

    #[test]
    fn bsgs_counter_matches_legendre_sum() {
        let p = 1_048_583u64; // prime > 2^20, exercises the BSGS path
        assert!(crate::cryptanalysis::chained_transfer::field::is_prime(p));
        for (a, b) in [(1u64, 1u64), (123456, 654321), (7, 99)] {
            assert_eq!(
                count_trace(p, a, b),
                Some(legendre_trace(p, a, b)),
                "a={a} b={b}"
            );
        }
    }

    /// The exact formula equals an exhaustive enumeration of every (a, b) for two primes.
    #[test]
    fn expected_fraction_matches_exhaustive_enumeration() {
        let discs = fundamental_discriminants(44);
        for p in [101u64, 103] {
            let (mut hit, mut tot) = (0u64, 0u64);
            for a in 0..p {
                for b in 0..p {
                    if (4 * a * a % p * a + 27 * b * b) % p == 0 {
                        continue;
                    }
                    tot += 1;
                    let t = legendre_trace(p, a, b);
                    if analyse(&big(p as i128), &big(t as i128), &discs).within_44() {
                        hit += 1;
                    }
                }
            }
            let want = hit as f64 / tot as f64;
            let got = expected_fraction(p, &discs);
            assert!(
                (want - got).abs() < 1e-12,
                "p={p} enumerated {want} formula {got}"
            );
        }
    }
}
