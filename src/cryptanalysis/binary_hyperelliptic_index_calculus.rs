//! # Index calculus on hyperelliptic Jacobians over `F_{2^m}`.
//!
//! This is the characteristic-two companion to
//! [`crate::cryptanalysis::hyperelliptic_index_calculus`].  It attacks the
//! discrete logarithm in `Jac(C)(F_{2^m})` for
//!
//! ```text
//!   C : y^2 + h(x) y = f(x),   deg f = 2g + 1,   deg h <= g,
//! ```
//!
//! using the Gaudry / Adleman–DeMarrais–Huang relation strategy with a
//! degree-one factor base.  The factor base is the set of places
//! `P = (x_0, y_0)`, one representative of each `±` pair, stored as the
//! divisor class `[P] - [∞]` in Mumford form `(x + x_0, y_0)`.
//!
//! ## What is new relative to the prime-field engine
//!
//! In characteristic two the affine equation is **linear in `y`** once
//! `h(x_0) ≠ 0`: `y_0 = f(x_0) / h(x_0)`, and the other point with the
//! same abscissa is `y_0 + h(x_0)`.  When `h(x_0) = 0` the point is
//! ramified and `y_0 = sqrt(f(x_0))` is unique.  Consequently *every*
//! `x_0 ∈ F_{2^m}` contributes exactly one factor-base column, and the
//! full base has `2^m` entries (versus about `p/2` in odd characteristic).
//!
//! Smoothness is decided by splitting `u(x)` into linear factors over
//! `F_{2^m}`.  The implementation uses the characteristic-two
//! Cantor–Zassenhaus split:
//!
//! 1. `gcd(u, x^{2^m} + x)` is the product of the distinct linear factors
//!    of `u`; comparing its degree with the squarefree radical of `u`
//!    decides smoothness in `O(m·deg(u)^2)` field operations, with no
//!    scan over the field.
//! 2. A random polynomial `a` is split by the absolute trace
//!    `Tr(a) = a + a^2 + ... + a^{2^{m-1}}`: at each root the trace is
//!    `0` or `1`, so `gcd(u, Tr(a))` separates the roots into two sets.
//!    Recursing gives every root.
//!
//! The relation walk is the factor-base walk from the prime engine: a
//! step by a factor-base place costs one Jacobian addition and is
//! recorded so that its unknown logarithm cancels when the resulting
//! divisor decomposes.
//!
//! ## Scope and honesty
//!
//! - Odd-degree, one-rational-point-at-infinity models only, matching
//!   [`crate::binary_ecc::hyperelliptic`].
//! - The factor base is `2^m` places when materialised in full, so the
//!   feasible full-base range is `m <= ~20`; a reduced base (Gaudry's
//!   `q^{2/(g+1)}`) is selected automatically for larger fields.
//! - `p`-adic / extension-field base fields are not handled here; the
//!   base field is exactly `F_{2^m}`.
//! - Nothing here is a claim about deployed curves.  It is an engine and
//!   a measurement substrate.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

use crate::binary_ecc::f2m::{F2mElement, IrreduciblePoly};
use crate::binary_ecc::hyperelliptic::{HyperellipticCurve, MumfordDivisor};
use crate::binary_ecc::poly_f2m::F2mPoly;
use crate::cryptanalysis::ec_index_calculus::gaussian_eliminate_mod_n_particular;
use crate::cryptanalysis::hyperelliptic_index_calculus::{
    factorise_small, HecIndexCalculusReport, HecRelation, LinearAlgebra, RelationSearch,
};
use crate::utils::mod_inverse;

// ── Factor base ────────────────────────────────────────────────────────

/// One degree-one place and its divisor class `[P] - [∞]`.
#[derive(Clone, Debug)]
pub struct BhecFactorBaseEntry {
    pub x: F2mElement,
    pub y: F2mElement,
    pub divisor: MumfordDivisor,
}

/// The degree-one factor base, plus the `x ↦ index` map the decomposition
/// needs.
#[derive(Clone, Debug)]
pub struct BhecFactorBase {
    pub entries: Vec<BhecFactorBaseEntry>,
    by_x: HashMap<BigUint, usize>,
}

impl BhecFactorBase {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn index_of_x(&self, x: &BigUint) -> Option<usize> {
        self.by_x.get(x).copied()
    }
}

/// Solve `y^2 + h(x) y = f(x)` for the canonical representative of the
/// `±` pair at abscissa `x`.
///
/// When `h(x) = 0` the equation is `y^2 = f(x)`, which has a unique root in
/// `F_{2^m}`.  When `h(x) ≠ 0`, substitute `y = h(x)·z`; the equation
/// becomes the Artin–Schreier equation
///
/// ```text
///   z^2 + z = f(x) / h(x)^2.
/// ```
///
/// It has two solutions when the absolute trace of the right-hand side is
/// zero and none when the trace is one, so not every abscissa contributes a
/// factor-base column.  The two roots are `y` and `y + h(x)`; the one with
/// the smaller integer encoding is stored.
pub fn affine_point_for_x(curve: &HyperellipticCurve, x: &F2mElement) -> Option<F2mElement> {
    let irr = &curve.irr;
    let hx = curve.h.eval(x, irr);
    let fx = curve.f.eval(x, irr);
    let y = if hx.is_zero() {
        // In F_{2^m} squaring is a bijection, so f(x) has a unique root.
        fx.square_k_times(curve.m.saturating_sub(1), irr)
    } else {
        let inv = hx.flt_inverse(irr)?;
        let a = fx.mul(&inv, irr).mul(&inv, irr);
        let z = artin_schreier_solve(&a, irr)?;
        hx.mul(&z, irr)
    };
    let other = y.add(&hx);
    Some(if other.to_biguint() < y.to_biguint() {
        other
    } else {
        y
    })
}

