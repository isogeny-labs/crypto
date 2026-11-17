// Standalone prototype: quartic (Ferrari) radical root finder over F_p with a clean
// modular cube root (F_p and F_{p^2} Adleman-Manders-Miller, no brute fallback),
// validated vs brute force / Cantor-Zassenhaus, field multiplications counted.
// Usage: radical_quartic cube | quartic | measure
// Requires p >= 5 (divisions by 2,3). Public synthetic data only.
use std::cell::Cell;

thread_local! { static MULS: Cell<u64> = Cell::new(0); }
fn reset() { MULS.with(|m| m.set(0)); }
fn muls() -> u64 { MULS.with(|m| m.get()) }
fn bump() { MULS.with(|m| m.set(m.get() + 1)); }

#[derive(Clone, Copy)]
pub struct Fp { pub p: u64 }
impl Fp {
    pub fn new(p: u64) -> Self { Fp { p } }
    #[inline] pub fn add(&self, a: u64, b: u64) -> u64 { ((a as u128 + b as u128) % self.p as u128) as u64 }
    #[inline] pub fn sub(&self, a: u64, b: u64) -> u64 { ((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64 }
    #[inline] pub fn neg(&self, a: u64) -> u64 { if a == 0 { 0 } else { self.p - a } }
    #[inline] pub fn mul(&self, a: u64, b: u64) -> u64 { bump(); ((a as u128 * b as u128) % self.p as u128) as u64 }
    #[inline] pub fn sqr(&self, a: u64) -> u64 { self.mul(a, a) }
    pub fn powm(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64 % self.p; a %= self.p;
        while e > 0 { if e & 1 == 1 { r = self.mul(r, a); } a = self.sqr(a); e >>= 1; }
        r
    }
    pub fn inv(&self, a: u64) -> u64 { self.powm(a, self.p - 2) }
}

// ---------- generic finite-field ring for the AMM cube root ----------
pub trait Ring {
    type E: Copy + PartialEq + std::fmt::Debug;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E;
    fn order(&self) -> u128;            // |K^*|
    fn cand(&self, k: u64) -> Self::E;  // k-th nonzero candidate (for non-cube search)
}
impl Ring for Fp {
    type E = u64;
    fn zero(&self) -> u64 { 0 }
    fn one(&self) -> u64 { 1 }
    fn mul(&self, a: u64, b: u64) -> u64 { Fp::mul(self, a, b) }
    fn order(&self) -> u128 { (self.p - 1) as u128 }
    fn cand(&self, k: u64) -> u64 { (k + 2) % self.p }
}
// F_{p^2} = F_p[i]/(i^2 - n), n a fixed quadratic non-residue.
#[derive(Clone, Copy)]
pub struct Fq2 { pub f: Fp, pub n: u64 }
impl Ring for Fq2 {
    type E = (u64, u64);
    fn zero(&self) -> (u64, u64) { (0, 0) }
    fn one(&self) -> (u64, u64) { (1, 0) }
    fn mul(&self, x: (u64, u64), y: (u64, u64)) -> (u64, u64) {
        let f = &self.f;
        let ac = f.mul(x.0, y.0);
        let bd = f.mul(x.1, y.1);
        let nbd = f.mul(self.n, bd);
        let s = f.mul(f.add(x.0, x.1), f.add(y.0, y.1));
        (f.add(ac, nbd), f.sub(f.sub(s, ac), bd))
    }
    fn order(&self) -> u128 { (self.f.p as u128) * (self.f.p as u128) - 1 }
    fn cand(&self, k: u64) -> (u64, u64) { (k % self.f.p, 1) }
}
pub fn rpow<R: Ring>(r: &R, a: R::E, mut e: u128) -> R::E {
    let mut acc = r.one(); let mut b = a;
    while e > 0 { if e & 1 == 1 { acc = r.mul(acc, b); } e >>= 1; if e > 0 { b = r.mul(b, b); } }
    acc
}

