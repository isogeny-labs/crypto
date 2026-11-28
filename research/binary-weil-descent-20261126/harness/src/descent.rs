//! Arm D: the Gaudry–Diem k = 3 Weil-descent decomposition oracle for the
//! subfield factor base `x ∈ K' = F_{2^{n'}}` ⊂ `F_{2^n}`, `n = 3n'`.
//!
//! **The equation.**  For `y² + xy = x³ + ax² + b`, the fourth Semaev
//! polynomial written in the elementary symmetric functions
//! `e₁, e₂, e₃` of `x₁, x₂, x₃` is (crypto `koblitz_sym_s4.rs`, checked here
//! against the counted resultant quartic of the #5253 harness):
//!
//! ```text
//! S₄ = x⁴b² + e₃x³b + e₃²(x⁴ + b) + e₃³x + e₃⁴ + e₂²x²b + e₂²e₃x³
//!    + e₂⁴x⁴ + e₁²e₃xb + e₁²e₃²x² + e₁⁴b²               (x = x_R)
//! ```
//!
//! `e₁`, `e₂` occur only squared.  With `E₁ = e₁²`, `E₂ = e₂²`, `t = e₃`
//! (all in K' exactly when `e₁, e₂, e₃` are):
//!
//! ```text
//! f(E₁,E₂,t; b,x) = b²E₁² + x⁴E₂² + (bx·t + x²t²)E₁ + (bx² + x³t)E₂
//!                 + t⁴ + x t³ + (x⁴+b) t² + bx³ t + b²x⁴ .
//! ```
//!
//! **Weil descent.**  For `(E₁, E₂, t) ∈ K'³`, `f(·; b, x) = 0` is three
//! equations over K' (its coordinates on a K'-basis of `F_{2^n}`).  Over
//! `F_{2^n}` the same system is the three conjugate equations
//! `f_j = f(·; b^{σʲ}, x^{σʲ}) = 0`, `σ(z) = z^{q'}`, `j = 0, 1, 2`, which
//! this oracle uses (the K'-linear change between the two is invertible and
//! the K'-solutions are identical).
//!
//! **Elimination.**  In the unknown vector `(E₁², E₂², E₁, E₂, 1)` the
//! conjugate system is linear with coefficients in `F_{2^n}[t]` of degrees
//! `(0, 0, 2, 1, 4)`.  Constant row operations clear the two square columns
//! from two rows, leaving
//!
//! ```text
//! R0: c₀E₁² + α₁E₁ + β₁E₂ + γ₁ = 0
//! R1: c₁E₂² + α₂E₁ + β₂E₂ + γ₂ = 0
//! R2:         α₃E₁ + β₃E₂ + γ₃ = 0      (deg α ≤ 2, β ≤ 1, γ ≤ 4)
//! ```
//!
//! Eliminating `E₁` with R2 gives two quadratics in `E₂`,
//! `a₂E₂² + a₁E₂ + a₀` (from R1·α₃) and `b₂E₂² + b₁E₂ + b₀` (from R0·α₃²),
//! and their resultant
//!
//! ```text
//! U(t) = (a₂b₀ + a₀b₂)² + (a₂b₁ + a₁b₂)(a₁b₀ + a₀b₁),   deg U ≤ 20,
//! ```
//!
//! vanishes at the `t`-coordinate of every solution (each step only
//! multiplies, so `U(t) = 0` is necessary, also where `α₃(t) = 0`).
//!
//! **Subfield root finding.**  `U' = U / t^v`; its roots in K' are the roots
//! of `gcd(U', t^{q'} + t)`, by `n'` squarings modulo `U'` and one gcd, then
//! split over K' (Artin–Schreier for degree 2, the K'-trace for degree ≥ 3).
//! `t = 0` (a summand is the 2-torsion point T) is always solved directly.
//!
//! **Lifting.**  At each candidate `t = τ ∈ K'` the system is solved exactly
//! for `(E₁, E₂) ∈ K'²` by the case analysis in [`solve_at`] (no search);
//! then `e_i = √E_i`, and `X³ + e₁X² + e₂X + e₃` must split over K' with all
//! roots in the factor base.  Every field operation of all of this is
//! counted through [`CF`].  The only search is the never-expected
//! degenerate fallback (rank-deficient constant block, or `U ≡ 0`), which is
//! counted too and flagged in the record.
use crate::field::{Curve, CF};
use crate::ic::{Fb, Sub};

