//! Route S toy pipeline (W3b, T8): the Joux-Vitse / Momose-Chao `m = 3, r = 5`
//! hyperelliptic descent `E0/F_{q^3} -> H/F_q`, then Smith's `psi` to a plane
//! quartic and a discrete log there.
//!
//! The descent part (field, polynomials, Cantor, the cover `build`, `fwd`,
//! `frc`, `cof`) is ported verbatim from
//! `research/ghs_cubic_hyperelliptic_genus3_20261013/hyper_genus3.rs`, lines
//! 7-220 (its P1-P5 checks passed there); only the driver at the end of this
//! file is new.  Synthetic toy fields only; no deployed-curve, sub-rho or
//! key-recovery claim.
#![allow(dead_code, unused_imports, clippy::all)]

use std::collections::{HashMap, HashSet};

const SEED: u64 = 20261013;
struct Rng(u64);
impl Rng {
    fn new(tag: u64) -> Self {
        Rng(SEED ^ tag.wrapping_mul(0x9E3779B97F4A7C15))
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

// ---------------------------------------------------------------- finite fields, log representation
// 0 is zero, v >= 1 is g^(v-1). Index = coefficients (base q, little endian) in the polynomial basis.
struct F {
    q: u32,
    size: u32,
    m: u32,
    zech: Vec<u32>,
    idx_of: Vec<u32>,
    el_of: Vec<u32>,
    modulus: Vec<u32>,
}
fn digits(q: u32, n: usize, mut i: u32) -> Vec<u32> {
    let mut v = vec![0; n];
    for k in 0..n {
        v[k] = i % q;
        i /= q;
    }
    v
}
fn index(q: u32, v: &[u32]) -> u32 {
    let mut i = 0;
    for k in (0..v.len()).rev() {
        i = i * q + v[k];
    }
    i
}
impl F {
    fn finish(q: u32, n: u32, exp_idx: Vec<u32>, modulus: Vec<u32>) -> F {
        let size = q.pow(n);
        let m = size - 1;
        let mut el_of = vec![0u32; size as usize];
        let mut idx_of = vec![0u32; size as usize];
        for k in 0..m as usize {
            el_of[exp_idx[k] as usize] = k as u32 + 1;
            idx_of[k + 1] = exp_idx[k];
        }
        let mut zech = vec![0u32; m as usize];
        for k in 0..m as usize {
            let i = exp_idx[k];
            let d0 = i % q;
            let j = i - d0 + (d0 + 1) % q;
            zech[k] = el_of[j as usize];
        }
        F {
            q,
            size,
            m,
            zech,
            idx_of,
            el_of,
            modulus,
        }
    }
    // L = F_{q^n}: first monic x^n + sum c_i x^i (c in index order, c0 != 0) with x primitive
    fn build_l(q: u32, n: u32) -> F {
        let size = q.pow(n);
        let m = size - 1;
        let nn = n as usize;
        for cand in 0..size {
            let c = digits(q, nn, cand);
            if c[0] == 0 {
                continue;
            }
            let mut exp_idx = vec![0u32; m as usize];
            let mut cur = vec![0u32; nn];
            cur[0] = 1;
            let mut ok = true;
            for k in 0..m as usize {
                let ci = index(q, &cur);
                if k > 0 && ci == 1 {
                    ok = false;
                    break;
                }
                exp_idx[k] = ci;
                let top = cur[nn - 1];
                for i in (1..nn).rev() {
                    cur[i] = (cur[i - 1] + (q - c[i]) * top) % q;
                }
                cur[0] = ((q - c[0]) * top) % q;
            }
            if !ok || index(q, &cur) != 1 {
                continue;
            }
            return F::finish(q, n, exp_idx, c);
        }
        panic!("no primitive polynomial");
    }
    // verbatim field search of research/ghs_cubic_cover_census_20261012/cover_census.rs (same representation)
    fn build_census(q: u32, n: u32) -> F {
        let size = q.pow(n);
        let m = size - 1;
        let nn = n as usize;
        for cand in 0..size {
            let md = digits(q, nn, cand);
            for s in 0..q {
                let mut exp_idx = vec![0u32; m as usize];
                let mut cur = vec![0u32; nn];
                cur[0] = 1;
                let mut ok = true;
                for k in 0..m as usize {
                    let ci = index(q, &cur);
                    if ci == 0 || (k > 0 && ci == 1) {
                        ok = false;
                        break;
                    }
                    exp_idx[k] = ci;
                    let mut nx = vec![0u32; nn];
                    let top = cur[nn - 1];
                    for i in 0..nn {
                        let sh = if i > 0 { cur[i - 1] } else { 0 };
                        nx[i] = (sh + (q - md[i]) * top + s * cur[i]) % q;
                    }
                    cur = nx;
                }
                if !ok || index(q, &cur) != 1 {
                    continue;
                }
                return F::finish(q, n, exp_idx, md);
            }
        }
        panic!("no field");
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            0
        } else {
            let s = a + b - 2;
            (if s >= self.m { s - self.m } else { s }) + 1
        }
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        assert!(a != 0, "inverse of zero");
        (self.m - (a - 1)) % self.m + 1
    }
    #[inline]
    fn div(&self, a: u32, b: u32) -> u32 {
        self.mul(a, self.inv(b))
    }
    #[inline]
    fn neg(&self, a: u32) -> u32 {
        if a == 0 {
            0
        } else {
            (a - 1 + self.m / 2) % self.m + 1
        }
    }
    #[inline]
    fn add(&self, a: u32, b: u32) -> u32 {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        let dlt = if b >= a { b - a } else { b + self.m - a };
        let z = self.zech[dlt as usize];
        if z == 0 {
            0
        } else {
            let s = a + z - 2;
            (if s >= self.m { s - self.m } else { s }) + 1
        }
    }
    #[inline]
    fn sub(&self, a: u32, b: u32) -> u32 {
        self.add(a, self.neg(b))
    }
    #[inline]
    fn frob(&self, a: u32) -> u32 {
        if a == 0 {
            0
        } else {
            (((a - 1) as u64 * self.q as u64) % self.m as u64) as u32 + 1
        }
    }
    fn pow(&self, a: u32, e: u64) -> u32 {
        if e == 0 {
            return 1;
        }
        if a == 0 {
            0
        } else {
            (((a - 1) as u64 * (e % self.m as u64)) % self.m as u64) as u32 + 1
        }
    }
    fn int(&self, c: i64) -> u32 {
        self.el_of[c.rem_euclid(self.q as i64) as usize]
    }
    fn to_int(&self, v: u32) -> u32 {
        self.idx_of[v as usize]
    } // prime-field elements only
    fn step(&self, k: u32) -> u32 {
        self.m / (self.q.pow(k) - 1)
    }
    fn in_sub(&self, v: u32, k: u32) -> bool {
        v == 0 || (v - 1) % self.step(k) == 0
    }
    fn chi(&self, v: u32, k: u32) -> i32 {
        if v == 0 {
            return 0;
        }
        let s = self.step(k);
        assert!((v - 1) % s == 0, "chi outside subfield");
        if ((v - 1) / s) % 2 == 0 {
            1
        } else {
            -1
        }
    }
    fn sqrt(&self, v: u32) -> Option<u32> {
        if v == 0 {
            Some(0)
        } else if (v - 1) % 2 == 0 {
            Some((v - 1) / 2 + 1)
        } else {
            None
        }
    }
    fn elems(&self, k: u32) -> Vec<u32> {
        let s = self.step(k);
        let mut v = vec![0u32];
        for t in 0..(self.q.pow(k) - 1) {
            v.push(t * s + 1);
        }
        v
    }
}

// ---------------------------------------------------------------- polynomials over L, low -> high, trimmed
type P = Vec<u32>;
fn tr(mut a: P) -> P {
    while let Some(&0) = a.last() {
        a.pop();
    }
    a
}
fn dg(a: &P) -> i32 {
    a.len() as i32 - 1
}
fn padd(f: &F, a: &P, b: &P) -> P {
    let n = a.len().max(b.len());
    tr((0..n)
        .map(|i| f.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
        .collect())
}
fn pneg(f: &F, a: &P) -> P {
    a.iter().map(|&c| f.neg(c)).collect()
}
fn psub(f: &F, a: &P, b: &P) -> P {
    padd(f, a, &pneg(f, b))
}
fn pmul(f: &F, a: &P, b: &P) -> P {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u32; a.len() + b.len() - 1];
    for i in 0..a.len() {
        if a[i] == 0 {
            continue;
        }
        for j in 0..b.len() {
            if b[j] != 0 {
                r[i + j] = f.add(r[i + j], f.mul(a[i], b[j]));
            }
        }
    }
    tr(r)
}
fn psc(f: &F, a: &P, c: u32) -> P {
    tr(a.iter().map(|&x| f.mul(x, c)).collect())
}
fn pdivrem(f: &F, a: &P, b: &P) -> (P, P) {
    assert!(!b.is_empty(), "division by zero polynomial");
    let mut r = a.clone();
    if r.len() < b.len() {
        return (vec![], r);
    }
    let mut qv = vec![0u32; r.len() - b.len() + 1];
    let li = f.inv(*b.last().unwrap());
    while r.len() >= b.len() {
        let k = r.len() - b.len();
        let c = f.mul(*r.last().unwrap(), li);
        qv[k] = c;
        for j in 0..b.len() {
            r[k + j] = f.sub(r[k + j], f.mul(c, b[j]));
        }
        r = tr(r);
        if r.is_empty() {
            break;
        }
    }
    (tr(qv), r)
}
fn pmod(f: &F, a: &P, b: &P) -> P {
    pdivrem(f, a, b).1
}
fn pexact(f: &F, a: &P, b: &P) -> P {
    let (qq, r) = pdivrem(f, a, b);
    assert!(r.is_empty(), "inexact division");
    qq
}
fn pmonic(f: &F, a: &P) -> P {
    if a.is_empty() {
        return vec![];
    }
    psc(f, a, f.inv(*a.last().unwrap()))
}
fn pgcd(f: &F, a: &P, b: &P) -> P {
    let (mut x, mut y) = (a.clone(), b.clone());
    while !y.is_empty() {
        let r = pmod(f, &x, &y);
        x = y;
        y = r;
    }
    pmonic(f, &x)
}
fn pxgcd(f: &F, a: &P, b: &P) -> (P, P, P) {
    // s a + t b = g, g monic
    let (mut r0, mut r1) = (a.clone(), b.clone());
    let (mut s0, mut s1) = (vec![1u32], vec![]);
    let (mut t0, mut t1) = (vec![], vec![1u32]);
    while !r1.is_empty() {
        let (qq, r) = pdivrem(f, &r0, &r1);
        r0 = r1;
        r1 = r;
        let s2 = psub(f, &s0, &pmul(f, &qq, &s1));
        s0 = s1;
        s1 = s2;
        let t2 = psub(f, &t0, &pmul(f, &qq, &t1));
        t0 = t1;
        t1 = t2;
    }
    let li = f.inv(*r0.last().unwrap());
    (psc(f, &r0, li), psc(f, &s0, li), psc(f, &t0, li))
}
fn pev(f: &F, a: &P, x: u32) -> u32 {
    let mut r = 0;
    for &c in a.iter().rev() {
        r = f.add(f.mul(r, x), c);
    }
    r
}
fn pder(f: &F, a: &P) -> P {
    tr((1..a.len()).map(|i| f.mul(f.int(i as i64), a[i])).collect())
}
fn p_in(f: &F, a: &P, k: u32) -> bool {
    a.iter().all(|&c| f.in_sub(c, k))
}
fn plin(f: &F, r: u32) -> P {
    vec![f.neg(r), 1]
}
fn pints(f: &F, a: &P) -> String {
    format!(
        "[{}]",
        a.iter()
            .map(|&c| f.to_int(c).to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

// ---------------------------------------------------------------- Cantor arithmetic, w^2 = h(z), deg h = 2g+1
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Dv {
    u: P,
    v: P,
}
fn dzero() -> Dv {
    Dv {
        u: vec![1],
        v: vec![],
    }
}
fn dneg(f: &F, d: &Dv) -> Dv {
    Dv {
        u: d.u.clone(),
        v: pneg(f, &d.v),
    }
}
fn dpt(f: &F, z: u32, w: u32) -> Dv {
    Dv {
        u: plin(f, z),
        v: tr(vec![w]),
    }
}
fn dred(f: &F, h: &P, g: i32, u: P, v: P) -> Dv {
    let mut u = pmonic(f, &u);
    let mut v = pmod(f, &v, &u);
    while dg(&u) > g {
        let num = psub(f, h, &pmul(f, &v, &v));
        u = pmonic(f, &pexact(f, &num, &u));
        v = pmod(f, &pneg(f, &v), &u);
    }
    if dg(&u) <= 0 {
        return dzero();
    }
    Dv { u, v }
}
fn dadd(f: &F, h: &P, g: i32, a: &Dv, b: &Dv) -> Dv {
    let (d0, e1, e2) = pxgcd(f, &a.u, &b.u);
    let vs = padd(f, &a.v, &b.v);
    let (d, c1, c2) = pxgcd(f, &d0, &vs);
    let (s1, s2, s3) = (pmul(f, &c1, &e1), pmul(f, &c1, &e2), c2);
    let u = pexact(f, &pmul(f, &a.u, &b.u), &pmul(f, &d, &d));
    let t = padd(
        f,
        &padd(
            f,
            &pmul(f, &pmul(f, &s1, &a.u), &b.v),
            &pmul(f, &pmul(f, &s2, &b.u), &a.v),
        ),
        &pmul(f, &s3, &padd(f, &pmul(f, &a.v, &b.v), h)),
    );
    let v = pexact(f, &t, &d);
    dred(f, h, g, u, v)
}
fn dmul(f: &F, h: &P, g: i32, mut n: u64, a: &Dv) -> Dv {
    let mut r = dzero();
    let mut b = a.clone();
    while n > 0 {
        if n & 1 == 1 {
            r = dadd(f, h, g, &r, &b);
        }
        b = dadd(f, h, g, &b, &b);
        n >>= 1;
    }
    r
}
fn dv_in(f: &F, d: &Dv) -> bool {
    p_in(f, &d.u, 1) && p_in(f, &d.v, 1)
}
fn dv_ok(f: &F, h: &P, d: &Dv) -> bool {
    pmod(f, &psub(f, &pmul(f, &d.v, &d.v), h), &d.u).is_empty() && dg(&d.v) < dg(&d.u).max(0)
        || d == &dzero()
}

// ---------------------------------------------------------------- elliptic curve Y^2 = S^3 + a2 S^2 + a4 S + a6
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum WP {
    O,
    A(u32, u32),
}
struct Wc {
    a2: u32,
    a4: u32,
    a6: u32,
}
fn wrhs(f: &F, w: &Wc, s: u32) -> u32 {
    let s2 = f.mul(s, s);
    f.add(
        f.add(f.add(f.mul(s2, s), f.mul(w.a2, s2)), f.mul(w.a4, s)),
        w.a6,
    )
}
fn wneg(f: &F, p: WP) -> WP {
    match p {
        WP::O => WP::O,
        WP::A(x, y) => WP::A(x, f.neg(y)),
    }
}
fn wadd(f: &F, w: &Wc, p: WP, q: WP) -> WP {
    let (x1, y1, x2, y2) = match (p, q) {
        (WP::O, _) => return q,
        (_, WP::O) => return p,
        (WP::A(a, b), WP::A(c, d)) => (a, b, c, d),
    };
    let lam = if x1 == x2 {
        if f.add(y1, y2) == 0 {
            return WP::O;
        }
        f.div(
            f.add(
                f.add(
                    f.mul(f.int(3), f.mul(x1, x1)),
                    f.mul(f.int(2), f.mul(w.a2, x1)),
                ),
                w.a4,
            ),
            f.mul(f.int(2), y1),
        )
    } else {
        f.div(f.sub(y2, y1), f.sub(x2, x1))
    };
    let x3 = f.sub(f.sub(f.sub(f.mul(lam, lam), w.a2), x1), x2);
    let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
    WP::A(x3, y3)
}
fn wmul(f: &F, w: &Wc, mut n: u64, p: WP) -> WP {
    let mut r = WP::O;
    let mut b = p;
    while n > 0 {
        if n & 1 == 1 {
            r = wadd(f, w, r, b);
        }
        b = wadd(f, w, b, b);
        n >>= 1;
    }
    r
}

// ---------------------------------------------------------------- 3x3 linear algebra over L
type M3 = [[u32; 3]; 3];
fn mv(f: &F, m: &M3, v: &[u32; 3]) -> [u32; 3] {
    let mut r = [0u32; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[i] = f.add(r[i], f.mul(m[i][j], v[j]));
        }
    }
    r
}
fn inv3(f: &F, a: &M3) -> Option<M3> {
    let mut c = [[0u32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = f.sub(
                f.mul(a[(j + 1) % 3][(i + 1) % 3], a[(j + 2) % 3][(i + 2) % 3]),
                f.mul(a[(j + 1) % 3][(i + 2) % 3], a[(j + 2) % 3][(i + 1) % 3]),
            );
        }
    }
    let det = f.add(
        f.add(f.mul(a[0][0], c[0][0]), f.mul(a[0][1], c[1][0])),
        f.mul(a[0][2], c[2][0]),
    );
    if det == 0 {
        return None;
    }
    let di = f.inv(det);
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = f.mul(c[i][j], di);
        }
    }
    Some(c)
}
fn bil(f: &F, g: &M3, a: &[u32; 3], b: &[u32; 3]) -> u32 {
    let gb = mv(f, g, b);
    f.add(
        f.add(f.mul(a[0], gb[0]), f.mul(a[1], gb[1])),
        f.mul(a[2], gb[2]),
    )
}
fn parallel(f: &F, a: &[u32; 3], b: &[u32; 3]) -> bool {
    (0..3).all(|i| {
        let j = (i + 1) % 3;
        f.mul(a[i], b[j]) == f.mul(a[j], b[i])
    })
}

// ---------------------------------------------------------------- the cover
struct Inst {
    split: bool,
    c0: u32,
    c1: u32,
    ab: Option<(u32, u32)>,
    r: [u32; 3],
    qp: P,
    fi: [P; 3],
    np: P,
    mm: M3,
    g: M3,
    ps: [u32; 3],
    v0: [u32; 3],
    v1: [u32; 3],
    binv: M3,
    xp: [P; 3],
    xn: P,
    xd: P,
    h: P,
    gen: i32,
    pts_conic: u32,
    pts_over_a: u32,
}
impl Inst {
    fn special(&self, f: &F, x: u32) -> bool {
        pev(f, &self.qp, x) == 0 || self.r.contains(&x)
    }
    fn fwd(&self, f: &F, pt: [u32; 4]) -> Option<(u32, u32)> {
        let [x, y0, y1, y2] = pt;
        let qx = pev(f, &self.qp, x);
        if qx == 0 || self.r.contains(&x) || y0 == 0 || y1 == 0 || y2 == 0 {
            return None;
        }
        let u01 = f.div(f.mul(y0, y1), f.mul(qx, f.sub(x, self.r[1])));
        let u12 = f.div(f.mul(y1, y2), f.mul(qx, f.sub(x, self.r[2])));
        let u20 = f.div(f.mul(y2, y0), f.mul(qx, f.sub(x, self.r[0])));
        let xx = [f.mul(u12, u20), f.mul(u20, u01), f.mul(u01, u12)];
        let zz = mv(f, &self.mm, &xx);
        let co = mv(f, &self.binv, &zz);
        if co[2] == 0 {
            return None;
        }
        let z = f.div(co[1], co[2]);
        let w = f.div(
            f.mul(f.mul(f.mul(y0, y1), y2), pev(f, &self.xd, z)),
            f.mul(qx, pev(f, &self.np, x)),
        );
        Some((z, w))
    }
    fn xtrue(&self, f: &F, pt: [u32; 4]) -> [u32; 3] {
        let [x, y0, y1, y2] = pt;
        let qx = pev(f, &self.qp, x);
        let u01 = f.div(f.mul(y0, y1), f.mul(qx, f.sub(x, self.r[1])));
        let u12 = f.div(f.mul(y1, y2), f.mul(qx, f.sub(x, self.r[2])));
        let u20 = f.div(f.mul(y2, y0), f.mul(qx, f.sub(x, self.r[0])));
        [f.mul(u12, u20), f.mul(u20, u01), f.mul(u01, u12)]
    }
    // (z, w) -> (x, y0, y1, y2): the unique sign pattern matching w and the conic point
    fn bwd(&self, f: &F, z: u32, w: u32) -> Result<[u32; 4], &'static str> {
        let xdz = pev(f, &self.xd, z);
        if xdz == 0 {
            return Err("x=inf");
        }
        let x = f.div(pev(f, &self.xn, z), xdz);
        if self.special(f, x) {
            return Err("special x");
        }
        let mut s = [0u32; 3];
        for i in 0..3 {
            s[i] = match f.sqrt(pev(f, &self.fi[i], x)) {
                Some(v) => v,
                None => return Err("y not in L"),
            };
        }
        let target = f.div(
            f.mul(f.mul(w, pev(f, &self.qp, x)), pev(f, &self.np, x)),
            xdz,
        );
        let xz = [
            pev(f, &self.xp[0], z),
            pev(f, &self.xp[1], z),
            pev(f, &self.xp[2], z),
        ];
        let mut found = vec![];
        for e in 0..8u32 {
            let y: Vec<u32> = (0..3)
                .map(|i| if e >> i & 1 == 1 { f.neg(s[i]) } else { s[i] })
                .collect();
            if f.mul(f.mul(y[0], y[1]), y[2]) != target {
                continue;
            }
            let pt = [x, y[0], y[1], y[2]];
            if parallel(f, &self.xtrue(f, pt), &xz) {
                found.push(pt);
            }
        }
        if found.len() == 1 {
            Ok(found[0])
        } else {
            Err("sign match not unique")
        }
    }
}
fn frc(f: &F, p: [u32; 4]) -> [u32; 4] {
    [f.frob(p[0]), f.frob(p[3]), f.frob(p[1]), f.frob(p[2])]
}

