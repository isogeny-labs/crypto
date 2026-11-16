//! System builders (secant branch, shared-epsilon, dense S3), the three
//! extractor arms, group replay and the uncharged exhaustive reference.
use crate::alg::*;
use crate::ec::*;
use crate::ops::*;
use std::collections::{BTreeSet, HashMap};

pub const X: usize = 0;
pub const Z: usize = 1;
pub const M: usize = 2;

fn mono(e: &[u32]) -> Mono {
    e.iter().enumerate().fold(0u64, |a, (i, x)| a + ((*x as u64) << (8 * i)))
}
fn sgn(c: u64, eps: i8, p: u64) -> u64 {
    if eps >= 0 { c } else { fneg(c, p) }
}
fn horner_terms(h: &[u64], v: usize, nv: usize, eps: i8, p: u64, out: &mut Vec<(Mono, u64)>) {
    for (k, c) in h.iter().enumerate() {
        if *c == 0 {
            continue;
        }
        let mut e = vec![0u32; nv];
        e[v] = k as u32;
        out.push((mono(&e), sgn(*c, eps, p)));
    }
}
/// g1 = m^2 - x - z - r ; g2 = x z - r m^2 - 2 s m - r^2 - a   (3 variables x,z,m)
pub fn secant_core(c: &Curve, r: u64, s: u64) -> (Poly, Poly) {
    let p = c.p;
    let g1 = normalize(
        vec![(mono(&[0, 0, 2]), 1), (mono(&[1, 0, 0]), p - 1), (mono(&[0, 1, 0]), p - 1), (0, fneg(r, p))],
        p,
        3,
    );
    let rr = fmul(r, r, p);
    let g2 = normalize(
        vec![
            (mono(&[1, 1, 0]), 1),
            (mono(&[0, 0, 2]), fneg(r, p)),
            (mono(&[0, 0, 1]), fneg(fmul(2, s, p), p)),
            (0, fneg(fadd(rr, c.a, p), p)),
        ],
        p,
        3,
    );
    (g1, g2)
}
/// q(v) = eps*h(v) - m (v - r) + s
pub fn secant_q(h: &[u64], v: usize, eps: i8, r: u64, s: u64, p: u64) -> Poly {
    let mut t = Vec::new();
    horner_terms(h, v, 3, eps, p, &mut t);
    let mut e = [0u32; 3];
    e[v] = 1;
    e[M] = 1;
    t.push((mono(&e), p - 1));
    t.push((mono(&[0, 0, 1]), r));
    t.push((0, s));
    normalize(t, p, 3)
}
pub fn memb_poly(w: &[u64], v: usize, nv: usize, p: u64) -> Poly {
    let mut t = Vec::new();
    horner_terms(w, v, nv, 1, p, &mut t);
    normalize(t, p, nv)
}
pub fn secant_branch(c: &Curve, b: &Base, r: u64, s: u64, eps: i8, eta: i8) -> Vec<Poly> {
    let p = c.p;
    let (g1, g2) = secant_core(c, r, s);
    let mut v = vec![g1, g2, secant_q(&b.h, X, eps, r, s, p), secant_q(&b.h, Z, eta, r, s, p)];
    if !b.graph {
        v.push(memb_poly(&b.memb, X, 3, p));
        v.push(memb_poly(&b.memb, Z, 3, p));
    }
    v
}
/// Shared part for fixed eps: {g1, g2, q_x(eps)} (+ membership for nongraph bases).
pub fn secant_shared(c: &Curve, b: &Base, r: u64, s: u64, eps: i8) -> Vec<Poly> {
    let p = c.p;
    let (g1, g2) = secant_core(c, r, s);
    let mut v = vec![g1, g2, secant_q(&b.h, X, eps, r, s, p)];
    if !b.graph {
        v.push(memb_poly(&b.memb, X, 3, p));
        v.push(memb_poly(&b.memb, Z, 3, p));
    }
    v
}
/// Dense: F(x), F(z), S3(x,z,r) in 2 variables.
pub fn dense_sys(c: &Curve, b: &Base, r: u64) -> Vec<Poly> {
    let p = c.p;
    let rr = fmul(r, r, p);
    let two_r = fmul(2, r, p);
    let ra = fmul(r, c.a, p);
    let xz_coef = fneg(fadd(fmul(2, rr, p), fmul(2, c.a, p), p), p);
    let lin = fneg(fadd(fmul(two_r, c.a, p), fmul(4, c.b, p), p), p);
    let cst = fsub(fmul(c.a, c.a, p), fmul(fmul(4, r, p), c.b, p), p);
    let _ = ra;
    let s3 = normalize(
        vec![
            (mono(&[2, 0]), rr),
            (mono(&[0, 2]), rr),
            (mono(&[1, 1]), xz_coef),
            (mono(&[2, 1]), fneg(two_r, p)),
            (mono(&[1, 2]), fneg(two_r, p)),
            (mono(&[1, 0]), lin),
            (mono(&[0, 1]), lin),
            (0, cst),
            (mono(&[2, 2]), 1),
        ],
        p,
        2,
    );
    vec![memb_poly(&b.memb, X, 2, p), memb_poly(&b.memb, Z, 2, p), s3]
}

