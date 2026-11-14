//! Genus-3 non-hyperelliptic covers: Ciani plane quartics and their three
//! elliptic quotients, generated, verified and stored at import.
//!
//! # The curves
//!
//! C: a x^4 + b y^4 + c z^4 + d y^2 z^2 + e x^2 z^2 + f x^2 y^2 = 0 over F_p,
//! p >= 5. The three involutions x -> -x, y -> -y, z -> -z act on C, and
//! Jac(C) is isogenous over F_p to E_x x E_y x E_z, the Jacobians of the three
//! quotients (Ciani; Howe–Leprévost–Poonen, Forum Math. 12 (2000), §5). ecdb
//! does not rely on that theorem: it verifies L(Jac C) = L(E_x) L(E_y) L(E_z)
//! from exhaustive point counts for every stored quartic.
//!
//! **Smoothness (proved criterion).** In characteristic != 2 a singular point
//! satisfies x F_x = 2x^2 (2a x^2 + f y^2 + e z^2) = 0 and its two analogues
//! (Euler's identity then gives F = 0). With X, Y, Z = x^2, y^2, z^2 the cases
//! are: two coordinates zero (singular iff a, b or c is 0); one zero, say x
//! (singular iff R = d^2 - 4bc = 0, and likewise Q = e^2 - 4ac, P = f^2 - 4ab);
//! none zero (singular iff det M = 0, M = [[2a, f, e], [f, 2b, d], [e, d, 2c]],
//! once P Q R != 0). So C is smooth iff a b c P Q R det M != 0 with
//! det M = 8abc + 2def - 2(a d^2 + b e^2 + c f^2). Every singular point is
//! defined over F_{p^2} (square roots of F_p elements), so the criterion is
//! tested against an exhaustive search over F_{p^2} for small p.
//!
//! **The quotients.** For x -> -x put w = 2a x^2 + f y^2 + e z^2; on C,
//! w^2 = (f y^2 + e z^2)^2 - 4a(b y^4 + d y^2 z^2 + c z^4), so
//! C / <x -> -x> is D_x: w^2 = alpha s^4 + beta s^2 t^2 + gamma t^4 in
//! P(1, 1, 2) with (s, t) = (y, z), alpha = f^2 - 4ab, beta = 2ef - 4ad,
//! gamma = e^2 - 4ac, and C -> D_x, (x : y : z) -> (y : z : w), has degree 2
//! (a != 0, so it is a morphism). Likewise D_y: (s, t) = (x, z),
//! w = 2b y^2 + f x^2 + d z^2, (alpha, beta, gamma) = (f^2 - 4ab, 2df - 4be,
//! d^2 - 4bc), and D_z: (s, t) = (x, y), w = 2c z^2 + e x^2 + d y^2,
//! (e^2 - 4ac, 2de - 4cf, d^2 - 4bc). beta^2 - 4 alpha gamma = -8a det M (and
//! the analogues), so D_i is smooth of genus 1 when C is smooth.
//!
//! **Weierstrass models.** D_i has an F_p-point; the chart and point are
//! chosen deterministically: u = s/t if gamma is a non-zero square
//! (v = w/t^2, point (0, sqrt gamma)); else u = t/s if alpha is (v = w/s^2);
//! else u = s/t at the least u0 in 1..p-1 whose value is a non-zero square
//! (the smaller square root). With u' = u - u0 the model is
//! v^2 = a4 u'^4 + a3 u'^3 + a2 u'^2 + a1 u' + q^2, q != 0, and
//! X = (2q(v + q) + a1 u') / u'^2,
//! Y = (4q^2(v + q) + 2q(a1 u' + a2 u'^2) - a1^2 u'^2 / (2q)) / u'^3
//! is a birational map onto Y^2 + A1 XY + A3 Y = X^3 + A2 X^2 + A4 X + A6 with
//! A1 = a1/q, A2 = a2 - a1^2/(4q^2), A3 = 2q a3, A4 = -4q^2 a4, A6 = A2 A4
//! (Washington, Elliptic Curves, 2nd ed., Thm 2.17), followed by the usual
//! x = 36X + 3b2, y = 108(2Y + A1 X + A3) onto y^2 = x^3 - 27c4 x - 54c6. The
//! stored morphism C -> E_i is that composite, of degree 2.
//!
//! # Verification (every stored quartic)
//!
//! * #C(F_{p^k}), k = 1, 2, 3, counted exhaustively: for each (y : z) in
//!   P^1(F_{p^k}) the quadratic a X^2 + (e z^2 + f y^2) X + b y^4 + d y^2 z^2
//!   plus c z^4 is solved in X = x^2 and each root X gives 1 + chi(X) values
//!   of x; (1 : 0 : 0) is not on C since a != 0. L(T) follows from Newton's
//!   identities (the counts fix the genus-3 L-polynomial).
//! * #E_i(F_p) counted exhaustively on the short model (trace t_i), and
//!   #D_i(F_p) counted on the quartic model must be equal (birational smooth
//!   curves).
//! * Every F_p-point of C off the chart's exceptional points maps onto E_i.
//! * L(T) = prod_i (1 - t_i T + p T^2), the independent identity.
//!
//! A sample failing any check is counted (`ciani_crosscheck_failures`) and
//! reported as a problem, and nothing of it is stored.
//!
//! # The generator (`ciani_quartic_seeded_v1`)
//!
//! For each prime p in [`PRIMES`] (every prime 5 <= p < 64), SplitMix64 is
//! seeded with [`SEED`] xor (p * 0x9E3779B97F4A7C15) and draws a, b, c, d, e, f
//! in turn as next() mod p. A draw failing the smoothness criterion is
//! rejected (`ciani_rejected_singular`), a repeated one is skipped, until
//! [`PER_PRIME`] quartics are accepted. The set is fixed by the constants and
//! the code; [`sample`] reproduces it.
//!
//! # Stored
//!
//! The quartic as an `ec.curve` (genus 3, model `plane_curve`, family
//! `ciani-quartic`, smoothness proved) with its `curve_equation_model` (P^2)
//! and `generation_provenance`; its Jacobian's `isogeny_class_fq` (genus 3,
//! label `3.p.a1_a2_a3`), the link from `abelian_variety` and a `curve_zeta`
//! row [#C(F_p), #C(F_p^2), #C(F_p^3)]; each E_i as a short-Weierstrass ICV1
//! curve (family `ciani-quotient`) with its class, invariants and zeta row;
//! the three maps as `curve_morphism` rows of kind `quotient` (constructions
//! `ciani_quotient_x_v1`, `_y_`, `_z_`), and `av_component` rows
//! (`isogeny_factor`) from Jac(C) to each E_i. Insert or fill-NULL only:
//! before writing, the stored rows each quartic would meet (its curve, class
//! link, zeta, equation model, generation row, maps and components, and its
//! quotient curves' slug, class link, zeta and invariants) are compared with
//! the computed values; a quartic meeting any disagreement is refused whole
//! (`ciani_refused_conflict`, one problem per conflict), and after the
//! inserts every kept quartic's three maps must be stored exactly
//! (`ciani_morphisms_missing`).

use super::cover_lpoly::l_from_power_sums;
use super::registry::{
    class_row as genus1_class_row, invariant_row, prime_field, ALIAS_SQL, CLASS_SQL as G1_CLASS_SQL, CURVE_SQL,
    FIELD_SQL, INVARIANT_SQL,
};
use super::siblings::prime_icv1;
use super::{bulk, bulk1, Ctx, Report};
use crate::math::weil_label;
use anyhow::Result;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Zero;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// The generator's seed ("CIANIQ" and version 1).
pub const SEED: u64 = 0x4349_414E_4951_0001;
/// Every prime 5 <= p < 64.
pub const PRIMES: [u64; 16] = [5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61];
/// Quartics accepted per prime.
pub const PER_PRIME: usize = 8;
/// Draws per prime before the generator gives up (never reached for these p).
const MAX_DRAWS: usize = 100_000;
pub const GENERATOR: &str = "ciani_quartic_seeded_v1";
pub const FAMILY: &str = "ciani-quartic";
pub const QUOTIENT_FAMILY: &str = "ciani-quotient";
const FORM: &str = "a*x^4+b*y^4+c*z^4+d*y^2*z^2+e*x^2*z^2+f*x^2*y^2=0";
const NAMES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

// ------------------------------------------------------------- F_p helpers

fn md(a: i128, p: u64) -> u64 {
    a.rem_euclid(p as i128) as u64
}

fn powm(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

/// a^-1 mod p (p prime, a != 0).
fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}

/// Legendre symbol: 0, 1 or -1.
fn chi(a: u64, p: u64) -> i64 {
    let a = a % p;
    if a == 0 {
        0
    } else if powm(a, (p - 1) / 2, p) == 1 {
        1
    } else {
        -1
    }
}

/// The smaller square root of a non-zero square (p small), else None.
fn sqrt_small(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    (1..=p / 2).find(|r| r * r % p == a)
}

