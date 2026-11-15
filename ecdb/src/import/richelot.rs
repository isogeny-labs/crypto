//! Richelot (2,2)-isogenies out of the genus-2 cover sources over small
//! prime fields.
//!
//! # Mathematics
//!
//! Let C: y^2 = f(x) be a genus-2 curve over F_p, p odd, with f squarefree of
//! degree 5 or 6. A *quadratic splitting* is a factorisation f = G1 G2 G3 into
//! three polynomials of degree <= 2 over F_p (degree 1 for the factor that
//! carries the root at infinity of a quintic). The divisor classes of the zeros
//! of G1, G2, G3 are the three non-zero points of a maximal isotropic subgroup
//! K of Jac(C)[2] defined over F_p, and Jac(C) -> Jac(C)/K is a (2,2)-isogeny
//! defined over F_p. With G_i = g_i2 x^2 + g_i1 x + g_i0 and
//! Delta = det (g_ij) (rows G1, G2, G3; columns g_i0, g_i1, g_i2):
//!
//! * Delta != 0: Jac(C)/K is the Jacobian of the genus-2 curve
//!   C': y^2 = Delta^-1 [G2,G3] [G3,G1] [G1,G2], [Gi,Gj] = Gi' Gj - Gi Gj'
//!   (Richelot; Bost–Mestre; B. Smith, thesis, Ch. 8). The result does not
//!   depend on the order of the G_i or on which of them carries the leading
//!   coefficient of f.
//! * Delta = 0: Jac(C)/K is a product of two elliptic curves with the product
//!   polarisation, i.e. Jac(C) is split. Nothing is stored for it here; it is
//!   counted (`richelot_split_jacobians`). Every cover source
//!   v^2 = G(u^2) has the splitting G_i = u^2 - r_i (roots r_i of G when G
//!   splits), whose Delta is 0: it is the known decomposition Jac(H) ~ E x E'.
//!
//! The quadratic splittings are found from the factorisation of f over F_p:
//! the roots in F_p (and infinity for a quintic) and the irreducible quadratic
//! factors, by distinct- and equal-degree factorisation (Cantor–Zassenhaus with
//! a deterministic sequence of test polynomials). An irreducible quadratic is
//! a factor of every splitting; the rational roots are paired in every way
//! (1, 3 or 15 matchings). A factor of degree >= 3 leaves no splitting.
//!
//! # Cross-checks (all must pass, or nothing is stored)
//!
//! * the image f' has degree 5 or 6 and is squarefree (C' is a smooth genus-2
//!   curve);
//! * isogenous Jacobians have equal L-polynomials: #C'(F_p) and
//!   #C'(F_{p^2}) of the image, counted exhaustively (so only for
//!   p < [`MAX_P`] = 2^14), equal those of the source. The F_p count detects
//!   the wrong quadratic twist (the sign convention of Delta when p = 3 mod 4),
//!   the F_{p^2} count the rest of L;
//! * the source's counts: for p < 2^12 (`cover_lpoly::DIRECT_FP2_MAX`) both
//!   are counted exhaustively and must equal the `curve_zeta` row that
//!   `cover_lpoly` stored, when there is one; for 2^12 <= p < 2^14 that row is
//!   required (there `cover_lpoly` derived it from the group orders of the two
//!   elliptic quotients, an independent method) and #C(F_p) is recounted
//!   exhaustively against it;
//! * after the insert, every Richelot isogeny joins two Jacobians of the same
//!   `isogeny_class_fq` (a mismatch is a problem).
//!
//! # Rows
//!
//! For each verified image: an `ec.curve` (family `richelot`, model
//! `hyperelliptic`, coefficients `{"f": [...], "h": []}`), its
//! `hyperelliptic_model`, its Jacobian (by the `ensure_jacobian` trigger)
//! linked to the source's isogeny class, a `curve_zeta` row with the two
//! counts, and an `ec.isogeny` (dimension 2, degree 4, ell 2, kernel {2,2},
//! formula `richelot`) from Jac(C) to Jac(C'), with the kernel as three
//! Mumford divisors (u = monic G_i, v = 0) and Delta, [G2,G3], [G3,G1],
//! [G1,G2] in `codomain_raw`. `external_id` is `richelot1_` and 16 hex digits
//! of the SHA-256 of (p, f, kernel), so a re-import inserts nothing. Status
//! `proved` when the characteristic is proved prime (always, for p < 2^14).
//! Insert-only (plus filling a NULL Jacobian class link): nothing is
//! overwritten or deleted. An image curve already stored must agree exactly
//! (model f, `curve_zeta` counts, isogeny class); otherwise it is reported
//! (`richelot_existing_conflicts`), left unchanged, and no isogeny is attached
//! to it. After the insert every isogeny sent must be stored with exactly its
//! source, target, kernel and formula; a row with the same `external_id` and
//! other content is reported and left unchanged. A composite characteristic is
//! skipped (`richelot_sources_skipped_composite_characteristic`).
//!
//! # Report
//!
//! `richelot_sources`, `richelot_sources_skipped_*`, `richelot_no_splitting`,
//! `richelot_splittings`, `richelot_split_jacobians`, `richelot_images_verified`,
//! `richelot_crosscheck_failures` (each also a problem), `richelot_curves`,
//! `richelot_models`, `richelot_classes`, `richelot_jacobians_linked`,
//! `richelot_zeta`, `richelot_isogenies`, `richelot_existing_conflicts`,
//! `richelot_class_mismatches`.

use super::cover_lpoly::{
    class_row, count_hyperelliptic_fp, count_hyperelliptic_fp2, Computed, Method, DIRECT_FP2_MAX,
};
use super::covers::poly_text;
use super::{bulk, bulk1, Ctx, Report};
use crate::ec::is_probable_prime;
use crate::math::sha256_hex;
use anyhow::Result;
use num_bigint::BigInt;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Largest p (exclusive) whose images are counted exhaustively over F_{p^2}.
pub const MAX_P: u64 = 1 << 14;

// ------------------------------------------------------- F_p[x], p < 2^31

/// A polynomial over F_p, ascending coefficients, no trailing zeros (the zero
/// polynomial is empty).
pub type Poly = Vec<u64>;

fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn deg(a: &[u64]) -> isize {
    a.len() as isize - 1
}