pub type Poly = Vec<u64>;

#[inline]
fn trim(p: &mut Poly) {
    while let Some(&0) = p.last() {
        p.pop();
    }
}
#[inline]
pub fn deg(p: &Poly) -> Option<usize> {
    if p.is_empty() {
        None
    } else {
        Some(p.len() - 1)
    }
}
pub fn padd(a: &Poly, b: &Poly) -> Poly {
    let mut r = vec![0u64; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() {
        r[i] ^= x;
    }
    for (i, &x) in b.iter().enumerate() {
        r[i] ^= x;
    }
    trim(&mut r);
    r
}
/// Schoolbook product; one `m` per product of two non-zero coefficients.
pub fn pmul(cf: &CF, a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            if y != 0 {
                r[i + j] ^= cf.mul(x, y);
            }
        }
    }
    trim(&mut r);
    r
}
pub fn pscale(cf: &CF, a: &Poly, c: u64) -> Poly {
    if c == 0 {
        return vec![];
    }
    if c == 1 {
        return a.clone();
    }
    let mut r: Poly = a.iter().map(|&x| if x == 0 { 0 } else { cf.mul(x, c) }).collect();
    trim(&mut r);
    r
}
/// Square: one `s` per non-zero coefficient (char 2: no cross terms).
pub fn psqr(cf: &CF, a: &Poly) -> Poly {
    if a.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; 2 * a.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x != 0 {
            r[2 * i] = cf.sqr(x);
        }
    }
    r
}
pub fn peval(cf: &CF, a: &Poly, t: u64) -> u64 {
    let mut v = 0;
    for (k, &c) in a.iter().enumerate().rev() {
        v = if k + 1 == a.len() { c } else { cf.mul(v, t) ^ c };
    }
    v
}
/// `a mod f`, `f` monic of degree `df ≥ 1`.
pub fn prem_monic(cf: &CF, a: &Poly, f: &Poly) -> Poly {
    let df = f.len() - 1;
    let mut r = a.clone();
    while r.len() > df {
        let dd = r.len() - 1;
        let lead = r[dd];
        if lead != 0 {
            let sh = dd - df;
            for j in 0..df {
                if f[j] != 0 {
                    r[sh + j] ^= cf.mul(f[j], lead);
                }
            }
        }
        r.pop();
    }
    trim(&mut r);
    r
}
pub fn monic(cf: &CF, a: &Poly) -> Poly {
    let l = *a.last().unwrap();
    if l == 1 {
        return a.clone();
    }
    let il = cf.inv(l);
    let mut r: Poly = a[..a.len() - 1].iter().map(|&x| if x == 0 { 0 } else { cf.mul(x, il) }).collect();
    r.push(1);
    r
}
/// Monic gcd by Euclid with monic normalisation (one inversion per step).
pub fn pgcd(cf: &CF, a: &Poly, b: &Poly) -> Poly {
    let (mut a, mut b) = (a.clone(), b.clone());
    trim(&mut a);
    trim(&mut b);
    if a.is_empty() {
        return if b.is_empty() { b } else { monic(cf, &b) };
    }
    a = monic(cf, &a);
    while !b.is_empty() {
        let bm = monic(cf, &b);
        let r = prem_monic(cf, &a, &bm);
        a = bm;
        b = r;
    }
    a
}

/// What a K'-root search did (for the record).
#[derive(Clone, Copy, Debug, Default)]
pub struct DStats {
    pub targets: u64,
    pub deg_u_sum: u64,
    pub deg_u_max: u64,
    pub kroots_u: u64,
    pub solves: u64,
    pub ksolutions: u64,
    pub nonsplit: u64,
    pub split_not_fb: u64,
    pub decompositions: u64,
    pub fallback_rank: u64,
    pub fallback_uzero: u64,
    pub fallback_quartic_zero: u64,
    pub degenerate_r2: u64,
    pub tau0_mult: u64,
}