/// Adleman-Manders-Miller cube root in a finite field K with group order N = |K^*|.
/// Per-field precomputation (non-cube, Sylow generator) is done once in `new`.
pub struct Cube<R: Ring> {
    pub r: R, n: u128, s: u32, t: u128, e3: u128, c: R::E, pub w: R::E, p3s: u128,
}
impl<R: Ring> Cube<R> {
    pub fn new(r: R) -> Self {
        let n = r.order();
        let (mut t, mut s) = (n, 0u32);
        while t % 3 == 0 { t /= 3; s += 1; }
        if s == 0 {
            // cubing is a bijection: root = a^(3^{-1} mod N)
            let e3 = if n % 3 == 1 { (2 * n + 1) / 3 } else { (n + 1) / 3 };
            let one = r.one();
            return Cube { r, n, s, t, e3, c: one, w: one, p3s: 1 };
        }
        let mut k = 0u64;
        let g = loop { let g = r.cand(k); if rpow(&r, g, n / 3) != r.one() { break g; } k += 1; };
        let c = rpow(&r, g, t);                       // generator of the 3-Sylow, order 3^s
        let p3s = 3u128.pow(s);
        let w = rpow(&r, c, p3s / 3);                 // primitive cube root of unity
        let e3 = if t % 3 == 1 { (2 * t + 1) / 3 } else { (t + 1) / 3 }; // 3*e3 = 1 mod t
        Cube { r, n, s, t, e3, c, w, p3s }
    }
    pub fn bijective(&self) -> bool { self.s == 0 }
    /// One cube root of `a`, or None if `a` is not a cube.
    pub fn root(&self, a: R::E) -> Option<R::E> {
        let r = &self.r;
        if a == r.zero() { return Some(a); }
        if self.s == 0 { return Some(rpow(r, a, self.e3)); }
        if rpow(r, a, self.n / 3) != r.one() { return None; }
        let x0 = rpow(r, a, self.e3);                 // x0^3 = a * h, h in the 3-Sylow
        let h = rpow(r, a, 3 * self.e3 - 1);
        let w2 = r.mul(self.w, self.w);
        let mut l: u128 = 0;                          // h = c^l, l found digit by digit
        for j in 0..self.s {
            let corr = rpow(r, self.c, (self.p3s - l) % self.p3s);   // c^{-l}
            let x = r.mul(h, corr);
            let z = rpow(r, x, 3u128.pow(self.s - 1 - j));           // w^digit
            let d = if z == r.one() { 0 } else if z == self.w { 1 } else if z == w2 { 2 } else { return None };
            l += d * 3u128.pow(j);
        }
        if l % 3 != 0 { return None; }
        let y_inv = rpow(r, self.c, (self.p3s - l / 3) % self.p3s);  // c^{-l/3}
        Some(r.mul(x0, y_inv))
    }
}

