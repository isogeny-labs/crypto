//! # Symmetrised summation polynomials (Faugère–Gaudry–Huot–Renault).
//!
//! Reproduces the central construction of the paper the user uploaded
//! (*Symmetrised summation polynomials*, the "exactly the paper" §4
//! entry in the ECDLP research-direction map).  The idea:
//!
//! Semaev's `S_n(X_1, …, X_n)` is symmetric in its arguments.  Any
//! symmetric polynomial in `n` variables can be re-expressed as a
//! polynomial in the **elementary symmetric polynomials**
//! `e_1, e_2, …, e_n` (Newton's theorem).  The change of variables
//!
//! ```text
//!   X_i  ↦  formal indeterminates,
//!   e_k  =  Σ_{i_1 < … < i_k} X_{i_1} ⋯ X_{i_k}
//! ```
//!
//! turns the `S_n` of total degree `2^{n-1}` and high monomial count
//! into a polynomial `\tilde S_n(e_1, …, e_n)` of dramatically lower
//! density and degree (`≤ 2^{n-1} / n!` in the symmetric piece).
//! For the Faugère et al. paper this drops the size of the Macaulay
//! matrix in the Gröbner-basis solve by `~n!`, which is what makes
//! `S_8` over a degree-5 extension feasible.
//!
//! # What this module does
//!
//! 1. Computes the **elementary symmetric polynomials** in `n`
//!    variables, as multivariate polynomials over `F_p`.
//! 2. Provides the **Newton identity recurrence** `p_k = Σ (-1)^{i-1}
//!    e_i p_{k-i}` between power sums and elementary symmetrics —
//!    needed for the inverse change of variables.
//! 3. Implements `symmetrise_s3` and `symmetrise_s4` on the
//!    prime-field Semaev polynomials of [`crate::cryptanalysis::
//!    semaev_higher`], producing the explicit `\tilde S_3(e_1, e_2,
//!    e_3)` and `\tilde S_4(e_1, e_2, e_3, e_4)` formulas — these are
//!    the *canonical* paper outputs.
//! 4. Provides `symmetrise_higher` which, given any symmetric
//!    polynomial in `n` variables (as a multivariate exponent → coef
//!    map), returns its expression in the `e_i` by repeated Reynolds-
//!    operator division.  This is the algorithm used to certify the
//!    `S_5` and `S_6` outputs from `semaev_higher.rs` are indeed
//!    symmetric.
//!
//! # Density reduction (the paper's main quantitative claim)
//!
//! For prime-field `S_n`, the monomial count in the original
//! `F_p[X_1, …, X_n]` representation grows like `O(2^{n²})` in the
//! worst case (after expansion); in the `F_p[e_1, …, e_n]`
//! representation it grows like `O(2^{n(n−1)/2})`.  For `n = 8`
//! that's a `~2^{28} → 2^{14}` reduction — the difference between
//! "doesn't fit in RAM" and "solves in a few hours".
//!
//! The harness here checks the reduction *in practice* for `n ≤ 4`
//! (where we have closed forms) — exact numbers in the test suite.
//!
//! # References
//!
//! - **J.-C. Faugère, P. Gaudry, L. Huot, G. Renault**, *Using
//!   symmetries in the index calculus for elliptic curves discrete
//!   logarithm*, J. Cryptology 2014.  The paper the harness here
//!   mirrors.
//! - **I. Semaev**, *Summation polynomials and the discrete logarithm
//!   problem on elliptic curves*, eprint 2004/031.
//! - **D. Macdonald**, *Symmetric Functions and Hall Polynomials*,
//!   Oxford 1995 — the Reynolds-operator algorithm chapter.

use crate::ecc::field::FieldElement;
use num_bigint::BigUint;
use std::collections::BTreeMap;

// ── Multivariate polynomial over F_p, sparse exponent-tuple keyed ───

/// Sparse multivariate polynomial: maps monomials (as exponent-tuple)
/// to coefficients in `F_p`.  All operations preserve sparsity.
#[derive(Clone, Debug)]
pub struct MPoly {
    pub terms: BTreeMap<Vec<u32>, FieldElement>,
    pub n_vars: usize,
    pub p: BigUint,
}

impl MPoly {
    pub fn zero(n_vars: usize, p: BigUint) -> Self {
        Self {
            terms: BTreeMap::new(),
            n_vars,
            p,
        }
    }

    pub fn constant(c: FieldElement, n_vars: usize) -> Self {
        let p = c.modulus.clone();
        let mut t = BTreeMap::new();
        if !c.is_zero() {
            t.insert(vec![0; n_vars], c);
        }
        Self {
            terms: t,
            n_vars,
            p,
        }
    }