pub struct Descent<'a> {
    pub c: &'a Curve,
    pub sub: &'a Sub,
    pub np: u32,
    /// `b^{σʲ}` and `(b^{σʲ})²`, j = 0, 1, 2 (target-independent setup).
    pub bj: [u64; 3],
    pub bj2: [u64; 3],
    pub st: DStats,
    /// Root-finding route for U': `gcd(U', t^{q'} + t)` (default), or the
    /// Frobenius-conjugate gcd `gcd(U', σ(U'))` (cross-check only: U' is in
    /// K'[t] up to a scalar, so that gcd is U' itself and gains nothing).
    pub frob_gcd: bool,
}

/// K'-roots (distinct) of `p` over `F_{2^n}`; `None` if `p ≡ 0`.
pub fn kroots(c: &Curve, cf: &CF, sub: &Sub, np: u32, p: &Poly) -> Option<Vec<u64>> {
    let mut p = p.clone();
    trim(&mut p);
    if p.is_empty() {
        return None;
    }
    let d = p.len() - 1;
    if d == 0 {
        return Some(vec![]);
    }
    let f = monic(cf, &p);
    if d == 1 {
        let r = f[0];
        return Some(if sub.contains(cf, r) { vec![r] } else { vec![] });
    }
    // h = t^{q'} mod f: start at the largest monomial t^{2^k} of degree < d.
    let mut k = 0u32;
    while k < np && (1usize << (k + 1)) < d {
        k += 1;
    }
    let mut h = vec![0u64; (1usize << k) + 1];
    h[1 << k] = 1;
    for _ in k..np {
        h = prem_monic(cf, &psqr(cf, &h), &f);
    }
    let hx = padd(&h, &vec![0, 1]);
    let g = pgcd_pseudo(cf, &f, &hx);
    let mut out = vec![];
    split_k(c, cf, sub, np, &g, 0, &mut out);
    out.sort_unstable();
    Some(out)
}

/// Roots of a monic squarefree `g` that splits over K' into linear factors.
fn split_k(c: &Curve, cf: &CF, sub: &Sub, np: u32, g: &Poly, depth: usize, out: &mut Vec<u64>) {
    let d = g.len() - 1;
    match d {
        0 => {}
        1 => out.push(g[0]),
        2 => {
            // t² + p t + r
            let (p, r) = (g[1], g[0]);
            if p == 0 {
                cf.lin();
                out.push(c.asv.sqrt(r));
            } else {
                // t = p z, z² + z = r/p²
                let ip = cf.inv(p);
                let w = cf.mul(r, cf.sqr(ip));
                cf.lin();
                let z = c.asv.solve(w);
                let t0 = cf.mul(p, z);
                out.push(t0);
                out.push(t0 ^ p);
            }
        }
        _ => {
            // Trace splitting: Tr_{K'/F2}(β t) mod g for β over the K' basis.
            assert!(depth < sub.basis.len() + 2, "trace splitting did not separate the roots");
            let beta = sub.basis[depth % sub.basis.len()];
            let mut acc: Poly = vec![0, beta];
            let mut cur: Poly = vec![0, beta];
            for _ in 1..np {
                cur = prem_monic(cf, &psqr(cf, &cur), g);
                acc = padd(&acc, &cur);
            }
            let acc = prem_monic(cf, &acc, g);
            let g1 = pgcd(cf, g, &acc);
            let dg1 = g1.len().saturating_sub(1);
            if g1.is_empty() || dg1 == 0 || dg1 == d {
                split_k(c, cf, sub, np, g, depth + 1, out);
                return;
            }
            // g2 = g / g1 (exact): the roots with trace 1.
            let g2 = pdiv_exact(cf, g, &g1);
            split_k(c, cf, sub, np, &g1, depth + 1, out);
            split_k(c, cf, sub, np, &g2, depth + 1, out);
        }
    }
}

/// `a / b` for monic `b` dividing `a`.
pub fn pdiv_exact(cf: &CF, a: &Poly, b: &Poly) -> Poly {
    let db = b.len() - 1;
    let mut r = a.clone();
    let mut q = vec![0u64; a.len() - db];
    while r.len() > db {
        let dd = r.len() - 1;
        let lead = r[dd];
        q[dd - db] = lead;
        if lead != 0 {
            for j in 0..db {
                if b[j] != 0 {
                    r[dd - db + j] ^= cf.mul(b[j], lead);
                }
            }
        }
        r.pop();
    }
    let mut r2 = r;
    trim(&mut r2);
    debug_assert!(r2.is_empty(), "inexact division");
    trim(&mut q);
    q
}