// ---------- per-prime context (one-time precomputation, not counted per call) ----------
pub struct Ctx {
    pub f: Fp,
    inv2: u64, inv3: u64, inv4: u64, inv27: u64, inv_n: u64,
    n: u64,                     // quadratic non-residue
    s2: u32, t2: u64, zc: u64,  // Tonelli-Shanks: p-1 = 2^s2 t2, zc = n^t2
    cube_p: Cube<Fp>,
    cube_q: Cube<Fq2>,
    w2: (u64, u64),             // primitive cube root of unity in F_{p^2}
}
impl Ctx {
    pub fn new(p: u64) -> Self {
        assert!(p >= 5);
        let f = Fp::new(p);
        let n = { let mut z = 2u64; while f.powm(z, (p - 1) / 2) == 1 { z += 1; } z };
        let (mut t2, mut s2) = (p - 1, 0u32);
        while t2 & 1 == 0 { t2 >>= 1; s2 += 1; }
        let zc = f.powm(n, t2);
        let cube_p = Cube::new(f);
        let cube_q = Cube::new(Fq2 { f, n });
        let w2 = cube_q.w;
        let i4 = f.inv(4 % p); let i27 = f.inv(27 % p);
        Ctx { f, inv2: f.inv(2), inv3: f.inv(3), inv4: i4, inv27: i27, inv_n: f.inv(n), n, s2, t2, zc, cube_p, cube_q, w2 }
    }
    /// Square root in F_p (p%4==3: one pow; else Tonelli-Shanks), None for non-residues.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        let f = &self.f; let p = f.p;
        if a == 0 { return Some(0); }
        if p % 4 == 3 {
            let r = f.powm(a, (p + 1) / 4);
            return if f.sqr(r) == a { Some(r) } else { None };
        }
        if f.powm(a, (p - 1) / 2) != 1 { return None; }
        let mut m = self.s2; let mut c = self.zc;
        let mut t = f.powm(a, self.t2); let mut r = f.powm(a, (self.t2 + 1) / 2);
        loop {
            if t == 1 { return Some(r); }
            let mut i = 0u32; let mut t2 = t;
            while t2 != 1 && i < m { t2 = f.sqr(t2); i += 1; }
            if i == m { return None; }
            let b = f.powm(c, 1u64 << (m - i - 1));
            m = i; c = f.sqr(b); t = f.mul(t, c); r = f.mul(r, b);
        }
    }
    /// Distinct roots of monic y^2 + b y + c.
    pub fn quad_monic(&self, b: u64, c: u64) -> Vec<u64> {
        let f = &self.f;
        let disc = f.sub(f.sqr(b), f.mul(4, c));
        match self.sqrt(disc) {
            None => vec![],
            Some(0) => vec![f.mul(f.neg(b), self.inv2)],
            Some(s) => {
                let nb = f.neg(b);
                vec![f.mul(f.add(nb, s), self.inv2), f.mul(f.sub(nb, s), self.inv2)]
            }
        }
    }
    /// General quadratic a x^2 + b x + c (a != 0), sorted distinct roots.
    pub fn quad_roots(&self, a: u64, b: u64, c: u64) -> Vec<u64> {
        let f = &self.f;
        let ia = f.inv(a);
        let mut v = self.quad_monic(f.mul(b, ia), f.mul(c, ia));
        v.sort_unstable(); v.dedup(); v
    }
    /// F_p-roots of the depressed cubic t^3 + P t + Q = 0 (Cardano; F_{p^2} when D is a non-residue).
    pub fn cubic_depressed(&self, pp: u64, q: u64) -> Vec<u64> {
        let f = &self.f; let p = f.p;
        if pp == 0 {
            if q == 0 { return vec![0]; }
            return match self.cube_p.root(f.neg(q)) {
                None => vec![],
                Some(u) => if self.cube_p.bijective() { vec![u] } else {
                    let w = self.cube_p.w; let uw = f.mul(u, w);
                    vec![u, uw, f.mul(uw, w)]
                },
            };
        }
        let q2 = f.mul(q, self.inv2);
        let p2 = f.sqr(pp);
        let d = f.add(f.mul(q2, q2), f.mul(f.mul(p2, pp), self.inv27)); // q^2/4 + P^3/27
        if d == 0 {
            // double root -3Q/(2P), simple root 3Q/P
            let ip = f.inv(pp);
            let r1 = f.mul(f.mul(3, q), ip);
            let r2 = f.mul(f.neg(r1), self.inv2);
            return vec![r1, r2];
        }
        let nq2 = f.neg(q2);
        let m = f.neg(f.mul(pp, self.inv3));  // -P/3
        match self.sqrt(d) {
            Some(sg) => {
                let u3 = f.add(nq2, sg);
                match self.cube_p.root(u3) {
                    None => vec![],
                    Some(u) => {
                        let v = f.mul(m, f.inv(u));
                        if self.cube_p.bijective() { vec![f.add(u, v)] } else {
                            let w = self.cube_p.w; let w_2 = f.sqr(w);
                            let (mut uk, mut vk) = (u, v);
                            let mut out = vec![f.add(u, v)];
                            for _ in 0..2 {
                                uk = f.mul(uk, w); vk = f.mul(vk, w_2);
                                out.push(f.add(uk, vk));
                            }
                            out
                        }
                    }
                }
            }
            None => {
                // casus irreducibilis route: sqrt(D) = i * sqrt(D/n) in F_{p^2}
                let sg = self.sqrt(f.mul(d, self.inv_n)).expect("D/n must be a residue");
                let u3 = (nq2, sg);
                let k = &self.cube_q;
                let u = match k.root(u3) { None => return vec![], Some(u) => u };
                // 1/u = conj(u)/N(u)
                let nrm = f.sub(f.sqr(u.0), f.mul(self.n, f.sqr(u.1)));
                let ni = f.inv(nrm);
                let uinv = (f.mul(u.0, ni), f.mul(f.neg(u.1), ni));
                let v = (f.mul(m, uinv.0), f.mul(m, uinv.1));
                let w = self.w2; let w_2 = k.r.mul(w, w);
                let mut out = vec![];
                let (mut uk, mut vk) = (u, v);
                for kk in 0..3 {
                    if kk > 0 { uk = k.r.mul(uk, w); vk = k.r.mul(vk, w_2); }
                    let im = f.add(uk.1, vk.1);
                    if im == 0 { out.push(f.add(uk.0, vk.0)); }
                }
                let _ = p;
                out
            }
        }
    }
    /// F_p-roots of the monic cubic x^3 + c2 x^2 + c1 x + c0.
    pub fn cubic_monic(&self, c2: u64, c1: u64, c0: u64) -> Vec<u64> {
        let f = &self.f;
        let c2sq = f.sqr(c2);
        let pp = f.sub(c1, f.mul(c2sq, self.inv3));
        // q = 2 c2^3/27 - c1 c2/3 + c0
        let t1 = f.mul(f.mul(c2sq, c2), f.mul(2, self.inv27));
        let t2 = f.mul(f.mul(c1, c2), self.inv3);
        let q = f.add(f.sub(t1, t2), c0);
        let sh = f.mul(c2, self.inv3);
        self.cubic_depressed(pp, q).into_iter().map(|t| f.sub(t, sh)).collect()
    }
    /// Distinct F_p roots of a x^4 + b x^3 + c x^2 + d x + e (a != 0). Returns (sorted roots, used_fallback).
    pub fn quartic(&self, a: u64, b: u64, c: u64, d: u64, e: u64, seed: &mut u64) -> (Vec<u64>, bool) {
        let f = &self.f;
        // monic
        let (b1, c1, d1, e1) = if a == 1 { (b, c, d, e) } else {
            let ia = f.inv(a); (f.mul(b, ia), f.mul(c, ia), f.mul(d, ia), f.mul(e, ia))
        };
        // x = y - B, B = b1/4
        let bb = f.mul(b1, self.inv4);
        let b2 = f.sqr(bb);
        let pp = f.sub(c1, f.mul(6, b2));
        let b3 = f.mul(b2, bb);
        let qq = f.add(f.sub(d1, f.mul(f.mul(2, bb), c1)), f.mul(8, b3));
        let rr = f.sub(f.add(f.sub(e1, f.mul(bb, d1)), f.mul(b2, c1)), f.mul(3, f.sqr(b2)));
        let mut ys: Vec<u64> = vec![];
        let mut fallback = false;
        if qq == 0 {
            for z in self.quad_monic(pp, rr) {
                match self.sqrt(z) { None => {}, Some(0) => ys.push(0), Some(s) => { ys.push(s); ys.push(f.neg(s)); } }
            }
        } else {
            // resolvent in z = (r1+r2)^2:  z^3 + 2P z^2 + (P^2 - 4R) z - Q^2 = 0
            let zs = self.cubic_monic(f.mul(2, pp), f.sub(f.sqr(pp), f.mul(4, rr)), f.neg(f.sqr(qq)));
            if zs.is_empty() {
                // Frobenius type (1,3): no rational pairing; the unique root needs F_{p^3} radicals.
                // Use the distinct-degree (CZ) path on the depressed quartic - NOT radical.
                fallback = true;
                ys = cz_roots_o(f, &[rr, qq, pp, 0, 1], seed);
            } else {
                for &z in &zs {
                    if let Some(al) = self.sqrt(z) {
                        // quartic = (y^2 + al y + be)(y^2 - al y + ga)
                        let qa = f.mul(qq, f.inv(al));
                        let s = f.add(pp, z);
                        let be = f.mul(f.sub(s, qa), self.inv2);
                        let ga = f.mul(f.add(s, qa), self.inv2);
                        ys.extend(self.quad_monic(al, be));
                        ys.extend(self.quad_monic(f.neg(al), ga));
                        break;
                    }
                }
            }
        }
        let mut xs: Vec<u64> = ys.into_iter().map(|y| f.sub(y, bb)).collect();
        xs.sort_unstable(); xs.dedup();
        (xs, fallback)
    }
}