/// Solve `z^2 + z = a` over `F_{2^m}`.
///
/// The map `L(z) = z^2 + z` is `F_2`-linear with image exactly the
/// trace-zero elements.  The implementation writes `L` on the monomial
/// basis as an `m × m` matrix over `F_2` and solves the linear system; the
/// kernel is `{0, 1}`, so setting the one free variable to zero gives a
/// deterministic solution.
fn artin_schreier_solve(a: &F2mElement, irr: &IrreduciblePoly) -> Option<F2mElement> {
    let m = a.m_value();
    if a.trace(irr) != 0 {
        return None;
    }
    if a.is_zero() {
        return Some(F2mElement::zero(m));
    }
    // The repository already carries the half-trace table for odd m.
    if let Some(half) = a.half_trace(irr) {
        return Some(half);
    }
    let words = (m as usize).div_ceil(64);
    let mut columns: Vec<Vec<u64>> = Vec::with_capacity(m as usize);
    for i in 0..m {
        let mut basis = vec![0u64; words];
        basis[i as usize / 64] |= 1u64 << (i % 64);
        let e = F2mElement::from_words(&basis, m);
        let image = e.square(irr).add(&e);
        columns.push(to_words(&image, words));
    }

    // rows[j] is the j-th equation over the m unknown coefficients of z.
    let mut rows = vec![vec![0u64; words]; m as usize];
    let mut rhs = vec![0u8; m as usize];
    let a_words = to_words(a, words);
    for (i, column) in columns.iter().enumerate() {
        for j in 0..m as usize {
            if get_bit(column, j) {
                rows[j][i / 64] |= 1u64 << (i % 64);
            }
        }
    }
    for j in 0..m as usize {
        rhs[j] = get_bit(&a_words, j) as u8;
    }

    let mut pivots: Vec<(usize, usize)> = Vec::new();
    let mut row = 0usize;
    for col in 0..m as usize {
        if row == m as usize {
            break;
        }
        let Some(pivot) = (row..m as usize).find(|&r| get_bit(&rows[r], col)) else {
            continue;
        };
        rows.swap(row, pivot);
        rhs.swap(row, pivot);
        let pivot_row = rows[row].clone();
        let pivot_rhs = rhs[row];
        for r in 0..m as usize {
            if r != row && get_bit(&rows[r], col) {
                xor_into(&mut rows[r], &pivot_row);
                rhs[r] ^= pivot_rhs;
            }
        }
        pivots.push((row, col));
        row += 1;
    }
    for r in row..m as usize {
        if rows[r].iter().all(|w| *w == 0) && rhs[r] != 0 {
            return None;
        }
    }
    let mut z = vec![0u64; words];
    for (r, col) in pivots {
        if rhs[r] != 0 {
            z[col / 64] |= 1u64 << (col % 64);
        }
    }
    Some(F2mElement::from_words(&z, m))
}

fn to_words(element: &F2mElement, words: usize) -> Vec<u64> {
    let mut out = vec![0u64; words];
    for (i, limb) in element.to_biguint().to_u64_digits().iter().enumerate() {
        if i < words {
            out[i] = *limb;
        }
    }
    out
}

fn get_bit(words: &[u64], index: usize) -> bool {
    (words[index / 64] >> (index % 64)) & 1 == 1
}

fn xor_into(dst: &mut [u64], src: &[u64]) {
    for (a, b) in dst.iter_mut().zip(src.iter()) {
        *a ^= *b;
    }
}

/// `(x + x_0, y_0)` as a reduced Mumford divisor.
pub fn divisor_for_point(
    curve: &HyperellipticCurve,
    x: &F2mElement,
    y: &F2mElement,
) -> MumfordDivisor {
    let u = F2mPoly::from_coeffs(vec![x.clone(), F2mElement::one(curve.m)], curve.m);
    let v = F2mPoly::constant(y.clone());
    MumfordDivisor { u, v }
}

/// Build the degree-one factor base.
///
/// `max_size == usize::MAX` (or a cap at least `2^m`) materialises every
/// place when the field is small enough to enumerate; otherwise the base
/// is a deterministic random subset of that size.  Sampling is what makes
/// the reduced-base regime (`2^m` far above the LA budget) usable at all.
pub fn build_factor_base(curve: &HyperellipticCurve, max_size: usize, seed: u64) -> BhecFactorBase {
    let q = if curve.m < 63 {
        Some(1u64 << curve.m)
    } else {
        None
    };
    let enumerate =
        q.is_some_and(|q| q <= (1u64 << 20) && (max_size == usize::MAX || max_size as u64 >= q));

    let mut entries = Vec::new();
    let mut by_x = HashMap::new();

    if enumerate {
        let q = q.expect("enumerated field");
        for xi in 0..q {
            let x = F2mElement::from_biguint(&BigUint::from(xi), curve.m);
            let Some(y) = affine_point_for_x(curve, &x) else {
                continue;
            };
            let key = x.to_biguint();
            by_x.insert(key, entries.len());
            entries.push(BhecFactorBaseEntry {
                x: x.clone(),
                y: y.clone(),
                divisor: divisor_for_point(curve, &x, &y),
            });
        }
        return BhecFactorBase { entries, by_x };
    }

    let wanted = if max_size == usize::MAX {
        1usize << curve.m.min(20)
    } else {
        max_size.max(1)
    };
    let mut rng = StdRng::seed_from_u64(seed ^ 0x8f3f_73b5_c9a1_2d47);
    let mut seen: HashSet<BigUint> = HashSet::new();
    let budget = wanted.saturating_mul(16).saturating_add(256);
    for _ in 0..budget {
        if entries.len() >= wanted {
            break;
        }
        let x = random_element(&mut rng, curve.m);
        let key = x.to_biguint();
        if !seen.insert(key.clone()) {
            continue;
        }
        let Some(y) = affine_point_for_x(curve, &x) else {
            continue;
        };
        by_x.insert(key, entries.len());
        entries.push(BhecFactorBaseEntry {
            x: x.clone(),
            y: y.clone(),
            divisor: divisor_for_point(curve, &x, &y),
        });
    }
    BhecFactorBase { entries, by_x }
}

/// The engine's default base: the full degree-one base while it is small
/// enough to materialise and factor comfortably (`m <= 12`), then Gaudry's
/// reduced base.
pub fn default_factor_base_size(curve: &HyperellipticCurve) -> usize {
    if curve.m <= 12 {
        1usize << curve.m
    } else {
        suggested_factor_base_size(curve)
    }
}

/// Gaudry's reduced-base heuristic `q^{2/(g+1)}`, clamped to the field
/// size and to a practical minimum.
pub fn suggested_factor_base_size(curve: &HyperellipticCurve) -> usize {
    let q = 2f64.powi(curve.m as i32);
    let g = curve.genus.max(1) as f64;
    let b = q.powf(2.0 / (g + 1.0)).ceil();
    (b as usize).clamp(8, q.min(usize::MAX as f64) as usize)
}

// ── Smoothness / root finding over F_{2^m} ─────────────────────────────

/// How smoothness is decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BhecSmoothnessTest {
    /// Characteristic-two Cantor–Zassenhaus: `O(m·deg(u)^2)` field
    /// operations and no scan over the field.
    #[default]
    Trace,
    /// Evaluate `u` at every field element.  A reference for tests on
    /// small fields, not the production path.
    Scan,
}

