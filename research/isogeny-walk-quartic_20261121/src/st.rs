//! The explicit part of Smith's trigonal construction: Recillas' curve `X`
//! in Smith's affine model and the (3,2)-correspondence `R` between `H` and
//! `X` that induces the isogeny `phi: J_H -> J_X` with kernel `S`.
//!
//! Source: B. Smith, "Isogenies and the discrete logarithm problem in
//! Jacobians of genus 3 hyperelliptic curves", J. Cryptology 22 (2009),
//! §6 (eqs. (5)-(11), Lemma 4, Propositions 3 and 4) and §7 (Example 1).
//!
//! # What is built
//!
//! For `H: y^2 = F(x)` (`deg F` 7 or 8) over `F_p` (odd `p < 2^31`) and a
//! trigonal map `g(x) = N(x)/D(x) = (x^3 + n1 x + n0)/(x^2 + d1 x + d0)`:
//!
//! * `G(t, x) = N(x) - t D(x) = x^3 + g2 x^2 + g1 x + g0` and
//!   `f0 + f1 x + f2 x^2 = F mod G` (computed by
//!   [`smith_prop24::g_coeffs`](crypto_lib::cryptanalysis::smith_prop24::g_coeffs) and
//!   [`smith_prop24::f_mod_g`](crypto_lib::cryptanalysis::smith_prop24::f_mod_g));
//! * the affine model `X|U ⊂ U x A^6` (§6, after eq. (6)): the three linear
//!   equations `c0, c1, c2` and the six Veronese quadrics in
//!   `(b00, b01, b02, b11, b12, b22)`;
//! * `s(t)` of eq. (7), its square root in `F_p[t]` (which exists iff
//!   Proposition 4 holds; otherwise [`Trigonal::new`] returns `Err`), and the
//!   `delta_0, delta_1 = 8 sqrt(s), delta_2, delta_4` of Proposition 3, so
//!   that `rho = (delta_4 b22^2 + delta_2 b22 + delta_0) / delta_1` is the
//!   square root of `b22` on `X|U`;
//! * the correspondence `R = V(G(t, x), y - (b02 + b12 x + b22 x^2)/rho)` of
//!   eq. (11), as the map [`Trigonal::image_points`] sending an affine point
//!   `P` of `H` over `U` to the two points of `X` paired with it by `R`.
//!
//! # Reconstruction of Proposition 3
//!
//! The printed `delta_2` is garbled in the extracted text, so it was rederived
//! from the proof.  Write `d_i = (x_i - x_j)(x_i - x_k)`, so
//! `1/d_i^2 = Delta_i / Delta` and `d_1 d_2 d_3 = -Delta`.  For a triple
//! `(y_1, y_2, y_3)` the Lagrange leading coefficient is
//! `b2 = sum y_i / d_i`, and with `Gamma_i = F(x_i) Delta_i`:
//!
//! ```text
//! (b22 - sum Gamma_i / Delta)^2 - 4 sum_{i<j} Gamma_i Gamma_j / Delta^2
//!      = 8 y1 y2 y3 b2 / (d1 d2 d3) = -8 y1 y2 y3 b2 / Delta,
//! ```
//!
//! i.e. `Delta b22^2 - 2 (sum Gamma_i) b22 + (2 sum Gamma_i^2 - (sum Gamma_i)^2)/Delta
//! = -8 y1 y2 y3 b2` (squaring gives (10), with `prod Gamma_i = s Delta`).
//! Substituting `e1 = -g2, e2 = g1, e3 = -g0`:
//! `sum Delta_i = 2 g2^2 - 6 g1`, `sum x_i Delta_i = 9 g0 - g1 g2`,
//! `sum x_i^2 Delta_i = 2 g1^2 - 6 g0 g2`, hence
//!
//! ```text
//! delta_4 = Delta = -27 g0^2 + 18 g0 g1 g2 - 4 g0 g2^3 - 4 g1^3 + g1^2 g2^2,
//! delta_2 = -2 sum Gamma_i
//!         = 12 f0 g1 - 4 f0 g2^2 - 18 f1 g0 + 2 f1 g1 g2 + 12 f2 g0 g2 - 4 f2 g1^2,
//! delta_0 = (2 sum Gamma_i^2 - (sum Gamma_i)^2) / Delta = f1^2 - 4 f0 f2,
//! ```
//!
//! which is exactly what `smith_prop24::smith_deltas` already implements and
//! what Example 1 prints.  The identity also fixes the branch: a triple `T`
//! has `b2(T) = rho` iff `y1 y2 y3 = -delta_1(t)/8 = -sqrt(s)(t)`.
//! `sqrt(s)` is normalised so that its leading coefficient is the smaller
//! representative in `[0, p/2]`; for Example 1 this reproduces the printed
//! `delta_1 = 35t^3 + 8t^2 + 33t + 3` with the same sign.
//!
//! # What is verified (unit tests)
//!
//! * T1: Example 1 (`F_37`): `g_i`, `f_i`, `delta_0, delta_1, delta_2, delta_4`
//!   and the printed equations of `X` are reproduced; `s = (delta_1/8)^2`.
//! * T2: every image point lies on `X|U` (`c0, c1, c2`, the Veronese
//!   quadrics, eq. (8), `b22 = rho^2`, and `y = (b02 + b12 x + b22 x^2)/rho`).
//! * T3: two image points per `P`; with those of `iota(P)` they are the four
//!   distinct points of the fibre (5).
//! * T4: `#X(F_p)` over `U` counted two independent ways (enumerating the
//!   model, and the Galois-stable partitions of the fibres of `G`), plus the
//!   fibres outside `U` by the combinatorial rule of
//!   `smith_prop24::x_point_count`, against `#H(F_p)` (Smith, Remark 4).
//! * T5: `phi([P - P'])` as the formal sum `Q1(P) + Q2(P) - Q1(P') - Q2(P')`
//!   of points on `X|U` over the right `t`.  Linear equivalence on `X`
//!   cannot be tested here: there is no `J_X` arithmetic.
//!
//! # What is NOT built
//!
//! The canonical embedding `psi: X -> C ⊂ P^2` by a basis of the
//! Riemann–Roch space of a canonical divisor (Smith §7 delegates it to
//! Magma), the plane quartic `C`, and any `J_X` / `J_C` arithmetic.  The
//! fibre of `X` over `t = infinity` and over the finitely many `t` outside
//! `U` is not in the affine model; the point count only infers it.

