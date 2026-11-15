//! Genus-2 curves glued from two elliptic curves along their 2-torsion: for
//! census-fp pairs (E1, E2) with full rational 2-torsion over a small prime
//! field, the curve C with Jac(C) (2,2)-isogenous to E1 x E2, the two degree-2
//! maps C -> E1 and C -> E2, and the exact L-polynomial of Jac(C).
//!
//! # Mathematics
//!
//! **Formula source.** Howe, Leprévost and Poonen, "Large torsion subgroups
//! of split Jacobians of curves of genus two or three", Forum Math. 12 (2000),
//! Proposition 4 (the (2,2) case of Frey–Kani, "Curves of genus 2 covering
//! elliptic curves and an arithmetical application", 1991). Let
//! E1: y^2 = (x - a1)(x - a2)(x - a3) and E2: y^2 = (x - b1)(x - b2)(x - b3)
//! over a field k of characteristic not 2, and psi: E1[2] -> E2[2] the group
//! isomorphism (a_i, 0) -> (b_i, 0) (every group isomorphism of (Z/2)^2 is an
//! anti-isometry for the Weil pairing). Put
//!
//! * A1 = (a3 - a2)^2/(b3 - b2) + (a2 - a1)^2/(b2 - b1) + (a1 - a3)^2/(b1 - b3),
//! * B1 = (b3 - b2)^2/(a3 - a2) + (b2 - b1)^2/(a2 - a1) + (b1 - b3)^2/(a1 - a3),
//! * A2 = a1(b3 - b2) + a2(b1 - b3) + a3(b2 - b1),
//! * B2 = b1(a3 - a2) + b2(a1 - a3) + b3(a2 - a1),
//! * Δ1 = ((a1 - a2)(a2 - a3)(a3 - a1))^2, Δ2 = ((b1 - b2)(b2 - b3)(b3 - b1))^2,
//!   A = Δ2 A1 / A2, B = Δ1 B1 / B2.
//!
//! If psi is not the restriction of a geometric isomorphism E1 -> E2, then
//! A1, B1 != 0, and
//!
//! > C: y^2 = -(A(a2 - a1)(a1 - a3) x^2 + B(b2 - b1)(b1 - b3))
//! >           (A(a3 - a2)(a2 - a1) x^2 + B(b3 - b2)(b2 - b1))
//! >           (A(a1 - a3)(a3 - a2) x^2 + B(b1 - b3)(b3 - b2))
//!
//! is a curve of genus 2 over k whose Jacobian is isogenous to E1 x E2 by an
//! isogeny with kernel the graph of psi. A triple with A1, B1, A2 or B2 equal
//! to 0 is counted `degenerate` and skipped (A2 = 0 or B2 = 0 leaves A or B
//! undefined).
//!
//! **The maps.** The sextic is even, h(u) = G(u^2) with G(X) = h0 + h2 X +
//! h4 X^2 + h6 X^3, so C has the involutions u -> -u and (u, v) -> (-u, -v).
//! Their quotients are Q_X: Y^2 = G(X) (X = u^2, Y = v) and Q_Z: W^2 =
//! Z^3 G(1/Z) (Z = 1/u^2, W = v/u^3). For each target E (y^2 = x^3 + a x + b
//! with roots e_i) the importer looks for an F_p-isomorphism Q -> E of the
//! form X = lambda x + mu, Y = w y: the roots of Q must be lambda e_s(i) + mu
//! for a permutation s, and w^2 = (leading coefficient of Q) lambda^3 must be a
//! square (otherwise Q is the quadratic twist, and the pair is refused). The
//! composite C -> E is stored as x = xn(u)/xd(u), y = v yn(u)/yd(u) and is
//! re-proved as a polynomial identity over F_p:
//!
//! > h(u) yn(u)^2 xd(u)^3 = yd(u)^2 (xn^3 + a xn xd^2 + b xd^3)(u),
//!
//! so the map is a morphism onto E. x o phi has degree 4 on C (degree 2 in u,
//! and u has degree 2), hence deg phi = 2. One quotient must go to E1 and the
//! other to E2.
//!
//! **Checks.** Before anything is written:
//! - each census trace is recounted exhaustively and must equal the trace in
//!   ecdb (a curve that disagrees is refused);
//! - C is smooth of genus 2: deg h = 6 and gcd(h, h') = 1 over F_p;
//! - both maps pass the identity above;
//! - L(Jac C) from exhaustive counts #C(F_p) and #C(F_p^2) (`cover_lpoly`'s
//!   counters: a1 = #C(F_p) - p - 1, a2 = (#C(F_p^2) - p^2 - 1 + a1^2)/2,
//!   which determine L for genus 2) must equal L(E1) L(E2) = (1 - t1 T +
//!   pT^2)(1 - t2 T + pT^2), with t1, t2 the traces ecdb holds.
//!
//! - the kernel: the two stored maps send each Weierstrass pair {(±r, 0)} of
//!   C to (a_i, 0) on E1 and (b_i, 0) on E2 for the same i (see
//!   [`kernel_is_graph_of_psi`]). Since phi1^*(T1 - O1) - phi2^*(T2 - O2) is
//!   the divisor of u, each (T1, psi(T1)) lies in the order-4 kernel of
//!   phi1^* + phi2^*, which is therefore the graph of psi. For j = 0 or 1728
//!   several isomorphisms Q -> E exist (they differ by automorphisms that
//!   permute E[2]); a pair that realizes psi is kept, the others are not.
//!
//! A failure is a problem line (`gluing_crosscheck_failures`) and nothing of
//! that gluing is stored. That the kernel has order 4 (phi^* injective for a
//! degree-2 map not factoring through an isogeny) is the theorem's (Frey–Kani,
//! Kani); ecdb verifies the isogeny class (Tate: equal L-polynomials), the two
//! maps and the correspondence of the 2-torsion above.
//!
//! **Existing rows.** Before any write, the stored rows of every glued curve
//! (curve, hyperelliptic_model, curve_zeta, the Jacobian's class, its
//! gluing_2_2 morphisms, the class under the computed label) are compared with
//! the computed values. A stored NULL is no disagreement; any other difference
//! refuses that curve as a whole (`gluing_existing_conflicts`, a problem line,
//! nothing of it written). Agreeing rows are left as they are.
//!
//! # Scope
//!
//! Census-fp short-Weierstrass curves over F_p, 5 <= p < [`MAX_P`], whose
//! cubic splits into three distinct linear factors over F_p. Every unordered
//! pair {E1, E2} (E1 = E2 included) and each of the six bijections of the
//! sorted 2-torsion abscissae. n = 3 (Kani's (3,3) gluing) is not computed.
//!
//! # Writes
//!
//! Insert-only (`ON CONFLICT DO NOTHING`; a Jacobian is linked only where its
//! class is NULL): `curve` (family `gluing`, model `hyperelliptic`),
//! `hyperelliptic_model`, two `curve_morphism` rows of kind `gluing_2_2`
//! (construction [`CONSTRUCTION_X`] or [`CONSTRUCTION_Z`] by the quotient the
//! map factors through, so a self-gluing E1 = E2 keeps both maps; degree 2,
//! the map and the gluing data in
//! `map`), `isogeny_class_fq` (status `proved`), `abelian_variety.isogeny_class_id`
//! and `curve_zeta`. A curve whose stored rows disagree with the gluing (a
//! Jacobian linked to another class among them) is a problem, never
//! overwritten, and nothing of it is written.

use super::cover_lpoly::{class_row, count_hyperelliptic_fp, count_hyperelliptic_fp2, Computed, Method, CLASS_SQL};
use super::{bulk, bulk1, Ctx, Report};
use anyhow::Result;
use num_bigint::BigInt;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Fields covered: 5 <= p < MAX_P.
pub const MAX_P: u64 = 50;