/// Split a monic `u` into linear factors over `F_{2^m}`.
///
/// Returns `(root, multiplicity)` pairs, or `None` when `u` has an
/// irreducible factor of degree at least two.  The empty polynomial is
/// the empty product.
pub fn split_into_linear_factors(
    u: &F2mPoly,
    irr: &IrreduciblePoly,
) -> Option<Vec<(F2mElement, usize)>> {
    split_with(u, irr, BhecSmoothnessTest::Trace)
}

/// As [`split_into_linear_factors`], choosing the oracle.
pub fn split_with(
    u: &F2mPoly,
    irr: &IrreduciblePoly,
    test: BhecSmoothnessTest,
) -> Option<Vec<(F2mElement, usize)>> {
    match test {
        BhecSmoothnessTest::Trace => split_by_trace(u, irr),
        BhecSmoothnessTest::Scan => split_by_scan(u, irr),
    }
}

fn split_by_trace(u: &F2mPoly, irr: &IrreduciblePoly) -> Option<Vec<(F2mElement, usize)>> {
    let deg = u.degree()?;
    if deg == 0 {
        return Some(Vec::new());
    }
    let m = u.m;

    // Work modulo the squarefree radical so repeated factors do not make
    // `x^{2^m} - x` vanish as a whole polynomial; the multiplicities are
    // attached afterwards from the original `u`.
    let radical = squarefree_radical(u, irr);
    let x = F2mPoly::x(m);
    let mut frob = x.rem(&radical, irr);
    for _ in 0..m {
        frob = frob.square_mod(&radical, irr);
    }
    // frob = x^{2^m} mod radical; gcd(radical, frob + x) is the product
    // of the distinct linear factors.
    let linear = radical.gcd(&frob.add(&x), irr).monic(irr);
    if linear.degree() != radical.degree() {
        return None;
    }

    let roots = roots_of_split(&linear, irr)?;
    multiplicities(u, roots, irr)
}

/// Squarefree radical of a monic polynomial in characteristic two.
///
/// The naive `u / gcd(u, u')` is only the product of the factors with
/// **odd** multiplicity in characteristic two; an even-multiplicity factor
/// has derivative zero and disappears.  The correct recursion combines the
/// odd part `w = u / gcd(u, u')` with the radical of the square root of the
/// even part, removing the factors the two share.
fn squarefree_radical(u: &F2mPoly, irr: &IrreduciblePoly) -> F2mPoly {
    let u = u.monic(irr);
    let Some(deg) = u.degree() else {
        return u;
    };
    if deg == 0 {
        return u;
    }
    let derivative = derivative_poly(&u);
    if derivative.is_zero() {
        // u = h(x)^2; every root has even multiplicity.
        return squarefree_radical(&poly_sqrt(&u, irr), irr);
    }
    let g = u.gcd(&derivative, irr);
    if g.degree() == Some(0) {
        return u;
    }
    let (w, rem) = u.divrem(&g, irr);
    if !rem.is_zero() {
        return u;
    }
    let w = w.monic(irr);
    let h = poly_sqrt(&g, irr);
    let rh = squarefree_radical(&h, irr);
    let common = w.gcd(&rh, irr);
    let (rh_unique, rem) = rh.divrem(&common, irr);
    if !rem.is_zero() {
        return u;
    }
    w.mul(&rh_unique, irr).monic(irr)
}

fn derivative_poly(u: &F2mPoly) -> F2mPoly {
    let m = u.m;
    let Some(deg) = u.degree() else {
        return F2mPoly::zero(m);
    };
    let mut coeffs = vec![F2mElement::zero(m); deg];
    for i in (1..=deg).step_by(2) {
        coeffs[i - 1] = u.coeff(i);
    }
    F2mPoly::from_coeffs(coeffs, m)
}

/// Square root of a polynomial whose exponents are all even:
/// `sqrt(Σ a_i x^i) = Σ sqrt(a_i) x^{i/2}`.
fn poly_sqrt(u: &F2mPoly, irr: &IrreduciblePoly) -> F2mPoly {
    let m = u.m;
    let Some(deg) = u.degree() else {
        return F2mPoly::zero(m);
    };
    let mut coeffs = vec![F2mElement::zero(m); deg / 2 + 1];
    for i in (0..=deg).step_by(2) {
        coeffs[i / 2] = u.coeff(i).square_k_times(m.saturating_sub(1), irr);
    }
    F2mPoly::from_coeffs(coeffs, m)
}

/// Roots of a monic, squarefree, completely split polynomial.
fn roots_of_split(d: &F2mPoly, irr: &IrreduciblePoly) -> Option<Vec<F2mElement>> {
    let deg = d.degree()?;
    match deg {
        0 => Some(Vec::new()),
        1 => Some(vec![d.coeff(0)]),
        _ => {
            let m = d.m;
            let mut rng = StdRng::seed_from_u64(0x2b1c_5e11_7a93_d4f1 ^ deg as u64);
            for _ in 0..128 {
                let mut coeffs = Vec::with_capacity(deg);
                for _ in 0..deg {
                    coeffs.push(random_element(&mut rng, m));
                }
                let a = F2mPoly::from_coeffs(coeffs, m);
                if a.is_zero() {
                    continue;
                }
                let trace = trace_mod(&a, d, irr);
                let factor = d.gcd(&trace, irr);
                let fdeg = factor.degree().unwrap_or(0);
                if fdeg == 0 || fdeg == deg {
                    continue;
                }
                let (other, rem) = d.divrem(&factor, irr);
                if !rem.is_zero() {
                    continue;
                }
                let mut roots = roots_of_split(&factor.monic(irr), irr)?;
                roots.extend(roots_of_split(&other.monic(irr), irr)?);
                return Some(roots);
            }
            None
        }
    }
}

/// `a + a^2 + ... + a^{2^{m-1}} mod d`.
fn trace_mod(a: &F2mPoly, d: &F2mPoly, irr: &IrreduciblePoly) -> F2mPoly {
    let m = d.m;
    let mut sum = F2mPoly::zero(m);
    let mut term = a.rem(d, irr);
    for _ in 0..m {
        sum = sum.add(&term);
        term = term.square_mod(d, irr);
    }
    sum.rem(d, irr)
}