/// One row of the conjugate system: coefficients of `E₁², E₂²` (constants)
/// and of `E₁, E₂, 1` (polynomials in t).
#[derive(Clone, Debug)]
struct Row {
    s1: u64,
    s2: u64,
    e1: Poly,
    e2: Poly,
    k: Poly,
}

fn row_comb(cf: &CF, u: u64, r: &Row, v: u64, w: &Row) -> Row {
    // u·r + v·w
    Row {
        s1: cf.mul(u, r.s1) ^ cf.mul(v, w.s1),
        s2: cf.mul(u, r.s2) ^ cf.mul(v, w.s2),
        e1: padd(&pscale(cf, &r.e1, u), &pscale(cf, &w.e1, v)),
        e2: padd(&pscale(cf, &r.e2, u), &pscale(cf, &w.e2, v)),
        k: padd(&pscale(cf, &r.k, u), &pscale(cf, &w.k, v)),
    }
}

/// The reduced system and its resultant for one target.
pub struct Reduced {
    pub c0: u64,
    pub c1: u64,
    pub a: [Poly; 3],
    pub b: [Poly; 3],
    pub g: [Poly; 3],
    pub u: Poly,
    pub rank_ok: bool,
}

impl<'a> Descent<'a> {
    pub fn new(c: &'a Curve, cf: &CF, sub: &'a Sub, np: u32) -> Self {
        let b0 = c.b;
        let b1 = cf.sqr_k(b0, np);
        let b2 = cf.sqr_k(b1, np);
        let bj = [b0, b1, b2];
        let bj2 = [cf.sqr(b0), cf.sqr(b1), cf.sqr(b2)];
        Descent { c, sub, np, bj, bj2, st: DStats::default(), frob_gcd: false }
    }

    /// The symmetric S₄ (module doc) at `(E₁, E₂, t)`, conjugate `j`.
    pub fn f_at(&self, cf: &CF, j: usize, xj: u64, e1: u64, e2: u64, t: u64) -> u64 {
        let (b, b2) = (self.bj[j], self.bj2[j]);
        let x2 = cf.sqr(xj);
        let x4 = cf.sqr(x2);
        let x3 = cf.mul(x2, xj);
        let t2 = cf.sqr(t);
        let mut v = cf.mul(b2, cf.sqr(e1)) ^ cf.mul(x4, cf.sqr(e2));
        v ^= cf.mul(e1, cf.mul(t, cf.mul(b, xj) ^ cf.mul(x2, t)));
        v ^= cf.mul(e2, cf.mul(b, x2) ^ cf.mul(x3, t));
        v ^= cf.sqr(t2) ^ cf.mul(xj, cf.mul(t2, t)) ^ cf.mul(x4 ^ b, t2) ^ cf.mul(cf.mul(b, x3), t) ^ cf.mul(b2, x4);
        v
    }

