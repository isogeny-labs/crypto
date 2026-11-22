//! # Genus-3 plane-quartic rung of the genus ladder: Diem index calculus
//! against negation-map Pollard rho on the Jacobian of a generic smooth plane
//! quartic over `F_q`.
//!
//! Synthetic random curves only; nothing here is a claim about a deployed
//! curve.  Every operation of a solve is counted and every logarithm is
//! certified by `[t]G == T`.
//!
//! ## Conventions (checked numerically in the tests)
//!
//! [`PlaneQuartic`] stores a class `x` through the unique effective degree-3
//! divisor `D` in `x + D_inf`, `D_inf ~ K - P0` (`K` the line section).  With
//! `c(P) := [P - P0]` and `kappa := [K - 4 P0]`:
//!
//! * a line section `P1 + P2 + P3 + P4` gives `c(P1) + c(P2) + c(P3) + c(P4) = kappa`;
//! * a typical class `Typ(m)` whose conic net passes through three distinct
//!   rational points `Q1, Q2, Q3` is `x = c(Q1) + c(Q2) + c(Q3) - kappa`.
//!
//! Unknowns modulo `r` are `lambda_P = log_G([h] c(P))` for the factor-base
//! points `P != P0` (`c(P0) = 0` has no column), `lambda_kappa` and `t`.
//!
//! ## What is counted
//!
//! * `group_ops`: every `Pic^0` addition or doubling the solver performs
//!   (decomposition starts and steps, the certificate).  The `F_q` work inside
//!   them is the [`PlaneQuartic`] counter delta (`pic_mul`, `pic_inv`).
//! * `rel`: every `F_q` multiplication / addition of the relation phase:
//!   point enumeration (root finding of `F(1, y, z)` for every `y`), monomial
//!   values, Taylor data of the factor-base points, line coefficients,
//!   discriminants, square roots, residual points and their normalisation
//!   (inversions by Fermat, counted as multiplications), and conic
//!   evaluations of the decompositions.  `rel_enum` and `rel_decomp` are the
//!   enumeration and decomposition shares of `rel`.
//! * `la`: graph, peeling and Wiedemann modulo `r` (reused from
//!   `chained_transfer::ic_control`).
//! * Optional verification (`verify_relations`) is reported apart
//!   (`verify_ops`, `verify_pic_mul`) and is not in `group_ops` / `pic_*`.
//!
//! Instance set-up (point count, order by BSGS, generator, target) is not
//! charged to any arm.

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::chained_transfer::ic_control::{
    build_rows, mulmod, pa, peel, pm, ppow, split_order, subm, wiedemann, Dsu, Nat, Rel, REL_FB,
};
use super::genus_ladder::{sge_reduce, SgeParams, SgeStats};
use super::ic_boundary::{rho_reference_walk, CountedGroup, GroupOps, RhoWalk};
use super::plane_quartic_pic::{
    is_odd_prime_below_2_31, mons_of_degree, next_smooth_quartic_with_point, FqCounter, PicClass,
    PlaneQuartic, Pt,
};
use super::residual_walk::mix64;

// ── a Copy image of `PicClass` ───────────────────────────────────────────

/// `PicClass` as a `Copy` value (both variants are plain arrays), so that
/// instances are plain data and the class can be a `CountedGroup::Elt`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QClass {
    Exc(Pt),
    Typ([[u32; 6]; 3]),
}

impl From<&PicClass> for QClass {
    fn from(x: &PicClass) -> QClass {
        match x {
            PicClass::Exc(p) => QClass::Exc(*p),
            PicClass::Typ(m) => QClass::Typ(*m),
        }
    }
}

impl From<QClass> for PicClass {
    fn from(x: QClass) -> PicClass {
        match x {
            QClass::Exc(p) => PicClass::Exc(p),
            QClass::Typ(m) => PicClass::Typ(m),
        }
    }
}

fn class_hash(x: &PicClass) -> u64 {
    let k = x.key();
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for w in k.0 {
        h = mix64(h ^ w as u64);
    }
    h
}

// ── counted F_q helpers ──────────────────────────────────────────────────

#[inline]
fn fs(q: u64, a: u64, b: u64, n: &mut Nat) -> u64 {
    n.add += 1;
    if a >= b {
        a - b
    } else {
        a + q - b
    }
}

#[inline]
fn finv(q: u64, a: u64, n: &mut Nat) -> u64 {
    ppow(q, a, q - 2, n)
}

/// Counted square roots in `F_q` (Tonelli-Shanks; `q = 3 mod 4` shortcut).
struct Sqrt {
    q: u64,
    s: u32,
    odd: u64,
    z: u64,
}

impl Sqrt {
    fn new(q: u64) -> Sqrt {
        let mut odd = q - 1;
        let mut s = 0;
        while odd.is_multiple_of(2) {
            odd /= 2;
            s += 1;
        }
        let mut scratch = Nat::default();
        let mut z = 2;
        while ppow(q, z, (q - 1) / 2, &mut scratch) != q - 1 {
            z += 1;
        }
        Sqrt { q, s, odd, z }
    }

    fn sqrt(&self, a: u64, n: &mut Nat) -> Option<u64> {
        let q = self.q;
        if a == 0 {
            return Some(0);
        }
        if q % 4 == 3 {
            let d = ppow(q, a, (q + 1) / 4, n);
            return if pm(q, d, d, n) == a { Some(d) } else { None };
        }
        if ppow(q, a, (q - 1) / 2, n) != 1 {
            return None;
        }
        let mut m = self.s;
        let mut c = ppow(q, self.z, self.odd, n);
        let mut t = ppow(q, a, self.odd, n);
        let mut x = ppow(q, a, self.odd.div_ceil(2), n);
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = pm(q, tt, tt, n);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = pm(q, b, b, n);
            }
            m = i;
            c = pm(q, b, b, n);
            t = pm(q, t, c, n);
            x = pm(q, x, b, n);
        }
        Some(x)
    }
}

// ── counted polynomials over F_q (degree <= 4) ──────────────────────────

fn ptrim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn pmonic(q: u64, a: &mut [u64], n: &mut Nat) {
    let l = *a.last().expect("nonzero polynomial");
    if l != 1 {
        let il = finv(q, l, n);
        for c in a.iter_mut() {
            *c = pm(q, *c, il, n);
        }
    }
}

/// `a mod m`, `m` monic.
fn prem(q: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let mut r = a.to_vec();
    ptrim(&mut r);
    let dm = m.len() - 1;
    while r.len() > dm {
        let dr = r.len() - 1;
        let c = r[dr];
        for j in 0..dm {
            if m[j] != 0 {
                let t = pm(q, c, m[j], n);
                r[dr - dm + j] = fs(q, r[dr - dm + j], t, n);
            }
        }
        r.pop();
        ptrim(&mut r);
    }
    r
}

/// Quotient `a / m`, `m` monic.
fn pdiv(q: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let mut r = a.to_vec();
    let dm = m.len() - 1;
    if r.len() <= dm {
        return Vec::new();
    }
    let mut quo = vec![0u64; r.len() - dm];
    while r.len() > dm {
        let dr = r.len() - 1;
        let c = r[dr];
        quo[dr - dm] = c;
        for j in 0..dm {
            if m[j] != 0 {
                let t = pm(q, c, m[j], n);
                r[dr - dm + j] = fs(q, r[dr - dm + j], t, n);
            }
        }
        r.pop();
    }
    quo
}

fn pmulmod(q: u64, a: &[u64], b: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            if y != 0 {
                let t = pm(q, x, y, n);
                c[i + j] = pa(q, c[i + j], t, n);
            }
        }
    }
    prem(q, &c, m, n)
}

fn ppowmod(q: u64, base: &[u64], mut e: u64, m: &[u64], n: &mut Nat) -> Vec<u64> {
    let mut res = prem(q, &[1], m, n);
    let mut b = prem(q, base, m, n);
    while e > 0 {
        if e & 1 == 1 {
            res = pmulmod(q, &res, &b, m, n);
        }
        e >>= 1;
        if e > 0 {
            b = pmulmod(q, &b, &b, m, n);
        }
    }
    res
}

/// Monic gcd.
fn pgcd(q: u64, a: &[u64], b: &[u64], n: &mut Nat) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        pmonic(q, &mut b, n);
        let r = prem(q, &a, &b, n);
        a = b;
        b = r;
    }
    if !a.is_empty() {
        pmonic(q, &mut a, n);
    }
    a
}

fn psplit(q: u64, h: Vec<u64>, rs: &mut u64, n: &mut Nat, out: &mut Vec<u64>) {
    match h.len() {
        0 | 1 => return,
        2 => {
            out.push(if h[0] == 0 { 0 } else { q - h[0] });
            return;
        }
        _ => {}
    }
    loop {
        *rs = mix64(rs.wrapping_add(0x9E37_79B9_7F4A_7C15));
        let a = *rs % q;
        let mut t = ppowmod(q, &[a, 1], (q - 1) / 2, &h, n);
        if t.is_empty() {
            continue;
        }
        t[0] = fs(q, t[0], 1, n);
        ptrim(&mut t);
        if t.is_empty() {
            continue;
        }
        let d = pgcd(q, &h, &t, n);
        if d.len() > 1 && d.len() < h.len() {
            let quo = pdiv(q, &h, &d, n);
            psplit(q, d, rs, n, out);
            psplit(q, quo, rs, n, out);
            return;
        }
    }
}

