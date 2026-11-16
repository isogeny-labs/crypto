//! F_2[t] polynomials as little-endian u64 words, plus small fields F_{2^D} (D <= 63).

#[inline]
pub fn clmul(a: u64, b: u64) -> (u64, u64) {
    unsafe {
        use std::arch::x86_64::*;
        let r = _mm_clmulepi64_si128(_mm_set_epi64x(0, a as i64), _mm_set_epi64x(0, b as i64), 0);
        (_mm_extract_epi64(r, 0) as u64, _mm_extract_epi64(r, 1) as u64)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct P(pub Vec<u64>);

impl P {
    pub fn zero() -> P { P(vec![]) }
    pub fn one() -> P { P(vec![1]) }
    pub fn from_u64(x: u64) -> P { let mut p = P(vec![x]); p.norm(); p }
    pub fn mono(i: usize) -> P { P::one().shl(i) }
    pub fn norm(&mut self) { while self.0.last() == Some(&0) { self.0.pop(); } }
    pub fn is_zero(&self) -> bool { self.0.is_empty() }
    pub fn is_one(&self) -> bool { self.0.len() == 1 && self.0[0] == 1 }
    pub fn deg(&self) -> i64 {
        match self.0.last() {
            None => -1,
            Some(&w) => ((self.0.len() - 1) * 64 + 63 - w.leading_zeros() as usize) as i64,
        }
    }
    pub fn bit(&self, i: usize) -> bool { let w = i / 64; w < self.0.len() && (self.0[w] >> (i % 64)) & 1 == 1 }
    pub fn to_u64(&self) -> u64 { assert!(self.0.len() <= 1); if self.0.is_empty() { 0 } else { self.0[0] } }
    /// self ^= o << s ; returns number of words touched
    pub fn xor_shl(&mut self, o: &P, s: usize) -> u64 {
        if o.is_zero() { return 0; }
        let ws = s / 64;
        let bs = s % 64;
        let need = o.0.len() + ws + 1;
        if self.0.len() < need { self.0.resize(need, 0); }
        if bs == 0 {
            for (i, &w) in o.0.iter().enumerate() { self.0[i + ws] ^= w; }
        } else {
            for (i, &w) in o.0.iter().enumerate() {
                self.0[i + ws] ^= w << bs;
                self.0[i + ws + 1] ^= w >> (64 - bs);
            }
        }
        self.norm();
        o.0.len() as u64
    }
    pub fn add(&self, o: &P) -> P { let mut r = self.clone(); r.xor_shl(o, 0); r }
    pub fn shl(&self, s: usize) -> P { let mut r = P::zero(); r.xor_shl(self, s); r }
    pub fn shr(&self, s: usize) -> P {
        let d = self.deg();
        if d < s as i64 { return P::zero(); }
        let mut r = P::zero();
        for i in s..=(d as usize) { if self.bit(i) { r.set_bit(i - s); } }
        r
    }
    pub fn low_zero(&self, s: usize) -> bool { (0..s).all(|i| !self.bit(i)) }
    pub fn set_bit(&mut self, i: usize) {
        let w = i / 64;
        if self.0.len() <= w { self.0.resize(w + 1, 0); }
        self.0[w] |= 1u64 << (i % 64);
    }
    pub fn mul(&self, o: &P) -> P {
        if self.is_zero() || o.is_zero() { return P::zero(); }
        let mut r = vec![0u64; self.0.len() + o.0.len()];
        for (i, &a) in self.0.iter().enumerate() {
            if a == 0 { continue; }
            for (j, &b) in o.0.iter().enumerate() {
                let (lo, hi) = clmul(a, b);
                r[i + j] ^= lo;
                r[i + j + 1] ^= hi;
            }
        }
        let mut p = P(r);
        p.norm();
        p
    }
    pub fn pow(&self, e: usize) -> P { let mut r = P::one(); for _ in 0..e { r = r.mul(self); } r }
    pub fn rem(&self, m: &P) -> P {
        let dm = m.deg();
        assert!(dm >= 0);
        let mut r = self.clone();
        loop {
            let d = r.deg();
            if d < dm { break; }
            r.xor_shl(m, (d - dm) as usize);
        }
        r
    }
    pub fn random(rng: &mut Rng, bits: usize) -> P {
        let mut p = P(vec![0; (bits + 63) / 64]);
        for i in 0..bits { if rng.next() & 1 == 1 { p.set_bit(i); } }
        p.norm();
        p
    }
    pub fn hex(&self) -> String {
        if self.is_zero() { return "0".into(); }
        let mut s = String::new();
        for (i, w) in self.0.iter().rev().enumerate() {
            if i == 0 { s += &format!("{:x}", w); } else { s += &format!("{:016x}", w); }
        }
        s
    }
}

/// splitmix64
pub struct Rng(pub u64);
impl Rng {
    pub fn new(seed: u64) -> Rng { Rng(seed ^ 0x9E3779B97F4A7C15) }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

pub fn derive_seed(parts: &[u64]) -> u64 {
    let mut r = Rng::new(0x5EED_0005);
    let mut s = 0u64;
    for &p in parts { r.0 ^= p.wrapping_mul(0x100000001B3); s ^= r.next(); }
    s
}

/// Irreducibility of a degree-d polynomial for PRIME d: q | t^{2^d} - t and no linear factor.
pub fn irreducible_prime_degree(f: &P) -> bool {
    let d = f.deg();
    if d < 2 { return d == 1; }
    // no root at 0 or 1
    if !f.bit(0) { return false; }
    let w: u32 = f.0.iter().map(|x| x.count_ones()).sum();
    if w % 2 == 0 { return false; }
    let mut x = P::mono(1);
    for _ in 0..d { x = x.mul(&x).rem(f); }
    x == P::mono(1)
}

/// lowest-weight irreducible: trinomial t^n+t^k+1 (k<=n/2) else pentanomial with small middle terms.
pub fn find_modulus(n: usize) -> P {
    if n == 131 {
        let mut f = P::mono(131);
        for i in [13usize, 2, 1, 0] { f.set_bit(i); }
        assert!(irreducible_prime_degree(&f));
        return f;
    }
    for k in 1..=n / 2 {
        let mut f = P::mono(n);
        f.set_bit(k);
        f.set_bit(0);
        if irreducible_prime_degree(&f) { return f; }
    }
    for a in 3..n / 2 { for b in 2..a { for c in 1..b {
        let mut f = P::mono(n);
        for i in [a, b, c, 0] { f.set_bit(i); }
        if irreducible_prime_degree(&f) { return f; }
    }}}
    panic!("no modulus")
}

/// Small field F_{2^D}, D<=63, q includes bit D.
#[derive(Clone, Debug)]
pub struct F64 { pub d: u32, pub q: u64 }
impl F64 {
    pub fn new(d: u32) -> F64 {
        assert!(d <= 63 && d >= 2);
        let qp = find_modulus(d as usize);
        F64 { d, q: qp.to_u64() }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        let (lo, hi) = clmul(a, b);
        let mut x = (lo as u128) | ((hi as u128) << 64);
        let d = self.d;
        let mut i = 127 - x.leading_zeros() as i64;
        while i >= d as i64 {
            x ^= (self.q as u128) << (i as u32 - d);
            if x == 0 { break; }
            i = 127 - x.leading_zeros() as i64;
        }
        x as u64
    }
    pub fn pow(&self, a: u64, mut e: u128) -> u64 {
        let mut r = 1u64; let mut b = a;
        while e > 0 { if e & 1 == 1 { r = self.mul(r, b); } b = self.mul(b, b); e >>= 1; }
        r
    }
    pub fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        let (mut u, mut v) = (a, self.q);
        let (mut g1, mut g2) = (1u64, 0u64);
        while u != 1 {
            let mut j = (63 - u.leading_zeros() as i32) - (63 - v.leading_zeros() as i32);
            if j < 0 { std::mem::swap(&mut u, &mut v); std::mem::swap(&mut g1, &mut g2); j = -j; }
            u ^= v << j; g1 ^= g2 << j;
        }
        g1
    }
}
