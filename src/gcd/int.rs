//! Integer GCD, extended GCD and modular inverse.
//!
//! Machine words: binary (Stein) GCD and extended Euclid.  `BigUint`:
//! Lehmer's algorithm (Knuth, TAOCP vol. 2, §4.5.2, Algorithm L, on the
//! top 62 bits) and, above [`HGCD_THRESHOLD_BITS`], a recursive half-GCD
//! that computes the Euclidean matrix of the top `2p` bits to strip about
//! `p/2` bits at a time (Schönhage's control structure, as in Möller, "On
//! Schönhage's algorithm and subquadratic integer gcd computation", Math.
//! Comp. 77 (2008)), so multiplication cost, not quadratic division cost,
//! dominates.
//!
//! Every matrix applied is unimodular, and the recursion applies one only
//! if it leaves `a > b ≥ 0`; otherwise that round falls back to one exact
//! division step.  So the gcd is exact whatever the truncation does, and
//! the cofactors are reduced to the classical ones at the end.

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// Bit length at and above which [`gcd_big`], [`xgcd_big`] and
/// [`modinv_big`] use the recursive half-GCD (below: Lehmer).  Measured
/// with `src/bin/hgcd_bench.rs`.
pub const HGCD_THRESHOLD_BITS: u64 = 4096;
/// Bits stripped per round below which the recursion takes Lehmer steps.
const RECURSION_MIN_BITS: u64 = 1024;

// ───────────────────────────── machine words ─────────────────────────────

/// `gcd(a, b)` by binary GCD; `gcd(0, 0) = 0`.
#[inline]
pub fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            return a << shift;
        }
    }
}

/// `gcd(a, b)` by binary GCD on 128-bit words.
pub fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            return a << shift;
        }
    }
}

/// `gcd(|a|, |b|)`.
#[inline]
pub fn gcd_i64(a: i64, b: i64) -> u64 {
    gcd_u64(a.unsigned_abs(), b.unsigned_abs())
}

/// `gcd(|a|, |b|)`.
pub fn gcd_i128(a: i128, b: i128) -> u128 {
    gcd_u128(a.unsigned_abs(), b.unsigned_abs())
}

/// `(g, x, y)` with `g = gcd(a, b) ≥ 0` and `a·x + b·y = g`: the classical
/// extended Euclidean algorithm on `(a, b)` as given (signs included), with
/// `g` made non-negative at the end.  Operands are `i64`, so the cofactors
/// fit (`|x| ≤ |b|`, `|y| ≤ |a|`).
pub fn xgcd_i64(a: i64, b: i64) -> (i64, i64, i64) {
    let (g, x, y) = xgcd_i128(a as i128, b as i128);
    (g as i64, x as i64, y as i64)
}

/// [`xgcd_i64`] on `i128` operands whose magnitudes are below `2^126`.
pub fn xgcd_i128(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1i128, 0i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if r0 < 0 {
        (-r0, -s0, -t0)
    } else {
        (r0, s0, t0)
    }
}

/// `a⁻¹ mod m` in `[0, m)` when `gcd(a, m) = 1`, else `None` (`m = 0`
/// gives `None`, `m = 1` gives `Some(0)`).
///
/// Unsigned extended Euclid on `(m, a)`: one 64-bit division per step, the
/// cofactors kept as magnitudes (their signs alternate, so the parity of
/// the step count gives the sign), no 128-bit arithmetic.
#[inline]
pub fn inv_mod_u64(a: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    if m == 1 {
        return Some(0);
    }
    let a = a % m;
    if a == 0 {
        return None;
    }
    // r_{i+1} = r_{i-1} − q·r_i, |t_{i+1}| = |t_{i-1}| + q·|t_i| (≤ m),
    // sign(t_i) = (−1)^(i+1); start (r₀, r₁) = (m, a), (t₀, t₁) = (0, 1).
    let (mut r_prev, mut r) = (m, a);
    let (mut t_prev, mut t) = (0u64, 1u64);
    let mut odd = true; // index of `r` is odd
    while r != 0 {
        let q = r_prev / r;
        (r_prev, r) = (r, r_prev - q * r);
        (t_prev, t) = (t, t_prev + q * t);
        odd = !odd;
    }
    // r_prev = gcd at index n; its cofactor t_prev has sign (−1)^(n+1),
    // positive exactly when n is odd, i.e. when `odd` (now the index of the
    // zero remainder, n + 1) is false.
    if r_prev != 1 {
        return None;
    }
    Some(if !odd { t_prev } else { m - t_prev })
}

