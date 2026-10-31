//! The quartic-extension genus-five cover as an explicit twisted Humbert curve.
//!
//! E / F_{q^4}:  y^2 = eta (x - a)(x - a^q)(x - a^{q^2}),  a of degree 4.
//! (h1 of degree one is the same family after the F_q-Moebius map x -> 1/(x - beta).)
//!
//! Model of the descended curve C / F_q (w_4 = 1 affine chart):
//!     w_i^2 = eta_i (x - lam_i),   eta_i = eta^{q^i},  lam_i = a^{q^i},  i = 0..3,
//! with F_q-Frobenius  Phi(x, w_0, w_1, w_2, w_3) = (x^q, w_3^q, w_0^q, w_1^q, w_2^q).
//! In P^4 this is the intersection of three diagonal quadrics (Humbert curve,
//! genus 5, branch points {lam_0..lam_3, oo}).  The degree-8 map to E is
//!     pi_0(x, w) = (x, w_0 w_1 w_2 / mu),  mu^2 = eta_1 eta_2.
//! Rational points: x in F_q, w_0 in F_{q^4}, w_0^2 = eta (x - a), w_i = w_0^{q^i}.
use crate::curve::{factor, group_order, Curve, Pt, Rng};
use crate::field::{addm, legendre, mulm, subm, FieldOps, Gfk, E4, EL, K4, L};
use crate::poly::{self, Interp};

pub struct Setup {
    pub p: u64,
    pub l: L,
    pub eta: E4,
    pub etas: [E4; 4],
    pub lams: [E4; 4],
    pub mu: E4,
    pub chi_eta: i32,
    pub gamma: E4,
    /// tconj[j][i] = (t^j)^{p^i}
    pub tconj: [[E4; 4]; 4],
    pub a2: E4,
    pub a4: E4,
    pub a6: E4,
    pub order: u128,
    pub r: u64,
    pub d_count: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct GPt {
    pub x: EL,
    pub w: [EL; 4],
}

impl Setup {
    pub fn k(&self) -> &K4 {
        &self.l.k
    }
    pub fn sqrt_k4(&self, a: &E4) -> Option<E4> {
        let s = self.l.sqrt(&self.l.emb(a))?;
        if L::in_k4(&s) {
            Some(s.0)
        } else {
            None
        }
    }
    pub fn curve(&self) -> Curve<'_, K4> {
        Curve { f: self.k(), a2: self.a2, a4: self.a4, a6: self.a6 }
    }

    /// Build a random instance: alpha of degree 4, eta with quadratic character
    /// `twist`, and r = the largest prime factor of #E(F_{p^4}) (required: cofactor <= 128, r^2 not | #E, r not | #D).
    pub fn new(p: u64, twist: i32, rng: &mut Rng) -> Setup {
        assert!(p >= 17 && p < (1 << 15));
        loop {
            let m: Vec<u64> = vec![rng.below(p), rng.below(p), rng.below(p), rng.below(p), 1];
            if !poly::is_irreducible_quartic(&m, p) {
                continue;
            }
            let k = K4::new(p, [m[0], m[1], m[2], m[3]]);
            let nu = loop {
                let c = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                if k.chi(&c) == -1 {
                    break c;
                }
            };
            let l = L::new(k.clone(), nu);
            let eta = loop {
                let c = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                if c != [0; 4] && k.chi(&c) == twist {
                    break c;
                }
            };
            let alpha = [0, 1, 0, 0];
            let mut lams = [alpha; 4];
            let mut etas = [eta; 4];
            for i in 1..4 {
                lams[i] = k.frob(&lams[i - 1]);
                etas[i] = k.frob(&etas[i - 1]);
            }
            let mu_sq = k.mul(&etas[1], &etas[2]);
            let mu_l = l.sqrt(&l.emb(&mu_sq)).unwrap();
            assert!(L::in_k4(&mu_l), "eta_1 eta_2 must be a square in F_{{q^4}}");
            let mu = mu_l.0;
            // F_{p^2} generator inside K4
            let mut gamma = k.add(&alpha, &lams[2]);
            if K4::is_fp(&gamma) {
                gamma = k.add(&k.mul(&alpha, &alpha), &k.mul(&lams[2], &lams[2]));
            }
            assert!(!K4::is_fp(&gamma) && k.frob_n(&gamma, 2) == gamma);
            let mut tconj = [[[0u64; 4]; 4]; 4];
            let mut tj = k.one();
            for j in 0..4 {
                tconj[j][0] = tj;
                for i in 1..4 {
                    tconj[j][i] = k.frob(&tconj[j][i - 1]);
                }
                tj = k.mul(&tj, &alpha);
            }
            let e: Vec<E4> = (0..3).map(|i| k.mul(&eta, &lams[i])).collect();
            let a2 = k.neg(&k.add(&k.add(&e[0], &e[1]), &e[2]));
            let a4 = k.add(&k.add(&k.mul(&e[0], &e[1]), &k.mul(&e[0], &e[2])), &k.mul(&e[1], &e[2]));
            let a6 = k.neg(&k.mul(&k.mul(&e[0], &e[1]), &e[2]));
            let mut s = Setup {
                p,
                l,
                eta,
                etas,
                lams,
                mu,
                chi_eta: twist,
                gamma,
                tconj,
                a2,
                a4,
                a6,
                order: 0,
                r: 0,
                d_count: 0,
            };
            let order = {
                let c = s.curve();
                let sq = |a: &E4| s.sqrt_k4(a);
                group_order(&c, &sq, rng)
            };
            let fs = factor(order);
            let (r, e) = *fs.last().unwrap();
            s.d_count = s.count_d();
            if r * 128 < (p as u128).pow(4) || e != 1 || r >= (1u128 << 62) || s.d_count as u128 % r == 0 {
                continue;
            }
            s.order = order;
            s.r = r as u64;
            return s;
        }
    }

