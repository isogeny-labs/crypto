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

    /// The rational points on all three conics of `m`: `Some` iff exactly
    /// three distinct ones (then they are the divisor `D`).
    fn decompose(&self, m: &[[u32; 6]; 3], nat: &mut Nat) -> Option<[u32; 3]> {
        let q = self.q;
        let mut found = [0u32; 3];
        let mut nf = 0usize;
        'pts: for (k, v) in self.val2.iter().enumerate() {
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
                    continue 'pts;
                }
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

    /// The same base locus as [`Ctx::decompose`], found algebraically in
    /// `polylog q` counted work instead of a scan over all `~q` points.
    ///
    /// Choose a coordinate `v` whose square has a nonzero coefficient in
    /// some conic `C_i0` of the net (so the coordinate point `e_v` is not a
    /// base point), eliminate `v^2` to get two conics `D_1, D_2` of degree
    /// at most 1 in `v`, and take `g = Res_v(D_1, D_2) = b_1 c_2 - b_2 c_1`,
    /// a binary cubic in the other two coordinates `(u, w)`.  Every base
    /// point of the net projects to a root of `g`.  The distinct rational
    /// roots of `g` (Cantor-Zassenhaus, [`proots`], plus the root at
    /// infinity when `deg g(t, 1) < 3`) give `v` by one division, or, when
    /// `b_1 = b_2 = 0` there, by the quadratic `C_i0 = 0` on that line (one
    /// counted square root).  Each candidate is checked on all three
    /// conics, normalised and looked up in the point index (the same
    /// identity map as [`Ctx::line`]).  If no coordinate square occurs, the
    /// net is `<xy, xz, yz>` and the candidates are the coordinate points.
    ///
    /// `Err(())` when `g` vanishes identically or more than three points
    /// are found (the net has a common component); the caller then falls
    /// back to the counted scan, so the output always equals
    /// [`Ctx::decompose`].
    fn decompose_alg(
        &self,
        m: &[[u32; 6]; 3],
        rs: &mut u64,
        nat: &mut Nat,
    ) -> Result<Option<[u32; 3]>, ()> {
        let q = self.q;
        // index of the conic monomial x_i x_j in [x^2, xy, xz, y^2, yz, z^2]
        const I2: [[usize; 3]; 3] = [[0, 1, 2], [1, 3, 4], [2, 4, 5]];
        let cs: [[u64; 6]; 3] = m.map(|row| row.map(u64::from));
        let mut cands: Vec<[u64; 3]> = Vec::with_capacity(6);
        let vsel = [1usize, 0, 2]
            .into_iter()
            .find(|&v| cs.iter().any(|c| c[I2[v][v]] != 0));
        match vsel {
            None => {
                cands.extend([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
            }
            Some(v) => {
                let (u, w) = match v {
                    0 => (1, 2),
                    1 => (0, 2),
                    _ => (0, 1),
                };
                let i0 = (0..3).find(|&k| cs[k][I2[v][v]] != 0).expect("pivot");
                let a0 = cs[i0][I2[v][v]];
                // D_k = a0 C_k - a_k C_i0 for the two other conics
                let mut d = [[0u64; 6]; 2];
                for (slot, k) in (0..3).filter(|&k| k != i0).enumerate() {
                    let ak = cs[k][I2[v][v]];
                    for s in 0..6 {
                        let x = if a0 == 1 || cs[k][s] == 0 {
                            cs[k][s]
                        } else {
                            pm(q, a0, cs[k][s], nat)
                        };
                        let y = if ak == 0 || cs[i0][s] == 0 {
                            0
                        } else {
                            pm(q, ak, cs[i0][s], nat)
                        };
                        d[slot][s] = fs(q, x, y, nat);
                    }
                }
                // b(u, w) = [coef w, coef u]; c(u, w) = [w^2, uw, u^2]
                let lin = |c: &[u64; 6]| [c[I2[v][w]], c[I2[u][v]]];
                let quad = |c: &[u64; 6]| [c[I2[w][w]], c[I2[u][w]], c[I2[u][u]]];
                let (b1, c1, b2, c2) = (lin(&d[0]), quad(&d[0]), lin(&d[1]), quad(&d[1]));
                // g = b1 c2 - b2 c1, ascending powers of u (w = 1)
                let mut g = [0u64; 4];
                for i in 0..2 {
                    for j in 0..3 {
                        let t1 = if b1[i] != 0 && c2[j] != 0 {
                            pm(q, b1[i], c2[j], nat)
                        } else {
                            0
                        };
                        let t2 = if b2[i] != 0 && c1[j] != 0 {
                            pm(q, b2[i], c1[j], nat)
                        } else {
                            0
                        };
                        let t = fs(q, t1, t2, nat);
                        g[i + j] = pa(q, g[i + j], t, nat);
                    }
                }
                if g.iter().all(|&x| x == 0) {
                    return Err(());
                }
                // roots (u0 : w0) of g
                let mut roots: Vec<(u64, u64)> =
                    proots(q, &g, rs, nat).into_iter().map(|t| (t, 1)).collect();
                if g[3] == 0 {
                    roots.push((1, 0));
                }
                let ev_lin = |b: &[u64; 2], (u0, w0): (u64, u64), nat: &mut Nat| {
                    if w0 == 0 {
                        b[1]
                    } else {
                        let t = pm(q, b[1], u0, nat);
                        pa(q, b[0], t, nat)
                    }
                };
                let ev_quad = |c: &[u64; 3], (u0, w0): (u64, u64), u2: u64, nat: &mut Nat| {
                    if w0 == 0 {
                        c[2]
                    } else {
                        let t1 = pm(q, c[1], u0, nat);
                        let t2 = pm(q, c[2], u2, nat);
                        let s = pa(q, c[0], t1, nat);
                        pa(q, s, t2, nat)
                    }
                };
                for rt in roots {
                    let u2 = if rt.1 == 0 { 0 } else { pm(q, rt.0, rt.0, nat) };
                    let bb1 = ev_lin(&b1, rt, nat);
                    let bb2 = ev_lin(&b2, rt, nat);
                    let mut vs: Vec<u64> = Vec::with_capacity(2);
                    if bb1 != 0 || bb2 != 0 {
                        let (bb, cc) = if bb1 != 0 {
                            (bb1, ev_quad(&c1, rt, u2, nat))
                        } else {
                            (bb2, ev_quad(&c2, rt, u2, nat))
                        };
                        let ib = finv(q, bb, nat);
                        let y = pm(q, cc, ib, nat);
                        vs.push(fs(q, 0, y, nat));
                    } else {
                        // the line through e_v and the root: a0 v^2 + b0 v + c0 = 0
                        let b0 = ev_lin(&lin(&cs[i0]), rt, nat);
                        let c0 = ev_quad(&quad(&cs[i0]), rt, u2, nat);
                        let t1 = pm(q, b0, b0, nat);
                        let t2 = pm(q, a0, c0, nat);
                        let t2 = pm(q, t2, 4, nat);
                        let disc = fs(q, t1, t2, nat);
                        if let Some(sd) = self.sq.sqrt(disc, nat) {
                            let two_a = pa(q, a0, a0, nat);
                            let ia = finv(q, two_a, nat);
                            let nb0 = fs(q, 0, b0, nat);
                            let r1 = pa(q, nb0, sd, nat);
                            vs.push(pm(q, r1, ia, nat));
                            if sd != 0 {
                                let r2 = fs(q, nb0, sd, nat);
                                vs.push(pm(q, r2, ia, nat));
                            }
                        }
                    }
                    for y in vs {
                        let mut pt = [0u64; 3];
                        pt[u] = rt.0;
                        pt[w] = rt.1;
                        pt[v] = y;
                        cands.push(pt);
                    }
                }
            }
        }
        let mut found: Vec<u32> = Vec::with_capacity(3);
        for pt in cands {
            // on all three conics (counted)
            let mut mv = [0u64; 6];
            for i in 0..3 {
                for j in i..3 {
                    if pt[i] != 0 && pt[j] != 0 {
                        mv[I2[i][j]] = pm(q, pt[i], pt[j], nat);
                    }
                }
            }
            let on_all = cs.iter().all(|c| {
                let mut s = 0u64;
                for k in 0..6 {
                    if c[k] != 0 && mv[k] != 0 {
                        let t = if c[k] == 1 { mv[k] } else { pm(q, c[k], mv[k], nat) };
                        s = pa(q, s, t, nat);
                    }
                }
                s == 0
            });
            if !on_all {
                continue;
            }
            let Some(np) = self.normalize(pt, nat) else {
                continue;
            };
            if let Some(&id) = self.idx.get(&np) {
                if !found.contains(&id) {
                    found.push(id);
                }
            }
        }
        match found.len() {
            3 => {
                found.sort_unstable();
                Ok(Some([found[0], found[1], found[2]]))
            }
            0..=2 => Ok(None),
            _ => Err(()),
        }
    }
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
    /// [`QDescent::Algebraic`] trials that fell back to the counted scan.
    pub decomp_fallbacks: u64,
    /// Decomposition relations that closed a cycle in the component of "1"
    /// when found (counted towards the rows under [`QSolve::Linked`]).
    pub inhom_linked: u64,
    /// Attempts skipped before Wiedemann by [`QSolve::Linked`].
    pub la_skipped: u32,
    /// Per-attempt diagnosis ([`QDiemOpts::diag`]).
    pub diag: Vec<QAttemptDiag>,
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

/// How a decomposition trial finds the base points of the conic net.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QDescent {
    /// The frozen instrument: evaluate the net at every rational point
    /// ([`Ctx::decompose`], `O(q)` counted work per trial).
    #[default]
    Scan,
    /// [`Ctx::decompose_alg`]: resultant, cubic root finding and at most one
    /// square root per root, `polylog q` counted work per trial; identical
    /// output (a counted scan fallback when the net has a common component).
    Algebraic,
}

/// When the relation loop asks for a counted linear-algebra attempt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QSolve {
    /// The frozen instrument: every decomposition relation with at most two
    /// large primes counts towards the inhomogeneous rows, and an attempt
    /// runs Wiedemann whenever the core is large enough.
    #[default]
    Legacy,
    /// A decomposition relation counts towards the inhomogeneous rows only
    /// if it closes a cycle in the component of the vertex "1" of the
    /// large-prime graph at the moment it is found (its large primes are
    /// already linked to the factor base), i.e. it reaches the cycle
    /// matrix; others are still kept as edges.  Before Wiedemann, an
    /// attempt whose core has no row in the `t` column (so `t` cannot be
    /// determined) is skipped and charged only its structural pass.
    Linked,
}