// ---------- Cantor-Zassenhaus reference (counted) ----------

fn trim(mut v: Vec<u64>) -> Vec<u64> { while v.last() == Some(&0) { v.pop(); } v }
fn pmul(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 { continue; }
        for (j, &y) in b.iter().enumerate() {
            let z = f.mul(x, y);
            r[i + j] = f.add(r[i + j], z);
        }
    }
    trim(r)
}
fn prem(f: &Fp, a: &[u64], m: &[u64]) -> Vec<u64> {
    let mut r = a.to_vec();
    if m.is_empty() { return r; }
    let inv = f.inv(*m.last().unwrap());
    while r.len() >= m.len() && !r.is_empty() {
        let shift = r.len() - m.len();
        let c = f.mul(*r.last().unwrap(), inv);
        for (k, &v) in m.iter().enumerate() {
            let p = f.mul(c, v);
            r[shift + k] = f.sub(r[shift + k], p);
        }
        r = trim(r);
    }
    r
}
fn pgcd(f: &Fp, mut a: Vec<u64>, mut b: Vec<u64>) -> Vec<u64> {
    a = trim(a); b = trim(b);
    while !b.is_empty() {
        let r = prem(f, &a, &b);
        a = b; b = r;
    }
    if let Some(&lead) = a.last() { let inv = f.inv(lead); for c in &mut a { *c = f.mul(*c, inv); } }
    a
}
fn ppow(f: &Fp, base: &[u64], mut e: u64, m: &[u64]) -> Vec<u64> {
    let mut r = prem(f, &[1], m);
    let mut b = prem(f, base, m);
    while e > 0 {
        if e & 1 == 1 { r = prem(f, &pmul(f, &r, &b), m); }
        b = prem(f, &pmul(f, &b, &b), m);
        e >>= 1;
    }
    r
}
// CZ: distinct F_p-roots of squarefree-or-not f. Returns sorted roots.
fn cz_roots(f: &Fp, poly: &[u64], seed: &mut u64) -> Vec<u64> {
    let poly = trim(poly.to_vec());
    if poly.len() <= 1 { return vec![]; }
    // g = gcd(X^p - X, poly) = product of distinct linear factors
    let xp = ppow(f, &[0, 1], f.p, &poly);
    let mut xpx = xp.clone();
    xpx.resize(xpx.len().max(2), 0);
    xpx[1] = f.sub(xpx[1], 1);
    let g = pgcd(f, poly.clone(), trim(xpx));
    let mut roots = vec![];
    let mut stack = vec![g];
    let rand = |s: &mut u64| -> u64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (*s >> 33) % f.p };
    while let Some(h) = stack.pop() {
        match h.len() {
            0 | 1 => {}
            2 => { let r = f.mul(f.sub(0, h[0]), f.inv(h[1])); roots.push(r); }
            _ => {
                let delta = rand(seed);
                let mut s = ppow(f, &[delta, 1], (f.p - 1) / 2, &h);
                if s.is_empty() { s.push(0); }
                s[0] = f.sub(s[0], 1);
                let d = pgcd(f, h.clone(), trim(s));
                if d.len() > 1 && d.len() < h.len() {
                    let q = {
                        // h / d via prem-free long division
                        let mut rem = h.clone(); let mut quo = vec![0u64; h.len()-d.len()+1];
                        let invlead = f.inv(*d.last().unwrap());
                        while rem.len() >= d.len() && !rem.is_empty() {
                            let sh = rem.len()-d.len();
                            let c = f.mul(*rem.last().unwrap(), invlead);
                            quo[sh] = c;
                            for (k,&v) in d.iter().enumerate() { let p = f.mul(c,v); rem[sh+k]=f.sub(rem[sh+k],p); }
                            rem = trim(rem);
                        }
                        trim(quo)
                    };
                    stack.push(d); stack.push(q);
                } else { stack.push(h); }
            }
        }
    }
    roots.sort_unstable(); roots.dedup(); roots
}