/// Attach multiplicities by dividing out each linear factor.
fn multiplicities(
    u: &F2mPoly,
    roots: Vec<F2mElement>,
    irr: &IrreduciblePoly,
) -> Option<Vec<(F2mElement, usize)>> {
    let m = u.m;
    let deg = u.degree()?;
    let mut rest = u.clone();
    let mut out = Vec::with_capacity(roots.len());
    let mut total = 0usize;
    for root in roots {
        let lin = F2mPoly::from_coeffs(vec![root.clone(), F2mElement::one(m)], m);
        let mut mult = 0usize;
        loop {
            let (q, r) = rest.divrem(&lin, irr);
            if !r.is_zero() {
                break;
            }
            rest = q;
            mult += 1;
        }
        if mult > 0 {
            total += mult;
            out.push((root, mult));
        }
    }
    if total == deg {
        Some(out)
    } else {
        None
    }
}

fn split_by_scan(u: &F2mPoly, irr: &IrreduciblePoly) -> Option<Vec<(F2mElement, usize)>> {
    let deg = u.degree()?;
    if deg == 0 {
        return Some(Vec::new());
    }
    let m = u.m;
    if m >= 63 {
        return None;
    }
    let q = 1u64 << m;
    let mut rest = u.clone();
    let mut out = Vec::new();
    let mut total = 0usize;
    for xi in 0..q {
        if total == deg {
            break;
        }
        let x = F2mElement::from_biguint(&BigUint::from(xi), m);
        if !rest.eval(&x, irr).is_zero() {
            continue;
        }
        let lin = F2mPoly::from_coeffs(vec![x.clone(), F2mElement::one(m)], m);
        let mut mult = 0usize;
        loop {
            let (next, r) = rest.divrem(&lin, irr);
            if !r.is_zero() {
                break;
            }
            rest = next;
            mult += 1;
        }
        total += mult;
        out.push((x, mult));
    }
    if total == deg {
        Some(out)
    } else {
        None
    }
}

/// Decompose a reduced divisor over the factor base.
///
/// Returns `[(index, coefficient)]` with
/// `D ≡ Σ coefficient · F_index` in `Jac(C)(F_{2^m})`, or `None` when
/// `u` does not split completely over the *stored* base.
pub fn decompose_over_factor_base(
    curve: &HyperellipticCurve,
    d: &MumfordDivisor,
    fb: &BhecFactorBase,
) -> Option<Vec<(usize, i64)>> {
    decompose_with(curve, d, fb, BhecSmoothnessTest::Trace)
}

pub fn decompose_with(
    curve: &HyperellipticCurve,
    d: &MumfordDivisor,
    fb: &BhecFactorBase,
    test: BhecSmoothnessTest,
) -> Option<Vec<(usize, i64)>> {
    if d.is_identity() {
        return Some(Vec::new());
    }
    let roots = split_with(&d.u, &curve.irr, test)?;
    let mut acc: HashMap<usize, i64> = HashMap::new();
    for (x, mult) in roots {
        let idx = fb.index_of_x(&x.to_biguint())?;
        let y = d.v.eval(&x, &curve.irr);
        let entry = &fb.entries[idx];
        let sign = if y == entry.y {
            1
        } else {
            let other = entry.y.add(&curve.h.eval(&x, &curve.irr));
            if y == other {
                -1
            } else {
                return None;
            }
        };
        *acc.entry(idx).or_insert(0) += sign * mult as i64;
    }
    let mut out: Vec<(usize, i64)> = acc.into_iter().filter(|&(_, c)| c != 0).collect();
    out.sort_unstable_by_key(|&(i, _)| i);
    Some(out)
}

// ── Parameters and relation collection ─────────────────────────────────

/// Parameters of the characteristic-two relation search.
#[derive(Clone, Debug)]
pub struct BhecIndexCalculusParams {
    /// Factor-base cap.  `0` selects [`suggested_factor_base_size`].
    pub fb_size: usize,
    pub extra_relations: usize,
    pub max_trials: usize,
    pub seed: u64,
    pub search: RelationSearch,
    pub linear_algebra: LinearAlgebra,
    pub smoothness: BhecSmoothnessTest,
    /// Independent relation-collection threads.  `1` keeps the serial
    /// engine; larger values split the trial budget across walks and merge
    /// the rows before the linear algebra.
    pub threads: usize,
}

impl Default for BhecIndexCalculusParams {
    fn default() -> Self {
        Self {
            fb_size: 0,
            extra_relations: 8,
            max_trials: 200_000,
            seed: 0,
            search: RelationSearch::factor_base_walk(),
            linear_algebra: LinearAlgebra::Sparse,
            smoothness: BhecSmoothnessTest::Trace,
            threads: 1,
        }
    }
}

fn subtract_steps(
    decomposition: Vec<(usize, i64)>,
    taken: &HashMap<usize, i64>,
) -> Vec<(usize, i64)> {
    if taken.is_empty() {
        return decomposition;
    }
    let mut acc: BTreeMap<usize, i64> = decomposition.into_iter().collect();
    for (&idx, &count) in taken {
        *acc.entry(idx).or_insert(0) -= count;
    }
    acc.into_iter().filter(|&(_, c)| c != 0).collect()
}