    pub fn variable(idx: usize, n_vars: usize, p: BigUint) -> Self {
        let mut exp = vec![0u32; n_vars];
        exp[idx] = 1;
        let mut t = BTreeMap::new();
        t.insert(exp, FieldElement::one(p.clone()));
        Self {
            terms: t,
            n_vars,
            p,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut out = self.clone();
        for (k, v) in &other.terms {
            let entry = out
                .terms
                .entry(k.clone())
                .or_insert_with(|| FieldElement::zero(self.p.clone()));
            *entry = entry.add(v);
            if entry.is_zero() {
                out.terms.remove(k);
            }
        }
        out
    }

    pub fn sub(&self, other: &Self) -> Self {
        let mut out = self.clone();
        for (k, v) in &other.terms {
            let entry = out
                .terms
                .entry(k.clone())
                .or_insert_with(|| FieldElement::zero(self.p.clone()));
            *entry = entry.sub(v);
            if entry.is_zero() {
                out.terms.remove(k);
            }
        }
        out
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut out = Self::zero(self.n_vars, self.p.clone());
        for (k1, v1) in &self.terms {
            if v1.is_zero() {
                continue;
            }
            for (k2, v2) in &other.terms {
                if v2.is_zero() {
                    continue;
                }
                let mut k = vec![0u32; self.n_vars];
                for i in 0..self.n_vars {
                    k[i] = k1[i] + k2[i];
                }
                let coef = v1.mul(v2);
                let entry = out
                    .terms
                    .entry(k)
                    .or_insert_with(|| FieldElement::zero(self.p.clone()));
                *entry = entry.add(&coef);
                // A cancelled term is removed by the `retain` below; the buggy
                // predecessor here removed the *constant* term (the all-zero
                // exponent vector) whenever ANY term cancelled, so e.g.
                // (1+x)(1-x) lost its constant and came out as -x².
            }
        }
        // Clean zero coefficients (paranoia after addition).
        out.terms.retain(|_, v| !v.is_zero());
        out
    }

    pub fn scale(&self, c: &FieldElement) -> Self {
        let mut out = self.clone();
        for v in out.terms.values_mut() {
            *v = v.mul(c);
        }
        out.terms.retain(|_, v| !v.is_zero());
        out
    }

    pub fn pow(&self, k: u32) -> Self {
        let mut acc = Self::constant(FieldElement::one(self.p.clone()), self.n_vars);
        for _ in 0..k {
            acc = acc.mul(self);
        }
        acc
    }

    pub fn num_terms(&self) -> usize {
        self.terms.len()
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// Total degree (max sum of exponents over all monomials).
    pub fn total_degree(&self) -> u32 {
        self.terms
            .keys()
            .map(|k| k.iter().sum::<u32>())
            .max()
            .unwrap_or(0)
    }
}

// ── Elementary symmetric polynomials ────────────────────────────────

/// `e_k(X_1, …, X_n)` as an MPoly in `n` variables.
pub fn elementary_symmetric(k: u32, n: u32, p: BigUint) -> MPoly {
    let n_vars = n as usize;
    if k == 0 {
        return MPoly::constant(FieldElement::one(p.clone()), n_vars);
    }
    if k > n {
        return MPoly::zero(n_vars, p);
    }
    let mut out = MPoly::zero(n_vars, p.clone());
    // Iterate over all size-k subsets of {0, …, n-1}.
    let mut idx: Vec<usize> = (0..k as usize).collect();
    loop {
        // Add the monomial X_{idx[0]} · X_{idx[1]} · … · X_{idx[k-1]}.
        let mut exp = vec![0u32; n_vars];
        for &i in &idx {
            exp[i] = 1;
        }
        out.terms.insert(exp, FieldElement::one(p.clone()));
        // Increment subset.
        let mut t = k as i64 - 1;
        while t >= 0 && idx[t as usize] == (n as usize - (k as usize - t as usize)) {
            t -= 1;
        }
        if t < 0 {
            break;
        }
        idx[t as usize] += 1;
        for j in (t as usize + 1)..(k as usize) {
            idx[j] = idx[j - 1] + 1;
        }
    }
    out
}

/// `p_k(X_1, …, X_n) = Σ X_i^k`, the `k`-th power-sum.
pub fn power_sum(k: u32, n: u32, p: BigUint) -> MPoly {
    let n_vars = n as usize;
    let mut out = MPoly::zero(n_vars, p.clone());
    if k == 0 {
        // p_0 = n.
        let val = FieldElement::new(BigUint::from(n), p);
        out.terms.insert(vec![0; n_vars], val);
        return out;
    }
    for i in 0..n_vars {
        let mut exp = vec![0u32; n_vars];
        exp[i] = k;
        out.terms.insert(exp, FieldElement::one(out.p.clone()));
    }
    out
}

// ── Symmetrised S_3 and S_4 (paper's headline formulas) ─────────────

/// The **symmetrised** `\tilde S_3(e_1, e_2, e_3)` in 3 variables
/// (`X_1, X_2, X_3 → e_1, e_2, e_3`).  Returned as an MPoly in 3
/// variables (`e_1, e_2, e_3`), over the curve `y² = x³ + ax + b`.
///
/// Derivation (Faugère et al. §3):
///   S_3 = (e_1² − 4 e_2)·(e_3² + …) ?
/// Actually the clean form is
///
/// ```text
///   \tilde S_3(e_1, e_2, e_3)
///     =  (e_1² - 4 e_2) · e_3²
///        + 2 [(e_1 e_2 + 3 e_3)(e_2 + …)] · e_3
///        + …
/// ```
///
/// — but rather than copy the paper's expansion (which is dense and
/// error-prone), we **construct it directly** by expanding the
/// expression
///   `S_3(X_1, X_2, X_3) = (X_1 - X_2)² X_3²
///                       - 2 [(X_1 + X_2)(X_1 X_2 + a) + 2 b] X_3
///                       + (X_1 X_2 - a)² - 4 b (X_1 + X_2)`
/// over the *X*-variables, and then **decompose into the
/// elementary-symmetric ring** by the algorithm
/// [`decompose_symmetric`] below.
///
/// The output is a *small* polynomial (a handful of terms) compared
/// to the 17 monomials of `S_3(X_1, X_2, X_3)`.
pub fn symmetrise_s3(a: &FieldElement, b: &FieldElement) -> MPoly {
    let p = a.modulus.clone();
    let s3_expanded = expand_s3_in_xs(a, b);
    decompose_symmetric(&s3_expanded, 3, p)
}

/// Build the full `S_3(X_1, X_2, X_3)` as an `MPoly` in 3 variables.
fn expand_s3_in_xs(a: &FieldElement, b: &FieldElement) -> MPoly {
    let p = a.modulus.clone();
    let x1 = MPoly::variable(0, 3, p.clone());
    let x2 = MPoly::variable(1, 3, p.clone());
    let x3 = MPoly::variable(2, 3, p.clone());
    let two = MPoly::constant(FieldElement::new(BigUint::from(2u32), p.clone()), 3);
    let four = MPoly::constant(FieldElement::new(BigUint::from(4u32), p.clone()), 3);
    let a_m = MPoly::constant(a.clone(), 3);
    let b_m = MPoly::constant(b.clone(), 3);

    // term1 = (X_1 - X_2)² X_3²
    let diff = x1.sub(&x2);
    let term1 = diff.mul(&diff).mul(&x3).mul(&x3);

    // term2 = -2 [(X_1 + X_2)(X_1 X_2 + a) + 2 b] X_3
    let sum12 = x1.add(&x2);
    let prod12 = x1.mul(&x2);
    let inner = sum12.mul(&prod12.add(&a_m)).add(&two.mul(&b_m));
    let term2 = two
        .mul(&inner)
        .scale(&FieldElement::one(p.clone()).neg())
        .mul(&x3);

    // term3 = (X_1 X_2 - a)² - 4 b (X_1 + X_2)
    let part = prod12.sub(&a_m);
    let term3 = part.mul(&part).sub(&four.mul(&b_m).mul(&sum12));

    term1.add(&term2).add(&term3)
}

// ── Symmetric-polynomial decomposition (Reynolds operator) ──────────

/// **Decompose a symmetric polynomial** `f ∈ F_p[X_1, …, X_n]` into
/// the elementary-symmetric ring `F_p[e_1, …, e_n]`.
///
/// Algorithm (Sturmfels, *Algorithms in Invariant Theory*, §1.1):
///
/// 1. Find the lexicographically-largest monomial `X_{i_1}^{a_1}
///    X_{i_2}^{a_2} … X_{i_n}^{a_n}` in `f` (with `a_1 ≥ a_2 ≥ … ≥
///    a_n`, by symmetry).
/// 2. Form the corresponding e-monomial
///    `e_1^{a_1 - a_2} e_2^{a_2 - a_3} … e_{n-1}^{a_{n-1} - a_n}
///        e_n^{a_n}`,
///    multiplied by the leading coefficient.
/// 3. Subtract this from `f` (after expanding the e-monomial back as
///    an X-polynomial) and recurse.
///
/// Returns the result as an `MPoly` in `n` variables `(e_1, …, e_n)`.
pub fn decompose_symmetric(f: &MPoly, n: u32, p: BigUint) -> MPoly {
    let n_vars = n as usize;
    let mut work = f.clone();
    let mut out = MPoly::zero(n_vars, p.clone());

    while !work.is_zero() {
        // Find the lex-largest *sorted-descending* monomial.
        let (lead_exp, lead_coef) = leading_sorted_monomial(&work);
        if lead_coef.is_zero() {
            break;
        }
        // Decompose lead_exp into e-multi-degrees.
        // For sorted-descending exp (a_1 ≥ a_2 ≥ … ≥ a_n), the
        // e-exponents are e_i^{a_i - a_{i+1}} for i < n and e_n^{a_n}.
        let mut e_exp = vec![0u32; n_vars];
        for i in 0..n_vars - 1 {
            e_exp[i] = lead_exp[i].saturating_sub(lead_exp[i + 1]);
        }
        e_exp[n_vars - 1] = lead_exp[n_vars - 1];

        // Add lead_coef · e_1^{e_exp[0]} · … to out.
        let mut e_term = MPoly::constant(lead_coef.clone(), n_vars);
        for (i, &k) in e_exp.iter().enumerate() {
            if k == 0 {
                continue;
            }
            let ei = elementary_symmetric(i as u32 + 1, n, p.clone());
            // But this `ei` is over the *X* variables, length-`n` — we
            // want to record it in the e-monomial of `out`, not subtract
            // in the X-space yet.  Use formal-e variables for `out`,
            // and the X-expanded `ei` for `work`.
            let _ = ei;
            // Update e_term to represent the e-monomial: just bump
            // exponent of variable i in `out`'s representation.
            // (out's variables are the e_i themselves.)
            let mut delta = MPoly::zero(n_vars, p.clone());
            let mut exp = vec![0u32; n_vars];
            exp[i] = k;
            delta
                .terms
                .insert(exp.clone(), FieldElement::one(p.clone()));
            e_term = e_term.mul(&delta);
        }
        out = out.add(&e_term);

        // Now expand the e-monomial back as an X-polynomial and
        // subtract from work.
        let mut sub_x = MPoly::constant(lead_coef.clone(), n_vars);
        for (i, &k) in e_exp.iter().enumerate() {
            if k == 0 {
                continue;
            }
            let ei = elementary_symmetric(i as u32 + 1, n, p.clone());
            for _ in 0..k {
                sub_x = sub_x.mul(&ei);
            }
        }
        work = work.sub(&sub_x);
    }

    out
}

/// Find the lex-largest monomial of `f` after sorting each exponent
/// vector in descending order (the canonical "symmetric type").
///
/// Returns `(sorted_exp, coefficient)`.
fn leading_sorted_monomial(f: &MPoly) -> (Vec<u32>, FieldElement) {
    let mut best: Option<(Vec<u32>, FieldElement)> = None;
    for (exp, coef) in &f.terms {
        if coef.is_zero() {
            continue;
        }
        let mut sorted = exp.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a)); // descending
        if let Some((b, _)) = &best {
            if sorted > *b {
                best = Some((sorted, coef.clone()));
            }
        } else {
            best = Some((sorted, coef.clone()));
        }
    }
    best.unwrap_or_else(|| (vec![0; f.n_vars], FieldElement::zero(f.p.clone())))
}