// ---------- OPTIMIZED CZ baseline: no repeated leading-coefficient inversions (monic modulus), same counted primitive ----------

fn trim_o(mut v: Vec<u64>) -> Vec<u64> { while v.last() == Some(&0) { v.pop(); } v }
fn pmul_o(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 { continue; }
        for (j, &y) in b.iter().enumerate() {
            let z = f.mul(x, y);
            r[i + j] = f.add(r[i + j], z);
        }
    }
    trim_o(r)
}
fn prem_o(f: &Fp, a: &[u64], m: &[u64]) -> Vec<u64> {
    let mut r = a.to_vec();
    if m.is_empty() { return r; }
    let lead = *m.last().unwrap(); let inv = if lead == 1 { 1 } else { f.inv(lead) };
    while r.len() >= m.len() && !r.is_empty() {
        let shift = r.len() - m.len();
        let c = f.mul(*r.last().unwrap(), inv);
        for (k, &v) in m.iter().enumerate() {
            let p = f.mul(c, v);
            r[shift + k] = f.sub(r[shift + k], p);
        }
        r = trim_o(r);
    }
    r
}
fn pgcd_o(f: &Fp, mut a: Vec<u64>, mut b: Vec<u64>) -> Vec<u64> {
    a = trim_o(a); b = trim_o(b);
    while !b.is_empty() {
        let r = prem_o(f, &a, &b);
        a = b; b = r;
    }
    if let Some(&lead) = a.last() { let inv = f.inv(lead); for c in &mut a { *c = f.mul(*c, inv); } }
    a
}
fn ppow_o(f: &Fp, base: &[u64], mut e: u64, m: &[u64]) -> Vec<u64> {
    let mut r = prem_o(f, &[1], m);
    let mut b = prem_o(f, base, m);
    while e > 0 {
        if e & 1 == 1 { r = prem_o(f, &pmul_o(f, &r, &b), m); }
        b = prem_o(f, &pmul_o(f, &b, &b), m);
        e >>= 1;
    }
    r
}
// CZ: distinct F_p-roots of squarefree-or-not f. Returns sorted roots.
fn cz_roots_o(f: &Fp, poly: &[u64], seed: &mut u64) -> Vec<u64> {
    let mut poly = trim_o(poly.to_vec());
    if poly.len() <= 1 { return vec![]; }
    if let Some(&lead) = poly.last() { if lead != 1 { let il = f.inv(lead); for c in &mut poly { *c = f.mul(*c, il); } } }
    // g = gcd(X^p - X, poly) = product of distinct linear factors
    let xp = ppow_o(f, &[0, 1], f.p, &poly);
    let mut xpx = xp.clone();
    xpx.resize(xpx.len().max(2), 0);
    xpx[1] = f.sub(xpx[1], 1);
    let g = pgcd_o(f, poly.clone(), trim_o(xpx));
    let mut roots = vec![];
    let mut stack = vec![g];
    let rand = |s: &mut u64| -> u64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (*s >> 33) % f.p };
    while let Some(h) = stack.pop() {
        match h.len() {
            0 | 1 => {}
            2 => { let r = f.mul(f.sub(0, h[0]), f.inv(h[1])); roots.push(r); }
            _ => {
                let delta = rand(seed);
                let mut s = ppow_o(f, &[delta, 1], (f.p - 1) / 2, &h);
                if s.is_empty() { s.push(0); }
                s[0] = f.sub(s[0], 1);
                let d = pgcd_o(f, h.clone(), trim_o(s));
                if d.len() > 1 && d.len() < h.len() {
                    let q = {
                        // h / d via prem_o-free long division
                        let mut rem = h.clone(); let mut quo = vec![0u64; h.len()-d.len()+1];
                        let invlead = f.inv(*d.last().unwrap());
                        while rem.len() >= d.len() && !rem.is_empty() {
                            let sh = rem.len()-d.len();
                            let c = f.mul(*rem.last().unwrap(), invlead);
                            quo[sh] = c;
                            for (k,&v) in d.iter().enumerate() { let p = f.mul(c,v); rem[sh+k]=f.sub(rem[sh+k],p); }
                            rem = trim_o(rem);
                        }
                        trim_o(quo)
                    };
                    stack.push(d); stack.push(q);
                } else { stack.push(h); }
            }
        }
    }
    roots.sort_unstable(); roots.dedup(); roots
}


