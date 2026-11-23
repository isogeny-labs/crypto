// Models: binary curve E_{1,b} over GF(2^k) and short-Weierstrass prime curve; factor bases; Semaev quadratics.
use crate::gf2::Gf2;
use crate::util::*;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pt {
    pub x: u64,
    pub y: u64,
    pub inf: bool,
}
pub const INF: Pt = Pt { x: 0, y: 0, inf: true };

#[derive(Clone, Copy, Debug)]
pub struct Quad {
    pub pass: bool,
    pub nr: u8,
    pub r: [u64; 2],
    pub inf: bool, // the second root is infinity (leading coefficient 0)
}
impl Quad {
    pub fn fail() -> Quad {
        Quad { pass: false, nr: 0, r: [0, 0], inf: false }
    }
}

pub trait Model: Sync {
    fn kbits(&self) -> u32;
    fn lbits(&self) -> u32;
    fn star(&self) -> &[u64];
    fn is_e(&self, x: u64) -> bool; // independent classification (uncounted); x != 0
    fn pre_e(&self, c: &mut Cnt, x: u64) -> bool; // counted precompute of the E-type bit
    fn in_v_free(&self, x: u64) -> bool;
    fn in_v(&self, c: &mut Cnt, x: u64) -> bool; // counted membership in V (x != 0 assumed)
    fn quad(&self, c: &mut Cnt, u: u64, v: u64) -> Quad;
    fn lift(&self, c: &mut Cnt, x: u64) -> Option<Pt>;
    fn add(&self, c: &mut Cnt, p: &Pt, q: &Pt) -> Pt;
    fn neg(&self, p: &Pt) -> Pt;
    fn sb_over(&self, x: u64) -> Option<u64>;
    fn order(&self) -> u64;
    fn modulus(&self) -> u64; // q or p
}

// ---------------- factor base (F2-subspace) ----------------
#[derive(Clone)]
pub struct Vset {
    pub k: u32,
    pub l: u32,
    pub rows: Vec<u64>,
    pub star: Vec<u64>,
    pub syn: Vec<u32>, // 4 x 256 byte-chunk syndrome tables (implementation speed-up; counted cost is k-l parity tests)
}
fn syn_tables(rows: &[u64]) -> Vec<u32> {
    let mut t = vec![0u32; 1024];
    for ch in 0..4usize {
        for b in 0..256u64 {
            let x = b << (8 * ch);
            let mut s = 0u32;
            for (r, &row) in rows.iter().enumerate() {
                if (row & x).count_ones() & 1 == 1 {
                    s |= 1 << r;
                }
            }
            t[ch * 256 + b as usize] = s;
        }
    }
    t
}
impl Vset {
    pub fn poly(k: u32, l: u32) -> Vset {
        let rows: Vec<u64> = (l..k).map(|j| 1u64 << j).collect();
        let syn = syn_tables(&rows);
        Vset { k, l, rows, star: (1..(1u64 << l)).collect(), syn }
    }
    pub fn rand(k: u32, l: u32, rng: &mut Rng, extra: Option<u64>) -> Vset {
        let q = 1u64 << k;
        let mut piv = [0u64; 64];
        let mut cnt = 0u32;
        let mut insert = |x: u64, piv: &mut [u64; 64]| -> bool {
            let mut y = x;
            for bit in (0..k).rev() {
                if (y >> bit) & 1 == 1 {
                    if piv[bit as usize] != 0 {
                        y ^= piv[bit as usize];
                    } else {
                        piv[bit as usize] = y;
                        return true;
                    }
                }
            }
            false
        };
        if let Some(e) = extra {
            if insert(e, &mut piv) {
                cnt += 1;
            }
        }
        while cnt < l {
            if insert(rng.below(q), &mut piv) {
                cnt += 1;
            }
        }
        // reduced echelon
        let pivbits: Vec<u32> = (0..k).filter(|&b| piv[b as usize] != 0).collect();
        for &p in &pivbits {
            let pv = piv[p as usize];
            for &o in &pivbits {
                if o != p && (piv[o as usize] >> p) & 1 == 1 {
                    piv[o as usize] ^= pv;
                }
            }
        }
        let mut rows = vec![];
        for j in 0..k {
            if piv[j as usize] != 0 {
                continue;
            }
            let mut m = 1u64 << j;
            for &p in &pivbits {
                if (piv[p as usize] >> j) & 1 == 1 {
                    m |= 1u64 << p;
                }
            }
            rows.push(m);
        }
        let bas: Vec<u64> = pivbits.iter().map(|&p| piv[p as usize]).collect();
        let mut star = Vec::with_capacity((1usize << l) - 1);
        let mut cur = 0u64;
        for i in 1u64..(1u64 << l) {
            cur ^= bas[i.trailing_zeros() as usize];
            star.push(cur);
        }
        star.sort_unstable();
        let syn = syn_tables(&rows);
        Vset { k, l, rows, star, syn }
    }
    #[inline]
    pub fn contains(&self, x: u64) -> bool {
        (self.syn[(x & 255) as usize]
            ^ self.syn[256 + ((x >> 8) & 255) as usize]
            ^ self.syn[512 + ((x >> 16) & 255) as usize]
            ^ self.syn[768 + ((x >> 24) & 255) as usize])
            == 0
    }
}

