//! Toy finite fields: F_p (p < 2^17; products and their sums here are u128), K4 = F_p[t]/(m(t)) with m the
//! minimal polynomial of alpha = t, and L = K4[s]/(s^2 - nu) = F_{p^8}.
//! Every element of F_{p^4} is a K4 element; every square root of a K4
//! element lives in L.

pub fn addm(a: u64, b: u64, p: u64) -> u64 {
    let c = a + b;
    if c >= p {
        c - p
    } else {
        c
    }
}
pub fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}
pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn powm(mut a: u64, mut e: u128, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero mod {p}");
    powm(a, (p - 2) as u128, p)
}
/// Legendre symbol (0, 1 or -1).
pub fn legendre(a: u64, p: u64) -> i32 {
    let a = a % p;
    if a == 0 {
        return 0;
    }
    if powm(a, ((p - 1) / 2) as u128, p) == 1 {
        1
    } else {
        -1
    }
}

/// Common interface so curve arithmetic runs over K4 and over L.
pub trait FieldOps {
    type El: Copy + PartialEq + std::fmt::Debug;
    fn zero(&self) -> Self::El;
    fn one(&self) -> Self::El;
    fn from_u64(&self, a: u64) -> Self::El;
    fn add(&self, a: &Self::El, b: &Self::El) -> Self::El;
    fn sub(&self, a: &Self::El, b: &Self::El) -> Self::El;
    fn neg(&self, a: &Self::El) -> Self::El;
    fn mul(&self, a: &Self::El, b: &Self::El) -> Self::El;
    fn inv(&self, a: &Self::El) -> Self::El;
    fn is_zero(&self, a: &Self::El) -> bool;
}

pub type E4 = [u64; 4];

#[derive(Clone, Debug)]
pub struct K4 {
    pub p: u64,
    /// t^4 = red[0], t^5 = red[1], t^6 = red[2] (reduced)
    pub red: [E4; 3],
    /// (t^j)^p for j = 0..3
    pub frob_t: [E4; 4],
    /// monic modulus coefficients m0..m3 (t^4 + m3 t^3 + ... + m0)
    pub m: E4,
    /// counters of K4 multiplications (cost accounting)
    pub muls: std::cell::Cell<u64>,
}

impl K4 {
    pub fn new(p: u64, m: E4) -> K4 {
        let mut k = K4 {
            p,
            red: [[0; 4]; 3],
            frob_t: [[0; 4]; 4],
            m,
            muls: std::cell::Cell::new(0),
        };
        // t^4 = -(m0 + m1 t + m2 t^2 + m3 t^3)
        let t4: E4 = [
            (p - m[0]) % p,
            (p - m[1]) % p,
            (p - m[2]) % p,
            (p - m[3]) % p,
        ];
        k.red[0] = t4;
        let t = [0, 1, 0, 0];
        k.red[1] = k.mul_raw(&t4, &t);
        k.red[2] = k.mul_raw(&k.red[1], &t);
        // frobenius table
        let tp = k.pow(&t, p as u128);
        let mut acc = k.one();
        for j in 0..4 {
            k.frob_t[j] = acc;
            acc = k.mul_raw(&acc, &tp);
        }
        k
    }
    fn mul_raw(&self, a: &E4, b: &E4) -> E4 {
        let p = self.p;
        let mut c = [0u128; 7];
        for i in 0..4 {
            if a[i] == 0 {
                continue;
            }
            for j in 0..4 {
                c[i + j] += a[i] as u128 * b[j] as u128;
            }
        }
        let mut r = [0u128; 4];
        for i in 0..4 {
            r[i] = c[i];
        }
        for h in 0..3 {
            let ch = (c[4 + h] % p as u128) as u128;
            if ch == 0 {
                continue;
            }
            for i in 0..4 {
                r[i] += ch * self.red[h][i] as u128;
            }
        }
        [
            (r[0] % p as u128) as u64,
            (r[1] % p as u128) as u64,
            (r[2] % p as u128) as u64,
            (r[3] % p as u128) as u64,
        ]
    }
    pub fn pow(&self, a: &E4, mut e: u128) -> E4 {
        let mut r = self.one();
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul_raw(&r, &b);
            }
            b = self.mul_raw(&b, &b);
            e >>= 1;
        }
        r
    }
    pub fn frob(&self, a: &E4) -> E4 {
        let p = self.p;
        let mut r = [0u64; 4];
        for j in 0..4 {
            if a[j] == 0 {
                continue;
            }
            for i in 0..4 {
                r[i] = addm(r[i], mulm(a[j], self.frob_t[j][i], p), p);
            }
        }
        r
    }
    pub fn frob_n(&self, a: &E4, n: usize) -> E4 {
        let mut r = *a;
        for _ in 0..n {
            r = self.frob(&r);
        }
        r
    }
    pub fn norm(&self, a: &E4) -> u64 {
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        let n = self.mul_raw(&self.mul_raw(a, &f1), &self.mul_raw(&f2, &f3));
        debug_assert!(n[1] == 0 && n[2] == 0 && n[3] == 0);
        n[0]
    }
    pub fn trace(&self, a: &E4) -> u64 {
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        let p = self.p;
        let s = addm(addm(a[0], f1[0], p), addm(f2[0], f3[0], p), p);
        debug_assert!((0..4).skip(1).all(|i| (a[i] + f1[i] + f2[i] + f3[i]) % p == 0));
        s
    }
    pub fn chi(&self, a: &E4) -> i32 {
        legendre(self.norm(a), self.p)
    }
    pub fn is_fp(a: &E4) -> bool {
        a[1] == 0 && a[2] == 0 && a[3] == 0
    }
    pub fn scal(&self, c: u64, a: &E4) -> E4 {
        let p = self.p;
        [mulm(c, a[0], p), mulm(c, a[1], p), mulm(c, a[2], p), mulm(c, a[3], p)]
    }
}