use crypto_lib::cryptanalysis::smith_prop24::{
    chi as lib_chi, deg, f_mod_g, g_coeffs, inv_mod as lib_inv_mod, least_nonsquare,
    peval as lib_peval, smith_deltas, smith_s, sqrt_mod as lib_sqrt_mod, Poly,
};
use std::cell::Cell;

// ---- COUNTING HOOKS (round 2 of research/isogeny-walk-quartic): this file is a COPY of
// src/cryptanalysis/smith_trigonal.rs (tests removed) in which every F_p multiplication done by `md`
// is counted, and the library helpers are wrapped: `inv_mod` counts one inversion, `chi` and
// `sqrt_mod` are MODELLED (1.5 log2 p and 4 log2 p multiplications), `peval` counts len mults.
thread_local! {
    static C_M: Cell<u64> = const { Cell::new(0) };
    static C_I: Cell<u64> = const { Cell::new(0) };
}

/// (F_p multiplications, inversions) counted so far on this thread.
pub fn snap() -> (u64, u64) {
    (C_M.with(|c| c.get()), C_I.with(|c| c.get()))
}

fn cm(n: u64) {
    C_M.with(|c| c.set(c.get() + n));
}

/// Charge `n` F_p multiplications done outside this file (planefit).
pub fn count_m(n: u64) {
    cm(n);
}

/// Charge `n` F_p inversions done outside this file (planefit).
pub fn count_i(n: u64) {
    C_I.with(|c| c.set(c.get() + n));
}

fn log2p(p: u64) -> u64 {
    64 - u64::from(p.leading_zeros())
}

fn inv_mod(a: u64, p: u64) -> u64 {
    C_I.with(|c| c.set(c.get() + 1));
    lib_inv_mod(a, p)
}

fn chi(a: u64, p: u64) -> i8 {
    cm(3 * log2p(p) / 2);
    lib_chi(a, p)
}

fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    cm(4 * log2p(p));
    lib_sqrt_mod(a, p)
}

fn peval(a: &[u64], x: u64, p: u64) -> u64 {
    cm(a.len() as u64);
    lib_peval(a, x, p)
}

// ---------------------------------------------------------------------------
// F_p helpers (p odd, below 2^31).

#[inline]
fn md(a: u64, b: u64, p: u64) -> u64 {
    cm(1);
    if p < (1 << 32) {
        a * b % p
    } else {
        ((a as u128 * b as u128) % p as u128) as u64
    }
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

/// `c * h` with `h` monic, `c^2 = lc(s)`, `h^2 = s / lc(s)`; `c` is the
/// representative in `[0, p/2]`.  `None` if `s` is not a square in `F_p[t]`.
fn poly_sqrt(s: &[u64], p: u64) -> Option<Poly> {
    let n = deg(s);
    if n < 0 || n % 2 == 1 {
        return None;
    }
    let lc = s[n as usize];
    let c = sqrt_mod(lc, p)?; // smaller representative
    let sm = pscale(s, inv_mod(lc, p), p);
    let m = (n / 2) as usize;
    let mut h = vec![0u64; m + 1];
    h[m] = 1;
    let inv2 = inv_mod(2, p);
    for k in 1..=m {
        let mut acc = 0;
        for i in (m - k + 1)..=m {
            let j = 2 * m - k - i;
            if j > m - k && j <= m {
                acc = ad(acc, md(h[i], h[j], p), p);
            }
        }
        h[m - k] = md(sb(sm[2 * m - k], acc, p), inv2, p);
    }
    (pmul(&h, &h, p) == sm).then(|| pscale(&h, c, p))
}

// ---------------------------------------------------------------------------
// F_{p^2} = F_p[w] / (w^2 - nr).

/// An element `a + b w` of `F_{p^2}`, `w^2 = nr` (the least non-residue).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Fp2 {
    pub a: u64,
    pub b: u64,
}

