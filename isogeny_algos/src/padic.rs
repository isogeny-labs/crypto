//! Q_q, the unramified extension of Q_p of degree n (q = p^n), in a floating-point model, as a
//! [`Field`], so that algorithms written for fields of large characteristic (Elkies' normalised
//! codomain, the BMSS differential equation, Padé approximants) run unchanged over the p-adics.
//! This is the lifting technique of Joux–Lercier and Lercier–Sirvent (JTNB 20, 2008): lift the
//! curves of a small-characteristic problem to Q_q, where divisions by p are allowed, solve there,
//! and reduce the result modulo p.
//!
//! Representation. Z_q = Z_p[t]/(m(t)) with m a monic lift of the residue modulus m̄ (irreducible
//! of degree n over F_p). An element is p^v u with u a unit of Z_q known modulo p^N (relative
//! precision N), stored as n coefficients in [0, p^N); zero is v = [`ZERO_V`]. Products and
//! inverses keep N digits; a sum of two elements of equal valuation that cancels s digits
//! renormalises to valuation v + s with s unknown low digits filled by zeros, exactly as floating
//! point loses digits on cancellation. The model gives no guarantee on the number of correct
//! digits: callers reduce the result modulo p, verify it in the residue field, and retry at a
//! higher precision when the check fails.
//!
//! Arithmetic modulo p^N < 2^127 uses u128 coefficients: masking for p = 2, and for odd p
//! Montgomery reduction with R = 2^128 applied twice (a b = REDC(REDC(a b) R^2)).
use crate::bigint::Big;
use crate::field::{Field, Rng};
use crate::int::Int;

/// Largest extension degree n.
pub const NMAX: usize = 32;
/// Valuation of zero.
pub const ZERO_V: i32 = i32::MAX;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Qq {
    /// p-adic valuation ([`ZERO_V`] for zero)
    pub v: i32,
    /// unit part, coefficients of t^0 .. t^{n-1} modulo p^N
    pub u: [u128; NMAX],
}

fn wide_mul(a: u128, b: u128) -> (u128, u128) {
    let (a0, a1) = (a as u64 as u128, a >> 64);
    let (b0, b1) = (b as u64 as u128, b >> 64);
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;
    let mid = (p00 >> 64) + (p01 as u64 as u128) + (p10 as u64 as u128);
    let lo = (p00 as u64 as u128) | (mid << 64);
    let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (hi, lo)
}

/// Z/p^N Z with p^N < 2^127.
#[derive(Clone, Debug)]
struct Ring {
    p: u64,
    prec: u32,
    m: u128,
    pow2: bool,
    /// -m^-1 mod 2^128 (odd m)
    mprime: u128,
    /// 2^256 mod m (odd m)
    r2: u128,
}

impl Ring {
    fn new(p: u64, prec: u32) -> Ring {
        let m = (p as u128).pow(prec);
        if p == 2 {
            return Ring {
                p,
                prec,
                m,
                pow2: true,
                mprime: 0,
                r2: 0,
            };
        }
        let mut inv = m;
        for _ in 0..7 {
            inv = inv.wrapping_mul(2u128.wrapping_sub(m.wrapping_mul(inv)));
        }
        let mut r = (u128::MAX % m + 1) % m; // 2^128 mod m
        for _ in 0..128 {
            r = if r >= m - r { r - (m - r) } else { r + r };
        }
        Ring {
            p,
            prec,
            m,
            pow2: false,
            mprime: inv.wrapping_neg(),
            r2: r,
        }
    }
    #[inline]
    fn add(&self, a: u128, b: u128) -> u128 {
        if self.pow2 {
            (a + b) & (self.m - 1)
        } else if a >= self.m - b {
            a - (self.m - b)
        } else {
            a + b
        }
    }
    #[inline]
    fn neg(&self, a: u128) -> u128 {
        if a == 0 {
            0
        } else {
            self.m - a
        }
    }
    #[inline]
    fn sub(&self, a: u128, b: u128) -> u128 {
        self.add(a, self.neg(b))
    }
    #[inline]
    fn redc(&self, hi: u128, lo: u128) -> u128 {
        let q = lo.wrapping_mul(self.mprime);
        let (qh, ql) = wide_mul(q, self.m);
        let (_, carry) = lo.overflowing_add(ql);
        let t = hi + qh + carry as u128;
        if t >= self.m {
            t - self.m
        } else {
            t
        }
    }
    #[inline]
    fn mul(&self, a: u128, b: u128) -> u128 {
        if self.pow2 {
            return a.wrapping_mul(b) & (self.m - 1);
        }
        let (h, l) = wide_mul(a, b);
        let t = self.redc(h, l);
        let (h, l) = wide_mul(t, self.r2);
        self.redc(h, l)
    }
    /// Exact division of a representative by p^s (the top s digits become 0).
    fn shr(&self, a: u128, s: u32) -> u128 {
        if self.pow2 {
            a >> s
        } else {
            a / (self.p as u128).pow(s)
        }
    }
    fn mul_ppow(&self, a: u128, s: u32) -> u128 {
        if s >= self.prec {
            0
        } else {
            self.mul(a, (self.p as u128).pow(s))
        }
    }
    /// v_p(a) for a != 0 (at most prec - 1).
    fn val(&self, mut a: u128) -> u32 {
        if self.pow2 {
            return a.trailing_zeros();
        }
        let p = self.p as u128;
        let mut k = 0;
        while a.is_multiple_of(p) {
            a /= p;
            k += 1;
        }
        k
    }
}