// ---------- validation + measurement ----------
fn eval(f: &Fp, poly: &[u64], x: u64) -> u64 { poly.iter().rev().fold(0u64, |a, &b| f.add(f.mul(a, x), b)) }
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for &pr in &[2u64,3,5,7,11,13,17,19,23,29,31,37] { if n % pr == 0 { return n == pr; } }
    let mut d = n - 1; let mut r = 0; while d & 1 == 0 { d >>= 1; r += 1; }
    'wit: for &a in &[2u64,3,5,7,11,13,17,19,23,29,31,37] {
        let mut x = { let mut res=1u128; let mut b=a as u128 % n as u128; let mut e=d; while e>0 { if e&1==1 {res=res*b%n as u128;} b=b*b%n as u128; e>>=1;} res } as u64;
        if x == 1 || x == n-1 { continue; }
        for _ in 0..r-1 { x = ((x as u128 * x as u128) % n as u128) as u64; if x == n-1 { continue 'wit; } }
        return false;
    }
    true
}
fn lcg(s: &mut u64) -> u64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *s >> 33 }

fn cube_validate() {
    use std::collections::HashSet;
    let mut primes: Vec<u64> = (5..160).filter(|&n| is_prime(n)).collect();
    primes.extend([163, 487, 647, 1459]);
    let (mut fp_pairs, mut fp_roots, mut q_pairs, mut q_roots) = (0u64, 0u64, 0u64, 0u64);
    let mut deep = vec![];
    for &p in &primes {
        let f = Fp::new(p);
        // ---- F_p ----
        let k = Cube::new(f);
        let mut cubes = HashSet::new();
        for x in 0..p { cubes.insert(f.mul(x, f.mul(x, x))); }
        for a in 0..p {
            match (k.root(a), cubes.contains(&a)) {
                (Some(r), true) => { assert_eq!(f.mul(r, f.mul(r, r)), a, "Fp cbrt wrong p={p} a={a}"); fp_roots += 1; }
                (None, false) => {}
                (g, b) => panic!("Fp existence mismatch p={p} a={a} got={g:?} brute={b}"),
            }
            fp_pairs += 1;
        }
        // ---- F_{p^2} (all p^2 elements) for p <= 700 ----
        if p <= 700 {
            let n = { let mut z = 2u64; while f.powm(z, (p - 1) / 2) == 1 { z += 1; } z };
            let r = Fq2 { f, n };
            let kq = Cube::new(r);
            let mut cs: HashSet<(u64, u64)> = HashSet::new();
            for x in 0..p { for y in 0..p {
                let e = (x, y); cs.insert(r.mul(e, r.mul(e, e)));
            }}
            for a in 0..p { for b in 0..p {
                let e = (a, b);
                match (kq.root(e), cs.contains(&e)) {
                    (Some(rt), true) => { assert_eq!(r.mul(rt, r.mul(rt, rt)), e, "Fq2 cbrt wrong p={p} e={e:?}"); q_roots += 1; }
                    (None, false) => {}
                    (g, b2) => panic!("Fq2 existence mismatch p={p} e={e:?} got={g:?} brute={b2}"),
                }
                q_pairs += 1;
            }}
            let s3 = { let mut m = (p as u128)*(p as u128) - 1; let mut s=0; while m%3==0 { m/=3; s+=1;} s };
            let s3p = { let mut m = p-1; let mut s=0; while m%3==0 { m/=3; s+=1;} s };
            deep.push((p, s3p, s3));
        }
    }
    println!("F_p cube root: {fp_pairs} (a,p) pairs over {} primes ({} with p%3==1, {} with p%3==2), {fp_roots} real cube roots verified",
        primes.len(), primes.iter().filter(|&&p| p%3==1).count(), primes.iter().filter(|&&p| p%3==2).count());
    println!("F_p^2 cube root: {q_pairs} elements (all of F_p^2 for each p<=700), {q_roots} real cube roots verified");
    let maxs = deep.iter().map(|d| d.2).max().unwrap();
    let maxs1 = deep.iter().map(|d| d.1).max().unwrap();
    println!("3-adic depth s covered: F_p up to {maxs1}, F_p^2 (v3(p^2-1)) up to {maxs}");
    // large-prime spot checks of AMM: random cubes always recovered, random non-cubes rejected
    let mut seed = 99u64;
    for &p in &[1_000_000_007u64, 998_244_353, 2305843009213693951, 4611686018427387847] {
        if !is_prime(p) { continue; }
        let f = Fp::new(p); let ctx = Ctx::new(p);
        let (mut yes, mut no) = (0, 0);
        for _ in 0..300 {
            let x = lcg(&mut seed) % p; let a = f.mul(x, f.mul(x, x));
            let r = ctx.cube_p.root(a).expect("cube lost"); assert_eq!(f.mul(r, f.mul(r, r)), a); yes += 1;
            let y = (lcg(&mut seed) % p, lcg(&mut seed) % p);
            let r2 = Fq2 { f, n: ctx.n }; let a2 = r2.mul(y, r2.mul(y, y));
            let z = ctx.cube_q.root(a2).expect("cube lost F_p^2"); assert_eq!(r2.mul(z, r2.mul(z, z)), a2);
            let t = lcg(&mut seed) % p; if ctx.cube_p.root(t).is_none() { no += 1; }
        }
        println!("  large p={p} (p%3={}): 300 random cubes recovered in F_p and F_p^2, {no}/300 random F_p elems rejected as non-cubes (yes={yes})", p % 3);
    }
    println!("CUBE ROOT VALIDATION PASSED");
}