    /// #D(F_p) for the extra factor D: y^2 = N(eta) m_alpha(x) (smooth quartic model).
    pub fn count_d(&self) -> u64 {
        let k = self.k();
        let p = self.p;
        let ne = k.norm(&self.eta);
        let mut n: i64 = 1 + legendre(ne, p) as i64; // points at infinity
        for x in 0..p {
            let mx = k.norm(&k.sub(&[x, 0, 0, 0], &self.lams[0])); // = m_alpha(x)
            n += 1 + legendre(mulm(ne, mx, p), p) as i64;
        }
        n as u64
    }

    pub fn phi(&self, q: &GPt) -> GPt {
        let l = &self.l;
        GPt {
            x: l.frob(&q.x),
            w: [l.frob(&q.w[3]), l.frob(&q.w[0]), l.frob(&q.w[1]), l.frob(&q.w[2])],
        }
    }
    pub fn on_c(&self, q: &GPt) -> bool {
        let l = &self.l;
        (0..4).all(|i| {
            let rhs = l.mul(&l.emb(&self.etas[i]), &l.sub(&q.x, &l.emb(&self.lams[i])));
            l.mul(&q.w[i], &q.w[i]) == rhs
        })
    }
    /// The 12 basis functions of L(2 D_oo) evaluated at a geometric point.
    pub fn basis12(&self, q: &GPt) -> [EL; 12] {
        let l = &self.l;
        let k = self.k();
        let mut g = [l.zero(); 12];
        g[0] = l.one();
        g[1] = q.x;
        for j in 0..4 {
            let mut s1 = l.zero();
            let mut s2 = l.zero();
            for i in 0..4 {
                let c = l.emb(&self.tconj[j][i]);
                s1 = l.add(&s1, &l.mul(&c, &q.w[i]));
                s2 = l.add(&s2, &l.mul(&c, &l.mul(&q.w[i], &q.w[(i + 1) % 4])));
            }
            g[2 + j] = s1;
            g[6 + j] = s2;
        }
        let w02 = l.mul(&q.w[0], &q.w[2]);
        let w13 = l.mul(&q.w[1], &q.w[3]);
        for (idx, d) in [k.one(), self.gamma].iter().enumerate() {
            let dp = k.frob(d);
            g[10 + idx] = l.add(&l.mul(&l.emb(d), &w02), &l.mul(&l.emb(&dp), &w13));
        }
        g
    }
}

/// Factor base: rational points of C, paired by the involution iota: (x, w) -> (x, -w).
pub struct FactorBase {
    pub xs: Vec<u64>,
    /// w_0 of the "+" point for each x
    pub w0: Vec<E4>,
    /// x -> index in xs (or usize::MAX)
    pub xidx: Vec<usize>,
    /// basis12 rows (F_p values) of the + and - points: rows[2j], rows[2j+1]
    pub rows: Vec<[u64; 12]>,
    /// x-indices that relation and descent searches draw their chosen points from
    /// (all of them, or the small factor base of a large-prime arm)
    pub pick: Vec<usize>,
}

