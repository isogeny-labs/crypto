//! GF(2^N), N <= 25, elements as u32 bit vectors in a polynomial basis.
//! The subfield F_q (q = 2^d, d | N) is found as the image of a primitive
//! element's power; every structural fact used by the study (subfield,
//! traces, square roots, Artin-Schreier solver) is checked in `selftest`.

#[derive(Clone)]
pub struct Gf {
    pub n: u32,
    pub poly: u32, // modulus without the x^N term is (poly & mask); full poly has bit N set
    pub mask: u32,
    trace_mask: u32,      // Tr(z) = parity(z & trace_mask)
    as_piv: Vec<(u32, u32)>, // Artin-Schreier echelon: index = leading bit, (image, preimage); image 0 = empty
}

/// Known low-weight irreducible polynomials (irreducibility is re-checked at start-up).
pub fn default_poly(n: u32) -> u32 {
    let low: u32 = match n {
        3 => 0b11,                      // x^3+x+1
        4 => 0b11,                      // x^4+x+1
        5 => 0b101,                     // x^5+x^2+1
        6 => 0b11,                      // x^6+x+1
        8 => 0b11011,                   // x^8+x^4+x^3+x+1
        9 => (1 << 4) | 1,              // x^9+x^4+1
        10 => (1 << 3) | 1,             // x^10+x^3+1
        12 => (1 << 3) | 1,             // x^12+x^3+1
        15 => 0b11,                     // x^15+x+1
        16 => (1 << 5) | (1 << 3) | (1 << 1) | 1, // x^16+x^5+x^3+x+1
        20 => (1 << 3) | 1,             // x^20+x^3+1
        25 => (1 << 3) | 1,             // x^25+x^3+1
        _ => panic!("no polynomial for N = {n}"),
    };
    (1u32 << n) | low
}

#[inline]
fn clmul(a: u32, b: u32) -> u64 {
    let mut r = 0u64;
    let mut a = a as u64;
    let mut b = b;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    r
}

fn deg64(x: u64) -> i32 {
    63 - x.leading_zeros() as i32
}

/// polynomial remainder over F_2
fn pmod(mut a: u64, m: u64) -> u64 {
    let dm = deg64(m);
    while a != 0 && deg64(a) >= dm {
        a ^= m << (deg64(a) - dm);
    }
    a
}

fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = pmod(a, b);
        a = b;
        b = r;
    }
    a
}

pub fn prime_factors(mut x: u64) -> Vec<u64> {
    let mut f = vec![];
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 {
            f.push(p);
            while x % p == 0 {
                x /= p;
            }
        }
        p += 1;
    }
    if x > 1 {
        f.push(x);
    }
    f
}

impl Gf {
    pub fn new(n: u32) -> Gf {
        let poly = default_poly(n);
        let mask = if n == 32 { u32::MAX } else { (1u32 << n) - 1 };
        let mut g = Gf { n, poly, mask, trace_mask: 0, as_piv: vec![] };
        g.check_irreducible();
        // trace mask: Tr(x^i) for each basis vector
        let mut tm = 0u32;
        for i in 0..n {
            if g.tr_slow(1 << i) == 1 {
                tm |= 1 << i;
            }
        }
        g.trace_mask = tm;
        // Artin-Schreier solver for z^2 + z = c
        let mut piv = vec![(0u32, 0u32); n as usize];
        for i in 0..n {
            let mut img = g.sq(1 << i) ^ (1 << i);
            let mut pre = 1u32 << i;
            while img != 0 {
                let lb = 31 - img.leading_zeros() as usize;
                if piv[lb].0 == 0 {
                    piv[lb] = (img, pre);
                    break;
                }
                img ^= piv[lb].0;
                pre ^= piv[lb].1;
            }
        }
        g.as_piv = piv;
        g
    }

    /// x^(2^N) = x mod f and gcd(x^(2^(N/p)) - x, f) = 1 for every prime p | N
    fn check_irreducible(&self) {
        let x = 2u32 & self.mask;
        let pw = |k: u32| {
            let mut z = x;
            for _ in 0..k {
                z = self.sq(z);
            }
            z
        };
        assert_eq!(pw(self.n), x, "modulus not irreducible (N = {})", self.n);
        for p in prime_factors(self.n as u64) {
            let z = pw(self.n / p as u32) ^ x;
            assert_eq!(pgcd(self.poly as u64, z as u64), 1, "modulus not irreducible (N = {})", self.n);
        }
    }

