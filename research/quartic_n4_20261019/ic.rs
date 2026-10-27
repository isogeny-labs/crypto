// Relation-collection index calculus on the GHS plane quartic (lead N4).
// No subgroup enumeration: every discrete log comes from a kernel vector of a relation
// matrix mod ell and a descent of the target classes through the canonical Pic^0 arithmetic.
#![allow(dead_code)]
use crate::common::*;
use crate::ghs::*;
use crate::pic;
use std::collections::HashMap;

// ---------------------------------------------------------------- polynomials over F_q
type Poly = Vec<u64>;
fn ptrim(a: &mut Poly) {
    while let Some(&0) = a.last() {
        a.pop();
    }
}
fn pmonic(f: &Fp, a: &mut Poly) {
    ptrim(a);
    if let Some(&l) = a.last() {
        if l != 1 {
            let il = f.inv(l);
            for c in a.iter_mut() {
                *c = f.mul(*c, il);
            }
        }
    }
}
/// remainder of a mod m (m monic)
fn prem(f: &Fp, a: &Poly, m: &Poly) -> Poly {
    let mut r = a.clone();
    ptrim(&mut r);
    let dm = m.len() - 1;
    while r.len() > dm {
        let lead = *r.last().unwrap();
        let sh = r.len() - 1 - dm;
        if lead != 0 {
            for i in 0..m.len() {
                let t = f.mul(lead, m[i]);
                r[sh + i] = f.sub(r[sh + i], t);
            }
        }
        r.pop();
        ptrim(&mut r);
    }
    r
}
fn pmul(f: &Fp, a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            let t = f.mul(a[i], b[j]);
            r[i + j] = f.add(r[i + j], t);
        }
    }
    r
}
fn pgcd(f: &Fp, a: &Poly, b: &Poly) -> Poly {
    let (mut a, mut b) = (a.clone(), b.clone());
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        let mut bm = b.clone();
        pmonic(f, &mut bm);
        let r = prem(f, &a, &bm);
        a = bm;
        b = r;
    }
    pmonic(f, &mut a);
    a
}
fn pdiv_exact(f: &Fp, a: &Poly, d: &Poly) -> Poly {
    // a / d (d monic, exact)
    let mut r = a.clone();
    let dd = d.len() - 1;
    let mut qt = vec![0u64; r.len() - dd];
    for i in (0..qt.len()).rev() {
        let lead = r[i + dd];
        qt[i] = lead;
        if lead != 0 {
            for j in 0..d.len() {
                let t = f.mul(lead, d[j]);
                r[i + j] = f.sub(r[i + j], t);
            }
        }
    }
    qt
}
fn ppowx(f: &Fp, base: &Poly, mut e: u64, m: &Poly) -> Poly {
    let mut r: Poly = vec![1];
    let mut b = prem(f, base, m);
    while e > 0 {
        if e & 1 == 1 {
            r = prem(f, &pmul(f, &r, &b), m);
        }
        e >>= 1;
        if e > 0 {
            b = prem(f, &pmul(f, &b, &b), m);
        }
    }
    r
}
/// all roots in F_q of g (degree <= 4), distinct
pub fn roots(f: &Fp, g: &Poly, rng: &mut Rng) -> Vec<u64> {
    let mut g = g.clone();
    ptrim(&mut g);
    if g.len() <= 1 {
        return vec![];
    }
    pmonic(f, &mut g);
    if g.len() == 2 {
        return vec![f.neg(g[0])];
    }
    // h = x^q mod g ; d = gcd(g, h - x)
    let h = ppowx(f, &vec![0, 1], f.q, &g);
    let mut hx = h.clone();
    if hx.len() < 2 {
        hx.resize(2, 0);
    }
    hx[1] = f.sub(hx[1], 1);
    ptrim(&mut hx);
    let d = pgcd(f, &g, &hx);
    split(f, &d, rng)
}
fn split(f: &Fp, d: &Poly, rng: &mut Rng) -> Vec<u64> {
    let deg = d.len() - 1;
    if deg == 0 {
        return vec![];
    }
    if deg == 1 {
        return vec![f.neg(d[0])];
    }
    if deg == 2 {
        let (b, c) = (d[1], d[0]);
        let disc = f.sub(f.mul(b, b), f.mul(4, c));
        let s = match f.sqrt(disc) {
            Some(s) => s,
            None => return vec![],
        };
        let i2 = f.inv(2);
        let r1 = f.mul(f.sub(s, b), i2);
        let r2 = f.mul(f.sub(f.neg(s), b), i2);
        return if r1 == r2 { vec![r1] } else { vec![r1, r2] };
    }
    loop {
        let delta = rng.below(f.q);
        let e = ppowx(f, &vec![delta, 1], (f.q - 1) / 2, d);
        let mut e1 = e.clone();
        if e1.is_empty() {
            e1.push(0);
        }
        e1[0] = f.sub(e1[0], 1);
        ptrim(&mut e1);
        let g1 = pgcd(f, d, &e1);
        let dg = g1.len() - 1;
        if dg > 0 && dg < deg {
            let g2 = pdiv_exact(f, d, &g1);
            let mut r = split(f, &g1, rng);
            r.extend(split(f, &g2, rng));
            return r;
        }
    }
}

