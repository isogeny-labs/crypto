//! Existence threshold: for a random x3 = x(R), the smallest l such that
//! S3(x1, x2, x3) = 0 with x1, x2 valid x-coordinates of deg < l (so ±P1 ± P2 = ±R).
//! u64 field, n <= 63, f = t^n + g with deg g <= n/2 (two-fold sparse reduction).
use crate::gf2x::{clmul, Rng};

pub struct F { pub n: u32, pub g: u64, pub mask: u64, pub trmask: u64, ht: Vec<[u64; 256]> }

impl F {
    pub fn new(n: usize) -> F {
        let fp = crate::gf2x::find_modulus(n);
        let full = fp.0[0];
        let g = full & !(1u64 << n);
        assert!(64 - g.leading_zeros() as usize <= n / 2 + 1, "modulus not sparse enough");
        let mut f = F { n: n as u32, g, mask: (1u64 << n) - 1, trmask: 0, ht: vec![] };
        for i in 0..n {
            let x = 1u64 << i;
            let mut acc = x; let mut y = x;
            for _ in 1..n { y = f.sqr(y); acc ^= y; }
            assert!(acc <= 1);
            if acc == 1 { f.trmask |= x; }
        }
        // half-trace tables (linear map)
        let nb = (n + 7) / 8;
        let mut ht = vec![[0u64; 256]; nb];
        let basis: Vec<u64> = (0..n).map(|i| f.htr_slow(1u64 << i)).collect();
        for byte in 0..nb { for v in 0..256usize { let mut acc = 0; for b in 0..8 { let i = byte * 8 + b; if i < n && (v >> b) & 1 == 1 { acc ^= basis[i]; } } ht[byte][v] = acc; } }
        f.ht = ht;
        f
    }
    #[inline] pub fn red(&self, lo: u64, hi: u64) -> u64 {
        let n = self.n;
        let x = (lo as u128) | ((hi as u128) << 64);
        let h = (x >> n) as u64;
        let (l2, h2) = clmul(h, self.g);
        let x = (x & self.mask as u128) ^ ((l2 as u128) | ((h2 as u128) << 64));
        let h = (x >> n) as u64;
        let (l3, _) = clmul(h, self.g);
        ((x as u64) & self.mask) ^ l3
    }
    #[inline] pub fn mul(&self, a: u64, b: u64) -> u64 { let (l, h) = clmul(a, b); self.red(l, h) }
    #[inline] pub fn sqr(&self, a: u64) -> u64 { self.mul(a, a) }
    #[inline] pub fn tr(&self, a: u64) -> u32 { (a & self.trmask).count_ones() & 1 }
    #[inline] pub fn inv(&self, a: u64) -> u64 {
        let (mut u, mut v) = (a, (1u64 << self.n) | self.g);
        let (mut g1, mut g2) = (1u64, 0u64);
        while u != 1 {
            let mut j = (63 - u.leading_zeros() as i32) - (63 - v.leading_zeros() as i32);
            if j < 0 { std::mem::swap(&mut u, &mut v); std::mem::swap(&mut g1, &mut g2); j = -j; }
            u ^= v << j; g1 ^= g2 << j;
        }
        g1
    }
    fn htr_slow(&self, a: u64) -> u64 { let mut acc = a; let mut h = a; for _ in 0..(self.n - 1) / 2 { h = self.sqr(self.sqr(h)); acc ^= h; } acc }
    #[inline] pub fn htr(&self, a: u64) -> u64 { let mut acc = 0; let mut x = a; let mut i = 0; while x != 0 { acc ^= self.ht[i][(x & 0xff) as usize]; x >>= 8; i += 1; } acc }
    #[inline] pub fn is_x(&self, x: u64) -> bool { x != 0 && self.tr(x) == self.tr(self.inv(x)) }
    pub fn s3(&self, x1: u64, x2: u64, x3: u64) -> u64 {
        let a = self.mul(x1, x2) ^ self.mul(x1, x3) ^ self.mul(x2, x3);
        self.sqr(a) ^ self.mul(self.mul(x1, x2), x3) ^ 1
    }
}

pub struct ExistOut { pub x3: u64, pub l_min: u32, pub hist: Vec<u64>, pub x1_evals: u64, pub inversions: u64 }

#[inline] fn bitlen(x: u64) -> u32 { 64 - x.leading_zeros() }

/// enumerate x1 ascending; hist[l] = ordered solutions (x1,x2) with max(bitlen) == l, for l <= lmax.
pub fn search(f: &F, x3: u64, lmax: u32) -> ExistOut {
    let i3 = f.inv(x3);
    let mut best = f.n + 1;
    let mut hist = vec![0u64; lmax as usize + 1];
    let mut evals = 0u64; let mut invs = 1u64;
    let mut x1: u64 = 1;
    loop {
        let b1 = bitlen(x1);
        if b1 > f.n || (b1 > lmax && b1 >= best) { break; }
        evals += 1;
        if x1 != x3 {
            let i1 = f.inv(x1); invs += 1;
            if f.tr(x1) == f.tr(i1) {
                let s = x1 ^ x3;
                let w = f.mul(i1, i3);
                let v = f.mul(s, w ^ 1);
                if f.tr(v) == 0 {
                    let u = f.sqr(v);
                    let z = f.htr(u);
                    let is = f.inv(s); invs += 1;
                    let ba = f.mul(f.mul(x1, x3), f.sqr(is));
                    let x2a = f.mul(ba, z);
                    for x2 in [x2a, x2a ^ ba] {
                        if x2 == 0 { continue; }
                        let l = b1.max(bitlen(x2));
                        if l <= lmax || l < best {
                            invs += 1;
                            if f.is_x(x2) {
                                debug_assert_eq!(f.s3(x1, x2, x3), 0);
                                if l <= lmax { hist[l as usize] += 1; }
                                if l < best { best = l; }
                            }
                        }
                    }
                }
            }
        }
        x1 += 1;
    }
    ExistOut { x3, l_min: best, hist, x1_evals: evals, inversions: invs }
}

pub fn rand_x(f: &F, rng: &mut Rng) -> u64 { loop { let x = rng.next() & f.mask; if f.is_x(x) { return x; } } }