/// Distinct roots in `F_q` of `g` (ascending coefficients), counted.
fn proots(q: u64, g: &[u64], rs: &mut u64, n: &mut Nat) -> Vec<u64> {
    let mut g = g.to_vec();
    ptrim(&mut g);
    if g.len() <= 1 {
        return Vec::new();
    }
    pmonic(q, &mut g, n);
    let mut d = ppowmod(q, &[0, 1], q, &g, n);
    if d.len() < 2 {
        d.resize(2, 0);
    }
    d[1] = fs(q, d[1], 1, n);
    ptrim(&mut d);
    let h = if d.is_empty() { g } else { pgcd(q, &g, &d, n) };
    let mut out = Vec::new();
    psplit(q, h, rs, n, &mut out);
    out.sort_unstable();
    out
}

/// All rational points of `F = 0` (normalised, first nonzero coordinate 1),
/// in the order `[1, y, z]` by `(y, z)`, then `[0, 1, z]`, then `[0, 0, 1]`.
/// Root finding of `F(1, y, z)` for every `y`: `O(q log q)` counted work.
pub fn rational_points(q: u32, f: &[u32; 15], nat: &mut Nat) -> Vec<Pt> {
    let qq = q as u64;
    let m4 = mons_of_degree(4);
    let mut rs = 0x5EED_0000_0000_0001u64 ^ qq;
    let mut out = Vec::new();
    for y in 0..qq {
        let mut yp = [1u64, y, 0, 0, 0];
        for k in 2..5 {
            yp[k] = pm(qq, yp[k - 1], y, nat);
        }
        let mut co = [0u64; 5];
        for (i, &(_a, b, c)) in m4.iter().enumerate() {
            if f[i] == 0 {
                continue;
            }
            let t = if b == 0 {
                f[i] as u64
            } else {
                pm(qq, f[i] as u64, yp[b as usize], nat)
            };
            co[c as usize] = pa(qq, co[c as usize], t, nat);
        }
        for z in proots(qq, &co, &mut rs, nat) {
            out.push([1, y as u32, z as u32]);
        }
    }
    let mut co = [0u64; 5];
    for (i, &(a, _b, c)) in m4.iter().enumerate() {
        if a == 0 && f[i] != 0 {
            co[c as usize] = pa(qq, co[c as usize], f[i] as u64, nat);
        }
    }
    for z in proots(qq, &co, &mut rs, nat) {
        out.push([0, 1, z as u32]);
    }
    let i004 = m4.iter().position(|&m| m == (0, 0, 4)).expect("z^4");
    if f[i004] == 0 {
        out.push([0, 0, 1]);
    }
    out
}

// ── the relation context ────────────────────────────────────────────────

fn binom(n: u8, k: u8) -> u64 {
    match k {
        0 => 1,
        1 => n as u64,
        2 => (n as u64 * (n as u64).saturating_sub(1)) / 2,
        _ => unreachable!(),
    }
}

enum LineRes {
    /// A point repeats (tangency): skipped.
    Tangent,
    /// The residual quadratic does not split over `F_q`.
    NoSplit,
    /// The two residual points (indices into `pts`).
    Split(u32, u32),
    /// A residual point was not found among the rational points (a bug).
    Bad,
}

/// Points, their monomial values and Taylor data, and the line / conic
/// machinery of the relation phase.
struct Ctx {
    q: u64,
    sq: Sqrt,
    pts: Vec<Pt>,
    idx: HashMap<Pt, u32>,
    /// `[x^2, xy, xz, y^2, yz, z^2]` at each point (order of `mons_of_degree(2)`).
    val2: Vec<[u64; 6]>,
    /// Gradient of `F` as linear forms on the cubic monomials.
    gtab: [[u64; 10]; 3],
    /// Second-order Taylor coefficients (`F_xx/2, F_xy, F_xz, F_yy/2, F_yz,
    /// F_zz/2`) as linear forms on the conic monomials.
    htab: [[u64; 6]; 6],
    /// Cached `(grad, second-order)` per point.
    tay: Vec<Option<([u64; 3], [u64; 6])>>,
    pos2_of3: [(usize, usize); 10],
    /// Indices of the rational points on the line `x = 0` (at most four).
    inf: Vec<u32>,
}