// ---------------------------------------------------------------- points and lines
pub type P3 = [u64; 3];
pub fn pack(p: &P3) -> u64 {
    (p[0] << 42) | (p[1] << 21) | p[2]
}
pub fn pnorm(f: &Fp, p: &P3) -> P3 {
    let l = *p.iter().find(|x| **x != 0).expect("zero point");
    if l == 1 {
        return *p;
    }
    let il = f.inv(l);
    [f.mul(p[0], il), f.mul(p[1], il), f.mul(p[2], il)]
}

pub struct Curve {
    pub f: Fp,
    pub coef: [u64; 15],
    pub mons: Vec<(u8, u8, u8)>,
}
impl Curve {
    pub fn new(q: u64, f: &[u32; 15]) -> Curve {
        let mut coef = [0u64; 15];
        for i in 0..15 {
            coef[i] = f[i] as u64;
        }
        Curve { f: Fp { q }, coef, mons: pic::mons_of_degree(4) }
    }
    pub fn eval(&self, p: &P3) -> u64 {
        let f = &self.f;
        let mut pw = [[1u64; 5]; 3];
        for i in 0..3 {
            for e in 1..5 {
                pw[i][e] = f.mul(pw[i][e - 1], p[i]);
            }
        }
        let mut s = 0u64;
        for (i, &(a, b, c)) in self.mons.iter().enumerate() {
            if self.coef[i] == 0 {
                continue;
            }
            let t = f.mul(f.mul(pw[0][a as usize], pw[1][b as usize]), pw[2][c as usize]);
            s = f.add(s, f.mul(self.coef[i], t));
        }
        s
    }
    /// g(s) = F(s A + B), coefficients low -> high (degree <= 4)
    pub fn restrict(&self, a: &P3, b: &P3) -> Poly {
        let f = &self.f;
        // L_i(s) = A_i s + B_i ; powers
        let mut lp: Vec<Vec<Poly>> = Vec::new();
        for i in 0..3 {
            let l: Poly = vec![b[i], a[i]];
            let mut pws: Vec<Poly> = vec![vec![1], l.clone()];
            for e in 2..5 {
                let nx = pmul(f, &pws[e - 1], &l);
                pws.push(nx);
            }
            lp.push(pws);
        }
        let mut g = vec![0u64; 5];
        for (i, &(x, y, z)) in self.mons.iter().enumerate() {
            if self.coef[i] == 0 {
                continue;
            }
            let t = pmul(f, &pmul(f, &lp[0][x as usize], &lp[1][y as usize]), &lp[2][z as usize]);
            for (k, c) in t.iter().enumerate() {
                let m = f.mul(self.coef[i], *c);
                g[k] = f.add(g[k], m);
            }
        }
        g
    }
    /// all rational points of C on the line through A, B (distinct)
    pub fn line_points(&self, a: &P3, b: &P3, rng: &mut Rng) -> Vec<P3> {
        let f = &self.f;
        let g = self.restrict(a, b);
        let mut out = Vec::new();
        if g[4] == 0 {
            out.push(pnorm(f, a));
        }
        for s in roots(f, &g, rng) {
            let p = [
                f.add(f.mul(s, a[0]), b[0]),
                f.add(f.mul(s, a[1]), b[1]),
                f.add(f.mul(s, a[2]), b[2]),
            ];
            out.push(pnorm(f, &p));
        }
        out
    }
}