/// Collect relations `a·D_1 + b·D_2 = Σ c_i F_i`.
#[allow(clippy::too_many_arguments)]
pub fn collect_relations(
    curve: &HyperellipticCurve,
    d1: &MumfordDivisor,
    d2: &MumfordDivisor,
    n: &BigUint,
    fb: &BhecFactorBase,
    wanted: usize,
    params: &BhecIndexCalculusParams,
    report: &mut HecIndexCalculusReport,
) -> Vec<HecRelation> {
    let mut rng = StdRng::seed_from_u64(params.seed ^ 0x5eed_0f_b1a5_0b1e);
    let mut relations = Vec::with_capacity(wanted);

    let (branches, _restart_interval) = match params.search {
        RelationSearch::Random => (0usize, usize::MAX),
        RelationSearch::Walk {
            branches,
            restart_interval,
            ..
        } => (
            branches.max(1).min(fb.len().max(1)),
            restart_interval.max(1),
        ),
        RelationSearch::FactorBaseWalk { branches, .. } => {
            (branches.max(1).min(fb.len().max(1)), usize::MAX)
        }
    };

    let mut fb_steps: Vec<usize> = Vec::new();
    if branches > 0 {
        let mut chosen: Vec<usize> = (0..fb.len()).collect();
        for i in (1..chosen.len()).rev() {
            let j = (rng.next_u64() as usize) % (i + 1);
            chosen.swap(i, j);
        }
        fb_steps = chosen.into_iter().take(branches).collect();
    }

    struct Position {
        a: BigUint,
        b: BigUint,
        r: MumfordDivisor,
        taken: HashMap<usize, i64>,
    }

    let mut current: Option<Position> = None;
    let walking = !matches!(params.search, RelationSearch::Random);
    let d_step_in = match params.search {
        RelationSearch::FactorBaseWalk { d_step_in, .. } => d_step_in.max(2),
        _ => 4,
    };

    while relations.len() < wanted && report.trials < params.max_trials {
        report.trials += 1;
        let started = Instant::now();
        let pos = match current.take() {
            Some(pos) if walking => {
                let mut taken = pos.taken;
                let (mut a, mut b) = (pos.a, pos.b);
                let roll = rng.next_u64() % d_step_in;
                let next = if roll == 0 {
                    a = (a + BigUint::one()) % n;
                    report.jacobian_ops += 1;
                    pos.r.add_fast(d1, curve)
                } else if roll == 1 {
                    b = (b + BigUint::one()) % n;
                    report.jacobian_ops += 1;
                    pos.r.add_fast(d2, curve)
                } else {
                    let idx = fb_steps[(rng.next_u64() as usize) % fb_steps.len()];
                    *taken.entry(idx).or_insert(0) += 1;
                    report.jacobian_ops += 1;
                    pos.r.add_fast(&fb.entries[idx].divisor, curve)
                };
                Position {
                    a,
                    b,
                    r: next,
                    taken,
                }
            }
            _ if walking => {
                // The factor-base walk needs no expensive random start:
                // D1 + D2 already moves both coefficients.
                report.jacobian_ops += 1;
                report.precompute_ops += 1;
                Position {
                    a: BigUint::one(),
                    b: BigUint::one(),
                    r: d1.add_fast(d2, curve),
                    taken: HashMap::new(),
                }
            }
            _ => {
                let a = rand_below(&mut rng, n);
                let b = rand_below(&mut rng, n);
                report.jacobian_ops += 2 * (n.bits() as usize + 1);
                Position {
                    a: a.clone(),
                    b: b.clone(),
                    r: d1
                        .scalar_mul_fast(&a, curve)
                        .add_fast(&d2.scalar_mul_fast(&b, curve), curve),
                    taken: HashMap::new(),
                }
            }
        };
        let pos = pos;
        report.walk_wall_ns += started.elapsed().as_nanos() as u64;

        let (a, b, r) = (pos.a.clone(), pos.b.clone(), pos.r.clone());
        let taken = pos.taken.clone();
        if walking {
            current = Some(pos);
        }

        if r.is_identity() {
            relations.push(HecRelation {
                coef_a: a,
                coef_b: b,
                entries: subtract_steps(Vec::new(), &taken),
            });
            report.smooth_trials += 1;
            continue;
        }

        let oracle_started = Instant::now();
        let classified = decompose_with(curve, &r, fb, params.smoothness);
        report.smoothness_wall_ns += oracle_started.elapsed().as_nanos() as u64;
        match classified {
            Some(entries) => {
                report.smooth_trials += 1;
                relations.push(HecRelation {
                    coef_a: a,
                    coef_b: b,
                    entries: subtract_steps(entries, &taken),
                });
            }
            None => {}
        }
    }
    report.relations = relations.len();
    relations
}

/// Split the trial budget across independent walks and merge the rows.
///
/// The walks are independent, so this needs no locking on the group
/// operations and no shared distinguished-point table: each worker returns
/// a private relation list, and the linear algebra sees the concatenation.
fn collect_relations_parallel(
    curve: &HyperellipticCurve,
    d1: &MumfordDivisor,
    d2: &MumfordDivisor,
    n: &BigUint,
    fb: &BhecFactorBase,
    wanted: usize,
    params: &BhecIndexCalculusParams,
    report: &mut HecIndexCalculusReport,
) -> Vec<HecRelation> {
    let threads = params.threads.max(1).min(wanted.max(1));
    if threads == 1 {
        return collect_relations(curve, d1, d2, n, fb, wanted, params, report);
    }
    let per_thread = wanted / threads + 1;
    let trial_budget = params.max_trials / threads + 1;
    let chunks: Vec<(Vec<HecRelation>, HecIndexCalculusReport)> = std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(threads);
        for t in 0..threads {
            let mut local = params.clone();
            local.seed ^= (t as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
            local.max_trials = trial_budget;
            handles.push(scope.spawn(move || {
                let mut local_report = HecIndexCalculusReport::default();
                let relations =
                    collect_relations(curve, d1, d2, n, fb, per_thread, &local, &mut local_report);
                (relations, local_report)
            }));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().expect("relation worker panicked"))
            .collect()
    });

    let mut relations = Vec::with_capacity(wanted);
    for (rows, local) in chunks {
        report.trials += local.trials;
        report.smooth_trials += local.smooth_trials;
        report.discarded_off_base += local.discarded_off_base;
        report.partial_relations += local.partial_relations;
        report.combined_relations += local.combined_relations;
        report.jacobian_ops += local.jacobian_ops;
        report.precompute_ops += local.precompute_ops;
        report.walk_wall_ns = report.walk_wall_ns.saturating_add(local.walk_wall_ns);
        report.smoothness_wall_ns = report
            .smoothness_wall_ns
            .saturating_add(local.smoothness_wall_ns);
        report.smoothness_field_ops += local.smoothness_field_ops;
        relations.extend(rows);
    }
    report.relations = relations.len();
    relations
}

/// Uniform-ish `BigUint` in `[0, n)`.
fn rand_below(rng: &mut StdRng, n: &BigUint) -> BigUint {
    let bytes = (n.bits() as usize).div_ceil(8) + 8;
    let mut buf = vec![0u8; bytes];
    rng.fill_bytes(&mut buf);
    BigUint::from_bytes_be(&buf) % n
}

fn random_element(rng: &mut StdRng, m: u32) -> F2mElement {
    let words = m.div_ceil(64) as usize;
    let mut buf = vec![0u64; words.max(1)];
    for w in &mut buf {
        *w = rng.next_u64();
    }
    let digits: Vec<u32> = buf
        .iter()
        .flat_map(|w| [*w as u32, (*w >> 32) as u32])
        .collect();
    F2mElement::from_biguint(&BigUint::from_slice(&digits), m)
}

// ── Linear algebra ─────────────────────────────────────────────────────

fn signed_mod(v: i64, n: &BigUint) -> BigUint {
    if v >= 0 {
        BigUint::from(v as u64) % n
    } else {
        (n - BigUint::from(v.unsigned_abs()) % n) % n
    }
}