fn pw(p: u64, mut b: u64, mut e: u64) -> u64 {
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

fn inv(p: u64, a: u64) -> u64 {
    pw(p, a, p - 2)
}

fn sub(p: u64, a: &[u64], b: &[u64]) -> Poly {
    let n = a.len().max(b.len());
    trim((0..n).map(|i| (a.get(i).copied().unwrap_or(0) + p - b.get(i).copied().unwrap_or(0)) % p).collect())
}

fn mul(p: u64, a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y) % p;
        }
    }
    trim(r)
}

fn scale(p: u64, a: &[u64], c: u64) -> Poly {
    trim(a.iter().map(|&x| x * c % p).collect())
}

/// (quotient, remainder) of a by a non-zero b.
fn divrem(p: u64, a: &[u64], b: &[u64]) -> (Poly, Poly) {
    let mut r = trim(a.to_vec());
    let db = b.len() - 1;
    let li = inv(p, b[db]);
    if r.len() < b.len() {
        return (vec![], r);
    }
    let mut q = vec![0u64; r.len() - db];
    while r.len() > db && !r.is_empty() {
        let k = r.len() - 1 - db;
        let c = r[r.len() - 1] * li % p;
        q[k] = c;
        for (j, &bj) in b.iter().enumerate() {
            r[k + j] = (r[k + j] + p - c * bj % p) % p;
        }
        r = trim(r);
    }
    (trim(q), r)
}

fn monic(p: u64, a: &[u64]) -> Poly {
    match a.last() {
        None => vec![],
        Some(&l) => scale(p, a, inv(p, l)),
    }
}

/// The monic gcd.
fn gcd(p: u64, a: &[u64], b: &[u64]) -> Poly {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !b.is_empty() {
        let r = divrem(p, &a, &b).1;
        a = b;
        b = r;
    }
    monic(p, &a)
}

fn powmod(p: u64, base: &[u64], mut e: u64, m: &[u64]) -> Poly {
    let mut r = divrem(p, &[1], m).1;
    let mut b = divrem(p, base, m).1;
    while e > 0 {
        if e & 1 == 1 {
            r = divrem(p, &mul(p, &r, &b), m).1;
        }
        b = divrem(p, &mul(p, &b, &b), m).1;
        e >>= 1;
    }
    r
}

fn derivative(p: u64, a: &[u64]) -> Poly {
    trim(a.iter().enumerate().skip(1).map(|(i, &c)| (i as u64 % p) * c % p).collect())
}

/// f is squarefree: gcd(f, f') = 1 (p > deg f, so f' != 0 for deg f >= 1).
pub fn squarefree(p: u64, f: &[u64]) -> bool {
    let f = trim(f.to_vec());
    !f.is_empty() && deg(&gcd(p, &f, &derivative(p, &f))) == 0
}

fn eval(p: u64, a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &c| (acc * x + c) % p)
}

/// Split a monic squarefree h, all of whose irreducible factors have degree
/// d, into those factors (equal-degree factorisation with the test
/// polynomials x + a, then x^2 + a x + b). `false` when no test polynomial
/// separated two factors (never for odd p, by the Chinese remainder theorem
/// over the test polynomials of degree < 2d).
fn edf(p: u64, h: Poly, d: usize, out: &mut Vec<Poly>) -> bool {
    let n = h.len() - 1;
    if n == d {
        out.push(h);
        return true;
    }
    if n == 0 {
        return true;
    }
    let e = (pw_u128(p, d) - 1) / 2;
    let one: Poly = vec![1];
    let tests = (0..p).map(|a| vec![a, 1]).chain((0..p).flat_map(|a| (0..p).map(move |b| vec![b, a, 1])));
    for t in tests {
        let w = sub(p, &powmod(p, &t, e as u64, &h), &one);
        let g = gcd(p, &h, &w);
        let dg = g.len().saturating_sub(1);
        if dg > 0 && dg < n {
            let other = divrem(p, &h, &g).0;
            return edf(p, g, d, out) && edf(p, monic(p, &other), d, out);
        }
    }
    false
}

fn pw_u128(p: u64, d: usize) -> u128 {
    (0..d).fold(1u128, |acc, _| acc * p as u128)
}

/// The factorisation of a squarefree f over F_p into its rational roots and
/// irreducible monic quadratics; `None` when f has an irreducible factor of
/// degree >= 3, `Err` when the splitting failed (a problem).
#[allow(clippy::type_complexity)]
pub fn small_factors(p: u64, f: &[u64]) -> Result<Option<(Vec<u64>, Vec<Poly>)>, String> {
    let fm = monic(p, f);
    let n = fm.len() - 1;
    let x: Poly = vec![0, 1];
    let xp = powmod(p, &x, p, &fm);
    let lin = gcd(p, &fm, &sub(p, &xp, &x));
    let rest = monic(p, &divrem(p, &fm, &lin).0);
    let quad = if rest.len() > 1 {
        let xp2 = powmod(p, &powmod(p, &x, p, &rest), 1, &rest);
        let xp2 = powmod(p, &xp2, p, &rest);
        gcd(p, &rest, &sub(p, &xp2, &x))
    } else {
        vec![1]
    };
    if (lin.len() - 1) + (quad.len() - 1) != n {
        return Ok(None);
    }
    let mut linear = vec![];
    if !edf(p, lin, 1, &mut linear) {
        return Err("equal-degree factorisation of the linear part failed".into());
    }
    let mut quads = vec![];
    if !edf(p, quad, 2, &mut quads) {
        return Err("equal-degree factorisation of the quadratic part failed".into());
    }
    let mut roots: Vec<u64> = linear.iter().map(|l| (p - l[0]) % p).collect();
    roots.sort_unstable();
    quads.sort();
    for r in &roots {
        if eval(p, f, *r) != 0 {
            return Err(format!("{r} is not a root"));
        }
    }
    for q in &quads {
        if q.len() != 3 || !divrem(p, f, q).1.is_empty() {
            return Err(format!("{q:?} is not a quadratic factor"));
        }
    }
    Ok(Some((roots, quads)))
}

/// Every perfect matching of `pts` (even length), as lists of pairs.
fn matchings<T: Copy>(pts: &[T]) -> Vec<Vec<(T, T)>> {
    if pts.is_empty() {
        return vec![vec![]];
    }
    let mut out = vec![];
    let first = pts[0];
    for i in 1..pts.len() {
        let rest: Vec<T> = pts[1..].iter().enumerate().filter(|(j, _)| *j + 1 != i).map(|(_, &x)| x).collect();
        for mut m in matchings(&rest) {
            m.insert(0, (first, pts[i]));
            out.push(m);
        }
    }
    out
}

/// A factor of degree <= 2 as [g0, g1, g2].
pub type Quad = [u64; 3];