impl Ctx {
    fn new(q: u32, f: &[u32; 15], nat: &mut Nat) -> Ctx {
        let qq = q as u64;
        let pts = rational_points(q, f, nat);
        let idx: HashMap<Pt, u32> = pts
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, i as u32))
            .collect();
        let val2: Vec<[u64; 6]> = pts
            .iter()
            .map(|p| {
                let (x, y, z) = (p[0] as u64, p[1] as u64, p[2] as u64);
                [
                    pm(qq, x, x, nat),
                    pm(qq, x, y, nat),
                    pm(qq, x, z, nat),
                    pm(qq, y, y, nat),
                    pm(qq, y, z, nat),
                    pm(qq, z, z, nat),
                ]
            })
            .collect();
        let m2 = mons_of_degree(2);
        let m3 = mons_of_degree(3);
        let m4 = mons_of_degree(4);
        let pos2 = |e: (u8, u8, u8)| m2.iter().position(|&m| m == e).expect("conic monomial");
        let pos3 = |e: (u8, u8, u8)| m3.iter().position(|&m| m == e).expect("cubic monomial");
        let mut gtab = [[0u64; 10]; 3];
        let mut htab = [[0u64; 6]; 6];
        for (i, &(a, b, c)) in m4.iter().enumerate() {
            let fc = f[i] as u64;
            if fc == 0 {
                continue;
            }
            if a > 0 {
                let k = pos3((a - 1, b, c));
                gtab[0][k] = (gtab[0][k] + fc * a as u64) % qq;
            }
            if b > 0 {
                let k = pos3((a, b - 1, c));
                gtab[1][k] = (gtab[1][k] + fc * b as u64) % qq;
            }
            if c > 0 {
                let k = pos3((a, b, c - 1));
                gtab[2][k] = (gtab[2][k] + fc * c as u64) % qq;
            }
            for (s, &(ea, eb, ec)) in m2.iter().enumerate() {
                if a >= ea && b >= eb && c >= ec {
                    let co = binom(a, ea) * binom(b, eb) * binom(c, ec) % qq;
                    let k = pos2((a - ea, b - eb, c - ec));
                    htab[s][k] = (htab[s][k] + fc * co) % qq;
                }
            }
        }
        // each cubic monomial as (conic monomial) * (coordinate)
        let mut pos2_of3 = [(0usize, 0usize); 10];
        for (k, &(a, b, c)) in m3.iter().enumerate() {
            pos2_of3[k] = if a > 0 {
                (pos2((a - 1, b, c)), 0)
            } else if b > 0 {
                (pos2((0, b - 1, c)), 1)
            } else {
                (pos2((0, 0, c - 1)), 2)
            };
        }
        let n = pts.len();
        let inf: Vec<u32> = (0..n as u32).filter(|&k| pts[k as usize][0] == 0).collect();
        Ctx {
            q: qq,
            sq: Sqrt::new(qq),
            pts,
            idx,
            val2,
            gtab,
            htab,
            tay: vec![None; n],
            pos2_of3,
            inf,
        }
    }

    /// Taylor data at point `k` (counted once, then cached).
    fn taylor(&mut self, k: usize, nat: &mut Nat) {
        if self.tay[k].is_some() {
            return;
        }
        let q = self.q;
        let p = self.pts[k];
        let v2 = self.val2[k];
        let mut v3 = [0u64; 10];
        for (j, &(s, c)) in self.pos2_of3.iter().enumerate() {
            v3[j] = pm(q, v2[s], p[c] as u64, nat);
        }
        let mut g = [0u64; 3];
        for (gv, row) in g.iter_mut().zip(self.gtab.iter()) {
            for (&co, &v) in row.iter().zip(v3.iter()) {
                if co != 0 {
                    let t = pm(q, co, v, nat);
                    *gv = pa(q, *gv, t, nat);
                }
            }
        }
        let mut hq = [0u64; 6];
        for (hv, row) in hq.iter_mut().zip(self.htab.iter()) {
            for (&co, &v) in row.iter().zip(v2.iter()) {
                if co != 0 {
                    let t = pm(q, co, v, nat);
                    *hv = pa(q, *hv, t, nat);
                }
            }
        }
        self.tay[k] = Some((g, hq));
    }

    fn normalize(&self, r: [u64; 3], nat: &mut Nat) -> Option<Pt> {
        let q = self.q;
        let i = (0..3).find(|&i| r[i] != 0)?;
        let iv = finv(q, r[i], nat);
        let mut out = [0u32; 3];
        out[i] = 1;
        for j in i + 1..3 {
            out[j] = pm(q, r[j], iv, nat) as u32;
        }
        Some(out)
    }

    /// The line through points `i != j` (Taylor data must be cached).
    fn line(&self, i: usize, j: usize, nat: &mut Nat) -> LineRes {
        let q = self.q;
        let (gi, hi) = self.tay[i].expect("taylor data of i");
        let (gj, _) = self.tay[j].expect("taylor data of j");
        let pi = self.pts[i].map(|v| v as u64);
        let pj = self.pts[j].map(|v| v as u64);
        let dot = |g: &[u64; 3], p: &[u64; 3], nat: &mut Nat| {
            let mut s = 0u64;
            for k in 0..3 {
                if p[k] != 0 {
                    let t = pm(q, g[k], p[k], nat);
                    s = pa(q, s, t, nat);
                }
            }
            s
        };
        let c1 = dot(&gi, &pj, nat);
        let c3 = dot(&gj, &pi, nat);
        if c1 == 0 || c3 == 0 {
            return LineRes::Tangent;
        }
        let mut c2 = 0u64;
        for s in 0..6 {
            if hi[s] != 0 && self.val2[j][s] != 0 {
                let t = pm(q, hi[s], self.val2[j][s], nat);
                c2 = pa(q, c2, t, nat);
            }
        }
        let t1 = pm(q, c2, c2, nat);
        let t2 = pm(q, c1, c3, nat);
        let t2 = pm(q, t2, 4, nat);
        let disc = fs(q, t1, t2, nat);
        if disc == 0 {
            return LineRes::Tangent;
        }
        let Some(d) = self.sq.sqrt(disc, nat) else {
            return LineRes::NoSplit;
        };
        let mu = pa(q, c1, c1, nat);
        let l1 = fs(q, d, c2, nat);
        let s = pa(q, c2, d, nat);
        let l2 = fs(q, 0, s, nat);
        let mut mpj = [0u64; 3];
        for k in 0..3 {
            if pj[k] != 0 {
                mpj[k] = pm(q, mu, pj[k], nat);
            }
        }
        let mut res = [0u32; 2];
        for (slot, l) in [l1, l2].into_iter().enumerate() {
            let mut rr = [0u64; 3];
            for k in 0..3 {
                let t = if pi[k] != 0 { pm(q, l, pi[k], nat) } else { 0 };
                rr[k] = pa(q, t, mpj[k], nat);
            }
            let Some(pt) = self.normalize(rr, nat) else {
                return LineRes::Bad;
            };
            let Some(&id) = self.idx.get(&pt) else {
                return LineRes::Bad;
            };
            res[slot] = id;
        }
        LineRes::Split(res[0], res[1])
    }

    /// Whether the conic values `v` (at a point) vanish on all three conics
    /// of `m` (counted; stops at the first conic that does not vanish).
    fn on_net(&self, v: &[u64; 6], m: &[[u32; 6]; 3], nat: &mut Nat) -> bool {
        let q = self.q;
        for row in m {
            let mut s = 0u64;
            for c in 0..6 {
                if row[c] != 0 && v[c] != 0 {
                    let t = if row[c] == 1 {
                        v[c]
                    } else {
                        pm(q, row[c] as u64, v[c], nat)
                    };
                    s = pa(q, s, t, nat);
                }
            }
            if s != 0 {
                return false;
            }
        }
        true
    }

    /// The rational points on all three conics of `m`: `Some` iff exactly
    /// three distinct ones (then they are the divisor `D`).  The scan:
    /// evaluates the conics at every rational point of `C`.
    fn decompose(&self, m: &[[u32; 6]; 3], nat: &mut Nat) -> Option<[u32; 3]> {
        let mut found = [0u32; 3];
        let mut nf = 0usize;
        for (k, v) in self.val2.iter().enumerate() {
            if !self.on_net(v, m, nat) {
                continue;
            }
            found[nf] = k as u32;
            nf += 1;
            if nf == 3 {
                // the base locus of the net on C is supp(D): no fourth point
                return Some(found);
            }
        }
        None
    }

    /// The same point set by algebra: the affine common points of two
    /// conics of the net from the resultant (degree <= 4 in one variable,
    /// counted root finding, back-substitution), kept when they lie on the
    /// third conic and on `C`; the at most four rational points of `C` on
    /// `x = 0` by direct test.  Returns the first three by point index
    /// (as the scan does) and whether every elimination was degenerate, in
    /// which case the scan answered instead.
    fn decompose_alg(&self, m: &[[u32; 6]; 3], nat: &mut Nat) -> (Option<[u32; 3]>, bool) {
        let q = self.q;
        let mut rs = 0xA16E_B4A1_C0DE_0001u64;
        for row in m {
            for &c in row {
                rs = mix64(rs ^ c as u64);
            }
        }
        let mut found: Vec<u32> = Vec::new();
        for &k in &self.inf {
            if self.on_net(&self.val2[k as usize], m, nat) {
                found.push(k);
            }
        }
        // chart x = 1, unknowns (u, w); `swap` exchanges y and z.
        let mut res_poly: Option<(Vec<u64>, usize, usize, bool)> = None;
        'pick: for (ia, ib) in [(0usize, 1usize), (0, 2), (1, 2)] {
            for swap in [false, true] {
                let a = conic_uw(&m[ia], swap);
                let b = conic_uw(&m[ib], swap);
                let mut r = resultant_w(q, &a, &b, nat);
                ptrim(&mut r);
                if !r.is_empty() {
                    res_poly = Some((r, ia, ib, swap));
                    break 'pick;
                }
            }
        }
        let Some((rp, ia, ib, swap)) = res_poly else {
            return (self.decompose(m, nat), true);
        };
        let a = conic_uw(&m[ia], swap);
        let b = conic_uw(&m[ib], swap);
        let mut inv2: [Option<u64>; 2] = [None; 2];
        for u0 in proots(q, &rp, &mut rs, nat) {
            let mut cand: Vec<u64> = Vec::new();
            let mut done = false;
            for (ci, c) in [&a, &b].into_iter().enumerate() {
                // c as a polynomial in w at u = u0: [a0, a1, a2]
                let u2 = pm(q, u0, u0, nat);
                let t1 = pm(q, c[1], u0, nat);
                let t3 = pm(q, c[3], u2, nat);
                let a0 = pa(q, pa(q, c[0], t1, nat), t3, nat);
                let t4 = pm(q, c[4], u0, nat);
                let a1 = pa(q, c[2], t4, nat);
                let a2 = c[5];
                if a2 != 0 {
                    let i2 = match inv2[ci] {
                        Some(v) => v,
                        None => {
                            let two_a = pa(q, a2, a2, nat);
                            let v = finv(q, two_a, nat);
                            inv2[ci] = Some(v);
                            v
                        }
                    };
                    let t = pm(q, a1, a1, nat);
                    let f4 = pm(q, pm(q, a2, a0, nat), 4, nat);
                    let disc = fs(q, t, f4, nat);
                    if let Some(d) = self.sq.sqrt(disc, nat) {
                        let n1 = fs(q, d, a1, nat);
                        cand.push(pm(q, n1, i2, nat));
                        if d != 0 {
                            let n2 = fs(q, 0, pa(q, a1, d, nat), nat);
                            cand.push(pm(q, n2, i2, nat));
                        }
                    }
                    done = true;
                } else if a1 != 0 {
                    let i1 = finv(q, a1, nat);
                    let n0 = fs(q, 0, a0, nat);
                    cand.push(pm(q, n0, i1, nat));
                    done = true;
                } else if a0 != 0 {
                    done = true; // no root of this conic at u0
                }
                if done {
                    break;
                }
            }
            if !done {
                // both conics vanish on the whole line u = u0
                return (self.decompose(m, nat), true);
            }
            for w0 in cand {
                let (y, z) = if swap { (w0, u0) } else { (u0, w0) };
                if let Some(&k) = self.idx.get(&[1, y as u32, z as u32]) {
                    if self.on_net(&self.val2[k as usize], m, nat) {
                        found.push(k);
                    }
                }
            }
        }
        found.sort_unstable();
        found.dedup();
        if found.len() >= 3 {
            (Some([found[0], found[1], found[2]]), false)
        } else {
            (None, false)
        }
    }
}

/// A conic of the net in the chart `x = 1` with unknowns `(u, w)` (`w` the
/// variable eliminated; `swap`: `u = z`, `w = y`): the six coefficients of
/// `1, u, w, u^2, u w, w^2`.
fn conic_uw(c: &[u32; 6], swap: bool) -> [u64; 6] {
    let c: [u64; 6] = std::array::from_fn(|i| c[i] as u64);
    if swap {
        [c[0], c[2], c[1], c[5], c[4], c[3]]
    } else {
        c
    }
}

/// Counted product of polynomials (ascending coefficients).
fn pmul_poly(q: u64, a: &[u64], b: &[u64], n: &mut Nat) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            if y != 0 {
                let t = pm(q, x, y, n);
                c[i + j] = pa(q, c[i + j], t, n);
            }
        }
    }
    c
}

fn psub_poly(q: u64, a: &[u64], b: &[u64], n: &mut Nat) -> Vec<u64> {
    let mut c = vec![0u64; a.len().max(b.len())];
    for (i, ci) in c.iter_mut().enumerate() {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        *ci = if y == 0 { x } else { fs(q, x, y, n) };
    }
    c
}