fn solve_for_logarithm(
    relations: &[HecRelation],
    m: usize,
    n: &BigUint,
    algorithm: &LinearAlgebra,
    report: &mut HecIndexCalculusReport,
) -> Option<BigUint> {
    let mut rows: Vec<Vec<(usize, BigUint)>> = Vec::with_capacity(relations.len());
    let mut rhs: Vec<BigUint> = Vec::with_capacity(relations.len());
    for rel in relations {
        let mut row: BTreeMap<usize, BigUint> = BTreeMap::new();
        for &(j, c) in &rel.entries {
            let e = row.entry(j).or_insert_with(BigUint::zero);
            *e = (&*e + signed_mod(c, n)) % n;
        }
        row.insert(m, (n - &(&rel.coef_b % n)) % n);
        rows.push(row.into_iter().filter(|(_, v)| !v.is_zero()).collect());
        rhs.push(&rel.coef_a % n);
    }

    match algorithm {
        LinearAlgebra::Sparse => {
            let (k, ops) = sparse_solve_for_k(rows, rhs, m, n)?;
            report.solve_row_ops = ops;
            Some(k)
        }
        LinearAlgebra::Dense => {
            let mut matrix: Vec<Vec<BigUint>> = Vec::with_capacity(rows.len());
            for row in &rows {
                let mut dense = vec![BigUint::zero(); m + 1];
                for (j, v) in row {
                    dense[*j] = v.clone();
                }
                matrix.push(dense);
            }
            report.solve_row_ops = rows.len() * (m + 1) * (m + 1);
            let solution = gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, n)?;
            if !solution.determined[m] {
                return None;
            }
            Some(solution.values[m].clone())
        }
    }
}

/// Sparse elimination with Markowitz-lite pivoting.  This is the same
/// algorithm as the prime-field engine's solver, without the bit-op
/// accounting (which is specific to that engine's unit).
fn sparse_solve_for_k(
    mut rows: Vec<Vec<(usize, BigUint)>>,
    mut rhs: Vec<BigUint>,
    m: usize,
    n: &BigUint,
) -> Option<(BigUint, usize)> {
    let mut ops = 0usize;
    let mut col_count = vec![0usize; m + 1];
    let mut holders: Vec<Vec<usize>> = vec![Vec::new(); m + 1];
    for (i, row) in rows.iter().enumerate() {
        for &(j, _) in row {
            col_count[j] += 1;
            holders[j].push(i);
        }
    }
    let mut eliminated = vec![false; rows.len()];
    let has =
        |row: &[(usize, BigUint)], col: usize| row.binary_search_by_key(&col, |&(j, _)| j).ok();

    while let Some(col) = (0..m)
        .filter(|&j| col_count[j] > 0)
        .min_by_key(|&j| col_count[j])
    {
        let mut live = std::mem::take(&mut holders[col]);
        live.retain(|&i| !eliminated[i] && has(&rows[i], col).is_some());
        live.sort_unstable();
        live.dedup();
        let pivot = *live.iter().min_by_key(|&&i| rows[i].len())?;

        let at = has(&rows[pivot], col)?;
        let inv = mod_inverse(&rows[pivot][at].1, n)?;
        for (_, v) in rows[pivot].iter_mut() {
            *v = (&*v * &inv) % n;
            ops += 1;
        }
        rhs[pivot] = (&rhs[pivot] * &inv) % n;
        ops += 1;
        eliminated[pivot] = true;
        let pivot_row = std::mem::take(&mut rows[pivot]);
        let pivot_rhs = rhs[pivot].clone();
        for &(j, _) in &pivot_row {
            col_count[j] -= 1;
        }

        for i in live {
            if i == pivot {
                continue;
            }
            let old = std::mem::take(&mut rows[i]);
            let factor = old[has(&old, col)?].1.clone();
            let mut merged: Vec<(usize, BigUint)> = Vec::with_capacity(old.len() + pivot_row.len());
            let (mut a, mut b) = (0, 0);
            while a < old.len() || b < pivot_row.len() {
                let ja = old.get(a).map_or(usize::MAX, |e| e.0);
                let jb = pivot_row.get(b).map_or(usize::MAX, |e| e.0);
                if ja < jb {
                    merged.push(old[a].clone());
                    a += 1;
                    continue;
                }
                let term = (&factor * &pivot_row[b].1) % n;
                ops += 1;
                let v = if ja == jb {
                    let v = (&old[a].1 + n - &term) % n;
                    a += 1;
                    v
                } else {
                    (n - &term) % n
                };
                if !v.is_zero() {
                    if ja != jb {
                        holders[jb].push(i);
                    }
                    merged.push((jb, v));
                }
                b += 1;
            }
            for &(j, _) in &old {
                col_count[j] -= 1;
            }
            for &(j, _) in &merged {
                col_count[j] += 1;
            }
            rhs[i] = (&rhs[i] + n - &((&factor * &pivot_rhs) % n)) % n;
            ops += 1;
            rows[i] = merged;
        }
    }

    for (i, row) in rows.iter().enumerate() {
        if eliminated[i] {
            continue;
        }
        if row.len() == 1 && row[0].0 == m {
            let inv = mod_inverse(&row[0].1, n)?;
            ops += 1;
            return Some(((&rhs[i] * &inv) % n, ops));
        }
    }
    None
}

// ── End-to-end DLP ─────────────────────────────────────────────────────

/// Solve `D_2 = k·D_1` in `Jac(C)(F_{2^m})` by index calculus.
///
/// `n` must be the prime order of `D_1`; the linear algebra inverts
/// pivots modulo `n`.  The result is verified by scalar multiplication
/// before it is returned.
pub fn binary_hec_index_calculus_dlp(
    curve: &HyperellipticCurve,
    d1: &MumfordDivisor,
    d2: &MumfordDivisor,
    n: &BigUint,
    params: &BhecIndexCalculusParams,
) -> (Option<BigUint>, HecIndexCalculusReport) {
    let mut report = HecIndexCalculusReport::default();
    let fb_size = if params.fb_size == 0 {
        default_factor_base_size(curve)
    } else {
        params.fb_size
    };
    let fb = build_factor_base(curve, fb_size, params.seed);
    report.factor_base_size = fb.len();
    if fb.is_empty() {
        return (None, report);
    }
    let m = fb.len();
    let mut extra = params.extra_relations;
    let mut answer = None;
    for _ in 0..4 {
        let wanted = m + 1 + extra;
        let collected =
            collect_relations_parallel(curve, d1, d2, n, &fb, wanted, params, &mut report);
        if collected.len() < m + 1 {
            return (None, report);
        }
        let started = Instant::now();
        let solved = solve_for_logarithm(&collected, m, n, &params.linear_algebra, &mut report);
        report.solve_wall_ns += started.elapsed().as_nanos() as u64;
        if let Some(k) = solved {
            if d1.scalar_mul_fast(&k, curve) == *d2 {
                answer = Some(k);
                break;
            }
        }
        extra += m / 4 + 8;
    }
    let Some(k) = answer else {
        return (None, report);
    };
    if d1.scalar_mul_fast(&k, curve) == *d2 {
        report.solved = true;
        (Some(k), report)
    } else {
        (None, report)
    }
}