/// The construction of a map through the quotient X = u^2 (u -> -u).
pub const CONSTRUCTION_X: &str = "hlp_2_2_gluing_v1_qx";
/// The construction of a map through the quotient Z = 1/u^2 ((u, v) -> (-u, -v)).
pub const CONSTRUCTION_Z: &str = "hlp_2_2_gluing_v1_qz";

const METHOD: &str = "Howe–Leprévost–Poonen 2000, Prop. 4 ((2,2) Frey–Kani gluing along psi: (a_i,0) -> (b_i,0)); \
                      C smooth (deg h = 6, gcd(h, h') = 1); maps C -> E1, E2 re-proved as polynomial identities over F_p; \
                      L(Jac C) from exhaustive #C(F_p), #C(F_p^2) equals L(E1) L(E2) with the census traces in ecdb \
                      (recounted); kernel: the stored maps send each Weierstrass pair of C to psi-corresponding 2-torsion \
                      points of E1 and E2";

const ZETA_METHOD: &str =
    "exhaustive count of #C(F_p) and #C(F_p^2) of the glued curve; L(T) = 1 + a1 T + a2 T^2 + p a1 T^3 \
                           + p^2 T^4 equals L(E1) L(E2)";

// ------------------------------------------------------------ F_p arithmetic

fn md(x: i128, p: u64) -> u64 {
    x.rem_euclid(p as i128) as u64
}

fn pw(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = (r as u128 * b as u128 % p as u128) as u64;
        }
        b = (b as u128 * b as u128 % p as u128) as u64;
        e >>= 1;
    }
    r
}

/// The inverse of a nonzero residue (p prime).
fn inv(a: u64, p: u64) -> u64 {
    pw(a, p - 2, p)
}

fn mul(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}

fn sub(a: u64, b: u64, p: u64) -> u64 {
    md(a as i128 - b as i128, p)
}

/// A square root of a in F_p by search (p small), `None` for a non-square.
fn sqrt_small(a: u64, p: u64) -> Option<u64> {
    (0..p).find(|&w| mul(w, w, p) == a % p)
}

// ------------------------------------------------- polynomials over F_p

/// Drop high zero coefficients (ascending representation).
fn trim(mut f: Vec<u64>) -> Vec<u64> {
    while f.last() == Some(&0) {
        f.pop();
    }
    f
}

fn pmul(f: &[u64], g: &[u64], p: u64) -> Vec<u64> {
    if f.is_empty() || g.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; f.len() + g.len() - 1];
    for (i, &a) in f.iter().enumerate() {
        for (j, &b) in g.iter().enumerate() {
            r[i + j] = (r[i + j] + mul(a, b, p)) % p;
        }
    }
    trim(r)
}

fn padd(f: &[u64], g: &[u64], p: u64) -> Vec<u64> {
    let n = f.len().max(g.len());
    trim((0..n).map(|i| (f.get(i).copied().unwrap_or(0) + g.get(i).copied().unwrap_or(0)) % p).collect())
}

fn pscale(f: &[u64], c: u64, p: u64) -> Vec<u64> {
    trim(f.iter().map(|&a| mul(a, c, p)).collect())
}

fn peval(f: &[u64], x: u64, p: u64) -> u64 {
    f.iter().rev().fold(0, |acc, &c| (mul(acc, x, p) + c) % p)
}

fn pderiv(f: &[u64], p: u64) -> Vec<u64> {
    trim(f.iter().enumerate().skip(1).map(|(i, &c)| mul(c, i as u64 % p, p)).collect())
}

/// f mod g, g nonzero.
fn prem(f: &[u64], g: &[u64], p: u64) -> Vec<u64> {
    let mut r = trim(f.to_vec());
    let g = trim(g.to_vec());
    let lc = inv(*g.last().expect("nonzero divisor"), p);
    while r.len() >= g.len() && !r.is_empty() {
        let shift = r.len() - g.len();
        let c = mul(*r.last().unwrap(), lc, p);
        for (i, &b) in g.iter().enumerate() {
            r[shift + i] = sub(r[shift + i], mul(c, b, p), p);
        }
        r = trim(r);
    }
    r
}

/// Is gcd(f, f') = 1, i.e. f squarefree over F_p (f of degree < p)?
fn squarefree(f: &[u64], p: u64) -> bool {
    let (mut a, mut b) = (trim(f.to_vec()), pderiv(f, p));
    while !b.is_empty() {
        let r = prem(&a, &b, p);
        a = b;
        b = r;
    }
    a.len() == 1
}

fn ppow(f: &[u64], e: usize, p: u64) -> Vec<u64> {
    (0..e).fold(vec![1], |acc, _| pmul(&acc, f, p))
}

// ------------------------------------------------------------ the gluing

/// The three distinct roots of x^3 + a x + b in F_p, ascending; `None` unless
/// the cubic splits into distinct linear factors.
pub fn two_torsion(a: u64, b: u64, p: u64) -> Option<[u64; 3]> {
    let r: Vec<u64> = (0..p).filter(|&x| peval(&[b % p, a % p, 0, 1], x, p) == 0).collect();
    (r.len() == 3).then(|| [r[0], r[1], r[2]])
}

/// The HLP sextic (ascending, length 7) for roots `a` of E1 and `b` of E2,
/// glued along (a_i, 0) -> (b_i, 0); `None` in the degenerate case.
pub fn hlp_sextic(a: [u64; 3], b: [u64; 3], p: u64) -> Option<Vec<u64>> {
    let d = |x: u64, y: u64| sub(x, y, p);
    let [a1, a2, a3] = a;
    let [b1, b2, b3] = b;
    let frac = |n: u64, den: u64| mul(mul(n, n, p), inv(den, p), p);
    let big_a1 = (frac(d(a3, a2), d(b3, b2)) + frac(d(a2, a1), d(b2, b1)) + frac(d(a1, a3), d(b1, b3))) % p;
    let big_b1 = (frac(d(b3, b2), d(a3, a2)) + frac(d(b2, b1), d(a2, a1)) + frac(d(b1, b3), d(a1, a3))) % p;
    let big_a2 = (mul(a1, d(b3, b2), p) + mul(a2, d(b1, b3), p) + mul(a3, d(b2, b1), p)) % p;
    let big_b2 = (mul(b1, d(a3, a2), p) + mul(b2, d(a1, a3), p) + mul(b3, d(a2, a1), p)) % p;
    if big_a1 == 0 || big_b1 == 0 || big_a2 == 0 || big_b2 == 0 {
        return None;
    }
    let sq = |x: u64| mul(x, x, p);
    let delta1 = sq(mul(mul(d(a1, a2), d(a2, a3), p), d(a3, a1), p));
    let delta2 = sq(mul(mul(d(b1, b2), d(b2, b3), p), d(b3, b1), p));
    let big_a = mul(mul(delta2, big_a1, p), inv(big_a2, p), p);
    let big_b = mul(mul(delta1, big_b1, p), inv(big_b2, p), p);
    let factor = |ai: u64, aj: u64, ak: u64, bi: u64, bj: u64, bk: u64| {
        // A (aj - ai)(ai - ak) x^2 + B (bj - bi)(bi - bk)
        vec![mul(big_b, mul(d(bj, bi), d(bi, bk), p), p), 0, mul(big_a, mul(d(aj, ai), d(ai, ak), p), p)]
    };
    let f1 = factor(a1, a2, a3, b1, b2, b3);
    let f2 = factor(a2, a3, a1, b2, b3, b1);
    let f3 = factor(a3, a1, a2, b3, b1, b2);
    let h = pscale(&pmul(&pmul(&f1, &f2, p), &f3, p), p - 1, p);
    let mut h = h;
    h.resize(7, 0);
    Some(h)
}

/// A degree-2 map C -> E: x = xn(u)/xd(u), y = v yn(u)/yd(u).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Map {
    pub xn: Vec<u64>,
    pub xd: Vec<u64>,
    pub yn: Vec<u64>,
    pub yd: Vec<u64>,
}