    /// Conjugates, rows, constant-block elimination and `U(t)`.
    pub fn reduce(&self, cf: &CF, xr: u64) -> Reduced {
        let np = self.np;
        let x0 = xr;
        let x1 = cf.sqr_k(x0, np);
        let x2 = cf.sqr_k(x1, np);
        let xs = [x0, x1, x2];
        let mut rows: Vec<Row> = Vec::with_capacity(3);
        for j in 0..3 {
            let (b, b2, x) = (self.bj[j], self.bj2[j], xs[j]);
            let xx2 = cf.sqr(x);
            let xx4 = cf.sqr(xx2);
            let xx3 = cf.mul(xx2, x);
            let bx = cf.mul(b, x);
            let bx2 = cf.mul(b, xx2);
            let bx3 = cf.mul(b, xx3);
            let b2x4 = cf.mul(b2, xx4);
            let mut e1 = vec![0, bx, xx2];
            trim(&mut e1);
            let mut e2 = vec![bx2, xx3];
            trim(&mut e2);
            let mut k = vec![b2x4, bx3, xx4 ^ b, x, 1];
            trim(&mut k);
            rows.push(Row { s1: b2, s2: xx4, e1, e2, k });
        }
        // Clear E₁² from rows 1, 2 with row 0 (b² ≠ 0 since b ≠ 0).
        let r0 = rows[0].clone();
        let r1 = row_comb(cf, r0.s1, &rows[1], rows[1].s1, &r0);
        let r2 = row_comb(cf, r0.s1, &rows[2], rows[2].s1, &r0);
        // Pivot for E₂² among r1, r2.
        let (p, q) = if r1.s2 != 0 { (r1, r2) } else { (r2, r1) };
        let rank_ok = p.s2 != 0;
        // R2 = p.s2·q + q.s2·p (no squares); R0 = p.s2·r0 + r0.s2·p (no E₂²).
        let rr2 = row_comb(cf, p.s2, &q, q.s2, &p);
        let rr0 = row_comb(cf, p.s2, &r0, r0.s2, &p);
        let (c0, c1) = (rr0.s1, p.s2);
        let (al1, be1, ga1) = (rr0.e1, rr0.e2, rr0.k);
        let (al2, be2, ga2) = (p.e1.clone(), p.e2.clone(), p.k.clone());
        let (al3, be3, ga3) = (rr2.e1, rr2.e2, rr2.k);
        // Quadratics in E₂ after E₁ = (β₃E₂ + γ₃)/α₃.
        let a2 = pscale(cf, &al3, c1);
        let a1 = padd(&pmul(cf, &al2, &be3), &pmul(cf, &al3, &be2));
        let a0 = padd(&pmul(cf, &al2, &ga3), &pmul(cf, &al3, &ga2));
        let al13 = pmul(cf, &al1, &al3);
        let al33 = psqr(cf, &al3);
        let b2 = pscale(cf, &psqr(cf, &be3), c0);
        let b1 = padd(&pmul(cf, &al13, &be3), &pmul(cf, &al33, &be1));
        let b0 = padd(&padd(&pscale(cf, &psqr(cf, &ga3), c0), &pmul(cf, &al13, &ga3)), &pmul(cf, &al33, &ga1));
        let s = padd(&pmul(cf, &a2, &b0), &pmul(cf, &a0, &b2));
        let l = padd(&pmul(cf, &a2, &b1), &pmul(cf, &a1, &b2));
        let r = padd(&pmul(cf, &a1, &b0), &pmul(cf, &a0, &b1));
        let u = padd(&psqr(cf, &s), &pmul(cf, &l, &r));
        Reduced { c0, c1, a: [al1, al2, al3], b: [be1, be2, be3], g: [ga1, ga2, ga3], u, rank_ok }
    }