fn as_quad(a: &[u64]) -> Quad {
    [a.first().copied().unwrap_or(0), a.get(1).copied().unwrap_or(0), a.get(2).copied().unwrap_or(0)]
}

/// Every quadratic splitting f = G1 G2 G3 over F_p of a squarefree f of
/// degree 5 or 6 (the G_i of degree <= 2, monic except G1, which carries the
/// leading coefficient; listed in a canonical order).
pub fn quadratic_splittings(p: u64, f: &[u64]) -> Result<Vec<[Quad; 3]>, String> {
    let f = trim(f.to_vec());
    let n = f.len().saturating_sub(1);
    if !(n == 5 || n == 6) {
        return Err(format!("degree {n} is not 5 or 6"));
    }
    let Some((roots, quads)) = small_factors(p, &f)? else { return Ok(vec![]) };
    // Rational points of the root set; None is the root at infinity.
    let mut pts: Vec<Option<u64>> = roots.iter().map(|&r| Some(r)).collect();
    if n == 5 {
        pts.insert(0, None);
    }
    let lc = f[n];
    let mut out = vec![];
    for m in matchings(&pts) {
        let mut gs: Vec<Poly> = quads.clone();
        for (a, b) in m {
            gs.push(match (a, b) {
                (Some(a), Some(b)) => vec![a * b % p, (2 * p - a - b) % p, 1],
                (None, Some(r)) | (Some(r), None) => vec![(p - r) % p, 1],
                (None, None) => unreachable!("one root at infinity"),
            });
        }
        gs.sort_by(|x, y| (x.len(), x).cmp(&(y.len(), y)));
        gs[0] = scale(p, &gs[0], lc);
        let split = [as_quad(&gs[0]), as_quad(&gs[1]), as_quad(&gs[2])];
        let prod = mul(p, &mul(p, &gs[0], &gs[1]), &gs[2]);
        if prod != f {
            return Err(format!("{split:?} does not multiply to f"));
        }
        out.push(split);
    }
    Ok(out)
}

/// The Richelot codomain of a splitting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Richelot {
    /// C': y^2 = f, f = Delta^-1 h[0] h[1] h[2], h = ([G2,G3], [G3,G1], [G1,G2]).
    Curve { delta: u64, h: [Quad; 3], f: Poly },
    /// Delta = 0: Jac(C)/K is a product of elliptic curves.
    Split,
    /// Delta != 0 but the image is not a smooth genus-2 model (never for a
    /// smooth source; a problem when it happens).
    Singular { delta: u64, f: Poly },
}

/// Gi' Gj - Gi Gj'.
fn bracket(p: u64, a: &Quad, b: &Quad) -> Poly {
    let (a, b) = (trim(a.to_vec()), trim(b.to_vec()));
    sub(p, &mul(p, &derivative(p, &a), &b), &mul(p, &a, &derivative(p, &b)))
}

/// The Richelot isogenous curve of y^2 = G1 G2 G3 (p odd).
pub fn richelot(p: u64, g: &[Quad; 3]) -> Richelot {
    let m = |i: usize, j: usize| g[i][j] % p;
    let t = |a: u64, b: u64, c: u64, d: u64| (a * b % p + p - c * d % p) % p;
    let delta = (m(0, 0) * t(m(1, 1), m(2, 2), m(1, 2), m(2, 1)) % p + p
        - m(0, 1) * t(m(1, 0), m(2, 2), m(1, 2), m(2, 0)) % p
        + m(0, 2) * t(m(1, 0), m(2, 1), m(1, 1), m(2, 0)) % p)
        % p;
    if delta == 0 {
        return Richelot::Split;
    }
    let h = [bracket(p, &g[1], &g[2]), bracket(p, &g[2], &g[0]), bracket(p, &g[0], &g[1])];
    let f = scale(p, &mul(p, &mul(p, &h[0], &h[1]), &h[2]), inv(p, delta));
    let n = deg(&f);
    if !(n == 5 || n == 6) || !squarefree(p, &f) {
        return Richelot::Singular { delta, f };
    }
    Richelot::Curve { delta, h: [as_quad(&h[0]), as_quad(&h[1]), as_quad(&h[2])], f }
}

/// (#C(F_p), #C(F_{p^2})) of y^2 = f, genus 2, exhaustively.
pub fn counts(p: u64, f: &[u64]) -> (u64, u64) {
    (count_hyperelliptic_fp(p, f, 2), count_hyperelliptic_fp2(p, f, 2))
}

/// L(T) of a genus-2 curve over F_p from (#C(F_p), #C(F_{p^2})).
pub fn l_from_counts(p: u64, n: (u64, u64)) -> Option<Vec<BigInt>> {
    let q = BigInt::from(p);
    let a1: BigInt = BigInt::from(n.0) - &q - 1;
    let num: BigInt = BigInt::from(n.1) - &q * &q - 1 + &a1 * &a1;
    let two = BigInt::from(2);
    if &num % &two != BigInt::from(0) {
        return None;
    }
    let a2 = num / two;
    Some(vec![BigInt::from(1), a1.clone(), a2, &q * &a1, &q * &q])
}

/// One verified image.
#[derive(Debug, Clone)]
pub struct Image {
    pub kernel: [Quad; 3],
    pub delta: u64,
    pub h: [Quad; 3],
    pub f: Poly,
    pub counts: (u64, u64),
}

/// Everything found for one source curve.
#[derive(Debug, Default)]
pub struct SourceResult {
    pub counts: (u64, u64),
    pub splittings: usize,
    pub split_jacobians: usize,
    pub images: Vec<Image>,
    /// Cross-check failures: never inserted.
    pub failures: Vec<String>,
}

/// The cross-check of a claimed image y^2 = fi of a source with counts
/// `source` = (#C(F_p), #C(F_{p^2})): fi is a smooth genus-2 model and its
/// exhaustive counts are equal (equal L-polynomials).
pub fn check_image(p: u64, source: (u64, u64), fi: &[u64]) -> Result<(u64, u64), String> {
    let n = deg(fi);
    if !(n == 5 || n == 6) || fi.last() == Some(&0) || !squarefree(p, fi) {
        return Err(format!("image {fi:?} is not a smooth genus-2 model"));
    }
    let c = counts(p, fi);
    if c != source {
        return Err(format!("image {fi:?} has (#C(F_p), #C(F_p^2)) = {c:?}, the source {source:?}"));
    }
    Ok(c)
}

