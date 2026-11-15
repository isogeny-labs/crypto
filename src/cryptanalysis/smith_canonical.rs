//! The canonical map `psi: X -> C` of Smith's trigonal construction, built
//! natively, and the transfer `Psi = psi_* . phi: J_H -> J_C` to the Jacobian
//! of a smooth plane quartic.
//!
//! Source: B. Smith, "Isogenies and the discrete logarithm problem in
//! Jacobians of genus 3 hyperelliptic curves", J. Cryptology 22 (2009), §6-§7.
//! [`smith_trigonal`](super::smith_trigonal) builds Recillas' curve `X` and the
//! correspondence `R`; Smith then obtains the plane quartic `C` from Magma
//! ("a basis of the Riemann-Roch space of a canonical divisor"). This module
//! replaces that step. Study: `research/smith_canonical_20261115/`.
//!
//! # Route: push the differentials of `H` through the correspondence
//!
//! The task brief suggested computing the adjoint space of the plane model
//! `Phi(t, rho) = delta_4 rho^4 + delta_2 rho^2 - delta_1 rho + delta_0`.
//! That model has `t`-degree 10 and many non-ordinary singularities. It is
//! not needed. `phi = R_*: J_H -> J_X` is an isogeny with kernel of order 8,
//! prime to `p`, hence separable, hence an isomorphism on cotangent spaces:
//!
//! ```text
//! H^0(X, Omega) = R_* H^0(H, Omega) = < R_*(x^i dx / y) : i = 0, 1, 2 >.
//! ```
//!
//! A point `Q` of `X` over `t` is a triple `{(x_j, y_j)}` with `G(t, x_j) = 0`
//! and `y_j = b(x_j)`, `b = b0 + b1 x + b2 x^2` the Mumford polynomial of the
//! `R`-triple (`b2 = rho`). The trace of `x^i dx / y` along `R` is
//!
//! ```text
//! omega_i(Q) = sum_j x_j^i dx_j / y_j,    dx_j = dt / g'(x_j) = D(x_j) dt / G_x(t, x_j),
//! 1 / y_j = b(x_j) / F(x_j),
//! ```
//!
//! so `omega_i = h_i dt` with `h_i = Tr( x^i D b / (F G_x) )`, the trace of
//! `F_p(t)[x]/G` over `F_p(t)`. By Euler's formula `Tr(x^k / G_x)` is 0, 0, 1
//! for `k = 0, 1, 2` (`G` monic cubic), so with `M = D adj(f) mod G`
//! (`f = F mod G`, `f adj(f) = Norm(f) = s`, a scalar common to all `i`):
//!
//! ```text
//! h_i = [x^2] ( x^i M b mod G ) = sum_k T_ik(t) b_k,   T_ik = [x^2]( x^(i+k) M mod G ).
//! ```
//!
//! Hence the canonical map is linear in the Veronese point of `X`:
//!
//! ```text
//! psi(t; b) = T(t) (b0 : b1 : b2) = T(t) (b02 : b12 : b22),   T in F_p[t]^(3x3) symmetric.
//! ```
//!
//! `b` and `-b` are the same point of `X` and give the same image. `T` is
//! [`psi_matrix`]. The quartic form of `C` is found by linear algebra on the
//! 15 quartic monomials at the images of the points of `X` over `U`
//! ([`fit_quartic`]); the fit is accepted only if the images span the plane,
//! lie on no conic (`X` not hyperelliptic) and on exactly one quartic, with
//! at least 17 distinct points (a second quartic through 17 points of an
//! irreducible quartic is a multiple of it).
//!
//! What this argument assumes and what code checks: the argument uses
//! Smith's theorem that `phi` is an isogeny. Code checks the consequences:
//! independence of the three `h_i`, the unique quartic, its smoothness,
//! `#C(F_p^k) = #H(F_p^k)`, and that `Psi` respects linear equivalence.
//!
//! # The transfer
//!
//! For an affine point `P` of `H` over `U`, `E(P) = psi(Q1) + psi(Q2)` with
//! `{Q1, Q2}` = [`Trigonal::image_points`]; the two points are rational or
//! conjugate over `F_p^2`. The class of the rational divisor is built without
//! leaving `F_p`: the conics through `Q1 + Q2 + P0` form a 3-dimensional
//! `F_p`-space cut out by the real and imaginary parts of the conic monomials
//! at `Q1` plus the values at `P0`; `PlaneQuartic::class_from_conics` turns it
//! into the class `[Q1 + Q2 + P0 - D_inf] = [Q1 + Q2 + 2 P0 - K]`. When
//! `Q1, Q2, P0` are collinear or not distinct, another rational point `A` is
//! used and `[A - P0]` subtracted. The constant `2 P0 - K` cancels in every
//! degree-0 combination ([`Transfer::e_class`], [`Transfer::point_sum`]).
//!
//! An arbitrary class of `J_H(F_p)` given in Mumford form is moved onto
//! rational points over `U` first ([`Transfer::transfer_divisor`]): with
//! random sums `S1, S2` of point differences such that `D + S1 = R1 + R2 + R3
//! - 3 inf` and `S2 = R1' + R2' + R3' - 3 inf` split over `F_p` inside `U`,
//! `D = sum R_i - sum R_i' + S2 - S1` exactly, so no point at infinity and no
//! extension-field point of `H` is needed.
//!
//! # The `(t, rho)` input
//!
//! `Phi(t, rho) = delta_4 rho^4 + delta_2 rho^2 - delta_1 rho + delta_0 = 0`
//! is a plane model of `X`, birational only. [`CanonicalMap::lifts_t_rho`]
//! recovers `b0, b1` from `(t, rho)` as the common roots of `c1` and `c0`
//! after eliminating `b0` with `c2`. At a node of the plane model two points
//! of `X` share `(t, rho)` (2 of the 60 image points of Example 1); both
//! lifts are returned and [`CanonicalMap::psi_t_rho`] refuses to choose.
//!
//! # What is verified
//!
//! Unit tests (Example 1, `F_37`): `#C(F_37^k) = #H(F_37^k) = 42, 1342,
//! 50070` (the printed zeta numerator); the fitted quartic is
//! `F_37`-projectively equivalent to the printed one (exhaustive search finds
//! the projectivity); classes of conjugate pairs against line sections;
//! principal divisors of `H` map to zero and perturbed ones do not; `Psi` of
//! independent representations agree; rational 2-torsion is killed exactly
//! on `S`; a planted `k = 22359` is recovered on `C`. A perturbed `T` fails
//! the fit. The frozen sweep over random Smith-positive curves is in
//! `research/smith_canonical_20261115/RESULTS.md`.
//!
//! # Scope
//!
//! `psi` is evaluated on `X|U` only (rational or `F_p^2` coordinates; `t`
//! may be in `F_p^2` for [`CanonicalMap::psi_b`] and
//! [`CanonicalMap::psi_t_rho`]). The fibres of `X` over `t = infinity` and
//! outside `U` are never evaluated; the point counts of `C` cover them.
//! The Jacobian arithmetic of `H` ([`HJac`]) is for `deg F = 7`. Point
//! counting and root finding are brute force: toy primes only.

use std::cell::RefCell;
use std::collections::HashMap;

use super::plane_quartic_pic::{
    first_rational_point_canonical, mons_of_degree, smoothness_certificate, PicClass, PicKey,
    PlaneQuartic, Pt,
};
use super::smith_prop24::{chi, deg, inv_mod, peval, Poly, SplitMix64};
use super::smith_trigonal::{Fp2, Trigonal, XPoint, F2};

// ---------------------------------------------------------------------------
// F_p and F_p[x] helpers.

#[inline]
fn md(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
#[inline]
fn ad(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
#[inline]
fn sb(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
#[inline]
fn ng(a: u64, p: u64) -> u64 {
    (p - a % p) % p
}

fn trim(a: &mut Poly) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn padd(a: &[u64], b: &[u64], p: u64) -> Poly {
    let n = a.len().max(b.len());
    let mut r: Poly = (0..n)
        .map(|i| ad(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0), p))
        .collect();
    trim(&mut r);
    r
}

fn psub(a: &[u64], b: &[u64], p: u64) -> Poly {
    let n = a.len().max(b.len());
    let mut r: Poly = (0..n)
        .map(|i| sb(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0), p))
        .collect();
    trim(&mut r);
    r
}

fn pscale(a: &[u64], c: u64, p: u64) -> Poly {
    let mut r: Poly = a.iter().map(|&x| md(x, c, p)).collect();
    trim(&mut r);
    r
}

fn pmul(a: &[u64], b: &[u64], p: u64) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = ad(r[i + j], md(x, y, p), p);
        }
    }
    trim(&mut r);
    r
}

fn pmonic(a: &[u64], p: u64) -> Poly {
    let mut a = a.to_vec();
    trim(&mut a);
    match a.last() {
        Some(&l) if l != 1 => pscale(&a, inv_mod(l, p), p),
        _ => a,
    }
}

/// Quotient and remainder (`b` nonzero).
fn pdivrem(a: &[u64], b: &[u64], p: u64) -> (Poly, Poly) {
    let mut r = a.to_vec();
    trim(&mut r);
    let mut b = b.to_vec();
    trim(&mut b);
    assert!(!b.is_empty(), "division by the zero polynomial");
    if r.len() < b.len() {
        return (Vec::new(), r);
    }
    let il = inv_mod(b[b.len() - 1], p);
    let mut q = vec![0u64; r.len() - b.len() + 1];
    for i in (0..q.len()).rev() {
        let c = md(r[i + b.len() - 1], il, p);
        q[i] = c;
        if c != 0 {
            for (j, &bj) in b.iter().enumerate() {
                r[i + j] = sb(r[i + j], md(c, bj, p), p);
            }
        }
    }
    trim(&mut q);
    trim(&mut r);
    (q, r)
}

fn prem(a: &[u64], b: &[u64], p: u64) -> Poly {
    pdivrem(a, b, p).1
}

/// `(g, s, t)` with `g = s a + t b` monic (`g = 0` only if `a = b = 0`).
fn pxgcd(a: &[u64], b: &[u64], p: u64) -> (Poly, Poly, Poly) {
    let (mut r0, mut r1) = (a.to_vec(), b.to_vec());
    trim(&mut r0);
    trim(&mut r1);
    let (mut s0, mut s1): (Poly, Poly) = (vec![1], Vec::new());
    let (mut t0, mut t1): (Poly, Poly) = (Vec::new(), vec![1]);
    while !r1.is_empty() {
        let (q, r) = pdivrem(&r0, &r1, p);
        let s2 = psub(&s0, &pmul(&q, &s1, p), p);
        let t2 = psub(&t0, &pmul(&q, &t1, p), p);
        r0 = r1;
        r1 = r;
        s0 = s1;
        s1 = s2;
        t0 = t1;
        t1 = t2;
    }
    if let Some(&l) = r0.last() {
        let il = inv_mod(l, p);
        (pscale(&r0, il, p), pscale(&s0, il, p), pscale(&t0, il, p))
    } else {
        (r0, s0, t0)
    }
}