// ---------------- binary curve ----------------
pub struct BinModel {
    pub f: Gf2,
    pub k: u32,
    pub a: u64,
    pub b: u64,
    pub sb: u64,
    pub tra: u64,
    pub ne: u64,
    pub n: u64,
    pub v: Vset,
}
impl BinModel {
    pub fn new(c: &BinCurve, v: Vset) -> BinModel {
        let f = Gf2::new(c.k, c.poly);
        let sb = f.sqrt(c.b);
        let tra = f.tr(1);
        BinModel { f, k: c.k, a: 1, b: c.b, sb, tra, ne: c.ne, n: c.n, v }
    }
    pub fn with_v(&self, v: Vset) -> BinModel {
        BinModel { f: self.f.clone(), k: self.k, a: self.a, b: self.b, sb: self.sb, tra: self.tra, ne: self.ne, n: self.n, v }
    }
    pub fn mu_e(&self) -> f64 {
        (self.ne as f64 - 2.0) / (2.0 * ((1u64 << self.k) as f64 - 1.0))
    }
}
impl Model for BinModel {
    fn kbits(&self) -> u32 {
        self.k
    }
    fn lbits(&self) -> u32 {
        self.v.l
    }
    fn star(&self) -> &[u64] {
        &self.v.star
    }
    fn is_e(&self, x: u64) -> bool {
        let ix = self.f.inv_raw(x);
        let t = self.f.mulr(self.sb, ix);
        self.f.tr(x ^ t) == self.tra
    }
    fn pre_e(&self, c: &mut Cnt, x: u64) -> bool {
        let ix = self.f.inv(c, x);
        let t = self.f.mul(c, self.sb, ix);
        c.t += 1;
        self.f.tr(x ^ t) == self.tra
    }
    fn in_v_free(&self, x: u64) -> bool {
        self.v.contains(x)
    }
    #[inline]
    fn in_v(&self, c: &mut Cnt, x: u64) -> bool {
        c.x += (self.k - self.v.l) as u64;
        self.v.contains(x)
    }
    #[inline]
    fn quad(&self, c: &mut Cnt, u: u64, v: u64) -> Quad {
        let f = &self.f;
        if u == 0 || v == 0 {
            c.deg += 1;
            let w = u | v;
            if w == 0 {
                return Quad::fail();
            }
            let iw = f.inv(c, w);
            let r = f.mul(c, self.sb, iw);
            return Quad { pass: true, nr: 1, r: [r, 0], inf: false };
        }
        let a = f.sqr(c, u ^ v);
        let bb = f.mul(c, u, v);
        let b2 = f.sqr(c, bb);
        let cc = b2 ^ self.b;
        if a == 0 {
            c.deg += 1;
            let ib = f.inv(c, bb);
            let r = f.mul(c, cc, ib);
            return Quad { pass: true, nr: 1, r: [r, 0], inf: true };
        }
        let ib = f.inv(c, bb);
        let ib2 = f.sqr(c, ib);
        let ca = f.mul(c, cc, a);
        let q = f.mul(c, ca, ib2);
        c.t += 1;
        if f.tr(q) != 0 {
            return Quad::fail();
        }
        let z = f.ht(c, q);
        let ia = f.inv(c, a);
        let boa = f.mul(c, bb, ia);
        let r0 = f.mul(c, boa, z);
        Quad { pass: true, nr: 2, r: [r0, r0 ^ boa], inf: false }
    }
    fn lift(&self, c: &mut Cnt, x: u64) -> Option<Pt> {
        c.lift_att += 1;
        let f = &self.f;
        if x == 0 {
            return Some(Pt { x: 0, y: self.sb, inf: false });
        }
        let ix = f.inv(c, x);
        let ix2 = f.sqr(c, ix);
        let fx = x ^ self.a ^ f.mul(c, self.b, ix2);
        c.t += 1;
        if f.tr(fx) != 0 {
            c.lift_fail += 1;
            return None;
        }
        let z = f.ht(c, fx);
        let y = f.mul(c, x, z);
        Some(Pt { x, y: y.min(x ^ y), inf: false })
    }
    fn add(&self, c: &mut Cnt, p: &Pt, q: &Pt) -> Pt {
        if p.inf {
            return *q;
        }
        if q.inf {
            return *p;
        }
        c.g += 1;
        let f = &self.f;
        let mut d = Cnt::default();
        if p.x == q.x {
            if p.y ^ q.y == p.x {
                return INF;
            }
            let ix = f.inv(&mut d, p.x);
            let lam = p.x ^ f.mul(&mut d, p.y, ix);
            let x3 = f.sqr(&mut d, lam) ^ lam ^ self.a;
            let y3 = f.sqr(&mut d, p.x) ^ f.mul(&mut d, lam ^ 1, x3);
            return Pt { x: x3, y: y3, inf: false };
        }
        let ix = f.inv(&mut d, p.x ^ q.x);
        let lam = f.mul(&mut d, p.y ^ q.y, ix);
        let x3 = f.sqr(&mut d, lam) ^ lam ^ p.x ^ q.x ^ self.a;
        let y3 = f.mul(&mut d, lam, p.x ^ x3) ^ x3 ^ p.y;
        Pt { x: x3, y: y3, inf: false }
    }
    fn neg(&self, p: &Pt) -> Pt {
        if p.inf {
            *p
        } else {
            Pt { x: p.x, y: p.x ^ p.y, inf: false }
        }
    }
    fn sb_over(&self, x: u64) -> Option<u64> {
        let mut d = Cnt::default();
        let ix = self.f.inv(&mut d, x);
        Some(self.f.mul(&mut d, self.sb, ix))
    }
    fn order(&self) -> u64 {
        self.n
    }
    fn modulus(&self) -> u64 {
        1u64 << self.k
    }
}