/// Options of [`run_diem_with`]; the default is the frozen instrument.
#[derive(Clone, Copy, Debug, Default)]
pub struct QDiemOpts {
    pub descent: QDescent,
    pub solve: QSolve,
    /// Per-attempt diagnostics (dense rank of the core over `F_r`,
    /// uncounted, test / diagnosis only).
    pub diag: bool,
}

/// One counted attempt's diagnosis (only with [`QDiemOpts::diag`]).
#[derive(Clone, Copy, Debug, Default)]
pub struct QAttemptDiag {
    pub core_rows: usize,
    pub core_cols: usize,
    /// Core rows with a nonzero `t` coefficient.
    pub t_weight: usize,
    pub rank: usize,
    /// Rank without the `t` column: `t` is determined iff `rank_no_t + 1 == rank`.
    pub rank_no_t: usize,
    /// 0 core too small, 1 Wiedemann `None`, 2 wrong `t`, 3 certified,
    /// 4 skipped by [`QSolve::Linked`] (no `t` row).
    pub outcome: u8,
    /// Inhomogeneous relations found so far / of those, ones that closed a
    /// cycle in the component of "1" when found.
    pub inhom: u64,
    pub inhom_linked: u64,
}

/// Rank of `rows` (columns `0..ncols`, minus `skip`) over `F_r`, dense
/// Gaussian elimination, uncounted (diagnosis only).
fn dense_rank(rows: &[(Vec<(u32, u64)>, u64)], ncols: usize, skip: Option<u32>, r: u64) -> usize {
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|(t, _)| {
            let mut v = vec![0u64; ncols];
            for &(c, x) in t {
                if Some(c) != skip {
                    v[c as usize] = x % r;
                }
            }
            v
        })
        .collect();
    let inv = |a: u64| {
        let (mut b, mut e, mut res) = (a, r - 2, 1u64);
        while e > 0 {
            if e & 1 == 1 {
                res = mulmod(res, b, r);
            }
            b = mulmod(b, b, r);
            e >>= 1;
        }
        res
    };
    let mut rank = 0;
    for c in 0..ncols {
        let Some(pr) = (rank..m.len()).find(|&i| m[i][c] != 0) else {
            continue;
        };
        m.swap(rank, pr);
        let ip = inv(m[rank][c]);
        let piv = m[rank].clone();
        for (i, row) in m.iter_mut().enumerate() {
            if i != rank && row[c] != 0 {
                let f = mulmod(row[c], ip, r);
                for k in c..ncols {
                    if piv[k] != 0 {
                        row[k] = subm(row[k], mulmod(f, piv[k], r), r);
                    }
                }
            }
        }
        rank += 1;
    }
    rank
}