/// The roots in `F_p` of a nonzero polynomial, with multiplicity collapsed
/// (brute force).
fn proots(a: &[u64], p: u64) -> Vec<u64> {
    (0..p).filter(|&x| peval(a, x, p) == 0).collect()
}

/// A basis of the kernel of `rows` (vectors of length `n`) over `F_p`.
fn nullspace(rows: &[Vec<u64>], n: usize, p: u64) -> Vec<Vec<u64>> {
    let mut m: Vec<Vec<u64>> = rows.to_vec();
    let mut piv = Vec::new();
    let mut r = 0;
    for c in 0..n {
        let Some(pr) = (r..m.len()).find(|&i| m[i][c] != 0) else {
            continue;
        };
        m.swap(r, pr);
        let iv = inv_mod(m[r][c], p);
        for v in m[r].iter_mut() {
            *v = md(*v, iv, p);
        }
        let prow = m[r].clone();
        for (i, row) in m.iter_mut().enumerate() {
            if i != r && row[c] != 0 {
                let f = row[c];
                for (v, &w) in row.iter_mut().zip(&prow) {
                    *v = sb(*v, md(f, w, p), p);
                }
            }
        }
        piv.push(c);
        r += 1;
    }
    let mut out = Vec::new();
    for fc in (0..n).filter(|c| !piv.contains(c)) {
        let mut v = vec![0u64; n];
        v[fc] = 1;
        for (i, &pc) in piv.iter().enumerate() {
            v[pc] = ng(m[i][fc], p);
        }
        out.push(v);
    }
    out
}

// ---------------------------------------------------------------------------
// The matrix T(t) of the canonical map.

type Tx = [Poly; 3];

/// `a x mod G` for `a` in `F_p[t][x]/G`.
fn tx_mul_x(a: &Tx, g: &[Poly; 3], p: u64) -> Tx {
    [
        psub(&[], &pmul(&a[2], &g[0], p), p),
        psub(&a[0], &pmul(&a[2], &g[1], p), p),
        psub(&a[1], &pmul(&a[2], &g[2], p), p),
    ]
}

fn tx_mul(a: &Tx, b: &Tx, g: &[Poly; 3], p: u64) -> Tx {
    let mut acc: Tx = [Vec::new(), Vec::new(), Vec::new()];
    let mut cur = a.clone();
    for bk in b {
        for i in 0..3 {
            acc[i] = padd(&acc[i], &pmul(&cur[i], bk, p), p);
        }
        cur = tx_mul_x(&cur, g, p);
    }
    acc
}

/// The symmetric matrix `T(t)` with `psi(t; b) = T(t) (b0 : b1 : b2)`:
/// `T_ik = [x^2]( x^(i+k) D adj(f) mod G )` (module doc).
pub fn psi_matrix(tr: &Trigonal) -> [[Poly; 3]; 3] {
    let p = tr.p;
    let g = &tr.g;
    let f: Tx = tr.fpoly.clone();
    // Columns of the multiplication-by-f matrix on (1, x, x^2).
    let c0 = f.clone();
    let c1 = tx_mul_x(&c0, g, p);
    let c2 = tx_mul_x(&c1, g, p);
    let m = |i: usize, k: usize| -> &Poly { [&c0, &c1, &c2][k].get(i).unwrap() };
    // adj(f) = adj(M) e_0: entry i is the cofactor of (0, i).
    let minor = |r0: usize, r1: usize, k0: usize, k1: usize| {
        psub(
            &pmul(m(r0, k0), m(r1, k1), p),
            &pmul(m(r0, k1), m(r1, k0), p),
            p,
        )
    };
    let adj: Tx = [
        minor(1, 2, 1, 2),
        psub(&[], &minor(1, 2, 0, 2), p),
        minor(1, 2, 0, 1),
    ];
    let dpoly: Tx = [vec![tr.d0], vec![tr.d1], vec![1]];
    let mut dp = dpoly;
    for q in dp.iter_mut() {
        trim(q);
    }
    let mut w = tx_mul(&adj, &dp, g, p);
    let mut col: Vec<Poly> = Vec::with_capacity(5);
    for _ in 0..5 {
        col.push(w[2].clone());
        w = tx_mul_x(&w, g, p);
    }
    let row = |i: usize| [col[i].clone(), col[i + 1].clone(), col[i + 2].clone()];
    [row(0), row(1), row(2)]
}

// ---------------------------------------------------------------------------
// Projective points over F_{p^2}.

/// Scale so that the first nonzero coordinate is 1 (`None` for `(0,0,0)`).
pub fn normalize_fp2(k: &F2, h: [Fp2; 3]) -> Option<[Fp2; 3]> {
    let i = (0..3).find(|&i| !k.is_zero(h[i]))?;
    let iv = k.inv(h[i]);
    Some([k.mul(h[0], iv), k.mul(h[1], iv), k.mul(h[2], iv)])
}

fn is_rational(h: &[Fp2; 3]) -> bool {
    h.iter().all(Fp2::is_rational)
}

fn fp2_pow(k: &F2, z: Fp2, e: u8) -> Fp2 {
    let mut r = k.c(1);
    for _ in 0..e {
        r = k.mul(r, z);
    }
    r
}

/// Values of the monomials `mons` at `h`.
fn mon_values(k: &F2, mons: &[(u8, u8, u8)], h: &[Fp2; 3]) -> Vec<Fp2> {
    mons.iter()
        .map(|&(a, b, c)| {
            k.mul(
                k.mul(fp2_pow(k, h[0], a), fp2_pow(k, h[1], b)),
                fp2_pow(k, h[2], c),
            )
        })
        .collect()
}

fn det3(k: &F2, a: &[Fp2; 3], b: &[Fp2; 3], c: &[Fp2; 3]) -> Fp2 {
    let m = |x: Fp2, y: Fp2, z: Fp2, w: Fp2| k.sub(k.mul(x, w), k.mul(y, z));
    k.add(
        k.sub(
            k.mul(a[0], m(b[1], b[2], c[1], c[2])),
            k.mul(a[1], m(b[0], b[2], c[0], c[2])),
        ),
        k.mul(a[2], m(b[0], b[1], c[0], c[1])),
    )
}

/// Outcome of a quartic fit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuarticFit {
    /// The quartic, first nonzero coefficient 1, in `mons_of_degree(4)` order.
    pub quartic: [u32; 15],
    /// Distinct projective points used.
    pub points: usize,
}

/// The unique quartic through `pts` (points of `P^2(F_{p^2})`).
///
/// `Err` unless there are at least 17 distinct points that span the plane
/// (the three differentials are independent), lie on no conic (`X` is not
/// hyperelliptic) and on exactly one quartic.
pub fn fit_quartic(p: u64, pts: &[[Fp2; 3]]) -> Result<QuarticFit, String> {
    let k = F2::new(p);
    let mut seen: Vec<[Fp2; 3]> = Vec::new();
    for h in pts {
        let n = normalize_fp2(&k, *h).ok_or("zero point in the fit")?;
        if !seen.contains(&n) {
            seen.push(n);
        }
    }
    if seen.len() < 17 {
        return Err(format!(
            "only {} distinct image points (need at least 17)",
            seen.len()
        ));
    }
    let null_dim = |d: usize| -> Vec<Vec<u64>> {
        let mons = mons_of_degree(d);
        let mut rows = Vec::with_capacity(2 * seen.len());
        for h in &seen {
            let v = mon_values(&k, &mons, h);
            rows.push(v.iter().map(|z| z.a).collect::<Vec<u64>>());
            rows.push(v.iter().map(|z| z.b).collect::<Vec<u64>>());
        }
        nullspace(&rows, mons.len(), p)
    };
    let n1 = null_dim(1).len();
    if n1 != 0 {
        return Err(format!(
            "the images satisfy {n1} linear relation(s): the three differentials are dependent"
        ));
    }
    let n2 = null_dim(2).len();
    if n2 != 0 {
        return Err(format!(
            "the images lie on {n2} conic(s): X is hyperelliptic for this input"
        ));
    }
    if !null_dim(3).is_empty() {
        return Err("the images lie on a cubic".into());
    }
    let ns = null_dim(4);
    if ns.len() != 1 {
        return Err(format!(
            "{} independent quartics through the images (expected exactly 1)",
            ns.len()
        ));
    }
    let v = &ns[0];
    let lead = *v.iter().find(|&&c| c != 0).unwrap();
    let il = inv_mod(lead, p);
    let mut quartic = [0u32; 15];
    for (q, &c) in quartic.iter_mut().zip(v) {
        *q = md(c, il, p) as u32;
    }
    Ok(QuarticFit {
        quartic,
        points: seen.len(),
    })
}

// ---------------------------------------------------------------------------
// Polynomials over F_{p^2} (for the (t, rho) input).

fn q_trim(k: &F2, a: &mut Vec<Fp2>) {
    while a.last().is_some_and(|&z| k.is_zero(z)) {
        a.pop();
    }
}

fn q_mul(k: &F2, a: &[Fp2], b: &[Fp2]) -> Vec<Fp2> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![Fp2::default(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = k.add(r[i + j], k.mul(x, y));
        }
    }
    q_trim(k, &mut r);
    r
}

fn q_add(k: &F2, a: &[Fp2], b: &[Fp2]) -> Vec<Fp2> {
    let n = a.len().max(b.len());
    let z = Fp2::default();
    let mut r: Vec<Fp2> = (0..n)
        .map(|i| k.add(*a.get(i).unwrap_or(&z), *b.get(i).unwrap_or(&z)))
        .collect();
    q_trim(k, &mut r);
    r
}

fn q_rem(k: &F2, a: &[Fp2], b: &[Fp2]) -> Vec<Fp2> {
    let mut r = a.to_vec();
    q_trim(k, &mut r);
    let il = k.inv(b[b.len() - 1]);
    while r.len() >= b.len() {
        let c = k.mul(r[r.len() - 1], il);
        let off = r.len() - b.len();
        for (j, &bj) in b.iter().enumerate() {
            r[off + j] = k.sub(r[off + j], k.mul(c, bj));
        }
        q_trim(k, &mut r);
    }
    r
}

fn q_gcd(k: &F2, a: &[Fp2], b: &[Fp2]) -> Vec<Fp2> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    q_trim(k, &mut a);
    q_trim(k, &mut b);
    while !b.is_empty() {
        let r = q_rem(k, &a, &b);
        a = b;
        b = r;
    }
    a
}

// ---------------------------------------------------------------------------
// The canonical map.