impl Fp2 {
    pub fn is_rational(&self) -> bool {
        self.b == 0
    }
}

/// Arithmetic context for [`Fp2`].
#[derive(Clone, Copy, Debug)]
pub struct F2 {
    pub p: u64,
    pub nr: u64,
}

impl F2 {
    pub fn new(p: u64) -> F2 {
        F2 {
            p,
            nr: least_nonsquare(p),
        }
    }
    pub fn c(&self, x: u64) -> Fp2 {
        Fp2 {
            a: x % self.p,
            b: 0,
        }
    }
    pub fn add(&self, x: Fp2, y: Fp2) -> Fp2 {
        Fp2 {
            a: ad(x.a, y.a, self.p),
            b: ad(x.b, y.b, self.p),
        }
    }
    pub fn sub(&self, x: Fp2, y: Fp2) -> Fp2 {
        Fp2 {
            a: sb(x.a, y.a, self.p),
            b: sb(x.b, y.b, self.p),
        }
    }
    pub fn neg(&self, x: Fp2) -> Fp2 {
        Fp2 {
            a: ng(x.a, self.p),
            b: ng(x.b, self.p),
        }
    }
    pub fn mul(&self, x: Fp2, y: Fp2) -> Fp2 {
        let p = self.p;
        Fp2 {
            a: ad(md(x.a, y.a, p), md(self.nr, md(x.b, y.b, p), p), p),
            b: ad(md(x.a, y.b, p), md(x.b, y.a, p), p),
        }
    }
    pub fn scale(&self, x: Fp2, c: u64) -> Fp2 {
        self.mul(x, self.c(c))
    }
    /// Frobenius: `a - b w`.
    pub fn conj(&self, x: Fp2) -> Fp2 {
        Fp2 {
            a: x.a,
            b: ng(x.b, self.p),
        }
    }
    /// `x * conj(x)` in `F_p`.
    pub fn norm(&self, x: Fp2) -> u64 {
        self.mul(x, self.conj(x)).a
    }
    pub fn inv(&self, x: Fp2) -> Fp2 {
        let n = self.norm(x);
        assert!(n != 0, "inverse of zero in F_p^2");
        self.scale(self.conj(x), inv_mod(n, self.p))
    }
    pub fn is_zero(&self, x: Fp2) -> bool {
        x.a == 0 && x.b == 0
    }
    /// A square root in `F_{p^2}`, `None` if `x` is not a square there.
    pub fn sqrt(&self, x: Fp2) -> Option<Fp2> {
        let p = self.p;
        let r = if x.b == 0 {
            match sqrt_mod(x.a, p) {
                Some(r) => Fp2 { a: r, b: 0 },
                None => Fp2 {
                    a: 0,
                    b: sqrt_mod(md(x.a, inv_mod(self.nr, p), p), p)?,
                },
            }
        } else {
            let m = sqrt_mod(self.norm(x), p)?;
            let inv2 = inv_mod(2, p);
            let mut h = md(ad(x.a, m, p), inv2, p);
            if chi(h, p) != 1 {
                h = md(sb(x.a, m, p), inv2, p);
            }
            let c = sqrt_mod(h, p)?;
            if c == 0 {
                return None;
            }
            Fp2 {
                a: c,
                b: md(x.b, inv_mod(md(2, c, p), p), p),
            }
        };
        (self.mul(r, r) == x).then_some(r)
    }
    /// `poly(z)` for a polynomial over `F_p` (low degree first).
    pub fn eval(&self, poly: &[u64], z: Fp2) -> Fp2 {
        poly.iter().rev().fold(Fp2::default(), |acc, &c| {
            self.add(self.mul(acc, z), self.c(c))
        })
    }
    /// The two roots of the monic `X^2 + q1 X + q0` over `F_p`, distinct.
    pub fn quad_roots(&self, q1: u64, q0: u64) -> Option<(Fp2, Fp2)> {
        let p = self.p;
        let disc = sb(md(q1, q1, p), md(4, q0, p), p);
        if disc == 0 {
            return None;
        }
        let r = self.sqrt(self.c(disc))?;
        let inv2 = inv_mod(2, p);
        let mq = self.c(ng(q1, p));
        Some((
            self.scale(self.add(mq, r), inv2),
            self.scale(self.sub(mq, r), inv2),
        ))
    }
}