fn build(
    f: &F,
    split: bool,
    c0: u32,
    c1: u32,
    ab: Option<(u32, u32)>,
    p: u32,
) -> Result<Inst, String> {
    let q = f.q;
    let r = [p, f.frob(p), f.frob(f.frob(p))];
    let qp: P = vec![c0, c1, 1];
    let fi = [
        pmul(f, &qp, &pmul(f, &plin(f, r[0]), &plin(f, r[1]))),
        pmul(f, &qp, &pmul(f, &plin(f, r[1]), &plin(f, r[2]))),
        pmul(f, &qp, &pmul(f, &plin(f, r[2]), &plin(f, r[0]))),
    ];
    let np = pmul(f, &plin(f, r[0]), &pmul(f, &plin(f, r[1]), &plin(f, r[2])));
    if !p_in(f, &np, 1) {
        return Err("N not over F_q".into());
    }
    let gam = (q * q * q + 1) + 1;
    let th = [1u32, gam, f.mul(gam, gam)];
    let mut mm = [[0u32; 3]; 3];
    for j in 0..3 {
        let mut t = th[j];
        for i in 0..3 {
            mm[j][i] = t;
            t = f.frob(t);
        }
    }
    let minv = inv3(f, &mm).ok_or("M singular")?;
    let c = [f.sub(r[2], r[0]), f.sub(r[0], r[1]), f.sub(r[1], r[2])];
    let mut g = [[0u32; 3]; 3];
    for j in 0..3 {
        for k in 0..3 {
            for i in 0..3 {
                g[j][k] = f.add(g[j][k], f.mul(f.mul(minv[i][j], c[i]), minv[i][k]));
            }
        }
    }
    if !(0..9).all(|t| f.in_sub(g[t / 3][t % 3], 1)) {
        return Err("P1 fail: conic not over F_q".into());
    }
    let xof = |zv: &[u32; 3]| -> Option<u32> {
        let xx = mv(f, &minv, zv);
        let (a0, a2) = (f.mul(xx[0], xx[0]), f.mul(xx[2], xx[2]));
        let d = f.sub(a0, a2);
        if d == 0 {
            None
        } else {
            Some(f.div(f.sub(f.mul(r[0], a0), f.mul(r[1], a2)), d))
        }
    };
    // F_q points of the conic, normalised, in enumeration order
    let mut cpts: Vec<[u32; 3]> = vec![];
    for t in 0..(q * q + q + 1) {
        let v = if t < q * q {
            [1, t % q, t / q]
        } else if t < q * q + q {
            [0, 1, t - q * q]
        } else {
            [0, 0, 1]
        };
        let v = [f.int(v[0] as i64), f.int(v[1] as i64), f.int(v[2] as i64)];
        if bil(f, &g, &v, &v) == 0 {
            cpts.push(v);
        }
    }
    let over_a: Vec<[u32; 3]> = match ab {
        Some((a, _)) => cpts.iter().filter(|z| xof(z) == Some(a)).cloned().collect(),
        None => vec![],
    };
    let ps = if split {
        if over_a.len() != 1 {
            return Err(format!("P1 fail: {} F_q points over a", over_a.len()));
        }
        over_a[0]
    } else {
        cpts[0]
    };
    let l = mv(f, &g, &ps);
    let dot = |a: &[u32; 3], b: &[u32; 3]| {
        f.add(
            f.add(f.mul(a[0], b[0]), f.mul(a[1], b[1])),
            f.mul(a[2], b[2]),
        )
    };
    let mut v0 = [0u32; 3];
    for t in 1..q * q * q {
        let d = digits(q, 3, t);
        let v = [f.int(d[0] as i64), f.int(d[1] as i64), f.int(d[2] as i64)];
        if dot(&l, &v) == 0 && !parallel(f, &v, &ps) {
            v0 = v;
            break;
        }
    }
    let mut v1 = [0u32; 3];
    for k in 0..3 {
        let mut v = [0u32; 3];
        v[k] = 1;
        if dot(&l, &v) != 0 {
            v1 = v;
            break;
        }
    }
    let (qv0, bv01, qv1, b1) = (
        bil(f, &g, &v0, &v0),
        bil(f, &g, &v0, &v1),
        bil(f, &g, &v1, &v1),
        bil(f, &g, &ps, &v1),
    );
    let two = f.int(2);
    let mut zp: [P; 3] = [vec![], vec![], vec![]];
    for j in 0..3 {
        zp[j] = tr(vec![
            f.sub(f.mul(qv1, ps[j]), f.mul(f.mul(two, b1), v1[j])),
            f.sub(f.mul(f.mul(two, bv01), ps[j]), f.mul(f.mul(two, b1), v0[j])),
            f.mul(qv0, ps[j]),
        ]);
    }
    let mut xp: [P; 3] = [vec![], vec![], vec![]];
    for i in 0..3 {
        for j in 0..3 {
            xp[i] = padd(f, &xp[i], &psc(f, &zp[j], minv[i][j]));
        }
    }
    let (s0, s2) = (pmul(f, &xp[0], &xp[0]), pmul(f, &xp[2], &xp[2]));
    let num = psub(f, &psc(f, &s0, r[0]), &psc(f, &s2, r[1]));
    let den = psub(f, &s0, &s2);
    let gg = pgcd(f, &num, &den);
    let (mut xn, mut xd) = (pexact(f, &num, &gg), pexact(f, &den, &gg));
    let li = f.inv(*xd.last().unwrap());
    xn = psc(f, &xn, li);
    xd = psc(f, &xd, li);
    let h = padd(
        f,
        &padd(f, &pmul(f, &xn, &xn), &psc(f, &pmul(f, &xn, &xd), c1)),
        &psc(f, &pmul(f, &xd, &xd), c0),
    );
    let bm = [
        [ps[0], v0[0], v1[0]],
        [ps[1], v0[1], v1[1]],
        [ps[2], v0[2], v1[2]],
    ];
    let binv = inv3(f, &bm).ok_or("basis singular")?;
    let pts_conic = cpts.len() as u32;
    let pts_over_a = over_a.len() as u32;
    Ok(Inst {
        split,
        c0,
        c1,
        ab,
        r,
        qp,
        fi,
        np,
        mm,
        g,
        ps,
        v0,
        v1,
        binv,
        xp,
        xn,
        xd,
        h,
        gen: 3,
        pts_conic,
        pts_over_a,
    })
}