fn brute_roots(f: &Fp, poly: &[u64]) -> Vec<u64> { (0..f.p).filter(|&x| eval(f, poly, x) == 0).collect() }

fn quartic_validate() {
    let primes: Vec<u64> = (5..40).filter(|&n| is_prime(n)).collect();
    println!("primes: {:?}", primes);
    let mut grand = 0u64; let mut hist = [0u64; 5]; let mut fb_total = 0u64;
    for &p in &primes {
        let ctxp = p;
        // parallel over leading coefficient a
        let mut handles = vec![];
        let chunks: Vec<Vec<u64>> = { let mut c = vec![vec![]; 4]; for a in 1..p { c[(a % 4) as usize].push(a); } c };
        for ch in chunks {
            handles.push(std::thread::spawn(move || {
                let ctx = Ctx::new(ctxp); let f = ctx.f; let mut seed = 7u64;
                let mut cnt = 0u64; let mut hist = [0u64; 5]; let mut fb = 0u64;
                for &a in &ch { for b in 0..ctxp { for c in 0..ctxp { for d in 0..ctxp { for e in 0..ctxp {
                    let (got, usedfb) = ctx.quartic(a, b, c, d, e, &mut seed);
                    let want = brute_roots(&f, &[e, d, c, b, a]);
                    assert_eq!(got, want, "quartic mismatch p={ctxp} ({a},{b},{c},{d},{e})");
                    hist[want.len()] += 1; if usedfb { fb += 1; } cnt += 1;
                }}}}}
                (cnt, hist, fb)
            }));
        }
        let mut pc = 0u64;
        for h in handles { let (c, hh, fb) = h.join().unwrap(); pc += c; for i in 0..5 { hist[i] += hh[i]; } fb_total += fb; }
        grand += pc;
        println!("  p={p} (p%3={}) all {pc} tuples ok", p % 3);
    }
    println!("QUARTIC exhaustive: {grand} (a,b,c,d,e) tuples over {} primes agree with brute force; root-count histogram [0..4] = {:?}; (1,3)-fallback used on {fb_total}", primes.len(), hist);

    // vs reference CZ on random primes up to 2^61-1
    let big: Vec<u64> = vec![1_000_003, 1_000_033, 1_000_000_007, 998_244_353, 2147483647, 2305843009213693951, 1_000_000_009, 4_294_967_291];
    let mut seed = 424242u64; let mut agree = 0u64; let mut byc = [0u64; 5];
    for &p in &big {
        let ctx = Ctx::new(p); let f = ctx.f;
        for it in 0..3000 {
            let (a,b,c,d,e);
            if it % 4 == 0 { // force 4 distinct roots / structured
                let r: Vec<u64> = (0..4).map(|_| lcg(&mut seed) % p).collect();
                let mut pl = vec![1u64];
                for &ri in &r { let mut np = vec![0u64; pl.len()+1]; for (i,&cf) in pl.iter().enumerate() { np[i+1] = f.add(np[i+1], cf); np[i] = f.sub(np[i], f.mul(cf, ri)); } pl = np; }
                a = pl[4]; b = pl[3]; c = pl[2]; d = pl[1]; e = pl[0];
            } else { a = 1 + lcg(&mut seed) % (p-1); b = lcg(&mut seed)%p; c = lcg(&mut seed)%p; d = lcg(&mut seed)%p; e = lcg(&mut seed)%p; }
            let (got, _) = ctx.quartic(a,b,c,d,e,&mut seed);
            let want = cz_roots(&f, &[e,d,c,b,a], &mut seed);
            assert_eq!(got, want, "vs CZ mismatch p={p} ({a},{b},{c},{d},{e})");
            for &g in &got { assert_eq!(eval(&f, &[e,d,c,b,a], g), 0); }
            byc[want.len()] += 1; agree += 1;
        }
    }
    println!("QUARTIC vs CZ: {agree} random/structured quartics over {} primes (up to 2^61-1) agree; root-count classes [0..4] = {:?}", big.len(), byc);
    println!("QUARTIC VALIDATION PASSED");
}