impl FactorBase {
    pub fn build(s: &Setup) -> FactorBase {
        let k = s.k();
        let p = s.p;
        let mut xs = Vec::new();
        let mut w0s = Vec::new();
        let mut xidx = vec![usize::MAX; p as usize];
        let mut rows = Vec::new();
        for x in 0..p {
            let z = k.mul(&s.eta, &k.sub(&[x, 0, 0, 0], &s.lams[0]));
            if k.chi(&z) != 1 {
                continue;
            }
            let mut w = s.sqrt_k4(&z).unwrap();
            let wn = k.neg(&w);
            if wn < w {
                w = wn;
            }
            xidx[x as usize] = xs.len();
            xs.push(x);
            w0s.push(w);
            for sign in [false, true] {
                let w0 = if sign { k.neg(&w) } else { w };
                let q = rational_point(s, x, &w0);
                debug_assert!(s.on_c(&q) && phi_eq(s, &q));
                let b = s.basis12(&q);
                let mut row = [0u64; 12];
                for c in 0..12 {
                    assert!(L::in_fp(&b[c]), "basis function not F_p-valued at a rational point");
                    row[c] = b[c].0[0];
                }
                rows.push(row);
            }
        }
        let pick = (0..xs.len()).collect();
        FactorBase { xs, w0: w0s, xidx, rows, pick }
    }
    pub fn n_points(&self) -> usize {
        self.rows.len()
    }
}

pub fn rational_point(s: &Setup, x: u64, w0: &E4) -> GPt {
    let k = s.k();
    let l = &s.l;
    let mut w = [l.zero(); 4];
    let mut c = *w0;
    for i in 0..4 {
        w[i] = l.emb(&c);
        c = k.frob(&c);
    }
    GPt { x: l.from_u64(x), w }
}
fn phi_eq(s: &Setup, q: &GPt) -> bool {
    let f = s.phi(q);
    f.x == q.x && f.w == q.w
}

/// Precomputed square roots at the interpolation nodes x_t = 0..=16.
pub struct Nodes {
    pub n: usize,
    pub r: Vec<[EL; 4]>,
    pub rr: Vec<[EL; 6]>, // r0r1, r1r2, r2r3, r3r0, r0r2, r1r3
    pub interp8: Interp,
    pub interp16: Interp,
}
impl Nodes {
    pub fn new(s: &Setup) -> Nodes {
        let l = &s.l;
        let k = s.k();
        let n = 16;
        let mut r = Vec::new();
        let mut rr = Vec::new();
        for t in 0..=n as u64 {
            let z = k.mul(&s.eta, &k.sub(&[t, 0, 0, 0], &s.lams[0]));
            let r0 = l.sqrt(&l.emb(&z)).unwrap();
            let mut ri = [r0; 4];
            for i in 1..4 {
                ri[i] = l.frob(&ri[i - 1]);
            }
            rr.push([
                l.mul(&ri[0], &ri[1]),
                l.mul(&ri[1], &ri[2]),
                l.mul(&ri[2], &ri[3]),
                l.mul(&ri[3], &ri[0]),
                l.mul(&ri[0], &ri[2]),
                l.mul(&ri[1], &ri[3]),
            ]);
            r.push(ri);
        }
        Nodes { n, r, rr, interp8: Interp::new(8, s.p), interp16: Interp::new(16, s.p) }
    }
}