pub fn cross(f: &Fp, a: &P3, b: &P3) -> P3 {
    [
        f.sub(f.mul(a[1], b[2]), f.mul(a[2], b[1])),
        f.sub(f.mul(a[2], b[0]), f.mul(a[0], b[2])),
        f.sub(f.mul(a[0], b[1]), f.mul(a[1], b[0])),
    ]
}

// ---------------------------------------------------------------- relations
#[derive(Clone)]
pub struct Rel {
    pub c: Vec<(u32, u8)>, // (fb index, multiplicity) of the FB points
    pub lp: Option<u32>,   // large prime id
}

pub struct RelOut {
    pub full: Vec<Rel>,
    pub lp: Vec<Rel>,
    pub lp_pts: Vec<P3>,
    pub pairs: u64,
    pub multi_lines: u64,
    pub unsure: u64,
}

pub fn find_relations(cv: &Curve, fb: &[P3], rng: &mut Rng) -> RelOut {
    let f = &cv.f;
    let n = fb.len();
    let idx: HashMap<u64, u32> = fb.iter().enumerate().map(|(i, p)| (pack(p), i as u32)).collect();
    bump_hash(n as u64);
    let mut first: HashMap<u64, (u32, u32)> = HashMap::with_capacity(n * n / 2 + 16);
    let mut multi: HashMap<u64, Vec<u32>> = HashMap::new();
    let mut pairs = 0u64;
    for i in 0..n {
        // lines through fb[i] and fb[j], j>i, normalised by batch inversion of the pivot
        let m = n - i - 1;
        if m == 0 {
            break;
        }
        let mut ls: Vec<P3> = Vec::with_capacity(m);
        let mut piv: Vec<u64> = Vec::with_capacity(m);
        for j in i + 1..n {
            let l = cross(f, &fb[i], &fb[j]);
            let pv = *l.iter().find(|x| **x != 0).unwrap_or(&1);
            piv.push(pv);
            ls.push(l);
        }
        // Montgomery batch inversion
        let mut pref = Vec::with_capacity(m);
        let mut acc = 1u64;
        for &p in &piv {
            pref.push(acc);
            acc = f.mul(acc, p);
        }
        let mut inv_acc = f.inv(acc);
        let mut invs = vec![0u64; m];
        for k in (0..m).rev() {
            invs[k] = f.mul(inv_acc, pref[k]);
            inv_acc = f.mul(inv_acc, piv[k]);
        }
        for (k, l) in ls.iter().enumerate() {
            let key = pack(&[f.mul(l[0], invs[k]), f.mul(l[1], invs[k]), f.mul(l[2], invs[k])]);
            let j = (i + 1 + k) as u32;
            pairs += 1;
            bump_hash(1);
            match first.get(&key) {
                None => {
                    first.insert(key, (i as u32, j));
                }
                Some(&(a, b)) => {
                    let e = multi.entry(key).or_insert_with(|| vec![a, b]);
                    e.push(i as u32);
                    e.push(j);
                }
            }
        }
    }
    drop(first);
    let mut out = RelOut {
        full: vec![],
        lp: vec![],
        lp_pts: vec![],
        pairs,
        multi_lines: multi.len() as u64,
        unsure: 0,
    };
    let mut lp_ids: HashMap<u64, u32> = HashMap::new();
    for (_key, mut pts) in multi {
        pts.sort_unstable();
        pts.dedup();
        if pts.len() >= 4 {
            if pts.len() > 4 {
                out.unsure += 1;
                continue;
            }
            out.full.push(Rel { c: pts.iter().map(|&i| (i, 1u8)).collect(), lp: None });
            continue;
        }
        // three distinct FB points A, B, C on the line: the fourth intersection
        let (ia, ib, ic) = (pts[0], pts[1], pts[2]);
        let (a, b, c) = (fb[ia as usize], fb[ib as usize], fb[ic as usize]);
        let g = cv.restrict(&a, &b);
        // g(s)=F(sA+B): roots: s=inf (A), s=0 (B), and the quadratic g3 s^2+g2 s+g1 (C, P4)
        if g[0] != 0 || g[4] != 0 {
            out.unsure += 1;
            continue;
        }
        // parameter of C = s A + t B: use a nonzero minor of A x B
        let ab = cross(f, &a, &b);
        let k = (0..3).find(|&k| ab[k] != 0).unwrap();
        let cb = cross(f, &c, &b);
        let ac = cross(f, &a, &c);
        let ik = f.inv(ab[k]);
        let sc = f.mul(cb[k], ik);
        let tc = f.mul(ac[k], ik);
        if tc == 0 {
            out.unsure += 1;
            continue;
        }
        let rho_c = f.mul(sc, f.inv(tc));
        // fourth point
        let mut mult = [1u8, 1, 1];
        let mut lp_pt: Option<P3> = None;
        if g[3] == 0 {
            mult[0] = 2; // quadratic degenerates: its second root is s = infinity, i.e. A
        } else {
            let sum = f.neg(f.mul(g[2], f.inv(g[3])));
            let rho4 = f.sub(sum, rho_c);
            let p4 = [
                f.add(f.mul(rho4, a[0]), b[0]),
                f.add(f.mul(rho4, a[1]), b[1]),
                f.add(f.mul(rho4, a[2]), b[2]),
            ];
            let p4 = pnorm(f, &p4);
            if p4 == a {
                mult[0] = 2;
            } else if p4 == b {
                mult[1] = 2;
            } else if p4 == c {
                mult[2] = 2;
            } else {
                bump_hash(1);
                if idx.contains_key(&pack(&p4)) {
                    out.unsure += 1;
                    continue;
                }
                lp_pt = Some(p4);
            }
        }
        let cs = vec![(ia, mult[0]), (ib, mult[1]), (ic, mult[2])];
        match lp_pt {
            None => {
                // 4 FB points with multiplicity
                let tot: u8 = cs.iter().map(|x| x.1).sum();
                if tot == 4 {
                    out.full.push(Rel { c: cs, lp: None });
                } else {
                    out.unsure += 1;
                }
            }
            Some(p4) => {
                let id = *lp_ids.entry(pack(&p4)).or_insert_with(|| {
                    out.lp_pts.push(p4);
                    (out.lp_pts.len() - 1) as u32
                });
                out.lp.push(Rel { c: cs, lp: Some(id) });
            }
        }
    }
    out
}