/// Resultant in `w` of two conics given by `conic_uw` coefficient vectors:
/// `(a2 b0 - a0 b2)^2 - (a2 b1 - a1 b2)(a1 b0 - a0 b1)`, a polynomial in `u`
/// of degree <= 4 (counted).
fn resultant_w(q: u64, a: &[u64; 6], b: &[u64; 6], n: &mut Nat) -> Vec<u64> {
    let (a0, a1, a2) = ([a[0], a[1], a[3]], [a[2], a[4]], [a[5]]);
    let (b0, b1, b2) = ([b[0], b[1], b[3]], [b[2], b[4]], [b[5]]);
    let x = psub_poly(q, &pmul_poly(q, &a2, &b0, n), &pmul_poly(q, &a0, &b2, n), n);
    let y = psub_poly(q, &pmul_poly(q, &a2, &b1, n), &pmul_poly(q, &a1, &b2, n), n);
    let z = psub_poly(q, &pmul_poly(q, &a1, &b0, n), &pmul_poly(q, &a0, &b1, n), n);
    let xx = pmul_poly(q, &x, &x, n);
    let yz = pmul_poly(q, &y, &z, n);
    psub_poly(q, &xx, &yz, n)
}

/// `kappa = [K - 4 P0]` from the first line (in pair order) whose residual
/// splits, by Pic arithmetic: `c(P1) + c(P2) + c(P3) + c(P4)`.  Returns the
/// class and the four point indices.  Verification / tests only.
fn kappa_from_lines(
    pq: &PlaneQuartic,
    ctx: &mut Ctx,
    skip: usize,
    scratch: &mut Nat,
) -> Option<(PicClass, [u32; 4])> {
    let n = ctx.pts.len();
    let mut seen = 0usize;
    for i in 0..n {
        for j in 0..i {
            ctx.taylor(i, scratch);
            ctx.taylor(j, scratch);
            if let LineRes::Split(a, b) = ctx.line(i, j, scratch) {
                if seen < skip {
                    seen += 1;
                    continue;
                }
                let ids = [i as u32, j as u32, a, b];
                let mut s = pq.zero();
                for &k in &ids {
                    s = pq.add(&s, &pq.point_class(ctx.pts[k as usize]));
                }
                return Some((s, ids));
            }
        }
    }
    None
}

// ── instances ───────────────────────────────────────────────────────────

/// A Jacobian of a random smooth plane quartic with `#J = h r`, `r` prime,
/// a generator `G` of the order-`r` subgroup and a target `T = [t]G`.
/// Plain data (`Send + Sync`); every run rebuilds its own [`PlaneQuartic`].
#[derive(Clone, Debug)]
pub struct QInstance {
    pub q: u32,
    pub f: [u32; 15],
    pub p0: Pt,
    pub order: u64,
    pub h: u64,
    pub r: u64,
    pub n_points: u64,
    pub t_secret: u64,
    pub g: QClass,
    pub target: QClass,
    pub curve_seed: u64,
}

impl QInstance {
    /// The curve with its `Pic^0` arithmetic (fresh counters).
    pub fn curve(&self) -> PlaneQuartic {
        PlaneQuartic::new(self.q, self.f, self.p0).expect("instance curve is smooth")
    }
}

fn random_class(pq: &PlaneQuartic, pts: &[Pt], rng: &mut StdRng) -> PicClass {
    let mut s = pq.zero();
    for _ in 0..3 {
        let p = pts[rng.gen_range(0..pts.len())];
        s = pq.add(&s, &pq.point_class(p));
    }
    s
}

/// `#Pic^0(F_q)` of a smooth plane quartic: `c1 = N1 - q - 1` from the point
/// count, then baby-step giant-step for `L(1)` over the window
/// `|c2| <= 15 q`, `|c3| <= 20 q^1.5` intersected with the Hasse-Weil
/// interval, every candidate checked against 32 random classes; exactly one
/// must survive.  Not charged to any arm.
pub fn jacobian_order(pq: &PlaneQuartic, pts: &[Pt], seed: u64) -> Result<u64, String> {
    let q = pq.q() as i128;
    let n1 = pts.len() as i128;
    let c1 = n1 - q - 1;
    let base = 1 + c1 + q * q * c1 + q * q * q;
    let dev = (1 + q) * 15 * q + (20.0 * (q as f64).powf(1.5)).ceil() as i128 + 1;
    let sq = (q as f64).sqrt();
    let lo = ((sq - 1.0).powi(6) * (1.0 - 1e-9)).floor().max(1.0) as i128;
    let hi = ((sq + 1.0).powi(6) * (1.0 + 1e-9)).ceil() as i128;
    let from = (base - dev).max(lo).max(1);
    let to = (base + dev).min(hi);
    if to <= from {
        return Err("empty window".into());
    }
    let width = (to - from) as u64;
    let m = ((width as f64).sqrt().ceil() as u64).max(1);
    let mut rng = StdRng::seed_from_u64(seed ^ 0xB5B5_0DD5_0DD5_B5B5);
    // a base class whose baby steps do not repeat
    let mut baby: HashMap<super::plane_quartic_pic::PicKey, u64> = HashMap::new();
    let mut d0 = pq.zero();
    let mut ok = false;
    for _ in 0..16 {
        d0 = random_class(pq, pts, &mut rng);
        baby.clear();
        baby.reserve(m as usize + 1);
        let mut cur = pq.zero();
        ok = true;
        for j in 0..=m {
            if baby.insert(cur.key(), j).is_some() {
                ok = false;
                break;
            }
            cur = pq.add(&cur, &d0);
        }
        if ok {
            break;
        }
    }
    if !ok {
        return Err("no base class of large order".into());
    }
    let w = pq.neg(&pq.mul_scalar(&d0, from as u128));
    let nmd = pq.neg(&pq.mul_scalar(&d0, m as u128));
    let mut cur = w;
    let mut cands = Vec::new();
    for i in 0..=(width / m + 1) {
        if let Some(&j) = baby.get(&cur.key()) {
            let s = i * m + j;
            if s <= width {
                cands.push(from as u64 + s);
            }
        }
        cur = pq.add(&cur, &nmd);
    }
    cands.sort_unstable();
    cands.dedup();
    let tests: Vec<PicClass> = (0..32).map(|_| random_class(pq, pts, &mut rng)).collect();
    let surv: Vec<u64> = cands
        .into_iter()
        .filter(|&n| {
            tests
                .iter()
                .all(|t| pq.is_zero(&pq.mul_scalar(t, n as u128)))
        })
        .collect();
    match surv.as_slice() {
        [one] => Ok(*one),
        [] => Err("no candidate".into()),
        many => Err(format!("ambiguous: {} candidates", many.len())),
    }
}

/// A random smooth quartic with a rational point from `curve_seed`, its exact
/// `#Pic^0`, and (if `#J = h r` with `r` prime, `r > h`, `h <= max_h`) a
/// generator `G` of order `r` and `T = [t]G`, `t` from `target_seed`.
/// `Err` starting with `"not admitted"` when the order does not split.
pub fn make_instance(
    q: u32,
    curve_seed: u64,
    target_seed: u64,
    max_h: u64,
) -> Result<QInstance, String> {
    if !is_odd_prime_below_2_31(q as u64) || q < 5 {
        return Err(format!("bad field size {q}"));
    }
    let mut st = mix64(curve_seed ^ ((q as u64) << 32)) | 1;
    let (f, p0) = next_smooth_quartic_with_point(q, &mut st);
    let pq = PlaneQuartic::new(q, f, p0).map_err(|e| format!("curve: {e:?}"))?;
    let p0 = pq.base_point();
    let mut scratch = Nat::default();
    let pts = rational_points(q, &f, &mut scratch);
    let order = jacobian_order(&pq, &pts, curve_seed)?;
    let (h, r) = split_order(order, max_h)
        .ok_or_else(|| format!("not admitted: order {order} does not split with h <= {max_h}"))?;
    let mut rng = StdRng::seed_from_u64(mix64(curve_seed ^ 0x6E6E_7A7A_0101_2323));
    let mut g = None;
    for _ in 0..64 {
        let x = random_class(&pq, &pts, &mut rng);
        let gg = pq.mul_scalar(&x, h as u128);
        if !pq.is_zero(&gg) && pq.is_zero(&pq.mul_scalar(&gg, r as u128)) {
            g = Some(gg);
            break;
        }
    }
    let g = g.ok_or("no generator of order r")?;
    let t = StdRng::seed_from_u64(mix64(target_seed ^ 0x7A26_E7A2_6E7A_26E7)).gen_range(1..r);
    let target = pq.mul_scalar(&g, t as u128);
    Ok(QInstance {
        q,
        f,
        p0,
        order,
        h,
        r,
        n_points: pts.len() as u64,
        t_secret: t,
        g: QClass::from(&g),
        target: QClass::from(&target),
        curve_seed,
    })
}