/// Norm to F_p(x) of G = sum c_k g_k, as a polynomial in x (degree <= 16, or <= 8
/// when only the hyperplane coordinates 0, 2..5 are used).
pub fn norm_poly(s: &Setup, nodes: &Nodes, c: &[u64; 12], hyper: bool) -> Vec<u64> {
    let l = &s.l;
    let k = s.k();
    let p = s.p;
    let mut b = [k.zero(); 4];
    let mut cc = [k.zero(); 4];
    for i in 0..4 {
        for j in 0..4 {
            b[i] = k.add(&b[i], &k.scal(c[2 + j], &s.tconj[j][i]));
            cc[i] = k.add(&cc[i], &k.scal(c[6 + j], &s.tconj[j][i]));
        }
    }
    let d0 = k.add(&[c[10], 0, 0, 0], &k.scal(c[11], &s.gamma));
    let d1 = k.frob(&d0);
    let npts = if hyper { 8 } else { 16 };
    let mut vals = Vec::with_capacity(npts + 1);
    for t in 0..=npts {
        let r = &nodes.r[t];
        let rr = &nodes.rr[t];
        let br: Vec<EL> = (0..4).map(|i| l.mul(&l.emb(&b[i]), &r[i])).collect();
        let a = addm(c[0], mulm(c[1], t as u64, p), p);
        let (cr, d0r, d1r) = if hyper {
            (vec![l.zero(); 4], l.zero(), l.zero())
        } else {
            (
                (0..4).map(|i| l.mul(&l.emb(&cc[i]), &rr[i])).collect::<Vec<_>>(),
                l.mul(&l.emb(&d0), &rr[4]),
                l.mul(&l.emb(&d1), &rr[5]),
            )
        };
        let mut prod = l.one();
        for e in 0..8u32 {
            let eps = [1i32, if e & 1 != 0 { -1 } else { 1 }, if e & 2 != 0 { -1 } else { 1 }, if e & 4 != 0 { -1 } else { 1 }];
            let mut v = l.zero();
            for i in 0..4 {
                v = if eps[i] > 0 { l.add(&v, &br[i]) } else { l.sub(&v, &br[i]) };
            }
            let mut u = l.from_u64(a);
            if !hyper {
                for i in 0..4 {
                    let sg = eps[i] * eps[(i + 1) % 4];
                    u = if sg > 0 { l.add(&u, &cr[i]) } else { l.sub(&u, &cr[i]) };
                }
                u = if eps[0] * eps[2] > 0 { l.add(&u, &d0r) } else { l.sub(&u, &d0r) };
                u = if eps[1] * eps[3] > 0 { l.add(&u, &d1r) } else { l.sub(&u, &d1r) };
            }
            let f = l.sub(&l.mul(&u, &u), &l.mul(&v, &v));
            prod = l.mul(&prod, &f);
        }
        assert!(L::in_fp(&prod), "norm value not in F_p: Frobenius structure broken");
        vals.push(prod.0[0]);
    }
    if hyper {
        nodes.interp8.interpolate(&vals, p)
    } else {
        nodes.interp16.interpolate(&vals, p)
    }
}

pub fn eval_row(row: &[u64; 12], c: &[u64; 12], p: u64) -> u64 {
    let mut s = 0;
    for i in 0..12 {
        s = addm(s, mulm(row[i], c[i], p), p);
    }
    s
}

/// Nullspace over F_p (rows x ncols).
pub fn kernel_fp(rows: &[Vec<u64>], ncols: usize, p: u64) -> Vec<Vec<u64>> {
    let mut m: Vec<Vec<u64>> = rows.to_vec();
    let mut piv = Vec::new();
    let mut r = 0;
    for c in 0..ncols {
        let Some(pr) = (r..m.len()).find(|&i| m[i][c] != 0) else { continue };
        m.swap(r, pr);
        let inv = crate::field::invm(m[r][c], p);
        for j in 0..ncols {
            m[r][j] = mulm(m[r][j], inv, p);
        }
        for i in 0..m.len() {
            if i != r && m[i][c] != 0 {
                let f = m[i][c];
                for j in 0..ncols {
                    m[i][j] = subm(m[i][j], mulm(f, m[r][j], p), p);
                }
            }
        }
        piv.push(c);
        r += 1;
        if r == m.len() {
            break;
        }
    }
    let free: Vec<usize> = (0..ncols).filter(|c| !piv.contains(c)).collect();
    let mut out = Vec::new();
    for &f in &free {
        let mut v = vec![0u64; ncols];
        v[f] = 1;
        for (ri, &pc) in piv.iter().enumerate() {
            v[pc] = (p - m[ri][f]) % p;
        }
        out.push(v);
    }
    out
}

/// One relation: FB point indices (with multiplicity 1 each) and the D_oo coefficient.
#[derive(Clone, Debug)]
pub struct Relation {
    pub pts: Vec<usize>,
    pub inf: i64,
    pub sym: bool,
}