// ---------------------------------------------------------------- arms
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pres {
    Secant1, // four branches, each solved separately
    Secant2, // two shared-epsilon solves, eta classified by evaluation
    Dense,
}
/// (column, sign) of a signed base point: sign +1 means (x, h(x)), -1 means (x, -h(x)).
pub type SP = (usize, i8);
#[derive(Default, Debug, Clone)]
pub struct Solved {
    pub roots_raw: usize,      // roots returned by the extractor before replay/dedup
    pub pairs: BTreeSet<(SP, SP)>, // ordered signed pairs (P,Q) with P+Q=R, verified by replay
    pub replay_fail: usize,
    pub censored: bool,
    pub not_zero_dim: bool,
    pub unresolved: bool,
    pub handled_degree: u32,
    pub dim_sum: usize,
    pub solves: u32,
    pub fallbacks: u32,
}
fn merge_ex(s: &mut Solved, e: &Extract) {
    s.censored |= e.censored;
    s.not_zero_dim |= e.not_zero_dim;
    s.unresolved |= e.unresolved;
    s.handled_degree = s.handled_degree.max(e.handled_degree);
    s.dim_sum += e.dim;
    s.solves += 1;
    s.fallbacks += e.fallbacks;
}
fn colmap(b: &Base) -> HashMap<u64, usize> {
    b.xs.iter().enumerate().map(|(i, x)| (*x, i)).collect()
}
fn point_of(c: &Curve, b: &Base, sp: SP) -> Pt {
    let y = b.ys[sp.0];
    (b.xs[sp.0], if sp.1 >= 0 { y } else { fneg(y, c.p) })
}
/// Verify P+Q == R by one group addition; charged to the replay phase.
fn replay_one(c: &Curve, b: &Base, pq: (SP, SP), rt: Pt) -> bool {
    let (p1, p2) = (point_of(c, b, pq.0), point_of(c, b, pq.1));
    add(c, Some(p1), Some(p2)) == Some(rt)
}

pub fn solve_secant(c: &Curve, b: &Base, rt: Pt, pres: Pres) -> Solved {
    let (r, s) = rt;
    let p = c.p;
    let cm = colmap(b);
    let mut out = Solved::default();
    let mut found: Vec<(SP, SP)> = Vec::new();
    match pres {
        Pres::Secant1 => {
            for eps in [1i8, -1] {
                for eta in [1i8, -1] {
                    let save = phase(PH_BUILD);
                    let sys = secant_branch(c, b, r, s, eps, eta);
                    phase(save);
                    let ex = extract(&sys, p, 3);
                    merge_ex(&mut out, &ex);
                    for pt in &ex.points {
                        out.roots_raw += 1;
                        native_n(2);
                        if let (Some(&i), Some(&j)) = (cm.get(&pt[X]), cm.get(&pt[Z])) {
                            found.push(((i, eps), (j, eta)));
                        }
                    }
                }
            }
        }
        Pres::Secant2 => {
            for eps in [1i8, -1] {
                let save = phase(PH_BUILD);
                let sys = secant_shared(c, b, r, s, eps);
                phase(save);
                let ex = extract(&sys, p, 3);
                merge_ex(&mut out, &ex);
                for pt in &ex.points {
                    out.roots_raw += 1;
                    let save = phase(PH_ROOTS);
                    // classify eta: L(z) = m (z - r) - s must equal eta * h(z)
                    let l = fsub(fmul(pt[M], fsub(pt[Z], r, p), p), s, p);
                    let hz = ueval(&b.h, pt[Z], p);
                    let eta = if l == hz {
                        Some(1i8)
                    } else if l == fneg(hz, p) {
                        Some(-1i8)
                    } else {
                        None
                    };
                    phase(save);
                    native_n(2);
                    if let (Some(eta), Some(&i), Some(&j)) = (eta, cm.get(&pt[X]), cm.get(&pt[Z])) {
                        found.push(((i, eps), (j, eta)));
                    }
                }
            }
        }
        Pres::Dense => unreachable!(),
    }
    let save = phase(PH_REPLAY);
    for f in found.iter() {
        if replay_one(c, b, *f, rt) {
            // pair kept below after dedup
        } else {
            out.replay_fail += 1;
        }
    }
    phase(PH_DEDUP);
    for f in found {
        native_n(1);
        out.pairs.insert(f);
    }
    phase(save);
    out
}