/// `a⁻¹ mod m` in `[0, m)` for 128-bit words, `None` unless `gcd(a, m) = 1`.
pub fn inv_mod_u128(a: u128, m: u128) -> Option<u128> {
    if m == 0 {
        return None;
    }
    if m == 1 {
        return Some(0);
    }
    let a = a % m;
    if a == 0 {
        return None;
    }
    // Cofactors as (magnitude, negative) to stay within u128.
    let (mut r0, mut r1) = (m, a);
    let (mut t0, mut t0n, mut t1, mut t1n) = (0u128, false, 1u128, false);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        // t2 = t0 − q·t1, signed-magnitude, |t| ≤ m.
        let qt1 = q.wrapping_mul(t1);
        let (t2, t2n) = if t0n == t1n {
            if t0 >= qt1 {
                (t0 - qt1, t0n)
            } else {
                (qt1 - t0, !t0n)
            }
        } else {
            (t0 + qt1, t0n)
        };
        (t0, t0n, t1, t1n) = (t1, t1n, t2, t2n);
    }
    if r0 != 1 {
        return None;
    }
    Some(if t0n && t0 != 0 { m - t0 % m } else { t0 % m })
}

/// `a⁻¹ mod m` in `[0, m)` for signed operands (`m > 0`), `None` unless
/// `gcd(a, m) = 1`.
pub fn inv_mod_i64(a: i64, m: i64) -> Option<i64> {
    if m <= 0 {
        return None;
    }
    inv_mod_u64(a.rem_euclid(m) as u64, m as u64).map(|v| v as i64)
}

/// `a⁻¹ mod m` in `[0, m)` for signed 128-bit operands (`m > 0`).
pub fn inv_mod_i128(a: i128, m: i128) -> Option<i128> {
    if m <= 0 {
        return None;
    }
    inv_mod_u128(a.rem_euclid(m) as u128, m as u128).map(|v| v as i128)
}

// ─────────────────────────────── BigUint ────────────────────────────────

/// A 2×2 integer matrix `[[m00, m01], [m10, m11]]` with determinant ±1.
#[derive(Clone, Debug, PartialEq)]
struct IMat {
    m: [BigInt; 4],
}

impl IMat {
    fn identity() -> Self {
        IMat {
            m: [BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one()],
        }
    }
    /// `self·other`.
    fn mul(&self, o: &IMat) -> IMat {
        let [a, b, c, d] = &self.m;
        let [e, f, g, h] = &o.m;
        IMat {
            m: [a * e + b * g, a * f + b * h, c * e + d * g, c * f + d * h],
        }
    }
    /// `M·(x, y)ᵀ`.
    fn apply(&self, x: &BigInt, y: &BigInt) -> (BigInt, BigInt) {
        let [a, b, c, d] = &self.m;
        (a * x + b * y, c * x + d * y)
    }
    /// The quotient step `[[0, 1], [1, −q]]·self`.
    fn step(&mut self, q: &BigInt) {
        let [m00, m01, m10, m11] = std::mem::take(&mut self.m);
        let n10 = &m00 - q * &m10;
        let n11 = &m01 - q * &m11;
        self.m = [m10, m11, n10, n11];
    }
}

impl Default for IMat {
    fn default() -> Self {
        IMat::identity()
    }
}