// ---------------------------------------------------------------- mod-ell linear algebra
#[inline(always)]
pub fn lm(a: u64, b: u64, ell: u64) -> u64 {
    bump_lmul(1);
    ((a as u128 * b as u128) % ell as u128) as u64
}
#[inline(always)]
pub fn la(a: u64, b: u64, ell: u64) -> u64 {
    bump_ladd(1);
    let s = a + b;
    if s >= ell {
        s - ell
    } else {
        s
    }
}
#[inline(always)]
pub fn ls(a: u64, b: u64, ell: u64) -> u64 {
    bump_ladd(1);
    if a >= b {
        a - b
    } else {
        a + ell - b
    }
}

pub type Row = Vec<(u32, u64)>;

fn matvec(rows: &[Row], d: &[u64], v: &[u64], ell: u64, out: &mut Vec<u64>, tmp: &mut Vec<u64>) {
    // out = A^T D A v
    for (i, r) in rows.iter().enumerate() {
        let mut s = 0u64;
        for &(c, a) in r {
            s = la(s, lm(a, v[c as usize], ell), ell);
        }
        tmp[i] = lm(s, d[i], ell);
    }
    for o in out.iter_mut() {
        *o = 0;
    }
    for (i, r) in rows.iter().enumerate() {
        let t = tmp[i];
        if t == 0 {
            continue;
        }
        for &(c, a) in r {
            out[c as usize] = la(out[c as usize], lm(a, t, ell), ell);
        }
    }
}
fn dot(a: &[u64], b: &[u64], ell: u64) -> u64 {
    let mut s = 0u64;
    for i in 0..a.len() {
        s = la(s, lm(a[i], b[i], ell), ell);
    }
    s
}