// ---------------------------------------------------------------------------
// The construction.

/// The explicit data of Smith §6 for one trigonal map in normal form.
#[derive(Clone, Debug)]
pub struct Trigonal {
    pub p: u64,
    /// `F`, low degree first.
    pub f: Vec<u64>,
    pub n1: u64,
    pub n0: u64,
    pub d1: u64,
    pub d0: u64,
    /// `[g0, g1, g2]` in `F_p[t]`.
    pub g: [Poly; 3],
    /// `[f0, f1, f2]` in `F_p[t]`, `f0 + f1 x + f2 x^2 = F mod G(t, x)`.
    pub fpoly: [Poly; 3],
    /// `s(t)` of eq. (7).
    pub s: Poly,
    /// The square root of `s` with leading coefficient in `[0, p/2]`.
    pub sqrt_s: Poly,
    /// `[delta_0, delta_1 = 8 sqrt_s, delta_2, delta_4]` (Proposition 3).
    pub delta: [Poly; 4],
}

/// A point of `X|U ⊂ U x A^6`, coordinates in `F_{p^2}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XPoint {
    pub t: u64,
    /// `(b00, b01, b02, b11, b12, b22)`.
    pub b: [Fp2; 6],
    /// `rho = (delta_4 b22^2 + delta_2 b22 + delta_0) / delta_1`, `rho^2 = b22`.
    pub rho: Fp2,
    /// The Mumford `b(X) = b0 + b1 X + b2 X^2` of the `R`-triple (`b2 = rho`).
    pub mumford: [Fp2; 3],
}

/// The two points of `X` paired with an affine point `P` of `H` by `R`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImagePair {
    pub points: [XPoint; 2],
    /// Both points are `F_p`-rational; otherwise they are Frobenius conjugates.
    pub rational: bool,
}

/// Per-curve point counts for T4.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CountReport {
    pub p: u64,
    pub h_points: u64,
    /// Affine points of `H` with `D(x) != 0` and `g(x) in U`.
    pub h_over_u: u64,
    /// `t in F_p` in `U` (with `delta_1(t) != 0`).
    pub t_in_u: u64,
    /// `#X(F_p)` over `U` from the model (rational `X~` points / 2).
    pub x_over_u_model: u64,
    /// `#X(F_p)` over `U` from Galois-stable partitions of the fibres.
    pub x_over_u_fibres: u64,
    /// `X~` points with `b2 = 0` over `U` (must be 0: `b22` is a unit).
    pub model_b2_zero: u64,
    /// Finite `t` outside `U`.
    pub t_outside_u: Vec<u64>,
    /// `#X(F_p)` over the finite `t` outside `U` by the fibre rule, `None` if
    /// some fibre is of a type the rule does not cover.
    pub x_outside_u_finite: Option<u64>,
    /// `#H(F_p) - #X(F_p over all finite t)`: what the fibre over
    /// `t = infinity` must contribute if `#X(F_p) = #H(F_p)`.
    pub implied_x_infinity: Option<i64>,
}

impl Trigonal {
    /// Build the data of §6.  `Err` when Proposition 4 fails (`s` has no
    /// square root in `F_p[t]`), or on degenerate input.
    pub fn new(f: &[u64], p: u64, n1: u64, n0: u64, d1: u64, d0: u64) -> Result<Trigonal, String> {
        if p.is_multiple_of(2) || !(7..(1u64 << 62)).contains(&p) {
            return Err(format!("p = {p} must be an odd prime in [7, 2^62)"));
        }
        let mut f: Poly = f.iter().map(|&c| c % p).collect();
        trim(&mut f);
        if !(7..=8).contains(&deg(&f)) {
            return Err(format!("deg F = {} (need 7 or 8)", deg(&f)));
        }
        let (n1, n0, d1, d0) = (n1 % p, n0 % p, d1 % p, d0 % p);
        let g = g_coeffs(n1, n0, d1, d0, p);
        let fpoly = f_mod_g(&f, n1, n0, d1, d0, p);
        let s = smith_s(&fpoly, &g, p);
        if s.is_empty() {
            return Err("s(t) = 0".into());
        }
        let sqrt_s = poly_sqrt(&s, p).ok_or_else(|| {
            format!(
                "Proposition 4 fails: s(t) is not a square in F_p[t] (lc = {}, chi = {})",
                s[s.len() - 1],
                chi(s[s.len() - 1], p)
            )
        })?;
        let [d0p, d2p, d4p] = smith_deltas(&fpoly, &g, p);
        let d1p = pscale(&sqrt_s, 8, p);
        Ok(Trigonal {
            p,
            f,
            n1,
            n0,
            d1,
            d0,
            g,
            fpoly,
            s,
            sqrt_s,
            delta: [d0p, d1p, d2p, d4p],
        })
    }