    #[inline]
    pub fn reduce(&self, mut r: u64) -> u32 {
        let n = self.n as i32;
        let p = self.poly as u64;
        while r >> self.n != 0 {
            let d = deg64(r);
            r ^= p << (d - n);
        }
        r as u32
    }
    #[inline]
    pub fn mul(&self, a: u32, b: u32) -> u32 {
        self.reduce(clmul(a, b))
    }
    #[inline]
    pub fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    /// extended Euclid; inv(0) panics
    pub fn inv(&self, a: u32) -> u32 {
        assert!(a != 0, "inverse of 0");
        let (mut r0, mut r1) = (self.poly as u64, a as u64);
        let (mut s0, mut s1) = (0u64, 1u64);
        while r1 != 1 {
            // r0 = q r1 + r
            let mut q = 0u64;
            let mut r = r0;
            let d1 = deg64(r1);
            while r != 0 && deg64(r) >= d1 {
                let sh = deg64(r) - d1;
                q ^= 1 << sh;
                r ^= r1 << sh;
            }
            let s = s0 ^ clmul64(q, s1);
            r0 = r1;
            r1 = r;
            s0 = s1;
            s1 = s;
            debug_assert!(r1 != 0);
        }
        pmod(s1, self.poly as u64) as u32
    }
    pub fn pow(&self, a: u32, mut e: u64) -> u32 {
        let (mut r, mut b) = (1u32, a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.sq(b);
            e >>= 1;
        }
        r
    }
    fn tr_slow(&self, z: u32) -> u32 {
        let (mut t, mut w) = (0u32, z);
        for _ in 0..self.n {
            t ^= w;
            w = self.sq(w);
        }
        assert!(t <= 1);
        t
    }
    /// absolute trace Tr_{N/1}
    #[inline]
    pub fn tr(&self, z: u32) -> u32 {
        (z & self.trace_mask).count_ones() & 1
    }
    /// z^(2^k)
    pub fn frob(&self, z: u32, k: u32) -> u32 {
        let mut w = z;
        for _ in 0..k {
            w = self.sq(w);
        }
        w
    }
    /// relative trace Tr_{N/d}(z) = sum_{i<N/d} z^(q^i)
    pub fn tr_rel(&self, z: u32, d: u32) -> u32 {
        let (mut t, mut w) = (0u32, z);
        for _ in 0..self.n / d {
            t ^= w;
            w = self.frob(w, d);
        }
        t
    }
    pub fn sqrt(&self, z: u32) -> u32 {
        self.frob(z, self.n - 1)
    }
    /// a root of z^2 + z = c, if Tr(c) = 0
    pub fn solve_as(&self, c: u32) -> Option<u32> {
        let (mut c, mut pre) = (c, 0u32);
        for lb in (0..self.n as usize).rev() {
            if (c >> lb) & 1 == 1 {
                let (img, p) = self.as_piv[lb];
                if img == 0 {
                    return None;
                }
                c ^= img;
                pre ^= p;
            }
        }
        if c == 0 { Some(pre) } else { None }
    }
    /// least j >= 1 with z^(2^j) = z
    pub fn deg(&self, z: u32) -> u32 {
        let mut w = self.sq(z);
        let mut j = 1;
        while w != z {
            w = self.sq(w);
            j += 1;
        }
        j
    }
    /// a primitive element of F_{2^N}^*
    pub fn primitive(&self) -> u32 {
        let order = (1u64 << self.n) - 1;
        let fs = prime_factors(order);
        'g: for g in 2..=self.mask {
            for &p in &fs {
                if self.pow(g, order / p) == 1 {
                    continue 'g;
                }
            }
            return g;
        }
        unreachable!()
    }
    /// all elements of the subfield F_{2^d} (0 first)
    pub fn subfield(&self, d: u32) -> Vec<u32> {
        assert_eq!(self.n % d, 0);
        let g = self.primitive();
        let gam = self.pow(g, ((1u64 << self.n) - 1) / ((1u64 << d) - 1));
        let mut v = vec![0u32];
        let mut z = 1u32;
        for _ in 0..(1u64 << d) - 1 {
            v.push(z);
            z = self.mul(z, gam);
        }
        assert_eq!(z, 1);
        v
    }
}

fn clmul64(a: u64, b: u64) -> u64 {
    let mut r = 0u64;
    let mut b = b;
    let mut a = a;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    r
}