// ---------------------------------------------------------------- helpers for output
struct Iso {
    cen2l: Vec<u32>,
    l2cen: HashMap<u32, u32>,
}
fn kc(iso: &Iso, q: u32, v: u32) -> String {
    let i = iso.l2cen[&v];
    let d = digits(q, 3, i);
    format!("[{},{},{}]", d[0], d[1], d[2])
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
fn cof(n: u64) -> Option<(u64, u64)> {
    for h in [4u64, 8] {
        if n % h == 0 && is_prime(n / h) && n / h >= 5 {
            return Some((h, n / h));
        }
    }
    None
}

// ============================================================================
// New driver (W3b T8): enumerate the split-arm instances in the
// construction's own order, test Props. 2 and 4 on each descended `H`, and
// where they pass run the planted ECDLP through `psi` to the quartic.

use super::smith_isogeny::{smith_data_with_subgroups, Hyp, Mum, Smith, W1Bridge};
use super::smith_prop24::SplitMix64;

/// One instance of the split arm.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RouteSRecord {
    pub q: u32,
    pub instance: usize,
    pub q_roots: [u32; 2],
    pub p_census_idx: u32,
    pub order_e0: u64,
    /// Largest prime factor of `#E0(K)` (protocol filter: `>= 5`).
    pub ell: u64,
    pub cofactor: u64,
    /// The old prototype's quota rule `#E0 = 4 ell` or `8 ell` (recorded only).
    pub old_cofactor_rule: bool,
    pub build_error: Option<String>,
    /// `H: w^2 = h(z)` made monic (`Z = c z`, `W = c^3 w`), low -> high.
    pub h_monic: Vec<u64>,
    pub h_ok: bool,
    pub prop2_maps: usize,
    pub rational_maps: usize,
    pub prop4_pass: bool,
    pub dlp_attempted: bool,
    pub skip_reason: Option<String>,
    pub transfer_hom_ok: bool,
    pub transfer_order_ok: bool,
    pub quartic_t1_pass: bool,
    pub n_jac: i128,
    /// `#J_H(F_q) = #E0(F_{q^3})` (`L(T) = 1 - t T^3 + q^3 T^6`).
    pub n_jac_equals_order_e0: bool,
    pub psi_base_retries: u32,
    pub psi_g_nonzero: bool,
    pub psi_g_order_ell: bool,
    pub k_planted: u64,
    pub k_found: Option<u64>,
    pub verified_on_e: bool,
    pub dlp_steps: u64,
    pub seconds_transfer: f64,
    pub seconds_quartic: f64,
    pub seconds_psi: f64,
    pub seconds_dlp: f64,
    pub seconds_total: f64,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RouteSRun {
    pub q: u32,
    pub split_candidates: u64,
    /// Candidates whose `#E0(K)` has a prime factor `>= 5` (protocol filter).
    pub filter_instances: u64,
    /// Of those, how many meet the old `4 ell / 8 ell` rule.
    pub old_rule_instances: u64,
    /// Instances not processed because the time cap was hit.
    pub not_reached_time_cap: u64,
    pub records: Vec<RouteSRecord>,
}

fn to_ints(f: &F, a: &P) -> Vec<u64> {
    a.iter().map(|&c| f.to_int(c) as u64).collect()
}

fn mpow(b: u64, mut e: u64, p: u64) -> u64 {
    let (mut r, mut b) = (1u64, b % p);
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut lp = 1;
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            lp = d;
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        lp = n;
    }
    lp
}

