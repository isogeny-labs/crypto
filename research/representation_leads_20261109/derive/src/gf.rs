//! GF(2^n) for prime n <= 31 in a polynomial basis.

#[derive(Clone, Copy, Debug)]
pub struct Gf {
    pub n: u32,
    /// Exponents e < n with t^n = sum t^e.
    pub low: &'static [u32],
}

pub fn field(n: u32) -> Gf {
    let low: &'static [u32] = match n {
        11 => &[2, 0],
        13 => &[4, 3, 1, 0],
        17 => &[3, 0],
        19 => &[5, 2, 1, 0],
        23 => &[5, 0],
        29 => &[2, 0],
        31 => &[3, 0],
        _ => panic!("unsupported n = {n}"),
    };
    Gf { n, low }
}

impl Gf {
    pub fn mask(&self) -> u64 {
        (1u64 << self.n) - 1
    }
    pub fn modulus(&self) -> u64 {
        self.low.iter().fold(1u64 << self.n, |m, &e| m | (1 << e))
    }
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        let mut r: u128 = 0;
        for i in 0..self.n {
            if b >> i & 1 == 1 {
                r ^= (a as u128) << i;
            }
        }
        let m = self.modulus() as u128;
        for k in (self.n..2 * self.n).rev() {
            if r >> k & 1 == 1 {
                r ^= m << (k - self.n);
            }
        }
        r as u64
    }
    pub fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    pub fn pow(&self, mut a: u64, mut e: u128) -> u64 {
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.sq(a);
            e >>= 1;
        }
        r
    }
    pub fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        self.pow(a, (1u128 << self.n) - 2)
    }
    pub fn trace(&self, a: u64) -> u64 {
        let (mut t, mut s) = (0, a);
        for _ in 0..self.n {
            t ^= s;
            s = self.sq(s);
        }
        t
    }
    /// Solution z of z^2 + z = c when Tr(c) = 0 (n odd: the half-trace).
    pub fn half_trace(&self, c: u64) -> u64 {
        let (mut h, mut s) = (0, c);
        for i in 0..self.n {
            if i % 2 == 0 {
                h ^= s;
            }
            s = self.sq(s);
        }
        h
    }
    /// Rabin test for prime n: t^(2^n) = t mod f and f has no root in GF(2).
    pub fn is_irreducible(&self) -> bool {
        let mut s = 2u64; // t
        for _ in 0..self.n {
            s = self.sq(s);
        }
        s == 2 && self.low.contains(&0)
    }
}

/// SplitMix64: deterministic, seed-derived draws.
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn elem(&mut self, f: &Gf) -> u64 {
        self.next() & f.mask()
    }
    pub fn nonzero(&mut self, f: &Gf) -> u64 {
        loop {
            let v = self.elem(f);
            if v != 0 {
                return v;
            }
        }
    }
}