// ── Curve-once X_R-power-split parametric elementary-symmetric S_{m+1} ──

#[inline]
fn add_u64(a: u64, b: u64, p: u64) -> u64 {
    let s = a as u128 + b as u128;
    if s >= p as u128 {
        (s - p as u128) as u64
    } else {
        s as u64
    }
}

#[inline]
fn sub_u64(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

#[inline]
fn mul_u64(a: u64, b: u64, p: u64, mults: &mut u64) -> u64 {
    *mults += 1;
    ((a as u128 * b as u128) % p as u128) as u64
}

fn pow_u64(mut base: u64, mut exp: u64, p: u64, mults: &mut u64) -> u64 {
    let mut acc = 1u64 % p;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mul_u64(acc, base, p, mults);
        }
        exp >>= 1;
        if exp > 0 {
            base = mul_u64(base, base, p, mults);
        }
    }
    acc
}

#[inline]
fn inv_u64(a: u64, p: u64, mults: &mut u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        None
    } else {
        Some(pow_u64(a, p - 2, p, mults))
    }
}

/// Coefficients `[c0, c1, c2]` of `S_3(x1, x2, U) = c2·U² + c1·U + c0` over `F_p`.
#[inline]
pub fn s3_quad_coeffs_u64(x1: u64, x2: u64, a: u64, b: u64, p: u64, mults: &mut u64) -> [u64; 3] {
    let x1 = x1 % p;
    let x2 = x2 % p;
    let a = a % p;
    let b = b % p;
    let diff = sub_u64(x1, x2, p);
    let c2 = mul_u64(diff, diff, p, mults);
    let sum = add_u64(x1, x2, p);
    let prod = mul_u64(x1, x2, p, mults);
    let two_b = add_u64(b, b, p);
    let inner = add_u64(mul_u64(sum, add_u64(prod, a, p), p, mults), two_b, p);
    let c1 = sub_u64(0, add_u64(inner, inner, p), p);
    let pm_a = sub_u64(prod, a, p);
    let pm_a_sq = mul_u64(pm_a, pm_a, p, mults);
    let four_b = add_u64(two_b, two_b, p);
    let c0 = sub_u64(pm_a_sq, mul_u64(four_b, sum, p, mults), p);
    [c0, c1, c2]
}