/// Every Richelot image of y^2 = f over F_p (3 <= p < [`MAX_P`] prime), each
/// cross-checked by equal point counts over F_p and F_{p^2}.
///
/// The source's counts: for p < [`DIRECT_FP2_MAX`] both are counted
/// exhaustively and must equal `stored` (the `curve_zeta` row of
/// `cover_lpoly`) when given; above it `stored` is required (`cover_lpoly`
/// derived it from the two elliptic quotients by baby-step giant-step, an
/// independent method) and #C(F_p) is recounted exhaustively against it.
/// Only the images are counted over F_{p^2} there.
pub fn richelot_images(p: u64, f: &[u64], stored: Option<(u64, u64)>) -> Result<SourceResult, &'static str> {
    if p < 3 || p.is_multiple_of(2) {
        return Err("even_characteristic");
    }
    if p >= MAX_P {
        return Err("field_too_large");
    }
    if (3..).step_by(2).take_while(|d| d * d <= p).any(|d| p.is_multiple_of(d)) {
        return Err("composite_characteristic");
    }
    let f = trim(f.to_vec());
    if !(f.len() == 6 || f.len() == 7) {
        return Err("unsupported_degree");
    }
    if !squarefree(p, &f) {
        return Err("singular_model");
    }
    let source = if p < DIRECT_FP2_MAX {
        counts(p, &f)
    } else {
        let Some(st) = stored else { return Err("no_stored_counts") };
        (count_hyperelliptic_fp(p, &f, 2), st.1)
    };
    let mut out = SourceResult { counts: source, ..Default::default() };
    if let Some(st) = stored {
        if st != source {
            out.failures.push(format!("source counts {source:?} disagree with the stored curve_zeta counts {st:?}"));
            return Ok(out);
        }
    }
    let splits = match quadratic_splittings(p, &f) {
        Ok(s) => s,
        Err(e) => {
            out.failures.push(format!("factorisation over F_{p}: {e}"));
            return Ok(out);
        }
    };
    out.splittings = splits.len();
    for g in splits {
        match richelot(p, &g) {
            Richelot::Split => out.split_jacobians += 1,
            Richelot::Singular { delta, f: fi } => out
                .failures
                .push(format!("kernel {g:?}: Delta = {delta} but the image {fi:?} is not a smooth genus-2 model")),
            Richelot::Curve { delta, h, f: fi } => match check_image(p, out.counts, &fi) {
                Ok(c) => out.images.push(Image { kernel: g, delta, h, f: fi, counts: c }),
                Err(e) => out.failures.push(format!("kernel {g:?}: {e}")),
            },
        }
    }
    Ok(out)
}

// ----------------------------------------------------------- import

struct Job {
    curve_id: i64,
    field_id: i64,
    p: u64,
    f: Vec<String>,
    stored_counts: Option<Vec<String>>,
    field_proved: bool,
}

fn strs(v: &[u64]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn quad_strs(q: &Quad) -> Vec<String> {
    strs(&trim(q.to_vec()))
}

fn run_job(j: &Job) -> Result<SourceResult, &'static str> {
    let Some(fu) = j.f.iter().map(|s| s.parse::<u64>().ok().filter(|&x| x < j.p)).collect::<Option<Vec<u64>>>() else {
        return Err("unparsed_model");
    };
    let stored = match &j.stored_counts {
        None => None,
        Some(v) => match v.as_slice() {
            [a, b] => match (a.parse::<u64>(), b.parse::<u64>()) {
                (Ok(a), Ok(b)) => Some((a, b)),
                _ => return Err("unparsed_stored_counts"),
            },
            _ => return Err("unparsed_stored_counts"),
        },
    };
    richelot_images(j.p, &fu, stored)
}

/// Run every job on up to four cores, in job order.
fn run_jobs(jobs: Vec<Job>) -> Vec<(Job, Result<SourceResult, &'static str>)> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(2, |n| n.get()).min(4);
    let results = std::sync::Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };
                let out = run_job(job);
                if let Ok(mut r) = results.lock() {
                    r.push((i, out));
                }
            });
        }
    });
    let mut r = results.into_inner().unwrap_or_default();
    r.sort_by_key(|(i, _)| *i);
    jobs.into_iter().zip(r).map(|(j, (_, o))| (j, o)).collect()
}

/// The content address of one isogeny: (p, source f, kernel).
pub fn external_id(p: u64, f: &[u64], kernel: &[Quad; 3]) -> String {
    let pre = json!({ "p": p.to_string(), "f": strs(f), "kernel": kernel.iter().map(quad_strs).collect::<Vec<_>>() });
    format!("richelot1_{}", &sha256_hex(&pre.to_string())[..16])
}

/// The `hyperelliptic_model` infinity data of y^2 = f, p odd.
fn infinity(p: u64, f: &[u64]) -> (&'static str, i32) {
    if f.len() == 6 {
        return ("ramified", 1);
    }
    let lc = f[f.len() - 1];
    if pw(p, lc, (p - 1) / 2) == 1 {
        ("split", 2)
    } else {
        ("inert", 0)
    }
}

const JOBS_SQL: &str = "
SELECT h.id, h.field_id, fl.characteristic::text, hm.f, z.point_counts::text[]
FROM ec.curve h
JOIN ec.hyperelliptic_model hm ON hm.curve_id = h.id
JOIN ec.field fl ON fl.id = h.field_id
LEFT JOIN ec.curve_zeta z ON z.curve_id = h.id
WHERE h.genus = 2 AND h.family = 'cover' AND fl.kind = 'prime' AND hm.h = '[]'::jsonb
ORDER BY h.id";

const CURVE_SQL: &str = "
INSERT INTO ec.curve (field_id, genus, model, form, coefficients, equation, family, is_smooth, smoothness_status,
       description, provenance_id)
SELECT r.field_id, 2, 'hyperelliptic', 'y^2+h(x)*y=f(x)', r.coefficients, r.equation, 'richelot', true, 'proved',
       r.description, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, equation text, description text)
ON CONFLICT DO NOTHING";

const MODEL_SQL: &str = "
INSERT INTO ec.hyperelliptic_model (curve_id, f, h, deg_f, deg_h, infinity_type, points_at_infinity, status,
       provenance_id)
SELECT c.id, r.coefficients->'f', '[]'::jsonb, r.deg_f, NULL, r.infinity_type, r.points_at_infinity, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, deg_f int, infinity_type text,
     points_at_infinity int)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
ON CONFLICT (curve_id) DO NOTHING";

const LINK_SQL: &str = "
UPDATE ec.abelian_variety av SET isogeny_class_id = ic.id
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, label text)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE av.curve_id = c.id AND av.isogeny_class_id IS NULL";