impl FieldOps for K4 {
    type El = E4;
    fn zero(&self) -> E4 {
        [0; 4]
    }
    fn one(&self) -> E4 {
        [1, 0, 0, 0]
    }
    fn from_u64(&self, a: u64) -> E4 {
        [a % self.p, 0, 0, 0]
    }
    fn add(&self, a: &E4, b: &E4) -> E4 {
        let p = self.p;
        [addm(a[0], b[0], p), addm(a[1], b[1], p), addm(a[2], b[2], p), addm(a[3], b[3], p)]
    }
    fn sub(&self, a: &E4, b: &E4) -> E4 {
        let p = self.p;
        [subm(a[0], b[0], p), subm(a[1], b[1], p), subm(a[2], b[2], p), subm(a[3], b[3], p)]
    }
    fn neg(&self, a: &E4) -> E4 {
        self.sub(&[0; 4], a)
    }
    fn mul(&self, a: &E4, b: &E4) -> E4 {
        self.muls.set(self.muls.get() + 1);
        self.mul_raw(a, b)
    }
    fn inv(&self, a: &E4) -> E4 {
        assert!(!self.is_zero(a), "K4 inverse of zero");
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        let c = self.mul_raw(&self.mul_raw(&f1, &f2), &f3);
        let n = self.mul_raw(a, &c)[0];
        self.muls.set(self.muls.get() + 3);
        self.scal(invm(n, self.p), &c)
    }
    fn is_zero(&self, a: &E4) -> bool {
        *a == [0; 4]
    }
}

pub type EL = (E4, E4);

/// L = K4[s]/(s^2 - nu), nu a non-square in K4.
#[derive(Clone, Debug)]
pub struct L {
    pub k: K4,
    pub nu: E4,
    /// s^p = s * nu^((p-1)/2)
    pub s_frob: E4,
    two_adic: u32,
    /// odd part of p^8 - 1 (< 2^136 for p < 2^17, so three u64 limbs, little endian;
    /// a u128 overflowed for p >= 2^16)
    odd_part: [u64; 3],
    /// (odd_part + 1) / 2
    odd_half: [u64; 3],
}