    /// `D(x)`.
    pub fn den(&self, x: u64) -> u64 {
        let p = self.p;
        ad(md(x, ad(x, self.d1, p), p), self.d0, p)
    }

    /// `t = g(x)`, `None` where `D(x) = 0`.
    pub fn g_of(&self, x: u64) -> Option<u64> {
        let p = self.p;
        let x = x % p;
        let dx = self.den(x);
        if dx == 0 {
            return None;
        }
        let nx = ad(md(x, ad(md(x, x, p), self.n1, p), p), self.n0, p);
        Some(md(nx, inv_mod(dx, p), p))
    }

    /// Smith's `U`: `delta_4(t) delta_0(t) != 0`.
    pub fn in_u_paper(&self, t: u64) -> bool {
        peval(&self.delta[3], t, self.p) != 0 && peval(&self.delta[0], t, self.p) != 0
    }

    /// The `U` used here: Smith's `U` and `delta_1(t) != 0` (no Weierstrass
    /// point in the fibre), where `rho` is defined.
    pub fn in_u(&self, t: u64) -> bool {
        self.in_u_paper(t) && peval(&self.delta[1], t, self.p) != 0
    }

    /// `[f0, f1, f2, g0, g1, g2, delta_0, delta_1, delta_2, delta_4]` at `t`.
    fn at(&self, t: u64) -> [u64; 10] {
        let p = self.p;
        let e = |q: &Poly| peval(q, t, p);
        [
            e(&self.fpoly[0]),
            e(&self.fpoly[1]),
            e(&self.fpoly[2]),
            e(&self.g[0]),
            e(&self.g[1]),
            e(&self.g[2]),
            e(&self.delta[0]),
            e(&self.delta[1]),
            e(&self.delta[2]),
            e(&self.delta[3]),
        ]
    }

    /// For `c0, c1, c2`: `[coefficient of b22, coefficient of b12, constant]`
    /// in `F_p[t]` (the `b00, b01, b02, b11` coefficients are `1; 2; 2, 1`).
    pub fn x_equations(&self) -> [[Poly; 3]; 3] {
        let p = self.p;
        let [g0, g1, g2] = &self.g;
        let neg = |a: &Poly| pscale(a, p - 1, p);
        [
            [pmul(g2, g0, p), pscale(g0, p - 2, p), neg(&self.fpoly[0])],
            [
                padd(&pmul(g2, g1, p), &neg(g0), p),
                pscale(g1, p - 2, p),
                neg(&self.fpoly[1]),
            ],
            [
                padd(&pmul(g2, g2, p), &neg(g1), p),
                pscale(g2, p - 2, p),
                neg(&self.fpoly[2]),
            ],
        ]
    }

    /// `rho` at `(t, b22)`.
    pub fn rho(&self, k: &F2, t: u64, b22: Fp2) -> Fp2 {
        let v = self.at(t);
        let (dl0, dl1, dl2, dl4) = (v[6], v[7], v[8], v[9]);
        let num = k.add(
            k.add(k.scale(k.mul(b22, b22), dl4), k.scale(b22, dl2)),
            k.c(dl0),
        );
        k.scale(num, inv_mod(dl1, self.p))
    }