// ------------------------------------------------------- F_{p^k}, k <= 3

/// F_{p^k} for k = 1, 2, 3 and p < 2^16, on digit vectors c0 + c1 X + c2 X^2
/// modulo the least monic irreducible X^k + m_{k-1} X^{k-1} + ... + m_0 (by
/// the index m_0 + m_1 p + ...; for k <= 3 irreducible means no root in F_p).
pub struct SmallField {
    pub p: u64,
    pub k: usize,
    pub q: u64,
    m: [u64; 3],
    /// For each element index, a square root's index, or u32::MAX.
    sqrt: Vec<u32>,
}

pub type E3 = [u64; 3];

impl SmallField {
    pub fn new(p: u64, k: usize) -> SmallField {
        assert!((1..=3).contains(&k) && p < 1 << 16);
        let q = p.pow(k as u32);
        let mut m = [0u64; 3];
        if k > 1 {
            'search: for idx in 0..p.pow(k as u32) {
                let mut cand = [0u64; 3];
                let mut n = idx;
                for c in cand.iter_mut().take(k) {
                    *c = n % p;
                    n /= p;
                }
                for r in 0..p {
                    // X^k + sum m_i X^i at r
                    let mut v = powm(r, k as u64, p);
                    for (i, c) in cand.iter().enumerate().take(k) {
                        v = (v + c * powm(r, i as u64, p)) % p;
                    }
                    if v == 0 {
                        continue 'search;
                    }
                }
                m = cand;
                break;
            }
        }
        let mut f = SmallField { p, k, q, m, sqrt: Vec::new() };
        let mut sqrt = vec![u32::MAX; q as usize];
        for i in 0..q {
            let r = f.elt(i);
            let s = f.idx(&f.mul(&r, &r));
            if sqrt[s as usize] == u32::MAX {
                sqrt[s as usize] = i as u32;
            }
        }
        f.sqrt = sqrt;
        f
    }

    pub fn elt(&self, mut n: u64) -> E3 {
        let mut e = [0u64; 3];
        for c in e.iter_mut().take(self.k) {
            *c = n % self.p;
            n /= self.p;
        }
        e
    }

    pub fn idx(&self, e: &E3) -> u64 {
        e.iter().take(self.k).rev().fold(0, |acc, c| acc * self.p + c)
    }

    pub fn konst(&self, c: u64) -> E3 {
        [c % self.p, 0, 0]
    }

    pub fn add(&self, a: &E3, b: &E3) -> E3 {
        let p = self.p;
        [(a[0] + b[0]) % p, (a[1] + b[1]) % p, (a[2] + b[2]) % p]
    }

    pub fn sub(&self, a: &E3, b: &E3) -> E3 {
        let p = self.p;
        [(a[0] + p - b[0]) % p, (a[1] + p - b[1]) % p, (a[2] + p - b[2]) % p]
    }

    pub fn scale(&self, c: u64, a: &E3) -> E3 {
        let p = self.p;
        [c * a[0] % p, c * a[1] % p, c * a[2] % p]
    }

    pub fn mul(&self, a: &E3, b: &E3) -> E3 {
        let (p, k) = (self.p, self.k);
        let mut prod = [0u64; 5];
        for i in 0..k {
            for j in 0..k {
                prod[i + j] = (prod[i + j] + a[i] * b[j]) % p;
            }
        }
        for i in (k..2 * k - 1).rev() {
            let c = prod[i];
            if c != 0 {
                for j in 0..k {
                    prod[i - k + j] = (prod[i - k + j] + (p - c) * self.m[j]) % p;
                }
            }
            prod[i] = 0;
        }
        [prod[0], prod[1], prod[2]]
    }

    pub fn is_zero(&self, a: &E3) -> bool {
        a[0] == 0 && a[1] == 0 && a[2] == 0
    }

    /// A square root, or None for a non-square.
    pub fn sqrt(&self, a: &E3) -> Option<E3> {
        let s = self.sqrt[self.idx(a) as usize];
        (s != u32::MAX).then(|| self.elt(s as u64))
    }

    /// #{x in F : x^2 = a}.
    fn roots_of_square(&self, a: &E3) -> u64 {
        if self.is_zero(a) {
            1
        } else if self.sqrt(a).is_some() {
            2
        } else {
            0
        }
    }
}

// ------------------------------------------------------------ the quartic

/// Coefficients (a, b, c, d, e, f) as residues mod p.
pub type Ciani = [u64; 6];

/// The proved smoothness criterion (module docs): a b c P Q R det M != 0.
pub fn is_smooth(p: u64, c: &Ciani) -> bool {
    let [a, b, cc, d, e, f] = c.map(|x| x as i128);
    let pp = f * f - 4 * a * b;
    let qq = e * e - 4 * a * cc;
    let rr = d * d - 4 * b * cc;
    let det = 8 * a * b * cc + 2 * d * e * f - 2 * (a * d * d + b * e * e + cc * f * f);
    [a, b, cc, pp, qq, rr, det].iter().all(|v| md(*v, p) != 0)
}

/// #C(F_q), q = p^k, by solving the quadratic in X = x^2 over each (y : z).
pub fn count_points(fld: &SmallField, c: &Ciani) -> u64 {
    let p = fld.p;
    let [a, b, cc, d, e, f] = *c;
    let inv2a = invm(2 * a % p, p);
    let four_a = 4 * a % p;
    let mut total = 0u64;
    let mut fibre = |y: &E3, z: &E3| {
        let y2 = fld.mul(y, y);
        let z2 = fld.mul(z, z);
        let bq = fld.add(&fld.scale(e, &z2), &fld.scale(f, &y2));
        let y4 = fld.mul(&y2, &y2);
        let z4 = fld.mul(&z2, &z2);
        let y2z2 = fld.mul(&y2, &z2);
        let cq = fld.add(&fld.add(&fld.scale(b, &y4), &fld.scale(d, &y2z2)), &fld.scale(cc, &z4));
        let disc = fld.sub(&fld.mul(&bq, &bq), &fld.scale(four_a, &cq));
        let neg_b = fld.sub(&[0; 3], &bq);
        if fld.is_zero(&disc) {
            total += fld.roots_of_square(&fld.scale(inv2a, &neg_b));
        } else if let Some(r) = fld.sqrt(&disc) {
            total += fld.roots_of_square(&fld.scale(inv2a, &fld.add(&neg_b, &r)));
            total += fld.roots_of_square(&fld.scale(inv2a, &fld.sub(&neg_b, &r)));
        }
    };
    let one = fld.konst(1);
    for i in 0..fld.q {
        fibre(&fld.elt(i), &one);
    }
    fibre(&one, &[0; 3]);
    total
}

// ----------------------------------------------------------- the quotients

/// One elliptic quotient C / <sigma> and its Weierstrass model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quotient {
    /// The coordinate sigma negates: 'x', 'y' or 'z'.
    pub involution: char,
    /// (s, t): the coordinates kept.
    pub st: (char, char),
    /// w = w[0] x^2 + w[1] y^2 + w[2] z^2.
    pub w: [u64; 3],
    /// D: w^2 = alpha s^4 + beta s^2 t^2 + gamma t^4, as (alpha, beta, gamma).
    pub quartic: [u64; 3],
    /// u = t/s (true) or s/t (false).
    pub swapped: bool,
    /// u' = u - u0; v(u' = 0) = q.
    pub u0: u64,
    pub q: u64,
    /// v^2 = a4 u'^4 + a3 u'^3 + a2 u'^2 + a1 u' + q^2, as [q^2, a1, a2, a3, a4].
    pub translated: [u64; 5],
    /// Long Weierstrass [A1, A2, A3, A4, A6].
    pub long: [u64; 5],
    /// Short Weierstrass y^2 = x^3 + A x + B: [A, B] and b2 of the long model.
    pub short: [u64; 2],
    pub b2: u64,
}

impl Quotient {
    pub fn construction(&self) -> String {
        format!("ciani_quotient_{}_v1", self.involution)
    }

    /// The (s, t, w) coordinates of a point (x : y : z) of C.
    fn stw(&self, p: u64, pt: [u64; 3]) -> (u64, u64, u64) {
        let [x, y, z] = pt;
        let w = (self.w[0] * x % p * x + self.w[1] * y % p * y + self.w[2] * z % p * z) % p;
        let (s, t) = match self.involution {
            'x' => (y, z),
            'y' => (x, z),
            _ => (x, y),
        };
        (s, t, w)
    }