/// The top 62 bits of `a` and the same bits of `b` (`a ≥ b`); 62 so the
/// Lehmer cofactors (bounded by the top word) stay inside `i64`.
fn top62(a: &BigUint, b: &BigUint) -> (u64, u64) {
    let shift = a.bits().saturating_sub(62);
    let ah = (a >> shift).to_u64().unwrap();
    let bh = (b >> shift).to_u64().unwrap_or(0);
    (ah, bh)
}

/// One Lehmer round (Knuth Algorithm L) on the top words of `a ≥ b`: the
/// matrix `[[A, B], [C, D]]` of the quotients certified by both
/// approximations, or `None` when no quotient is certified.
fn lehmer_matrix(a: &BigUint, b: &BigUint) -> Option<[i64; 4]> {
    let (mut ah, mut bh) = top62(a, b);
    let (mut aa, mut bb, mut cc, mut dd) = (1i64, 0i64, 0i64, 1i64);
    loop {
        let (den1, den2) = (bh as i128 + cc as i128, bh as i128 + dd as i128);
        let (num1, num2) = (ah as i128 + aa as i128, ah as i128 + bb as i128);
        if den1 <= 0 || den2 <= 0 || num1 < 0 || num2 < 0 {
            break;
        }
        let q = num1 / den1;
        if q != num2 / den2 {
            break;
        }
        let q = q as i64;
        (aa, cc) = (cc, aa - q * cc);
        (bb, dd) = (dd, bb - q * dd);
        (ah, bh) = (bh, ah - q as u64 * bh);
    }
    (bb != 0).then_some([aa, bb, cc, dd])
}

/// `(a, b) ← (b, a mod b)` with the quotient applied to `track` (the
/// cofactor pair) when given.
fn div_step(a: &mut BigInt, b: &mut BigInt, track: Option<&mut (BigInt, BigInt)>) -> BigInt {
    let (q, r) = a.div_rem(b);
    *a = std::mem::replace(b, r);
    if let Some((s0, s1)) = track {
        let s2 = &*s0 - &q * &*s1;
        *s0 = std::mem::replace(s1, s2);
    }
    q
}

/// Reduce `a > b ≥ 0` by Euclidean steps until `bits(b) ≤ stop`, returning
/// the accumulated matrix when `want` (else identity) and the new pair.
/// Lehmer rounds while the numbers are multi-word, recursion when many bits
/// remain to strip.
fn reduce(a: BigInt, b: BigInt, stop: u64, want: bool, thr: u64) -> (IMat, BigInt, BigInt) {
    let mut m = IMat::identity();
    let (mut a, mut b) = (a, b);
    while b.bits() > stop && !b.is_zero() {
        let n = a.bits();
        let p = n.saturating_sub(stop);
        if p >= RECURSION_MIN_BITS.min(thr) && n >= thr {
            // Work on the top 2p bits; strip about p/2 of them, keeping a
            // margin above the half-way point where truncated quotients
            // stop being reliable.
            let k = n.saturating_sub(2 * p);
            let (ah, bh) = (&a >> k, &b >> k);
            let target = (2 * p).min(ah.bits()) / 2 + p / 4 + 64;
            if target < ah.bits() {
                let (mr, _, _) = reduce(ah, bh, target, true, thr);
                let (c, d) = mr.apply(&a, &b);
                // Accept only a valid matrix that made progress: the
                // identity (the truncated b already below target) would
                // loop forever.
                if !c.is_negative() && !d.is_negative() && c > d && d.bits() < b.bits() {
                    a = c;
                    b = d;
                    if want {
                        m = mr.mul(&m);
                    }
                    continue;
                }
            }
        }
        if b.bits() > 64 {
            let au = a.magnitude();
            let bu = b.magnitude();
            if let Some([aa, bb, cc, dd]) = lehmer_matrix(au, bu) {
                let (a2, b2) = (
                    BigInt::from(aa) * &a + BigInt::from(bb) * &b,
                    BigInt::from(cc) * &a + BigInt::from(dd) * &b,
                );
                a = a2;
                b = b2;
                if want {
                    let l = IMat {
                        m: [aa.into(), bb.into(), cc.into(), dd.into()],
                    };
                    m = l.mul(&m);
                }
                continue;
            }
        }
        let q = {
            let (q, r) = a.div_rem(&b);
            a = std::mem::replace(&mut b, r);
            q
        };
        if want {
            m.step(&q);
        }
    }
    (m, a, b)
}