/// One kernel vector of A (rows over n columns) via Lanczos on A^T D A. None on breakdown.
pub fn lanczos_kernel(rows: &[Row], n: usize, ell: u64, rng: &mut Rng) -> Option<Vec<u64>> {
    let d: Vec<u64> = (0..rows.len()).map(|_| 1 + rng.below(ell - 1)).collect();
    let b: Vec<u64> = (0..n).map(|_| rng.below(ell)).collect();
    let mut tmp = vec![0u64; rows.len()];
    let mut y = vec![0u64; n];
    matvec(rows, &d, &b, ell, &mut y, &mut tmp);
    // solve M x = y
    let mut x = vec![0u64; n];
    let mut w = y.clone();
    let mut w_prev: Vec<u64> = vec![0; n];
    let mut v = vec![0u64; n];
    let mut v_prev: Vec<u64> = vec![0; n];
    let mut s_prev = 0u64;
    let mut first = true;
    let mut iters = 0;
    loop {
        if w.iter().all(|&e| e == 0) {
            break;
        }
        matvec(rows, &d, &w, ell, &mut v, &mut tmp);
        let s = dot(&w, &v, ell);
        if s == 0 {
            return None;
        }
        let si = inv_mod_u64(s, ell);
        let cy = lm(dot(&w, &y, ell), si, ell);
        for i in 0..n {
            x[i] = la(x[i], lm(cy, w[i], ell), ell);
        }
        let a1 = lm(dot(&v, &v, ell), si, ell);
        let mut wn = vec![0u64; n];
        if first {
            for i in 0..n {
                wn[i] = ls(v[i], lm(a1, w[i], ell), ell);
            }
        } else {
            let b1 = lm(dot(&v, &v_prev, ell), inv_mod_u64(s_prev, ell), ell);
            for i in 0..n {
                let t = ls(v[i], lm(a1, w[i], ell), ell);
                wn[i] = ls(t, lm(b1, w_prev[i], ell), ell);
            }
        }
        first = false;
        w_prev = w;
        w = wn;
        v_prev = v.clone();
        s_prev = s;
        iters += 1;
        if iters > n + 5 {
            return None;
        }
    }
    let z: Vec<u64> = (0..n).map(|i| ls(x[i], b[i], ell)).collect();
    if z.iter().all(|&e| e == 0) {
        return None;
    }
    Some(z)
}

pub fn row_apply(r: &Row, z: &[u64], ell: u64) -> u64 {
    let mut s = 0u64;
    for &(c, a) in r {
        s = la(s, lm(a, z[c as usize], ell), ell);
    }
    s
}