    /// Check a point against `X|U`: `c0, c1, c2`, the six Veronese quadrics,
    /// eq. (8), and `b22 = rho^2` with `rho` from Proposition 3.
    pub fn check_point(&self, k: &F2, q: &XPoint) -> Result<(), String> {
        let v = self.at(q.t);
        let [f0, f1, f2, g0, g1, g2, dl0, dl1, dl2, dl4] = v;
        let p = self.p;
        let [b00, b01, b02, b11, b12, b22] = q.b;
        let c0 = k.add(
            k.add(
                k.scale(b22, md(g2, g0, p)),
                k.scale(b12, ng(md(2, g0, p), p)),
            ),
            k.sub(b00, k.c(f0)),
        );
        let c1 = k.add(
            k.add(
                k.scale(b22, sb(md(g2, g1, p), g0, p)),
                k.scale(b12, ng(md(2, g1, p), p)),
            ),
            k.sub(k.scale(b01, 2), k.c(f1)),
        );
        let c2 = k.add(
            k.add(
                k.scale(b22, sb(md(g2, g2, p), g1, p)),
                k.scale(b12, ng(md(2, g2, p), p)),
            ),
            k.sub(k.add(k.scale(b02, 2), b11), k.c(f2)),
        );
        for (name, c) in [("c0", c0), ("c1", c1), ("c2", c2)] {
            if !k.is_zero(c) {
                return Err(format!("{name} != 0 at t = {}", q.t));
            }
        }
        let quadrics = [
            k.sub(k.mul(b01, b01), k.mul(b00, b11)),
            k.sub(k.mul(b01, b02), k.mul(b00, b12)),
            k.sub(k.mul(b02, b02), k.mul(b00, b22)),
            k.sub(k.mul(b02, b11), k.mul(b01, b12)),
            k.sub(k.mul(b02, b12), k.mul(b01, b22)),
            k.sub(k.mul(b12, b12), k.mul(b11, b22)),
        ];
        if let Some(i) = quadrics.iter().position(|&z| !k.is_zero(z)) {
            return Err(format!("Veronese quadric {i} != 0 at t = {}", q.t));
        }
        let a = k.add(
            k.add(k.scale(k.mul(b22, b22), dl4), k.scale(b22, dl2)),
            k.c(dl0),
        );
        let e8 = k.sub(k.mul(a, a), k.scale(b22, md(dl1, dl1, p)));
        if !k.is_zero(e8) {
            return Err(format!("eq. (8) fails at t = {}", q.t));
        }
        let rho = self.rho(k, q.t, b22);
        if rho != q.rho || k.mul(rho, rho) != b22 {
            return Err(format!("b22 != rho^2 at t = {}", q.t));
        }
        Ok(())
    }

    /// The four points of `X` over `t = g(x)` (eq. (5)), each with the
    /// Mumford `b` of its triple through `(x, y)`, and whether that triple is
    /// the `R`-triple (`b2 = rho`).
    pub fn fibre_points(&self, x: u64, y: u64) -> Result<Vec<(XPoint, bool)>, String> {
        let p = self.p;
        let (x, y) = (x % p, y % p);
        let k = F2::new(p);
        if md(y, y, p) != peval(&self.f, x, p) {
            return Err(format!("({x}, {y}) is not on H"));
        }
        let t = self.g_of(x).ok_or("D(x) = 0: P lies over t = infinity")?;
        if !self.in_u_paper(t) {
            return Err(format!("t = {t} is outside Smith's U"));
        }
        if !self.in_u(t) {
            return Err(format!(
                "t = {t}: delta_1(t) = 0 (Weierstrass point in the fibre), rho undefined"
            ));
        }
        let [_, _, _, _, g1, g2, ..] = self.at(t);
        // G(t, X) / (X - x) = X^2 + q1 X + q0.
        let q1 = ad(g2, x, p);
        let q0 = ad(g1, md(x, q1, p), p);
        let (x2, x3) = k.quad_roots(q1, q0).ok_or("repeated root of G(t, .)")?;
        let xs = [k.c(x), x2, x3];
        let y2 = k
            .sqrt(k.eval(&self.f, x2))
            .ok_or("F(x2) is not a square in F_p^2")?;
        let y3 = k
            .sqrt(k.eval(&self.f, x3))
            .ok_or("F(x3) is not a square in F_p^2")?;
        let d: Vec<Fp2> = (0..3)
            .map(|i| {
                let (j, l) = ((i + 1) % 3, (i + 2) % 3);
                k.inv(k.mul(k.sub(xs[i], xs[j]), k.sub(xs[i], xs[l])))
            })
            .collect();
        let mut out = Vec::with_capacity(4);
        for (e2, e3) in [(false, false), (false, true), (true, false), (true, true)] {
            let ys = [
                k.c(y),
                if e2 { k.neg(y2) } else { y2 },
                if e3 { k.neg(y3) } else { y3 },
            ];
            let mut bm = [Fp2::default(); 3];
            for i in 0..3 {
                let (j, l) = ((i + 1) % 3, (i + 2) % 3);
                let w = k.mul(ys[i], d[i]);
                bm[2] = k.add(bm[2], w);
                bm[1] = k.sub(bm[1], k.mul(w, k.add(xs[j], xs[l])));
                bm[0] = k.add(bm[0], k.mul(w, k.mul(xs[j], xs[l])));
            }
            let m = |i: usize, j: usize| k.mul(bm[i], bm[j]);
            let b = [m(0, 0), m(0, 1), m(0, 2), m(1, 1), m(1, 2), m(2, 2)];
            let rho = self.rho(&k, t, b[5]);
            let on_r = rho == bm[2];
            if !on_r && rho != k.neg(bm[2]) {
                return Err(format!("rho != +-b2 at t = {t}"));
            }
            out.push((
                XPoint {
                    t,
                    b,
                    rho,
                    mumford: bm,
                },
                on_r,
            ));
        }
        Ok(out)
    }