impl Map {
    fn json(&self) -> Value {
        let s = |v: &[u64]| v.iter().map(|c| c.to_string()).collect::<Vec<_>>();
        json!({ "x": { "num": s(&self.xn), "den": s(&self.xd) }, "y": { "v_num": s(&self.yn), "den": s(&self.yd) } })
    }
}

/// h yn^2 xd^3 = yd^2 (xn^3 + a xn xd^2 + b xd^3): the map sends C: v^2 = h(u)
/// onto y^2 = x^3 + a x + b.
pub fn map_is_morphism(h: &[u64], m: &Map, a: u64, b: u64, p: u64) -> bool {
    let lhs = pmul(&pmul(h, &ppow(&m.yn, 2, p), p), &ppow(&m.xd, 3, p), p);
    let cubic = padd(
        &padd(&ppow(&m.xn, 3, p), &pscale(&pmul(&m.xn, &ppow(&m.xd, 2, p), p), a, p), p),
        &pscale(&ppow(&m.xd, 3, p), b, p),
        p,
    );
    let rhs = pmul(&ppow(&m.yd, 2, p), &cubic, p);
    !lhs.is_empty() && lhs == rhs
}

/// Which quotient of C a map factors through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quotient {
    /// X = u^2, Y = v onto Y^2 = G(X).
    X,
    /// Z = 1/u^2, W = v/u^3 onto W^2 = Z^3 G(1/Z).
    Z,
}

/// Every F_p-isomorphism of the quotient `qt` of C (even sextic `h`) onto
/// E: y^2 = x^3 + a x + b with roots `e`, each composed into a map C -> E
/// (one per admissible matching of the roots; for j = 0 or 1728 they differ by
/// automorphisms of E, which permute E[2]).
fn maps_through(h: &[u64], qt: Quotient, a: u64, b: u64, e: [u64; 3], p: u64) -> Vec<Map> {
    let g = match qt {
        Quotient::X => [h[0], h[2], h[4], h[6]],
        Quotient::Z => [h[6], h[4], h[2], h[0]],
    };
    if g[3] == 0 {
        return vec![];
    }
    let roots: Vec<u64> = (0..p).filter(|&x| peval(&g, x, p) == 0).collect();
    if roots.len() != 3 {
        return vec![];
    }
    const PERMS: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut out = vec![];
    for s in PERMS {
        let es = [e[s[0]], e[s[1]], e[s[2]]];
        let lam = mul(sub(roots[1], roots[0], p), inv(sub(es[1], es[0], p), p), p);
        let mu = sub(roots[0], mul(lam, es[0], p), p);
        if lam == 0 || (mul(lam, es[2], p) + mu) % p != roots[2] {
            continue;
        }
        let Some(w) = sqrt_small(mul(g[3], pw(lam, 3, p), p), p) else { continue };
        let (il, iw) = (inv(lam, p), inv(w, p));
        // x = (X - mu)/lambda, y = Y/w
        let m = match qt {
            Quotient::X => {
                Map { xn: trim(vec![mul(p - mu % p, il, p), 0, il]), xd: vec![1], yn: vec![iw], yd: vec![1] }
            }
            // X = 1/u^2: x = (1 - mu u^2)/(lambda u^2), y = v/(w u^3)
            Quotient::Z => Map {
                xn: trim(vec![il, 0, mul(p - mu % p, il, p)]),
                xd: vec![0, 0, 1],
                yn: vec![iw],
                yd: vec![0, 0, 0, 1],
            },
        };
        if map_is_morphism(h, &m, a, b, p) && !out.contains(&m) {
            out.push(m);
        }
    }
    out
}

/// f(u) for an even polynomial f at u^2 = s (f has only even powers of u).
fn peval_even(f: &[u64], s: u64, p: u64) -> Option<u64> {
    if f.iter().skip(1).step_by(2).any(|&c| c != 0) {
        return None;
    }
    let half: Vec<u64> = f.iter().step_by(2).copied().collect();
    Some(peval(&half, s, p))
}

/// The x-coordinate on E of the image of the Weierstrass pair {(r, 0), (-r, 0)}
/// of C, r^2 = s a root of G, under x = xn(u)/xd(u).
fn image_of_pair(m: &Map, s: u64, p: u64) -> Option<u64> {
    let (n, d) = (peval_even(&m.xn, s, p)?, peval_even(&m.xd, s, p)?);
    (d != 0).then(|| mul(n, inv(d, p), p))
}

/// The kernel cross-check: the maps C -> E1 and C -> E2 send each Weierstrass
/// pair {(±r, 0)} of C to 2-torsion points T1 of E1 and T2 of E2, and
/// phi1^*(T1 - O1) and phi2^*(T2 - O2) differ by a difference of two fibres of
/// u, which is principal; so (T1, T2) lies in the kernel of
/// phi1^* + phi2^*: E1 x E2 -> Jac C, a group of order 4. The kernel is the
/// graph of psi exactly when every pair gives T2 = psi(T1).
pub fn kernel_is_graph_of_psi(h: &[u64], m1: &Map, m2: &Map, a: [u64; 3], b: [u64; 3], p: u64) -> bool {
    let g = [h[0], h[2], h[4], h[6]];
    let roots: Vec<u64> = (0..p).filter(|&x| peval(&g, x, p) == 0).collect();
    if roots.len() != 3 {
        return false;
    }
    let mut seen = vec![];
    for s in roots {
        let (Some(x1), Some(x2)) = (image_of_pair(m1, s, p), image_of_pair(m2, s, p)) else { return false };
        let Some(i) = a.iter().position(|&ai| ai == x1) else { return false };
        if b[i] != x2 || seen.contains(&i) {
            return false;
        }
        seen.push(i);
    }
    true
}

/// One elliptic curve of the census: y^2 = x^3 + a x + b over F_p, trace t.
#[derive(Clone, Copy, Debug)]
pub struct Ell {
    pub a: u64,
    pub b: u64,
    pub t: i64,
}

/// A verified gluing.
#[derive(Debug)]
pub struct Gluing {
    /// C: v^2 = h(u), ascending, length 7.
    pub h: Vec<u64>,
    pub to_e1: (Quotient, Map),
    pub to_e2: (Quotient, Map),
    /// L(Jac C), ascending.
    pub l: Vec<BigInt>,
    /// #C(F_p), #C(F_p^2).
    pub counts: [u64; 2],
    /// The 2-torsion abscissae of E1 and their images under psi.
    pub a: [u64; 3],
    pub b: [u64; 3],
}

#[derive(Debug, PartialEq, Eq)]
pub enum Glue {
    /// psi is degenerate for the formula (A1, B1, A2 or B2 vanishes).
    Degenerate,
    /// A check failed: a problem, never stored.
    Fail(String),
}