/// The unramified extension of Q_p of degree n at relative precision N.
#[derive(Clone, Debug)]
pub struct PadicField {
    pub p: u64,
    pub n: usize,
    pub prec: u32,
    r: Ring,
    /// monic residue modulus over F_p, coefficients 0 .. n (m̄_n = 1)
    pub residue_modulus: Vec<u64>,
    /// non-zero coefficients (k, c) of m(t) - t^n, k < n, c in [0, p^N): t^n = -sum c t^k
    terms: Vec<(usize, u128)>,
}

/// Largest N with p^N < 2^127.
pub fn max_precision(p: u64) -> u32 {
    assert!(p >= 2, "p-adic characteristic must be at least 2");
    let mut n = 0u32;
    let mut x: u128 = 1;
    while let Some(y) = x.checked_mul(p as u128) {
        if y >= 1u128 << 127 {
            break;
        }
        x = y;
        n += 1;
    }
    n
}

fn sub_mod(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn pow_mod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut out = 1;
    while e > 0 {
        if e & 1 == 1 {
            out = mul_mod(out, a, p);
        }
        a = mul_mod(a, a, p);
        e >>= 1;
    }
    out
}

fn trim_poly(poly: &mut Vec<u64>) {
    while poly.len() > 1 && poly.last() == Some(&0) {
        poly.pop();
    }
}

fn poly_rem(mut a: Vec<u64>, b: &[u64], p: u64) -> Vec<u64> {
    let mut divisor = b.to_vec();
    trim_poly(&mut a);
    trim_poly(&mut divisor);
    assert!(divisor.last().is_some_and(|&c| c != 0));
    let inv_lead = pow_mod(*divisor.last().unwrap(), p - 2, p);
    while !(a.len() == 1 && a[0] == 0) && a.len() >= divisor.len() {
        let shift = a.len() - divisor.len();
        let factor = mul_mod(*a.last().unwrap(), inv_lead, p);
        for (i, &coefficient) in divisor.iter().enumerate() {
            let product = mul_mod(factor, coefficient, p);
            a[shift + i] = sub_mod(a[shift + i], product, p);
        }
        trim_poly(&mut a);
    }
    a
}

fn poly_mul_mod(a: &[u64], b: &[u64], modulus: &[u64], p: u64) -> Vec<u64> {
    let n = modulus.len() - 1;
    let mut product = vec![0u64; 2 * n - 1];
    for (i, &x) in a.iter().enumerate().take(n) {
        for (j, &y) in b.iter().enumerate().take(n) {
            let index = i + j;
            let term = mul_mod(x, y, p);
            product[index] = ((product[index] as u128 + term as u128) % p as u128) as u64;
        }
    }
    for degree in (n..product.len()).rev() {
        let leading = product[degree];
        if leading == 0 {
            continue;
        }
        for (i, &coefficient) in modulus[..n].iter().enumerate() {
            let index = degree - n + i;
            product[index] = sub_mod(product[index], mul_mod(leading, coefficient, p), p);
        }
    }
    product.truncate(n);
    trim_poly(&mut product);
    product
}

fn poly_pow_mod(mut base: Vec<u64>, mut exponent: u64, modulus: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![1];
    while exponent > 0 {
        if exponent & 1 == 1 {
            out = poly_mul_mod(&out, &base, modulus, p);
        }
        base = poly_mul_mod(&base, &base, modulus, p);
        exponent >>= 1;
    }
    out
}