/// `psi: X -> C` for one Smith trigonal datum.
#[derive(Clone, Debug)]
pub struct CanonicalMap {
    pub tr: Trigonal,
    /// `T(t)` ([`psi_matrix`]).
    pub tmat: [[Poly; 3]; 3],
    /// The quartic form of `C` in `mons_of_degree(4)` order.
    pub quartic: [u32; 15],
    /// Distinct image points used in the fit.
    pub fit_points: usize,
}

impl CanonicalMap {
    /// Build `T(t)` and fit `C` through the images of the points of `X` over
    /// the rational points of `H` over `U`.
    pub fn new(tr: Trigonal) -> Result<CanonicalMap, String> {
        let tmat = psi_matrix(&tr);
        let mut cm = CanonicalMap {
            tr,
            tmat,
            quartic: [0; 15],
            fit_points: 0,
        };
        let mut pts = Vec::new();
        for (x, y) in cm.tr.points_over_u() {
            let Ok(ip) = cm.tr.image_points(x, y) else {
                continue;
            };
            for q in &ip.points {
                if let Ok(h) = cm.psi_xpoint(q) {
                    pts.push(h);
                }
            }
        }
        let fit = fit_quartic(cm.tr.p, &pts)?;
        cm.quartic = fit.quartic;
        cm.fit_points = fit.points;
        Ok(cm)
    }

    /// `psi` at the point of `X` over `t` whose Veronese point is
    /// `(b0 : b1 : b2)` (equivalently `(b02 : b12 : b22)`). `Err` where
    /// `T(t) b = 0`.
    pub fn psi_b(&self, t: Fp2, b: [Fp2; 3]) -> Result<[Fp2; 3], String> {
        let k = F2::new(self.tr.p);
        let mut h = [Fp2::default(); 3];
        for (i, hi) in h.iter_mut().enumerate() {
            for (j, &bj) in b.iter().enumerate() {
                *hi = k.add(*hi, k.mul(k.eval(&self.tmat[i][j], t), bj));
            }
        }
        normalize_fp2(&k, h).ok_or_else(|| "T(t) b = 0: psi is not defined by T here".to_string())
    }

    /// `psi` at a point of the affine model `(t, b00, .., b22)`.
    pub fn psi_xpoint(&self, q: &XPoint) -> Result<[Fp2; 3], String> {
        let k = F2::new(self.tr.p);
        let [b00, b01, b02, b11, b12, b22] = q.b;
        // Any nonzero column of the rank-1 matrix (b_i b_j) is the Veronese point.
        let col = [[b02, b12, b22], [b01, b11, b12], [b00, b01, b02]]
            .into_iter()
            .find(|c| c.iter().any(|&z| !k.is_zero(z)))
            .ok_or("all b_ij vanish")?;
        self.psi_b(k.c(q.t), col)
    }

    /// `Phi(t, rho) = delta_4 rho^4 + delta_2 rho^2 - delta_1 rho + delta_0`.
    pub fn plane_model(&self, t: Fp2, rho: Fp2) -> Fp2 {
        let k = F2::new(self.tr.p);
        let d = |i: usize| k.eval(&self.tr.delta[i], t);
        let r2 = k.mul(rho, rho);
        k.add(
            k.sub(
                k.add(k.mul(d(3), k.mul(r2, r2)), k.mul(d(2), r2)),
                k.mul(d(1), rho),
            ),
            d(0),
        )
    }

    /// The Mumford polynomials `(b0, b1, b2 = rho)` of the points of `X` over
    /// the point `(t, rho)` of the plane model with coordinates in `F_{p^2}`:
    /// the common roots in `b1` of `c1` and `c0` after eliminating `b0` with
    /// `c2`. One element at a smooth point of the plane model, two at a node
    /// (the plane model is only birational to `X`). `Err` when `rho = 0` or
    /// `(t, rho)` is not on the plane model.
    pub fn lifts_t_rho(&self, t: Fp2, rho: Fp2) -> Result<Vec<[Fp2; 3]>, String> {
        let p = self.tr.p;
        let k = F2::new(p);
        if k.is_zero(rho) {
            return Err("rho = 0".into());
        }
        if !k.is_zero(self.plane_model(t, rho)) {
            return Err("Phi(t, rho) != 0".into());
        }
        let e = |q: &Poly| k.eval(q, t);
        let (f0, f1, f2) = (
            e(&self.tr.fpoly[0]),
            e(&self.tr.fpoly[1]),
            e(&self.tr.fpoly[2]),
        );
        let (g0, g1, g2) = (e(&self.tr.g[0]), e(&self.tr.g[1]), e(&self.tr.g[2]));
        let r2 = k.mul(rho, rho);
        let c = k.inv(k.scale(rho, 2));
        // b0(z) = c (A + B z - z^2), z = b1.
        let a = k.sub(f2, k.mul(r2, k.sub(k.mul(g2, g2), g1)));
        let b = k.scale(k.mul(rho, g2), 2);
        let b0: Vec<Fp2> = vec![k.mul(c, a), k.mul(c, b), k.neg(c)];
        // c1: 2 b0 z - 2 rho g1 z + rho^2 (g1 g2 - g0) - f1.
        let two_b0: Vec<Fp2> = b0.iter().map(|&z| k.scale(z, 2)).collect();
        let e1 = q_add(
            &k,
            &q_mul(&k, &two_b0, &[k.c(0), k.c(1)]),
            &[
                k.sub(k.mul(r2, k.sub(k.mul(g1, g2), g0)), f1),
                k.neg(k.scale(k.mul(rho, g1), 2)),
            ],
        );
        // c0: b0^2 - 2 rho g0 z + rho^2 g0 g2 - f0.
        let e0 = q_add(
            &k,
            &q_mul(&k, &b0, &b0),
            &[
                k.sub(k.mul(r2, k.mul(g0, g2)), f0),
                k.neg(k.scale(k.mul(rho, g0), 2)),
            ],
        );
        let g = q_gcd(&k, &e0, &e1);
        let b_of = |z: Fp2| [k.mul(c, k.add(a, k.sub(k.mul(b, z), k.mul(z, z)))), z, rho];
        match g.len() {
            0 | 1 => Ok(Vec::new()),
            2 => Ok(vec![b_of(k.neg(k.mul(g[0], k.inv(g[1]))))]),
            3 => {
                // g0 + g1 z + g2 z^2 over F_{p^2}.
                let i2 = k.inv(k.scale(g[2], 2));
                let disc = k.sub(k.mul(g[1], g[1]), k.scale(k.mul(g[0], g[2]), 4));
                let Some(r) = k.sqrt(disc) else {
                    return Ok(Vec::new());
                };
                let z1 = k.mul(k.sub(r, g[1]), i2);
                let z2 = k.mul(k.sub(k.neg(r), g[1]), i2);
                Ok(if z1 == z2 {
                    vec![b_of(z1)]
                } else {
                    vec![b_of(z1), b_of(z2)]
                })
            }
            _ => Err("more than two points of X over (t, rho)".into()),
        }
    }

    /// The unique lift of `(t, rho)`; `Err` at a singular point of the plane
    /// model, where `(t, rho)` does not determine the point of `X`.
    pub fn lift_t_rho(&self, t: Fp2, rho: Fp2) -> Result<[Fp2; 3], String> {
        let v = self.lifts_t_rho(t, rho)?;
        match v.len() {
            1 => Ok(v[0]),
            0 => Err("(t, rho) does not lift to X over F_p^2".into()),
            _ => Err("(t, rho) does not determine the point of X".into()),
        }
    }

    /// `psi` at a point of the plane model `Phi(t, rho) = 0`.
    pub fn psi_t_rho(&self, t: Fp2, rho: Fp2) -> Result<[Fp2; 3], String> {
        let b = self.lift_t_rho(t, rho)?;
        self.psi_b(t, b)
    }

    /// Is `h` on `C`?
    pub fn on_quartic(&self, h: &[Fp2; 3]) -> bool {
        let p = self.tr.p;
        let k = F2::new(p);
        let v = mon_values(&k, &mons_of_degree(4), h);
        let mut s = Fp2::default();
        for (z, &c) in v.iter().zip(&self.quartic) {
            s = k.add(s, k.scale(*z, c as u64));
        }
        k.is_zero(s)
    }

    /// Smoothness certificate of `C`.
    pub fn is_smooth(&self) -> bool {
        smoothness_certificate(self.tr.p as u32, &self.quartic)
    }
}

// ---------------------------------------------------------------------------
// F_{p^k}, k <= 3, and point counts.

/// `F_{p^k} = F_p[w]/(w^k - m0 - m1 w - m2 w^2)`, `k` in `{1, 2, 3}`.
#[derive(Clone, Debug)]
pub struct Fk {
    pub p: u64,
    pub k: usize,
    m: [u64; 3],
}

pub type Ek = [u64; 3];

impl Fk {
    pub fn new(p: u64, k: usize) -> Fk {
        let m = match k {
            1 => [0, 0, 0],
            2 => [super::smith_prop24::least_nonsquare(p), 0, 0],
            3 => {
                let mut found = None;
                'o: for a in 0..p {
                    for b in 1..p {
                        if (0..p).all(|x| ad(md(md(x, x, p), x, p), ad(md(a, x, p), b, p), p) != 0)
                        {
                            found = Some([ng(b, p), ng(a, p), 0]);
                            break 'o;
                        }
                    }
                }
                found.expect("an irreducible cubic x^3 + a x + b exists")
            }
            _ => panic!("Fk: k must be 1, 2 or 3"),
        };
        Fk { p, k, m }
    }
    pub fn size(&self) -> u64 {
        self.p.pow(self.k as u32)
    }
    pub fn el(&self, mut i: u64) -> Ek {
        let mut e = [0u64; 3];
        for c in e.iter_mut().take(self.k) {
            *c = i % self.p;
            i /= self.p;
        }
        e
    }
    pub fn c(&self, x: u64) -> Ek {
        [x % self.p, 0, 0]
    }
    pub fn is_zero(&self, a: &Ek) -> bool {
        *a == [0, 0, 0]
    }
    pub fn add(&self, a: &Ek, b: &Ek) -> Ek {
        [
            ad(a[0], b[0], self.p),
            ad(a[1], b[1], self.p),
            ad(a[2], b[2], self.p),
        ]
    }
    pub fn sub(&self, a: &Ek, b: &Ek) -> Ek {
        [
            sb(a[0], b[0], self.p),
            sb(a[1], b[1], self.p),
            sb(a[2], b[2], self.p),
        ]
    }
    pub fn mul(&self, a: &Ek, b: &Ek) -> Ek {
        let p = self.p;
        let k = self.k;
        let mut t = [0u64; 5];
        for i in 0..k {
            if a[i] == 0 {
                continue;
            }
            for j in 0..k {
                t[i + j] = (t[i + j] + a[i] * b[j]) % p;
            }
        }
        if k > 1 {
            for i in (k..2 * k - 1).rev() {
                let c = t[i];
                t[i] = 0;
                if c != 0 {
                    for j in 0..k {
                        t[i - k + j] = (t[i - k + j] + c * self.m[j]) % p;
                    }
                }
            }
        }
        [t[0], t[1], t[2]]
    }
    pub fn pow(&self, a: &Ek, mut e: u64) -> Ek {
        let mut r = self.c(1);
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    pub fn inv(&self, a: &Ek) -> Ek {
        self.pow(a, self.size() - 2)
    }
}