const ZETA_SQL: &str = "
INSERT INTO ec.curve_zeta (curve_id, isogeny_class_id, point_counts, order_check, order_method, provenance_id)
SELECT c.id, ic.id, r.point_counts, r.status::ec.fact_status, 'exhaustive count', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field_id bigint, coefficients jsonb, label text, point_counts numeric[],
     status text)
JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
ON CONFLICT (curve_id) DO NOTHING";

const ISOGENY_SQL: &str = "
INSERT INTO ec.isogeny (external_id, source_av_id, target_av_id, source_curve_id, target_curve_id, dimension, degree,
       degree_factorization, ell, kernel_structure, is_separable, field_of_definition_id, formula, kernel_generators,
       codomain_raw, is_normalized, certificate, status, provenance_id)
SELECT r.id, sa.id, ta.id, sc.id, tc.id, 2, 4, '[[2, 2]]'::jsonb, 2, ARRAY[2, 2]::numeric[], true, sc.field_id,
       'richelot', r.kernel, r.codomain, false, r.certificate, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(id text, source_id bigint, field_id bigint, coefficients jsonb, kernel jsonb,
     codomain jsonb, certificate text, status text)
JOIN ec.curve sc ON sc.id = r.source_id
JOIN ec.curve tc ON tc.field_id = r.field_id AND tc.model = 'hyperelliptic' AND tc.coefficients = r.coefficients
JOIN ec.abelian_variety sa ON sa.curve_id = sc.id
JOIN ec.abelian_variety ta ON ta.curve_id = tc.id
ON CONFLICT DO NOTHING";

/// Image curves whose stored row disagrees with the computed one: (k, what).
const CURVE_CHECK_SQL: &str = "
SELECT r.k::bigint, concat_ws(', ',
         CASE WHEN c.id IS NULL THEN 'curve missing' END,
         CASE WHEN c.id IS NOT NULL AND c.genus IS DISTINCT FROM 2 THEN 'genus' END,
         CASE WHEN c.id IS NOT NULL AND hm.f IS DISTINCT FROM r.coefficients->'f' THEN 'hyperelliptic_model f' END,
         CASE WHEN c.id IS NOT NULL AND z.point_counts::numeric[] IS DISTINCT FROM r.point_counts THEN 'curve_zeta point_counts' END,
         CASE WHEN c.id IS NOT NULL AND av.isogeny_class_id IS DISTINCT FROM ic.id THEN 'isogeny class' END)
FROM jsonb_to_recordset($1::jsonb) AS r(k int, field_id bigint, coefficients jsonb, label text, point_counts numeric[])
LEFT JOIN ec.curve c ON c.field_id = r.field_id AND c.model = 'hyperelliptic' AND c.coefficients = r.coefficients
LEFT JOIN ec.hyperelliptic_model hm ON hm.curve_id = c.id
LEFT JOIN ec.curve_zeta z ON z.curve_id = c.id
LEFT JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE c.id IS NULL OR c.genus IS DISTINCT FROM 2 OR hm.f IS DISTINCT FROM r.coefficients->'f'
   OR z.point_counts::numeric[] IS DISTINCT FROM r.point_counts OR av.isogeny_class_id IS DISTINCT FROM ic.id";

/// Isogenies sent whose stored row is missing or has other content.
const ISOGENY_CHECK_SQL: &str = "
SELECT r.id
FROM jsonb_to_recordset($1::jsonb) AS r(id text, source_id bigint, field_id bigint, coefficients jsonb, kernel jsonb)
LEFT JOIN ec.isogeny i ON i.external_id = r.id
LEFT JOIN ec.curve tc ON tc.id = i.target_curve_id
WHERE i.id IS NULL OR i.source_curve_id IS DISTINCT FROM r.source_id OR tc.field_id IS DISTINCT FROM r.field_id
   OR tc.coefficients IS DISTINCT FROM r.coefficients OR i.kernel_generators IS DISTINCT FROM r.kernel
   OR i.formula IS DISTINCT FROM 'richelot' OR i.dimension IS DISTINCT FROM 2 OR i.degree IS DISTINCT FROM 4";

const MISMATCH_SQL: &str = "
SELECT i.external_id, sa.isogeny_class_id, ta.isogeny_class_id
FROM ec.isogeny i
JOIN ec.abelian_variety sa ON sa.id = i.source_av_id
JOIN ec.abelian_variety ta ON ta.id = i.target_av_id
WHERE i.formula = 'richelot' AND i.external_id LIKE 'richelot1\\_%'
  AND sa.isogeny_class_id IS DISTINCT FROM ta.isogeny_class_id";

pub const METHOD: &str = "Richelot (2,2)-isogeny y^2 = Delta^-1 [G2,G3][G3,G1][G1,G2] for every quadratic splitting \
     f = G1 G2 G3 over F_p of the cover source (Cantor–Zassenhaus factorisation; Delta = 0, a split Jacobian, not \
     stored); image smooth of genus 2; #C(F_p) and #C(F_p^2) of the image counted exhaustively (p < 2^14) and equal to \
     the source's: counted exhaustively for p < 2^12, else the cover_lpoly curve_zeta row (elliptic quotients) with \
     #C(F_p) recounted; source counts equal to its stored curve_zeta row";