/// Run one Diem arm on an instance (the frozen instrument).
pub fn run_diem(inst: &QInstance, prm: QIcParams) -> QIcResult {
    run_diem_with(inst, prm, QDiemOpts::default())
}

/// [`run_diem`] with the descent and solve policy chosen by `opts`.
pub fn run_diem_with(inst: &QInstance, prm: QIcParams, opts: QDiemOpts) -> QIcResult {
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
    let mut drs = 0xDEC0_5EED_0000_0001u64 ^ inst.q as u64 ^ prm.seed.rotate_left(17);

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
                    let dec = match opts.descent {
                        QDescent::Scan => ctx.decompose(m, &mut res.rel_decomp),
                        QDescent::Algebraic => {
                            match ctx.decompose_alg(m, &mut drs, &mut res.rel_decomp) {
                                Ok(d) => d,
                                Err(()) => {
                                    res.decomp_fallbacks += 1;
                                    ctx.decompose(m, &mut res.rel_decomp)
                                }
                            }
                        }
                    };
                    if let Some(qs) = dec {
                        let tcoef = subm(0, mulmod(h, b, r), r);
                        let rhs = mulmod(h, a, r);
                        if let Some((rel, lps)) = make_rel(&qs, tcoef, rhs, &col) {
                            let good =
                                !prm.verify_relations || ver.check(&pq, &mut ctx, &qs, true, &x);
                            if good {
                                // does the edge close a cycle in the component of "1"?
                                // (counted DSU finds under Linked; an uncounted,
                                // non-compressing read under Legacy, so the
                                // frozen instrument's counts are unchanged)
                                let linked = if opts.solve == QSolve::Linked {
                                    let r0 = dsu.find(0);
                                    lps.iter().all(|k| match vid.get(k) {
                                        Some(&v) => dsu.find(v) == r0,
                                        None => false,
                                    })
                                } else {
                                    let r0 = dsu_root(&dsu, 0);
                                    lps.iter().all(|k| match vid.get(k) {
                                        Some(&v) => dsu_root(&dsu, v) == r0,
                                        None => false,
                                    })
                                };
                                push_rel(rel, &lps, &mut vid, &mut dsu, &mut rels, &mut res);
                                res.decomp_relations += 1;
                                if linked {
                                    res.inhom_linked += 1;
                                }
                                if linked || opts.solve == QSolve::Legacy {
                                    ninhom += 1;
                                }
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
        let mut dg = QAttemptDiag::default();
        if opts.diag {
            dg.core_rows = core.len();
            dg.core_cols = cc;
            dg.t_weight = core
                .iter()
                .filter(|(t, _)| t.iter().any(|&(c, v)| c == ct && v != 0))
                .count();
            dg.rank = dense_rank(&core, cc, None, r);
            dg.rank_no_t = dense_rank(&core, cc, Some(ct), r);
            dg.inhom = res.decomp_relations;
            dg.inhom_linked = res.inhom_linked;
        }
        if core.len() >= cc + margin(cc) {
            // Linked: no core row holds t, so t is not determined (counted scan)
            let t_seen = opts.solve == QSolve::Legacy || {
                let mut seen = false;
                for (t, _) in &core {
                    res.la.add += t.len() as u64;
                    if t.iter().any(|&(c, v)| c == ct && v != 0) {
                        seen = true;
                        break;
                    }
                }
                seen
            };
            dg.outcome = 1;
            if !t_seen {
                res.la_skipped += 1;
                dg.outcome = 4;
            } else {
                let dense = res.attempts > 1;
                if let Some(xs) = wiedemann(&core, cc, r, &mut rng, dense, &mut res.la) {
                    let t = xs[ct as usize];
                    let chk = cmul(&pq, &mut ops, &g, t);
                    dg.outcome = 2;
                    if chk == tgt {
                        res.recovered = Some(t);
                        res.certified = true;
                        solved = true;
                        dg.outcome = 3;
                        if opts.diag {
                            res.diag.push(dg);
                        }
                        continue;
                    }
                }
            }
        }
        if opts.diag {
            res.diag.push(dg);
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

/// Root of `x` without path compression or counting (diagnosis only).
fn dsu_root(dsu: &Dsu, mut x: u32) -> u32 {
    while dsu.parent[x as usize] != x {
        x = dsu.parent[x as usize];
    }
    x
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

    /// The algebraic descent returns exactly the scan's base points (or
    /// `None` with it) on every typical class tried: random sums of three
    /// points (which mostly decompose) and random classes `[k] x` (which
    /// mostly do not), over several curves and fields.
    #[test]
    fn decompose_alg_matches_scan() {
        let mut total = 0u64;
        let mut some = 0u64;
        let mut fallbacks = 0u64;
        for (q, seed, n) in [
            (31u32, 1u64, 3000usize),
            (43, 2, 3000),
            (101, 3, 3000),
            (103, 4, 3000),
            (251, 5, 2000),
            (1031, 6, 1500),
            (1033, 7, 1500),
        ] {
            let (pq, pts) = curve(q, seed);
            let mut nat = Nat::default();
            let ctx = Ctx::new(q, pq.coefficients(), &mut nat);
            let mut rng = StdRng::seed_from_u64(seed ^ 0xABCD);
            let mut rs = 12345u64;
            let (mut ns, mut na) = (Nat::default(), Nat::default());
            for i in 0..n {
                let mut x = random_class(&pq, &pts, &mut rng);
                if i % 2 == 1 {
                    x = pq.mul_scalar(&x, rng.gen_range(2..1_000_000u64) as u128);
                }
                let PicClass::Typ(m) = &x else { continue };
                let a = ctx.decompose(m, &mut ns);
                let b = match ctx.decompose_alg(m, &mut rs, &mut na) {
                    Ok(b) => b,
                    Err(()) => {
                        fallbacks += 1;
                        ctx.decompose(m, &mut na)
                    }
                };
                assert_eq!(a, b, "q={q} seed={seed} trial {i}");
                total += 1;
                some += u64::from(a.is_some());
            }
            if q > 1000 {
                // polylog vs linear: far fewer counted multiplications
                assert!(na.mul * 5 < ns.mul, "q={q}: alg {} vs scan {}", na.mul, ns.mul);
            }
        }
        eprintln!("decompose_alg_matches_scan: {total} typical classes, {some} decomposed, {fallbacks} fallbacks");
        assert!(total > 10_000 && some > 1_000, "total {total} some {some}");
    }

    /// The net `<xy, xz, yz>` (no coordinate square) and nets whose base
    /// points share a projection: built from three chosen points, checked
    /// against the scan.
    #[test]
    fn decompose_alg_special_nets() {
        for (q, seed) in [(31u32, 11u64), (43, 12), (101, 13)] {
            let (pq, pts) = curve(q, seed);
            let mut nat = Nat::default();
            let mut ctx = Ctx::new(q, pq.coefficients(), &mut nat);
            let (k0, _) = kappa_from_lines(&pq, &mut ctx, 0, &mut nat).expect("a line");
            let mut rs = 7u64;
            let mut n_checked = 0;
            let mut n_some = 0;
            // all triples of points among the first 12 (includes collinear and
            // same-projection triples)
            let k = pts.len().min(12);
            for a in 0..k {
                for b in a + 1..k {
                    for c in b + 1..k {
                        let x = {
                            let mut s = pq.zero();
                            for &i in &[a, b, c] {
                                s = pq.add(&s, &pq.point_class(pts[i]));
                            }
                            pq.sub(&s, &k0)
                        };
                        let PicClass::Typ(m) = &x else { continue };
                        let s = ctx.decompose(m, &mut nat);
                        let g = ctx
                            .decompose_alg(m, &mut rs, &mut nat)
                            .unwrap_or_else(|()| ctx.decompose(m, &mut nat));
                        assert_eq!(s, g, "q={q} triple {a},{b},{c}");
                        n_checked += 1;
                        n_some += usize::from(s.is_some());
                    }
                }
            }
            eprintln!("q={q}: {n_checked} triples, {n_some} decomposed");
            assert!(n_checked > 50 && n_some > 50);
        }
    }

    fn same_but_counts(a: &QIcResult, b: &QIcResult) {
        assert_eq!(a.recovered, b.recovered);
        assert_eq!(a.certified, b.certified);
        assert_eq!(a.group_ops, b.group_ops);
        assert_eq!(a.pic_mul, b.pic_mul);
        assert_eq!(a.la, b.la);
        assert_eq!(a.rel_enum, b.rel_enum);
        assert_eq!(
            (a.lines_tried, a.relations, a.rels_by_lp, a.decomp_trials),
            (b.lines_tried, b.relations, b.rels_by_lp, b.decomp_trials)
        );
        assert_eq!(
            (a.core_rows, a.core_cols, a.attempts, a.decomp_relations),
            (b.core_rows, b.core_cols, b.attempts, b.decomp_relations)
        );
    }

    /// With the algebraic descent the frozen solve sees identical relations:
    /// everything but the decomposition count is unchanged, and the run
    /// still certifies.
    #[test]
    fn run_diem_algebraic_descent_same_relations() {
        for q in [101u32, 251, 1031] {
            let inst = instance(q);
            for seed in 1..=3u64 {
                let prm = QIcParams {
                    arm: QArm::DoubleLp,
                    nb: (3.0 * f64::from(q).sqrt()).ceil() as usize,
                    seed,
                    max_lines: 1 << 24,
                    verify_relations: false,
                };
                let a = run_diem(&inst, prm);
                let b = run_diem_with(
                    &inst,
                    prm,
                    QDiemOpts {
                        descent: QDescent::Algebraic,
                        ..Default::default()
                    },
                );
                same_but_counts(&a, &b);
                assert!(a.certified && a.recovered == Some(inst.t_secret));
                assert_eq!(b.decomp_fallbacks, 0);
                if q == 1031 {
                    assert!(b.rel_decomp.mul < a.rel_decomp.mul);
                }
            }
        }
    }

    /// The linked solve policy with the algebraic descent certifies and
    /// returns the planted logarithm (relations verified by Pic arithmetic).
    #[test]
    fn run_diem_linked_certifies() {
        for q in [101u32, 251, 1031] {
            let inst = instance(q);
            for seed in 1..=4u64 {
                let prm = QIcParams {
                    arm: QArm::DoubleLp,
                    nb: (3.0 * f64::from(q).sqrt()).ceil() as usize,
                    seed,
                    max_lines: 1 << 24,
                    verify_relations: seed == 1,
                };
                let o = run_diem_with(
                    &inst,
                    prm,
                    QDiemOpts {
                        descent: QDescent::Algebraic,
                        solve: QSolve::Linked,
                        diag: false,
                    },
                );
                assert!(o.certified, "q={q} seed={seed}: {o:?}");
                assert_eq!(o.recovered, Some(inst.t_secret));
                assert_eq!(o.bad_relations, 0);
            }
        }
    }

    /// Diagnosis of the Wiedemann attempts (run with `--ignored --nocapture`).
    #[test]
    #[ignore]
    fn diag_attempts() {
        let qs: Vec<u32> = std::env::var("QDIAG_Q")
            .unwrap_or("4099".into())
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let seeds: Vec<u64> = std::env::var("QDIAG_CS")
            .unwrap_or("35,205,220,257".into())
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        for &q in &qs {
            for &cs in &seeds {
                let Ok(inst) = make_instance(q, cs, cs, 1) else {
                    continue;
                };
                for seed in 1..=2u64 {
                    for solve in [QSolve::Legacy, QSolve::Linked] {
                        let o = run_diem_with(
                            &inst,
                            QIcParams {
                                arm: QArm::DoubleLp,
                                nb: (3.0 * f64::from(q).sqrt()).ceil() as usize,
                                seed,
                                max_lines: 50_000_000,
                                verify_relations: false,
                            },
                            QDiemOpts {
                                descent: QDescent::Algebraic,
                                solve,
                                diag: true,
                            },
                        );
                        eprintln!(
                            "q={q} cs={cs} seed={seed} {solve:?}: cert={} att={} skipped={} decomp={} inhom={} linked={} la_mul={} rel_mul={}",
                            o.certified, o.attempts, o.la_skipped, o.decomp_trials, o.decomp_relations,
                            o.inhom_linked, o.la.mul, o.rel.mul
                        );
                        for d in &o.diag {
                            eprintln!("    {d:?} t_determined={}", d.rank_no_t + 1 == d.rank);
                        }
                    }
                }
            }
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