/// Number of distinct roots in `F_{p^k}` of a polynomial over it (`q` for the
/// zero polynomial): `deg gcd(z^q - z, c)`.
fn count_roots(fk: &Fk, c: &[Ek]) -> u64 {
    let mut c = c.to_vec();
    while c.last().is_some_and(|z| fk.is_zero(z)) {
        c.pop();
    }
    if c.is_empty() {
        return fk.size();
    }
    let d = c.len() - 1;
    if d <= 1 {
        return d as u64;
    }
    let il = fk.inv(&c[d]);
    for z in c.iter_mut() {
        *z = fk.mul(z, &il);
    }
    // r = z^q mod c.
    let reduce = |t: &mut Vec<Ek>| {
        while t.len() > d {
            let top = t.pop().unwrap();
            if !fk.is_zero(&top) {
                let off = t.len() - d;
                for j in 0..d {
                    let s = fk.mul(&top, &c[j]);
                    t[off + j] = fk.sub(&t[off + j], &s);
                }
            }
        }
    };
    let q = fk.size();
    let mut r: Vec<Ek> = vec![fk.c(1)];
    for bit in (0..64 - q.leading_zeros()).rev() {
        let mut t = vec![[0u64; 3]; 2 * r.len() - 1];
        for i in 0..r.len() {
            for j in 0..r.len() {
                let s = fk.mul(&r[i], &r[j]);
                t[i + j] = fk.add(&t[i + j], &s);
            }
        }
        if (q >> bit) & 1 == 1 {
            t.insert(0, [0, 0, 0]);
        }
        reduce(&mut t);
        r = t;
    }
    while r.len() < 2 {
        r.push([0, 0, 0]);
    }
    r[1] = fk.sub(&r[1], &fk.c(1));
    // gcd(c, r).
    let mut a = c;
    let mut b = r;
    loop {
        while b.last().is_some_and(|z| fk.is_zero(z)) {
            b.pop();
        }
        if b.is_empty() {
            return (a.len() - 1) as u64;
        }
        let il = fk.inv(&b[b.len() - 1]);
        while a.len() >= b.len() {
            let cc = fk.mul(&a[a.len() - 1], &il);
            let off = a.len() - b.len();
            for j in 0..b.len() {
                let s = fk.mul(&cc, &b[j]);
                a[off + j] = fk.sub(&a[off + j], &s);
            }
            a.pop();
            while a.last().is_some_and(|z| fk.is_zero(z)) {
                a.pop();
            }
        }
        std::mem::swap(&mut a, &mut b);
    }
}

/// `#C(F_{p^k})` for the plane quartic `f` (`mons_of_degree(4)` order),
/// `k <= 3`: for each `(1 : y)` the number of roots in `z` of `f(1, y, z)`,
/// then the line `x = 0`.
pub fn count_quartic_points(p: u64, f: &[u32; 15], k: usize) -> u64 {
    let fk = Fk::new(p, k);
    let m4 = mons_of_degree(4);
    let mut n = 0u64;
    for yi in 0..fk.size() {
        let y = fk.el(yi);
        let mut yp = [fk.c(1); 5];
        for b in 1..5 {
            yp[b] = fk.mul(&yp[b - 1], &y);
        }
        let mut coef = [[0u64; 3]; 5];
        for (i, &(_, b, c)) in m4.iter().enumerate() {
            if f[i] != 0 {
                let t = fk.mul(&fk.c(f[i] as u64), &yp[b as usize]);
                coef[c as usize] = fk.add(&coef[c as usize], &t);
            }
        }
        n += count_roots(&fk, &coef);
    }
    let mut coef = [[0u64; 3]; 5];
    for (i, &(a, b, c)) in m4.iter().enumerate() {
        if a == 0 && b > 0 {
            // (0 : 1 : z)
            coef[c as usize] = fk.add(&coef[c as usize], &fk.c(f[i] as u64));
        }
    }
    // f(0, 1, z) includes the z^4 term.
    coef[4] = fk.c(f[14] as u64);
    n += count_roots(&fk, &coef);
    if f[14] == 0 {
        n += 1; // (0 : 0 : 1)
    }
    n
}

/// `#H(F_{p^k})` for `y^2 = F(x)` (`deg F` 7 or 8), points at infinity
/// included, `k <= 3`.
pub fn count_h_points(f: &[u64], p: u64, k: usize) -> u64 {
    let fk = Fk::new(p, k);
    let q = fk.size();
    let one = fk.c(1);
    let mut n = 0u64;
    for xi in 0..q {
        let x = fk.el(xi);
        let mut v = [0u64; 3];
        for &c in f.iter().rev() {
            v = fk.add(&fk.mul(&v, &x), &fk.c(c));
        }
        if fk.is_zero(&v) {
            n += 1;
        } else if fk.pow(&v, (q - 1) / 2) == one {
            n += 2;
        }
    }
    let d = deg(f);
    if d % 2 == 1 {
        n + 1
    } else if k.is_multiple_of(2) || chi(f[d as usize], p) == 1 {
        n + 2
    } else {
        n
    }
}

/// `#J(F_p)` of a genus-3 curve from `N_1, N_2, N_3`.
pub fn jac_order(p: u64, n: [u64; 3]) -> i128 {
    let q = p as i128;
    let t1 = q + 1 - n[0] as i128;
    let t2 = q * q + 1 - n[1] as i128;
    let t3 = q * q * q + 1 - n[2] as i128;
    let e1 = t1;
    let e2 = (t1 * t1 - t2) / 2;
    let e3 = (t1 * t1 * t1 - 3 * t1 * t2 + 2 * t3) / 6;
    1 - e1 + e2 - e3 + q * e2 - q * q * e1 + q * q * q
}