    /// The two points of `X` that `R` (eq. (11)) pairs with `P = (x, y)`.
    pub fn image_points(&self, x: u64, y: u64) -> Result<ImagePair, String> {
        let k = F2::new(self.p);
        let pts: Vec<XPoint> = self
            .fibre_points(x, y)?
            .into_iter()
            .filter(|(_, r)| *r)
            .map(|(q, _)| q)
            .collect();
        if pts.len() != 2 {
            return Err(format!("{} R-points over P, expected 2", pts.len()));
        }
        let rat = |q: &XPoint| q.b.iter().all(Fp2::is_rational) && q.rho.is_rational();
        let rational = rat(&pts[0]) && rat(&pts[1]);
        if !rational {
            let conj = |q: &XPoint| XPoint {
                t: q.t,
                b: q.b.map(|z| k.conj(z)),
                rho: k.conj(q.rho),
                mumford: q.mumford.map(|z| k.conj(z)),
            };
            if conj(&pts[0]) != pts[1] {
                return Err("image points neither rational nor conjugate".into());
            }
        }
        Ok(ImagePair {
            points: [pts[0], pts[1]],
            rational,
        })
    }

    /// The affine points of `H` over `U`.
    pub fn points_over_u(&self) -> Vec<(u64, u64)> {
        let p = self.p;
        let mut v = Vec::new();
        for x in 0..p {
            let Some(t) = self.g_of(x) else { continue };
            if !self.in_u(t) {
                continue;
            }
            if let Some(y) = sqrt_mod(peval(&self.f, x, p), p) {
                v.push((x, y));
                if y != 0 {
                    v.push((x, p - y));
                }
            }
        }
        v
    }

    /// Rational points of `X~` over `t` (`(b0, b1, b2) in F_p^3` on
    /// `c~0, c~1, c~2`), as `(total, with b2 = 0)`.  O(p^2).
    fn model_count_at(&self, t: u64) -> (u64, u64) {
        let p = self.p;
        let [f0, f1, f2, g0, g1, g2, ..] = self.at(t);
        let (a0, a1, a2) = (
            md(g2, g0, p),
            sb(md(g2, g1, p), g0, p),
            sb(md(g2, g2, p), g1, p),
        );
        let on = |b0: u64, b1: u64, b2: u64| {
            let b22 = md(b2, b2, p);
            let b12 = md(b1, b2, p);
            let c0 = sb(
                ad(
                    sb(md(a0, b22, p), md(2, md(g0, b12, p), p), p),
                    md(b0, b0, p),
                    p,
                ),
                f0,
                p,
            );
            let c1 = sb(
                ad(
                    sb(md(a1, b22, p), md(2, md(g1, b12, p), p), p),
                    md(2, md(b1, b0, p), p),
                    p,
                ),
                f1,
                p,
            );
            let c2 = sb(
                ad(
                    ad(
                        sb(md(a2, b22, p), md(2, md(g2, b12, p), p), p),
                        md(2, md(b2, b0, p), p),
                        p,
                    ),
                    md(b1, b1, p),
                    p,
                ),
                f2,
                p,
            );
            c0 == 0 && c1 == 0 && c2 == 0
        };
        let (mut total, mut zero) = (0u64, 0u64);
        // b2 = 0: enumerate b0, b1.
        for b0 in 0..p {
            for b1 in 0..p {
                if on(b0, b1, 0) {
                    total += 1;
                    zero += 1;
                }
            }
        }
        let inv2 = inv_mod(2, p);
        for b2 in 1..p {
            let i2b2 = md(inv2, inv_mod(b2, p), p);
            let b22 = md(b2, b2, p);
            for b1 in 0..p {
                // c~2 solved for b0.
                let rhs = sb(
                    ad(
                        sb(f2, md(a2, b22, p), p),
                        md(2, md(g2, md(b2, b1, p), p), p),
                        p,
                    ),
                    md(b1, b1, p),
                    p,
                );
                let b0 = md(rhs, i2b2, p);
                if on(b0, b1, b2) {
                    total += 1;
                }
            }
        }
        (total, zero)
    }