/// Glue E1 and E2 (full rational 2-torsion) along (a_i, 0) -> (b_{sigma(i)}, 0),
/// a and b the sorted roots, and run every check against the traces given.
pub fn glue(p: u64, e1: Ell, e2: Ell, sigma: [usize; 3]) -> Result<Gluing, Glue> {
    let fail = |m: String| Err(Glue::Fail(m));
    let (Some(a), Some(b0)) = (two_torsion(e1.a, e1.b, p), two_torsion(e2.a, e2.b, p)) else {
        return fail("a curve lacks full rational 2-torsion".into());
    };
    let b = [b0[sigma[0]], b0[sigma[1]], b0[sigma[2]]];
    let h = hlp_sextic(a, b, p).ok_or(Glue::Degenerate)?;
    if h[6] == 0 || !squarefree(&h, p) {
        return fail(format!("HLP sextic {h:?} over F_{p} is not a separable sextic"));
    }
    let both = [Quotient::Z, Quotient::X];
    let (mut found, mut maps_exist) = (None, false);
    'search: for (i, &q1) in both.iter().enumerate() {
        let q2 = both[1 - i];
        let c1 = maps_through(&h, q1, e1.a, e1.b, a, p);
        let c2 = maps_through(&h, q2, e2.a, e2.b, b0, p);
        for m1 in &c1 {
            for m2 in &c2 {
                maps_exist = true;
                if kernel_is_graph_of_psi(&h, m1, m2, a, b, p) {
                    found = Some(((q1, m1.clone()), (q2, m2.clone())));
                    break 'search;
                }
            }
        }
    }
    if found.is_none() && maps_exist {
        return fail(format!(
            "C: v^2 = {h:?} maps onto E1 and E2 over F_{p}, but no pair of maps sends the Weierstrass pairs of C to \
             psi-corresponding 2-torsion points (a = {a:?}, psi(a) = {b:?})"
        ));
    }
    let Some((to_e1, to_e2)) = found else {
        return fail(format!(
            "no F_p-isomorphisms of the two quotients of C: v^2 = {h:?} onto E1 = ({}, {}) and E2 = ({}, {}) over F_{p}",
            e1.a, e1.b, e2.a, e2.b
        ));
    };
    let n1 = count_hyperelliptic_fp(p, &h, 2);
    let n2 = count_hyperelliptic_fp2(p, &h, 2);
    let q = p as i128;
    let a1 = n1 as i128 - q - 1;
    let num = n2 as i128 - q * q - 1 + a1 * a1;
    if num % 2 != 0 {
        return fail(format!("#C(F_p^2) = {n2} gives a non-integral a2 over F_{p}"));
    }
    let a2 = num / 2;
    let (t1, t2) = (e1.t as i128, e2.t as i128);
    let want = [1, -(t1 + t2), 2 * q + t1 * t2];
    if [1, a1, a2] != want {
        return fail(format!(
            "L(Jac C) = 1 + {a1} T + {a2} T^2 + ... from the counts, L(E1) L(E2) gives {want:?} over F_{p} (t1 = {t1}, t2 = {t2})"
        ));
    }
    let l = [1, a1, a2, q * a1, q * q].iter().map(|&x| BigInt::from(x)).collect();
    Ok(Gluing { h, to_e1, to_e2, l, counts: [n1, n2], a, b })
}

/// The trace of y^2 = x^3 + a x + b over F_p, counted.
pub fn count_trace(a: u64, b: u64, p: u64) -> i64 {
    let f = [b % p, a % p, 0, 1];
    let n = count_hyperelliptic_fp(p, &f, 1);
    p as i64 + 1 - n as i64
}

pub const SIGMAS: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

fn poly_text(c: &[u64]) -> String {
    let t: Vec<String> = c
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, &x)| x != 0)
        .map(|(i, x)| match i {
            0 => format!("{x}"),
            1 => format!("{x}*x"),
            _ => format!("{x}*x^{i}"),
        })
        .collect();
    if t.is_empty() {
        "0".into()
    } else {
        t.join(" + ")
    }
}

// ------------------------------------------------------------------ import

const CURVES_SQL: &str = "
SELECT c.id, c.field_id, ic.p::int8, c.coefficients->>'a', c.coefficients->>'b', ic.trace::int8, coalesce(c.icv1_slug, '')
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE c.family = 'census-fp' AND c.genus = 1 AND c.model = 'short_weierstrass'
  AND ic.g = 1 AND ic.q = ic.p AND ic.p >= 5 AND ic.p < $1::int8
ORDER BY ic.p, c.id";

const CURVE_SQL: &str = "
INSERT INTO ec.curve (field_id, genus, model, form, coefficients, equation, family, is_smooth, smoothness_status,
       geometrically_irreducible, provenance_id)
SELECT r.field_id, 2, 'hyperelliptic', 'y^2+h(x)*y=f(x)', r.coefficients, r.equation, 'gluing', true, 'proved', true,
       $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, equation text)
ON CONFLICT DO NOTHING";

const MODEL_SQL: &str = "
INSERT INTO ec.hyperelliptic_model (curve_id, f, h, deg_f, deg_h, infinity_type, points_at_infinity, status, provenance_id)
SELECT c.id, r.coefficients->'f', r.coefficients->'h', 6, NULL, r.infinity_type, r.points_at_infinity, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, infinity_type text, points_at_infinity int)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
ON CONFLICT (curve_id) DO NOTHING";

const MORPHISM_SQL: &str = "
INSERT INTO ec.curve_morphism (source_curve_id, target_curve_id, kind, construction, degree, source_field_id,
       target_field_id, cover_genus, map, map_direction, field_relation, exists_over_base, minimal_degree,
       subgroup_transfer, status, method, provenance_id)
SELECT c.id, r.target_id, 'gluing_2_2', r.construction, 2, r.field_id, r.field_id, 2, r.map, 'C -> E', 'same_field',
       true, 2, 'not_tested', 'proved', r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, target_id bigint, construction text,
     map jsonb, method text)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
ON CONFLICT DO NOTHING";

const LINK_SQL: &str = "
UPDATE ec.abelian_variety av SET isogeny_class_id = ic.id
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, label text)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE av.curve_id = c.id AND av.isogeny_class_id IS NULL";

/// What ecdb already holds for the glued curves (matched by field and
/// coefficients), read before any write: `r.idx` is the curve's index.
const EXISTING_SQL: &str = "
SELECT r.idx, c.id, c.genus::int8, c.is_smooth, hm.f, hm.h, hm.deg_f::int8, hm.infinity_type,
       hm.points_at_infinity::int8, z.point_counts::text[], zic.label, aic.label
FROM jsonb_to_recordset($1::jsonb) AS r(idx int8, field_id bigint, coefficients jsonb)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
LEFT JOIN ec.hyperelliptic_model hm ON hm.curve_id = c.id
LEFT JOIN ec.curve_zeta z ON z.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq zic ON zic.id = z.isogeny_class_id
LEFT JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq aic ON aic.id = av.isogeny_class_id";

/// The stored gluing_2_2 morphisms of the glued curves.
const EXISTING_MORPH_SQL: &str = "
SELECT r.idx, m.target_curve_id, coalesce(m.construction, ''), m.map, m.degree::int8, m.status::text, c.id
FROM jsonb_to_recordset($1::jsonb) AS r(idx int8, field_id bigint, coefficients jsonb)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
JOIN ec.curve_morphism m ON m.source_curve_id = c.id AND m.kind = 'gluing_2_2'";

/// Stored isogeny classes under the labels the gluings compute.
const EXISTING_CLASS_SQL: &str = "
SELECT label, l_polynomial::text[] FROM ec.isogeny_class_fq WHERE label = ANY($1::text[])";

const ZETA_SQL: &str = "
INSERT INTO ec.curve_zeta (curve_id, isogeny_class_id, point_counts, order_check, order_method, provenance_id)
SELECT c.id, ic.id, r.point_counts, 'proved', r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, label text, point_counts numeric[],
     method text)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
ON CONFLICT (curve_id) DO NOTHING";

struct Census {
    id: i64,
    field_id: i64,
    slug: String,
    e: Ell,
}

/// What the importer would write for one glued curve.
#[derive(Clone, Debug)]
pub struct Want {
    pub f: Vec<u64>,
    pub infinity_type: &'static str,
    pub points_at_infinity: i64,
    pub counts: [u64; 2],
    pub label: String,
}

/// What ecdb already holds for that curve (None: no row or NULL).
#[derive(Clone, Debug, Default)]
pub struct Have {
    pub curve_id: i64,
    pub genus: Option<i64>,
    pub is_smooth: Option<bool>,
    pub model_f: Option<Value>,
    pub model_h: Option<Value>,
    pub deg_f: Option<i64>,
    pub infinity_type: Option<String>,
    pub points_at_infinity: Option<i64>,
    pub zeta_counts: Option<Vec<String>>,
    pub zeta_label: Option<String>,
    pub jacobian_label: Option<String>,
}