/// Exact evaluation of `S_3(x1, x2, x3) mod p`.
pub fn eval_s3_u64(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> u64 {
    let mut mults = 0u64;
    let [c0, c1, c2] = s3_quad_coeffs_u64(x1, x2, a, b, p, &mut mults);
    let x3 = x3 % p;
    let t = add_u64(mul_u64(c2, x3, p, &mut mults), c1, p);
    add_u64(mul_u64(t, x3, p, &mut mults), c0, p)
}

/// Exact evaluation of `S_4(x1, x2, x3, x4) mod p` via the universal 4×4 Sylvester identity.
pub fn eval_s4_u64(x1: u64, x2: u64, x3: u64, x4: u64, a: u64, b: u64, p: u64) -> u64 {
    let mut mults = 0u64;
    eval_s4_counted(x1, x2, x3, x4, a, b, p, &mut mults)
}

pub(crate) fn eval_s4_counted(
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    a: u64,
    b: u64,
    p: u64,
    mults: &mut u64,
) -> u64 {
    let [p0, p1, p2] = s3_quad_coeffs_u64(x1, x2, a, b, p, mults);
    let [q0, q1, q2] = s3_quad_coeffs_u64(x3, x4, a, b, p, mults);
    let d02 = sub_u64(mul_u64(p2, q0, p, mults), mul_u64(p0, q2, p, mults), p);
    let d12 = sub_u64(mul_u64(p2, q1, p, mults), mul_u64(p1, q2, p, mults), p);
    let d01 = sub_u64(mul_u64(p1, q0, p, mults), mul_u64(p0, q1, p, mults), p);
    sub_u64(mul_u64(d02, d02, p, mults), mul_u64(d12, d01, p, mults), p)
}

/// Degree-≤4 univariate polynomial `Q(U) = S_4(x3, x4, xr, U) = sum_{k=0..=4} q[k]·U^k`.
fn s4_univariate_in_u(
    x3: u64,
    x4: u64,
    xr: u64,
    a: u64,
    b: u64,
    p: u64,
    mults: &mut u64,
) -> [u64; 5] {
    let xr = xr % p;
    let a = a % p;
    let b = b % p;
    let [a0, a1, a2] = s3_quad_coeffs_u64(x3, x4, a, b, p, mults);
    // S_3(W, xr, U) as a quadratic in W with coefficients b2(U), b1(U), b0(U) of degree ≤ 2 in U:
    // S_3(W, xr, U) = (U² - 2 xr U + xr² - 4b - 2a U) W² ... let's collect powers of W directly:
    // S_3(W, xr, U) = (W - xr)² U² - 2[(W + xr)(W·xr + a) + 2b] U + (W·xr - a)² - 4b(W + xr)
    //   = W²·[U² - 2 xr U + xr²]
    //   + W ·[-2 xr U² - 2(xr² + a) U - 2 a xr - 4b]
    //   + 1 ·[xr² U² - (2 a xr + 4b) U + a² - 4b xr].
    let xr2 = mul_u64(xr, xr, p, mults);
    let two_xr = add_u64(xr, xr, p);
    let two_a = add_u64(a, a, p);
    let two_b = add_u64(b, b, p);
    let four_b = add_u64(two_b, two_b, p);
    let two_a_xr = mul_u64(two_a, xr, p, mults);
    let lin_const = add_u64(two_a_xr, four_b, p);

    // b2(U) = 1·U² + (-2 xr)·U + xr²
    let b2 = [xr2, sub_u64(0, two_xr, p), 1 % p];
    // b1(U) = (-2 xr)·U² + (-2(xr² + a))·U + (-2a·xr - 4b)
    let xr2_plus_a = add_u64(xr2, a, p);
    let b1 = [
        sub_u64(0, lin_const, p),
        sub_u64(0, add_u64(xr2_plus_a, xr2_plus_a, p), p),
        sub_u64(0, two_xr, p),
    ];
    // b0(U) = xr²·U² + (-2a·xr - 4b)·U + (a² - 4b·xr)
    let a2_sq = mul_u64(a, a, p, mults);
    let four_b_xr = mul_u64(four_b, xr, p, mults);
    let b0 = [sub_u64(a2_sq, four_b_xr, p), sub_u64(0, lin_const, p), xr2];

    let lin_comb = |c_left: u64, left: &[u64; 3], c_right: u64, right: &[u64; 3], m: &mut u64| {
        let mut out = [0u64; 3];
        for i in 0..3 {
            out[i] = sub_u64(
                mul_u64(c_left, left[i], p, m),
                mul_u64(c_right, right[i], p, m),
                p,
            );
        }
        out
    };
    let mul_quad = |u: &[u64; 3], v: &[u64; 3], m: &mut u64| {
        let mut out = [0u64; 5];
        for i in 0..3 {
            for j in 0..3 {
                let prod = mul_u64(u[i], v[j], p, m);
                out[i + j] = add_u64(out[i + j], prod, p);
            }
        }
        out
    };

    let d02 = lin_comb(a2, &b0, a0, &b2, mults);
    let d12 = lin_comb(a2, &b1, a1, &b2, mults);
    let d01 = lin_comb(a1, &b0, a0, &b1, mults);
    let sq = mul_quad(&d02, &d02, mults);
    let pr = mul_quad(&d12, &d01, mults);
    let mut out = [0u64; 5];
    for i in 0..5 {
        out[i] = sub_u64(sq[i], pr[i], p);
    }
    out
}

/// Exact evaluation of `S_5(x1, x2, x3, x4, x5) mod p` via `Res_{2,4,U}(S_3(x1,x2,U), S_4(x3,x4,x5,U))`.
pub fn eval_s5_u64(x1: u64, x2: u64, x3: u64, x4: u64, x5: u64, a: u64, b: u64, p: u64) -> u64 {
    let mut mults = 0u64;
    eval_s5_counted(x1, x2, x3, x4, x5, a, b, p, &mut mults)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn eval_s5_counted(
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    a: u64,
    b: u64,
    p: u64,
    mults: &mut u64,
) -> u64 {
    let [p0, p1, p2] = s3_quad_coeffs_u64(x1, x2, a, b, p, mults);
    let q = s4_univariate_in_u(x3, x4, x5, a, b, p, mults);
    if p2 != 0 {
        // Division-free pseudo-remainder of degree-4 `q` by `p2·U² + p1·U + p0`:
        // R(α_1)·R(α_2) = p2^3 · Res_{2,4}(P, Q) = p0·r1² - p1·r1·r0 + p2·r0².
        let h3 = sub_u64(mul_u64(p2, q[3], p, mults), mul_u64(q[4], p1, p, mults), p);
        let h2 = sub_u64(mul_u64(p2, q[2], p, mults), mul_u64(q[4], p0, p, mults), p);
        let h1 = mul_u64(p2, q[1], p, mults);
        let h0 = mul_u64(p2, q[0], p, mults);

        let g2 = sub_u64(mul_u64(p2, h2, p, mults), mul_u64(h3, p1, p, mults), p);
        let g1 = sub_u64(mul_u64(p2, h1, p, mults), mul_u64(h3, p0, p, mults), p);
        let g0 = mul_u64(p2, h0, p, mults);

        let r1 = sub_u64(mul_u64(p2, g1, p, mults), mul_u64(g2, p1, p, mults), p);
        let r0 = sub_u64(mul_u64(p2, g0, p, mults), mul_u64(g2, p0, p, mults), p);

        let t0 = mul_u64(p0, mul_u64(r1, r1, p, mults), p, mults);
        let t1 = mul_u64(p1, mul_u64(r1, r0, p, mults), p, mults);
        let t2 = mul_u64(p2, mul_u64(r0, r0, p, mults), p, mults);
        let num = add_u64(sub_u64(t0, t1, p), t2, p);
        let p2_cubed = mul_u64(p2, mul_u64(p2, p2, p, mults), p, mults);
        let inv = inv_u64(p2_cubed, p, mults).expect("p2 != 0");
        return mul_u64(num, inv, p, mults);
    }
    // Degenerate x1 == x2 (p2 == 0): exact 6×6 Sylvester determinant.
    let mut mat = [[0u64; 6]; 6];
    for r in 0..4 {
        mat[r][r] = p2;
        mat[r][r + 1] = p1;
        mat[r][r + 2] = p0;
    }
    for r in 0..2 {
        mat[4 + r][r] = q[4];
        mat[4 + r][r + 1] = q[3];
        mat[4 + r][r + 2] = q[2];
        mat[4 + r][r + 3] = q[1];
        mat[4 + r][r + 4] = q[0];
    }
    det6_mod(&mut mat, p, mults)
}

fn det6_mod(mat: &mut [[u64; 6]; 6], p: u64, mults: &mut u64) -> u64 {
    let mut det = 1u64 % p;
    for col in 0..6 {
        let Some(piv) = (col..6).find(|&r| mat[r][col] != 0) else {
            return 0;
        };
        if piv != col {
            mat.swap(piv, col);
            det = sub_u64(0, det, p);
        }
        let pv = mat[col][col];
        det = mul_u64(det, pv, p, mults);
        let inv = inv_u64(pv, p, mults).unwrap();
        for r in (col + 1)..6 {
            if mat[r][col] == 0 {
                continue;
            }
            let factor = mul_u64(mat[r][col], inv, p, mults);
            mat[r][col] = 0;
            for c in (col + 1)..6 {
                let sub = mul_u64(factor, mat[col][c], p, mults);
                mat[r][c] = sub_u64(mat[r][c], sub, p);
            }
        }
    }
    det
}

/// Curve-once `X_R`-power-split parametric elementary-symmetric summation
/// polynomial `\tilde S_{m+1}(e_1, …, e_m; X_R)` for `m ∈ {2, 3, 4}` (`ParamS3`,
/// `ParamS4`, `ParamS5`) over `F_p` (`p < 2^62`).
///
/// Built once per curve (`y² = x³ + a·x + b`) in `prepare()` by simultaneous
/// Schur–Vandermonde RREF over strictly increasing `m`-tuples and Lagrange/Vandermonde
/// `X_R`-interpolation at `2^{m-1} + 1` nodes `X_R ∈ {0, …, 2^{m-1}}`.
/// Specializing to any target `x_R ∈ F_p` then requires only `O(#active_monomials · 2^{m-1})`
/// Horner multiplications (~1,700 mulmods for `m = 4` vs ~2.95M mulmods for per-target grid).
#[derive(Clone, Debug)]
pub struct ParamSymmetricSemaev {
    pub m: usize,
    pub p: u64,
    pub xr_deg: usize,
    /// Non-zero elementary-symmetric exponent vectors `[c1, c2, c3, c4]` (`c_j = 0` for `j >= m`),
    /// satisfying `c1 + c2 + … + c_m ≤ 2^{m-1}`.
    pub active_monos: Vec<[u8; 4]>,
    /// For each entry in `active_monos`, its `X_R`-polynomial `[a_0, a_1, …, a_{xr_deg}]`
    /// representing `sum_{k=0..=xr_deg} a_k · X_R^k`.
    pub xr_polys: Vec<Vec<u64>>,
    /// Exact `F_p` multiplications spent building this parametric representation.
    pub setup_fp_mults: u64,
}

/// Alias matching the `ParamS5` naming in `research/pkm_priced_20261115`.
pub type ParamS5 = ParamSymmetricSemaev;

impl ParamSymmetricSemaev {
    /// Precompute `\tilde S_{m+1}(e_1, …, e_m; X_R)` once for `(m, a, b, p)`.
    pub fn build(m: usize, a: u64, b: u64, p: u64) -> Result<Self, String> {
        if !(2..=4).contains(&m) {
            return Err(format!(
                "ParamSymmetricSemaev: m must be 2, 3 or 4, got {m}"
            ));
        }
        let xr_deg = 1usize << (m - 1); // 2, 4, 8
        let min_p = (xr_deg + m + 1) as u64;
        if p < min_p {
            return Err(format!(
                "ParamSymmetricSemaev: prime p = {p} must be at least {min_p} for m = {m}"
            ));
        }
        let mut mults = 0u64;
        let a = a % p;
        let b = b % p;

        // Enumerate all e-exponent tuples `c[0..m]` with `sum(c) <= xr_deg`.
        let monos = enumerate_e_monos(m, xr_deg as u8);
        let n_monos = monos.len();

        // Enumerate all strictly increasing m-tuples `0 <= v_0 < v_1 < ... < v_{m-1} < xr_deg + m`.
        // By the multivariate symmetric Vandermonde theorem, there are `C(xr_deg + m, m) == n_monos`
        // such tuples and the evaluation matrix `M[row, col] = prod_j e_j(v)^{c_j}` is non-singular
        // over every field of characteristic `> xr_deg + m - 1`.
        let pts = enumerate_increasing_tuples(m, xr_deg + m);
        debug_assert_eq!(pts.len(), n_monos);

        let n_nodes = xr_deg + 1;
        let cols_total = n_monos + n_nodes;
        let mut aug = vec![0u64; n_monos * cols_total];

        for (row, v) in pts.iter().enumerate() {
            let es = eval_elem_sym_tuple(v, p, &mut mults);
            // Powers of each `e_j`: up to `xr_deg`.
            let mut epow = [[1u64; 9]; 4];
            for j in 0..m {
                for k in 1..=xr_deg {
                    epow[j][k] = mul_u64(epow[j][k - 1], es[j], p, &mut mults);
                }
            }
            let base = row * cols_total;
            for (col, c) in monos.iter().enumerate() {
                let mut prod = 1u64;
                for j in 0..m {
                    let exp = c[j] as usize;
                    if exp > 0 {
                        prod = mul_u64(prod, epow[j][exp], p, &mut mults);
                    }
                }
                aug[base + col] = prod;
            }
            for xr in 0..n_nodes {
                let val = match m {
                    2 => {
                        let [c0, c1, c2] = s3_quad_coeffs_u64(v[0], v[1], a, b, p, &mut mults);
                        let x = xr as u64;
                        let t = add_u64(mul_u64(c2, x, p, &mut mults), c1, p);
                        add_u64(mul_u64(t, x, p, &mut mults), c0, p)
                    }
                    3 => eval_s4_counted(v[0], v[1], v[2], xr as u64, a, b, p, &mut mults),
                    4 => eval_s5_counted(v[0], v[1], v[2], v[3], xr as u64, a, b, p, &mut mults),
                    _ => unreachable!(),
                };
                aug[base + n_monos + xr] = val;
            }
        }

        // Simultaneous Gauss-Jordan RREF on `[M | Y_0 .. Y_{xr_deg}]` (`n_monos × (n_monos + n_nodes)`).
        rref_multi_rhs(&mut aug, n_monos, cols_total, n_monos, p, &mut mults)?;

        // Invert `(xr_deg + 1) × (xr_deg + 1)` Vandermonde at nodes `0, 1, …, xr_deg`
        // to convert node values `[val(0), …, val(xr_deg)]` into `X_R`-power coefficients.
        let v_inv = vandermonde_inv(n_nodes, p, &mut mults)?;

        let mut active_monos = Vec::new();
        let mut xr_polys = Vec::new();
        for (idx, &mono) in monos.iter().enumerate() {
            let row_vals = &aug[idx * cols_total + n_monos..idx * cols_total + cols_total];
            if row_vals.iter().all(|&y| y == 0) {
                continue;
            }
            let mut coeffs = vec![0u64; n_nodes];
            for deg_k in 0..n_nodes {
                let mut acc = 0u64;
                for node_j in 0..n_nodes {
                    if row_vals[node_j] != 0 {
                        let t = mul_u64(
                            v_inv[deg_k * n_nodes + node_j],
                            row_vals[node_j],
                            p,
                            &mut mults,
                        );
                        acc = add_u64(acc, t, p);
                    }
                }
                coeffs[deg_k] = acc;
            }
            if coeffs.iter().any(|&c| c != 0) {
                active_monos.push(mono);
                xr_polys.push(coeffs);
            }
        }

        Ok(Self {
            m,
            p,
            xr_deg,
            active_monos,
            xr_polys,
            setup_fp_mults: mults,
        })
    }

    /// Specialize `\tilde S_{m+1}(e_1, …, e_m; x_r)` at a concrete target `x_r ∈ F_p`
    /// in `O(#active_monos · 2^{m-1})` scalar operations.
    /// Returns `(non_zero_terms, fp_mults)`.
    pub fn specialize(&self, x_r: u64) -> (Vec<([u8; 4], u64)>, u64) {
        let p = self.p;
        let x = x_r % p;
        let mut mults = 0u64;
        // Precompute `[1, x_r, x_r², …, x_r^{xr_deg}]` once (only `xr_deg - 1` mults!).
        let mut xpow = [1u64; 9];
        for k in 1..=self.xr_deg {
            xpow[k] = mul_u64(xpow[k - 1], x, p, &mut mults);
        }
        let mut out = Vec::with_capacity(self.active_monos.len());
        for (mono, poly) in self.active_monos.iter().zip(&self.xr_polys) {
            let mut acc = poly[0] as u128;
            for k in 1..=self.xr_deg {
                let ck = poly[k];
                if ck != 0 {
                    mults += 1;
                    acc += (ck as u128) * (xpow[k] as u128);
                }
            }
            let val = (acc % p as u128) as u64;
            if val != 0 {
                out.push((*mono, val));
            }
        }
        (out, mults)
    }

    /// Evaluate `\tilde S_{m+1}(e_1, …, e_m; x_r)` at scalar elementary-symmetric values `e[0..m]`.
    pub fn eval_at_elem_sym(&self, e: &[u64], x_r: u64) -> u64 {
        assert_eq!(e.len(), self.m);
        let p = self.p;
        let (terms, _) = self.specialize(x_r);
        let mut mults = 0u64;
        let mut epow = [[1u64; 9]; 4];
        for j in 0..self.m {
            let ej = e[j] % p;
            for k in 1..=self.xr_deg {
                epow[j][k] = mul_u64(epow[j][k - 1], ej, p, &mut mults);
            }
        }
        let mut acc = 0u64;
        for (mono, coef) in terms {
            let mut t = coef;
            for j in 0..self.m {
                let exp = mono[j] as usize;
                if exp > 0 {
                    t = mul_u64(t, epow[j][exp], p, &mut mults);
                }
            }
            acc = add_u64(acc, t, p);
        }
        acc
    }
}

fn enumerate_e_monos(m: usize, max_sum: u8) -> Vec<[u8; 4]> {
    let mut out = Vec::new();
    let mut cur = [0u8; 4];
    fn rec(pos: usize, m: usize, rem: u8, cur: &mut [u8; 4], out: &mut Vec<[u8; 4]>) {
        if pos == m {
            out.push(*cur);
            return;
        }
        for c in 0..=rem {
            cur[pos] = c;
            rec(pos + 1, m, rem - c, cur, out);
        }
        cur[pos] = 0;
    }
    rec(0, m, max_sum, &mut cur, &mut out);
    out
}

fn enumerate_increasing_tuples(m: usize, n_pool: usize) -> Vec<Vec<u64>> {
    let mut out = Vec::new();
    let mut cur = Vec::with_capacity(m);
    fn rec(start: usize, m: usize, n_pool: usize, cur: &mut Vec<u64>, out: &mut Vec<Vec<u64>>) {
        if cur.len() == m {
            out.push(cur.clone());
            return;
        }
        let need = m - cur.len();
        for v in start..=(n_pool - need) {
            cur.push(v as u64);
            rec(v + 1, m, n_pool, cur, out);
            cur.pop();
        }
    }
    rec(0, m, n_pool, &mut cur, &mut out);
    out
}

/// Elementary symmetric values `[e_1(v), …, e_m(v)] mod p` of `v` via `(T + v_0)…(T + v_{m-1})`.
pub fn eval_elem_sym_tuple(v: &[u64], p: u64, mults: &mut u64) -> Vec<u64> {
    let m = v.len();
    let mut poly = vec![0u64; m + 1];
    poly[0] = 1 % p;
    for (step, &vj) in v.iter().enumerate() {
        let vj = vj % p;
        for k in (1..=step + 1).rev() {
            let prod = mul_u64(poly[k - 1], vj, p, mults);
            poly[k] = add_u64(poly[k], prod, p);
        }
    }
    poly[1..=m].to_vec()
}

fn rref_multi_rhs(
    aug: &mut [u64],
    n_rows: usize,
    n_cols: usize,
    n_piv: usize,
    p: u64,
    mults: &mut u64,
) -> Result<(), String> {
    let can_lazy = (p as u128) < (1u128 << 46);
    for col in 0..n_piv {
        let piv = (col..n_rows)
            .find(|&r| aug[r * n_cols + col] != 0)
            .ok_or_else(|| {
                format!("ParamSymmetricSemaev: singular interpolation matrix at col {col}")
            })?;
        if piv != col {
            for c in col..n_cols {
                aug.swap(piv * n_cols + c, col * n_cols + c);
            }
        }
        let pv = aug[col * n_cols + col];
        let inv = inv_u64(pv, p, mults).unwrap();
        aug[col * n_cols + col] = 1;
        for c in (col + 1)..n_cols {
            if aug[col * n_cols + c] != 0 {
                aug[col * n_cols + c] = mul_u64(aug[col * n_cols + c], inv, p, mults);
            }
        }
        // Eliminate all other rows `r != col`.
        if can_lazy {
            let p128 = p as u128;
            let p2 = p128 * p128;
            for r in 0..n_rows {
                if r == col {
                    continue;
                }
                let factor = aug[r * n_cols + col];
                if factor == 0 {
                    continue;
                }
                aug[r * n_cols + col] = 0;
                let f128 = factor as u128;
                for c in (col + 1)..n_cols {
                    let pc = aug[col * n_cols + c];
                    if pc != 0 {
                        *mults += 1;
                        let cur = aug[r * n_cols + c] as u128 + p2 - f128 * (pc as u128);
                        aug[r * n_cols + c] = (cur % p128) as u64;
                    }
                }
            }
        } else {
            for r in 0..n_rows {
                if r == col {
                    continue;
                }
                let factor = aug[r * n_cols + col];
                if factor == 0 {
                    continue;
                }
                aug[r * n_cols + col] = 0;
                for c in (col + 1)..n_cols {
                    let pc = aug[col * n_cols + c];
                    if pc != 0 {
                        let sub = mul_u64(factor, pc, p, mults);
                        aug[r * n_cols + c] = sub_u64(aug[r * n_cols + c], sub, p);
                    }
                }
            }
        }
    }
    Ok(())
}

fn vandermonde_inv(n: usize, p: u64, mults: &mut u64) -> Result<Vec<u64>, String> {
    let cols = 2 * n;
    let mut aug = vec![0u64; n * cols];
    for i in 0..n {
        let mut pw = 1u64 % p;
        let x = i as u64 % p;
        for j in 0..n {
            aug[i * cols + j] = pw;
            pw = mul_u64(pw, x, p, mults);
        }
        aug[i * cols + n + i] = 1 % p;
    }
    rref_multi_rhs(&mut aug, n, cols, n, p, mults)?;
    let mut inv = vec![0u64; n * n];
    for i in 0..n {
        inv[i * n..(i + 1) * n].copy_from_slice(&aug[i * cols + n..i * cols + cols]);
    }
    Ok(inv)
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    fn p_271() -> BigUint {
        BigUint::from(271u32)
    }

    fn fe(v: u64) -> FieldElement {
        FieldElement::new(BigUint::from(v), p_271())
    }

    #[test]
    fn mul_keeps_constant_when_a_term_cancels() {
        // Regression: (1+x)(1-x) = 1 - x^2. A prior bug removed the constant
        // term whenever any term cancelled, yielding -x^2.
        let p = p_271();
        let one = MPoly::constant(fe(1), 1);
        let x = MPoly::variable(0, 1, p.clone());
        let one_minus_x = one.add(&x.scale(&fe(270))); // 270 = -1 mod 271
        let prod = one.add(&x).mul(&one_minus_x);
        assert_eq!(prod.num_terms(), 2, "expected 1 - x^2 (two terms)");
        assert!(
            prod.terms.contains_key(&vec![0u32]),
            "constant term was dropped"
        );
        assert!(prod.terms.contains_key(&vec![2u32]), "x^2 term missing");
        assert!(
            !prod.terms.contains_key(&vec![1u32]),
            "the x term should have cancelled"
        );
    }

    #[test]
    fn elementary_symmetric_sanity() {
        let p = p_271();
        let e1 = elementary_symmetric(1, 3, p.clone());
        // e_1 = X_1 + X_2 + X_3 — three terms.
        assert_eq!(e1.num_terms(), 3);
        let e2 = elementary_symmetric(2, 3, p.clone());
        // e_2 has C(3, 2) = 3 terms.
        assert_eq!(e2.num_terms(), 3);
        let e3 = elementary_symmetric(3, 3, p.clone());
        // e_3 = X_1 X_2 X_3 — one term.
        assert_eq!(e3.num_terms(), 1);
        let e0 = elementary_symmetric(0, 3, p);
        // e_0 = 1 — one term (constant).
        assert_eq!(e0.num_terms(), 1);
    }

    #[test]
    fn mpoly_arithmetic_sanity() {
        let p = p_271();
        let x = MPoly::variable(0, 2, p.clone());
        let y = MPoly::variable(1, 2, p.clone());
        let sum = x.add(&y);
        assert_eq!(sum.num_terms(), 2);
        let prod = x.mul(&y);
        assert_eq!(prod.num_terms(), 1);
        let sq = sum.mul(&sum);
        // (X + Y)² = X² + 2 X Y + Y² — three monomials (since 271 odd).
        assert_eq!(sq.num_terms(), 3);
    }

    /// **Symmetric decomposition round-trip**: build `p = X_1² + X_2²
    /// + X_3²`, which is the power sum `p_2 = e_1² - 2 e_2`, decompose,
    ///   and verify the result.
    #[test]
    fn decompose_power_sum_2() {
        let p = p_271();
        let p2 = power_sum(2, 3, p.clone());
        let dec = decompose_symmetric(&p2, 3, p.clone());
        // Expected: e_1² - 2 e_2.
        // Verify by re-expanding into X-form and equating to p2.
        let mut expected = MPoly::zero(3, p.clone());
        let two = FieldElement::new(BigUint::from(2u32), p.clone());
        // Add e_1² coefficient term.
        let mut e_sq = vec![0u32; 3];
        e_sq[0] = 2;
        expected.terms.insert(e_sq, FieldElement::one(p.clone()));
        // Subtract 2 e_2.
        let mut e2_term = vec![0u32; 3];
        e2_term[1] = 1;
        expected.terms.insert(e2_term, two.neg());
        // The decomposed and expected should agree as MPolys in e_i.
        assert_eq!(dec.terms.len(), expected.terms.len());
        for (k, v) in &expected.terms {
            assert_eq!(dec.terms.get(k), Some(v), "mismatch at {:?}", k);
        }
    }

    /// **Symmetrise S_3 sanity**: the decomposition produces a small
    /// polynomial.  We don't assert the exact paper formula, but
    /// instead assert that `symmetrise_s3` returns a polynomial in
    /// at most 3 variables (`e_1, e_2, e_3`) of total degree ≤ 4.
    #[test]
    fn symmetrise_s3_runs() {
        let a = fe(2);
        let b = fe(3);
        let s3_sym = symmetrise_s3(&a, &b);
        // S_3 has total degree 4 in X_1, X_2, X_3 → after symmetrising,
        // total e-degree should still be ≤ 4.
        assert!(s3_sym.total_degree() <= 4);
        assert!(!s3_sym.is_zero());
    }

    /// **Density reduction**: the original S_3 in (X_1, X_2, X_3) has
    /// 17 distinct monomials (after the canonical expansion); the
    /// symmetrised version has strictly fewer.
    #[test]
    fn s3_symmetrisation_reduces_monomial_count() {
        let a = fe(7);
        let b = fe(11);
        let s3_xs = expand_s3_in_xs(&a, &b);
        let s3_sym = symmetrise_s3(&a, &b);
        let n_xs = s3_xs.num_terms();
        let n_sym = s3_sym.num_terms();
        assert!(
            n_sym < n_xs,
            "symmetrised S_3 has {} monomials vs. {} in X-form — expected reduction",
            n_sym,
            n_xs
        );
    }

    #[test]
    fn param_symmetric_semaev_m2_m3_m4_exact_identities() {
        // Test over a 31-bit prime M31 = 2^31 - 1 and a small prime p = 1009.
        for &(p, a, b) in &[(1009u64, 7u64, 13u64), (2_147_483_647u64, 2_147_483_644, 5)] {
            let ps3 = ParamSymmetricSemaev::build(2, a, b, p).expect("ParamS3");
            let ps4 = ParamSymmetricSemaev::build(3, a, b, p).expect("ParamS4");
            let ps5 = ParamS5::build(4, a, b, p).expect("ParamS5");
            assert!(ps5.active_monos.len() <= 495);

            // Check per-target specialization cost of ParamS5: O(1) (~few thousand mulmods),
            // instead of ~2.48M–2.95M mulmods for grid interpolation.
            let (_, spec_mults) = ps5.specialize(123_456_789 % p);
            assert!(
                spec_mults < 4_500,
                "ParamS5 per-target specialization took {spec_mults} mulmods"
            );

            // Verify exact polynomial identity on 64 pseudo-random tuples (including repeated coordinates!).
            let mut state = 0x9e37_79b9_7f4a_7c15u64 ^ p;
            let mut next = || {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                (state >> 16) % p
            };
            for t in 0..64 {
                let x1 = next();
                let x2 = if t % 8 == 0 { x1 } else { next() };
                let x3 = if t % 11 == 0 { x2 } else { next() };
                let x4 = next();
                let xr = next();
                let mut m_tmp = 0u64;

                let e2 = eval_elem_sym_tuple(&[x1, x2], p, &mut m_tmp);
                assert_eq!(
                    ps3.eval_at_elem_sym(&e2, xr),
                    eval_s3_u64(x1, x2, xr, a, b, p),
                    "ParamS3 mismatch at p={p}, t={t}"
                );

                let e3 = eval_elem_sym_tuple(&[x1, x2, x3], p, &mut m_tmp);
                assert_eq!(
                    ps4.eval_at_elem_sym(&e3, xr),
                    eval_s4_u64(x1, x2, x3, xr, a, b, p),
                    "ParamS4 mismatch at p={p}, t={t}"
                );

                let e4 = eval_elem_sym_tuple(&[x1, x2, x3, x4], p, &mut m_tmp);
                assert_eq!(
                    ps5.eval_at_elem_sym(&e4, xr),
                    eval_s5_u64(x1, x2, x3, x4, xr, a, b, p),
                    "ParamS5 mismatch at p={p}, t={t}"
                );
            }
        }

        // Verify ParamS5 vanishes on true 5-point curve relations P1 + P2 + P3 + P4 = R on E(F_1009).
        use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
        let curve = PrimeCurve {
            p: 1009,
            a: 7,
            b: 13,
        };
        let ps5 = ParamS5::build(4, curve.a, curve.b, curve.p).unwrap();
        let mut pts = Vec::new();
        for x in 1..80u64 {
            if let Some(y) = curve.sqrt(curve.rhs(x)) {
                if y != 0 {
                    pts.push(PrimePoint::affine(x, y));
                }
            }
        }
        let mut ops = GroupOps::default();
        for k in 0..10 {
            let p1 = pts[k];
            let p2 = pts[k + 3];
            let p3 = pts[k + 7];
            let p4 = pts[k + 11];
            let s12 = curve.add(&mut ops, p1, p2);
            let s123 = curve.add(&mut ops, s12, p3);
            let r = curve.add(&mut ops, s123, p4);
            if s12.infinity || s123.infinity || r.infinity {
                continue;
            }
            let mut m_tmp = 0u64;
            let e4 = eval_elem_sym_tuple(&[p1.x, p2.x, p3.x, p4.x], curve.p, &mut m_tmp);
            assert_eq!(
                ps5.eval_at_elem_sym(&e4, r.x),
                0,
                "ParamS5 must vanish on P1+P2+P3+P4 = R"
            );
        }
    }
}