// ---------------------------------------------------------------- splitting a degree-3 divisor
/// Base points of the net of conics (3 x 6 rows over mons_of_degree(2)) as three distinct
/// rational points, or None if the divisor does not split into three distinct rational points.
pub fn split3(f: &Fp, conics: &[[u32; 6]; 3], mons2: &[(u8, u8, u8)], rng: &mut Rng) -> Option<[P3; 3]> {
    // coordinates along the line through R0=(0,0,1) and T(s)=(1,s,0): point (v, s v, u)
    // conic = alpha u^2 + (b0 + s b1) u v + (g0 + s g1 + s^2 g2) v^2
    let mut al = [0u64; 3];
    let mut b0 = [0u64; 3];
    let mut b1 = [0u64; 3];
    let mut g0 = [0u64; 3];
    let mut g1 = [0u64; 3];
    let mut g2 = [0u64; 3];
    for i in 0..3 {
        for (m, &(a, b, c)) in mons2.iter().enumerate() {
            let co = conics[i][m] as u64;
            match (a, b, c) {
                (0, 0, 2) => al[i] = co,
                (1, 0, 1) => b0[i] = co,
                (0, 1, 1) => b1[i] = co,
                (2, 0, 0) => g0[i] = co,
                (1, 1, 0) => g1[i] = co,
                (0, 2, 0) => g2[i] = co,
                _ => unreachable!(),
            }
        }
    }
    let row_at = |i: usize, s: u64| -> [u64; 3] {
        [al[i], f.add(b0[i], f.mul(s, b1[i])), f.add(f.add(g0[i], f.mul(s, g1[i])), f.mul(f.mul(s, s), g2[i]))]
    };
    let det_at = |s: u64| -> u64 {
        let r = [row_at(0, s), row_at(1, s), row_at(2, s)];
        let t0 = f.sub(f.mul(r[1][1], r[2][2]), f.mul(r[1][2], r[2][1]));
        let t1 = f.sub(f.mul(r[1][0], r[2][2]), f.mul(r[1][2], r[2][0]));
        let t2 = f.sub(f.mul(r[1][0], r[2][1]), f.mul(r[1][1], r[2][0]));
        f.add(f.sub(f.mul(r[0][0], t0), f.mul(r[0][1], t1)), f.mul(r[0][2], t2))
    };
    let d = [det_at(0), det_at(1), det_at(2), det_at(3)];
    // Lagrange on nodes 0..3 -> coefficients c0..c3
    let i2 = f.inv(2);
    let i6 = f.inv(6);
    let c3 = {
        // d0*(-1/6) + d1*(1/2) + d2*(-1/2) + d3*(1/6)
        let a = f.mul(f.sub(d[3], d[0]), i6);
        let b = f.mul(f.sub(d[1], d[2]), i2);
        f.add(a, b)
    };
    let c2 = {
        // l0: -(x^3 -6x^2+11x-6)/6 -> x^2: +1 ; l1: x(x^2-5x+6)/2 -> -5/2 ; l2: -x(x^2-4x+3)/2 -> +4/2 ; l3: x(x^2-3x+2)/6 -> -3/6
        let t0 = d[0];
        let t1 = f.mul(d[1], f.neg(f.mul(5, i2)));
        let t2 = f.mul(d[2], f.mul(4, i2));
        let t3 = f.mul(d[3], f.neg(f.mul(3, i6)));
        f.add(f.add(t0, t1), f.add(t2, t3))
    };
    let c1 = {
        // l0: -11/6 ; l1: 6/2=3 ; l2: -3/2 ; l3: 2/6
        let t0 = f.mul(d[0], f.neg(f.mul(11, i6)));
        let t1 = f.mul(d[1], 3);
        let t2 = f.mul(d[2], f.neg(f.mul(3, i2)));
        let t3 = f.mul(d[3], f.mul(2, i6));
        f.add(f.add(t0, t1), f.add(t2, t3))
    };
    let c0 = d[0];
    if c3 == 0 {
        return None;
    }
    let rs = roots(f, &vec![c0, c1, c2, c3], rng);
    if rs.len() != 3 {
        return None;
    }
    let mut pts = [[0u64; 3]; 3];
    for (k, &s) in rs.iter().enumerate() {
        let r = [row_at(0, s), row_at(1, s), row_at(2, s)];
        // common root: nullvector (u^2, uv, v^2) = cross of two independent rows
        let mut nv = None;
        for (i, j) in [(0, 1), (0, 2), (1, 2)] {
            let c = cross(f, &r[i], &r[j]);
            if c != [0, 0, 0] {
                nv = Some(c);
                break;
            }
        }
        let c = nv?;
        // (u:v) from (u^2 : uv : v^2)
        let (u, v) = if c[0] != 0 {
            (c[0], c[1])
        } else if c[2] != 0 {
            (c[1], c[2])
        } else {
            return None;
        };
        let p = [v, f.mul(s, v), u];
        if p == [0, 0, 0] {
            return None;
        }
        pts[k] = pnorm(f, &p);
    }
    if pts[0] == pts[1] || pts[0] == pts[2] || pts[1] == pts[2] {
        return None;
    }
    Some(pts)
}