/// Rabin's test for a monic polynomial over the prime field F_p.
fn residue_modulus_is_irreducible(modulus: &[u64], p: u64) -> bool {
    let n = modulus.len() - 1;
    if n == 1 {
        return true;
    }
    let x = vec![0, 1];
    let mut frobenius = x.clone();
    for _ in 0..n {
        frobenius = poly_pow_mod(frobenius, p, modulus, p);
    }
    if frobenius != x {
        return false;
    }

    let mut remaining = n;
    let mut q = 2usize;
    while q * q <= remaining {
        if remaining.is_multiple_of(q) {
            let mut power = x.clone();
            for _ in 0..(n / q) {
                power = poly_pow_mod(power, p, modulus, p);
            }
            let mut difference = power;
            if difference.len() < x.len() {
                difference.resize(x.len(), 0);
            }
            for i in 0..x.len() {
                difference[i] = sub_mod(difference[i], x[i], p);
            }
            trim_poly(&mut difference);
            let mut a = modulus.to_vec();
            let mut b = difference;
            while !(b.len() == 1 && b[0] == 0) {
                let remainder = poly_rem(a, &b, p);
                a = b;
                b = remainder;
            }
            trim_poly(&mut a);
            if a.len() != 1 {
                return false;
            }
            while remaining.is_multiple_of(q) {
                remaining /= q;
            }
        }
        q += 1;
    }
    if remaining > 1 {
        let mut power = x.clone();
        for _ in 0..(n / remaining) {
            power = poly_pow_mod(power, p, modulus, p);
        }
        let mut difference = power;
        if difference.len() < x.len() {
            difference.resize(x.len(), 0);
        }
        for i in 0..x.len() {
            difference[i] = sub_mod(difference[i], x[i], p);
        }
        trim_poly(&mut difference);
        let mut a = modulus.to_vec();
        let mut b = difference;
        while !(b.len() == 1 && b[0] == 0) {
            let remainder = poly_rem(a, &b, p);
            a = b;
            b = remainder;
        }
        trim_poly(&mut a);
        if a.len() != 1 {
            return false;
        }
    }
    true
}

impl PadicField {
    /// Q_q with residue field F_p[t]/(m̄); `residue_modulus` is m̄ monic of degree n (coefficients
    /// in [0, p), lowest first), lifted to Z_p with the same small coefficients.
    pub fn new(p: u64, residue_modulus: &[u64], prec: u32) -> PadicField {
        assert!(crate::field::is_prime(p), "p must be prime");
        assert!(
            residue_modulus.len() >= 2,
            "residue modulus must have positive degree"
        );
        let n = residue_modulus.len() - 1;
        assert!((1..=NMAX).contains(&n), "extension degree {n} out of range");
        assert!(
            residue_modulus.iter().all(|&c| c < p),
            "residue modulus coefficients must be in [0, p)"
        );
        assert_eq!(residue_modulus[n], 1, "residue modulus must be monic");
        assert!(
            residue_modulus_is_irreducible(residue_modulus, p),
            "residue modulus must be irreducible over F_p"
        );
        assert!(
            prec >= 1 && prec <= max_precision(p),
            "precision {prec} out of range for p = {p}"
        );
        let r = Ring::new(p, prec);
        let terms = (0..n)
            .filter(|&k| !residue_modulus[k].is_multiple_of(p))
            .map(|k| (k, (residue_modulus[k] % p) as u128))
            .collect();
        PadicField {
            p,
            n,
            prec,
            r,
            residue_modulus: residue_modulus.iter().map(|c| c % p).collect(),
            terms,
        }
    }

    /// Q_p itself.
    pub fn prime(p: u64, prec: u32) -> PadicField {
        PadicField::new(p, &[0, 1], prec)
    }

    fn zero_e() -> Qq {
        Qq {
            v: ZERO_V,
            u: [0; NMAX],
        }
    }

    /// p^v c for an arbitrary coefficient vector c (not necessarily a unit): extract the common
    /// power of p.
    fn normalise(&self, v: i32, mut c: [u128; NMAX]) -> Qq {
        let mut s = u32::MAX;
        for &x in &c[..self.n] {
            if x != 0 {
                s = s.min(self.r.val(x));
            }
        }
        if s == u32::MAX {
            return Self::zero_e();
        }
        if s > 0 {
            for x in c[..self.n].iter_mut() {
                *x = self.r.shr(*x, s);
            }
        }
        Qq {
            v: v.saturating_add(s as i32),
            u: c,
        }
    }