/// An instance on a GIVEN smooth quartic with GIVEN generator and target
/// classes (for example classes transferred from another Jacobian), in the
/// representation of the curve `(q, f, p0)`.  `order = h r` is `#Pic^0`,
/// `r` prime.  Checked here, uncharged: the curve builds with base point
/// `p0`, `g != 0`, `[r] g = 0`, `[r] target = 0`, and `target = [t] g` when
/// the logarithm is supplied.  `t_secret` is stored as given, or 0 when
/// unknown (a planted logarithm is never 0).
#[allow(clippy::too_many_arguments)]
pub fn make_instance_from_classes(
    q: u32,
    f: [u32; 15],
    p0: Pt,
    g: &PicClass,
    target: &PicClass,
    order: u64,
    h: u64,
    r: u64,
    n_points: u64,
    t_secret: Option<u64>,
) -> Result<QInstance, String> {
    let pq = PlaneQuartic::new(q, f, p0).map_err(|e| format!("curve: {e:?}"))?;
    if pq.base_point() != pq.normalize_point(p0) {
        return Err("base point changed".into());
    }
    if h.checked_mul(r) != Some(order) {
        return Err(format!("order {order} != h r = {h} * {r}"));
    }
    if pq.is_zero(g) || !pq.is_zero(&pq.mul_scalar(g, r as u128)) {
        return Err("generator does not have order r".into());
    }
    if !pq.is_zero(&pq.mul_scalar(target, r as u128)) {
        return Err("target is not in the subgroup of order r".into());
    }
    if let Some(t) = t_secret {
        if pq.mul_scalar(g, t as u128) != *target {
            return Err("target != [t] g".into());
        }
    }
    Ok(QInstance {
        q,
        f,
        p0: pq.base_point(),
        order,
        h,
        r,
        n_points,
        t_secret: t_secret.unwrap_or(0),
        g: QClass::from(g),
        target: QClass::from(target),
        curve_seed: 0,
    })
}

// ── Diem index calculus ─────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QArm {
    /// Factor base = all rational points; lines through random pairs.
    Full,
    /// Factor base = `nb` points; lines through base pairs, the two residual
    /// points outside the base are large primes (graph, cycles through "1").
    DoubleLp,
}

