//! Toy finite fields: F_p (p < 2^15), K4 = F_p[t]/(m(t)) with m the
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
    odd_part: u128,
}

impl L {
    pub fn new(k: K4, nu: E4) -> L {
        assert_eq!(k.chi(&nu), -1, "nu must be a non-square in K4");
        let p = k.p;
        let s_frob = k.pow(&nu, ((p - 1) / 2) as u128);
        let order: u128 = (p as u128).pow(8) - 1;
        let two_adic = order.trailing_zeros();
        let odd_part = order >> two_adic;
        L { k, nu, s_frob, two_adic, odd_part }
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
    /// Tonelli-Shanks square root in L (every K4 element has one).
    pub fn sqrt(&self, a: &EL) -> Option<EL> {
        if self.is_zero(a) {
            return Some(self.zero());
        }
        if self.chi(a) != 1 {
            return None;
        }
        let g = self.pow(&([0; 4], [1, 0, 0, 0]), self.odd_part); // s is a non-square in L
        let mut m = self.two_adic;
        let mut c = g;
        let mut t = self.pow(a, self.odd_part);
        let mut r = self.pow(a, (self.odd_part + 1) / 2);
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