// ── Orders and Hasse interval ──────────────────────────────────────────

/// `#Jac(C)(F_{2^m}) ∈ [(sqrt(q)-1)^{2g}, (sqrt(q)+1)^{2g}]`.
pub fn hasse_interval(curve: &HyperellipticCurve) -> (BigUint, BigUint) {
    let q = BigUint::one() << curve.m as usize;
    let root = q.sqrt();
    let g = curve.genus as u32;
    let lo = if root.is_zero() {
        BigUint::one()
    } else {
        (&root - BigUint::one()).pow(2 * g)
    };
    let hi = (&root + BigUint::one()).pow(2 * g);
    (lo, hi.max(BigUint::one()))
}

type DivisorKey = (Vec<Vec<u8>>, Vec<Vec<u8>>);

fn divisor_key(d: &MumfordDivisor) -> DivisorKey {
    (
        d.u.coeffs
            .iter()
            .map(|c| c.to_biguint().to_bytes_be())
            .collect(),
        d.v.coeffs
            .iter()
            .map(|c| c.to_biguint().to_bytes_be())
            .collect(),
    )
}

/// Exact order of `d`, by BSGS over the Hasse interval.
///
/// Returns `None` when `⌈sqrt(width)⌉` exceeds `max_baby_steps`.
pub fn divisor_order_bsgs(
    curve: &HyperellipticCurve,
    d: &MumfordDivisor,
    max_baby_steps: usize,
) -> Option<BigUint> {
    let (lo, hi) = hasse_interval(curve);
    let width = &hi - &lo;
    let width_u64 = match width.to_u64_digits().as_slice() {
        [] => 0u64,
        [v] => *v,
        _ => return None,
    };
    let steps = (width_u64 as f64).sqrt().ceil() as usize + 1;
    if steps > max_baby_steps {
        return None;
    }

    let neg_d = d.neg_fast(curve);
    let mut table: HashMap<DivisorKey, usize> = HashMap::new();
    let mut acc = MumfordDivisor::identity(curve.m);
    for j in 0..steps {
        table.entry(divisor_key(&acc)).or_insert(j);
        acc = acc.add_fast(&neg_d, curve);
    }

    let stride = d.scalar_mul_fast(&BigUint::from(steps as u64), curve);
    let mut cur = d.scalar_mul_fast(&lo, curve);
    let giant_max = width_u64 as usize / steps + 2;
    for i in 0..=giant_max {
        if let Some(&j) = table.get(&divisor_key(&cur)) {
            let multiple = &lo + BigUint::from(i * steps + j);
            if !multiple.is_zero() && d.scalar_mul_fast(&multiple, curve).is_identity() {
                return Some(exact_order_from_multiple(curve, d, &multiple));
            }
        }
        cur = cur.add_fast(&stride, curve);
    }
    None
}

fn exact_order_from_multiple(
    curve: &HyperellipticCurve,
    d: &MumfordDivisor,
    multiple: &BigUint,
) -> BigUint {
    let mut order = multiple.clone();
    for (q, e) in factorise_small(multiple.clone()) {
        for _ in 0..e {
            let candidate = &order / &q;
            if !candidate.is_zero() && d.scalar_mul_fast(&candidate, curve).is_identity() {
                order = candidate;
            } else {
                break;
            }
        }
    }
    order
}

/// Exact order of `d` given `#Jac(C)`, and only if that order is prime.
pub fn prime_order_of(
    curve: &HyperellipticCurve,
    d: &MumfordDivisor,
    jac_order: &BigUint,
) -> Option<BigUint> {
    let order = exact_order_from_multiple(curve, d, jac_order);
    let factors = factorise_small(order.clone());
    if factors.len() == 1 && factors[0].1 == 1 && order > BigUint::one() {
        Some(order)
    } else {
        None
    }
}