impl QArm {
    pub fn name(self) -> &'static str {
        match self {
            QArm::Full => "diem_full",
            QArm::DoubleLp => "diem_2lp",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct QIcParams {
    pub arm: QArm,
    /// Factor-base size for `DoubleLp` (ignored by `Full`).
    pub nb: usize,
    pub seed: u64,
    /// `Full`: lines tried; `DoubleLp`: base pairs processed.
    pub max_lines: u64,
    pub verify_relations: bool,
}

/// How the degree-3 divisor of a decomposition class is found.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Decomp {
    /// Evaluate the conics at every rational point of `C` (`O(q)`).
    #[default]
    Scan,
    /// Intersect two conics of the net by a resultant (`O(log q)`).
    Algebraic,
}

/// Additive options of [`run_diem_opts`]; the default reproduces
/// [`run_diem`] exactly.  (Kept apart from [`QIcParams`] so that existing
/// struct literals of `QIcParams` keep compiling.)
#[derive(Clone, Copy, Debug, Default)]
pub struct QIcOpts {
    pub decomp: Decomp,
    /// Structured Gaussian elimination between the peel and Wiedemann.
    pub sge: Option<SgeParams>,
}

#[derive(Clone, Debug, Default)]
pub struct QIcResult {
    pub arm: &'static str,
    pub recovered: Option<u64>,
    pub certified: bool,
    pub exhausted: bool,
    /// Pic additions + doublings performed by the solver.
    pub group_ops: u64,
    /// `F_q` multiplications / inversions inside those Pic operations.
    pub pic_mul: u64,
    pub pic_inv: u64,
    /// Relation phase native `F_q` operations (includes `rel_enum`, `rel_decomp`).
    pub rel: Nat,
    /// Linear algebra modulo `r`.
    pub la: Nat,
    pub fb_cols: usize,
    pub lines_tried: u64,
    pub relations: u64,
    pub rels_by_lp: [u64; 3],
    pub bad_relations: u64,
    pub decomp_trials: u64,
    pub core_rows: usize,
    pub core_cols: usize,
    pub attempts: u32,
    pub wall_ns: u64,
    // ── extra detail (not in the requested shape) ──
    /// Point enumeration share of `rel`.
    pub rel_enum: Nat,
    /// Decomposition (conic evaluation) share of `rel`.
    pub rel_decomp: Nat,
    /// Line relations / decomposition relations among `relations`.
    pub line_relations: u64,
    pub decomp_relations: u64,
    /// Pic operations and their `F_q` multiplications spent on optional
    /// verification (not in `group_ops`, `pic_mul`).
    pub verify_ops: u64,
    pub verify_pic_mul: u64,
    pub group: GroupOps,
    /// Merge stage (inside `la`) and its statistics; default when no SGE.
    pub la_sge: Nat,
    pub sge: SgeStats,
    /// Algebraic decompositions that fell back to the scan (every
    /// elimination degenerate); 0 for `Scan`.
    pub decomp_fallbacks: u64,
}

impl QIcResult {
    /// Native multiplications (relation phase + linear algebra).
    pub fn native_mul(&self) -> u64 {
        self.rel.mul + self.la.mul
    }
}

/// Counted left-to-right double-and-add.
fn cmul(pq: &PlaneQuartic, ops: &mut GroupOps, x: &PicClass, k: u64) -> PicClass {
    if k == 0 {
        return pq.zero();
    }
    let top = 63 - k.leading_zeros();
    let mut acc = x.clone();
    for i in (0..top).rev() {
        acc = pq.double(&acc);
        ops.doubles += 1;
        if (k >> i) & 1 == 1 {
            acc = pq.add(&acc, x);
            ops.adds += 1;
        }
    }
    acc
}

struct Verifier {
    ops: u64,
    fq: FqCounter,
    kappa: Option<PicClass>,
    pclass: HashMap<u32, PicClass>,
    scratch: Nat,
}

impl Verifier {
    fn pc(&mut self, pq: &PlaneQuartic, ctx: &Ctx, k: u32) -> PicClass {
        if let Some(c) = self.pclass.get(&k) {
            return c.clone();
        }
        let c = pq.point_class(ctx.pts[k as usize]);
        self.ops += 1;
        self.pclass.insert(k, c.clone());
        c
    }
    fn kappa(&mut self, pq: &PlaneQuartic, ctx: &mut Ctx) -> PicClass {
        if self.kappa.is_none() {
            let (k, _) = kappa_from_lines(pq, ctx, 0, &mut self.scratch).expect("a split line");
            self.ops += 8;
            self.kappa = Some(k);
        }
        self.kappa.clone().expect("kappa")
    }
    /// `sum c(P_k) - (kappa if minus_kappa) == x`.
    fn check(
        &mut self,
        pq: &PlaneQuartic,
        ctx: &mut Ctx,
        pts: &[u32],
        minus_kappa: bool,
        x: &PicClass,
    ) -> bool {
        let c0 = pq.counter();
        let kap = self.kappa(pq, ctx);
        let mut s = pq.zero();
        for &k in pts {
            let c = self.pc(pq, ctx, k);
            s = pq.add(&s, &c);
            self.ops += 1;
        }
        if minus_kappa {
            s = pq.sub(&s, &kap);
            self.ops += 2;
        }
        let ok = s == *x;
        let d = pq.counter().since(&c0);
        self.fq.mul += d.mul;
        self.fq.inv += d.inv;
        self.fq.add += d.add;
        ok
    }
}

/// Advance `rng` exactly as `wiedemann` on an `nr x ncols` system would
/// (see `genus_ladder::skip_wiedemann_draws`), so that the relation phase
/// after a failed attempt is the same with and without the merge.
fn skip_wiedemann_draws(rng: &mut StdRng, nr: usize, ncols: usize, r: u64, dense_u: bool) {
    for _ in 0..nr {
        let _: u64 = rng.gen_range(1..r);
    }
    if dense_u || ncols <= 16 {
        for _ in 0..ncols {
            let _: u64 = rng.gen_range(1..r);
        }
    } else {
        for _ in 0..16 {
            let _: usize = rng.gen_range(0..ncols);
            let _: u64 = rng.gen_range(1..r);
        }
    }
}

/// Run one Diem arm on an instance (scan decomposition, no merge).
pub fn run_diem(inst: &QInstance, prm: QIcParams) -> QIcResult {
    run_diem_opts(inst, prm, QIcOpts::default())
}

/// Run one Diem arm on an instance with the additive options.
pub fn run_diem_opts(inst: &QInstance, prm: QIcParams, opts: QIcOpts) -> QIcResult {
    let start = Instant::now();
    let pq = inst.curve();
    let c_start = pq.counter();
    let (r, h) = (inst.r, inst.h % inst.r);
    let g: PicClass = inst.g.into();
    let tgt: PicClass = inst.target.into();
    let mut rng = StdRng::seed_from_u64(prm.seed);
    let mut res = QIcResult {
        arm: prm.arm.name(),
        ..Default::default()
    };
    let mut ops = GroupOps::default();
    let mut ver = Verifier {
        ops: 0,
        fq: FqCounter::default(),
        kappa: None,
        pclass: HashMap::new(),
        scratch: Nat::default(),
    };

    // ── points (counted) ──
    let mut ctx = Ctx::new(inst.q, &inst.f, &mut res.rel_enum);
    let n = ctx.pts.len();
    let p0i = ctx.idx[&pq.base_point()] as usize;

    // ── factor base ──
    let mut col = vec![u32::MAX; n];
    let mut ncols = 0u32;
    let base: Vec<usize> = match prm.arm {
        QArm::Full => (0..n).collect(),
        QArm::DoubleLp => {
            let mut order: Vec<usize> = (0..n).filter(|&k| k != p0i).collect();
            let mut prng = StdRng::seed_from_u64(0x51A7_7E2D_0000_0000 ^ inst.q as u64);
            for i in (1..order.len()).rev() {
                let j = prng.gen_range(0..=i);
                order.swap(i, j);
            }
            order.truncate(prm.nb.clamp(2, n - 1));
            order
        }
    };
    for &k in &base {
        if k != p0i {
            col[k] = ncols;
            ncols += 1;
        }
    }
    let kcol = ncols;
    let tcol = ncols + 1;
    let nunk = ncols as usize + 2;
    res.fb_cols = nunk - 1;
    {
        let mut tn = Nat::default();
        for &k in &base {
            ctx.taylor(k, &mut tn);
        }
        res.rel.merge(tn);
    }

    let mut rels: Vec<Rel> = Vec::new();
    let mut vid: HashMap<u32, u32> = HashMap::new();
    let mut dsu = Dsu::new();
    let mut seen_lines: HashSet<[u32; 4]> = HashSet::new();
    let margin = |k: usize| 8 + k / 100;
    let mut next_check = nunk as u64;
    let mut want_inhom = 8u64;
    let mut ninhom = 0u64;
    let mut pair = (1usize, 0usize);
    let mut pairs_done = false;
    let max_decomp = 4096 + 64 * n as u64;
    let mut solved = false;

    // build a Rel from point indices; None if more than two large primes
    let make_rel = |pts_in: &[u32], tcoef: u64, rhs: u64, col: &[u32]| -> Option<(Rel, Vec<u32>)> {
        let mut rel = Rel {
            fb: [(0, 0); REL_FB],
            nfb: 0,
            lp: [(0, 0); 2],
            nlp: 0,
            tcoef,
            rhs,
        };
        let mut lps = Vec::new();
        for &k in pts_in {
            if k as usize == p0i {
                continue;
            }
            let c = col[k as usize];
            if c != u32::MAX {
                rel.fb[rel.nfb as usize] = (c, 1);
                rel.nfb += 1;
            } else {
                lps.push(k);
            }
        }
        if lps.len() > 2 {
            return None;
        }
        rel.fb[rel.nfb as usize] = (kcol, -1);
        rel.nfb += 1;
        Some((rel, lps))
    };

    'outer: while !solved {
        // ── line relations ──
        while (rels.len() as u64) < next_check {
            if res.lines_tried >= prm.max_lines || pairs_done {
                break;
            }
            let (i, j) = match prm.arm {
                QArm::Full => {
                    let i = rng.gen_range(0..n);
                    let mut j = rng.gen_range(0..n - 1);
                    if j >= i {
                        j += 1;
                    }
                    (i, j)
                }
                QArm::DoubleLp => {
                    let (a, b) = pair;
                    if a >= base.len() {
                        pairs_done = true;
                        break;
                    }
                    pair = if b + 1 < a { (a, b + 1) } else { (a + 1, 0) };
                    (base[a], base[b])
                }
            };
            res.lines_tried += 1;
            let (p3, p4) = match ctx.line(i, j, &mut res.rel) {
                LineRes::Split(a, b) => (a, b),
                LineRes::Bad => {
                    res.bad_relations += 1;
                    continue;
                }
                _ => continue,
            };
            let mut key = [i as u32, j as u32, p3, p4];
            key.sort_unstable();
            if !seen_lines.insert(key) {
                continue;
            }
            let Some((rel, lps)) = make_rel(&key, 0, 0, &col) else {
                continue;
            };
            if prm.verify_relations {
                let kap = ver.kappa(&pq, &mut ctx);
                if !ver.check(&pq, &mut ctx, &key, false, &kap) {
                    res.bad_relations += 1;
                    continue;
                }
            }
            push_rel(rel, &lps, &mut vid, &mut dsu, &mut rels, &mut res);
            res.line_relations += 1;
        }

        // ── inhomogeneous rows: x = [a]G + [b]T, a stepped by G ──
        if ninhom < want_inhom {
            let mut a = rng.gen_range(1..r);
            let b = rng.gen_range(1..r);
            let ag = cmul(&pq, &mut ops, &g, a);
            let bt = cmul(&pq, &mut ops, &tgt, b);
            let mut x = pq.add(&ag, &bt);
            ops.adds += 1;
            while ninhom < want_inhom {
                if res.decomp_trials >= max_decomp {
                    res.exhausted = true;
                    break 'outer;
                }
                res.decomp_trials += 1;
                if let PicClass::Typ(m) = &x {
                    let dec = match opts.decomp {
                        Decomp::Scan => ctx.decompose(m, &mut res.rel_decomp),
                        Decomp::Algebraic => {
                            let (d, fb) = ctx.decompose_alg(m, &mut res.rel_decomp);
                            res.decomp_fallbacks += u64::from(fb);
                            d
                        }
                    };
                    if let Some(qs) = dec {
                        let tcoef = subm(0, mulmod(h, b, r), r);
                        let rhs = mulmod(h, a, r);
                        if let Some((rel, lps)) = make_rel(&qs, tcoef, rhs, &col) {
                            let good =
                                !prm.verify_relations || ver.check(&pq, &mut ctx, &qs, true, &x);
                            if good {
                                push_rel(rel, &lps, &mut vid, &mut dsu, &mut rels, &mut res);
                                res.decomp_relations += 1;
                                ninhom += 1;
                            } else {
                                res.bad_relations += 1;
                            }
                        }
                    }
                }
                x = pq.add(&x, &g);
                ops.adds += 1;
                a = if a + 1 == r { 0 } else { a + 1 };
            }
        }

        let out_of_lines = pairs_done || res.lines_tried >= prm.max_lines;

        // ── is a solve worth trying? ──
        let nver = dsu.parent.len();
        if prm.arm == QArm::DoubleLp {
            let need = (nunk + margin(nunk)) as i64;
            let cyc = dsu.cycles0();
            if cyc < need {
                if out_of_lines {
                    res.exhausted = true;
                    break;
                }
                next_check = rels.len() as u64 + (need - cyc).max(1) as u64;
                continue;
            }
        } else {
            let mut scratch = Nat::default();
            let mut rows = build_rows(&rels, nver, tcol, r, &mut scratch);
            strip_zeros(&mut rows);
            let (core, cc, _) = peel(&rows, nunk, tcol, &mut scratch);
            if core.len() < cc + margin(cc) {
                if out_of_lines {
                    res.exhausted = true;
                    break;
                }
                next_check = rels.len() as u64 + (nunk as u64 / 32).max(8);
                continue;
            }
        }

        // ── counted attempt ──
        res.attempts += 1;
        let mut rows = build_rows(&rels, nver, tcol, r, &mut res.la);
        strip_zeros(&mut rows);
        let (core, cc, ct) = peel(&rows, nunk, tcol, &mut res.la);
        res.core_rows = core.len();
        res.core_cols = cc;
        if core.len() >= cc + margin(cc) {
            let dense = res.attempts > 1;
            let sol = match opts.sge {
                None => {
                    wiedemann(&core, cc, r, &mut rng, dense, &mut res.la).map(|xs| xs[ct as usize])
                }
                Some(sp) => {
                    let mut s_nat = Nat::default();
                    let (red, rcc, rct, st) = sge_reduce(&core, cc, ct, r, sp, &mut s_nat);
                    res.la_sge.merge(s_nat);
                    res.la.merge(s_nat);
                    res.sge = st;
                    // solved with a copy of the stream; the stream itself
                    // advances as the unmerged solve would
                    let mut wrng = rng.clone();
                    skip_wiedemann_draws(&mut rng, core.len(), cc, r, dense);
                    wiedemann(&red, rcc, r, &mut wrng, dense, &mut res.la)
                        .map(|xs| xs[rct as usize])
                }
            };
            if let Some(t) = sol {
                let chk = cmul(&pq, &mut ops, &g, t);
                if chk == tgt {
                    res.recovered = Some(t);
                    res.certified = true;
                    solved = true;
                    continue;
                }
            }
        }
        if out_of_lines || res.attempts >= 40 {
            res.exhausted = true;
            break;
        }
        next_check = rels.len() as u64 + (nunk as u64 / 20).max(16);
        want_inhom += 4;
    }
    res.la.add += dsu.ops;
    res.rel.merge(res.rel_enum);
    res.rel.merge(res.rel_decomp);
    let total = pq.counter().since(&c_start);
    res.group = ops;
    res.group_ops = ops.adds + ops.doubles;
    res.pic_mul = total.mul - ver.fq.mul;
    res.pic_inv = total.inv - ver.fq.inv;
    res.verify_ops = ver.ops;
    res.verify_pic_mul = ver.fq.mul;
    res.wall_ns = start.elapsed().as_nanos() as u64;
    res
}