    fn umul(&self, a: &[u128; NMAX], b: &[u128; NMAX]) -> [u128; NMAX] {
        let n = self.n;
        if n == 1 {
            let mut o = [0; NMAX];
            o[0] = self.r.mul(a[0], b[0]);
            return o;
        }
        let mut t = [0u128; 2 * NMAX];
        for i in 0..n {
            if a[i] == 0 {
                continue;
            }
            for j in 0..n {
                t[i + j] = self.r.add(t[i + j], self.r.mul(a[i], b[j]));
            }
        }
        for k in (n..2 * n - 1).rev() {
            let c = t[k];
            if c == 0 {
                continue;
            }
            t[k] = 0;
            for &(i, mc) in &self.terms {
                let d = k - n + i;
                t[d] = self.r.sub(t[d], self.r.mul(c, mc));
            }
        }
        let mut o = [0; NMAX];
        o[..n].copy_from_slice(&t[..n]);
        o
    }

    /// Inverse of a unit: inverse of the residue by the extended Euclidean algorithm over F_p,
    /// then Newton's iteration y <- y (2 - u y), doubling the number of correct digits.
    fn uinv(&self, u: &[u128; NMAX]) -> [u128; NMAX] {
        let p = self.p;
        let res: Vec<u64> = u[..self.n]
            .iter()
            .map(|&c| (c % p as u128) as u64)
            .collect();
        let ri = poly_inv_mod_p(&res, &self.residue_modulus, p);
        let mut y = [0u128; NMAX];
        for (k, &c) in ri.iter().enumerate() {
            y[k] = c as u128;
        }
        let mut digits = 1u32;
        while digits < self.prec {
            let uy = self.umul(u, &y);
            let mut two_minus = [0u128; NMAX];
            for k in 0..self.n {
                two_minus[k] = self.r.neg(uy[k]);
            }
            two_minus[0] = self.r.add(two_minus[0], 2);
            y = self.umul(&y, &two_minus);
            digits *= 2;
        }
        y
    }

    /// The element of Z_q with the given residue-field coordinates (digits in [0, p)).
    pub fn from_residue(&self, digits: &[u64]) -> Qq {
        let mut c = [0u128; NMAX];
        for (k, &d) in digits.iter().enumerate().take(self.n) {
            c[k] = (d % self.p) as u128;
        }
        self.normalise(0, c)
    }

    /// Residue-field coordinates of an element of Z_q; None when the valuation is negative.
    pub fn to_residue(&self, a: Qq) -> Option<Vec<u64>> {
        if a.v == ZERO_V || a.v > 0 {
            return Some(vec![0; self.n]);
        }
        if a.v < 0 {
            return None;
        }
        Some(
            a.u[..self.n]
                .iter()
                .map(|&c| (c % self.p as u128) as u64)
                .collect(),
        )
    }

    /// A rational integer.
    pub fn from_int(&self, x: &Int) -> Qq {
        if x.is_zero() {
            return Self::zero_e();
        }
        let pi = Int::from(self.p as i64);
        let mut y = x.clone();
        let mut v = 0i32;
        loop {
            let (q, r) = y.divrem_trunc(&pi);
            if !r.is_zero() {
                break;
            }
            y = q;
            v += 1;
            if v >= self.prec as i32 + 64 {
                return Self::zero_e();
            }
        }
        let m = Int::from_big(&Big::from_u128(self.r.m));
        let red = y.modulo(&m);
        let mut c = [0u128; NMAX];
        c[0] = red.to_i128().expect("residue below 2^127") as u128;
        Qq { v, u: c }
    }

    pub fn valuation(&self, a: Qq) -> i32 {
        a.v
    }
}