#[derive(Default)]
pub struct Stats {
    pub candidates: u64,
    pub degenerate: u64,
    pub found: u64,
    pub symmetric: u64,
    /// number of F_p roots of the non-degenerate residual (0..=4)
    pub root_hist: [u64; 5],
}

/// Identify the zero of G in the fibre over a root x (must be a rational point).
fn identify(fb: &FactorBase, c: &[u64; 12], x: u64, p: u64) -> Option<usize> {
    let j = fb.xidx[x as usize];
    if j == usize::MAX {
        return None;
    }
    let zp = eval_row(&fb.rows[2 * j], c, p) == 0;
    let zm = eval_row(&fb.rows[2 * j + 1], c, p) == 0;
    match (zp, zm) {
        (true, false) => Some(2 * j),
        (false, true) => Some(2 * j + 1),
        _ => None,
    }
}

/// Hyperplane relation search: hyperplane through 4 random FB points (distinct x),
/// residual degree-4 divisor must split into rational points.  Relation:
/// sum of the 8 points - D_oo ~ 0.  A hyperplane with a4 = 0 is iota-invariant
/// and its section is the 4 chosen iota-pairs (flagged `sym`: no Res(E) content).
pub fn hyperplane_relation(s: &Setup, fb: &FactorBase, nodes: &Nodes, rng: &mut Rng, st: &mut Stats) -> Option<Relation> {
    let p = s.p;
    st.candidates += 1;
    let mut chosen: Vec<usize> = Vec::new();
    while chosen.len() < 4 {
        let j = fb.pick[rng.below(fb.pick.len() as u64) as usize];
        if chosen.iter().any(|&c| c / 2 == j) {
            continue;
        }
        chosen.push(2 * j + (rng.next() & 1) as usize);
    }
    let hyp_cols = [0usize, 2, 3, 4, 5];
    let rows: Vec<Vec<u64>> = chosen.iter().map(|&i| hyp_cols.iter().map(|&c| fb.rows[i][c]).collect()).collect();
    let ker = kernel_fp(&rows, 5, p);
    if ker.len() != 1 {
        st.degenerate += 1;
        return None;
    }
    let mut c = [0u64; 12];
    for (t, &col) in hyp_cols.iter().enumerate() {
        c[col] = ker[0][t];
    }
    if c[0] == 0 {
        st.symmetric += 1;
        let mut pts = Vec::new();
        for &i in &chosen {
            debug_assert_eq!(eval_row(&fb.rows[i ^ 1], &c, p), 0);
            pts.push(i & !1);
            pts.push(i | 1);
        }
        return Some(Relation { pts, inf: -1, sym: true });
    }
    let n = norm_poly(s, nodes, &c, true);
    if poly::deg(&n) != 8 {
        st.degenerate += 1;
        return None;
    }
    let mut known = vec![1u64];
    for &i in &chosen {
        let x = fb.xs[i / 2];
        known = poly::mul(&known, &[(p - x) % p, 1], p);
    }
    let (res, rm) = poly::divrem(&n, &known, p);
    assert!(rm.is_empty(), "hyperplane does not vanish on its defining points");
    let nroots = poly::deg(&poly::gcd(&poly::sub(&poly::powmod(&[0, 1], p as u128, &res, p), &[0, 1], p), &res, p)).max(0) as usize;
    st.root_hist[nroots.min(4)] += 1;
    if !poly::splits_distinct(&res, p) {
        return None;
    }
    if chosen.iter().any(|&i| poly::eval(&res, fb.xs[i / 2], p) == 0) {
        st.degenerate += 1;
        return None;
    }
    let mut pts = chosen.clone();
    for x in poly::roots(&res, p) {
        pts.push(identify(fb, &c, x, p).expect("simple F_p root of the residual must be a rational point"));
    }
    st.found += 1;
    Some(Relation { pts, inf: -1, sym: false })
}