/// `(u(z), v(z))` on `w^2 = h` to the monic model `W^2 = H1(Z)`, `Z = c z`.
fn monic_div(u: &[u64], v: &[u64], c: u64, p: u64) -> Mum {
    let d = u.len() as u64 - 1;
    let mut uu: Vec<u64> = u
        .iter()
        .enumerate()
        .map(|(i, &x)| x * mpow(c, d - i as u64, p) % p)
        .collect();
    let li = mpow(*uu.last().unwrap(), p - 2, p);
    for x in uu.iter_mut() {
        *x = *x * li % p;
    }
    let ci = mpow(c, p - 2, p);
    let mut vv: Vec<u64> = v
        .iter()
        .enumerate()
        .map(|(i, &x)| x * mpow(c, 3, p) % p * mpow(ci, i as u64, p) % p)
        .collect();
    while vv.last() == Some(&0) {
        vv.pop();
    }
    Mum { u: uu, v: vv }
}

/// Route S at one `q`: every split-arm instance whose `#E0(K)` has a prime
/// factor `ell >= 5`, in the construction's order, up to `max_instances` and
/// `time_cap_s`; Props. 2 and 4 on each; the DLP on at most `max_dlp` of
/// those passing Prop. 4.
pub fn route_s(
    q: u32,
    max_instances: usize,
    max_dlp: usize,
    time_cap_s: f64,
    seed: u64,
) -> RouteSRun {
    let t_run = std::time::Instant::now();
    let f = F::build_l(q, 6);
    let cf = F::build_census(q, 3);
    let q3 = q * q * q;
    let md = &cf.modulus;
    let kel = f.elems(3);
    let beta = *kel
        .iter()
        .find(|&&b| {
            b != 0 && {
                let b2 = f.mul(b, b);
                f.add(
                    f.add(
                        f.add(f.mul(b2, b), f.mul(f.int(md[2] as i64), b2)),
                        f.mul(f.int(md[1] as i64), b),
                    ),
                    f.int(md[0] as i64),
                ) == 0
            }
        })
        .expect("no root");
    let mut cen2l = vec![0u32; q3 as usize];
    let mut l2cen = HashMap::new();
    for i in 0..q3 {
        let d = digits(q, 3, i);
        let v = f.add(
            f.add(f.int(d[0] as i64), f.mul(f.int(d[1] as i64), beta)),
            f.mul(f.int(d[2] as i64), f.mul(beta, beta)),
        );
        cen2l[i as usize] = v;
        l2cen.insert(v, i);
    }
    let mut qs: Vec<(u32, u32, (u32, u32), [u32; 2])> = vec![];
    for a in 0..q {
        for b in a + 1..q {
            qs.push((
                f.int((a * b) as i64),
                f.int(-((a + b) as i64)),
                (f.int(a as i64), f.int(b as i64)),
                [a, b],
            ));
        }
    }
    let chiq: Vec<Vec<i8>> = qs
        .iter()
        .map(|(c0, c1, _, _)| {
            kel.iter()
                .map(|&x| f.chi(f.add(f.add(f.mul(x, x), f.mul(*c1, x)), *c0), 3) as i8)
                .collect()
        })
        .collect();
    let mut run = RouteSRun {
        q,
        ..Default::default()
    };
    let mut sel: Vec<(usize, u32, u64)> = vec![];
    for i in 0..q3 {
        let v = cen2l[i as usize];
        if f.in_sub(v, 1) {
            continue;
        }
        let (v1, v2) = (f.frob(v), f.frob(f.frob(v)));
        if l2cen[&v1] < i || l2cen[&v2] < i {
            continue;
        }
        let chip: Vec<i8> = kel
            .iter()
            .map(|&x| f.chi(f.mul(f.sub(x, v), f.sub(x, v1)), 3) as i8)
            .collect();
        for k in 0..qs.len() {
            let s: i64 = chiq[k]
                .iter()
                .zip(chip.iter())
                .map(|(&a, &b)| (a * b) as i64)
                .sum();
            let n = (q3 as i64 + 2 + s) as u64;
            run.split_candidates += 1;
            if largest_prime_factor(n) >= 5 {
                run.filter_instances += 1;
                if cof(n).is_some() {
                    run.old_rule_instances += 1;
                }
                sel.push((k, i, n));
            }
        }
    }
    let mut n_dlp = 0;
    for (inst_no, &(k, pidx, n)) in sel.iter().enumerate() {
        if inst_no >= max_instances || t_run.elapsed().as_secs_f64() > time_cap_s {
            run.not_reached_time_cap = (sel.len() - inst_no) as u64;
            break;
        }
        let t_start = std::time::Instant::now();
        let (c0, c1, ab, roots) = qs[k];
        let p = cen2l[pidx as usize];
        let ll = largest_prime_factor(n);
        let hh = n / ll;
        let mut rec = RouteSRecord {
            q,
            instance: inst_no,
            q_roots: roots,
            p_census_idx: pidx,
            order_e0: n,
            ell: ll,
            cofactor: hh,
            old_cofactor_rule: cof(n).is_some(),
            ..Default::default()
        };
        let inst = match build(&f, true, c0, c1, Some(ab), p) {
            Ok(x) => x,
            Err(e) => {
                rec.build_error = Some(e);
                run.records.push(rec);
                continue;
            }
        };
        let hder = pder(&f, &inst.h);
        let sqf = !hder.is_empty() && dg(&pgcd(&f, &inst.h, &hder)) == 0;
        rec.h_ok = p_in(&f, &inst.h, 1) && dg(&inst.h) == 7 && sqf;
        if !rec.h_ok {
            rec.skip_reason = Some("descended h not a squarefree degree-7 F_q polynomial".into());
            run.records.push(rec);
            continue;
        }
        let pq = q as u64;
        let hz = to_ints(&f, &inst.h);
        let c = hz[7];
        let h1: Vec<u64> = hz
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                if i == 7 {
                    1
                } else {
                    x * mpow(c, 6 - i as u64, pq) % pq
                }
            })
            .collect();
        rec.h_monic = h1.clone();
        let mut rng = SplitMix64(seed ^ ((q as u64) << 32 | inst_no as u64));
        let sds = smith_data_with_subgroups(&h1, pq, &mut rng);
        rec.prop2_maps = sds.len();
        rec.rational_maps = sds.iter().filter(|(s, _)| s.rational()).count();
        rec.prop4_pass = rec.rational_maps > 0;
        if !rec.prop4_pass || n_dlp >= max_dlp {
            if rec.prop4_pass {
                rec.skip_reason = Some("DLP cap reached".into());
            }
            rec.seconds_total = t_start.elapsed().as_secs_f64();
            run.records.push(rec);
            continue;
        }
        n_dlp += 1;
        rec.dlp_attempted = true;
        // ---- E0 in Weierstrass form at r0 (as the prototype's P5/P4)
        let mut tay = inst.fi[0].clone();
        let mut ec = [0u32; 5];
        for j in 0..5 {
            let (qq, rr) = pdivrem(&f, &tay, &plin(&f, inst.r[0]));
            ec[j] = *rr.first().unwrap_or(&0);
            tay = qq;
        }
        let (te1, te2, te3) = (ec[1], ec[2], ec[3]);
        let wc = Wc {
            a2: te2,
            a4: f.mul(te1, te3),
            a6: f.mul(te1, te1),
        };
        let r = inst.r;
        let h = &inst.h;
        let gg = inst.gen;
        let to_e = |pp: WP| -> Option<(u32, u32)> {
            match pp {
                WP::O => None,
                WP::A(s, y) => {
                    if s == 0 {
                        None
                    } else {
                        Some((
                            f.add(r[0], f.div(te1, s)),
                            f.div(f.mul(y, te1), f.mul(s, s)),
                        ))
                    }
                }
            }
        };
        let elig = |pp: WP| -> bool {
            match to_e(pp) {
                None => false,
                Some((x, _)) => !inst.special(&f, x),
            }
        };
        let phi = |pp: WP| -> Dv {
            let (x, y0) = to_e(pp).unwrap();
            let s1 = f.sqrt(pev(&f, &inst.fi[1], x)).unwrap();
            let s2 = f.sqrt(pev(&f, &inst.fi[2], x)).unwrap();
            let mut d = dzero();
            for e in 0..4 {
                let mut pt = [
                    x,
                    y0,
                    if e & 1 == 1 { f.neg(s1) } else { s1 },
                    if e & 2 == 2 { f.neg(s2) } else { s2 },
                ];
                for _ in 0..3 {
                    let (z, w) = inst.fwd(&f, pt).expect("generic point maps");
                    d = dadd(&f, h, gg, &d, &dpt(&f, z, w));
                    pt = frc(&f, pt);
                }
            }
            d
        };
        let wpts: Vec<WP> = {
            let mut v = vec![WP::O];
            for &s in &kel {
                let rh = wrhs(&f, &wc, s);
                if rh == 0 {
                    v.push(WP::A(s, 0));
                } else if f.chi(rh, 3) == 1 {
                    let y = f.sqrt(rh).unwrap();
                    v.push(WP::A(s, y));
                    v.push(WP::A(s, f.neg(y)));
                }
            }
            v
        };
        if wpts.len() as u64 != n {
            rec.skip_reason = Some(format!("#E0(K) = {} != {}", wpts.len(), n));
            run.records.push(rec);
            continue;
        }
        let mut wr = Rng::new((q as u64) << 40 | inst_no as u64);
        let mut phi_any = |pp: WP| -> Dv {
            if pp == WP::O {
                return dzero();
            }
            if elig(pp) {
                return phi(pp);
            }
            loop {
                let rr = wpts[wr.below(wpts.len() as u64) as usize];
                let s = wadd(&f, &wc, pp, rr);
                if elig(rr) && elig(s) {
                    return dadd(&f, h, gg, &phi(s), &dneg(&f, &phi(rr)));
                }
            }
        };
        // planted ECDLP: G of order ell, Q = k G
        let g = loop {
            let pt = wpts[rng.below(wpts.len() as u64) as usize];
            let g = wmul(&f, &wc, hh, pt);
            if g != WP::O {
                break g;
            }
        };
        assert!(wmul(&f, &wc, ll, g) == WP::O, "G of order ell");
        let kpl = 1 + rng.below(ll - 1);
        rec.k_planted = kpl;
        let qp = wmul(&f, &wc, kpl, g);
        let t0 = std::time::Instant::now();
        let dg_ = phi_any(g);
        let dq = phi_any(qp);
        rec.transfer_hom_ok = dmul(&f, h, gg, kpl, &dg_) == dq && dv_in(&f, &dg_) && dv_in(&f, &dq);
        rec.transfer_order_ok = dg_ != dzero() && dmul(&f, h, gg, ll, &dg_) == dzero();
        rec.seconds_transfer = t0.elapsed().as_secs_f64();
        if !rec.transfer_order_ok {
            rec.skip_reason = Some("phi(G) zero or not of order ell".into());
            run.records.push(rec);
            continue;
        }
        // ---- to the monic model, then Smith's psi
        let mg = monic_div(&to_ints(&f, &dg_.u), &to_ints(&f, &dg_.v), c, pq);
        let mq = monic_div(&to_ints(&f, &dq.u), &to_ints(&f, &dq.v), c, pq);
        let hy = Hyp::new(&h1, pq);
        assert!(hy.is_valid(&mg) && hy.is_valid(&mq), "monic transform");
        let t1 = std::time::Instant::now();
        let (sd, _sg) = sds.into_iter().find(|(s, _)| s.rational()).unwrap();
        let sm = Smith::new(sd);
        let (cr, quartic) = sm.t1_curve(&mut rng);
        rec.quartic_t1_pass = cr.pass();
        rec.n_jac = cr.n_jac;
        rec.n_jac_equals_order_e0 = cr.n_jac == n as i128;
        rec.seconds_quartic = t1.elapsed().as_secs_f64();
        let Some(quartic) = quartic else {
            rec.skip_reason = Some(format!("quartic fit: {:?}", cr.fit_error));
            run.records.push(rec);
            continue;
        };
        let w1 = match W1Bridge::new(&quartic) {
            Ok(w) => w,
            Err(e) => {
                rec.skip_reason = Some(e);
                run.records.push(rec);
                continue;
            }
        };
        let t1b = std::time::Instant::now();
        // The class does not depend on the base divisor; an image the bridge
        // cannot reduce (a point of multiplicity > 2) is retried with fresh
        // bases (counted in `psi_base_retries`).
        let mut got = None;
        let mut last_err = String::new();
        for attempt in 0..8 {
            let base = sm.random_base(&mut rng);
            let (ig, _) = sm.psi_class_fixed(&mg, &base, &mut rng);
            let (iq, _) = sm.psi_class_fixed(&mq, &base, &mut rng);
            match (w1.class_psi(&ig), w1.class_psi(&iq)) {
                (Ok(a), Ok(b)) => {
                    got = Some((a, b));
                    rec.psi_base_retries = attempt;
                    break;
                }
                (Err(e), _) | (_, Err(e)) => last_err = e,
            }
        }
        let Some((pg, pqc)) = got else {
            rec.skip_reason = Some(last_err);
            run.records.push(rec);
            continue;
        };
        rec.psi_g_nonzero = !w1.c.is_zero(&pg);
        rec.psi_g_order_ell = w1.c.is_zero(&w1.c.mul_scalar(&pg, ll as u128));
        rec.seconds_psi = t1b.elapsed().as_secs_f64();
        // ---- DLP in J_X(F_q) on canonical keys (brute force over <Psi(G)>)
        let t2 = std::time::Instant::now();
        let target = pqc.key();
        let mut cur = w1.c.zero();
        for j in 0..ll {
            rec.dlp_steps += 1;
            if cur.key() == target {
                rec.k_found = Some(j);
                break;
            }
            cur = w1.c.add(&cur, &pg);
        }
        rec.seconds_dlp = t2.elapsed().as_secs_f64();
        if let Some(kf) = rec.k_found {
            rec.verified_on_e = wmul(&f, &wc, kf, g) == qp;
        }
        rec.seconds_total = t_start.elapsed().as_secs_f64();
        run.records.push(rec);
    }
    run
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Named so that `cargo test --lib smith_isogeny` runs it.
    #[test]
    fn smith_isogeny_route_s_q7() {
        let run = route_s(7, 400, 2, 600.0, 20261018);
        let mut ok = 0;
        for r in run.records.iter().filter(|r| r.dlp_attempted) {
            println!("{:?}", r);
            assert!(r.transfer_hom_ok && r.transfer_order_ok);
            if r.quartic_t1_pass {
                assert!(r.n_jac_equals_order_e0);
                assert!(r.psi_g_nonzero && r.psi_g_order_ell);
                assert!(r.verified_on_e, "recovered k must verify on E");
                ok += 1;
            }
        }
        let pass = run.records.iter().filter(|r| r.prop4_pass).count();
        println!(
            "q = 7: {} candidates, {} filter, {} run, {} pass Prop. 4, {} solved",
            run.split_candidates,
            run.filter_instances,
            run.records.len(),
            pass,
            ok
        );
        assert!(ok >= 1, "at least one end-to-end solve at q = 7");
    }
}