/// Compute, cross-check and store the Richelot images of the cover sources.
pub async fn import(ctx: &Ctx<'_>, report: &mut Report) -> Result<()> {
    let client = ctx.client;
    let rows = client.query(JOBS_SQL, &[]).await?;
    let mut jobs = vec![];
    for r in rows {
        let p_text: String = r.get(2);
        let Ok(p) = p_text.parse::<u64>() else {
            report.count("richelot_sources_skipped_field_too_large", 1);
            continue;
        };
        if p >= MAX_P {
            report.count("richelot_sources_skipped_field_too_large", 1);
            continue;
        }
        let f: Value = r.get(3);
        let f: Vec<String> =
            f.as_array().into_iter().flatten().filter_map(|x| x.as_str().map(str::to_string)).collect();
        let (prime, certain) = is_probable_prime(&BigInt::from(p));
        if !prime {
            report.count("richelot_sources_skipped_composite_characteristic", 1);
            continue;
        }
        jobs.push(Job { curve_id: r.get(0), field_id: r.get(1), p, f, stored_counts: r.get(4), field_proved: certain });
    }
    let outcomes = tokio::task::spawn_blocking(move || run_jobs(jobs)).await?;

    let mut curves = vec![];
    let mut classes: BTreeMap<String, Value> = BTreeMap::new();
    let mut links = vec![];
    let mut isogenies = vec![];
    for (job, res) in outcomes {
        let res = match res {
            Ok(r) => r,
            Err(reason) => {
                report.count(&format!("richelot_sources_skipped_{reason}"), 1);
                continue;
            }
        };
        report.count("richelot_sources", 1);
        let p = job.p;
        for msg in &res.failures {
            report.count("richelot_crosscheck_failures", 1);
            report.problem(format!("Richelot: cover source curve id {}: {msg}; not stored", job.curve_id));
        }
        report.count("richelot_splittings", res.splittings as u64);
        report.count("richelot_split_jacobians", res.split_jacobians as u64);
        if res.splittings == 0 {
            report.count("richelot_no_splitting", 1);
        }
        if res.images.is_empty() {
            continue;
        }
        let Some(l) = l_from_counts(p, res.counts) else {
            report.count("richelot_crosscheck_failures", 1);
            report.problem(format!("Richelot: cover source curve id {}: counts give a non-integral a2", job.curve_id));
            continue;
        };
        let status = if job.field_proved { "proved" } else { "probable" };
        let computed = Computed { g: 2, q: BigInt::from(p), p: BigInt::from(p), r: 1, l, method: Method::G2Count };
        let (label, class) = class_row(&computed, status)?;
        classes.entry(label.clone()).or_insert(class);
        let source_f: Vec<u64> = job.f.iter().filter_map(|s| s.parse().ok()).collect();
        for img in res.images {
            report.count("richelot_images_verified", 1);
            let fs = strs(&img.f);
            let coefficients = json!({ "f": fs, "h": [] });
            let (inf_type, inf_pts) = infinity(p, &img.f);
            let id = external_id(p, &source_f, &img.kernel);
            let k = curves.len();
            curves.push(json!({
                "k": k, "field_id": job.field_id, "coefficients": coefficients,
                "equation": format!("y^2 = {}", poly_text(&fs, "x")),
                "description": format!("Richelot (2,2)-isogenous to cover source curve id {} ({id})", job.curve_id),
                "deg_f": img.f.len() - 1, "infinity_type": inf_type, "points_at_infinity": inf_pts,
                "label": label, "point_counts": [img.counts.0.to_string(), img.counts.1.to_string()],
                "status": status,
            }));
            links.push(json!({ "field_id": job.field_id, "coefficients": coefficients, "label": label }));
            let kernel: Vec<Value> = img
                .kernel
                .iter()
                .map(|g| {
                    let m = monic(p, &trim(g.to_vec()));
                    json!({ "u": strs(&m), "v": ["0"] })
                })
                .collect();
            isogenies.push(json!({
                "k": k, "id": id, "source_id": job.curve_id, "field_id": job.field_id, "coefficients": coefficients,
                "kernel": kernel,
                "codomain": {
                    "splitting": img.kernel.iter().map(quad_strs).collect::<Vec<_>>(),
                    "delta": img.delta.to_string(),
                    "brackets": img.h.iter().map(quad_strs).collect::<Vec<_>>(),
                    "f": fs,
                    "convention": "source y^2 = G1 G2 G3 (splitting, ascending); Delta = det of the rows (g_i0, g_i1, g_i2); \
                                   brackets [G2,G3], [G3,G1], [G1,G2] with [A,B] = A'B - AB'; target y^2 = Delta^-1 times their product",
                },
                "certificate": format!(
                    "kernel: the 2-torsion classes of the zeros of the splitting factors (Mumford u = monic G_i, v = 0); \
                     Delta = {} != 0; image squarefree of degree {}; #C(F_p) = {} and #C(F_p^2) = {} for source and image \
                     (exhaustive counts), so equal L-polynomials {label}",
                    img.delta, img.f.len() - 1, img.counts.0, img.counts.1
                ),
                "status": status,
            }));
        }
    }
    if curves.is_empty() {
        return Ok(());
    }
    let src = client
        .query_one(
            "WITH ins AS (INSERT INTO ec.source (kind, title, uri, provenance_class)
                          VALUES ('computation', 'ecdb Richelot isogenies', 'ecdb:richelot', 'internal') ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.source WHERE kind = 'computation' AND uri = 'ecdb:richelot' AND repository IS NULL LIMIT 1",
            &[],
        )
        .await?
        .get::<_, i64>(0);
    let prov = ctx.provenance(src, METHOD, None).await?;
    let class_rows: Vec<Value> = classes.into_values().collect();
    report.count("richelot_curves", bulk(client, CURVE_SQL, &curves, prov).await?);
    report.count("richelot_models", bulk(client, MODEL_SQL, &curves, prov).await?);
    report.count("richelot_classes", bulk(client, super::cover_lpoly::CLASS_SQL, &class_rows, prov).await?);
    report.count("richelot_jacobians_linked", bulk1(client, LINK_SQL, &links).await?);
    report.count("richelot_zeta", bulk(client, ZETA_SQL, &curves, prov).await?);
    // An image curve that was already stored (by this pass or any other) must agree exactly with what was
    // computed: its model, its curve_zeta counts and its isogeny class. A disagreement is reported and no
    // isogeny is attached to that curve; nothing stored is changed.
    let mut refused = std::collections::BTreeSet::new();
    for chunk in curves.chunks(super::BATCH) {
        for r in client.query(CURVE_CHECK_SQL, &[&Value::Array(chunk.to_vec())]).await? {
            let k: i64 = r.get(0);
            let what: String = r.get(1);
            refused.insert(k);
            report.count("richelot_existing_conflicts", 1);
            report.problem(format!(
                "Richelot image {}: the stored curve disagrees with the computed one ({what}); no isogeny attached",
                curves[k as usize]["description"].as_str().unwrap_or_default()
            ));
        }
    }
    let isogenies: Vec<Value> =
        isogenies.into_iter().filter(|i| !i["k"].as_i64().is_some_and(|k| refused.contains(&k))).collect();
    report.count("richelot_isogenies", bulk(client, ISOGENY_SQL, &isogenies, prov).await?);
    // Every isogeny sent is now stored with exactly this source, target, kernel and formula; a row with the
    // same external_id and other content (left unchanged by ON CONFLICT DO NOTHING) is reported.
    for chunk in isogenies.chunks(super::BATCH) {
        for r in client.query(ISOGENY_CHECK_SQL, &[&Value::Array(chunk.to_vec())]).await? {
            let id: String = r.get(0);
            report.count("richelot_existing_conflicts", 1);
            report.problem(format!(
                "Richelot isogeny {id}: missing after insert, or a stored row with this external_id has another \
                 source, target, kernel or formula; left unchanged"
            ));
        }
    }
    let bad = client.query(MISMATCH_SQL, &[]).await?;
    report.count("richelot_class_mismatches", bad.len() as u64);
    for r in bad {
        let id: String = r.get(0);
        report.problem(format!("Richelot isogeny {id}: source and target Jacobians are in different isogeny classes"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::cover_lpoly::{curve_counts, genus2_prime, Outcome};
    use num_traits::ToPrimitive;

    /// #C(F_p) of y^2 = f by listing the pairs, plus the points at infinity.
    fn brute_fp(p: u64, f: &[u64]) -> u64 {
        let f = trim(f.to_vec());
        let mut n = 0;
        for x in 0..p {
            let fx = eval(p, &f, x);
            n += (0..p).filter(|&y| y * y % p == fx).count() as u64;
        }
        let lc = f[f.len() - 1];
        n + if f.len() == 6 { 1 } else { (0..p).filter(|&y| y * y % p == lc).count() as u64 }
    }

    /// #C(F_{p^2}) by listing the pairs of F_p[s]/(s^2 - nr), plus infinity
    /// (the leading coefficient is a square of F_{p^2}).
    fn brute_fp2(p: u64, f: &[u64]) -> u64 {
        let f = trim(f.to_vec());
        let nr = (2..p).find(|&x| (1..p).all(|y| y * y % p != x)).unwrap();
        let mul2 = |a: (u64, u64), b: (u64, u64)| ((a.0 * b.0 + nr * (a.1 * b.1 % p)) % p, (a.0 * b.1 + a.1 * b.0) % p);
        let mut squares = std::collections::HashMap::new();
        for a in 0..p {
            for b in 0..p {
                *squares.entry(mul2((a, b), (a, b))).or_insert(0u64) += 1;
            }
        }
        let mut n = 0;
        for a in 0..p {
            for b in 0..p {
                let v = f.iter().rev().fold((0, 0), |acc, &c| {
                    let m = mul2(acc, (a, b));
                    ((m.0 + c) % p, m.1)
                });
                n += squares.get(&v).copied().unwrap_or(0);
            }
        }
        n + if f.len() == 6 { 1 } else { 2 }
    }

    /// The prime_quadratic_pullback_v1 sextic G(u^2), G = F(x + c), F = x^3 + a x + b.
    fn cover_sextic(p: u64, a: u64, b: u64) -> Vec<u64> {
        let c = (0..4u64).find(|&c| !(c * c * c + a * c + b).is_multiple_of(p)).unwrap();
        vec![(c * c * c + a * c + b) % p, 0, (3 * c * c + a) % p, 0, 3 * c % p, 0, 1]
    }

    #[test]
    fn poly_arithmetic_and_factorisation() {
        let p = 13;
        // (x - 1)(x - 2)(x^2 + 2)(x^2 + 1)... x^2 + 1 splits mod 13 (5^2 = -1): use x^2 + 2 and x^2 + 5.
        let f = mul(p, &mul(p, &mul(p, &[12, 1], &[11, 1]), &[2, 0, 1]), &[5, 0, 1]);
        let (roots, quads) = small_factors(p, &f).unwrap().unwrap();
        assert_eq!(roots, vec![1, 2]);
        assert_eq!(quads, vec![vec![2, 0, 1], vec![5, 0, 1]]);
        // An irreducible cubic factor (x^3 - 2 is irreducible mod 7) leaves no splitting.
        let g = mul(7, &[5, 0, 0, 1], &[1, 1, 1, 1]);
        assert_eq!(small_factors(7, &g).unwrap(), None);
        assert_eq!(quadratic_splittings(7, &trim(g)).unwrap(), Vec::<[Quad; 3]>::new());
    }

    #[test]
    fn splittings_counts() {
        // Six rational roots: 15 matchings; a quintic with 5 rational roots: 15 (infinity is a root).
        let p = 31;
        let mut f: Poly = vec![1];
        for r in 1..=6 {
            f = mul(p, &f, &[p - r, 1]);
        }
        assert_eq!(quadratic_splittings(p, &f).unwrap().len(), 15);
        let mut g: Poly = vec![3];
        for r in 1..=5 {
            g = mul(p, &g, &[p - r, 1]);
        }
        let s = quadratic_splittings(p, &g).unwrap();
        assert_eq!(s.len(), 15);
        assert!(s.iter().all(|k| k.iter().filter(|q| q[2] == 0).count() == 1));
    }

    #[test]
    fn exhaustive_counts_match_brute_force() {
        for p in [3u64, 5, 7, 11, 13] {
            for f in [vec![1u64, 2, 0, 1, 0, 1], vec![2, 0, 1, 1, 0, 0, 1], vec![3, 1, 4, 1, 5, 2, 2]] {
                let f: Poly = trim(f.iter().map(|x| x % p).collect());
                if !squarefree(p, &f) || f.len() < 6 {
                    continue;
                }
                assert_eq!(counts(p, &f), (brute_fp(p, &f), brute_fp2(p, &f)), "p = {p}, f = {f:?}");
            }
        }
    }

    /// Every splitting of every smooth y^2 = f over tiny fields: the image has
    /// the same brute-force point counts over F_p and F_{p^2}.
    #[test]
    fn richelot_brute_force_tiny_fields() {
        let mut checked = 0;
        for p in [5u64, 7, 11] {
            // Products of three quadratics (x - a)(x - b) and x^2 - n.
            for a in 0..p {
                for b in (a + 1)..p {
                    for n in 1..p {
                        let g1 = [a * b % p, (2 * p - a - b) % p, 1];
                        let g2 = [(p - n) % p, 0, 1];
                        let g3 = [(n + 1) % p, 1, 2];
                        let f = mul(p, &mul(p, &trim(g1.to_vec()), &trim(g2.to_vec())), &trim(g3.to_vec()));
                        if !squarefree(p, &f) {
                            continue;
                        }
                        let res = richelot_images(p, &f, None).unwrap();
                        assert!(res.failures.is_empty(), "p = {p}, f = {f:?}: {:?}", res.failures);
                        assert!(res.splittings >= 1);
                        for img in &res.images {
                            assert_eq!(
                                (brute_fp(p, &img.f), brute_fp2(p, &img.f)),
                                (brute_fp(p, &f), brute_fp2(p, &f))
                            );
                            checked += 1;
                        }
                    }
                }
            }
        }
        assert!(checked > 100, "{checked}");
    }

    /// The other sign convention of Delta (columns g_i2, g_i1, g_i0) is the
    /// quadratic twist by -1, which the F_p count rejects when p = 3 mod 4.
    #[test]
    fn crosscheck_rejects_wrong_twist_and_corruption() {
        let p = 7;
        let g = [[1u64, 0, 1], [3, 1, 1], [6, 4, 1]]; // x^2 + 1, x^2 + x + 3, x^2 + 4x + 6
        let f = mul(p, &mul(p, &trim(g[0].to_vec()), &trim(g[1].to_vec())), &trim(g[2].to_vec()));
        assert_eq!(f, vec![4, 4, 3, 2, 0, 5, 1]);
        assert!(squarefree(p, &f));
        let Richelot::Curve { f: fi, .. } = richelot(p, &g) else { panic!("expected a curve") };
        // Pinned against an independent reference implementation (not committed).
        assert_eq!(fi, vec![1, 3, 0, 0, 1, 1, 3]);
        assert_eq!(counts(p, &fi), (12, 70));
        assert_eq!(counts(p, &f), (12, 70));
        // The whole pipeline finds this image among the splittings of f.
        let res = richelot_images(p, &f, None).unwrap();
        assert!(res.failures.is_empty());
        assert!(res.images.iter().any(|i| i.f == fi));
        // Corrupted stored counts of the source: reported, nothing verified.
        let res = richelot_images(p, &f, Some((13, 70))).unwrap();
        assert!(!res.failures.is_empty() && res.images.is_empty());
        let res = richelot_images(p, &f, Some((12, 70))).unwrap();
        assert!(res.failures.is_empty() && !res.images.is_empty());
        assert_eq!(check_image(p, counts(p, &f), &fi), Ok((12, 70)));
        // The wrong twist and a corrupted coefficient are rejected.
        let twisted = scale(p, &fi, p - 1);
        assert!(check_image(p, counts(p, &f), &twisted).is_err());
        let mut bad = fi.clone();
        bad[0] = (bad[0] + 1) % p;
        assert!(check_image(p, counts(p, &f), &bad).is_err());
        // A corrupted source: the old image no longer matches its counts.
        let mut bad_source = f.clone();
        bad_source[0] = (bad_source[0] + 1) % p;
        if squarefree(p, &bad_source) {
            assert!(check_image(p, counts(p, &bad_source), &fi).is_err());
        }
    }

    /// A cover source v^2 = G(u^2): the splitting u^2 - r_i has Delta = 0 (the
    /// split Jacobian Jac(H) ~ E x E'), and every image has the L-polynomial
    /// that cover_lpoly computes from the two elliptic quotients (baby-step
    /// giant-step), the value ecdb holds for the source.
    #[test]
    fn cover_source_pinned_against_cover_lpoly() {
        use crate::import::cover_lpoly::{quotient_traces, Fp64};
        let (mut images, mut splits, mut sources) = (0, 0, 0);
        for p in [13u64, 101, 103, 211, 1009] {
            for (a, b) in [(1u64, 1u64), (2, 3), (5, 7), (3, 11), (6, 1), (0, 5), (4, 0)] {
                let f = cover_sextic(p, a % p, b % p);
                if !squarefree(p, &f) {
                    continue;
                }
                // Baby-step giant-step can be inconclusive on tiny fields: skip those.
                let Ok((t, _)) = quotient_traces(&Fp64 { p }, &f) else { continue };
                let Outcome::Done(c) = genus2_prime(p, &f, &[BigInt::from(t)]) else { panic!("cover_lpoly result") };
                let res = richelot_images(p, &f, None).unwrap();
                assert!(res.failures.is_empty(), "{:?}", res.failures);
                assert_eq!(l_from_counts(p, res.counts).unwrap(), c.l, "p = {p}, f = {f:?}");
                for img in &res.images {
                    assert_eq!(l_from_counts(p, img.counts).unwrap(), c.l);
                }
                sources += 1;
                splits += res.split_jacobians;
                images += res.images.len();
            }
        }
        assert!(sources >= 30 && splits > 0 && images > 0, "{sources} {splits} {images}");
        // Above 2^12 the source's counts come from cover_lpoly (elliptic quotients).
        let mut above = 0;
        for p in [4099u64, 4111] {
            for (a, b) in [(1u64, 1u64), (2, 3), (5, 7), (3, 11), (6, 1), (0, 5), (4, 0), (7, 2)] {
                let f = cover_sextic(p, a, b);
                if !squarefree(p, &f) {
                    continue;
                }
                assert_eq!(richelot_images(p, &f, None).unwrap_err(), "no_stored_counts");
                let Ok((t, _)) = quotient_traces(&Fp64 { p }, &f) else { continue };
                let Outcome::Done(c) = genus2_prime(p, &f, &[BigInt::from(t)]) else { panic!("cover_lpoly result") };
                let cc = curve_counts(&c.l, &c.q);
                let stored = (cc[0].to_u64().unwrap(), cc[1].to_u64().unwrap());
                let res = richelot_images(p, &f, Some(stored)).unwrap();
                assert!(res.failures.is_empty(), "{:?}", res.failures);
                above += res.images.len();
                if let Some(img) = res.images.first() {
                    assert_eq!(counts(p, &img.f), stored);
                }
            }
        }
        assert!(above > 0, "no image above 2^12 in the sample");
    }

    /// A composite characteristic is refused before any arithmetic (inverses mod a composite are wrong).
    #[test]
    fn composite_characteristic_is_refused() {
        let f = [4u64, 4, 3, 2, 0, 5, 1];
        for n in [9u64, 15, 21, 25, 49, 91, 4095, 16383] {
            assert_eq!(richelot_images(n, &f, None).unwrap_err(), "composite_characteristic", "n = {n}");
        }
        assert!(richelot_images(7, &f, None).is_ok());
        assert!(richelot_images(16381, &f, Some((0, 0))).is_ok());
    }

    #[test]
    fn external_id_is_stable() {
        let k = [[1u64, 0, 1], [2, 1, 1], [3, 3, 1]];
        let a = external_id(7, &[6, 1, 2, 3, 4, 5, 1], &k);
        assert_eq!(a, external_id(7, &[6, 1, 2, 3, 4, 5, 1], &k));
        assert!(a.starts_with("richelot1_") && a.len() == 26);
        assert_ne!(a, external_id(11, &[6, 1, 2, 3, 4, 5, 1], &k));
    }
}