    /// Every `(E₁, E₂) ∈ K'²` solving the reduced system at `t = τ`.
    fn solve_at(&mut self, cf: &CF, red: &Reduced, tau: u64, out: &mut Vec<(u64, u64)>) {
        let (c, sub, np) = (self.c, self.sub, self.np);
        let ev = |p: &Poly| peval(cf, p, tau);
        let (a1, a2, a3) = (ev(&red.a[0]), ev(&red.a[1]), ev(&red.a[2]));
        let (b1, b2, b3) = (ev(&red.b[0]), ev(&red.b[1]), ev(&red.b[2]));
        let (g1, g2, g3) = (ev(&red.g[0]), ev(&red.g[1]), ev(&red.g[2]));
        let (c0, c1) = (red.c0, red.c1);
        // Candidate list, each checked against R0, R1, R2 and K' membership.
        let mut cand: Vec<(u64, u64)> = vec![];
        if a3 != 0 {
            // E₁ = (β₃E₂ + γ₃)/α₃ ; c₁α₃E₂² + (α₂β₃ + α₃β₂)E₂ + (α₂γ₃ + α₃γ₂) = 0.
            let q2 = cf.mul(c1, a3);
            let q1 = cf.mul(a2, b3) ^ cf.mul(a3, b2);
            let q0 = cf.mul(a2, g3) ^ cf.mul(a3, g2);
            let ia3 = cf.inv(a3);
            if let Some(rs) = kroots(c, cf, sub, np, &vec![q0, q1, q2]) {
                for e2 in rs {
                    let e1 = cf.mul(cf.mul(b3, e2) ^ g3, ia3);
                    cand.push((e1, e2));
                }
            }
        } else if b3 != 0 {
            self.st.degenerate_r2 += 1;
            let e2 = cf.mul(g3, cf.inv(b3));
            let k1 = cf.mul(c1, cf.sqr(e2)) ^ cf.mul(b2, e2) ^ g2;
            if a2 != 0 {
                cand.push((cf.mul(k1, cf.inv(a2)), e2));
            } else if k1 == 0 {
                let k0 = cf.mul(b1, e2) ^ g1;
                if let Some(rs) = kroots(c, cf, sub, np, &vec![k0, a1, c0]) {
                    for e1 in rs {
                        cand.push((e1, e2));
                    }
                }
            }
        } else if g3 == 0 {
            self.st.degenerate_r2 += 1;
            if a2 != 0 {
                // E₁ = (c₁E₂² + β₂E₂ + γ₂)/α₂ into R0·α₂²:
                // c₀(c₁²E₂⁴ + β₂²E₂² + γ₂²) + α₁α₂(c₁E₂² + β₂E₂ + γ₂) + α₂²(β₁E₂ + γ₁)
                let a12 = cf.mul(a1, a2);
                let a22 = cf.sqr(a2);
                let p4 = cf.mul(c0, cf.sqr(c1));
                let p2 = cf.mul(c0, cf.sqr(b2)) ^ cf.mul(a12, c1);
                let p1 = cf.mul(a12, b2) ^ cf.mul(a22, b1);
                let p0 = cf.mul(c0, cf.sqr(g2)) ^ cf.mul(a12, g2) ^ cf.mul(a22, g1);
                let ia2 = cf.inv(a2);
                match kroots(c, cf, sub, np, &vec![p0, p1, p2, 0, p4]) {
                    Some(rs) => {
                        for e2 in rs {
                            let e1 = cf.mul(cf.mul(c1, cf.sqr(e2)) ^ cf.mul(b2, e2) ^ g2, ia2);
                            cand.push((e1, e2));
                        }
                    }
                    None => {
                        self.st.fallback_quartic_zero += 1;
                        for &e2 in &sub.span {
                            let e1 = cf.mul(cf.mul(c1, cf.sqr(e2)) ^ cf.mul(b2, e2) ^ g2, ia2);
                            cand.push((e1, e2));
                        }
                    }
                }
            } else {
                // R1: c₁E₂² + β₂E₂ + γ₂ = 0, then R0 quadratic in E₁.
                if let Some(rs2) = kroots(c, cf, sub, np, &vec![g2, b2, c1]) {
                    for e2 in rs2 {
                        let k0 = cf.mul(b1, e2) ^ g1;
                        if let Some(rs1) = kroots(c, cf, sub, np, &vec![k0, a1, c0]) {
                            for e1 in rs1 {
                                cand.push((e1, e2));
                            }
                        }
                    }
                }
            }
        }
        for (e1, e2) in cand {
            if !sub.contains(cf, e1) || !sub.contains(cf, e2) {
                continue;
            }
            let r0 = cf.mul(c0, cf.sqr(e1)) ^ cf.mul(a1, e1) ^ cf.mul(b1, e2) ^ g1;
            let r1 = cf.mul(c1, cf.sqr(e2)) ^ cf.mul(a2, e1) ^ cf.mul(b2, e2) ^ g2;
            let r2 = cf.mul(a3, e1) ^ cf.mul(b3, e2) ^ g3;
            if r0 == 0 && r1 == 0 && r2 == 0 && !out.contains(&(e1, e2)) {
                out.push((e1, e2));
            }
        }
    }