/// `gcd(a, b)`: Lehmer, recursive half-GCD from [`HGCD_THRESHOLD_BITS`].
pub fn gcd_big(a: &BigUint, b: &BigUint) -> BigUint {
    gcd_big_with_threshold(a, b, HGCD_THRESHOLD_BITS)
}

/// [`gcd_big`] with an explicit half-GCD bit threshold (`u64::MAX`: Lehmer
/// only); for benchmarks.
pub fn gcd_big_with_threshold(a: &BigUint, b: &BigUint, thr: u64) -> BigUint {
    let (x, y) = if a >= b { (a, b) } else { (b, a) };
    if y.is_zero() {
        return x.clone();
    }
    if x.bits() <= 128 {
        return BigUint::from(gcd_u128(x.to_u128().unwrap(), y.to_u128().unwrap()));
    }
    // One division first so the pair is balanced.
    let r = x % y;
    let (_, g, _) = reduce(BigInt::from(y.clone()), BigInt::from(r), 0, false, thr);
    g.into_parts().1
}

/// `(g, s, t)` with `g = gcd(a, b)` and `s·a + t·b = g`; `s, t` the
/// classical extended-Euclid cofactors, i.e. reduced so that
/// `|s| ≤ b/(2g)` and `|t| ≤ a/(2g)` (`s` taken in `(−b/(2g), b/(2g)]`);
/// `gcd(a, 0) = (a, 1, 0)`, `gcd(0, b) = (b, 0, 1)`, `(0, 0, 0)` for both
/// zero.
pub fn xgcd_big(a: &BigUint, b: &BigUint) -> (BigUint, BigInt, BigInt) {
    xgcd_big_with_threshold(a, b, HGCD_THRESHOLD_BITS)
}

/// [`xgcd_big`] with an explicit half-GCD bit threshold; for benchmarks.
pub fn xgcd_big_with_threshold(a: &BigUint, b: &BigUint, thr: u64) -> (BigUint, BigInt, BigInt) {
    if b.is_zero() {
        return if a.is_zero() {
            (BigUint::zero(), BigInt::zero(), BigInt::zero())
        } else {
            (a.clone(), BigInt::one(), BigInt::zero())
        };
    }
    if a.is_zero() {
        return (b.clone(), BigInt::zero(), BigInt::one());
    }
    let (ai, bi) = (BigInt::from(a.clone()), BigInt::from(b.clone()));
    // Euclid on (a, b) with the cofactor of a tracked through matrices.
    let (m, g) = if a >= b {
        let (m, g, _) = reduce(ai.clone(), bi.clone(), 0, true, thr);
        (m, g)
    } else {
        let (m, g, _) = reduce(bi.clone(), ai.clone(), 0, true, thr);
        // (g, 0) = M·(b, a): swap the columns to read it against (a, b).
        let [m00, m01, m10, m11] = m.m;
        (
            IMat {
                m: [m01, m00, m11, m10],
            },
            g,
        )
    };
    let s_any = m.m[0].clone();
    let g_u = g.magnitude().clone();
    let bg = &bi / &g;
    // Canonical s in (−b/(2g), b/(2g)].
    let mut s = s_any.mod_floor(&bg);
    if &s * 2 > bg {
        s -= &bg;
    }
    let t = (&g - &s * &ai) / &bi;
    (g_u, s, t)
}