fn measure() {
    let primes: Vec<u64> = vec![1_000_003, 1_000_033, 1_000_000_007, 998_244_353, 2147483647, 2305843009213693951];
    let mut seed = 20261117u64;
    println!("Field multiplications per quartic (mean), radical vs CZ-harness (as in PR radical_rootfind: re-inverts the lead coeff on every reduction) vs CZ-opt (monic modulus, no repeated inversions)\n");
    println!("| p | p%3 | group | polys | radical | CZ-harness | CZ-opt | CZ-harness/rad | CZ-opt/rad |");
    println!("|---|---|---|---:|---:|---:|---:|---:|---:|");
    for &p in &primes {
        let ctx = Ctx::new(p); let f = ctx.f;
        // per root-count class: [count, rad, czh, czo]; fb flag class tracked separately
        let mut cls = [[0u64; 4]; 5];
        let mut fbs = [0u64; 5];
        for _ in 0..6000 {
            let b = lcg(&mut seed)%p; let c = lcg(&mut seed)%p; let d = lcg(&mut seed)%p; let e = lcg(&mut seed)%p;
            reset(); let (got, fb) = ctx.quartic(1,b,c,d,e,&mut seed); let mr = muls();
            reset(); let want = cz_roots(&f, &[e,d,c,b,1], &mut seed); let mc = muls();
            reset(); let wo = cz_roots_o(&f, &[e,d,c,b,1], &mut seed); let mo = muls();
            assert_eq!(got, want); assert_eq!(wo, want);
            let k = want.len(); cls[k][0]+=1; cls[k][1]+=mr; cls[k][2]+=mc; cls[k][3]+=mo; if fb { fbs[k]+=1; }
        }
        let row = |name: String, v: [u64; 4]| {
            let n = v[0].max(1);
            println!("| {p} | {} | {name} | {} | {} | {} | {} | {:.2}x | {:.2}x |", p%3, v[0], v[1]/n, v[2]/n, v[3]/n, v[2] as f64/v[1].max(1) as f64, v[3] as f64/v[1].max(1) as f64);
        };
        let mut none = [0u64;4]; let mut some = [0u64;4]; let mut all = [0u64;4];
        for k in 0..5 {
            if cls[k][0] == 0 { continue; }
            row(format!("{k} roots{}", if fbs[k]>0 {" (1,3)-fallback"} else {""}), cls[k]);
            for t in 0..4 { all[t]+=cls[k][t]; if k==0 { none[t]+=cls[k][t]; } else { some[t]+=cls[k][t]; } }
        }
        row("**no-root**".into(), none); row("**root-exists**".into(), some); row("**all**".into(), all);
    }
}

fn measure_quad() {
    println!("\nQuadratic recheck (radical vs CZ-harness vs CZ-opt), random monic quadratics, mean field muls\n");
    println!("| p | group | radical | CZ-harness | CZ-opt | harness/rad | opt/rad |");
    println!("|---|---|---:|---:|---:|---:|---:|");
    let mut seed = 77u64;
    for &p in &[1_000_003u64, 1_000_000_007, 2305843009213693951] {
        let ctx = Ctx::new(p); let f = ctx.f;
        let mut g = [[0u64; 4]; 2];
        for _ in 0..4000 {
            let b = lcg(&mut seed)%p; let c = lcg(&mut seed)%p;
            reset(); let r = ctx.quad_roots(1,b,c); let mr = muls();
            reset(); let w = cz_roots(&f, &[c,b,1], &mut seed); let mc = muls();
            reset(); let wo = cz_roots_o(&f, &[c,b,1], &mut seed); let mo = muls();
            assert_eq!(r, w); assert_eq!(wo, w);
            let k = if w.is_empty() {0} else {1}; g[k][0]+=1; g[k][1]+=mr; g[k][2]+=mc; g[k][3]+=mo;
        }
        for (k, nm) in [(0,"no-root"),(1,"root-exists")] { let v=g[k]; let n=v[0];
            println!("| {p} | {nm} | {} | {} | {} | {:.1}x | {:.1}x |", v[1]/n, v[2]/n, v[3]/n, v[2] as f64/v[1] as f64, v[3] as f64/v[1] as f64); }
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("cube") => cube_validate(),
        Some("quartic") => quartic_validate(),
        Some("measure") => { measure(); measure_quad(); }
        _ => eprintln!("usage: radical_quartic cube|quartic|measure"),
    }
}