fn factorize(mut n: u128) -> Vec<(u128, u32)> {
    let mut out = Vec::new();
    let mut d = 2u128;
    while d * d <= n {
        let mut e = 0;
        while n.is_multiple_of(d) {
            n /= d;
            e += 1;
        }
        if e > 0 {
            out.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

// ---------------------------------------------------------------------------
// J_H for deg F = 7 (Cantor).

/// A reduced divisor class `(u, v)` of `J_H(F_p)`: `u` monic of degree at
/// most 3, `deg v < deg u`, `u | F - v^2`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mumford {
    pub u: Poly,
    pub v: Poly,
}

/// `J_H(F_p)` for `H: y^2 = F(x)`, `deg F = 7`.
#[derive(Clone, Debug)]
pub struct HJac {
    pub f: Poly,
    pub p: u64,
}

impl HJac {
    pub fn new(f: &[u64], p: u64) -> Result<HJac, String> {
        let mut f = f.to_vec();
        trim(&mut f);
        if deg(&f) != 7 {
            return Err(format!("HJac needs deg F = 7, got {}", deg(&f)));
        }
        Ok(HJac { f, p })
    }
    pub fn zero(&self) -> Mumford {
        Mumford {
            u: vec![1],
            v: Vec::new(),
        }
    }
    pub fn is_zero(&self, d: &Mumford) -> bool {
        d.u.len() == 1
    }
    /// The class `[P - inf]`.
    pub fn point(&self, x: u64, y: u64) -> Mumford {
        let mut v = vec![y % self.p];
        trim(&mut v);
        Mumford {
            u: vec![ng(x, self.p), 1],
            v,
        }
    }
    pub fn neg(&self, d: &Mumford) -> Mumford {
        Mumford {
            u: d.u.clone(),
            v: psub(&[], &d.v, self.p),
        }
    }
    pub fn add(&self, a: &Mumford, b: &Mumford) -> Mumford {
        let p = self.p;
        let (d1, e1, e2) = pxgcd(&a.u, &b.u, p);
        let vs = padd(&a.v, &b.v, p);
        let (d, c1, c2) = pxgcd(&d1, &vs, p);
        let s1 = pmul(&c1, &e1, p);
        let s2 = pmul(&c1, &e2, p);
        let dd = pmul(&d, &d, p);
        let (mut u, r) = pdivrem(&pmul(&a.u, &b.u, p), &dd, p);
        debug_assert!(r.is_empty());
        let num = padd(
            &padd(
                &pmul(&pmul(&s1, &a.u, p), &b.v, p),
                &pmul(&pmul(&s2, &b.u, p), &a.v, p),
                p,
            ),
            &pmul(&c2, &padd(&pmul(&a.v, &b.v, p), &self.f, p), p),
            p,
        );
        let (nq, nr) = pdivrem(&num, &d, p);
        debug_assert!(nr.is_empty());
        let mut v = prem(&nq, &u, p);
        while deg(&u) > 3 {
            let (q, r) = pdivrem(&psub(&self.f, &pmul(&v, &v, p), p), &u, p);
            debug_assert!(r.is_empty());
            u = pmonic(&q, p);
            v = prem(&psub(&[], &v, p), &u, p);
        }
        Mumford { u, v }
    }
    pub fn sub(&self, a: &Mumford, b: &Mumford) -> Mumford {
        self.add(a, &self.neg(b))
    }
    pub fn mul(&self, d: &Mumford, mut k: u128) -> Mumford {
        let mut r = self.zero();
        let mut b = d.clone();
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            b = self.add(&b, &b);
            k >>= 1;
        }
        r
    }
    /// Order of `d`, given a multiple `n` of it.
    pub fn order(&self, d: &Mumford, n: u128) -> u128 {
        let mut o = n;
        for (q, e) in factorize(n) {
            for _ in 0..e {
                if self.is_zero(&self.mul(d, o / q)) {
                    o /= q;
                } else {
                    break;
                }
            }
        }
        o
    }
    /// The affine points of the support when `u` splits into distinct
    /// rational linear factors.
    pub fn split(&self, d: &Mumford) -> Option<Vec<(u64, u64)>> {
        let roots = proots(&d.u, self.p);
        (roots.len() as isize == deg(&d.u)).then(|| {
            roots
                .into_iter()
                .map(|x| (x, peval(&d.v, x, self.p)))
                .collect()
        })
    }
}

// ---------------------------------------------------------------------------
// The transfer to Pic^0(C).

/// `Psi = psi_* . phi` with `Pic^0(C)` arithmetic.
pub struct Transfer {
    pub cm: CanonicalMap,
    pub c: PlaneQuartic,
    /// Rational points of `C` used as auxiliary base points (`bases[0] = P0`).
    bases: Vec<Pt>,
    std2: Vec<(u8, u8, u8)>,
    cache: RefCell<HashMap<(u64, u64), Result<PicClass, String>>>,
}

impl Transfer {
    /// `Err` when `C` is singular or has no rational point.
    pub fn new(cm: CanonicalMap) -> Result<Transfer, String> {
        let q = cm.tr.p as u32;
        let p0 = first_rational_point_canonical(q, &cm.quartic).ok_or("C has no rational point")?;
        let c = PlaneQuartic::new(q, cm.quartic, p0).map_err(|e| format!("{e:?}"))?;
        let mut bases = vec![c.base_point()];
        for pt in c.rational_points_bruteforce() {
            if bases.len() >= 8 {
                break;
            }
            if !bases.contains(&pt) {
                bases.push(pt);
            }
        }
        let std2 = c.std_monomials(2);
        Ok(Transfer {
            cm,
            c,
            bases,
            std2,
            cache: RefCell::new(HashMap::new()),
        })
    }

    /// The class `[Q1 + Q2 + 2 P0 - K]` of a rational effective divisor of
    /// degree 2 on `C`: two distinct rational points, or a conjugate pair
    /// over `F_{p^2}` (module doc).
    pub fn pair_class(&self, q1: [Fp2; 3], q2: [Fp2; 3]) -> Result<PicClass, String> {
        let p = self.cm.tr.p;
        let k = F2::new(p);
        let n1 = normalize_fp2(&k, q1).ok_or("zero point")?;
        let n2 = normalize_fp2(&k, q2).ok_or("zero point")?;
        if !self.cm.on_quartic(&n1) || !self.cm.on_quartic(&n2) {
            return Err("point not on C".into());
        }
        let rational = is_rational(&n1) && is_rational(&n2);
        if !rational && n2 != n1.map(|z| k.conj(z)) {
            return Err("the pair is neither rational nor conjugate".into());
        }
        if n1 == n2 {
            return Err("double point: not supported".into());
        }
        let v1 = mon_values(&k, &self.std2, &n1);
        let v2 = mon_values(&k, &self.std2, &n2);
        for (bi, base) in self.bases.iter().enumerate() {
            let bf = [
                k.c(base[0] as u64),
                k.c(base[1] as u64),
                k.c(base[2] as u64),
            ];
            if k.is_zero(det3(&k, &n1, &n2, &bf)) {
                continue;
            }
            let mut rows: Vec<Vec<u64>> = if rational {
                vec![
                    v1.iter().map(|z| z.a).collect(),
                    v2.iter().map(|z| z.a).collect(),
                ]
            } else {
                vec![
                    v1.iter().map(|z| z.a).collect(),
                    v1.iter().map(|z| z.b).collect(),
                ]
            };
            rows.push(
                mon_values(&k, &self.std2, &bf)
                    .iter()
                    .map(|z| z.a)
                    .collect(),
            );
            let ns = nullspace(&rows, 6, p);
            if ns.len() != 3 {
                continue;
            }
            let conics: Vec<Vec<u32>> = ns
                .into_iter()
                .map(|r| r.into_iter().map(|x| x as u32).collect())
                .collect();
            let cls = self.c.class_from_conics(conics);
            return Ok(if bi == 0 {
                cls
            } else {
                self.c.sub(&cls, &self.c.point_class(*base))
            });
        }
        Err("no usable auxiliary base point".into())
    }

    /// `[E(P) + 2 P0 - K]`, `E(P) = psi(Q1) + psi(Q2)` the image of the affine
    /// point `P = (x, y)` of `H` over `U`.
    pub fn e_class(&self, x: u64, y: u64) -> Result<PicClass, String> {
        if let Some(r) = self.cache.borrow().get(&(x, y)) {
            return r.clone();
        }
        let r = (|| {
            let ip = self.cm.tr.image_points(x, y)?;
            let h1 = self.cm.psi_xpoint(&ip.points[0])?;
            let h2 = self.cm.psi_xpoint(&ip.points[1])?;
            self.pair_class(h1, h2)
        })();
        self.cache.borrow_mut().insert((x, y), r.clone());
        r
    }

    /// `Psi( sum plus - sum minus )` for affine points of `H` over `U`
    /// (`plus.len() == minus.len()`).
    pub fn point_sum(&self, plus: &[(u64, u64)], minus: &[(u64, u64)]) -> Result<PicClass, String> {
        if plus.len() != minus.len() {
            return Err("point_sum needs a divisor of degree 0".into());
        }
        let mut acc = self.c.zero();
        for &(x, y) in plus {
            acc = self.c.add(&acc, &self.e_class(x, y)?);
        }
        for &(x, y) in minus {
            acc = self.c.sub(&acc, &self.e_class(x, y)?);
        }
        Ok(acc)
    }

    /// The affine points of `H` over `U` on which `E` is defined.
    pub fn usable_points(&self) -> Vec<(u64, u64)> {
        self.cm
            .tr
            .points_over_u()
            .into_iter()
            .filter(|&(x, y)| self.e_class(x, y).is_ok())
            .collect()
    }

    /// `Psi(D)` for a class `D` of `J_H(F_p)` in Mumford form (`deg F = 7`),
    /// by moving it onto rational points over `U` (module doc). `pts` is
    /// [`usable_points`](Self::usable_points).
    pub fn transfer_divisor(
        &self,
        jac: &HJac,
        d: &Mumford,
        pts: &[(u64, u64)],
        rng: &mut SplitMix64,
    ) -> Result<PicClass, String> {
        if pts.len() < 8 {
            return Err("too few usable points of H over U".into());
        }
        let usable = |r: &[(u64, u64)]| r.len() == 3 && r.iter().all(|q| pts.contains(q));
        let p = self.cm.tr.p;
        let rand_sum = |rng: &mut SplitMix64| {
            let mut plus = Vec::new();
            let mut minus = Vec::new();
            let mut m = jac.zero();
            for _ in 0..2 {
                let a = pts[rng.below(pts.len() as u64) as usize];
                let b = pts[rng.below(pts.len() as u64) as usize];
                m = jac.add(&m, &jac.point(a.0, a.1));
                m = jac.add(&m, &jac.point(b.0, ng(b.1, p)));
                plus.push(a);
                minus.push(b);
            }
            (m, plus, minus)
        };
        // D + S1 = R1 + R2 + R3 - 3 inf and S2 = R1' + R2' + R3' - 3 inf.
        let find = |base: &Mumford, rng: &mut SplitMix64| {
            for _ in 0..20_000 {
                let (m, plus, minus) = rand_sum(rng);
                let s = jac.add(base, &m);
                if let Some(r) = jac.split(&s) {
                    if usable(&r) {
                        return Some((r, plus, minus));
                    }
                }
            }
            None
        };
        let (r1, p1, m1) = find(d, rng).ok_or("no split representative of D + S1")?;
        let (r2, p2, m2) = find(&jac.zero(), rng).ok_or("no split representative of S2")?;
        // D = sum R - sum R' + S2 - S1.
        let mut plus = r1;
        plus.extend(p2);
        plus.extend(m1);
        let mut minus = r2;
        minus.extend(m2);
        minus.extend(p1);
        self.point_sum(&plus, &minus)
    }

    /// Order of a class of `Pic^0(C)` given a multiple `n` of it.
    pub fn pic_order(&self, x: &PicClass, n: u128) -> Option<u128> {
        if !self.c.is_zero(&self.c.mul_scalar(x, n)) {
            return None;
        }
        let mut o = n;
        for (q, e) in factorize(n) {
            for _ in 0..e {
                if self.c.is_zero(&self.c.mul_scalar(x, o / q)) {
                    o /= q;
                } else {
                    break;
                }
            }
        }
        Some(o)
    }

    /// Baby-step giant-step in `Pic^0(C)`: the least `k < ord` with
    /// `target = k base`, `ord` the order of `base`.
    pub fn pic_dlog(&self, base: &PicClass, target: &PicClass, ord: u128) -> Option<u128> {
        let m = (ord as f64).sqrt().ceil() as u128 + 1;
        let mut table: HashMap<PicKey, u128> = HashMap::new();
        let mut cur = self.c.zero();
        for j in 0..m {
            table.entry(cur.key()).or_insert(j);
            cur = self.c.add(&cur, base);
        }
        let step = self.c.neg(&self.c.mul_scalar(base, m));
        let mut g = target.clone();
        for i in 0..=m {
            if let Some(&j) = table.get(&g.key()) {
                return Some((i * m + j) % ord);
            }
            g = self.c.add(&g, &step);
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Relations on H and the rational part of the kernel S.

/// A principal divisor on `H` supported on rational affine points over `U`:
/// the zeros of `y - v(x)`, `v` the cubic through four points of `pts` with
/// distinct `x`, when `v^2 - F` splits into distinct rational roots whose
/// points are all in `pts`. Two such zero divisors are linearly equivalent
/// (both are equivalent to the same multiple of the divisor at infinity).
pub fn function_zeros(
    f: &[u64],
    p: u64,
    pts: &[(u64, u64)],
    rng: &mut SplitMix64,
    max_tries: usize,
) -> Option<Vec<(u64, u64)>> {
    if pts.len() < 4 {
        return None;
    }
    for _ in 0..max_tries {
        let mut sel: Vec<(u64, u64)> = Vec::new();
        while sel.len() < 4 {
            let q = pts[rng.below(pts.len() as u64) as usize];
            if sel.iter().all(|s| s.0 != q.0) {
                sel.push(q);
            }
        }
        // Lagrange interpolation.
        let mut v: Poly = Vec::new();
        for (i, &(xi, yi)) in sel.iter().enumerate() {
            let mut num: Poly = vec![1];
            let mut den = 1u64;
            for (j, &(xj, _)) in sel.iter().enumerate() {
                if i != j {
                    num = pmul(&num, &[ng(xj, p), 1], p);
                    den = md(den, sb(xi, xj, p), p);
                }
            }
            v = padd(&v, &pscale(&num, md(yi, inv_mod(den, p), p), p), p);
        }
        let w = psub(&pmul(&v, &v, p), f, p);
        let roots = proots(&w, p);
        if roots.len() as isize != deg(&w) {
            continue;
        }
        let zs: Vec<(u64, u64)> = roots.into_iter().map(|x| (x, peval(&v, x, p))).collect();
        if zs.iter().all(|q| pts.contains(q)) {
            return Some(zs);
        }
    }
    None
}

/// Is the 2-torsion class `(u, 0)` (`u` a monic rational factor of `F` of
/// degree at most 3, `deg F = 7`) in Smith's kernel `S`? `S` is generated by
/// the differences of Weierstrass points in the same fibre of `g = N/D`; the
/// point at infinity is paired with the common root of `D` and `F`.
pub fn two_torsion_in_s(tr: &Trigonal, u: &[u64]) -> bool {
    let p = tr.p;
    let nn: Poly = vec![tr.n0, tr.n1, 0, 1];
    let dd: Poly = vec![tr.d0, tr.d1, 1];
    let is_pair = |u2: &[u64]| -> bool {
        // The two roots of u2 have the same image under g: N = t D mod u2.
        let a = prem(&nn, u2, p);
        let b = prem(&dd, u2, p);
        if b.is_empty() {
            return false;
        }
        let c = |q: &Poly, i: usize| *q.get(i).unwrap_or(&0);
        sb(md(c(&a, 1), c(&b, 0), p), md(c(&a, 0), c(&b, 1), p), p) == 0
    };
    match deg(u) {
        0 => true,
        1 => prem(&dd, u, p).is_empty(),
        2 => is_pair(u),
        3 => {
            let (g, _, _) = pxgcd(u, &dd, p);
            deg(&g) == 1 && is_pair(&pdivrem(u, &g, p).0)
        }
        _ => false,
    }
}

/// The monic rational factors of `F` of degree 1 to 3 (products of distinct
/// irreducible factors): the nonzero rational 2-torsion classes `(u, 0)` of
/// `J_H` for `deg F = 7`.
pub fn rational_two_torsion(f: &[u64], p: u64, rng: &mut SplitMix64) -> Vec<Poly> {
    let Some(irr) = super::smith_prop24::factor_squarefree(f, p, rng) else {
        return Vec::new();
    };
    let small: Vec<&Poly> = irr.iter().filter(|q| deg(q) <= 3).collect();
    let mut out = Vec::new();
    for mask in 1u32..(1 << small.len()) {
        let mut u: Poly = vec![1];
        for (i, q) in small.iter().enumerate() {
            if mask >> i & 1 == 1 {
                u = pmul(&u, q, p);
            }
        }
        if deg(&u) <= 3 {
            out.push(u);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Per-curve verification (the instrument of the study).

/// What to run in [`verify_curve`].
#[derive(Clone, Copy, Debug)]
pub struct VerifyOpts {
    /// Count `C` over `F_p .. F_{p^kmax}` (1 to 3); `H` is always counted
    /// over all three.
    pub kmax: usize,
    /// Pairs of principal divisors to push through `Psi`.
    pub relations: usize,
    /// Random classes for the order and representation tests.
    pub classes: usize,
    /// Planted scalar of the DLP transfer (reduced modulo the order).
    pub k: u128,
    pub seed: u64,
}

/// Every measured quantity for one curve. Counters are `ok / total`.
#[derive(Clone, Debug, Default)]
pub struct CurveReport {
    pub p: u64,
    pub deg_f: usize,
    pub fit_points: usize,
    pub smooth: bool,
    pub n_h: Vec<u64>,
    pub n_c: Vec<u64>,
    /// `#J_H(F_p)` from `N_1..N_3` of `H`.
    pub jac_h: Option<u128>,
    pub on_c: (usize, usize),
    pub t_rho: (usize, usize),
    /// Image points at a node of the plane model (two lifts of `(t, rho)`).
    pub t_rho_nodes: usize,
    pub usable_points: usize,
    pub relations: (usize, usize),
    /// Detector control: the same relations with one point replaced by
    /// another point of `H` must NOT map to zero (`nonzero / total`).
    pub relations_neg: (usize, usize),
    /// `Psi` of two independent representations of the same class agree.
    pub representation: (usize, usize),
    /// `Psi(D1 + D2) = Psi(D1) + Psi(D2)`, each side moved independently.
    pub additivity: (usize, usize),
    /// `#J_H . Psi(D) = 0`.
    pub annihilated: (usize, usize),
    /// `ord Psi(D) | ord D` with quotient in `{1, 2, 4, 8}`.
    pub order_ratio: (usize, usize),
    pub ratio_hist: [usize; 4],
    /// Rational 2-torsion classes: `Psi(u, 0) = 0` iff `(u, 0)` in `S`.
    pub two_torsion: (usize, usize),
    pub two_torsion_in_s: usize,
    /// `(k, ord_H(D), ord_C(Psi D), recovered k', Psi(kD) == k Psi(D))`.
    pub dlp: Option<(u128, u128, u128, Option<u128>, bool)>,
    pub notes: Vec<String>,
}

impl CurveReport {
    /// All checks that ran passed.
    pub fn all_pass(&self) -> bool {
        let full = |c: (usize, usize)| c.0 == c.1;
        self.smooth
            && !self.n_c.is_empty()
            && self.n_h[..self.n_c.len()] == self.n_c[..]
            && self.relations_neg.0 == self.relations_neg.1
            && full(self.on_c)
            && full(self.t_rho)
            && full(self.relations)
            && full(self.representation)
            && full(self.additivity)
            && full(self.annihilated)
            && full(self.order_ratio)
            && full(self.two_torsion)
            && self
                .dlp
                .is_none_or(|d| d.4 && d.3.is_some_and(|k2| d.2 > 0 && (k2 % d.2) == (d.0 % d.2)))
    }

    /// One row of the results table.
    pub fn row(&self) -> String {
        let c = |x: (usize, usize)| format!("{}/{}", x.0, x.1);
        format!(
            "p={} degF={} fit_pts={} smooth={} N_H={:?} N_C={:?} #J_H={:?} on_C={} t_rho={} (nodes {}) usable={} relations={} relations_neg={} repr={} add={} annih={} ord_ratio={} hist(1,2,4,8)={:?} 2tors={} (in S: {}) dlp={:?} pass={}",
            self.p,
            self.deg_f,
            self.fit_points,
            self.smooth,
            self.n_h,
            self.n_c,
            self.jac_h,
            c(self.on_c),
            c(self.t_rho),
            self.t_rho_nodes,
            self.usable_points,
            c(self.relations),
            c(self.relations_neg),
            c(self.representation),
            c(self.additivity),
            c(self.annihilated),
            c(self.order_ratio),
            self.ratio_hist,
            c(self.two_torsion),
            self.two_torsion_in_s,
            self.dlp,
            self.all_pass()
        )
    }
}

/// Run every check of the study on one Smith datum. `Err` only when the
/// canonical map itself cannot be built (the message says why).
pub fn verify_curve(tr: &Trigonal, o: &VerifyOpts) -> Result<CurveReport, String> {
    let p = tr.p;
    let k2 = F2::new(p);
    let mut rng = SplitMix64(o.seed);
    let cm = CanonicalMap::new(tr.clone())?;
    let mut rep = CurveReport {
        p,
        deg_f: deg(&tr.f) as usize,
        fit_points: cm.fit_points,
        smooth: cm.is_smooth(),
        ..Default::default()
    };
    for k in 1..=3 {
        rep.n_h.push(count_h_points(&tr.f, p, k));
        if k <= o.kmax {
            rep.n_c.push(count_quartic_points(p, &cm.quartic, k));
        }
    }
    rep.jac_h = Some(jac_order(p, [rep.n_h[0], rep.n_h[1], rep.n_h[2]]) as u128);
    // psi on the image points, by both inputs.
    for (x, y) in tr.points_over_u() {
        let Ok(ip) = tr.image_points(x, y) else {
            continue;
        };
        for q in &ip.points {
            rep.on_c.1 += 1;
            let Ok(h) = cm.psi_xpoint(q) else { continue };
            rep.on_c.0 += cm.on_quartic(&h) as usize;
            if !k2.is_zero(q.rho) {
                rep.t_rho.1 += 1;
                let lifts = cm.lifts_t_rho(k2.c(q.t), q.rho).unwrap_or_default();
                rep.t_rho_nodes += (lifts.len() > 1) as usize;
                rep.t_rho.0 += lifts.iter().any(|b| cm.psi_b(k2.c(q.t), *b) == Ok(h)) as usize;
            }
        }
    }
    if !rep.smooth {
        rep.notes.push("C is singular: no Pic arithmetic".into());
        return Ok(rep);
    }
    let tf = Transfer::new(cm)?;
    let pts = tf.usable_points();
    rep.usable_points = pts.len();
    // Principal divisors.
    for _ in 0..o.relations {
        let a = function_zeros(&tr.f, p, &pts, &mut rng, 200_000);
        let b = function_zeros(&tr.f, p, &pts, &mut rng, 200_000);
        let (Some(a), Some(b)) = (a, b) else {
            rep.notes.push("no split function found".into());
            continue;
        };
        if a == b {
            continue;
        }
        rep.relations.1 += 1;
        rep.relations.0 += tf.point_sum(&a, &b).is_ok_and(|c| tf.c.is_zero(&c)) as usize;
        // Detector control: swap one point for a point with another x.
        let mut a2 = a.clone();
        if let Some(&q) = pts.iter().find(|q| a.iter().all(|r| r.0 != q.0)) {
            a2[0] = q;
            rep.relations_neg.1 += 1;
            rep.relations_neg.0 += tf.point_sum(&a2, &b).is_ok_and(|c| !tf.c.is_zero(&c)) as usize;
        }
    }
    let Ok(jac) = HJac::new(&tr.f, p) else {
        rep.notes
            .push("deg F = 8: no J_H arithmetic, class tests skipped".into());
        return Ok(rep);
    };
    let n = rep.jac_h.unwrap();
    let rand_class = |rng: &mut SplitMix64| {
        let mut m = jac.zero();
        for _ in 0..3 {
            let a = pts[rng.below(pts.len() as u64) as usize];
            let b = pts[rng.below(pts.len() as u64) as usize];
            m = jac.add(&m, &jac.point(a.0, a.1));
            m = jac.add(&m, &jac.point(b.0, ng(b.1, p)));
        }
        m
    };
    let mut best: Option<(Mumford, PicClass, u128, u128)> = None;
    for _ in 0..o.classes {
        let d1 = rand_class(&mut rng);
        let d2 = rand_class(&mut rng);
        let (Ok(a1), Ok(a1b), Ok(a2)) = (
            tf.transfer_divisor(&jac, &d1, &pts, &mut rng),
            tf.transfer_divisor(&jac, &d1, &pts, &mut rng),
            tf.transfer_divisor(&jac, &d2, &pts, &mut rng),
        ) else {
            rep.notes.push("transfer_divisor failed".into());
            continue;
        };
        rep.representation.1 += 1;
        rep.representation.0 += (a1 == a1b) as usize;
        if let Ok(a12) = tf.transfer_divisor(&jac, &jac.add(&d1, &d2), &pts, &mut rng) {
            rep.additivity.1 += 1;
            rep.additivity.0 += (a12 == tf.c.add(&a1, &a2)) as usize;
        }
        rep.annihilated.1 += 1;
        rep.order_ratio.1 += 1;
        let oh = jac.order(&d1, n);
        if let Some(oc) = tf.pic_order(&a1, n) {
            rep.annihilated.0 += 1;
            if oh.is_multiple_of(oc) {
                if let Some(i) = [1u128, 2, 4, 8].iter().position(|&r| r == oh / oc) {
                    rep.order_ratio.0 += 1;
                    rep.ratio_hist[i] += 1;
                }
            }
            if best.as_ref().is_none_or(|b| oc > b.3) {
                best = Some((d1, a1, oh, oc));
            }
        }
    }
    // Rational 2-torsion against S.
    for u in rational_two_torsion(&tr.f, p, &mut rng) {
        let d = Mumford {
            u: u.clone(),
            v: Vec::new(),
        };
        let Ok(img) = tf.transfer_divisor(&jac, &d, &pts, &mut rng) else {
            rep.notes
                .push("transfer of a 2-torsion class failed".into());
            continue;
        };
        let in_s = two_torsion_in_s(tr, &u);
        rep.two_torsion.1 += 1;
        rep.two_torsion_in_s += in_s as usize;
        rep.two_torsion.0 += (tf.c.is_zero(&img) == in_s) as usize;
    }
    // Planted DLP.
    if let Some((d, img, oh, oc)) = best {
        let k = o.k % oh;
        let dk = jac.mul(&d, k);
        match tf.transfer_divisor(&jac, &dk, &pts, &mut rng) {
            Ok(imgk) => {
                let ok = imgk == tf.c.mul_scalar(&img, k);
                let rec = tf.pic_dlog(&img, &imgk, oc);
                rep.dlp = Some((k, oh, oc, rec, ok));
            }
            Err(e) => rep.notes.push(format!("DLP transfer failed: {e}")),
        }
    }
    Ok(rep)
}

#[cfg(test)]
mod tests {
    use super::super::plane_quartic_pic::tests as pq;
    use super::super::smith_prop24::{analyze_curve, pow_mod, EXAMPLE1_F, EXAMPLE1_ND, EXAMPLE1_P};
    use super::*;

    fn example1() -> Trigonal {
        let (n1, n0, d1, d0) = EXAMPLE1_ND;
        Trigonal::new(&EXAMPLE1_F, EXAMPLE1_P, n1, n0, d1, d0).unwrap()
    }

    /// The quartic printed in Smith's Example 1, in `mons_of_degree(4)` order
    /// with `(X, Y, Z) = (u, v, w)`.
    fn printed_c() -> [u32; 15] {
        // u^4 + 26u^3v + 2u^3w + 17u^2v^2 + 9u^2vw + 20u^2w^2 + 34uv^3
        // + 24uv^2w + 5uvw^2 + 36uw^3 + 19v^4 + 13v^3w + v^2w^2 + 23vw^3 + 5w^4
        [1, 26, 2, 17, 9, 20, 34, 24, 5, 36, 19, 13, 1, 23, 5]
    }

    /// `N_1, N_2, N_3` from the printed zeta numerator
    /// `37^3 T^6 + 4.37^2 T^5 - 6.37 T^4 - 240 T^3 - 6 T^2 + 4 T + 1`.
    const EX1_COUNTS: [u64; 3] = [42, 1342, 50070];

    /// Smith-positive curves as in `smith_trigonal`'s tests, `deg F = degf`.
    pub(super) fn random_positive(p: u64, want: usize, seed: u64, degf: usize) -> Vec<Trigonal> {
        let mut rng = SplitMix64(seed);
        let mut out = Vec::new();
        let mut tries = 0;
        while out.len() < want {
            tries += 1;
            assert!(tries < 200_000, "too few Smith-positive curves at p = {p}");
            let mut f: Poly = (0..degf).map(|_| rng.below(p)).collect();
            f.push(1 + rng.below(p - 1));
            let Some(co) = analyze_curve(&f, p, &mut rng) else {
                continue;
            };
            let found = co
                .subgroups
                .iter()
                .filter(|s| s.trig_rational() && s.iso_rational(false))
                .flat_map(|s| s.maps.iter())
                .filter(|m| m.valid() && m.alpha_chi == 1)
                .find_map(|m| m.map.smith_coeffs());
            if let Some((n1, n0, d1, d0)) = found {
                if let Ok(tr) = Trigonal::new(&f, p, n1, n0, d1, d0) {
                    out.push(tr);
                }
            }
        }
        out
    }

    #[test]
    fn ex1_printed_zeta_counts() {
        // 1 - e1 T + e2 T^2 - e3 T^3: e1 = -4, e2 = -6, e3 = 240.
        let (e1, e2, e3) = (-4i64, -6i64, 240i64);
        let s = [e1, e1 * e1 - 2 * e2, e1 * e1 * e1 - 3 * e1 * e2 + 3 * e3];
        for k in 0..3 {
            assert_eq!(
                EX1_COUNTS[k] as i64,
                37i64.pow(k as u32 + 1) + 1 - s[k],
                "k = {}",
                k + 1
            );
        }
        assert_eq!(jac_order(37, EX1_COUNTS), 55666);
    }

    #[test]
    fn counters_against_brute_force() {
        // The library's independent brute-force enumerator, small fields.
        let mut rng = 0x5eed_u64;
        for q in [7u32, 11] {
            let (f, _) =
                super::super::plane_quartic_pic::next_smooth_quartic_with_point(q, &mut rng);
            for k in 1..=3 {
                if q == 11 && k == 3 {
                    continue;
                }
                let ext = pq::Ext::new(q, k);
                let want = pq::ext_points(&ext, &f).len() as u64;
                assert_eq!(count_quartic_points(q as u64, &f, k), want, "q={q} k={k}");
            }
        }
        // H: against direct enumeration over F_p and the printed zeta.
        let tr = example1();
        for k in 1..=3 {
            assert_eq!(count_h_points(&tr.f, 37, k), EX1_COUNTS[k - 1]);
        }
    }

    #[test]
    fn ex1_canonical_quartic() {
        let cm = CanonicalMap::new(example1()).unwrap();
        eprintln!(
            "Example 1: C = {:?} ({} fit points)",
            cm.quartic, cm.fit_points
        );
        assert!(cm.fit_points >= 17);
        assert!(cm.is_smooth());
        // T is symmetric.
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(cm.tmat[i][j], cm.tmat[j][i]);
            }
        }
        for k in 1..=3 {
            assert_eq!(
                count_quartic_points(37, &cm.quartic, k),
                EX1_COUNTS[k - 1],
                "#C(F_37^{k})"
            );
            assert_eq!(
                count_quartic_points(37, &printed_c(), k),
                EX1_COUNTS[k - 1],
                "printed #C(F_37^{k})"
            );
        }
    }

    /// `psi` on every image point, by the `XPoint` input and by `(t, rho)`.
    #[test]
    fn ex1_psi_on_image_points() {
        let tr = example1();
        let cm = CanonicalMap::new(tr.clone()).unwrap();
        let k = F2::new(37);
        let mut n = 0;
        let mut unique = 0;
        for (x, y) in tr.points_over_u() {
            let ip = tr.image_points(x, y).unwrap();
            for q in &ip.points {
                let h = cm.psi_xpoint(q).unwrap();
                assert!(cm.on_quartic(&h));
                assert_eq!(cm.psi_b(k.c(q.t), q.mumford).unwrap(), h);
                assert!(k.is_zero(cm.plane_model(k.c(q.t), q.rho)));
                let lifts = cm.lifts_t_rho(k.c(q.t), q.rho).unwrap();
                assert!(lifts.contains(&q.mumford));
                if lifts.len() == 1 {
                    assert_eq!(cm.psi_t_rho(k.c(q.t), q.rho).unwrap(), h);
                    unique += 1;
                } else {
                    assert!(cm.psi_t_rho(k.c(q.t), q.rho).is_err());
                }
                n += 1;
            }
            // psi separates the two points of the pair.
            assert_ne!(
                cm.psi_xpoint(&ip.points[0]).unwrap(),
                cm.psi_xpoint(&ip.points[1]).unwrap()
            );
        }
        assert!(n >= 40 && unique >= 40, "{n} {unique}");
        eprintln!("psi on {n} image points; {unique} at smooth points of the plane model");
        // Points with t in F_{p^2} \ F_p: rho by search on Phi(t, rho) = 0.
        let mut hits = 0;
        let mut roots = 0;
        for ta in 0..12 {
            let t = Fp2 {
                a: ta,
                b: 1 + ta % 5,
            };
            for a in 0..37 {
                for b in 0..37 {
                    let rho = Fp2 { a, b };
                    if k.is_zero(rho) || !k.is_zero(cm.plane_model(t, rho)) {
                        continue;
                    }
                    roots += 1;
                    for bm in cm.lifts_t_rho(t, rho).unwrap() {
                        let h = cm.psi_b(t, bm).unwrap();
                        assert!(cm.on_quartic(&h), "t = {t:?} rho = {rho:?}");
                        assert!(!is_rational(&h));
                        hits += 1;
                    }
                }
            }
        }
        eprintln!("t in F_p^2: {roots} plane points, {hits} points of X mapped onto C");
        assert!(hits >= 5, "{hits}");
    }

    /// The fitted `C` is `F_37`-projectively equivalent to the printed one:
    /// exhaustive search for the projectivity through the rational points.
    #[test]
    fn ex1_equivalent_to_printed_quartic() {
        let p = 37u64;
        let cm = CanonicalMap::new(example1()).unwrap();
        let c1 = PlaneQuartic::new(
            37,
            cm.quartic,
            first_rational_point_canonical(37, &cm.quartic).unwrap(),
        )
        .unwrap();
        let pc = printed_c();
        let c2 =
            PlaneQuartic::new(37, pc, first_rational_point_canonical(37, &pc).unwrap()).unwrap();
        let a: Vec<[u64; 3]> = c1
            .rational_points_bruteforce()
            .iter()
            .map(|q| q.map(|x| x as u64))
            .collect();
        let b: Vec<[u64; 3]> = c2
            .rational_points_bruteforce()
            .iter()
            .map(|q| q.map(|x| x as u64))
            .collect();
        assert_eq!(a.len(), 42);
        assert_eq!(b.len(), 42);
        let det = |x: &[u64; 3], y: &[u64; 3], z: &[u64; 3]| -> u64 {
            let m = |a: u64, b: u64, c: u64, d: u64| sb(md(a, d, p), md(b, c, p), p);
            ad(
                sb(
                    md(x[0], m(y[1], y[2], z[1], z[2]), p),
                    md(x[1], m(y[0], y[2], z[0], z[2]), p),
                    p,
                ),
                md(x[2], m(y[0], y[1], z[0], z[1]), p),
                p,
            )
        };
        // Frame matrix [l1 p1 | l2 p2 | l3 p3] with p4 = sum l_i p_i (Cramer).
        let frame = |q: [&[u64; 3]; 4]| -> Option<[[u64; 3]; 3]> {
            let d = det(q[0], q[1], q[2]);
            let l = [
                det(q[3], q[1], q[2]),
                det(q[0], q[3], q[2]),
                det(q[0], q[1], q[3]),
            ];
            if d == 0 || l.contains(&0) {
                return None;
            }
            let mut m = [[0u64; 3]; 3];
            for c in 0..3 {
                for r in 0..3 {
                    m[r][c] = md(l[c], q[c][r], p);
                }
            }
            Some(m)
        };
        let mulm = |x: &[[u64; 3]; 3], y: &[[u64; 3]; 3]| {
            let mut m = [[0u64; 3]; 3];
            for r in 0..3 {
                for c in 0..3 {
                    for k in 0..3 {
                        m[r][c] = ad(m[r][c], md(x[r][k], y[k][c], p), p);
                    }
                }
            }
            m
        };
        let adjm = |x: &[[u64; 3]; 3]| {
            let mut m = [[0u64; 3]; 3];
            for r in 0..3 {
                for c in 0..3 {
                    let (r1, r2) = ((c + 1) % 3, (c + 2) % 3);
                    let (c1, c2) = ((r + 1) % 3, (r + 2) % 3);
                    m[r][c] = sb(md(x[r1][c1], x[r2][c2], p), md(x[r1][c2], x[r2][c1], p), p);
                }
            }
            m
        };
        // Four points of the fitted curve in general position.
        let mut src = None;
        'f: for i in 0..a.len() {
            for j in i + 1..a.len() {
                for k in j + 1..a.len() {
                    for l in k + 1..a.len() {
                        if let Some(m) = frame([&a[i], &a[j], &a[k], &a[l]]) {
                            src = Some(m);
                            break 'f;
                        }
                    }
                }
            }
        }
        let fa_inv = adjm(&src.unwrap());
        let on2 = |q: [u64; 3]| c2.eval_f(q.map(|x| x as u32)) == 0;
        let apply = |m: &[[u64; 3]; 3], q: &[u64; 3]| {
            let mut r = [0u64; 3];
            for i in 0..3 {
                for k in 0..3 {
                    r[i] = ad(r[i], md(m[i][k], q[k], p), p);
                }
            }
            r
        };
        let mut found = Vec::new();
        for i in 0..b.len() {
            for j in 0..b.len() {
                if j == i {
                    continue;
                }
                for k in 0..b.len() {
                    if k == i || k == j || det(&b[i], &b[j], &b[k]) == 0 {
                        continue;
                    }
                    for l in 0..b.len() {
                        if l == i || l == j || l == k {
                            continue;
                        }
                        let Some(fb) = frame([&b[i], &b[j], &b[k], &b[l]]) else {
                            continue;
                        };
                        let m = mulm(&fb, &fa_inv);
                        if a.iter().all(|q| on2(apply(&m, q))) {
                            found.push(m);
                        }
                    }
                }
            }
        }
        eprintln!(
            "projectivities C -> printed C: {}; first {:?}",
            found.len(),
            found.first()
        );
        assert!(
            !found.is_empty(),
            "no projectivity onto the printed quartic"
        );
        // 42 > 16 points of the image lie on the printed quartic: the image is it.
    }

    /// Classes of conjugate pairs: two rational line sections are linearly
    /// equivalent, so `[Q + Q' + R + R' + 4 P0 - 2K]` is the same for every
    /// rational line through a conjugate pair.
    #[test]
    fn ex1_pair_class_line_sections() {
        let tr = example1();
        let p = 37u64;
        let k = F2::new(p);
        let cm = CanonicalMap::new(tr.clone()).unwrap();
        let tf = Transfer::new(cm).unwrap();
        let m4 = mons_of_degree(4);
        let mut classes = Vec::new();
        let mut conj_pairs = 0;
        for (x, y) in tr.points_over_u() {
            let ip = tr.image_points(x, y).unwrap();
            let h1 = tf.cm.psi_xpoint(&ip.points[0]).unwrap();
            let h2 = tf.cm.psi_xpoint(&ip.points[1]).unwrap();
            // The rational line through h1, h2: points A + s B with A, B rational.
            let (pa, pb): ([u64; 3], [u64; 3]) = if is_rational(&h1) {
                (h1.map(|z| z.a), h2.map(|z| z.a))
            } else {
                conj_pairs += 1;
                (h1.map(|z| z.a), h1.map(|z| z.b))
            };
            // Restrict the quartic to A + s B: coefficients by interpolation at s = 0..4.
            let val = |s: u64| -> u64 {
                let q: Vec<u64> = (0..3).map(|i| ad(pa[i], md(s, pb[i], p), p)).collect();
                let mut acc = 0;
                for (i, &(a, b, c)) in m4.iter().enumerate() {
                    let t = md(
                        md(pow_mod(q[0], a as u128, p), pow_mod(q[1], b as u128, p), p),
                        pow_mod(q[2], c as u128, p),
                        p,
                    );
                    acc = ad(acc, md(t, tf.cm.quartic[i] as u64, p), p);
                }
                acc
            };
            let rows: Vec<Vec<u64>> = (0..5u64)
                .map(|s| {
                    let mut r: Vec<u64> = (0..5).map(|e| pow_mod(s, e, p)).collect();
                    r.push(ng(val(s), p));
                    r
                })
                .collect();
            let ns = nullspace(&rows, 6, p);
            assert_eq!(ns.len(), 1);
            let quart: Poly = ns[0][..5]
                .iter()
                .map(|&c| md(c, inv_mod(ns[0][5], p), p))
                .collect();
            // The known factor: s-parameters of h1, h2.
            let known: Poly = if is_rational(&h1) {
                // h1 = A (s = 0), h2 = B (s = infinity): quartic has degree 3 and root 0.
                assert_eq!(quart[0], 0);
                assert_eq!(*quart.get(4).unwrap_or(&0), 0);
                vec![0, 1]
            } else {
                // h1 = A + w B: s^2 = nr.
                vec![ng(k.nr, p), 0, 1]
            };
            let (res, rem) = pdivrem(&quart, &known, p);
            assert!(rem.is_empty());
            if deg(&res) != 2 {
                continue;
            }
            let res = pmonic(&res, p);
            let Some((s1, s2)) = k.quad_roots(res[1], res[0]) else {
                continue;
            };
            let pt = |s: Fp2| -> [Fp2; 3] {
                let mut q = [Fp2::default(); 3];
                for i in 0..3 {
                    q[i] = k.add(k.c(pa[i]), k.scale(s, pb[i]));
                }
                q
            };
            let (Ok(c1), Ok(c2)) = (tf.pair_class(h1, h2), tf.pair_class(pt(s1), pt(s2))) else {
                continue;
            };
            classes.push(tf.c.add(&c1, &c2));
        }
        eprintln!(
            "line sections: {} tested, {} through conjugate pairs",
            classes.len(),
            conj_pairs
        );
        assert!(classes.len() >= 10 && conj_pairs >= 5);
        assert!(classes.iter().all(|c| *c == classes[0]));
    }

    /// The full instrument on Example 1, with the paper's `k = 22359`.
    #[test]
    fn ex1_transfer() {
        let tr = example1();
        let r = verify_curve(
            &tr,
            &VerifyOpts {
                kmax: 3,
                relations: 10,
                classes: 10,
                k: 22359,
                seed: 0x5317_ca00,
            },
        )
        .unwrap();
        eprintln!("EX1 {}", r.row());
        eprintln!("EX1 notes {:?}", r.notes);
        assert_eq!(r.n_h, EX1_COUNTS.to_vec());
        assert_eq!(r.jac_h, Some(55666));
        assert!(r.all_pass(), "{r:?}");
        assert!(r.relations.1 >= 5 && r.representation.1 >= 5);
        let d = r.dlp.unwrap();
        assert_eq!(d.0, 22359 % d.1);
    }

    /// The trivial identity, as a check of the Pic arithmetic only:
    /// `Psi([P - P']) + Psi([P' - P'']) = Psi([P - P''])`.
    #[test]
    fn ex1_formal_additivity() {
        let tf = Transfer::new(CanonicalMap::new(example1()).unwrap()).unwrap();
        let pts = tf.usable_points();
        let mut rng = SplitMix64(7);
        for _ in 0..50 {
            let a = pts[rng.below(pts.len() as u64) as usize];
            let b = pts[rng.below(pts.len() as u64) as usize];
            let c = pts[rng.below(pts.len() as u64) as usize];
            let ab = tf.point_sum(&[a], &[b]).unwrap();
            let bc = tf.point_sum(&[b], &[c]).unwrap();
            let ac = tf.point_sum(&[a], &[c]).unwrap();
            assert_eq!(tf.c.add(&ab, &bc), ac);
        }
    }

    /// A broken map must be caught: `psi` with a wrong `T` gives no quartic.
    #[test]
    fn negative_control_wrong_matrix() {
        let tr = example1();
        let mut cm = CanonicalMap::new(tr.clone()).unwrap();
        // Drop the factor D: T'_ik = [x^2](x^(i+k) adj(f)) is not the trace of
        // the differentials.
        cm.tmat[0][0] = padd(&cm.tmat[0][0], &[1], 37);
        let mut pts = Vec::new();
        for (x, y) in tr.points_over_u() {
            let ip = tr.image_points(x, y).unwrap();
            for q in &ip.points {
                if let Ok(h) = cm.psi_xpoint(q) {
                    pts.push(h);
                }
            }
        }
        assert!(fit_quartic(37, &pts).is_err());
    }

    /// The frozen sweep of `research/smith_canonical_20261115/PROTOCOL.md`.
    /// `SMITH_CANONICAL_SWEEP=<p>:<degF>:<count>:<kmax>:<seed>` runs one cell.
    #[test]
    #[ignore]
    fn sweep() {
        let spec = std::env::var("SMITH_CANONICAL_SWEEP").expect("SMITH_CANONICAL_SWEEP");
        let v: Vec<u64> = spec
            .split(':')
            .map(|s| {
                s.strip_prefix("0x")
                    .map(|h| u64::from_str_radix(h, 16).unwrap())
                    .unwrap_or_else(|| s.parse().unwrap())
            })
            .collect();
        let (p, degf, count, kmax, seed) =
            (v[0], v[1] as usize, v[2] as usize, v[3] as usize, v[4]);
        let curves = random_positive(p, count, seed, degf);
        let mut pass = 0;
        for (i, tr) in curves.iter().enumerate() {
            println!(
                "CURVE p={p} i={i} F={:?} ND=({},{},{},{})",
                tr.f, tr.n1, tr.n0, tr.d1, tr.d0
            );
            match verify_curve(
                tr,
                &VerifyOpts {
                    kmax,
                    relations: 10,
                    classes: 10,
                    k: 22359,
                    seed: seed ^ (i as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15),
                },
            ) {
                Ok(r) => {
                    pass += r.all_pass() as usize;
                    println!("ROW p={p} i={i} {}", r.row());
                    if !r.notes.is_empty() {
                        println!("NOTES p={p} i={i} {:?}", r.notes);
                    }
                }
                Err(e) => println!("FAIL p={p} i={i} canonical map not built: {e}"),
            }
        }
        println!(
            "CELL p={p} degF={degf} kmax={kmax} seed={seed:#x} curves={count} all_pass={pass}"
        );
    }
}
