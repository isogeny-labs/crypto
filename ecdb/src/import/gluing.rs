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
//! A failure is a problem line (`gluing_crosscheck_failures`) and nothing of
//! that gluing is stored. The isogeny and its kernel are the theorem's; ecdb
//! verifies the isogeny class (Tate: equal L-polynomials) and the two maps, not
//! the kernel.
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
//! and `curve_zeta`. A Jacobian already linked to another class is a problem,
//! never overwritten.

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
                      (recounted). The kernel of the isogeny is the theorem's and is not re-verified";

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

/// An F_p-isomorphism of the quotient `qt` of C (even sextic `h`) onto
/// E: y^2 = x^3 + a x + b with roots `e`, composed into a map C -> E.
fn map_through(h: &[u64], qt: Quotient, a: u64, b: u64, e: [u64; 3], p: u64) -> Option<Map> {
    let g = match qt {
        Quotient::X => [h[0], h[2], h[4], h[6]],
        Quotient::Z => [h[6], h[4], h[2], h[0]],
    };
    if g[3] == 0 {
        return None;
    }
    let roots: Vec<u64> = (0..p).filter(|&x| peval(&g, x, p) == 0).collect();
    if roots.len() != 3 {
        return None;
    }
    const PERMS: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
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
        if map_is_morphism(h, &m, a, b, p) {
            return Some(m);
        }
    }
    None
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
    let mut found = None;
    for (i, &q1) in both.iter().enumerate() {
        let q2 = both[1 - i];
        if let (Some(m1), Some(m2)) = (map_through(&h, q1, e1.a, e1.b, a, p), map_through(&h, q2, e2.a, e2.b, b0, p)) {
            found = Some(((q1, m1), (q2, m2)));
            break;
        }
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

/// Glued curves whose Jacobian is linked to a class other than the computed one.
const CONFLICT_SQL: &str = "
SELECT c.id, ic.label, r.label
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, label text)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE ic.label <> r.label";

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
    let mut by_p: BTreeMap<u64, Vec<Census>> = BTreeMap::new();
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
        by_p.entry(p).or_default().push(Census { id, field_id: r.get(1), slug: r.get(6), e: Ell { a, b, t } });
    }

    // (field_id, h) -> rows; the first gluing that produces a curve names it.
    let mut curves: BTreeMap<(i64, Vec<u64>), (Value, Value)> = BTreeMap::new();
    let mut morphisms = Vec::new();
    for (&p, list) in &by_p {
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
                    let c = Computed {
                        g: 2,
                        q: BigInt::from(p),
                        p: BigInt::from(p),
                        r: 1,
                        l: g.l.clone(),
                        method: Method::G2Count,
                    };
                    let (label, class) = class_row(&c, "proved")?;
                    let key = (c1.field_id, g.h.clone());
                    let lead_square = sqrt_small(g.h[6], p).is_some();
                    curves.entry(key).or_insert_with(|| {
                        (
                            json!({
                                "field_id": c1.field_id, "coefficients": coefficients,
                                "equation": format!("y^2 = {}", poly_text(&g.h)),
                                "infinity_type": if lead_square { "split" } else { "inert" },
                                "points_at_infinity": if lead_square { 2 } else { 0 },
                                "label": label,
                                "point_counts": [g.counts[0].to_string(), g.counts[1].to_string()],
                                "method": ZETA_METHOD,
                            }),
                            class,
                        )
                    });
                    let gluing = json!({
                        "formula": "Howe–Leprévost–Poonen 2000, Prop. 4",
                        "n": 2,
                        "e1": { "curve_id": c1.id, "slug": c1.slug, "two_torsion_x": g.a },
                        "e2": { "curve_id": c2.id, "slug": c2.slug, "two_torsion_x_psi_images": g.b },
                        "psi": "(a_i, 0) -> (b_i, 0), i = 1..3",
                        "kernel": "graph of psi in E1[2] x E2[2] (the theorem's; not re-verified)",
                    });
                    for (target, (qt, m)) in [(c1, &g.to_e1), (c2, &g.to_e2)] {
                        let construction = match qt {
                            Quotient::X => CONSTRUCTION_X,
                            Quotient::Z => CONSTRUCTION_Z,
                        };
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
                        morphisms.push(json!({
                            "field_id": c1.field_id, "coefficients": coefficients, "target_id": target.id,
                            "construction": construction, "map": map, "method": METHOD,
                        }));
                    }
                }
            }
        }
    }
    report.count("gluing_distinct_curves", curves.len() as u64);
    let (rows, classes): (Vec<Value>, Vec<Value>) = curves.into_values().unzip();
    let mut uniq: BTreeMap<String, Value> = BTreeMap::new();
    for c in classes {
        uniq.entry(c["label"].as_str().unwrap_or_default().to_string()).or_insert(c);
    }
    let classes: Vec<Value> = uniq.into_values().collect();
    report.count("gluing_curves_inserted", bulk(client, CURVE_SQL, &rows, prov).await?);
    report.count("gluing_models_inserted", bulk(client, MODEL_SQL, &rows, prov).await?);
    report.count("gluing_morphisms_inserted", bulk(client, MORPHISM_SQL, &morphisms, prov).await?);
    report.count("gluing_classes_inserted", bulk(client, CLASS_SQL, &classes, prov).await?);
    report.count("gluing_jacobians_linked", bulk1(client, LINK_SQL, &rows).await?);
    let arr = Value::Array(rows.clone());
    for r in client.query(CONFLICT_SQL, &[&arr]).await? {
        report.count("gluing_link_conflicts", 1);
        report.problem(format!(
            "curve {}: Jacobian already linked to class {}, the gluing computes {}; left unchanged",
            r.get::<_, i64>(0),
            r.get::<_, String>(1),
            r.get::<_, String>(2)
        ));
    }
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

    #[test]
    fn squarefree_detects_repeated_roots() {
        let p = 13;
        assert!(squarefree(&[2, 0, 1], p));
        assert!(!squarefree(&pmul(&[1, 1], &pmul(&[1, 1], &[3, 0, 1], p), p), p));
    }
}