/// Inverse of a (non-zero, deg < n) modulo the monic m over F_p (extended Euclid).
fn poly_inv_mod_p(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let n = m.len() - 1;
    let mulm = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
    let inv = |x: u64| -> u64 {
        let mut r = 1u64;
        let mut b = x % p;
        let mut e = p - 2;
        while e > 0 {
            if e & 1 == 1 {
                r = mulm(r, b);
            }
            b = mulm(b, b);
            e >>= 1;
        }
        r
    };
    let trim = |v: &mut Vec<u64>| {
        while v.len() > 1 && *v.last().unwrap() == 0 {
            v.pop();
        }
    };
    // r0 = m, r1 = a; s0 = 0, s1 = 1 (coefficient of a)
    let mut r0: Vec<u64> = m.to_vec();
    let mut r1: Vec<u64> = a.iter().map(|&c| c % p).collect();
    trim(&mut r1);
    let mut s0: Vec<u64> = vec![0];
    let mut s1: Vec<u64> = vec![1];
    while !(r1.len() == 1 && r1[0] == 0) {
        // q, r = divmod(r0, r1)
        let mut rem = r0.clone();
        let d1 = r1.len() - 1;
        let li = inv(*r1.last().unwrap());
        let mut q = vec![0u64; rem.len().saturating_sub(d1).max(1)];
        while rem.len() > d1 && !(rem.len() == 1 && rem[0] == 0) {
            let k = rem.len() - 1 - d1;
            let c = mulm(*rem.last().unwrap(), li);
            q[k] = c;
            for i in 0..=d1 {
                let t = mulm(c, r1[i]);
                rem[k + i] = (rem[k + i] + p - t) % p;
            }
            rem.pop();
            if rem.is_empty() {
                rem.push(0);
            }
        }
        trim(&mut rem);
        // s2 = s0 - q s1
        let mut qs = vec![0u64; q.len() + s1.len()];
        for (i, &x) in q.iter().enumerate() {
            for (j, &y) in s1.iter().enumerate() {
                qs[i + j] = (qs[i + j] + mulm(x, y)) % p;
            }
        }
        let len = qs.len().max(s0.len());
        let mut s2 = vec![0u64; len];
        for i in 0..len {
            let a0 = s0.get(i).copied().unwrap_or(0);
            let b0 = qs.get(i).copied().unwrap_or(0);
            s2[i] = (a0 + p - b0) % p;
        }
        trim(&mut s2);
        r0 = r1;
        r1 = rem;
        s0 = s1;
        s1 = s2;
    }
    // r0 is a non-zero constant (m irreducible, a != 0)
    assert!(r0.len() == 1 && r0[0] != 0, "residue not invertible");
    let c = inv(r0[0]);
    let mut out = vec![0u64; n];
    for (i, &x) in s0.iter().enumerate().take(n) {
        out[i] = mulm(x, c);
    }
    out
}