/// Little-endian 3-limb (192-bit) exponents for L, whose group order p^8 - 1 is
/// >= 2^128 for p >= 2^16.
fn big_mul_u128(a: u128, b: u128) -> [u64; 4] {
    let (a0, a1) = (a as u64 as u128, a >> 64);
    let (b0, b1) = (b as u64 as u128, b >> 64);
    let mut out = [0u64; 4];
    let mut acc = [0u128; 4];
    for (i, x) in [a0, a1].iter().enumerate() {
        for (j, y) in [b0, b1].iter().enumerate() {
            // x, y < 2^64: x y < 2^128; add the two 64-bit halves into acc
            let pr = x * y;
            acc[i + j] += pr as u64 as u128;
            acc[i + j + 1] += pr >> 64;
        }
    }
    let mut carry = 0u128;
    for i in 0..4 {
        let v = acc[i] + carry;
        out[i] = v as u64;
        carry = v >> 64;
    }
    assert_eq!(carry, 0);
    out
}
fn big_odd_part_minus_one(n: [u64; 4]) -> (u32, [u64; 3]) {
    // n >= 2; (n - 1) = 2^s t with t odd
    let mut m = n;
    let mut i = 0;
    loop {
        let (v, borrow) = m[i].overflowing_sub(1);
        m[i] = v;
        if !borrow {
            break;
        }
        i += 1;
    }
    let tz = {
        let mut z = 0u32;
        for limb in m.iter() {
            if *limb == 0 {
                z += 64;
            } else {
                z += limb.trailing_zeros();
                break;
            }
        }
        z
    };
    let t = big_shr(m, tz);
    assert_eq!(t[3], 0, "odd part must fit in 192 bits");
    (tz, [t[0], t[1], t[2]])
}
fn big_shr(m: [u64; 4], sh: u32) -> [u64; 4] {
    let (w, b) = ((sh / 64) as usize, sh % 64);
    let mut out = [0u64; 4];
    for i in 0..4 {
        let lo = if i + w < 4 { m[i + w] } else { 0 };
        let hi = if i + w + 1 < 4 { m[i + w + 1] } else { 0 };
        out[i] = if b == 0 { lo } else { (lo >> b) | (hi << (64 - b)) };
    }
    out
}