    /// The image on y^2 = x^3 + A x + B of a point of C over F_p, or None at
    /// the chart's exceptional points (s or t = 0 off the chart, u' = 0).
    pub fn map_point(&self, p: u64, pt: [u64; 3]) -> Option<(u64, u64)> {
        let (s, t, w) = self.stw(p, pt);
        let (num, den) = if self.swapped { (t, s) } else { (s, t) };
        if den == 0 {
            return None;
        }
        let di = invm(den, p);
        let u = num * di % p;
        let v = w * di % p * di % p;
        let up = md(u as i128 - self.u0 as i128, p);
        if up == 0 {
            return None;
        }
        let [_, a1, a2, _, _] = self.translated;
        let q = self.q;
        let ui = invm(up, p);
        let vq = (v + q) % p;
        let x_big = (2 * q % p * vq % p + a1 * up % p) % p * ui % p * ui % p;
        let inv2q = invm(2 * q % p, p);
        let num_y = md(
            (4 * q % p * q % p * vq % p) as i128 + (2 * q % p * ((a1 * up + a2 * up % p * up) % p) % p) as i128
                - (a1 * a1 % p * up % p * up % p * inv2q % p) as i128,
            p,
        );
        let y_big = num_y * ui % p * ui % p * ui % p;
        let [la1, _, la3, _, _] = self.long;
        let xs = (36 * x_big + 3 * self.b2) % p;
        let ys = 108 * ((2 * y_big + la1 * x_big + la3) % p) % p;
        Some((xs, ys))
    }

    /// The map as stored in `curve_morphism.map`.
    pub fn map_json(&self) -> Value {
        let st = |c: char| c.to_string();
        let s = |v: u64| v.to_string();
        json!({
            "involution": format!("{} -> -{}", self.involution, self.involution),
            "quotient": {
                "model": "w^2 = alpha*s^4 + beta*s^2*t^2 + gamma*t^4 in P(1,1,2)",
                "s": st(self.st.0), "t": st(self.st.1),
                "w": { "x^2": s(self.w[0]), "y^2": s(self.w[1]), "z^2": s(self.w[2]) },
                "alpha": s(self.quartic[0]), "beta": s(self.quartic[1]), "gamma": s(self.quartic[2]),
                "degree": 2,
            },
            "chart": {
                "u": if self.swapped { "t/s" } else { "s/t" },
                "v": if self.swapped { "w/s^2" } else { "w/t^2" },
                "u0": s(self.u0), "q": s(self.q),
                "translated": "v^2 = a4*u'^4 + a3*u'^3 + a2*u'^2 + a1*u' + q^2, u' = u - u0",
                "a1": s(self.translated[1]), "a2": s(self.translated[2]), "a3": s(self.translated[3]),
                "a4": s(self.translated[4]),
            },
            "to_long_weierstrass": {
                "reference": "Washington, Elliptic Curves (2nd ed.), Thm 2.17",
                "X": "(2*q*(v + q) + a1*u') / u'^2",
                "Y": "(4*q^2*(v + q) + 2*q*(a1*u' + a2*u'^2) - a1^2*u'^2/(2*q)) / u'^3",
                "A1": s(self.long[0]), "A2": s(self.long[1]), "A3": s(self.long[2]), "A4": s(self.long[3]),
                "A6": s(self.long[4]),
            },
            "to_short_weierstrass": {
                "x": "36*X + 3*b2", "y": "108*(2*Y + A1*X + A3)", "b2": s(self.b2),
                "A": s(self.short[0]), "B": s(self.short[1]),
            },
            "composite_degree": 2,
        })
    }
}

/// The three quotients of a smooth Ciani quartic (module docs), or an error
/// when a step is undefined. That happens for some smooth quartics over tiny
/// fields: when every F_p-point of a D_i is a branch point (w = 0) the chart
/// search finds no point with q != 0 and the quartic is refused (reported,
/// never stored); e.g. 3x^4 + y^4 + z^4 + 3x^2y^2 over F_5 (test
/// `quartics_without_a_usable_chart_point_are_refused`). No generator sample
/// meets this.
pub fn quotients(p: u64, c: &Ciani) -> Result<[Quotient; 3], String> {
    let [a, b, cc, d, e, f] = c.map(|x| x as i128);
    let mk = |inv: char, st: (char, char), w: [i128; 3], abg: [i128; 3]| -> Result<Quotient, String> {
        let w = w.map(|v| md(v, p));
        let [al, be, ga] = abg.map(|v| md(v, p));
        // chart and point
        let (swapped, u0, q) = if let Some(r) = sqrt_small(ga, p) {
            (false, 0, r)
        } else if let Some(r) = sqrt_small(al, p) {
            (true, 0, r)
        } else {
            let found = (1..p).find_map(|u| {
                let u2 = u * u % p;
                let val = (al * u2 % p * u2 + be * u2 + ga) % p;
                sqrt_small(val, p).map(|r| (u, r))
            });
            let (u, r) = found.ok_or_else(|| format!("no F_{p}-point off the branch locus on D_{inv}"))?;
            (false, u, r)
        };
        let (c4q, c2q, c0q) = if swapped { (ga, be, al) } else { (al, be, ga) };
        let u0i = u0 as i128;
        let (a4, b2q, c0) = (c4q as i128, c2q as i128, c0q as i128);
        let t4 = md(a4, p);
        let t3 = md(4 * a4 * u0i, p);
        let t2 = md(6 * a4 * u0i * u0i + b2q, p);
        let t1 = md(4 * a4 * u0i * u0i % p as i128 * u0i + 2 * b2q * u0i, p);
        let t0 = md(a4 * md(u0i * u0i, p) as i128 % p as i128 * md(u0i * u0i, p) as i128 + b2q * u0i * u0i + c0, p);
        if t0 != q * q % p || q == 0 || t4 == 0 {
            return Err(format!("D_{inv}: chart point inconsistent"));
        }
        let (qi, t1i, t2i, t3i, t4i) = (q as i128, t1 as i128, t2 as i128, t3 as i128, t4 as i128);
        let inv_q = invm(q, p) as i128;
        let inv_4q2 = invm(4 * q % p * q % p, p) as i128;
        let la1 = md(t1i * inv_q, p);
        let la2 = md(t2i - md(t1i * t1i, p) as i128 * inv_4q2, p);
        let la3 = md(2 * qi * t3i, p);
        let la4 = md(-4 * qi * qi % p as i128 * t4i, p);
        let la6 = md(la2 as i128 * la4 as i128, p);
        let (l1, l2, l3, l4, l6) = (la1 as i128, la2 as i128, la3 as i128, la4 as i128, la6 as i128);
        let b2 = md(l1 * l1 + 4 * l2, p) as i128;
        let b4 = md(2 * l4 + l1 * l3, p) as i128;
        let b6 = md(l3 * l3 + 4 * l6, p) as i128;
        let c4v = md(b2 * b2 - 24 * b4, p) as i128;
        let c6v = md(-(b2 * b2 % p as i128) * b2 + 36 * b2 * b4 % p as i128 - 216 * b6, p) as i128;
        let sa = md(-27 * c4v, p);
        let sb = md(-54 * c6v, p);
        let (sai, sbi) = (sa as i128, sb as i128);
        if md(4 * sai * sai % p as i128 * sai + 27 * sbi * sbi, p) == 0 {
            return Err(format!("E_{inv}: singular Weierstrass model"));
        }
        Ok(Quotient {
            involution: inv,
            st,
            w,
            quartic: [al, be, ga],
            swapped,
            u0,
            q,
            translated: [t0, t1, t2, t3, t4],
            long: [la1, la2, la3, la4, la6],
            short: [sa, sb],
            b2: b2 as u64,
        })
    };
    Ok([
        mk('x', ('y', 'z'), [2 * a, f, e], [f * f - 4 * a * b, 2 * e * f - 4 * a * d, e * e - 4 * a * cc])?,
        mk('y', ('x', 'z'), [f, 2 * b, d], [f * f - 4 * a * b, 2 * d * f - 4 * b * e, d * d - 4 * b * cc])?,
        mk('z', ('x', 'y'), [e, d, 2 * cc], [e * e - 4 * a * cc, 2 * d * e - 4 * cc * f, d * d - 4 * b * cc])?,
    ])
}

/// #E(F_p) of y^2 = x^3 + A x + B, exhaustively.
pub fn count_short(p: u64, ab: [u64; 2]) -> u64 {
    let s: i64 = (0..p).map(|x| chi((x * x % p * x + ab[0] * x + ab[1]) % p, p)).sum();
    (p as i64 + 1 + s) as u64
}

/// #D(F_p) of w^2 = alpha s^4 + beta s^2 t^2 + gamma t^4 in P(1, 1, 2).
pub fn count_even_quartic(p: u64, abg: [u64; 3]) -> u64 {
    let [al, be, ga] = abg;
    let s: i64 = (0..p)
        .map(|u| {
            let u2 = u * u % p;
            1 + chi((al * u2 % p * u2 + be * u2 + ga) % p, p)
        })
        .sum();
    (s + 1 + chi(al, p)) as u64
}