/// Pick a generator of the subgroup of prime order `l`.
pub fn subgroup_generator(
    curve: &HyperellipticCurve,
    fb: &BhecFactorBase,
    jac_order: &BigUint,
    l: &BigUint,
) -> Option<MumfordDivisor> {
    let cofactor = jac_order / l;
    for entry in &fb.entries {
        let d = entry.divisor.scalar_mul_fast(&cofactor, curve);
        if !d.is_identity() {
            return Some(d);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::f2m::IrreduciblePoly;

    fn fe(m: u32, value: u64) -> F2mElement {
        F2mElement::from_biguint(&BigUint::from(value), m)
    }

    fn genus2_f2m10() -> HyperellipticCurve {
        let m = 10;
        let irr = IrreduciblePoly {
            degree: 10,
            low_terms: vec![0, 3],
        };
        // h(x) = x, f(x) = x^5 + x^2 + c x + 1.  The checked constructor
        // rejects non-smooth models, so try a short deterministic family.
        for c in 1u64..64 {
            let h = F2mPoly::from_coeffs(vec![fe(m, 0), F2mElement::one(m)], m);
            let mut fc = vec![F2mElement::zero(m); 6];
            fc[0] = F2mElement::one(m);
            fc[2] = F2mElement::one(m);
            fc[4] = fe(m, c);
            fc[5] = F2mElement::one(m);
            let f = F2mPoly::from_coeffs(fc, m);
            if let Ok(curve) = HyperellipticCurve::try_new(m, irr.clone(), h, f, 2) {
                return curve;
            }
        }
        panic!("no smooth genus-2 model found");
    }

    #[test]
    fn factor_base_points_are_on_the_curve() {
        let curve = genus2_f2m10();
        let fb = build_factor_base(&curve, usize::MAX, 1);
        assert!(fb.len() > 64, "only {} points in F_256", fb.len());
        for entry in &fb.entries {
            assert!(
                curve.is_on_curve(&entry.x, &entry.y),
                "factor-base point is not on the curve: x={:x}, y={:x}",
                entry.x.to_biguint(),
                entry.y.to_biguint()
            );
            assert_eq!(
                fb.index_of_x(&entry.x.to_biguint()),
                Some(fb.entries.iter().position(|e| e.x == entry.x).unwrap())
            );
        }
    }

    #[test]
    fn split_finds_explicit_roots_and_multiplicities() {
        let irr = IrreduciblePoly::deg_8();
        let m = 8;
        let a = fe(m, 3);
        let b = fe(m, 7);
        let c = fe(m, 11);
        let mut p = F2mPoly::one(m);
        for root in [&a, &b, &c] {
            let lin = F2mPoly::from_coeffs(vec![root.clone(), F2mElement::one(m)], m);
            p = p.mul(&lin, &irr);
        }
        let roots = split_into_linear_factors(&p, &irr).expect("splits");
        assert_eq!(roots.len(), 3);
        let mut got: Vec<u64> = roots
            .iter()
            .map(|(r, _)| r.to_biguint().to_u64_digits().first().copied().unwrap_or(0))
            .collect();
        got.sort_unstable();
        let mut want = vec![3u64, 7, 11];
        want.sort_unstable();
        assert_eq!(got, want);

        // (x+a)^2 (x+b)
        let lin_a = F2mPoly::from_coeffs(vec![a.clone(), F2mElement::one(m)], m);
        let lin_b = F2mPoly::from_coeffs(vec![b.clone(), F2mElement::one(m)], m);
        let sq = lin_a.mul(&lin_a, &irr).mul(&lin_b, &irr);
        let roots = split_into_linear_factors(&sq, &irr).expect("splits");
        let mults: HashMap<u64, usize> = roots
            .iter()
            .map(|(r, e)| {
                (
                    r.to_biguint().to_u64_digits().first().copied().unwrap_or(0),
                    *e,
                )
            })
            .collect();
        assert_eq!(mults.get(&3), Some(&2));
        assert_eq!(mults.get(&7), Some(&1));
    }

    #[test]
    fn split_rejects_irreducible_quadratic() {
        let irr = IrreduciblePoly::deg_8();
        let m = 8;
        // x^2 + x + a with no root in F_256: the absolute trace of a
        // decides, so try until one is trace-1.
        for v in 1u64..64 {
            let a = F2mElement::from_biguint(&BigUint::from(v), m);
            let p =
                F2mPoly::from_coeffs(vec![a.clone(), F2mElement::one(m), F2mElement::one(m)], m);
            if split_into_linear_factors(&p, &irr).is_none() {
                return;
            }
        }
        panic!("expected an irreducible quadratic");
    }

    #[test]
    fn hasse_interval_is_ordered() {
        let curve = genus2_f2m10();
        let (lo, hi) = hasse_interval(&curve);
        assert!(lo <= hi);
        assert!(!lo.is_zero());
    }

    #[test]
    fn genus_three_relation_search_recovers_a_scalar() {
        let m = 6;
        let irr = IrreduciblePoly {
            degree: m,
            low_terms: vec![0, 1],
        };
        let h = F2mPoly::from_coeffs(vec![fe(m, 0), F2mElement::one(m)], m);
        let fc = vec![
            fe(m, 1),
            fe(m, 0),
            fe(m, 1),
            fe(m, 0),
            fe(m, 2),
            fe(m, 0),
            fe(m, 0),
            fe(m, 1),
        ];
        let f = F2mPoly::from_coeffs(fc, m);
        let curve = HyperellipticCurve::try_new(m, irr, h, f, 3).expect("genus 3 model");
        let fb = build_factor_base(&curve, 32, 9);
        let mut chosen = None;
        for entry in &fb.entries {
            let Some(order) = divisor_order_bsgs(&curve, &entry.divisor, 10_000) else {
                continue;
            };
            let factors = factorise_small(order.clone());
            let Some((l, _)) = factors.iter().max_by_key(|(q, _)| q.clone()) else {
                continue;
            };
            if l <= &BigUint::from(100u32) {
                continue;
            }
            let d1 = entry.divisor.scalar_mul_fast(&(&order / l), &curve);
            if !d1.is_identity() {
                chosen = Some((d1, l.clone()));
                break;
            }
        }
        let (d1, l) = chosen.expect("a prime-order divisor in genus 3");
        let k = BigUint::from(11u32) % &l;
        let d2 = d1.scalar_mul_fast(&k, &curve);
        let params = BhecIndexCalculusParams {
            fb_size: 32,
            max_trials: 100_000,
            seed: 13,
            threads: 4,
            ..Default::default()
        };
        let (answer, report) = binary_hec_index_calculus_dlp(&curve, &d1, &d2, &l, &params);
        assert_eq!(answer, Some(k), "report: {report:?}");
    }

    #[test]
    fn end_to_end_small_dlp() {
        let curve = genus2_f2m10();
        let fb = build_factor_base(&curve, 64, 7);
        assert!(fb.len() >= 16);
        // Find a base divisor with a non-trivial prime-order multiple.
        let mut chosen = None;
        let mut best_l = BigUint::zero();
        for entry in &fb.entries {
            let Some(order) = divisor_order_bsgs(&curve, &entry.divisor, 4096) else {
                continue;
            };
            let factors = factorise_small(order.clone());
            let Some((l, _)) = factors.iter().max_by_key(|(q, _)| q.clone()) else {
                continue;
            };
            if l > &best_l {
                best_l = l.clone();
            }
            if l < &BigUint::from(8u32) {
                continue;
            }
            let d1 = entry.divisor.scalar_mul_fast(&(&order / l), &curve);
            if d1.is_identity() {
                continue;
            }
            chosen = Some((d1, l.clone()));
            break;
        }
        let (d1, l) =
            chosen.unwrap_or_else(|| panic!("a usable prime-order divisor; best l = {best_l}"));
        let k = BigUint::from(7u32) % &l;
        let d2 = d1.scalar_mul_fast(&k, &curve);
        let params = BhecIndexCalculusParams {
            fb_size: 64,
            max_trials: 20_000,
            seed: 11,
            ..Default::default()
        };
        let (answer, report) = binary_hec_index_calculus_dlp(&curve, &d1, &d2, &l, &params);
        assert_eq!(answer, Some(k.clone()), "report: {report:?}");

        // The independent-walk collector must recover the same scalar.
        let parallel = BhecIndexCalculusParams {
            threads: 4,
            ..params.clone()
        };
        let (answer, report) = binary_hec_index_calculus_dlp(&curve, &d1, &d2, &l, &parallel);
        assert_eq!(answer, Some(k), "parallel report: {report:?}");
    }
}