impl Field for PadicField {
    type E = Qq;
    fn zero(&self) -> Qq {
        Self::zero_e()
    }
    fn one(&self) -> Qq {
        let mut u = [0; NMAX];
        u[0] = 1;
        Qq { v: 0, u }
    }
    fn add(&self, a: Qq, b: Qq) -> Qq {
        if a.v == ZERO_V {
            return b;
        }
        if b.v == ZERO_V {
            return a;
        }
        let (a, b) = if a.v <= b.v { (a, b) } else { (b, a) };
        let d = (b.v as i64 - a.v as i64) as u64;
        if d >= self.prec as u64 {
            return a;
        }
        let mut c = a.u;
        for k in 0..self.n {
            c[k] = self.r.add(c[k], self.r.mul_ppow(b.u[k], d as u32));
        }
        if d > 0 {
            // the sum is still a unit times p^{a.v}
            return Qq { v: a.v, u: c };
        }
        self.normalise(a.v, c)
    }
    fn sub(&self, a: Qq, b: Qq) -> Qq {
        self.add(a, self.neg(b))
    }
    fn neg(&self, a: Qq) -> Qq {
        if a.v == ZERO_V {
            return a;
        }
        let mut u = a.u;
        for x in u[..self.n].iter_mut() {
            *x = self.r.neg(*x);
        }
        Qq { v: a.v, u }
    }
    fn mul(&self, a: Qq, b: Qq) -> Qq {
        if a.v == ZERO_V || b.v == ZERO_V {
            return Self::zero_e();
        }
        Qq {
            v: a.v + b.v,
            u: self.umul(&a.u, &b.u),
        }
    }
    fn inv(&self, a: Qq) -> Qq {
        assert!(a.v != ZERO_V, "inverse of zero in Q_q");
        Qq {
            v: -a.v,
            u: self.uinv(&a.u),
        }
    }
    fn from_u64(&self, n: u64) -> Qq {
        let mut v = 0i32;
        let mut x = n;
        if x == 0 {
            return Self::zero_e();
        }
        while x.is_multiple_of(self.p) {
            x /= self.p;
            v += 1;
        }
        let mut u = [0; NMAX];
        u[0] = x as u128 % self.r.m;
        Qq { v, u }
    }
    fn char(&self) -> u64 {
        0
    }
    fn characteristic_exceeds(&self, _n: u64) -> bool {
        true
    }
    fn size(&self) -> u128 {
        u128::MAX
    }
    /// A random unit.
    fn random(&self, rng: &mut Rng) -> Qq {
        loop {
            let mut c = [0u128; NMAX];
            for x in c[..self.n].iter_mut() {
                *x = ((rng.next() as u128) << 64 | rng.next() as u128) % self.r.m;
            }
            let e = self.normalise(0, c);
            if e.v == 0 {
                return e;
            }
        }
    }
    fn sqrt(&self, _a: Qq) -> Option<Qq> {
        unimplemented!("square roots are not needed in Q_q here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_arithmetic_matches_big_integers() {
        let mut rng = Rng::new(1);
        for &(p, prec) in &[(2u64, 127u32), (3, 80), (5, 54), (7, 45), (101, 19)] {
            let r = Ring::new(p, prec);
            let mb = Int::from_big(&Big::from_u128(r.m));
            for _ in 0..200 {
                let a = ((rng.next() as u128) << 64 | rng.next() as u128) % r.m;
                let b = ((rng.next() as u128) << 64 | rng.next() as u128) % r.m;
                let want = (&Int::from_big(&Big::from_u128(a))
                    * &Int::from_big(&Big::from_u128(b)))
                    .modulo(&mb);
                assert_eq!(Int::from_big(&Big::from_u128(r.mul(a, b))), want, "p = {p}");
            }
        }
    }

    #[test]
    fn field_axioms_in_an_unramified_extension() {
        // F_8 = F_2[t]/(t^3 + t + 1) lifted to Q_8 at 100 bits; F_9 = F_3[t]/(t^2 + 1).
        for (p, m, prec) in [(2u64, vec![1u64, 1, 0, 1], 100u32), (3, vec![1, 0, 1], 60)] {
            let f = PadicField::new(p, &m, prec);
            let mut rng = Rng::new(7);
            for _ in 0..50 {
                let a = f.random(&mut rng);
                let b = f.random(&mut rng);
                let c = f.mul(f.from_u64(p * p), f.random(&mut rng));
                assert_eq!(f.mul(f.div(a, b), b), a);
                assert_eq!(f.mul(f.inv(c), c), f.one());
                let s = f.add(a, c);
                assert_eq!(f.sub(s, c), a);
                assert_eq!(f.valuation(c), 2);
                let pinv = f.inv(f.from_u64(p));
                assert_eq!(f.valuation(pinv), -1);
                assert_eq!(f.mul(pinv, f.from_u64(p)), f.one());
            }
        }
    }

    #[test]
    fn integers_and_residues_round_trip() {
        let f = PadicField::new(2, &[1, 1, 0, 1], 90);
        let x = Int::from(-1728i64);
        let a = f.from_int(&x);
        assert_eq!(a.v, 6);
        assert_eq!(f.add(a, f.from_u64(1728)), f.zero());
        let r = f.from_residue(&[1, 0, 1]);
        assert_eq!(f.to_residue(r), Some(vec![1, 0, 1]));
        assert_eq!(f.to_residue(f.inv(f.from_u64(2))), None);
    }

    #[test]
    fn constructor_rejects_invalid_residue_fields() {
        for (p, modulus) in [
            (4, &[1, 1][..]),       // composite characteristic
            (2, &[][..]),           // no modulus
            (2, &[1][..]),          // degree zero
            (2, &[0, 0, 1][..]),    // reducible: t^2
            (2, &[1, 0, 0, 1][..]), // reducible: (t + 1)(t^2 + t + 1)
            (2, &[2, 1][..]),       // noncanonical coefficient
            (3, &[1, 2][..]),       // not monic
        ] {
            assert!(std::panic::catch_unwind(|| PadicField::new(p, modulus, 8)).is_err());
        }

        // These are irreducible over their respective prime fields.
        PadicField::new(2, &[1, 1, 0, 1], 8);
        PadicField::new(3, &[1, 0, 1], 8);
    }

    #[test]
    #[should_panic(expected = "p-adic characteristic must be at least 2")]
    fn max_precision_rejects_zero_characteristic() {
        max_precision(0);
    }

    #[test]
    #[should_panic(expected = "p-adic characteristic must be at least 2")]
    fn max_precision_rejects_unit_characteristic() {
        max_precision(1);
    }
}