/// The F_p-points of C, projectively normalised.
pub fn rational_points(p: u64, c: &Ciani) -> Vec<[u64; 3]> {
    let [a, b, cc, d, e, f] = *c;
    let ev = |x: u64, y: u64, z: u64| {
        let (x2, y2, z2) = (x * x % p, y * y % p, z * z % p);
        (a * x2 % p * x2 + b * y2 % p * y2 + cc * z2 % p * z2 + d * y2 % p * z2 + e * x2 % p * z2 + f * x2 % p * y2) % p
    };
    let mut pts = Vec::new();
    for x in 0..p {
        for y in 0..p {
            if ev(x, y, 1) == 0 {
                pts.push([x, y, 1]);
            }
        }
        if ev(x, 1, 0) == 0 {
            pts.push([x, 1, 0]);
        }
    }
    if ev(1, 0, 0) == 0 {
        pts.push([1, 0, 0]);
    }
    pts
}

/// A quartic that passed every check.
#[derive(Debug)]
pub struct Verified {
    pub p: u64,
    pub coeffs: Ciani,
    /// #C(F_{p^k}), k = 1, 2, 3.
    pub counts: [u64; 3],
    /// L(T) of Jac(C), ascending, degree 6.
    pub l: Vec<BigInt>,
    pub quotients: [Quotient; 3],
    /// #E_i(F_p).
    pub orders: [u64; 3],
    /// Points of C checked to map onto each E_i.
    pub mapped_points: [usize; 3],
}

/// 1 - tT + pT^2, multiplied out over the three traces.
pub fn product_l(p: u64, traces: &[i64; 3]) -> Vec<BigInt> {
    let mut acc = vec![BigInt::from(1)];
    for t in traces {
        let f = [BigInt::from(1), BigInt::from(-t), BigInt::from(p)];
        let mut out = vec![BigInt::zero(); acc.len() + 2];
        for (i, x) in acc.iter().enumerate() {
            for (j, y) in f.iter().enumerate() {
                out[i + j] += x * y;
            }
        }
        acc = out;
    }
    acc
}

/// Count, map and cross-check one quartic against its claimed quotients.
/// `fields` are F_p, F_p^2, F_p^3 for this p.
pub fn verify(fields: &[SmallField; 3], c: &Ciani, quotients: &[Quotient; 3]) -> Result<Verified, String> {
    let p = fields[0].p;
    if !is_smooth(p, c) {
        return Err("the quartic is singular".into());
    }
    let counts = [count_points(&fields[0], c), count_points(&fields[1], c), count_points(&fields[2], c)];
    let pb = BigInt::from(p);
    let s: Vec<BigInt> =
        counts.iter().enumerate().map(|(k, n)| num_traits::pow(pb.clone(), k + 1) + 1 - BigInt::from(*n)).collect();
    let l = l_from_power_sums(&pb, &s).ok_or("Newton's identities do not give integer coefficients")?;
    let order: BigInt = l.iter().sum();
    if order <= BigInt::zero() {
        return Err(format!("#Jac = L(1) = {order} is not positive"));
    }
    let pts = rational_points(p, c);
    if pts.len() as u64 != counts[0] {
        return Err(format!("#C(F_p): fibre count {} but {} points found directly", counts[0], pts.len()));
    }
    let mut traces = [0i64; 3];
    let mut orders = [0u64; 3];
    let mut mapped = [0usize; 3];
    for (i, qt) in quotients.iter().enumerate() {
        let [sa, sb] = qt.short;
        let n = count_short(p, qt.short);
        let nd = count_even_quartic(p, qt.quartic);
        if n != nd {
            return Err(format!("E_{}: #E(F_p) = {n} but the quotient D has {nd} points", qt.involution));
        }
        for pt in &pts {
            if let Some((x, y)) = qt.map_point(p, *pt) {
                if (y * y) % p != (x * x % p * x + sa * x + sb) % p {
                    return Err(format!("E_{}: the image of {pt:?} is not on the curve", qt.involution));
                }
                mapped[i] += 1;
            }
        }
        orders[i] = n;
        traces[i] = p as i64 + 1 - n as i64;
    }
    let prod = product_l(p, &traces);
    if prod != l {
        return Err(format!("L(Jac C) = {l:?} but L(E_x) L(E_y) L(E_z) = {prod:?}"));
    }
    Ok(Verified { p, coeffs: *c, counts, l, quotients: quotients.clone(), orders, mapped_points: mapped })
}

// -------------------------------------------------------------- generator

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The generator's quartics over F_p (module docs) and the number of singular
/// draws rejected on the way.
pub fn sample(p: u64, n: usize) -> (Vec<Ciani>, usize) {
    let mut state = SEED ^ p.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    let mut rejected = 0;
    for _ in 0..MAX_DRAWS {
        if out.len() == n {
            break;
        }
        let mut c = [0u64; 6];
        for x in c.iter_mut() {
            *x = splitmix(&mut state) % p;
        }
        if !is_smooth(p, &c) {
            rejected += 1;
            continue;
        }
        if seen.insert(c) {
            out.push(c);
        }
    }
    (out, rejected)
}

// ------------------------------------------------------------------ rows

fn coeff_json(c: &Ciani) -> Value {
    Value::Object(NAMES.iter().zip(c).map(|(n, v)| (n.to_string(), Value::String(v.to_string()))).collect())
}

fn equation(c: &Ciani) -> String {
    let mono = ["x^4", "y^4", "z^4", "y^2*z^2", "x^2*z^2", "x^2*y^2"];
    let terms: Vec<String> = c.iter().zip(mono).filter(|(v, _)| **v != 0).map(|(v, m)| format!("{v}*{m}")).collect();
    format!("{} = 0", terms.join(" + "))
}

fn generators(c: &Ciani) -> Value {
    let exps = [[4, 0, 0], [0, 4, 0], [0, 0, 4], [0, 2, 2], [2, 0, 2], [2, 2, 0]];
    let terms: Vec<Value> =
        c.iter().zip(exps).filter(|(v, _)| **v != 0).map(|(v, e)| json!([e, v.to_string()])).collect();
    json!([terms])
}

pub const METHOD_L: &str =
    "exhaustive count of #C over F_p, F_p^2 and F_p^3 (Ciani plane quartic; for each (y : z) the quadratic \
     in X = x^2 is solved and each root X gives 1 + chi(X) points), L(T) of Jac(C) from Newton's identities; equal to \
     L(E_x) L(E_y) L(E_z) of the three quotient curves counted exhaustively over F_p (ecdb/src/import/ciani.rs)";
pub const METHOD_E: &str = "exhaustive count of #E(F_p) on the short Weierstrass model; equal to the count of the \
     quotient quartic w^2 = alpha s^4 + beta s^2 t^2 + gamma t^4 (ecdb/src/import/ciani.rs)";
pub const METHOD_MAP: &str =
    "quotient of the Ciani quartic by the involution (degree 2) composed with the birational map \
     of Washington Thm 2.17 and the change to short Weierstrass form; every F_p-point of C off the chart's exceptional \
     points checked to map onto E, and L(Jac C) = L(E_x) L(E_y) L(E_z) (ecdb/src/import/ciani.rs)";

const QUARTIC_SQL: &str = "
INSERT INTO ec.curve (field_id, genus, model, form, coefficients, equation, family, is_smooth, smoothness_status,
       geometrically_irreducible, description, provenance_id)
SELECT f.id, 3, 'plane_curve', r.form, r.coefficients, r.equation, r.family, true, 'proved', true, r.description, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field text, form text, coefficients jsonb, equation text, family text,
     description text)
JOIN ec.field f ON f.icv1_field = r.field
ON CONFLICT DO NOTHING";

const JOIN_QUARTIC: &str = "JOIN ec.field f ON f.icv1_field = r.field
JOIN ec.curve c ON c.field_id = f.id AND c.model = 'plane_curve' AND c.coefficients = r.coefficients";

fn with_quartic(head: &str, cols: &str, tail: &str) -> String {
    format!(
        "{head}\nFROM jsonb_to_recordset($1::jsonb) AS r(field text, coefficients jsonb{cols})\n{JOIN_QUARTIC}\n{tail}"
    )
}

fn equation_sql() -> String {
    with_quartic(
        "INSERT INTO ec.curve_equation_model (curve_id, ambient, variables, generators, degree, is_complete_intersection,
       singular_points, status, provenance_id)
SELECT c.id, 'P^2', ARRAY['x', 'y', 'z'], r.generators, 4, true, '[]'::jsonb, 'proved', $2::bigint",
        ", generators jsonb",
        "ON CONFLICT (curve_id) DO NOTHING",
    )
}

const G3_CLASS_SQL: &str = "
INSERT INTO ec.isogeny_class_fq (q, p, r, g, l_polynomial, label, order_value, order_bits, point_counts, curve_counts,
       is_ordinary, has_jacobian, status, provenance_id)
SELECT r.p, r.p, 1, 3, r.l_polynomial, r.label, r.order_value, ec.bits(r.order_value), ARRAY[r.order_value],
       r.curve_counts, r.ordinary, true, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(p numeric, l_polynomial numeric[], label text, order_value numeric,
     curve_counts numeric[], ordinary boolean)
ON CONFLICT DO NOTHING";