    /// Rational points of `X` over a finite `t` from the fibre of `G(t, .)`,
    /// the rule of `smith_prop24::x_point_count` (`None` where it does not
    /// apply: a triple root, or a Weierstrass point at a ramification point).
    pub fn fibre_rule_count(&self, t: u64) -> Option<u64> {
        let p = self.p;
        let k = F2::new(p);
        let [_, _, _, g0, g1, g2, ..] = self.at(t);
        let gt = [g0, g1, g2, 1u64];
        let roots: Vec<u64> = (0..p).filter(|&x| peval(&gt, x, p) == 0).collect();
        let fv = |x: Fp2| k.eval(&self.f, x);
        let is_w = |x: Fp2| k.is_zero(fv(x));
        // Distinct roots: all rational, or one rational and a conjugate pair.
        let disc = peval(&self.delta[3], t, p);
        if disc != 0 {
            let (xs, rational): (Vec<Fp2>, usize) = match roots.len() {
                3 => (roots.iter().map(|&r| k.c(r)).collect(), 3),
                1 => {
                    let x = roots[0];
                    let q1 = ad(g2, x, p);
                    let q0 = ad(g1, md(x, q1, p), p);
                    let (a, b) = k.quad_roots(q1, q0)?;
                    (vec![k.c(x), a, b], 1)
                }
                0 => (Vec::new(), 0),
                _ => return None,
            };
            let nw = xs.iter().filter(|&&x| is_w(x)).count();
            match nw {
                0 => Some(match rational {
                    3 => {
                        let c: Vec<i8> = xs.iter().map(|x| chi(fv(*x).a, p)).collect();
                        if c.iter().all(|&v| v == 1) || c.iter().all(|&v| v == -1) {
                            4
                        } else {
                            0
                        }
                    }
                    1 => {
                        // F(x2) F(x3) = N(F(x2)) lies in F_p.
                        let n = k.mul(fv(xs[1]), fv(xs[2]));
                        if chi(n.a, p) == 1 {
                            2
                        } else {
                            0
                        }
                    }
                    _ => {
                        // Irreducible cubic: one Galois-stable partition
                        // (unless the fibre meets the Weierstrass points).
                        if peval(&self.delta[1], t, p) == 0 {
                            return None;
                        }
                        1
                    }
                }),
                2 => {
                    // Weierstrass pair w = (X - r1)(X - r2): the sign of
                    // N_w(F') N_w(g'_num), as in x_point_count.
                    let ws: Vec<Fp2> = xs.iter().copied().filter(|&x| is_w(x)).collect();
                    let fd: Poly = (1..self.f.len())
                        .map(|i| md(i as u64, self.f[i], p))
                        .collect();
                    // g'_num = N' D - N D'.
                    let np = [self.n0, self.n1, 0, 1];
                    let dp = [self.d0, self.d1, 1];
                    let npd = [self.n1, 0, 3];
                    let dpd = [self.d1, 2];
                    let gnum = padd(
                        &pmul(&npd, &dp, p),
                        &pscale(&pmul(&np, &dpd, p), p - 1, p),
                        p,
                    );
                    let c = k.mul(
                        k.mul(k.eval(&fd, ws[0]), k.eval(&fd, ws[1])),
                        k.mul(k.eval(&gnum, ws[0]), k.eval(&gnum, ws[1])),
                    );
                    match chi(c.a, p) {
                        1 => Some(2),
                        -1 => Some(0),
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            // A double root xb and a simple root x3, both rational.
            let dg = [g1, md(2, g2, p), 3];
            let xb: Vec<u64> = roots
                .iter()
                .copied()
                .filter(|&r| peval(&dg, r, p) == 0)
                .collect();
            if xb.len() != 1 || roots.len() != 2 {
                return None;
            }
            let x3 = *roots.iter().find(|&&r| r != xb[0])?;
            let (cb, c3) = (
                chi(peval(&self.f, xb[0], p), p),
                chi(peval(&self.f, x3, p), p),
            );
            if cb == 0 || c3 == 0 {
                return None;
            }
            Some(1 + if cb * c3 == 1 { 2 } else { 0 })
        }
    }

    /// T4: counts over `U` (two ways) and outside, against `#H(F_p)`.
    pub fn count_report(&self) -> CountReport {
        let p = self.p;
        let affine: u64 = (0..p)
            .map(|x| (1 + chi(peval(&self.f, x, p), p) as i64) as u64)
            .sum();
        let inf = if deg(&self.f) == 8 {
            (1 + chi(self.f[8], p) as i64) as u64
        } else {
            1
        };
        let mut r = CountReport {
            p,
            h_points: affine + inf,
            h_over_u: self.points_over_u().len() as u64,
            ..Default::default()
        };
        let mut outside = Some(0u64);
        for t in 0..p {
            if self.in_u(t) {
                r.t_in_u += 1;
                let (m, z) = self.model_count_at(t);
                r.x_over_u_model += m / 2;
                r.model_b2_zero += z;
                r.x_over_u_fibres += self.fibre_rule_count(t).unwrap_or(u64::MAX / 4);
            } else {
                r.t_outside_u.push(t);
                outside = match (outside, self.fibre_rule_count(t)) {
                    (Some(a), Some(b)) => Some(a + b),
                    _ => None,
                };
            }
        }
        r.x_outside_u_finite = outside;
        r.implied_x_infinity =
            outside.map(|o| r.h_points as i64 - r.x_over_u_model as i64 - o as i64);
        r
    }
}