pub fn solve_dense(c: &Curve, b: &Base, rt: Pt) -> Solved {
    let (r, _s) = rt;
    let p = c.p;
    let cm = colmap(b);
    let mut out = Solved::default();
    let save = phase(PH_BUILD);
    let sys = dense_sys(c, b, r);
    phase(save);
    let ex = extract(&sys, p, 2);
    merge_ex(&mut out, &ex);
    let nr = (r, fneg(rt.1, p));
    let save = phase(PH_REPLAY);
    let mut found: Vec<(SP, SP)> = Vec::new();
    for pt in &ex.points {
        out.roots_raw += 1;
        native_n(2);
        let (Some(&i), Some(&j)) = (cm.get(&pt[0]), cm.get(&pt[1])) else { continue };
        let p1 = (b.xs[i], b.ys[i]);
        let q1 = (b.xs[j], b.ys[j]);
        // sums with the two relative orientations
        let s1 = add(c, Some(p1), Some(q1)); // + +
        let nq = (q1.0, fneg(q1.1, p));
        let s2 = add(c, Some(p1), Some(nq)); // + -
        let mut hit = false;
        if s1 == Some(rt) {
            found.push(((i, 1), (j, 1)));
            hit = true;
        } else if s1 == Some(nr) {
            found.push(((i, -1), (j, -1)));
            hit = true;
        }
        if s2 == Some(rt) {
            found.push(((i, 1), (j, -1)));
            hit = true;
        } else if s2 == Some(nr) {
            found.push(((i, -1), (j, 1)));
            hit = true;
        }
        if !hit {
            out.replay_fail += 1;
        }
    }
    phase(PH_DEDUP);
    for f in found {
        native_n(1);
        out.pairs.insert(f);
    }
    phase(save);
    out
}

pub fn solve(c: &Curve, b: &Base, rt: Pt, pres: Pres) -> Solved {
    match pres {
        Pres::Dense => solve_dense(c, b, rt),
        _ => solve_secant(c, b, rt, pres),
    }
}

// ------------------------------------------------ uncharged exhaustive reference
fn pmul(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
fn ppow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = pmul(r, a, p);
        }
        a = pmul(a, a, p);
        e >>= 1;
    }
    r
}
fn pinv(a: u64, p: u64) -> u64 {
    ppow(a, p - 2, p)
}
pub fn ref_add(a: u64, p: u64, q1: Pt, q2: Pt) -> Option<Pt> {
    let ((x, y), (z, w)) = (q1, q2);
    if x == z && (y + w) % p == 0 {
        return None;
    }
    let m = if x == z {
        pmul((3 * pmul(x, x, p) + a) % p, pinv(2 * y % p, p), p)
    } else {
        pmul((w + p - y) % p, pinv((z + p - x) % p, p), p)
    };
    let rx = (pmul(m, m, p) + 2 * p - x - z) % p;
    let ry = (pmul(m, (x + p - rx) % p, p) + p - y) % p;
    Some((rx, ry))
}
pub fn ref_slope(a: u64, p: u64, q1: Pt, q2: Pt) -> u64 {
    if q1.0 == q2.0 {
        pmul((3 * pmul(q1.0, q1.0, p) + a) % p, pinv(2 * q1.1 % p, p), p)
    } else {
        pmul((q2.1 + p - q1.1) % p, pinv((q2.0 + p - q1.0) % p, p), p)
    }
}
/// All ordered signed pairs (P,Q) in the base with P+Q=R, with their secant slopes.
pub fn reference(c: &Curve, b: &Base, rt: Pt) -> Vec<((SP, SP), u64)> {
    let p = c.p;
    let k = b.xs.len();
    let mut out = Vec::new();
    for i in 0..k {
        for si in [1i8, -1] {
            for j in 0..k {
                for sj in [1i8, -1] {
                    let q1 = (b.xs[i], if si > 0 { b.ys[i] } else { (p - b.ys[i]) % p });
                    let q2 = (b.xs[j], if sj > 0 { b.ys[j] } else { (p - b.ys[j]) % p });
                    if ref_add(c.a, p, q1, q2) == Some(rt) {
                        out.push((((i, si), (j, sj)), ref_slope(c.a, p, q1, q2)));
                    }
                }
            }
        }
    }
    out
}
pub fn reference_set(c: &Curve, b: &Base, rt: Pt) -> BTreeSet<(SP, SP)> {
    reference(c, b, rt).into_iter().map(|x| x.0).collect()
}