fn link_sql() -> String {
    with_quartic(
        "UPDATE ec.abelian_variety av SET isogeny_class_id = ic.id",
        ", label text",
        "JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE av.curve_id = c.id AND av.isogeny_class_id IS NULL",
    )
}

fn zeta_sql() -> String {
    with_quartic(
        "INSERT INTO ec.curve_zeta (curve_id, isogeny_class_id, point_counts, order_check, order_method, provenance_id)
SELECT c.id, ic.id, r.point_counts, 'proved', r.method, $2::bigint",
        ", label text, point_counts numeric[], method text",
        "JOIN ec.isogeny_class_fq ic ON ic.label = r.label
JOIN ec.abelian_variety av ON av.curve_id = c.id AND av.isogeny_class_id = ic.id
ON CONFLICT (curve_id) DO NOTHING",
    )
}

fn generation_sql() -> String {
    with_quartic(
        "INSERT INTO ec.generation_provenance (curve_id, method, seed, procedure_ref, generator_call, is_reproducible,
       status, provenance_id)
SELECT c.id, 'seeded_generator', r.seed, r.procedure, r.call, true, 'proved', $2::bigint",
        ", seed text, procedure text, call text",
        "ON CONFLICT (curve_id) DO NOTHING",
    )
}

fn morphism_sql() -> String {
    with_quartic(
        "INSERT INTO ec.curve_morphism (source_curve_id, target_curve_id, kind, construction, degree, source_field_id,
       target_field_id, cover_genus, map, map_direction, field_relation, exists_over_base, minimal_degree,
       subgroup_transfer, status, method, provenance_id)
SELECT c.id, e.id, 'quotient', r.construction, 2, f.id, f.id, 3, r.map, 'C -> E', 'same_field', true, 2,
       'not_tested', 'proved', r.method, $2::bigint",
        ", target text, construction text, map jsonb, method text",
        "JOIN ec.curve e ON e.icv1_slug = r.target
ON CONFLICT DO NOTHING",
    )
}

fn component_sql() -> String {
    with_quartic(
        "INSERT INTO ec.av_component (av_id, component_av_id, multiplicity, relation, status, provenance_id)
SELECT ja.id, ea.id, r.multiplicity, 'isogeny_factor', 'proved', $2::bigint",
        ", target text, multiplicity int",
        "JOIN ec.abelian_variety ja ON ja.curve_id = c.id
JOIN ec.curve e ON e.icv1_slug = r.target
JOIN ec.abelian_variety ea ON ea.curve_id = e.id
ON CONFLICT DO NOTHING",
    )
}

/// Links an E_i's Jacobian to its class only where no class is stored (a
/// stored different class is refused by the pre-check, never overwritten).
const E_LINK_SQL: &str = "
UPDATE ec.abelian_variety av SET isogeny_class_id = ic.id
FROM jsonb_to_recordset($1::jsonb) AS r(key text, label text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE av.curve_id = c.id AND av.isogeny_class_id IS NULL";

const E_ZETA_SQL: &str = "
INSERT INTO ec.curve_zeta (curve_id, isogeny_class_id, point_counts, order_check, order_method, provenance_id)
SELECT c.id, ic.id, r.point_counts, 'proved', r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, label text, point_counts numeric[], method text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
JOIN ec.abelian_variety av ON av.curve_id = c.id AND av.isogeny_class_id = ic.id
ON CONFLICT (curve_id) DO NOTHING";

// ------------------------------------------------- conflicts with stored rows
//
// Every statement above inserts or fills NULL only. Before anything is
// written, the stored rows that the inserts would meet are compared with the
// computed values; a quartic whose rows (or whose quotient curves' rows)
// disagree with anything stored is refused whole and reported as a problem.
// After the inserts every kept quartic's morphisms must be present exactly.

/// `a` and `b` agree on their common prefix (point counts of different lengths).
const PREFIX_DIFFERS: &str =
    "(SELECT bool_or(x IS DISTINCT FROM y) FROM unnest(%A%[1:least(cardinality(%A%), cardinality(%B%))],
     %B%[1:least(cardinality(%A%), cardinality(%B%))]) AS u(x, y))";

fn prefix_differs(a: &str, b: &str) -> String {
    PREFIX_DIFFERS.replace("%A%", a).replace("%B%", b)
}

/// Stored rows of a quartic that contradict the computed ones: (qi, reasons).
fn quartic_check_sql() -> String {
    format!(
        "SELECT r.qi, concat_ws('; ',
       CASE WHEN c.genus <> 3 THEN 'stored genus ' || c.genus END,
       CASE WHEN c.is_smooth = false THEN 'stored as singular' END,
       CASE WHEN ic.id IS NOT NULL AND ic.label <> r.label THEN 'Jacobian linked to class ' || ic.label END,
       CASE WHEN z.curve_id IS NOT NULL AND {zc} THEN 'stored zeta counts ' || z.point_counts::text END,
       CASE WHEN zic.id IS NOT NULL AND zic.label <> r.label THEN 'zeta row names class ' || zic.label END,
       CASE WHEN em.curve_id IS NOT NULL AND (em.ambient <> 'P^2' OR em.generators <> r.generators)
            THEN 'stored equation model differs' END,
       CASE WHEN gp.curve_id IS NOT NULL AND (gp.method <> 'seeded_generator' OR gp.seed IS DISTINCT FROM r.seed
                 OR gp.generator_call IS DISTINCT FROM r.call) THEN 'stored generation provenance differs' END,
       CASE WHEN lc.id IS NOT NULL AND (lc.g <> 3 OR lc.l_polynomial::numeric[] <> r.l_polynomial)
            THEN 'class ' || lc.label || ' stores another L-polynomial' END)
FROM jsonb_to_recordset($1::jsonb) AS r(qi int, field text, coefficients jsonb, label text, point_counts numeric[],
     generators jsonb, seed text, call text, l_polynomial numeric[])
JOIN ec.field f ON f.icv1_field = r.field
LEFT JOIN ec.isogeny_class_fq lc ON lc.label = r.label
LEFT JOIN ec.curve c ON c.field_id = f.id AND c.model = 'plane_curve' AND c.coefficients = r.coefficients
LEFT JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
LEFT JOIN ec.curve_zeta z ON z.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq zic ON zic.id = z.isogeny_class_id
LEFT JOIN ec.curve_equation_model em ON em.curve_id = c.id
LEFT JOIN ec.generation_provenance gp ON gp.curve_id = c.id",
        zc = prefix_differs("(z.point_counts::numeric[])", "r.point_counts")
    )
}

/// Stored rows of a quotient curve that contradict the computed ones: (slug, reasons).
fn quotient_check_sql() -> String {
    format!(
        "SELECT r.key, concat_ws('; ',
       CASE WHEN c.id IS NOT NULL AND (c.genus <> 1 OR c.model_sha256 IS DISTINCT FROM r.model_sha256
                 OR c.field_id IS DISTINCT FROM f.id) THEN 'slug stored for another curve' END,
       CASE WHEN o.id IS NOT NULL THEN 'the same model is stored under slug ' || coalesce(o.icv1_slug, 'NULL') END,
       CASE WHEN ic.id IS NOT NULL AND ic.label <> r.label THEN 'Jacobian linked to class ' || ic.label END,
       CASE WHEN z.curve_id IS NOT NULL AND {zc} THEN 'stored zeta counts ' || z.point_counts::text END,
       CASE WHEN zic.id IS NOT NULL AND zic.label <> r.label THEN 'zeta row names class ' || zic.label END,
       CASE WHEN gi.curve_id IS NOT NULL AND (gi.a4 IS DISTINCT FROM r.a4 OR gi.a6 IS DISTINCT FROM r.a6
                 OR gi.j_invariant IS DISTINCT FROM r.j_invariant) THEN 'stored invariants differ' END)
FROM jsonb_to_recordset($1::jsonb) AS r(key text, field text, coefficients jsonb, model_sha256 text, label text,
     point_counts numeric[], a4 text, a6 text, j_invariant text)
JOIN ec.field f ON f.icv1_field = r.field
LEFT JOIN ec.curve c ON c.icv1_slug = r.key
LEFT JOIN ec.curve o ON o.field_id = f.id AND o.model = 'short_weierstrass' AND o.coefficients = r.coefficients
     AND o.icv1_slug IS DISTINCT FROM r.key
LEFT JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
LEFT JOIN ec.curve_zeta z ON z.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq zic ON zic.id = z.isogeny_class_id
LEFT JOIN ec.genus1_invariants gi ON gi.curve_id = c.id",
        zc = prefix_differs("(z.point_counts::numeric[])", "r.point_counts")
    )
}

