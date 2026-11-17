// GF(2^k), k <= 31, polynomial basis; carry-less multiply via pclmulqdq when enabled at compile time.
use crate::util::Cnt;

#[cfg(all(target_arch = "x86_64", target_feature = "pclmulqdq"))]
#[inline(always)]
pub fn clmul(a: u64, b: u64) -> u64 {
    unsafe {
        use core::arch::x86_64::*;
        let r = _mm_clmulepi64_si128(_mm_set_epi64x(0, a as i64), _mm_set_epi64x(0, b as i64), 0);
        _mm_cvtsi128_si64(r) as u64
    }
}
#[cfg(not(all(target_arch = "x86_64", target_feature = "pclmulqdq")))]
#[inline(always)]
pub fn clmul(a: u64, b: u64) -> u64 {
    let mut r = 0u64;
    let mut b = b;
    let mut s = 0;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a << s;
        }
        b >>= 1;
        s += 1;
    }
    r
}

#[derive(Clone)]
pub struct Gf2 {
    pub k: u32,
    pub fl: u64,
    pub mask: u64,
    pub trmask: u64,
    pub q: u64,
    pub sqt: Vec<u32>, // 4 x 256 byte-chunk squaring tables
    pub mk: u64,       // multiplications in one inversion
}
impl Gf2 {
    pub fn new(k: u32, poly: u64) -> Gf2 {
        assert!(poly >> k == 1);
        let e = k - 1;
        let mk = (31 - e.leading_zeros()) as u64 + e.count_ones() as u64 - 1;
        let mut g = Gf2 { k, fl: poly ^ (1 << k), mask: (1u64 << k) - 1, trmask: 0, q: 1u64 << k, sqt: vec![0; 1024], mk };
        for ch in 0..4usize {
            for b in 0..256u64 {
                let v = (b << (8 * ch)) & g.mask;
                let sq = g.red(clmul(v, v));
                g.sqt[ch * 256 + b as usize] = sq as u32;
            }
        }
        let mut tm = 0u64;
        let mut d = Cnt::default();
        for i in 0..k {
            let mut t = 1u64 << i;
            let mut s = 0u64;
            for _ in 0..k {
                s ^= t;
                t = g.sqr(&mut d, t);
            }
            assert!(s <= 1, "trace not in F2");
            if s == 1 {
                tm |= 1 << i;
            }
        }
        g.trmask = tm;
        g
    }
    #[inline(always)]
    fn red(&self, p: u64) -> u64 {
        let mut lo = p & self.mask;
        let mut hi = p >> self.k;
        while hi != 0 {
            let t = clmul(hi, self.fl);
            lo ^= t & self.mask;
            hi = t >> self.k;
        }
        lo
    }
    #[inline(always)]
    pub fn mul(&self, c: &mut Cnt, a: u64, b: u64) -> u64 {
        c.m += 1;
        self.red(clmul(a, b))
    }
    #[inline(always)]
    pub fn sqr(&self, c: &mut Cnt, a: u64) -> u64 {
        c.s += 1;
        self.sqrr(a)
    }
    #[inline(always)]
    pub fn sqrr(&self, a: u64) -> u64 {
        (self.sqt[(a & 255) as usize]
            ^ self.sqt[256 + ((a >> 8) & 255) as usize]
            ^ self.sqt[512 + ((a >> 16) & 255) as usize]
            ^ self.sqt[768 + ((a >> 24) & 255) as usize]) as u64
    }
    #[inline(always)]
    pub fn mulr(&self, a: u64, b: u64) -> u64 {
        self.red(clmul(a, b))
    }
    #[inline(always)]
    pub fn tr(&self, a: u64) -> u64 {
        ((a & self.trmask).count_ones() & 1) as u64
    }
    // Itoh-Tsujii: a^-1 = (a^(2^(k-1)-1))^2 ; counts (k-1) S + (floor(log2(k-1)) + popcount(k-1) - 1) M
    #[inline]
    pub fn inv(&self, c: &mut Cnt, a: u64) -> u64 {
        c.s += (self.k - 1) as u64;
        c.m += self.mk;
        self.inv_raw(a)
    }
    pub fn inv_raw(&self, a: u64) -> u64 {
        debug_assert!(a != 0);
        let e = self.k - 1;
        let nb = 32 - e.leading_zeros();
        let mut cur = 1u32;
        let mut beta = a;
        for i in (0..nb - 1).rev() {
            let mut t = beta;
            for _ in 0..cur {
                t = self.sqrr(t);
            }
            beta = self.mulr(t, beta);
            cur *= 2;
            if (e >> i) & 1 == 1 {
                let t = self.sqrr(beta);
                beta = self.mulr(t, a);
                cur += 1;
            }
        }
        self.sqrr(beta)
    }
    // half-trace for odd k: z with z^2+z = x when Tr(x)=0; (k-1) squarings
    pub fn ht(&self, c: &mut Cnt, x: u64) -> u64 {
        c.s += (self.k - 1) as u64;
        let mut s = x;
        let mut t = x;
        for _ in 0..(self.k - 1) / 2 {
            t = self.sqrr(self.sqrr(t));
            s ^= t;
        }
        s
    }
    pub fn powg(&self, c: &mut Cnt, a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        let mut b = a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(c, r, b);
            }
            b = self.sqr(c, b);
            e >>= 1;
        }
        r
    }
    pub fn sqrt(&self, a: u64) -> u64 {
        let mut d = Cnt::default();
        let mut t = a;
        for _ in 0..self.k - 1 {
            t = self.sqr(&mut d, t);
        }
        t
    }
}