/// `a⁻¹ mod m` in `[0, m)`, `None` unless `gcd(a, m) = 1` (and `None` for
/// `m = 0`; `Some(0)` for `m = 1`).
pub fn modinv_big(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    if m.is_zero() {
        return None;
    }
    if m.is_one() {
        return Some(BigUint::zero());
    }
    let ar = a % m;
    if ar.is_zero() {
        return None;
    }
    if m.bits() <= 64 {
        return inv_mod_u64(ar.to_u64().unwrap(), m.to_u64().unwrap()).map(BigUint::from);
    }
    let (g, s, _) = xgcd_big(&ar, m);
    if !g.is_one() {
        return None;
    }
    let mi = BigInt::from(m.clone());
    Some(s.mod_floor(&mi).to_biguint().unwrap())
}

/// `a⁻¹ mod m` for signed `a` (`m > 0`), in `[0, m)`.
pub fn modinv_bigint(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    if !m.is_positive() {
        return None;
    }
    let ar = a.mod_floor(m).to_biguint().unwrap();
    modinv_big(&ar, m.magnitude()).map(|v| BigInt::from_biguint(Sign::Plus, v))
}

/// Classical extended Euclid on `BigInt` (test and benchmark reference).
pub fn xgcd_big_euclid(a: &BigUint, b: &BigUint) -> (BigUint, BigInt, BigInt) {
    let (mut r0, mut r1) = (BigInt::from(a.clone()), BigInt::from(b.clone()));
    let mut s = (BigInt::one(), BigInt::zero());
    let mut t = (BigInt::zero(), BigInt::one());
    while !r1.is_zero() {
        let q = div_step(&mut r0, &mut r1, Some(&mut s));
        let t2 = &t.0 - &q * &t.1;
        t.0 = std::mem::replace(&mut t.1, t2);
    }
    (r0.magnitude().clone(), s.0, t.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::RandBigInt;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn word_gcds_match_euclid() {
        let mut rng = StdRng::seed_from_u64(20);
        let euclid = |mut a: u128, mut b: u128| {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        };
        for _ in 0..20_000 {
            let (a, b): (u64, u64) = (rng.gen(), rng.gen::<u64>() >> rng.gen_range(0..64));
            assert_eq!(gcd_u64(a, b) as u128, euclid(a as u128, b as u128));
            let (c, d): (u128, u128) = (rng.gen(), rng.gen::<u128>() >> rng.gen_range(0..128));
            assert_eq!(gcd_u128(c, d), euclid(c, d));
        }
        assert_eq!(gcd_u64(0, 0), 0);
        assert_eq!(gcd_u64(0, 7), 7);
        assert_eq!(gcd_i64(-12, 18), 6);
        assert_eq!(gcd_i64(i64::MIN, 0), 1u64 << 63);
    }

    #[test]
    fn word_inverses_and_xgcd() {
        let mut rng = StdRng::seed_from_u64(21);
        for _ in 0..20_000 {
            let m: u64 = rng.gen_range(2..u64::MAX);
            let a: u64 = rng.gen();
            match inv_mod_u64(a, m) {
                Some(x) => {
                    assert!(x < m);
                    assert_eq!((a as u128 * x as u128) % m as u128, 1);
                }
                None => assert_ne!(gcd_u64(a, m), 1),
            }
            // Small moduli and every residue: against brute force.
            let ms: u64 = rng.gen_range(2..200);
            for v in 0..ms {
                let brute = (1..ms).find(|x| (v * x) % ms == 1);
                assert_eq!(inv_mod_u64(v, ms), brute, "v={v} m={ms}");
            }
            let m2: u128 = rng.gen_range(2..u128::MAX);
            let a2: u128 = rng.gen();
            if let Some(x) = inv_mod_u128(a2, m2) {
                let (am, xm) = (BigUint::from(a2), BigUint::from(x));
                assert_eq!((am * xm) % BigUint::from(m2), BigUint::one());
            } else {
                assert_ne!(gcd_u128(a2, m2), 1);
            }
            let (x, y): (i64, i64) = (rng.gen::<i64>() >> 2, rng.gen::<i64>() >> 3);
            let (g, s, t) = xgcd_i64(x, y);
            assert_eq!(g as i128, gcd_i64(x, y) as i128);
            assert_eq!(x as i128 * s as i128 + y as i128 * t as i128, g as i128);
        }
        assert_eq!(inv_mod_u64(5, 1), Some(0));
        assert_eq!(inv_mod_u64(0, 7), None);
        assert_eq!(inv_mod_i64(-3, 7), Some(2));
    }

    /// Lehmer and half-GCD equal classical Euclid (gcd and cofactors) on
    /// random and planted-factor operands from 1 bit to 40 000 bits,
    /// including Fibonacci-like worst cases.
    #[test]
    fn big_matches_classical() {
        let mut rng = StdRng::seed_from_u64(22);
        let mut cases: Vec<(BigUint, BigUint)> = Vec::new();
        for bits in [
            1u64, 63, 64, 65, 127, 128, 129, 300, 1000, 4095, 4096, 6000, 9000, 20_000, 40_000,
        ] {
            for _ in 0..3 {
                let gb = rng.gen_range(1..=bits.max(2) / 2 + 1);
                let g = rng.gen_biguint(gb);
                let a = rng.gen_biguint(bits) * &g;
                let b = rng.gen_biguint(bits - bits / 7) * &g;
                cases.push((a, b));
            }
        }
        // Consecutive Fibonacci numbers: all quotients 1.
        let (mut f0, mut f1) = (BigUint::one(), BigUint::one());
        for _ in 0..9000 {
            let f2 = &f0 + &f1;
            f0 = std::mem::replace(&mut f1, f2);
        }
        cases.push((f1.clone(), f0.clone()));
        cases.push((BigUint::zero(), BigUint::from(5u8)));
        cases.push((BigUint::from(5u8), BigUint::zero()));
        cases.push((BigUint::from(5u8), BigUint::from(5u8)));
        for (a, b) in &cases {
            let want = xgcd_big_euclid(a, b);
            assert_eq!(gcd_big(a, b), want.0, "gcd {} {}", a.bits(), b.bits());
            // Recursion forced from 256 bits, and Lehmer only, agree too.
            assert_eq!(gcd_big_with_threshold(a, b, 256), want.0, "thr=256");
            assert_eq!(gcd_big_with_threshold(a, b, u64::MAX), want.0, "lehmer");
            assert_eq!(
                xgcd_big_with_threshold(a, b, 256),
                xgcd_big(a, b),
                "thr=256 xgcd"
            );
            let (g, s, t) = xgcd_big(a, b);
            assert_eq!(g, want.0);
            let lhs = &s * BigInt::from(a.clone()) + &t * BigInt::from(b.clone());
            assert_eq!(
                lhs,
                BigInt::from(g.clone()),
                "bezout {} {}",
                a.bits(),
                b.bits()
            );
            if !a.is_zero() && !b.is_zero() && a != b {
                // Classical cofactors up to the tie convention.
                let bg = BigInt::from(b / &g);
                assert!(BigInt::from(2u8) * s.abs() <= bg, "|s| bound");
                // Equal to classical Euclid's cofactor wherever that one is
                // strictly inside the symmetric range (it always is but for
                // the tie |s| = b/(2g)).
                if BigInt::from(2u8) * want.1.abs() < bg {
                    assert_eq!((&s, &t), (&want.1, &want.2), "classical cofactors");
                }
            }
            if !b.is_zero() {
                match modinv_big(a, b) {
                    Some(x) => assert_eq!((a * &x) % b, BigUint::one() % b),
                    None => assert!(!want.0.is_one() || b.is_one()),
                }
            }
        }
    }
}