/// Stored morphisms and components that contradict the computed ones: (qi, reasons).
const MAP_CHECK_SQL: &str = "
SELECT r.qi, concat_ws('; ',
       CASE WHEN m.id IS NOT NULL AND (m.map <> r.map OR m.degree IS DISTINCT FROM 2 OR m.status <> 'proved'
                 OR m.method IS DISTINCT FROM r.method)
            THEN 'stored morphism ' || r.construction || ' differs' END,
       CASE WHEN ac.av_id IS NOT NULL AND (ac.multiplicity <> r.multiplicity OR ac.status <> 'proved')
            THEN 'stored av_component to ' || r.target || ' differs' END)
FROM jsonb_to_recordset($1::jsonb) AS r(qi int, field text, coefficients jsonb, target text, construction text,
     map jsonb, method text, multiplicity int)
JOIN ec.field f ON f.icv1_field = r.field
JOIN ec.curve c ON c.field_id = f.id AND c.model = 'plane_curve' AND c.coefficients = r.coefficients
JOIN ec.curve e ON e.icv1_slug = r.target
LEFT JOIN ec.curve_morphism m ON m.source_curve_id = c.id AND m.target_curve_id = e.id AND m.kind = 'quotient'
     AND m.construction = r.construction
LEFT JOIN ec.abelian_variety ja ON ja.curve_id = c.id
LEFT JOIN ec.abelian_variety ea ON ea.curve_id = e.id
LEFT JOIN ec.av_component ac ON ac.av_id = ja.id AND ac.component_av_id = ea.id AND ac.relation = 'isogeny_factor'";

/// After the inserts: morphisms of kept quartics that are not stored exactly.
const MAP_PRESENT_SQL: &str = "
SELECT r.qi, r.construction
FROM jsonb_to_recordset($1::jsonb) AS r(qi int, field text, coefficients jsonb, target text, construction text,
     map jsonb)
WHERE NOT EXISTS (
  SELECT 1 FROM ec.field f
  JOIN ec.curve c ON c.field_id = f.id AND c.model = 'plane_curve' AND c.coefficients = r.coefficients
  JOIN ec.curve e ON e.icv1_slug = r.target
  JOIN ec.curve_morphism m ON m.source_curve_id = c.id AND m.target_curve_id = e.id AND m.kind = 'quotient'
       AND m.construction = r.construction AND m.map = r.map AND m.status = 'proved'
  WHERE f.icv1_field = r.field)";

#[derive(Default)]
struct Rows {
    fields: BTreeMap<String, Value>,
    quartics: Vec<Value>,
    equations: Vec<Value>,
    g3_classes: BTreeMap<String, Value>,
    links: Vec<Value>,
    zeta: Vec<Value>,
    generation: Vec<Value>,
    e_classes: BTreeMap<String, Value>,
    e_curves: BTreeMap<String, Value>,
    e_invariants: BTreeMap<String, Value>,
    e_links: BTreeMap<String, Value>,
    e_zeta: BTreeMap<String, Value>,
    e_aliases: BTreeMap<String, Value>,
    morphisms: Vec<Value>,
    components: Vec<Value>,
    /// Inputs of the conflict checks: one per quartic, one per quotient curve.
    checks: Vec<Value>,
    e_checks: BTreeMap<String, Value>,
}

/// The rows of one verified quartic.
fn rows_of(qi: usize, v: &Verified, rows: &mut Rows) -> Result<()> {
    let p = BigInt::from(v.p);
    let fid = format!("fp-{}", v.p);
    let spec = prime_field(&fid, &p)?;
    rows.fields.entry(fid.clone()).or_insert_with(|| spec.row.clone());
    let coefficients = coeff_json(&v.coeffs);
    let label = weil_label(3, &p, &v.l)?;
    let order: BigInt = v.l.iter().sum();
    rows.g3_classes.entry(label.clone()).or_insert_with(|| {
        json!({
            "p": v.p.to_string(), "l_polynomial": v.l.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            "label": label, "order_value": order.to_string(),
            "curve_counts": v.counts.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            "ordinary": !v.l[3].mod_floor(&p).is_zero(),
        })
    });
    let key = json!({ "qi": qi, "field": fid, "coefficients": coefficients });
    let with = |extra: Value| -> Value {
        let mut o = key.clone();
        if let (Some(m), Value::Object(x)) = (o.as_object_mut(), extra) {
            m.extend(x);
        }
        o
    };
    rows.quartics.push(with(json!({
        "form": FORM, "equation": equation(&v.coeffs), "family": FAMILY,
        "description": format!("Ciani plane quartic over F_{} ({GENERATOR}); Jac(C) ~ E_x x E_y x E_z", v.p),
    })));
    rows.equations.push(with(json!({ "generators": generators(&v.coeffs) })));
    rows.links.push(with(json!({ "label": label })));
    rows.zeta.push(with(json!({
        "label": label, "point_counts": v.counts.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
        "method": METHOD_L,
    })));
    rows.checks.push(with(json!({
        "label": label, "point_counts": v.counts.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
        "generators": generators(&v.coeffs), "seed": format!("{SEED:#018x}"),
        "call": format!("ciani::sample({}, {PER_PRIME})", v.p),
        "l_polynomial": v.l.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    })));
    rows.generation.push(with(json!({
        "seed": format!("{SEED:#018x}"), "procedure": "ecdb/src/import/ciani.rs (ciani_quartic_seeded_v1)",
        "call": format!("ciani::sample({}, {PER_PRIME})", v.p),
    })));
    let mut mult: BTreeMap<String, i32> = BTreeMap::new();
    for (qt, n) in v.quotients.iter().zip(v.orders) {
        let (a, b) = (BigInt::from(qt.short[0]), BigInt::from(qt.short[1]));
        let (slug, model_json, sha, t) = prime_icv1(&p, &a, &b, &BigInt::from(n))?;
        let (elabel, mut class) = genus1_class_row(&spec, &t)?;
        class["status"] = json!("proved");
        rows.e_classes.entry(elabel.clone()).or_insert(class);
        let coeff = json!({ "a": a.to_string(), "b": b.to_string() });
        let inv = invariant_row(&slug, &spec.fq, "short_weierstrass", coeff.as_object().unwrap(), None)?;
        let mut inv = inv;
        inv["status"] = json!("proved");
        rows.e_invariants.entry(slug.clone()).or_insert(inv.clone());
        rows.e_curves.entry(slug.clone()).or_insert_with(|| {
            json!({
                "field": fid, "model": "short_weierstrass", "form": "y^2=x^3+a*x+b", "coefficients": coeff,
                "equation": format!("y^2 = x^3 + {a}*x + {b}"), "family": QUOTIENT_FAMILY,
                "model_json": model_json, "model_sha256": sha, "icv1": null, "slug": slug,
                "is_smooth": true, "smoothness_status": "proved",
            })
        });
        rows.e_links.entry(slug.clone()).or_insert_with(|| json!({ "key": slug, "label": elabel }));
        rows.e_zeta.entry(slug.clone()).or_insert_with(
            || json!({ "key": slug, "label": elabel, "point_counts": [n.to_string()], "method": METHOD_E }),
        );
        rows.e_checks.entry(slug.clone()).or_insert_with(|| {
            json!({
                "key": slug, "field": fid, "coefficients": coeff, "model_sha256": sha, "label": elabel,
                "point_counts": [n.to_string()], "a4": inv["a4"], "a6": inv["a6"], "j_invariant": inv["j_invariant"],
            })
        });
        rows.e_aliases
            .entry(slug.clone())
            .or_insert_with(|| json!({ "key": slug, "namespace": "icv1_slug", "alias": slug, "retired": false }));
        rows.morphisms.push(with(json!({
            "target": slug, "construction": qt.construction(), "map": qt.map_json(), "method": METHOD_MAP,
        })));
        *mult.entry(slug).or_default() += 1;
    }
    for (slug, m) in &mult {
        rows.components.push(with(json!({ "target": slug, "multiplicity": m })));
    }
    // the morphism rows also carry the component multiplicity, for the conflict check
    let n = rows.morphisms.len();
    for m in rows.morphisms[n - 3..].iter_mut() {
        let t = m["target"].as_str().unwrap_or_default().to_string();
        m["multiplicity"] = json!(mult.get(&t).copied().unwrap_or(0));
    }
    Ok(())
}

/// A verified quartic, or (p, coefficients, the failed check).
pub type Outcome = Result<Verified, (u64, Ciani, String)>;

/// Every quartic of the generator, verified (blocking; run off the runtime).
pub fn run_all() -> (Vec<Outcome>, usize) {
    let mut out = Vec::new();
    let mut rejected = 0;
    for &p in PRIMES.iter() {
        let fields = [SmallField::new(p, 1), SmallField::new(p, 2), SmallField::new(p, 3)];
        let (sampled, rej) = sample(p, PER_PRIME);
        rejected += rej;
        for c in sampled {
            out.push(quotients(p, &c).and_then(|q| verify(&fields, &c, &q)).map_err(|e| (p, c, e)));
        }
    }
    (out, rejected)
}