// ---------------- prime curve y^2 = x^3 + A x + B ----------------
pub struct PrimeModel {
    pub p: u64,
    pub k: u32,
    pub l: u32,
    pub a: u64,
    pub b: u64,
    pub n: u64,
    pub star: Vec<u64>,
    pub set: HashSet<u64>,
}
impl PrimeModel {
    pub fn new(c: &PrimeCurve, l: u32, rng: &mut Rng, extra: Option<u64>) -> PrimeModel {
        let mut set: HashSet<u64> = HashSet::new();
        if let Some(e) = extra {
            set.insert(e);
        }
        let want = 1usize << l;
        while set.len() < want {
            let x = 1 + rng.below(c.p - 1);
            set.insert(x);
        }
        let mut star: Vec<u64> = set.iter().cloned().collect();
        star.sort_unstable();
        PrimeModel { p: c.p, k: c.k, l, a: c.a, b: c.b, n: c.n, star, set }
    }
    #[inline(always)]
    fn mulm(&self, c: &mut Cnt, a: u64, b: u64) -> u64 {
        c.m += 1;
        a * b % self.p
    }
    #[inline(always)]
    fn sqrm(&self, c: &mut Cnt, a: u64) -> u64 {
        c.s += 1;
        a * a % self.p
    }
    pub fn powm(&self, c: &mut Cnt, a: u64, e: u64) -> u64 {
        // left-to-right; S for every bit below the top, M for every set bit below the top
        let nb = 64 - e.leading_zeros();
        let mut r = a;
        for i in (0..nb - 1).rev() {
            r = self.sqrm(c, r);
            if (e >> i) & 1 == 1 {
                r = self.mulm(c, r, a);
            }
        }
        r
    }
    fn invm(&self, c: &mut Cnt, a: u64) -> u64 {
        self.powm(c, a, self.p - 2)
    }
    fn fx(&self, c: &mut Cnt, x: u64) -> u64 {
        let x2 = self.sqrm(c, x);
        let x3 = self.mulm(c, x2, x);
        let ax = self.mulm(c, self.a, x);
        (x3 + ax + self.b) % self.p
    }
    pub fn mu_e(&self) -> f64 {
        // E-type density among x in 1..p-1 : (n-1)/2 values over p-1 (x != 0 may be either)
        (self.n as f64 - 1.0) / 2.0 / (self.p as f64 - 1.0)
    }
}
impl Model for PrimeModel {
    fn kbits(&self) -> u32 {
        self.k
    }
    fn lbits(&self) -> u32 {
        self.l
    }
    fn star(&self) -> &[u64] {
        &self.star
    }
    fn is_e(&self, x: u64) -> bool {
        let mut d = Cnt::default();
        let f = self.fx(&mut d, x);
        // Euler criterion by plain square-and-multiply (independent of powm)
        let mut r = 1u64;
        let mut b = f;
        let mut e = (self.p - 1) / 2;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % self.p;
            }
            b = b * b % self.p;
            e >>= 1;
        }
        r == 1
    }
    fn pre_e(&self, c: &mut Cnt, x: u64) -> bool {
        let f = self.fx(c, x);
        let r = self.powm(c, f, (self.p - 1) / 2);
        r == 1
    }
    fn in_v_free(&self, x: u64) -> bool {
        self.set.contains(&x)
    }
    fn in_v(&self, c: &mut Cnt, x: u64) -> bool {
        c.l += 1;
        self.star.binary_search(&x).is_ok()
    }
    fn quad(&self, c: &mut Cnt, u: u64, v: u64) -> Quad {
        let p = self.p;
        let uv = self.mulm(c, u, v);
        let su = (u + v) % p;
        let a2 = self.sqrm(c, (u + p - v) % p);
        let t = self.mulm(c, su, (uv + self.a) % p);
        let w = (t + 2 * self.b) % p;
        let w2 = self.sqrm(c, w);
        let q1 = self.sqrm(c, (uv + p - self.a) % p);
        let q2 = self.mulm(c, (4 * self.b) % p, su);
        let a0 = (q1 + p - q2) % p;
        if a2 == 0 {
            c.deg += 1;
            let tw = (2 * w) % p;
            if tw == 0 {
                return Quad { pass: true, nr: 0, r: [0, 0], inf: true };
            }
            let it = self.invm(c, tw);
            let r = self.mulm(c, a0, it);
            return Quad { pass: true, nr: 1, r: [r, 0], inf: true };
        }
        let aa0 = self.mulm(c, a2, a0);
        let dd = (w2 + p - aa0) % p;
        let s = if dd == 0 { 0 } else { self.powm(c, dd, (p + 1) / 4) };
        let s2 = self.sqrm(c, s);
        if s2 != dd {
            return Quad::fail();
        }
        let ia = self.invm(c, a2);
        let r0 = self.mulm(c, (w + s) % p, ia);
        if s == 0 {
            return Quad { pass: true, nr: 1, r: [r0, 0], inf: false };
        }
        let r1 = self.mulm(c, (w + p - s) % p, ia);
        Quad { pass: true, nr: 2, r: [r0, r1], inf: false }
    }
    fn lift(&self, c: &mut Cnt, x: u64) -> Option<Pt> {
        c.lift_att += 1;
        let f = self.fx(c, x);
        let y = self.powm(c, f.max(1), (self.p + 1) / 4);
        let y = if f == 0 { 0 } else { y };
        if self.sqrm(c, y) != f {
            c.lift_fail += 1;
            return None;
        }
        Some(Pt { x, y: y.min(self.p - y), inf: false })
    }
    fn add(&self, c: &mut Cnt, pp: &Pt, q: &Pt) -> Pt {
        if pp.inf {
            return *q;
        }
        if q.inf {
            return *pp;
        }
        c.g += 1;
        let p = self.p;
        let mut d = Cnt::default();
        let lam;
        if pp.x == q.x {
            if (pp.y + q.y) % p == 0 {
                return INF;
            }
            let num = (3 * (pp.x * pp.x % p) + self.a) % p;
            let il = self.invm(&mut d, 2 * pp.y % p);
            lam = num * il % p;
        } else {
            let il = self.invm(&mut d, (q.x + p - pp.x) % p);
            lam = (q.y + p - pp.y) % p * il % p;
        }
        let x3 = (lam * lam % p + 2 * p - pp.x - q.x) % p;
        let y3 = (lam * ((pp.x + p - x3) % p) % p + p - pp.y) % p;
        Pt { x: x3, y: y3, inf: false }
    }
    fn neg(&self, pp: &Pt) -> Pt {
        if pp.inf {
            *pp
        } else {
            Pt { x: pp.x, y: (self.p - pp.y) % self.p, inf: false }
        }
    }
    fn sb_over(&self, _x: u64) -> Option<u64> {
        None
    }
    fn order(&self) -> u64 {
        self.n
    }
    fn modulus(&self) -> u64 {
        self.p
    }
}

pub fn smul<M: Model>(md: &M, c: &mut Cnt, mut k: u64, p: &Pt) -> Pt {
    let mut r = INF;
    let mut b = *p;
    // right-to-left double and add (counted by md.add)
    while k > 0 {
        if k & 1 == 1 {
            r = md.add(c, &r, &b);
        }
        k >>= 1;
        if k > 0 {
            b = md.add(c, &b, &b);
        }
    }
    r
}