    /// `{x₁, x₂, x₃}` (sorted multisets, x_i in the factor base) with
    /// `S₄(x₁, x₂, x₃, x_R) = 0`.  `cf_red` is charged the descent and
    /// elimination, `cf_rf` the subfield root finding of `U`, `cf_lift` the
    /// solve at each root, the square roots and the cubic split.
    pub fn decompose(&mut self, cf_red: &CF, cf_rf: &CF, cf_lift: &CF, fb: &Fb, xr: u64, out: &mut Vec<[u64; 3]>) {
        out.clear();
        self.st.targets += 1;
        let (c, sub, np) = (self.c, self.sub, self.np);
        let red = self.reduce(cf_red, xr);
        let mut taus: Vec<u64> = vec![];
        let mut sols: Vec<(u64, u64, u64)> = vec![];
        if !red.rank_ok {
            // x_R²/b ∈ K': the constant block has rank < 2.  Counted search
            // over t ∈ K' with the conjugate equations checked directly.
            self.st.fallback_rank += 1;
            self.fallback_full(cf_lift, xr, &mut sols);
        } else {
            let mut u = red.u.clone();
            trim(&mut u);
            if u.is_empty() {
                self.st.fallback_uzero += 1;
                taus.extend(sub.span.iter().copied());
            } else {
                let v = u.iter().position(|&x| x != 0).unwrap();
                let mut up: Poly = u[v..].to_vec();
                // α₃ = t(λ + μt) divides U (module doc); its root τ₀ = λ/μ is
                // solved directly like t = 0 and divided out of U'.
                let al3 = &red.a[2];
                let mut tau0 = None;
                if al3.len() == 3 && al3[0] == 0 && al3[1] != 0 {
                    let t0 = cf_rf.mul(al3[1], cf_rf.inv(al3[2]));
                    if sub.contains(cf_rf, t0) {
                        tau0 = Some(t0);
                        loop {
                            // synthetic division by (t + τ₀)
                            let dd = up.len() - 1;
                            if dd == 0 {
                                break;
                            }
                            let mut q = vec![0u64; dd];
                            let mut acc = up[dd];
                            q[dd - 1] = acc;
                            for i in (1..dd).rev() {
                                acc = up[i] ^ cf_rf.mul(acc, t0);
                                q[i - 1] = acc;
                            }
                            let rem = up[0] ^ cf_rf.mul(acc, t0);
                            if rem != 0 {
                                break;
                            }
                            self.st.tau0_mult += 1;
                            up = q;
                        }
                    }
                }
                let du = (up.len() - 1) as u64;
                self.st.deg_u_sum += du;
                self.st.deg_u_max = self.st.deg_u_max.max(du);
                let rs = if self.frob_gcd { kroots_frob(c, cf_rf, sub, np, &up) } else { kroots(c, cf_rf, sub, np, &up).unwrap() };
                self.st.kroots_u += rs.len() as u64;
                taus.push(0);
                if let Some(t0) = tau0 {
                    if t0 != 0 {
                        taus.push(t0);
                    }
                }
                taus.extend(rs.into_iter().filter(|&r| r != 0 && Some(r) != tau0));
            }
            for &tau in &taus {
                self.st.solves += 1;
                let mut ee = vec![];
                self.solve_at(cf_lift, &red, tau, &mut ee);
                for (e1, e2) in ee {
                    sols.push((e1, e2, tau));
                }
            }
        }
        for (e1s, e2s, e3) in sols {
            self.st.ksolutions += 1;
            // e_i = √E_i (one table map each)
            cf_lift.lin();
            let e1 = c.asv.sqrt(e1s);
            cf_lift.lin();
            let e2 = c.asv.sqrt(e2s);
            let Some(ms) = split_cubic(c, cf_lift, sub, np, e1, e2, e3) else {
                self.st.nonsplit += 1;
                continue;
            };
            let mut ok = true;
            for &x in &ms {
                cf_lift.lin();
                if !fb.idx.contains_key(&x) {
                    ok = false;
                }
            }
            if !ok {
                self.st.split_not_fb += 1;
                continue;
            }
            let mut t = ms;
            t.sort_unstable();
            out.push(t);
        }
        out.sort_unstable();
        out.dedup();
        self.st.decompositions += out.len() as u64;
    }