/// Generate, verify and store the Ciani quartics and their quotients.
pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: format!("{GENERATOR}: {PER_PRIME} Ciani quartics per prime 5 <= p < 64 (seed {SEED:#018x})"),
        sha256: crate::math::sha256_hex(include_str!("ciani.rs")),
        ..Default::default()
    };
    let client = ctx.client;
    let (results, rejected) = tokio::task::spawn_blocking(run_all).await?;
    report.count("ciani_rejected_singular", rejected as u64);
    let mut verified: Vec<&Verified> = Vec::new();
    for r in &results {
        report.count("ciani_sampled", 1);
        match r {
            Ok(v) => {
                verified.push(v);
                report.count("ciani_verified", 1);
            }
            Err((p, c, msg)) => {
                report.count("ciani_crosscheck_failures", 1);
                report.problem(format!("Ciani quartic {c:?} over F_{p}: cross-check failed, nothing stored: {msg}"));
            }
        }
    }
    // Compare with what is stored; a quartic meeting any conflicting row is refused whole.
    let mut all = Rows::default();
    for (qi, v) in verified.iter().enumerate() {
        rows_of(qi, v, &mut all)?;
    }
    let refused = conflicts(client, &all, &mut report).await?;
    report.count("ciani_refused_conflict", refused.len() as u64);
    let mut rows = Rows::default();
    for (qi, v) in verified.iter().enumerate() {
        if !refused.contains(&qi) {
            rows_of(qi, v, &mut rows)?;
        }
    }
    let src = ctx
        .source(
            "ecdb/src/import/ciani.rs",
            &report.sha256,
            "Ciani plane quartics and their elliptic quotients, generated and verified by ecdb",
        )
        .await?;
    let prov_l = ctx.provenance(src, METHOD_L, None).await?;
    let prov_e = ctx.provenance(src, METHOD_E, None).await?;
    let prov_m = ctx.provenance(src, METHOD_MAP, None).await?;
    let vals = |m: BTreeMap<String, Value>| -> Vec<Value> { m.into_values().collect() };

    report.count("ciani_fields", bulk(client, FIELD_SQL, &vals(rows.fields), prov_l).await?);
    report.count("ciani_quartics", bulk(client, QUARTIC_SQL, &rows.quartics, prov_l).await?);
    report.count("ciani_equation_models", bulk(client, &equation_sql(), &rows.equations, prov_l).await?);
    report.count("ciani_generation", bulk(client, &generation_sql(), &rows.generation, prov_l).await?);
    report.count("ciani_classes_genus3", bulk(client, G3_CLASS_SQL, &vals(rows.g3_classes), prov_l).await?);
    report.count("ciani_jacobians_linked", bulk1(client, &link_sql(), &rows.links).await?);
    report.count("ciani_zeta", bulk(client, &zeta_sql(), &rows.zeta, prov_l).await?);

    report.count("ciani_quotient_classes", bulk(client, G1_CLASS_SQL, &vals(rows.e_classes), prov_e).await?);
    report.count("ciani_quotient_curves", bulk(client, CURVE_SQL, &vals(rows.e_curves), prov_e).await?);
    report.count("ciani_quotient_invariants", bulk(client, INVARIANT_SQL, &vals(rows.e_invariants), prov_e).await?);
    bulk1(client, ALIAS_SQL, &vals(rows.e_aliases)).await?;
    report.count("ciani_quotient_links", bulk1(client, E_LINK_SQL, &vals(rows.e_links)).await?);
    report.count("ciani_quotient_zeta", bulk(client, E_ZETA_SQL, &vals(rows.e_zeta), prov_e).await?);

    report.count("ciani_morphisms", bulk(client, &morphism_sql(), &rows.morphisms, prov_m).await?);
    report.count("ciani_av_components", bulk(client, &component_sql(), &rows.components, prov_m).await?);

    // Every kept quartic's three maps must now be stored exactly.
    let missing = client.query(MAP_PRESENT_SQL, &[&Value::Array(rows.morphisms.clone())]).await?;
    for r in &missing {
        let (qi, construction): (i32, String) = (r.get(0), r.get(1));
        let v = verified[qi as usize];
        report.count("ciani_morphisms_missing", 1);
        report.problem(format!(
            "Ciani quartic {:?} over F_{}: morphism {construction} is not stored after the import",
            v.coeffs, v.p
        ));
    }
    Ok(report)
}