impl L {
    pub fn new(k: K4, nu: E4) -> L {
        assert_eq!(k.chi(&nu), -1, "nu must be a non-square in K4");
        let p = k.p;
        let s_frob = k.pow(&nu, ((p - 1) / 2) as u128);
        let p4 = (p as u128).pow(4);
        let (two_adic, odd_part) = big_odd_part_minus_one(big_mul_u128(p4, p4));
        // odd_part is odd: (odd_part + 1) / 2 = (odd_part >> 1) + 1
        let mut odd_half = [odd_part[0] >> 1 | odd_part[1] << 63, odd_part[1] >> 1 | odd_part[2] << 63, odd_part[2] >> 1];
        for limb in odd_half.iter_mut() {
            let (v, c) = limb.overflowing_add(1);
            *limb = v;
            if !c {
                break;
            }
        }
        L { k, nu, s_frob, two_adic, odd_part, odd_half }
    }
    pub fn emb(&self, a: &E4) -> EL {
        (*a, [0; 4])
    }
    pub fn frob(&self, a: &EL) -> EL {
        (self.k.frob(&a.0), self.k.mul_raw(&self.k.frob(&a.1), &self.s_frob))
    }
    pub fn frob_n(&self, a: &EL, n: usize) -> EL {
        let mut r = *a;
        for _ in 0..n {
            r = self.frob(&r);
        }
        r
    }
    pub fn norm_k4(&self, a: &EL) -> E4 {
        let k = &self.k;
        k.sub(&k.mul_raw(&a.0, &a.0), &k.mul_raw(&self.nu, &k.mul_raw(&a.1, &a.1)))
    }
    pub fn chi(&self, a: &EL) -> i32 {
        self.k.chi(&self.norm_k4(a))
    }
    pub fn pow(&self, a: &EL, mut e: u128) -> EL {
        let mut r = self.one();
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    /// a^e for a 192-bit exponent: the same right-to-left square-and-multiply as
    /// `pow` (same products in the same order, so the same K4 counters).
    pub fn pow_big(&self, a: &EL, e: &[u64; 3]) -> EL {
        let mut r = self.one();
        let mut b = *a;
        let mut e = *e;
        while e != [0; 3] {
            if e[0] & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e = [e[0] >> 1 | e[1] << 63, e[1] >> 1 | e[2] << 63, e[2] >> 1];
        }
        r
    }
    /// Tonelli-Shanks square root in L (every K4 element has one).
    pub fn sqrt(&self, a: &EL) -> Option<EL> {
        if self.is_zero(a) {
            return Some(self.zero());
        }
        if self.chi(a) != 1 {
            return None;
        }
        let g = self.pow_big(&([0; 4], [1, 0, 0, 0]), &self.odd_part); // s is a non-square in L
        let mut m = self.two_adic;
        let mut c = g;
        let mut t = self.pow_big(a, &self.odd_part);
        let mut r = self.pow_big(a, &self.odd_half);
        let one = self.one();
        while t != one {
            let mut i = 0;
            let mut tt = t;
            while tt != one {
                tt = self.mul(&tt, &tt);
                i += 1;
                assert!(i < m);
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(&b, &b);
            }
            m = i;
            c = self.mul(&b, &b);
            t = self.mul(&t, &c);
            r = self.mul(&r, &b);
        }
        debug_assert_eq!(self.mul(&r, &r), *a);
        Some(r)
    }
    pub fn in_k4(a: &EL) -> bool {
        a.1 == [0; 4]
    }
    pub fn in_fp(a: &EL) -> bool {
        a.1 == [0; 4] && K4::is_fp(&a.0)
    }
    /// F_p coordinates (8 of them) of an L element.
    pub fn coords(a: &EL) -> [u64; 8] {
        [a.0[0], a.0[1], a.0[2], a.0[3], a.1[0], a.1[1], a.1[2], a.1[3]]
    }
}

impl FieldOps for L {
    type El = EL;
    fn zero(&self) -> EL {
        ([0; 4], [0; 4])
    }
    fn one(&self) -> EL {
        ([1, 0, 0, 0], [0; 4])
    }
    fn from_u64(&self, a: u64) -> EL {
        ([a % self.k.p, 0, 0, 0], [0; 4])
    }
    fn add(&self, a: &EL, b: &EL) -> EL {
        (self.k.add(&a.0, &b.0), self.k.add(&a.1, &b.1))
    }
    fn sub(&self, a: &EL, b: &EL) -> EL {
        (self.k.sub(&a.0, &b.0), self.k.sub(&a.1, &b.1))
    }
    fn neg(&self, a: &EL) -> EL {
        (self.k.neg(&a.0), self.k.neg(&a.1))
    }
    fn mul(&self, a: &EL, b: &EL) -> EL {
        let k = &self.k;
        let a0b0 = k.mul(&a.0, &b.0);
        let a1b1 = k.mul(&a.1, &b.1);
        let c0 = k.add(&a0b0, &k.mul(&self.nu, &a1b1));
        let c1 = k.add(&k.mul(&a.0, &b.1), &k.mul(&a.1, &b.0));
        (c0, c1)
    }
    fn inv(&self, a: &EL) -> EL {
        let k = &self.k;
        let n = self.norm_k4(a);
        let ni = k.inv(&n);
        (k.mul(&a.0, &ni), k.neg(&k.mul(&a.1, &ni)))
    }
    fn is_zero(&self, a: &EL) -> bool {
        a.0 == [0; 4] && a.1 == [0; 4]
    }
}

/// Generic F_{p^k} for point counting over F_{p^3}, F_{p^5} (Vec-based, slow, exact).
#[derive(Clone, Debug)]
pub struct Gfk {
    pub p: u64,
    pub k: usize,
    pub m: Vec<u64>, // monic modulus, coefficients 0..k (m[k] = 1)
}

impl Gfk {
    pub fn mul(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let p = self.p;
        let k = self.k;
        let mut c = vec![0u64; 2 * k - 1];
        for i in 0..k {
            if a[i] == 0 {
                continue;
            }
            for j in 0..k {
                c[i + j] = addm(c[i + j], mulm(a[i], b[j], p), p);
            }
        }
        for d in (k..2 * k - 1).rev() {
            let cd = c[d];
            if cd == 0 {
                continue;
            }
            c[d] = 0;
            for i in 0..k {
                c[d - k + i] = subm(c[d - k + i], mulm(cd, self.m[i], p), p);
            }
        }
        c.truncate(k);
        c
    }
    pub fn pow(&self, a: &[u64], mut e: u128) -> Vec<u64> {
        let mut r = vec![0u64; self.k];
        r[0] = 1;
        let mut b = a.to_vec();
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    pub fn norm(&self, a: &[u64]) -> u64 {
        let p = self.p as u128;
        let e = (p.pow(self.k as u32) - 1) / (p - 1);
        let r = self.pow(a, e);
        debug_assert!(r[1..].iter().all(|&x| x == 0));
        r[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::Rng;

    fn random_k4(p: u64, rng: &mut Rng) -> K4 {
        loop {
            let m = vec![rng.below(p), rng.below(p), rng.below(p), rng.below(p), 1];
            if crate::poly::is_irreducible_quartic(&m, p) {
                break K4::new(p, [m[0], m[1], m[2], m[3]]);
            }
        }
    }

    /// p^8 - 1 >= 2^128 for p >= 2^16: L keeps its odd part in 192 bits. The
    /// decomposition 2^s t = p^8 - 1 is exact (checked with u128 arithmetic on
    /// p^8 - 1 = (p^4 - 1)(p^4 + 1)), pow_big equals pow below 2^128 with the same
    /// K4 product count, and Tonelli-Shanks in L returns square roots of K4
    /// elements at p = 131071 (it hit `assert!(i < m)` with the wrapped u128).
    #[test]
    fn l_big_exponent_and_sqrt_up_to_2_17() {
        let mut rng = Rng(8);
        for &p in &[101u64, 65521, 65537, 131071] {
            let k = random_k4(p, &mut rng);
            let nu = loop {
                let c = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                if k.chi(&c) == -1 {
                    break c;
                }
            };
            let l = L::new(k.clone(), nu);
            let p4 = (p as u128).pow(4);
            // 2^s t = (p^4 - 1)(p^4 + 1): compare the 2-adic valuation and t mod a few moduli
            let (a, b) = (p4 - 1, p4 + 1);
            assert_eq!(l.two_adic, a.trailing_zeros() + b.trailing_zeros(), "p={p}");
            let t_big = |m: u128| -> u128 {
                // t mod m from the limbs
                let w = (1u128 << 64) % m;
                ((l.odd_part[2] as u128 % m) * w % m * w % m + (l.odd_part[1] as u128 % m) * w % m + l.odd_part[0] as u128 % m) % m
            };
            for m in [1_000_003u128, 4_294_967_291, 998_244_353] {
                let want = ((a >> a.trailing_zeros()) % m) * ((b >> b.trailing_zeros()) % m) % m;
                assert_eq!(t_big(m), want, "p={p} m={m}");
            }
            assert_eq!(l.odd_part[0] & 1, 1);
            // odd_half = (odd_part + 1) / 2: 2 odd_half - 1 = odd_part
            let dbl = [l.odd_half[0] << 1, l.odd_half[1] << 1 | l.odd_half[0] >> 63, l.odd_half[2] << 1 | l.odd_half[1] >> 63];
            assert_eq!([dbl[0] - 1, dbl[1], dbl[2]], l.odd_part);
            // pow_big = pow for exponents below 2^128, same number of K4 products
            for _ in 0..5 {
                let x: EL = ([rng.below(p), rng.below(p), rng.below(p), rng.below(p)], [rng.below(p), rng.below(p), rng.below(p), rng.below(p)]);
                let e = (rng.next() as u128) << 64 | rng.next() as u128;
                let m0 = l.k.muls.get();
                let r1 = l.pow(&x, e);
                let m1 = l.k.muls.get();
                let r2 = l.pow_big(&x, &[e as u64, (e >> 64) as u64, 0]);
                let m2 = l.k.muls.get();
                assert_eq!(r1, r2);
                assert_eq!(m1 - m0, m2 - m1);
            }
            for _ in 0..6 {
                let x = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                let r = l.sqrt(&l.emb(&x)).expect("every K4 element is a square in L");
                assert_eq!(l.mul(&r, &r), l.emb(&x), "p={p}");
            }
        }
    }
}