/// Quadric relation search: a function of L(2 D_oo) through 11 random FB points
/// (distinct x); the residual degree-5 divisor must split.  Relation:
/// sum of the 16 points - 2 D_oo ~ 0.
pub fn quadric_relation(s: &Setup, fb: &FactorBase, nodes: &Nodes, rng: &mut Rng, st: &mut Stats) -> Option<Relation> {
    let p = s.p;
    st.candidates += 1;
    let mut chosen: Vec<usize> = Vec::new();
    while chosen.len() < 11 {
        let j = fb.pick[rng.below(fb.pick.len() as u64) as usize];
        if chosen.iter().any(|&c| c / 2 == j) {
            continue;
        }
        chosen.push(2 * j + (rng.next() & 1) as usize);
    }
    let rows: Vec<Vec<u64>> = chosen.iter().map(|&i| fb.rows[i].to_vec()).collect();
    let ker = kernel_fp(&rows, 12, p);
    if ker.len() != 1 {
        st.degenerate += 1;
        return None;
    }
    let mut c = [0u64; 12];
    c.copy_from_slice(&ker[0]);
    let n = norm_poly(s, nodes, &c, false);
    if poly::deg(&n) != 16 {
        st.degenerate += 1;
        return None;
    }
    let mut known = vec![1u64];
    for &i in &chosen {
        known = poly::mul(&known, &[(p - fb.xs[i / 2]) % p, 1], p);
    }
    let (res, rm) = poly::divrem(&n, &known, p);
    assert!(rm.is_empty(), "quadric does not vanish on its defining points");
    if !poly::splits_distinct(&res, p) {
        return None;
    }
    if chosen.iter().any(|&i| poly::eval(&res, fb.xs[i / 2], p) == 0) {
        st.degenerate += 1;
        return None;
    }
    let mut pts = chosen.clone();
    for x in poly::roots(&res, p) {
        pts.push(identify(fb, &c, x, p)?);
    }
    st.found += 1;
    Some(Relation { pts, inf: -2, sym: false })
}

/// The 8 geometric points of pi_0^*(P) and their Frobenius orbits (the F_q-places of T(P)).
/// `pt_xy` is the Weierstrass point (X, Y) of Y^2 = prod(X - eta lam_i).
pub fn transfer_places(s: &Setup, pt_xy: (E4, E4)) -> Vec<Vec<GPt>> {
    let l = &s.l;
    let k = s.k();
    // Weierstrass (X, Y) -> eta-model (x, y) = (X / eta, Y / eta)
    let ei = k.inv(&s.eta);
    let (xp, yp) = (k.mul(&pt_xy.0, &ei), k.mul(&pt_xy.1, &ei));
    let mut rho = [l.zero(); 4];
    for i in 0..4 {
        let z = k.mul(&s.etas[i], &k.sub(&xp, &s.lams[i]));
        rho[i] = l.sqrt(&l.emb(&z)).unwrap();
    }
    let target = l.emb(&k.mul(&s.mu, &yp));
    let mut b = Vec::new();
    for e in 0..8u32 {
        let e0 = e & 1 != 0;
        let e1 = e & 2 != 0;
        let e3 = e & 4 != 0;
        let w0 = if e0 { l.neg(&rho[0]) } else { rho[0] };
        let w1 = if e1 { l.neg(&rho[1]) } else { rho[1] };
        let w3 = if e3 { l.neg(&rho[3]) } else { rho[3] };
        let w01 = l.mul(&w0, &w1);
        let w2 = if l.mul(&w01, &rho[2]) == target {
            rho[2]
        } else {
            assert_eq!(l.mul(&w01, &l.neg(&rho[2])), target, "pi_0 fibre inconsistent");
            l.neg(&rho[2])
        };
        let q = GPt { x: l.emb(&xp), w: [w0, w1, w2, w3] };
        debug_assert!(s.on_c(&q));
        b.push(q);
    }
    let same = |a: &GPt, b: &GPt| a.x == b.x && a.w == b.w;
    let mut places: Vec<Vec<GPt>> = Vec::new();
    for q in &b {
        if places.iter().any(|z| z.iter().any(|y| same(y, q))) {
            continue;
        }
        let mut orb = vec![*q];
        let mut cur = s.phi(q);
        while !same(&cur, q) {
            orb.push(cur);
            cur = s.phi(&cur);
            assert!(orb.len() <= 16);
        }
        places.push(orb);
    }
    places
}