/// The quartics (indices into the verified list) whose rows, or whose
/// quotient curves' rows, conflict with stored values; each conflict is a
/// problem. Read-only.
async fn conflicts(client: &tokio_postgres::Client, rows: &Rows, report: &mut Report) -> Result<BTreeSet<usize>> {
    let mut refused = BTreeSet::new();
    let describe = |rows: &Rows, qi: usize| -> String {
        rows.checks
            .iter()
            .find(|r| r["qi"] == json!(qi))
            .map(|r| format!("{} {}", r["field"], r["coefficients"]))
            .unwrap_or_default()
    };
    for r in client.query(&quartic_check_sql(), &[&Value::Array(rows.checks.clone())]).await? {
        let (qi, why): (i32, String) = (r.get(0), r.get(1));
        if !why.is_empty() {
            refused.insert(qi as usize);
            report.problem(format!(
                "Ciani quartic {}: conflicts with stored rows, nothing stored: {why}",
                describe(rows, qi as usize)
            ));
        }
    }
    let e_rows: Vec<Value> = rows.e_checks.values().cloned().collect();
    for r in client.query(&quotient_check_sql(), &[&Value::Array(e_rows)]).await? {
        let (slug, why): (String, String) = (r.get(0), r.get(1));
        if !why.is_empty() {
            for m in rows.morphisms.iter().filter(|m| m["target"] == json!(slug)) {
                let qi = m["qi"].as_u64().unwrap_or_default() as usize;
                refused.insert(qi);
                report.problem(format!(
                    "Ciani quartic {}: its quotient {slug} conflicts with stored rows, nothing stored: {why}",
                    describe(rows, qi)
                ));
            }
        }
    }
    for r in client.query(MAP_CHECK_SQL, &[&Value::Array(rows.morphisms.clone())]).await? {
        let (qi, why): (i32, String) = (r.get(0), r.get(1));
        if !why.is_empty() {
            refused.insert(qi as usize);
            report.problem(format!(
                "Ciani quartic {}: conflicts with stored rows, nothing stored: {why}",
                describe(rows, qi as usize)
            ));
        }
    }
    Ok(refused)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields(p: u64) -> [SmallField; 3] {
        [SmallField::new(p, 1), SmallField::new(p, 2), SmallField::new(p, 3)]
    }

    fn eval(fld: &SmallField, c: &Ciani, x: &E3, y: &E3, z: &E3) -> [E3; 4] {
        let [a, b, cc, d, e, f] = *c;
        let (x2, y2, z2) = (fld.mul(x, x), fld.mul(y, y), fld.mul(z, z));
        let terms = [
            fld.scale(a, &fld.mul(&x2, &x2)),
            fld.scale(b, &fld.mul(&y2, &y2)),
            fld.scale(cc, &fld.mul(&z2, &z2)),
            fld.scale(d, &fld.mul(&y2, &z2)),
            fld.scale(e, &fld.mul(&x2, &z2)),
            fld.scale(f, &fld.mul(&x2, &y2)),
        ];
        let fv = terms.iter().fold([0; 3], |acc, t| fld.add(&acc, t));
        // partial derivatives divided by 2x, 2y, 2z times x, y, z: F_x = 2x(2a x^2 + f y^2 + e z^2)
        let lin = |c1: u64, v1: &E3, c2: u64, v2: &E3, c3: u64, v3: &E3| {
            fld.add(&fld.add(&fld.scale(c1, v1), &fld.scale(c2, v2)), &fld.scale(c3, v3))
        };
        let fx = fld.mul(x, &lin(2 * a, &x2, f, &y2, e, &z2));
        let fy = fld.mul(y, &lin(2 * b, &y2, f, &x2, d, &z2));
        let fz = fld.mul(z, &lin(2 * cc, &z2, e, &x2, d, &y2));
        [fv, fx, fy, fz]
    }

    /// Every projective point of P^2(F_q).
    fn projective_points(fld: &SmallField) -> Vec<[E3; 3]> {
        let one = fld.konst(1);
        let mut v = Vec::new();
        for i in 0..fld.q {
            for j in 0..fld.q {
                v.push([fld.elt(i), fld.elt(j), one]);
            }
            v.push([fld.elt(i), one, [0; 3]]);
        }
        v.push([one, [0; 3], [0; 3]]);
        v
    }

    fn brute_count(fld: &SmallField, c: &Ciani) -> u64 {
        projective_points(fld).iter().filter(|[x, y, z]| fld.is_zero(&eval(fld, c, x, y, z)[0])).count() as u64
    }

    fn has_singular_point(fld: &SmallField, c: &Ciani) -> bool {
        projective_points(fld).iter().any(|[x, y, z]| eval(fld, c, x, y, z).iter().all(|v| fld.is_zero(v)))
    }

    #[test]
    fn small_field_is_a_field() {
        for (p, k) in [(5, 2), (5, 3), (7, 3), (11, 2)] {
            let f = SmallField::new(p, k);
            // every non-zero element has an inverse, and squares are half of the units
            let mut squares = 0;
            for i in 1..f.q {
                let a = f.elt(i);
                assert!((1..f.q).any(|j| f.mul(&a, &f.elt(j)) == f.konst(1)), "{p}^{k}: {a:?} not invertible");
                if f.sqrt(&a).is_some() {
                    squares += 1;
                }
            }
            assert_eq!(squares, (f.q - 1) / 2);
        }
    }

    #[test]
    fn fibre_counts_match_brute_force_on_tiny_fields() {
        for p in [5u64, 7, 11] {
            let (cs, _) = sample(p, 4);
            let flds = [SmallField::new(p, 1), SmallField::new(p, 2)];
            for c in &cs {
                for f in &flds {
                    assert_eq!(count_points(f, c), brute_count(f, c), "p = {p}, k = {}, {c:?}", f.k);
                }
            }
            // F_{p^3} for p = 5 only (125^2 points)
            if p == 5 {
                let f3 = SmallField::new(5, 3);
                for c in &cs {
                    assert_eq!(count_points(&f3, c), brute_count(&f3, c));
                }
            }
        }
    }

    #[test]
    fn smoothness_criterion_matches_a_singular_point_search_over_fp2() {
        // all singular points are defined over F_{p^2} (module docs)
        let mut state = 7u64;
        for p in [5u64, 7] {
            let f2 = SmallField::new(p, 2);
            let (mut smooth, mut singular) = (0, 0);
            for _ in 0..120 {
                let mut c = [0u64; 6];
                for x in c.iter_mut() {
                    *x = splitmix(&mut state) % p;
                }
                let crit = is_smooth(p, &c);
                assert_eq!(crit, !has_singular_point(&f2, &c), "p = {p}, {c:?}");
                if crit {
                    smooth += 1;
                } else {
                    singular += 1;
                }
            }
            assert!(smooth > 10 && singular > 10, "both cases exercised: {smooth} / {singular}");
        }
    }

    #[test]
    fn quotient_maps_land_on_the_curve_and_l_factors() {
        for p in [5u64, 7, 11, 13, 17] {
            let flds = fields(p);
            let (cs, _) = sample(p, PER_PRIME);
            assert_eq!(cs.len(), PER_PRIME);
            for c in &cs {
                let q = quotients(p, c).unwrap();
                let v = verify(&flds, c, &q).unwrap_or_else(|e| panic!("p = {p}, {c:?}: {e}"));
                assert_eq!(v.l.len(), 7);
                assert_eq!(v.l[6], BigInt::from(p.pow(3)));
                // the map is exercised on most points
                for (i, m) in v.mapped_points.iter().enumerate() {
                    assert!(*m + 8 >= v.counts[0] as usize, "{p} {c:?} E_{i}: {m} of {}", v.counts[0]);
                }
            }
        }
    }

    #[test]
    fn every_chart_case_is_exercised() {
        let (mut plain, mut swapped, mut translated) = (0, 0, 0);
        for &p in PRIMES.iter().take(8) {
            for c in sample(p, PER_PRIME).0 {
                for q in quotients(p, &c).unwrap() {
                    match (q.swapped, q.u0) {
                        (false, 0) => plain += 1,
                        (true, _) => swapped += 1,
                        _ => translated += 1,
                    }
                }
            }
        }
        assert!(plain > 0 && swapped > 0 && translated > 0, "{plain} {swapped} {translated}");
    }

    /// Fermat quartic x^4 + y^4 + z^4: the Fermat curve of degree d is maximal
    /// over F_{p^2} when d | p + 1 (e.g. Stichtenoth, Algebraic Function Fields
    /// and Codes, 2nd ed., Prop. 6.4.1 and the Hermitian case), so for p = 7, 11
    /// L(T) = (1 + pT^2)^3: #C(F_p) = p + 1, #C(F_{p^2}) = p^2 + 1 + 6p.
    /// Over F_5 it has no points (x^4 is 0 or 1 and three of them never sum to
    /// 0 mod 5 unless all vanish).
    #[test]
    fn fermat_quartic_pinned_values() {
        let c = [1, 1, 1, 0, 0, 0];
        for p in [7u64, 11] {
            let flds = fields(p);
            let v = verify(&flds, &c, &quotients(p, &c).unwrap()).unwrap();
            let pb = BigInt::from(p);
            let expect: Vec<BigInt> =
                [1, 0, 3 * p, 0, 3 * p * p, 0, p * p * p].iter().map(|x| BigInt::from(*x)).collect();
            assert_eq!(v.l, expect);
            assert_eq!(v.counts[0], p + 1);
            assert_eq!(v.counts[1], p * p + 1 + 6 * p);
            assert_eq!(v.orders, [p + 1; 3]);
            let label = weil_label(3, &pb, &v.l).unwrap();
            assert_eq!(label, if p == 7 { "3.7.a_v_a" } else { "3.11.a_bh_a" });
        }
        let f5 = fields(5);
        let v = verify(&f5, &c, &quotients(5, &c).unwrap()).unwrap();
        assert_eq!(v.counts[0], 0);
        assert_eq!(v.l[1], BigInt::from(-6));
    }

    #[test]
    fn corrupted_inputs_are_rejected() {
        let p = 13;
        let flds = fields(p);
        let c = sample(p, 1).0[0];
        let good = quotients(p, &c).unwrap();
        assert!(verify(&flds, &c, &good).is_ok());
        // a wrong quotient curve: change B of E_y
        let mut bad = good.clone();
        bad[1].short[1] = (bad[1].short[1] + 1) % p;
        let err = verify(&flds, &c, &bad).unwrap_err();
        assert!(err.contains("E_y"), "{err}");
        // quotients of another quartic claimed for this one
        let other = sample(p, 2).0[1];
        let wrong = quotients(p, &other).unwrap();
        assert!(verify(&flds, &c, &wrong).is_err());
        // a singular quartic is refused
        assert!(verify(&flds, &[1, 1, 0, 0, 0, 0], &good).is_err());
    }

    #[test]
    fn generator_is_deterministic_and_documented() {
        let (a, ra) = sample(11, PER_PRIME);
        let (b, rb) = sample(11, PER_PRIME);
        assert_eq!((a.clone(), ra), (b, rb));
        assert!(a.iter().all(|c| is_smooth(11, c) && c.iter().all(|x| *x < 11)));
        // pinned first draw, so a change to the generator is visible
        assert_eq!(sample(5, 1).0[0], FIRST_DRAW_P5);
    }

    const FIRST_DRAW_P5: Ciani = [1, 2, 4, 4, 3, 0];

    #[test]
    fn quartics_without_a_usable_chart_point_are_refused() {
        // smooth, but D_x: w^2 = alpha y^4 + beta y^2 z^2 + gamma z^4 has only branch points over F_5
        let c = [3, 1, 1, 0, 0, 3];
        assert!(is_smooth(5, &c));
        let err = quotients(5, &c).unwrap_err();
        assert!(err.contains("D_x"), "{err}");
        // independently: every F_5-point of D_x has w = 0, or lies at t = 0 with alpha a non-square
        let [a, b, cc, d, e, f] = c.map(|x| x as i64);
        let (al, be, ga) = (
            (f * f - 4 * a * b).rem_euclid(5),
            (2 * e * f - 4 * a * d).rem_euclid(5),
            (e * e - 4 * a * cc).rem_euclid(5),
        );
        assert_eq!(chi(al as u64, 5), -1);
        for u in 0..5i64 {
            let val = (al * u.pow(4) + be * u * u + ga).rem_euclid(5) as u64;
            assert!(chi(val, 5) != 1, "u = {u}: a point with w != 0");
        }
    }

    #[test]
    fn rows_have_the_registry_shape() {
        let p = 7;
        let c = sample(p, 1).0[0];
        let v = verify(&fields(p), &c, &quotients(p, &c).unwrap()).unwrap();
        let mut rows = Rows::default();
        rows_of(0, &v, &mut rows).unwrap();
        assert_eq!(rows.quartics.len(), 1);
        assert_eq!(rows.morphisms.len(), 3);
        let m = &rows.morphisms[0];
        assert_eq!(m["construction"], "ciani_quotient_x_v1");
        assert!(m["target"].as_str().unwrap().starts_with("icv1-fp3-t"));
        let comp: i64 = rows.components.iter().map(|r| r["multiplicity"].as_i64().unwrap()).sum();
        assert_eq!(comp, 3);
        let g3 = rows.g3_classes.values().next().unwrap();
        assert!(g3["label"].as_str().unwrap().starts_with("3.7."));
        assert_eq!(rows.zeta[0]["point_counts"].as_array().unwrap().len(), 3);
        assert_eq!(rows.equations[0]["generators"].as_array().unwrap().len(), 1);
    }
}