/// A stored gluing_2_2 morphism of a glued curve.
#[derive(Clone, Debug)]
pub struct HaveMorph {
    pub source_id: i64,
    pub target_id: i64,
    pub construction: String,
    pub map: Option<Value>,
    pub degree: Option<i64>,
    pub status: String,
}

fn ints(v: &Value) -> Option<Vec<BigInt>> {
    v.as_array()?
        .iter()
        .map(|x| match x {
            Value::String(s) => s.trim().parse().ok(),
            Value::Number(n) => n.to_string().parse().ok(),
            _ => None,
        })
        .collect()
}

/// Every way the stored rows of a glued curve disagree with what the gluing
/// computes. A NULL (unknown) stored value is no disagreement.
pub fn curve_conflicts(want: &Want, have: &Have) -> Vec<String> {
    let mut out = vec![];
    if have.genus.is_some_and(|g| g != 2) {
        out.push(format!("curve genus {:?} stored, 2 computed", have.genus));
    }
    if have.is_smooth == Some(false) {
        out.push("curve stored as not smooth, gcd(h, h') = 1 computed".into());
    }
    let wf: Vec<BigInt> = want.f.iter().map(|&c| BigInt::from(c)).collect();
    if let Some(f) = &have.model_f {
        if ints(f).as_ref() != Some(&wf) {
            out.push(format!("hyperelliptic_model f = {f} stored, {:?} computed", want.f));
        }
    }
    if let Some(h) = &have.model_h {
        if ints(h).is_none_or(|v| v.iter().any(|c| c != &BigInt::from(0))) {
            out.push(format!("hyperelliptic_model h = {h} stored, 0 computed"));
        }
    }
    if have.deg_f.is_some_and(|d| d != 6) {
        out.push(format!("hyperelliptic_model deg_f = {:?} stored, 6 computed", have.deg_f));
    }
    if have.infinity_type.as_deref().is_some_and(|t| t != want.infinity_type) {
        out.push(format!("infinity_type {:?} stored, {} computed", have.infinity_type, want.infinity_type));
    }
    if have.points_at_infinity.is_some_and(|n| n != want.points_at_infinity) {
        out.push(format!(
            "points_at_infinity {:?} stored, {} computed",
            have.points_at_infinity, want.points_at_infinity
        ));
    }
    if let Some(z) = &have.zeta_counts {
        let stored: Option<Vec<BigInt>> = z.iter().map(|s| s.trim().parse().ok()).collect();
        let k = z.len().min(2);
        let wc: Vec<BigInt> = want.counts[..k].iter().map(|&c| BigInt::from(c)).collect();
        if stored.map(|s| s[..k].to_vec()) != Some(wc) {
            out.push(format!("curve_zeta point_counts {z:?} stored, {:?} computed", want.counts));
        }
    }
    if have.zeta_label.as_deref().is_some_and(|l| l != want.label) {
        out.push(format!("curve_zeta class {:?} stored, {} computed", have.zeta_label, want.label));
    }
    if have.jacobian_label.as_deref().is_some_and(|l| l != want.label) {
        out.push(format!("Jacobian linked to class {:?}, {} computed", have.jacobian_label, want.label));
    }
    out
}

/// A stored morphism under the same (source, target, kind, construction) key
/// that differs from the computed one.
pub fn morphism_conflict(want_map: &Value, have: &HaveMorph) -> Option<String> {
    let mut why = vec![];
    if have.map.as_ref() != Some(want_map) {
        why.push("map");
    }
    if have.degree != Some(2) {
        why.push("degree");
    }
    if have.status != "proved" {
        why.push("status");
    }
    (!why.is_empty()).then(|| {
        format!(
            "curve {}: curve_morphism to curve {} ({}) stored with a different {}",
            have.source_id,
            have.target_id,
            have.construction,
            why.join(", ")
        )
    })
}

/// A stored class under the computed label with another L-polynomial.
pub fn class_conflict(want_l: &[BigInt], have_l: &[String]) -> bool {
    let stored: Option<Vec<BigInt>> = have_l.iter().map(|s| s.trim().parse().ok()).collect();
    stored.as_deref() != Some(want_l)
}