/// Pushforward check: pi_{0*} T(P) = 8 P, computed on E over L point by point.
pub fn pushforward_check(s: &Setup, places: &[Vec<GPt>], pt_xy: (E4, E4)) -> bool {
    let l = &s.l;
    let k = s.k();
    let ce = Curve { f: l, a2: l.emb(&s.a2), a4: l.emb(&s.a4), a6: l.emb(&s.a6) };
    let mu_inv = l.emb(&k.inv(&s.mu));
    let eta = l.emb(&s.eta);
    let mut acc: Pt<EL> = None;
    let mut npts = 0;
    for z in places {
        for q in z {
            let y = l.mul(&l.mul(&l.mul(&q.w[0], &q.w[1]), &q.w[2]), &mu_inv);
            let pt = Some((l.mul(&eta, &q.x), l.mul(&eta, &y)));
            assert!(ce.on_curve(&pt));
            acc = ce.add(&acc, &pt);
            npts += 1;
        }
    }
    assert_eq!(npts, 32);
    let p8 = ce.mul(&Some((l.emb(&pt_xy.0), l.emb(&pt_xy.1))), 8);
    acc == p8
}

/// Quadric descent of one place Z of degree ell: a function in L(2 D_oo) through
/// Z and 11 - ell random FB points whose residual degree-5 divisor splits.
/// Returns the FB points of the relation  Z ~ 2 D_oo - sum(FB points).
/// `has_log(i)` says whether FB point i has a log (every point, or in a large-prime
/// arm the small base plus the large primes solved from partial relations).
pub fn descend_place(s: &Setup, fb: &FactorBase, nodes: &Nodes, z: &[GPt], has_log: &dyn Fn(usize) -> bool, rng: &mut Rng, st: &mut Stats) -> Vec<usize> {
    let p = s.p;
    let k = s.k();
    let l = &s.l;
    let ell = z.len();
    let g = s.basis12(&z[0]);
    let mut zrows: Vec<Vec<u64>> = (0..8).map(|t| (0..12).map(|c| L::coords(&g[c])[t]).collect()).collect();
    // m_Z(x) = prod over the place's points of (x - x_Q), in F_p[x]
    let mut mz: Vec<EL> = vec![l.one()];
    for q in z {
        let mut nm = vec![l.zero(); mz.len() + 1];
        for (i, c) in mz.iter().enumerate() {
            nm[i + 1] = l.add(&nm[i + 1], c);
            nm[i] = l.sub(&nm[i], &l.mul(c, &q.x));
        }
        mz = nm;
    }
    let mz: Vec<u64> = mz
        .iter()
        .map(|c| {
            assert!(L::in_fp(c));
            c.0[0]
        })
        .collect();
    let _ = k;
    zrows.retain(|r| r.iter().any(|&v| v != 0));
    loop {
        st.candidates += 1;
        let mut chosen: Vec<usize> = Vec::new();
        while chosen.len() < 11 - ell {
            let j = fb.pick[rng.below(fb.pick.len() as u64) as usize];
            if chosen.iter().any(|&c| c / 2 == j) {
                continue;
            }
            chosen.push(2 * j + (rng.next() & 1) as usize);
        }
        let mut rows = zrows.clone();
        for &i in &chosen {
            rows.push(fb.rows[i].to_vec());
        }
        let ker = kernel_fp(&rows, 12, p);
        if ker.len() != 1 {
            st.degenerate += 1;
            continue;
        }
        let mut c = [0u64; 12];
        c.copy_from_slice(&ker[0]);
        let n = norm_poly(s, nodes, &c, false);
        if poly::deg(&n) != 16 {
            st.degenerate += 1;
            continue;
        }
        let mut known = mz.clone();
        for &i in &chosen {
            known = poly::mul(&known, &[(p - fb.xs[i / 2]) % p, 1], p);
        }
        let (res, rm) = poly::divrem(&n, &known, p);
        assert!(rm.is_empty(), "descent function does not vanish on Z and the chosen points");
        if !poly::splits_distinct(&res, p) {
            continue;
        }
        if chosen.iter().any(|&i| poly::eval(&res, fb.xs[i / 2], p) == 0) {
            continue;
        }
        let mut pts = chosen.clone();
        let mut ok = true;
        for x in poly::roots(&res, p) {
            match identify(fb, &c, x, p) {
                Some(i) if has_log(i) => pts.push(i),
                _ => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        st.found += 1;
        assert_eq!(pts.len(), 16 - ell);
        return pts;
    }
}

/// Point counts N_k(C) for k = 1..5 on the explicit model (no use of the
/// predicted decomposition), plus the points at infinity.
pub fn count_c(s: &Setup, kmax: usize, rng: &mut Rng) -> Vec<u64> {
    let k = s.k();
    let p = s.p;
    let chi = s.chi_eta;
    let ne = k.norm(&s.eta);
    let mut out = Vec::new();
    for kk in 1..=kmax {
        // points at infinity: sign classes eps with Phi^k(eps) = +-eps, Phi(eps) = (chi e3, e0, e1, e2)
        let mut inf = 0u64;
        for e in 0..8u32 {
            let eps: [i32; 4] = [1, if e & 1 != 0 { -1 } else { 1 }, if e & 2 != 0 { -1 } else { 1 }, if e & 4 != 0 { -1 } else { 1 }];
            let mut f = eps;
            for _ in 0..kk {
                f = [chi * f[3], f[0], f[1], f[2]];
            }
            if f == eps || f.iter().zip(eps.iter()).all(|(a, b)| *a == -*b) {
                inf += 1;
            }
        }
        let mut aff: u64 = 0;
        match kk {
            1 | 3 | 5 => {
                // one cycle of length 4; chi over F_{p^{4k}} = legendre(N(eta)^k N_{p^k/p}(m(x)))
                let gk = if kk == 1 {
                    None
                } else {
                    let m = loop {
                        let mut m: Vec<u64> = (0..kk).map(|_| rng.below(p)).collect();
                        m.push(1);
                        if poly::is_irreducible_prime_deg(&m, kk, p) {
                            break m;
                        }
                    };
                    Some(Gfk { p, k: kk, m })
                };
                let mcoef = {
                    // m_alpha(x) = t^4 + m3 t^3 + .. + m0 (the K4 modulus)
                    let mm = k.m;
                    vec![mm[0], mm[1], mm[2], mm[3], 1]
                };
                let nek = crate::field::powm(ne, kk as u128, p);
                let total = (p as u128).pow(kk as u32) as u64;
                for idx in 0..total {
                    let nm = match &gk {
                        None => poly::eval(&mcoef, idx, p),
                        Some(g) => {
                            let mut x = vec![0u64; kk];
                            let mut v = idx;
                            for c in x.iter_mut() {
                                *c = v % p;
                                v /= p;
                            }
                            // Horner for m(x) in F_{p^k}
                            let mut acc = vec![0u64; kk];
                            for &co in mcoef.iter().rev() {
                                acc = g.mul(&acc, &x);
                                acc[0] = addm(acc[0], co, p);
                            }
                            g.norm(&acc)
                        }
                    };
                    let ch = legendre(mulm(nek, nm, p), p);
                    aff += (1 + ch) as u64;
                }
            }
            2 => {
                for a in 0..p {
                    for b in 0..p {
                        let x = k.add(&[a, 0, 0, 0], &k.scal(b, &s.gamma));
                        let c0 = k.chi(&k.mul(&s.etas[0], &k.sub(&x, &s.lams[0])));
                        let c1 = k.chi(&k.mul(&s.etas[1], &k.sub(&x, &s.lams[1])));
                        aff += ((1 + c0) * (1 + c1)) as u64;
                    }
                }
            }
            4 => {
                for idx in 0..(p as u64).pow(4) {
                    let x = [idx % p, (idx / p) % p, (idx / p / p) % p, idx / p / p / p];
                    let mut prod = 1u64;
                    for i in 0..4 {
                        let c = k.chi(&k.mul(&s.etas[i], &k.sub(&x, &s.lams[i])));
                        prod *= (1 + c) as u64;
                        if prod == 0 {
                            break;
                        }
                    }
                    aff += prod;
                }
            }
            _ => unreachable!(),
        }
        out.push(aff + inf);
    }
    let _ = subm;
    out
}

/// L-polynomial coefficients a_0..a_{2g} from N_1..N_g (Newton + functional equation).
pub fn lpoly_from_counts(p: u64, counts: &[u64], g: usize) -> Vec<i128> {
    let p = p as i128;
    let s: Vec<i128> = (1..=g).map(|k| p.pow(k as u32) + 1 - counts[k - 1] as i128).collect();
    let mut a = vec![0i128; 2 * g + 1];
    a[0] = 1;
    for j in 1..=g {
        let mut acc = 0i128;
        for i in 1..=j {
            acc += s[i - 1] * a[j - i];
        }
        assert!(acc % j as i128 == 0, "Newton identity not integral");
        a[j] = -acc / j as i128;
    }
    for j in 0..g {
        a[2 * g - j] = p.pow((g - j) as u32) * a[j];
    }
    a
}