    /// Degenerate-case fallback (never expected): for every `t ∈ K'`, solve
    /// conjugate equation 0 for `E₁` per `E₂ ∈ K'` would be `q'²`; instead
    /// use equations 0 and 1 as two quadrics in `(E₁, E₂)` with E₂
    /// enumerated: `q'²` evaluations.  Counted.
    fn fallback_full(&mut self, cf: &CF, xr: u64, sols: &mut Vec<(u64, u64, u64)>) {
        let np = self.np;
        let x1 = cf.sqr_k(xr, np);
        let x2 = cf.sqr_k(x1, np);
        let xs = [xr, x1, x2];
        let span = self.sub.span.clone();
        for &t in &span {
            for &e2 in &span {
                // f_0 is quadratic in E₁: b²E₁² + A E₁ + rest.
                let b2 = self.bj2[0];
                let x = xr;
                let xx2 = cf.sqr(x);
                let a = cf.mul(t, cf.mul(self.bj[0], x) ^ cf.mul(xx2, t));
                let rest = self.f_at(cf, 0, x, 0, e2, t);
                if let Some(rs) = kroots(self.c, cf, self.sub, np, &vec![rest, a, b2]) {
                    for e1 in rs {
                        if (1..3).all(|j| self.f_at(cf, j, xs[j], e1, e2, t) == 0) {
                            sols.push((e1, e2, t));
                        }
                    }
                }
            }
        }
    }
}

/// The roots of `X³ + e₁X² + e₂X + e₃` as a multiset, if it splits over K'.
pub fn split_cubic(c: &Curve, cf: &CF, sub: &Sub, np: u32, e1: u64, e2: u64, e3: u64) -> Option<[u64; 3]> {
    let rs = kroots(c, cf, sub, np, &vec![e3, e2, e1, 1]).unwrap();
    let cands: Vec<[u64; 3]> = match rs.len() {
        3 => vec![[rs[0], rs[1], rs[2]]],
        2 => vec![[rs[0], rs[0], rs[1]], [rs[0], rs[1], rs[1]]],
        1 => vec![[rs[0], rs[0], rs[0]]],
        _ => vec![],
    };
    for m in cands {
        let s1 = m[0] ^ m[1] ^ m[2];
        let p01 = cf.mul(m[0], m[1]);
        let s2 = p01 ^ cf.mul(m[0] ^ m[1], m[2]);
        let s3 = cf.mul(p01, m[2]);
        if s1 == e1 && s2 == e2 && s3 == e3 {
            return Some(m);
        }
    }
    None
}

/// `σ(p)`: every coefficient raised to `q' = 2^{n'}` (n' squarings each).
pub fn conj(cf: &CF, p: &Poly, np: u32) -> Poly {
    p.iter().map(|&x| if x == 0 || x == 1 { x } else { cf.sqr_k(x, np) }).collect()
}

/// gcd by pseudo-remainders (no inversion until the final monic step).
pub fn pgcd_pseudo(cf: &CF, a: &Poly, b: &Poly) -> Poly {
    let (mut a, mut b) = (a.clone(), b.clone());
    trim(&mut a);
    trim(&mut b);
    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }
    while !b.is_empty() {
        let db = b.len() - 1;
        if db == 0 {
            return vec![1];
        }
        let lb = b[db];
        let mut r = a;
        while r.len() > db {
            let dd = r.len() - 1;
            let lead = r[dd];
            let sh = dd - db;
            for j in 0..dd {
                if r[j] != 0 && lb != 1 {
                    r[j] = cf.mul(lb, r[j]);
                }
            }
            for j in 0..db {
                if b[j] != 0 {
                    r[sh + j] ^= cf.mul(lead, b[j]);
                }
            }
            r.pop();
            trim(&mut r);
        }
        a = b;
        b = r;
    }
    if a.is_empty() {
        a
    } else {
        monic(cf, &a)
    }
}

/// K'-roots of `p` by the descent's Frobenius structure: every K'-root of
/// `p` is a root of `σ(p)`, so `g = gcd(p, σ(p))` carries them all; `g` is
/// almost always of degree ≤ 1 and is finished by [`kroots`].
pub fn kroots_frob(c: &Curve, cf: &CF, sub: &Sub, np: u32, p: &Poly) -> Vec<u64> {
    let sp = conj(cf, p, np);
    let g = pgcd_pseudo(cf, p, &sp);
    if g.len() <= 1 {
        return vec![];
    }
    kroots(c, cf, sub, np, &g).unwrap()
}