struct Out {
    row: Value,
    class: Value,
    want: Want,
    l: Vec<BigInt>,
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: format!("census-fp curves with full rational 2-torsion over F_p, 5 <= p < {MAX_P} (stored ec.curve)"),
        ..Default::default()
    };
    let client = ctx.client;
    let src = ctx
        .source(
            "ecdb/src/import/gluing.rs",
            &crate::math::sha256_hex(include_str!("gluing.rs")),
            "Genus-2 (2,2)-gluings of census-fp pairs, computed by ecdb (Howe–Leprévost–Poonen Prop. 4)",
        )
        .await?;
    let prov = ctx.provenance(src, METHOD, None).await?;

    let max_p = MAX_P as i64;
    // Keyed by (p, field_id): a pair is glued only within one stored field.
    let mut by_field: BTreeMap<(u64, i64), Vec<Census>> = BTreeMap::new();
    for r in client.query(CURVES_SQL, &[&max_p]).await? {
        let id: i64 = r.get(0);
        let p = r.get::<_, i64>(2) as u64;
        report.count("gluing_census_curves", 1);
        let coeff = |i: usize| r.get::<_, Option<String>>(i).and_then(|s| s.parse::<u64>().ok()).filter(|&x| x < p);
        let (Some(a), Some(b)) = (coeff(3), coeff(4)) else {
            report.count("gluing_census_refused", 1);
            report.problem(format!("curve {id}: coefficients a, b are not residues mod {p}"));
            continue;
        };
        if two_torsion(a, b, p).is_none() {
            continue;
        }
        let t: i64 = r.get(5);
        let counted = count_trace(a, b, p);
        if counted != t {
            report.count("gluing_census_refused", 1);
            report.problem(format!("curve {id}: trace {t} in ecdb, {counted} recounted over F_{p}; not glued"));
            continue;
        }
        report.count("gluing_full_two_torsion", 1);
        let field_id: i64 = r.get(1);
        by_field.entry((p, field_id)).or_default().push(Census { id, field_id, slug: r.get(6), e: Ell { a, b, t } });
    }

    // (field_id, h) -> index into `outs`; the first gluing that produces a curve names it.
    let mut index: BTreeMap<(i64, Vec<u64>), usize> = BTreeMap::new();
    let mut outs: Vec<Out> = Vec::new();
    // (curve index, target, construction) -> morphism row; the first gluing names it.
    let mut morphisms: BTreeMap<(usize, i64, &'static str), Value> = BTreeMap::new();
    for (&(p, _), list) in &by_field {
        for (i, c1) in list.iter().enumerate() {
            for c2 in &list[i..] {
                report.count("gluing_pairs", 1);
                for sigma in SIGMAS {
                    report.count("gluing_attempts", 1);
                    let g = match glue(p, c1.e, c2.e, sigma) {
                        Ok(g) => g,
                        Err(Glue::Degenerate) => {
                            report.count("gluing_degenerate", 1);
                            continue;
                        }
                        Err(Glue::Fail(m)) => {
                            report.count("gluing_crosscheck_failures", 1);
                            report.problem(format!(
                                "curves {} x {} (sigma {sigma:?}): gluing refused, nothing stored: {m}",
                                c1.id, c2.id
                            ));
                            continue;
                        }
                    };
                    report.count("gluing_computed", 1);
                    let fstr: Vec<String> = g.h.iter().map(|c| c.to_string()).collect();
                    let coefficients = json!({ "f": fstr, "h": Vec::<String>::new() });
                    let key = (c1.field_id, g.h.clone());
                    let idx = match index.get(&key) {
                        Some(&k) => k,
                        None => {
                            let c = Computed {
                                g: 2,
                                q: BigInt::from(p),
                                p: BigInt::from(p),
                                r: 1,
                                l: g.l.clone(),
                                method: Method::G2Count,
                            };
                            let (label, class) = class_row(&c, "proved")?;
                            let lead_square = sqrt_small(g.h[6], p).is_some();
                            let infinity_type = if lead_square { "split" } else { "inert" };
                            let points_at_infinity = if lead_square { 2 } else { 0 };
                            let k = outs.len();
                            outs.push(Out {
                                row: json!({
                                    "idx": k, "field_id": c1.field_id, "coefficients": coefficients,
                                    "equation": format!("y^2 = {}", poly_text(&g.h)),
                                    "infinity_type": infinity_type,
                                    "points_at_infinity": points_at_infinity,
                                    "label": label,
                                    "point_counts": [g.counts[0].to_string(), g.counts[1].to_string()],
                                    "method": ZETA_METHOD,
                                }),
                                class,
                                want: Want {
                                    f: g.h.clone(),
                                    infinity_type,
                                    points_at_infinity,
                                    counts: g.counts,
                                    label,
                                },
                                l: g.l.clone(),
                            });
                            index.insert(key, k);
                            k
                        }
                    };
                    let gluing = json!({
                        "formula": "Howe–Leprévost–Poonen 2000, Prop. 4",
                        "n": 2,
                        "e1": { "curve_id": c1.id, "slug": c1.slug, "two_torsion_x": g.a },
                        "e2": { "curve_id": c2.id, "slug": c2.slug, "two_torsion_x_psi_images": g.b },
                        "psi": "(a_i, 0) -> (b_i, 0), i = 1..3",
                        "kernel": "graph of psi in E1[2] x E2[2]: checked, each Weierstrass pair of C maps to (a_i, 0) \
                                   on E1 and (b_i, 0) on E2 under the two stored maps",
                    });
                    for (target, (qt, m)) in [(c1, &g.to_e1), (c2, &g.to_e2)] {
                        let construction = match qt {
                            Quotient::X => CONSTRUCTION_X,
                            Quotient::Z => CONSTRUCTION_Z,
                        };
                        let mkey = (idx, target.id, construction);
                        if morphisms.contains_key(&mkey) {
                            report.count("gluing_morphisms_duplicate", 1);
                            continue;
                        }
                        let mut map = m.json();
                        map["quotient"] = json!(match qt {
                            Quotient::X => "X = u^2, Y = v",
                            Quotient::Z => "Z = 1/u^2, W = v/u^3",
                        });
                        map["convention"] = json!(
                            "x = x.num(u)/x.den(u), y = v * y.v_num(u)/y.den(u) on C: v^2 = f(u), onto the target \
                             y^2 = x^3 + a x + b; ascending coefficients mod p"
                        );
                        map["gluing"] = gluing.clone();
                        morphisms.insert(
                            mkey,
                            json!({
                                "idx": idx, "field_id": c1.field_id, "coefficients": coefficients,
                                "target_id": target.id, "construction": construction, "map": map, "method": METHOD,
                            }),
                        );
                    }
                }
            }
        }
    }
    report.count("gluing_distinct_curves", outs.len() as u64);

    // Existing rows: a glued curve whose stored rows disagree with the gluing is
    // refused as a whole (a problem, nothing of it written); agreeing or NULL
    // stored values are left as they are.
    let mut refused: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let all_rows: Vec<Value> = outs.iter().map(|o| o.row.clone()).collect();
    for chunk in all_rows.chunks(2000) {
        let arr = Value::Array(chunk.to_vec());
        for r in client.query(EXISTING_SQL, &[&arr]).await? {
            let idx = r.get::<_, i64>(0) as usize;
            let have = Have {
                curve_id: r.get(1),
                genus: r.get(2),
                is_smooth: r.get(3),
                model_f: r.get(4),
                model_h: r.get(5),
                deg_f: r.get(6),
                infinity_type: r.get(7),
                points_at_infinity: r.get(8),
                zeta_counts: r.get(9),
                zeta_label: r.get(10),
                jacobian_label: r.get(11),
            };
            let why = curve_conflicts(&outs[idx].want, &have);
            if !why.is_empty() {
                refused
                    .entry(idx)
                    .or_default()
                    .extend(why.into_iter().map(|w| format!("curve {}: {w}", have.curve_id)));
            }
        }
        for r in client.query(EXISTING_MORPH_SQL, &[&arr]).await? {
            let idx = r.get::<_, i64>(0) as usize;
            let have = HaveMorph {
                source_id: r.get(6),
                target_id: r.get(1),
                construction: r.get(2),
                map: r.get(3),
                degree: r.get(4),
                status: r.get(5),
            };
            let key = match have.construction.as_str() {
                CONSTRUCTION_X => (idx, have.target_id, CONSTRUCTION_X),
                CONSTRUCTION_Z => (idx, have.target_id, CONSTRUCTION_Z),
                _ => continue,
            };
            if let Some(want) = morphisms.get(&key) {
                if let Some(why) = morphism_conflict(&want["map"], &have) {
                    refused.entry(idx).or_default().push(why);
                }
            }
        }
    }
    let labels: Vec<String> =
        outs.iter().map(|o| o.want.label.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
    let mut bad_labels = std::collections::BTreeSet::new();
    for r in client.query(EXISTING_CLASS_SQL, &[&labels]).await? {
        let label: String = r.get(0);
        let stored: Vec<String> = r.get(1);
        if let Some(o) = outs.iter().find(|o| o.want.label == label) {
            if class_conflict(&o.l, &stored) {
                report.count("gluing_class_conflicts", 1);
                report.problem(format!(
                    "isogeny_class_fq {label}: stored L = {stored:?}, the gluings compute {:?}; left unchanged",
                    o.l.iter().map(|x| x.to_string()).collect::<Vec<_>>()
                ));
                bad_labels.insert(label);
            }
        }
    }
    for (k, o) in outs.iter().enumerate() {
        if bad_labels.contains(&o.want.label) {
            refused.entry(k).or_default().push(format!("class {} conflicts with the stored row", o.want.label));
        }
    }
    for (k, why) in &refused {
        report.count("gluing_existing_conflicts", 1);
        report.problem(format!(
            "glued curve y^2 = {}: stored rows disagree, nothing of it written: {}",
            poly_text(&outs[*k].want.f),
            why.join("; ")
        ));
    }

    let mut rows = Vec::new();
    let mut uniq: BTreeMap<String, Value> = BTreeMap::new();
    for (k, o) in outs.into_iter().enumerate() {
        if refused.contains_key(&k) {
            continue;
        }
        uniq.entry(o.want.label.clone()).or_insert(o.class);
        rows.push(o.row);
    }
    let morphisms: Vec<Value> =
        morphisms.into_iter().filter(|((k, _, _), _)| !refused.contains_key(k)).map(|(_, v)| v).collect();
    let classes: Vec<Value> = uniq.into_values().collect();
    report.count("gluing_curves_inserted", bulk(client, CURVE_SQL, &rows, prov).await?);
    report.count("gluing_models_inserted", bulk(client, MODEL_SQL, &rows, prov).await?);
    report.count("gluing_morphisms_inserted", bulk(client, MORPHISM_SQL, &morphisms, prov).await?);
    report.count("gluing_classes_inserted", bulk(client, CLASS_SQL, &classes, prov).await?);
    report.count("gluing_jacobians_linked", bulk1(client, LINK_SQL, &rows).await?);
    report.count("gluing_zeta_inserted", bulk(client, ZETA_SQL, &rows, prov).await?);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// #E(F_p) of y^2 = x^3 + a x + b by listing pairs.
    fn brute_ec(p: u64, a: u64, b: u64) -> i64 {
        let mut n = 1;
        for x in 0..p {
            for y in 0..p {
                if mul(y, y, p) == peval(&[b, a, 0, 1], x, p) {
                    n += 1;
                }
            }
        }
        p as i64 + 1 - n
    }

    /// #C(F_p) of v^2 = h(u), h a sextic, by listing pairs plus the points at infinity.
    fn brute_c(p: u64, h: &[u64]) -> u64 {
        let mut n = if sqrt_small(h[6], p).is_some() { 2 } else { 0 };
        for u in 0..p {
            let hu = peval(h, u, p);
            n += (0..p).filter(|&v| mul(v, v, p) == hu).count() as u64;
        }
        n
    }

    /// #C(F_{p^2}) by listing (u, v) in F_p[s]/(s^2 - nr), plus 2 at infinity
    /// (every element of F_p is a square in F_{p^2}).
    fn brute_c2(p: u64, h: &[u64]) -> u64 {
        let nr = (2..p).find(|&x| sqrt_small(x, p).is_none()).unwrap();
        let m2 = |a: (u64, u64), b: (u64, u64)| ((a.0 * b.0 + nr * (a.1 * b.1 % p)) % p, (a.0 * b.1 + a.1 * b.0) % p);
        let ev = |u: (u64, u64)| {
            h.iter().rev().fold((0, 0), |acc, &c| {
                let m = m2(acc, u);
                ((m.0 + c) % p, m.1)
            })
        };
        let mut squares: BTreeMap<(u64, u64), u64> = BTreeMap::new();
        for vx in 0..p {
            for vy in 0..p {
                *squares.entry(m2((vx, vy), (vx, vy))).or_default() += 1;
            }
        }
        let mut n = 2;
        for ux in 0..p {
            for uy in 0..p {
                n += squares.get(&ev((ux, uy))).copied().unwrap_or(0);
            }
        }
        n
    }

    /// Every curve y^2 = x^3 + a x + b over F_p with full 2-torsion (not up to isomorphism).
    fn full_two_torsion(p: u64) -> Vec<Ell> {
        let mut out = vec![];
        for a in 0..p {
            for b in 0..p {
                let disc = (4 * pw(a, 3, p) + 27 * mul(b, b, p)) % p;
                if disc != 0 && two_torsion(a, b, p).is_some() {
                    out.push(Ell { a, b, t: brute_ec(p, a, b) });
                }
            }
        }
        out
    }

    /// Brute force on tiny fields: for every gluing over F_7 and F_11 (and a
    /// sample over F_13) the counts of C match listing the pairs, L(C) =
    /// L(E1) L(E2), and the maps are morphisms.
    #[test]
    fn tiny_fields_agree_with_brute_force() {
        for p in [7u64, 11, 13] {
            let curves = full_two_torsion(p);
            let (mut ok, mut degenerate) = (0, 0);
            for (i, e1) in curves.iter().enumerate().step_by(if p == 13 { 3 } else { 1 }) {
                for e2 in curves[i..].iter().step_by(if p == 13 { 4 } else { 2 }) {
                    for sigma in SIGMAS {
                        match glue(p, *e1, *e2, sigma) {
                            Ok(g) => {
                                assert_eq!(g.counts[0], brute_c(p, &g.h));
                                if p <= 11 {
                                    assert_eq!(g.counts[1], brute_c2(p, &g.h));
                                }
                                let q = p as i64;
                                let l: Vec<i64> = g.l.iter().map(|x| x.try_into().unwrap()).collect();
                                assert_eq!(l, vec![1, -(e1.t + e2.t), 2 * q + e1.t * e2.t, -q * (e1.t + e2.t), q * q]);
                                assert!(map_is_morphism(&g.h, &g.to_e1.1, e1.a, e1.b, p));
                                assert!(map_is_morphism(&g.h, &g.to_e2.1, e2.a, e2.b, p));
                                assert!(squarefree(&g.h, p));
                                assert!(kernel_is_graph_of_psi(&g.h, &g.to_e1.1, &g.to_e2.1, g.a, g.b, p));
                                ok += 1;
                            }
                            Err(Glue::Degenerate) => degenerate += 1,
                            Err(Glue::Fail(m)) => panic!("F_{p}: {m}"),
                        }
                    }
                }
            }
            assert!(ok > 0 && degenerate > 0, "p = {p}: {ok} gluings, {degenerate} degenerate");
        }
    }

    /// Pinned against the literature: E: y^2 = x^3 - x has trace 0 over F_p,
    /// p = 3 mod 4 (supersingular), and trace 2a with p = a^2 + b^2, a + bi = 1
    /// mod (2 + 2i), for p = 1 mod 4 (Ireland–Rosen, Ch. 18, Thm. 5, up to the
    /// sign convention, which the count fixes): over F_13 = 3^2 + 2^2, |t| = 6.
    /// The gluing of E with itself along a non-identity psi has L = L_E^2.
    #[test]
    fn pinned_j1728_self_gluing() {
        for (p, t) in [(7u64, 0i64), (11, 0), (19, 0), (13, 6)] {
            let e = Ell { a: p - 1, b: 0, t };
            assert_eq!(count_trace(p - 1, 0, p), t, "trace of y^2 = x^3 - x over F_{p}");
            assert_eq!(two_torsion(p - 1, 0, p), Some([0, 1, p - 1]));
            // sigma = identity is the restriction of the identity: degenerate.
            assert_eq!(glue(p, e, e, [0, 1, 2]).unwrap_err(), Glue::Degenerate);
            let g = glue(p, e, e, [1, 0, 2]).unwrap();
            let q = p as i64;
            let l: Vec<i64> = g.l.iter().map(|x| x.try_into().unwrap()).collect();
            assert_eq!(l, vec![1, -2 * t, 2 * q + t * t, -2 * q * t, q * q]);
        }
    }

    /// Pinned against an independent Python reference (the HLP formula typed
    /// from the paper and exhaustive counts over F_p and F_p^2, not committed):
    /// E1: y^2 = x^3 + 16 x + 6 (roots 1, 2, 20) and E2: y^2 = x^3 - x over
    /// F_23, psi = (1, 2, 20) -> (0, 1, 22).
    #[test]
    fn pinned_reference_rows() {
        let p = 23u64;
        // E1 with roots 1, 2, 20 (sum 0): x^3 + a x + b with a = 1*2 + 2*20 + 20*1, b = -1*2*20.
        let (a1, b1) = ((2 + 40 + 20) % p, md(-40, p));
        let (a2, b2) = (md(-1, p), 0); // y^2 = x^3 - x, roots 0, 1, 22
        let e1 = Ell { a: a1, b: b1, t: count_trace(a1, b1, p) };
        let e2 = Ell { a: a2, b: b2, t: count_trace(a2, b2, p) };
        assert_eq!(two_torsion(a1, b1, p), Some([1, 2, 20]));
        let g = glue(p, e1, e2, [0, 1, 2]).unwrap();
        assert_eq!(g.h, REF_H.to_vec());
        let l: Vec<i64> = g.l.iter().map(|x| x.try_into().unwrap()).collect();
        assert_eq!(l, REF_L.to_vec());
        assert_eq!(g.counts, REF_COUNTS);
        assert_eq!(e1.t, REF_T1);
    }

    const REF_H: [u64; 7] = [18, 0, 14, 0, 2, 0, 12];
    const REF_L: [i64; 5] = [1, 4, 46, 92, 529];
    const REF_COUNTS: [u64; 2] = [28, 606];
    const REF_T1: i64 = -4;

    /// A corrupted trace (as if ecdb held a wrong one) is a cross-check
    /// failure, never a result; a corrupted sextic coefficient breaks the maps.
    #[test]
    fn corrupted_inputs_are_refused() {
        let p = 23u64;
        let e = Ell { a: p - 1, b: 0, t: count_trace(p - 1, 0, p) };
        assert!(glue(p, e, e, [1, 0, 2]).is_ok());
        let bad = Ell { t: e.t + 2, ..e };
        assert!(matches!(glue(p, bad, e, [1, 0, 2]), Err(Glue::Fail(_))));
        let g = glue(p, e, e, [1, 0, 2]).unwrap();
        let mut h = g.h.clone();
        h[2] = (h[2] + 1) % p;
        assert!(!map_is_morphism(&h, &g.to_e1.1, e.a, e.b, p));
        assert!(!map_is_morphism(&h, &g.to_e2.1, e.a, e.b, p));
    }

    /// The import's whole range without a database: one curve per
    /// F_p-isomorphism class ((a, b) ~ (u^4 a, u^6 b)) with full 2-torsion, every
    /// pair and every psi, 5 <= p < MAX_P. Every non-degenerate gluing passes.
    #[test]
    #[ignore]
    fn census_range_sweep() {
        let (mut ok, mut degenerate) = (0u64, 0u64);
        for p in (5..MAX_P).filter(|&n| (2..n).all(|d| n % d != 0)) {
            let mut seen = std::collections::BTreeSet::new();
            let mut reps = vec![];
            for e in full_two_torsion(p) {
                if seen.contains(&(e.a, e.b)) {
                    continue;
                }
                for u in 1..p {
                    seen.insert((mul(pw(u, 4, p), e.a, p), mul(pw(u, 6, p), e.b, p)));
                }
                reps.push(e);
            }
            for (i, e1) in reps.iter().enumerate() {
                for e2 in &reps[i..] {
                    for sigma in SIGMAS {
                        match glue(p, *e1, *e2, sigma) {
                            Ok(_) => ok += 1,
                            Err(Glue::Degenerate) => degenerate += 1,
                            Err(Glue::Fail(m)) => panic!("F_{p}: {m}"),
                        }
                    }
                }
            }
        }
        eprintln!("census range: {ok} gluings, {degenerate} degenerate");
    }

    /// Regression (review, 2026-11-14): the stored maps must realize the stored
    /// psi. For j = 1728 the automorphism [i] permutes E[2], so the first
    /// isomorphism found for each quotient can send the Weierstrass pairs of C
    /// to 2-torsion points that do not correspond under psi; the first version
    /// stored such maps next to a psi they did not realize. Over F_5,
    /// E: y^2 = x^3 + x glued to itself along sigma = [2, 0, 1] is one case.
    #[test]
    fn stored_maps_realize_psi() {
        let p = 5u64;
        let e = Ell { a: 1, b: 0, t: count_trace(1, 0, p) };
        let sigma = [2, 0, 1];
        let a = two_torsion(e.a, e.b, p).unwrap();
        let b = [a[sigma[0]], a[sigma[1]], a[sigma[2]]];
        let h = hlp_sextic(a, b, p).unwrap();
        // Some pair of genuine morphisms C -> E, C -> E does not realize psi ...
        let mut wrong = 0;
        for (q1, q2) in [(Quotient::Z, Quotient::X), (Quotient::X, Quotient::Z)] {
            for m1 in maps_through(&h, q1, e.a, e.b, a, p) {
                for m2 in maps_through(&h, q2, e.a, e.b, a, p) {
                    assert!(map_is_morphism(&h, &m1, e.a, e.b, p) && map_is_morphism(&h, &m2, e.a, e.b, p));
                    if !kernel_is_graph_of_psi(&h, &m1, &m2, a, b, p) {
                        wrong += 1;
                    }
                }
            }
        }
        assert!(wrong > 0, "the j = 1728 case should offer maps that do not realize psi");
        // ... and glue keeps only a pair that does.
        let g = glue(p, e, e, sigma).unwrap();
        assert!(kernel_is_graph_of_psi(&g.h, &g.to_e1.1, &g.to_e2.1, g.a, g.b, p));
        // A psi permuted after the fact is not realized by the stored maps.
        let b_bad = [g.b[1], g.b[0], g.b[2]];
        assert!(!kernel_is_graph_of_psi(&g.h, &g.to_e1.1, &g.to_e2.1, g.a, b_bad, p));
    }

    fn want() -> Want {
        Want {
            f: vec![18, 0, 14, 0, 2, 0, 12],
            infinity_type: "inert",
            points_at_infinity: 0,
            counts: [28, 606],
            label: "2.23.e_bu".into(),
        }
    }

    fn agreeing() -> Have {
        Have {
            curve_id: 1,
            genus: Some(2),
            is_smooth: Some(true),
            model_f: Some(json!(["18", "0", "14", "0", "2", "0", "12"])),
            model_h: Some(json!([])),
            deg_f: Some(6),
            infinity_type: Some("inert".into()),
            points_at_infinity: Some(0),
            zeta_counts: Some(vec!["28".into(), "606".into(), "12000".into()]),
            zeta_label: Some("2.23.e_bu".into()),
            jacobian_label: Some("2.23.e_bu".into()),
        }
    }

    /// Existing rows: agreement (numbers or strings, a longer zeta prefix) and
    /// NULLs pass; every disagreeing stored value is reported, so the curve is
    /// refused rather than written next to a contradiction or skipped silently.
    #[test]
    fn existing_rows_agree_or_are_refused() {
        let w = want();
        assert!(curve_conflicts(&w, &agreeing()).is_empty());
        assert!(curve_conflicts(&w, &Have { curve_id: 1, ..Default::default() }).is_empty());
        let numeric = Have { model_f: Some(json!([18, 0, 14, 0, 2, 0, 12])), ..agreeing() };
        assert!(curve_conflicts(&w, &numeric).is_empty());
        let corrupt: Vec<Have> = vec![
            Have { genus: Some(3), ..agreeing() },
            Have { is_smooth: Some(false), ..agreeing() },
            Have { model_f: Some(json!(["18", "0", "14", "0", "2", "0", "11"])), ..agreeing() },
            Have { model_h: Some(json!(["1"])), ..agreeing() },
            Have { deg_f: Some(5), ..agreeing() },
            Have { infinity_type: Some("split".into()), ..agreeing() },
            Have { points_at_infinity: Some(2), ..agreeing() },
            Have { zeta_counts: Some(vec!["29".into()]), ..agreeing() },
            Have { zeta_counts: Some(vec!["28".into(), "607".into()]), ..agreeing() },
            Have { zeta_label: Some("2.23.a_a".into()), ..agreeing() },
            Have { jacobian_label: Some("2.23.a_a".into()), ..agreeing() },
        ];
        for h in &corrupt {
            assert_eq!(curve_conflicts(&w, h).len(), 1, "{h:?}");
        }

        let map = json!({ "x": { "num": ["1", "0", "3"], "den": ["1"] } });
        let stored = HaveMorph {
            source_id: 1,
            target_id: 7,
            construction: CONSTRUCTION_X.into(),
            map: Some(map.clone()),
            degree: Some(2),
            status: "proved".into(),
        };
        assert!(morphism_conflict(&map, &stored).is_none());
        assert!(morphism_conflict(&json!({ "x": { "num": ["2", "0", "3"], "den": ["1"] } }), &stored).is_some());
        assert!(morphism_conflict(&map, &HaveMorph { degree: Some(4), ..stored.clone() }).is_some());
        assert!(morphism_conflict(&map, &HaveMorph { status: "unknown".into(), ..stored.clone() }).is_some());
        assert!(morphism_conflict(&map, &HaveMorph { map: None, ..stored }).is_some());

        let l: Vec<BigInt> = REF_L.iter().map(|&x| BigInt::from(x)).collect();
        assert!(!class_conflict(&l, &["1", "4", "46", "92", "529"].map(String::from)));
        assert!(class_conflict(&l, &["1", "4", "46", "92", "528"].map(String::from)));
    }

    #[test]
    fn squarefree_detects_repeated_roots() {
        let p = 13;
        assert!(squarefree(&[2, 0, 1], p));
        assert!(!squarefree(&pmul(&[1, 1], &pmul(&[1, 1], &[3, 0, 1], p), p), p));
    }
}