fn strip_zeros(rows: &mut [(Vec<(u32, u64)>, u64)]) {
    for row in rows.iter_mut() {
        row.0.retain(|&(_, v)| v != 0);
    }
}

fn push_rel(
    mut rel: Rel,
    lps: &[u32],
    vid: &mut HashMap<u32, u32>,
    dsu: &mut Dsu,
    rels: &mut Vec<Rel>,
    res: &mut QIcResult,
) {
    for &k in lps {
        let id = *vid.entry(k).or_insert_with(|| dsu.add_vertex());
        rel.lp[rel.nlp as usize] = (id, 1);
        rel.nlp += 1;
    }
    let (a, _, b, _) = rel.endpoints();
    dsu.edge(a, b);
    res.rels_by_lp[rel.nlp as usize] += 1;
    res.relations += 1;
    rels.push(rel);
}

// ── rho ─────────────────────────────────────────────────────────────────

/// The instance's `Pic^0` as a counted group (additions and doublings into
/// `GroupOps`; `neg` and `key` uncharged as in every rho reference).
pub struct QGroup<'a> {
    pub pq: &'a PlaneQuartic,
}

impl CountedGroup for QGroup<'_> {
    type Elt = QClass;
    fn identity(&self) -> QClass {
        QClass::from(&self.pq.zero())
    }
    fn is_identity(&self, p: &QClass) -> bool {
        self.pq.is_zero(&PicClass::from(*p))
    }
    fn add(&self, ops: &mut GroupOps, a: QClass, b: QClass) -> QClass {
        if a == b {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        QClass::from(&self.pq.add(&a.into(), &b.into()))
    }
    fn double(&self, ops: &mut GroupOps, a: QClass) -> QClass {
        ops.doubles += 1;
        QClass::from(&self.pq.double(&a.into()))
    }
    fn neg(&self, a: QClass) -> QClass {
        QClass::from(&self.pq.neg(&a.into()))
    }
    fn key(&self, p: &QClass) -> u64 {
        class_hash(&PicClass::from(*p))
    }
}

#[derive(Clone, Debug, Default)]
pub struct QRhoOut {
    pub recovered: Option<u64>,
    pub certified: bool,
    pub group_ops: u64,
    /// `F_q` work of the walk, including the uncharged negations of the
    /// negation map's canonicalisation (not the independent certificate).
    pub pic_mul: u64,
    pub pic_inv: u64,
    pub steps: u64,
    pub wall_ns: u64,
    pub group: GroupOps,
}

/// Negation-map rho (`rho_reference_walk`, `RhoWalk::negation()`), certified
/// by an independent `mul_scalar`.
pub fn run_rho(inst: &QInstance, seed: u64) -> QRhoOut {
    let start = Instant::now();
    let pq = inst.curve();
    let grp = QGroup { pq: &pq };
    let max_steps = (60.0 * (inst.r as f64).sqrt()) as u64 + 10_000;
    let c0 = pq.counter();
    let rr = rho_reference_walk(
        &grp,
        inst.g,
        inst.target,
        inst.r,
        seed,
        max_steps,
        RhoWalk::negation(),
    );
    let d = pq.counter().since(&c0);
    let certified = rr
        .recovered
        .map(|t| pq.mul_scalar(&inst.g.into(), t as u128) == PicClass::from(inst.target))
        .unwrap_or(false);
    QRhoOut {
        recovered: rr.recovered,
        certified,
        group_ops: rr.group_ops.adds + rr.group_ops.doubles,
        pic_mul: d.mul,
        pic_inv: d.inv,
        steps: rr.steps,
        wall_ns: start.elapsed().as_nanos() as u64,
        group: rr.group_ops,
    }
}

// ── tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn curve(q: u32, seed: u64) -> (PlaneQuartic, Vec<Pt>) {
        let mut st = mix64(seed ^ ((q as u64) << 32)) | 1;
        let (f, p0) = next_smooth_quartic_with_point(q, &mut st);
        let pq = PlaneQuartic::new(q, f, p0).unwrap();
        let mut nat = Nat::default();
        let pts = rational_points(q, &f, &mut nat);
        (pq, pts)
    }

    /// An instance rebuilt from its own classes is the same instance, and a
    /// solve on it returns the planted logarithm.
    #[test]
    fn instance_from_given_classes() {
        let inst = (1..200)
            .find_map(|cs| make_instance(131, cs, cs, 64).ok())
            .expect("an admitted quartic");
        let g: PicClass = inst.g.into();
        let t: PicClass = inst.target.into();
        let again = make_instance_from_classes(
            inst.q,
            inst.f,
            inst.p0,
            &g,
            &t,
            inst.order,
            inst.h,
            inst.r,
            inst.n_points,
            Some(inst.t_secret),
        )
        .unwrap();
        assert_eq!(
            (again.q, again.f, again.p0, again.order, again.h, again.r),
            (inst.q, inst.f, inst.p0, inst.order, inst.h, inst.r)
        );
        assert_eq!(
            (again.g, again.target, again.t_secret),
            (inst.g, inst.target, inst.t_secret)
        );
        let res = run_diem(
            &again,
            QIcParams {
                arm: QArm::DoubleLp,
                nb: (3.0 * f64::from(inst.q).sqrt()).ceil() as usize,
                seed: 1,
                max_lines: 50_000_000,
                verify_relations: false,
            },
        );
        assert!(res.certified);
        assert_eq!(res.recovered, Some(inst.t_secret));
        let rho = run_rho(&again, 1);
        assert_eq!(rho.recovered, Some(inst.t_secret));
        // Wrong data is refused.
        let wrong = make_instance_from_classes(
            inst.q,
            inst.f,
            inst.p0,
            &g,
            &t,
            inst.order,
            inst.h,
            inst.r,
            inst.n_points,
            Some(inst.t_secret % (inst.r - 1) + 1),
        );
        assert!(wrong.is_err());
        let zero = inst.curve().zero();
        assert!(make_instance_from_classes(
            inst.q,
            inst.f,
            inst.p0,
            &zero,
            &t,
            inst.order,
            inst.h,
            inst.r,
            inst.n_points,
            None,
        )
        .is_err());
    }

    #[test]
    fn enumeration_matches_bruteforce() {
        for (q, s) in [(31u32, 1u64), (43, 2), (101, 3)] {
            let (pq, pts) = curve(q, s);
            let mut a = pts.clone();
            let mut b = pq.rational_points_bruteforce();
            a.sort();
            b.sort();
            assert_eq!(a, b, "q={q}");
        }
    }

    #[test]
    fn sqrt_is_correct() {
        for q in [101u64, 251, 97, 13] {
            let s = Sqrt::new(q);
            let mut n = Nat::default();
            for a in 0..q {
                let is_sq = (0..q).any(|x| x * x % q == a);
                match s.sqrt(a, &mut n) {
                    Some(d) => assert_eq!(d * d % q, a),
                    None => assert!(!is_sq),
                }
            }
        }
    }

    /// Every line section gives the same class kappa, and a typical class
    /// whose conic net passes through three rational points Q_i equals
    /// c(Q1) + c(Q2) + c(Q3) - kappa.
    #[test]
    fn kappa_and_decomposition_conventions() {
        for (q, seed) in [(31u32, 5u64), (43, 6), (31, 7)] {
            let (pq, pts) = curve(q, seed);
            let mut nat = Nat::default();
            let mut ctx = Ctx::new(q, pq.coefficients(), &mut nat);
            assert_eq!(ctx.pts, pts);
            let (k0, ids0) = kappa_from_lines(&pq, &mut ctx, 0, &mut nat).expect("a line");
            let mut distinct = 1;
            for skip in [1usize, 3, 7, 12, 20] {
                let (k, ids) = kappa_from_lines(&pq, &mut ctx, skip, &mut nat).expect("a line");
                assert_eq!(k, k0, "q={q} lines {ids0:?} vs {ids:?}");
                if ids != ids0 {
                    distinct += 1;
                }
            }
            assert!(distinct >= 4);
            // tangent line at P0: K - 4P0 also equals the sum over a line through P0
            let mut rng = StdRng::seed_from_u64(seed);
            let mut checked = 0;
            for _ in 0..400 {
                let x = random_class(&pq, &pts, &mut rng);
                let PicClass::Typ(m) = &x else { continue };
                let Some(qs) = ctx.decompose(m, &mut nat) else {
                    continue;
                };
                let mut s = pq.zero();
                for &k in &qs {
                    s = pq.add(&s, &pq.point_class(ctx.pts[k as usize]));
                }
                s = pq.sub(&s, &k0);
                assert_eq!(s, x, "q={q} decomposition {qs:?}");
                // and the conics through the three points give the same class
                let rows: Vec<Vec<u32>> = m.iter().map(|r| r.to_vec()).collect();
                assert_eq!(pq.class_from_conics(rows), x);
                checked += 1;
                if checked >= 12 {
                    break;
                }
            }
            assert!(checked >= 5, "q={q}: only {checked} decompositions");
        }
    }

    #[test]
    fn order_kills_random_classes_and_matches_independent_count() {
        use crate::cryptanalysis::plane_quartic_pic::tests::enumerate_points;
        // independent count on tiny q (N1, N2, N3 over extension fields)
        let mut compared = 0;
        'tiny: for tq in [11u32, 13, 17] {
            for seed in 0..12u64 {
                let (pq, pts) = curve(tq, seed);
                let Ok(n) = jacobian_order(&pq, &pts, seed) else {
                    continue;
                };
                let en = enumerate_points(tq, pq.coefficients());
                assert_eq!(n as i128, en.jac, "q={tq} seed {seed}");
                compared += 1;
                if compared >= 4 {
                    break 'tiny;
                }
            }
        }
        assert!(compared >= 2, "only {compared} tiny-q comparisons");
        for (q, seed) in [(101u32, 1u64), (251, 2)] {
            let (pq, pts) = curve(q, seed);
            let n = jacobian_order(&pq, &pts, seed).expect("order");
            let sq = (q as f64).sqrt();
            assert!((n as f64) >= (sq - 1.0).powi(6) && (n as f64) <= (sq + 1.0).powi(6));
            let mut rng = StdRng::seed_from_u64(99);
            for _ in 0..64 {
                let x = random_class(&pq, &pts, &mut rng);
                assert!(pq.is_zero(&pq.mul_scalar(&x, n as u128)));
            }
        }
    }

    fn instance(q: u32) -> QInstance {
        for cs in 0..200u64 {
            match make_instance(q, cs, cs + 17, 64) {
                Ok(i) => return i,
                Err(e) if e.starts_with("not admitted") => continue,
                Err(e) => panic!("q={q} seed {cs}: {e}"),
            }
        }
        panic!("no instance at q={q}");
    }

    fn end_to_end(q: u32, nb: usize) {
        let inst = instance(q);
        assert_eq!(inst.h * inst.r, inst.order);
        for arm in [QArm::Full, QArm::DoubleLp] {
            let prm = QIcParams {
                arm,
                nb,
                seed: 7,
                max_lines: 1 << 22,
                verify_relations: true,
            };
            let res = run_diem(&inst, prm);
            eprintln!(
                "q={q} order={} h={} r={} N={} {:?}: rec={:?} cert={} ops={} pic_mul={} rel.mul={} la.mul={} rels={} by_lp={:?} lines={} decomp={} att={} core={}x{} verify_ops={} wall={:.3}s",
                inst.order, inst.h, inst.r, inst.n_points, arm, res.recovered, res.certified,
                res.group_ops, res.pic_mul, res.rel.mul, res.la.mul, res.relations, res.rels_by_lp,
                res.lines_tried, res.decomp_trials, res.attempts, res.core_rows, res.core_cols,
                res.verify_ops, res.wall_ns as f64 * 1e-9
            );
            assert!(res.certified, "{arm:?} not certified: {res:?}");
            assert_eq!(res.recovered, Some(inst.t_secret));
            assert_eq!(res.bad_relations, 0);
        }
        let rho = run_rho(&inst, 11);
        eprintln!(
            "q={q} rho: rec={:?} cert={} ops={} pic_mul={} steps={} wall={:.3}s",
            rho.recovered,
            rho.certified,
            rho.group_ops,
            rho.pic_mul,
            rho.steps,
            rho.wall_ns as f64 * 1e-9
        );
        assert!(rho.certified);
        assert_eq!(rho.recovered, Some(inst.t_secret));
    }

    #[test]
    fn end_to_end_q101() {
        end_to_end(101, 30);
    }

    #[test]
    fn end_to_end_q251() {
        end_to_end(251, 48);
    }

    /// The algebraic decomposition returns exactly the scan's answer.
    #[test]
    fn algebraic_decomposition_matches_scan() {
        for (q, seed) in [(101u32, 3u64), (251, 4), (1031, 5)] {
            let (pq, pts) = curve(q, seed);
            let mut nat = Nat::default();
            let ctx = Ctx::new(q, pq.coefficients(), &mut nat);
            let mut rng = StdRng::seed_from_u64(seed ^ 0xD1CE);
            let (mut typ, mut some, mut fallbacks) = (0, 0, 0);
            let (mut sc, mut al) = (Nat::default(), Nat::default());
            for i in 0..260 {
                let x = if i % 2 == 0 {
                    random_class(&pq, &pts, &mut rng)
                } else {
                    let y = random_class(&pq, &pts, &mut rng);
                    pq.add(&y, &random_class(&pq, &pts, &mut rng))
                };
                let PicClass::Typ(m) = &x else { continue };
                typ += 1;
                let a = ctx.decompose(m, &mut sc);
                let (b, fb) = ctx.decompose_alg(m, &mut al);
                assert_eq!(a, b, "q={q} class {i}");
                some += usize::from(a.is_some());
                fallbacks += usize::from(fb);
            }
            eprintln!(
                "q={q}: {typ} typical classes, {some} decomposed, {fallbacks} fallbacks, scan mul {} alg mul {}",
                sc.mul, al.mul
            );
            assert!(typ >= 200, "q={q}: only {typ} typical classes");
            assert!(some >= 20, "q={q}: only {some} decompositions");
            if q >= 1031 {
                assert!(al.mul < sc.mul, "q={q}: algebraic not cheaper");
            }
        }
    }

    /// Same relations, rows and logarithm with the scan and with the
    /// algebraic decomposition; SGE leaves the relation phase unchanged.
    #[test]
    fn algebraic_and_sge_runs_are_identical_where_they_must() {
        for (q, nb) in [(101u32, 30usize), (251, 48)] {
            let inst = instance(q);
            let prm = QIcParams {
                arm: QArm::DoubleLp,
                nb,
                seed: 9,
                max_lines: 1 << 22,
                verify_relations: true,
            };
            let a = run_diem_opts(&inst, prm, QIcOpts::default());
            let b = run_diem_opts(
                &inst,
                prm,
                QIcOpts {
                    decomp: Decomp::Algebraic,
                    sge: None,
                },
            );
            let c = run_diem_opts(
                &inst,
                prm,
                QIcOpts {
                    decomp: Decomp::Algebraic,
                    sge: Some(SgeParams {
                        w_max: 8,
                        fill_max: 3.0,
                    }),
                },
            );
            for x in [&a, &b, &c] {
                assert!(x.certified);
                assert_eq!(x.recovered, Some(inst.t_secret));
                assert_eq!(x.bad_relations, 0);
            }
            let key = |x: &QIcResult| {
                (
                    x.relations,
                    x.rels_by_lp,
                    x.line_relations,
                    x.decomp_relations,
                    x.decomp_trials,
                    x.lines_tried,
                    x.attempts,
                    x.core_rows,
                    x.core_cols,
                    x.group_ops,
                    x.pic_mul,
                    (x.pic_inv, x.rel_enum.mul, x.rel_enum.add),
                )
            };
            assert_eq!(key(&a), key(&b), "q={q} scan vs algebraic");
            assert_eq!(key(&a), key(&c), "q={q} scan vs algebraic+sge");
            assert_eq!(a.decomp_fallbacks + b.decomp_fallbacks, 0);
            assert!(c.sge.rows_in > 0 && c.la_sge.mul + c.la_sge.add > 0);
            eprintln!(
                "q={q}: decomp scan {} alg {}; la {} vs +sge {} (sge {})",
                a.rel_decomp.mul, b.rel_decomp.mul, b.la.mul, c.la.mul, c.la_sge.mul
            );
        }
    }

    /// Timing check at q = 1031 (run with `--ignored`).
    #[test]
    #[ignore]
    fn timing_q1031() {
        let t0 = Instant::now();
        let inst = instance(1031);
        eprintln!(
            "q=1031 instance: order={} h={} r={} N={} setup={:.2}s",
            inst.order,
            inst.h,
            inst.r,
            inst.n_points,
            t0.elapsed().as_secs_f64()
        );
        for (arm, nb) in [(QArm::Full, 0usize), (QArm::DoubleLp, 96)] {
            let res = run_diem(
                &inst,
                QIcParams {
                    arm,
                    nb,
                    seed: 3,
                    max_lines: 1 << 24,
                    verify_relations: false,
                },
            );
            eprintln!("q=1031 {arm:?}: cert={} ops={} pic_mul={} rel.mul={} (enum {}, decomp {}) la.mul={} rels={} lines={} att={} wall={:.3}s",
                res.certified, res.group_ops, res.pic_mul, res.rel.mul, res.rel_enum.mul, res.rel_decomp.mul, res.la.mul, res.relations, res.lines_tried, res.attempts, res.wall_ns as f64 * 1e-9);
            assert_eq!(res.recovered, Some(inst.t_secret));
        }
        let rho = run_rho(&inst, 5);
        eprintln!(
            "q=1031 rho: cert={} ops={} pic_mul={} steps={} wall={:.3}s",
            rho.certified,
            rho.group_ops,
            rho.pic_mul,
            rho.steps,
            rho.wall_ns as f64 * 1e-9
        );
        assert_eq!(rho.recovered, Some(inst.t_secret));
    }
}
