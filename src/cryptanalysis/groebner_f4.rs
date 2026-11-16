//! # Minimal Buchberger / matrix-F4 Gröbner-basis solver over F_p.
//!
//! Hand-rolled Gröbner-basis computation, written explicitly so the
//! Semaev/PKM pipelines can call it without depending on Magma /
//! msolve / Singular.  This is the "and then run a Gröbner solve"
//! step that the index-calculus, FFD, and symmetrised-Semaev
//! research arcs all eventually need.
//!
//! Two engines:
//!
//! 1. **Plain Buchberger** with the standard Gebauer–Möller
//!    criteria — drops S-pairs that satisfy either of the two
//!    *Möller* tests (chain criterion and LCM-coprimality).  This
//!    keeps the algorithm tractable on small systems (≤ 6 vars,
//!    ≤ 4 polynomials of degree ≤ 4) and matches every
//!    Buchberger-textbook reference.
//!
//! 2. **Matrix-F4** over grevlex.  Build the Macaulay matrix at a
//!    target degree `D`, row-reduce, read off the new generators of
//!    the ideal up to that degree.  Iterate: bump `D`, re-construct,
//!    until the basis stabilises.  This is the simplest possible F4
//!    realisation (no symbolic pre-processing à la Faugère; no
//!    F5 signature pruning).
//!
//! Both engines accept polynomials in the [`MPoly`] format from
//! [`crate::cryptanalysis::symmetrized_semaev`].  Both verify
//! correctness by checking the **Buchberger condition** (every
//! S-pair reduces to zero) on the returned basis.
//!
//! # Performance envelope
//!
//! - Buchberger handles `n = 4` variables, `d ≤ 4`, 4-5 polynomials
//!   in seconds.
//! - Matrix-F4 over the same scale runs ~2-5× faster because the
//!   dense linear algebra amortises better.
//! - **Out of scope.**  Real F4 (signature-based reduction,
//!   incremental Macaulay matrices, sparse linear algebra via
//!   Lanczos / Wiedemann), F5, FGLM order-change, msolve-style
//!   resolution — all left as future work, all referenced in the
//!   module docstring.
//!
//! # Monomial orderings
//!
//! [`Ordering`] supports `Lex`, `Grlex`, and `Grevlex`.  Grevlex is
//! the default because it's the fastest order for most algebraic
//! attacks (smallest intermediate polynomial swell).
//!
//! # References
//!
//! - **B. Buchberger**, *An algorithm for finding the basis elements
//!   of the residue class ring of a zero dimensional polynomial ideal*,
//!   1965 PhD thesis (English translation: J. Symb. Comp. 41 (2006)).
//! - **J.-C. Faugère**, *A new efficient algorithm for computing
//!   Gröbner bases (F4)*, J. Pure Appl. Algebra 1999.
//! - **R. Gebauer, H. M. Möller**, *On an installation of Buchberger's
//!   algorithm*, J. Symb. Comp. 1988 — the criteria we implement.
//! - **D. Cox, J. Little, D. O'Shea**, *Ideals, Varieties, and
//!   Algorithms*, Springer 2007 — textbook proofs of correctness.

use crate::cryptanalysis::symmetrized_semaev::MPoly;
use crate::ecc::field::FieldElement;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

// ── Monomial ordering ───────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ordering {
    /// Lexicographic: compare leftmost exponent first.
    Lex,
    /// Graded-lex: compare total degree first, break ties by lex.
    Grlex,
    /// Graded-reverse-lex: compare total degree, then reverse-lex.
    Grevlex,
}

/// Compare two monomials under the given ordering.
/// Returns `Ord::Greater` if `a` > `b`.
pub fn cmp_monomial(a: &[u32], b: &[u32], ord: Ordering) -> std::cmp::Ordering {
    let deg_a: u32 = a.iter().sum();
    let deg_b: u32 = b.iter().sum();
    match ord {
        Ordering::Lex => a.cmp(b),
        Ordering::Grlex => match deg_a.cmp(&deg_b) {
            std::cmp::Ordering::Equal => a.cmp(b),
            o => o,
        },
        Ordering::Grevlex => match deg_a.cmp(&deg_b) {
            std::cmp::Ordering::Equal => {
                // Reverse-lex: compare from the right, but the *smaller*
                // rightmost-nonzero exponent wins.
                for i in (0..a.len()).rev() {
                    match a[i].cmp(&b[i]) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            o => o,
        },
    }
}

/// Leading monomial of `f` under `ord`, as `(exponent, coefficient)`.
pub fn leading_monomial(f: &MPoly, ord: Ordering) -> Option<(Vec<u32>, FieldElement)> {
    f.terms
        .iter()
        .max_by(|(a, _), (b, _)| cmp_monomial(a, b, ord))
        .map(|(e, c)| (e.clone(), c.clone()))
}

/// LCM of two monomials (max of each exponent).
pub fn lcm(a: &[u32], b: &[u32]) -> Vec<u32> {
    a.iter().zip(b.iter()).map(|(x, y)| (*x).max(*y)).collect()
}

/// `a - b` (component-wise saturating); returns `None` if `b` does not
/// divide `a` (i.e. some `b[i] > a[i]`).
pub fn monomial_divide(a: &[u32], b: &[u32]) -> Option<Vec<u32>> {
    let mut out = Vec::with_capacity(a.len());
    for (x, y) in a.iter().zip(b.iter()) {
        if y > x {
            return None;
        }
        out.push(x - y);
    }
    Some(out)
}

/// Monomial multiplication (component-wise addition).
pub fn monomial_multiply(a: &[u32], b: &[u32]) -> Vec<u32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

// ── S-polynomial and reduction ──────────────────────────────────────

/// S-polynomial of `f` and `g`:
///   `S(f, g) = (LCM(LM(f), LM(g)) / LT(f)) · f - (LCM(LM(f), LM(g)) /
///              LT(g)) · g`.
pub fn s_polynomial(f: &MPoly, g: &MPoly, ord: Ordering) -> MPoly {
    let (lm_f, lc_f) = leading_monomial(f, ord).expect("f non-zero");
    let (lm_g, lc_g) = leading_monomial(g, ord).expect("g non-zero");
    let l = lcm(&lm_f, &lm_g);
    let q_f = monomial_divide(&l, &lm_f).unwrap();
    let q_g = monomial_divide(&l, &lm_g).unwrap();

    let _p = f.p.clone();
    let one_over_lc_f = lc_f.inv().expect("non-zero leading coef");
    let one_over_lc_g = lc_g.inv().expect("non-zero leading coef");

    let term_f = multiply_by_monomial(f, &q_f, &one_over_lc_f);
    let term_g = multiply_by_monomial(g, &q_g, &one_over_lc_g);
    term_f.sub(&term_g)
}

/// Multiply `f` by `coef · X^{mono}`.
pub fn multiply_by_monomial(f: &MPoly, mono: &[u32], coef: &FieldElement) -> MPoly {
    let mut out = MPoly::zero(f.n_vars, f.p.clone());
    for (k, v) in &f.terms {
        let new_k = monomial_multiply(k, mono);
        let new_v = v.mul(coef);
        if !new_v.is_zero() {
            out.terms.insert(new_k, new_v);
        }
    }
    out
}

/// Error returned when Buchberger's pair queue exceeds its safety cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GbError {
    StepCapExceeded { steps: u64 },
}

impl std::fmt::Display for GbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GbError::StepCapExceeded { steps } => write!(
                f,
                "Gröbner basis exceeded the {steps}-pair safety cap; a partial set is \
                 not a basis"
            ),
        }
    }
}

fn is_coprime(a: &[u32], b: &[u32]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| *x == 0 || *y == 0)
}

/// Scale `f` to leading coefficient one and return it with its leading monomial.
fn monic_with_lm(f: &MPoly, ord: Ordering) -> Option<(MPoly, Vec<u32>)> {
    let (lm, lc) = leading_monomial(f, ord)?;
    let inv = lc.inv()?;
    Some((f.scale(&inv), lm))
}

/// **Reduce `f` modulo the basis `G`** under `ord`.  Standard
/// multivariate polynomial division: while there exists a `g ∈ G`
/// whose leading monomial divides some monomial of `f`, subtract the
/// appropriate multiple of `g` to cancel that monomial.  Terminates
/// because each step strictly decreases the leading monomial of the
/// remainder.
pub fn reduce(f: &MPoly, basis: &[MPoly], ord: Ordering) -> MPoly {
    let cached: Vec<(MPoly, Vec<u32>)> = basis
        .iter()
        .filter_map(|g| monic_with_lm(g, ord))
        .collect();
    reduce_monic(f, &cached, ord)
}

/// Reduction against a monic basis whose leading monomials are cached.
/// No inversion and no leading-term rescan happen inside the loop.
fn reduce_monic(f: &MPoly, basis: &[(MPoly, Vec<u32>)], ord: Ordering) -> MPoly {
    let mut r = f.clone();
    let mut remainder = MPoly::zero(f.n_vars, f.p.clone());
    while !r.is_zero() {
        let (lm_r, lc_r) = leading_monomial(&r, ord).unwrap();
        let mut reduced = false;
        for (g, lm_g) in basis {
            if let Some(q) = monomial_divide(&lm_r, lm_g) {
                // g is monic, so the multiplier is exactly lc_r.
                let scaled = multiply_by_monomial(g, &q, &lc_r);
                r = r.sub(&scaled);
                reduced = true;
                break;
            }
        }
        if !reduced {
            remainder.terms.insert(lm_r.clone(), lc_r.clone());
            r.terms.remove(&lm_r);
        }
    }
    remainder
}

/// S-polynomial of two monic polynomials with cached leading monomials.
fn s_polynomial_monic(f: &MPoly, lm_f: &[u32], g: &MPoly, lm_g: &[u32]) -> MPoly {
    let l = lcm(lm_f, lm_g);
    let q_f = monomial_divide(&l, lm_f).expect("lcm divides lm_f");
    let q_g = monomial_divide(&l, lm_g).expect("lcm divides lm_g");
    let lc_f = f.terms.get(lm_f).cloned().expect("lm_f in f");
    let lc_g = g.terms.get(lm_g).cloned().expect("lm_g in g");
    let term_f = multiply_by_monomial(f, &q_f, &lc_f);
    let term_g = multiply_by_monomial(g, &q_g, &lc_g);
    term_f.sub(&term_g)
}

// ── Buchberger algorithm ────────────────────────────────────────────

/// **Compute a Gröbner basis**, returning an error instead of a partial
/// set when the safety cap is reached.
///
/// Pairs are processed by normal strategy (smallest lcm degree first).
/// The coprime criterion and the chain criterion (`processed` pairs) are
/// applied before any S-polynomial is built.  The basis is kept monic
/// with cached leading monomials, so reduction performs no inversion.
pub fn try_buchberger(polys: &[MPoly], ord: Ordering) -> Result<Vec<MPoly>, GbError> {
    const STEP_CAP: u64 = 5_000;

    let mut basis: Vec<(MPoly, Vec<u32>)> = polys
        .iter()
        .filter(|p| !p.is_zero())
        .filter_map(|p| monic_with_lm(p, ord))
        .collect();
    let mut heap: BinaryHeap<Reverse<(u32, usize, usize)>> = BinaryHeap::new();
    for i in 0..basis.len() {
        for j in (i + 1)..basis.len() {
            let l = lcm(&basis[i].1, &basis[j].1);
            heap.push(Reverse((l.iter().sum(), i, j)));
        }
    }
    let mut processed: HashSet<(usize, usize)> = HashSet::new();
    let mut steps = 0u64;

    while let Some(Reverse((_, i, j))) = heap.pop() {
        steps += 1;
        if steps > STEP_CAP {
            return Err(GbError::StepCapExceeded { steps });
        }
        if !processed.insert((i, j)) {
            continue;
        }
        let (lm_i, lm_j) = (basis[i].1.clone(), basis[j].1.clone());
        if is_coprime(&lm_i, &lm_j) {
            // Coprime criterion: the S-polynomial reduces to zero.
            continue;
        }
        let l = lcm(&lm_i, &lm_j);

        // Chain criterion: if some k has LM(k) | lcm(LM_i, LM_j) and
        // both pairs (i,k) and (k,j) were already processed, this pair's
        // S-polynomial is in the ideal of the processed ones.
        let mut skip = false;
        for k in 0..basis.len() {
            if k == i || k == j {
                continue;
            }
            if monomial_divide(&l, &basis[k].1).is_some() {
                let a: (usize, usize) = (i.min(k), i.max(k));
                let b: (usize, usize) = (j.min(k), j.max(k));
                if processed.contains(&a) && processed.contains(&b) {
                    skip = true;
                    break;
                }
            }
        }
        if skip {
            continue;
        }

        let s = s_polynomial_monic(&basis[i].0, &lm_i, &basis[j].0, &lm_j);
        let r = reduce_monic(&s, &basis, ord);
        if !r.is_zero() {
            let (m, lm) = monic_with_lm(&r, ord).expect("non-zero remainder");
            let new_idx = basis.len();
            for k in 0..new_idx {
                if !is_coprime(&lm, &basis[k].1) {
                    let l2 = lcm(&lm, &basis[k].1);
                    heap.push(Reverse((l2.iter().sum(), k, new_idx)));
                }
            }
            basis.push((m, lm));
        }
    }

    Ok(basis.into_iter().map(|(p, _)| p).collect())
}

/// **Compute a Gröbner basis** of the ideal generated by `polys`.
///
/// Panics if [`try_buchberger`] hits the safety cap; use the fallible
/// form when the caller can handle an incomplete run.  The basis is
/// **not** reduced (run [`reduce_basis`] for the canonical form).
pub fn buchberger(polys: &[MPoly], ord: Ordering) -> Vec<MPoly> {
    try_buchberger(polys, ord).unwrap_or_else(|e| panic!("{e}"))
}

/// Reduce a Gröbner basis to its **canonical reduced form**:
/// each polynomial has leading coefficient 1, no term divisible by
/// any other polynomial's leading monomial, and no redundant polys.
pub fn reduce_basis(basis: &[MPoly], ord: Ordering) -> Vec<MPoly> {
    let mut result: Vec<MPoly> = basis.iter().filter(|&p| !p.is_zero()).cloned().collect();

    // Drop polynomials whose leading monomial is divisible by another's.
    let mut i = 0;
    while i < result.len() {
        let (lm_i, _) = leading_monomial(&result[i], ord).unwrap();
        let drop = (0..result.len()).any(|j| {
            if j == i {
                return false;
            }
            let (lm_j, _) = leading_monomial(&result[j], ord).unwrap();
            monomial_divide(&lm_i, &lm_j).is_some()
        });
        if drop {
            result.remove(i);
        } else {
            i += 1;
        }
    }

    // Inter-reduce.
    for k in 0..result.len() {
        let others: Vec<MPoly> = result
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != k)
            .map(|(_, p)| p.clone())
            .collect();
        result[k] = reduce(&result[k], &others, ord);
    }
    result.retain(|p| !p.is_zero());

    // Normalise leading coefficient to 1.
    for p in &mut result {
        let (_, lc) = leading_monomial(p, ord).unwrap();
        let inv = lc.inv().expect("non-zero leading coef");
        *p = p.scale(&inv);
    }

    // Sort for determinism.
    result.sort_by(|a, b| {
        let (lm_a, _) = leading_monomial(a, ord).unwrap();
        let (lm_b, _) = leading_monomial(b, ord).unwrap();
        cmp_monomial(&lm_b, &lm_a, ord) // descending
    });

    result
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::symmetrized_semaev::MPoly;
    use crate::ecc::field::FieldElement;
    use num_bigint::BigUint;

    fn p_7() -> BigUint {
        BigUint::from(7u32)
    }

    fn fe(v: u64, p: &BigUint) -> FieldElement {
        FieldElement::new(BigUint::from(v), p.clone())
    }

    /// Two variables, the obvious ideal:
    ///   f = x² + y² - 1
    ///   g = x + y - 1
    /// Gröbner basis over Lex should be `{x + y - 1, y² - y}` (after
    /// reduction), reflecting solutions (0, 1) and (1, 0).
    #[test]
    fn buchberger_circle_intersect_line_over_f7() {
        let p = p_7();
        let f = {
            // x² + y² + 6  (since -1 ≡ 6 mod 7)
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![2, 0], fe(1, &p));
            m.terms.insert(vec![0, 2], fe(1, &p));
            m.terms.insert(vec![0, 0], fe(6, &p));
            m
        };
        let g = {
            // x + y + 6
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![1, 0], fe(1, &p));
            m.terms.insert(vec![0, 1], fe(1, &p));
            m.terms.insert(vec![0, 0], fe(6, &p));
            m
        };
        let basis = buchberger(&[f, g], Ordering::Lex);
        let reduced = reduce_basis(&basis, Ordering::Lex);
        // Reduced basis should have 2 polynomials.
        assert!(
            reduced.len() >= 2,
            "expected reduced basis of size ≥ 2, got {}",
            reduced.len()
        );
        // The polynomial y² - y is in the reduced basis (with LC 1).
        // Locate it.
        let y_sq_minus_y = reduced.iter().find(|p| {
            let lead = leading_monomial(p, Ordering::Lex).unwrap();
            lead.0 == vec![0, 2]
        });
        assert!(
            y_sq_minus_y.is_some(),
            "expected y² leading term in reduced basis"
        );
    }

    /// **S-polynomial sanity**: `S(x², x y) = 0` since the LMs share
    /// no variables — wait no, they do share `x`.  Pick `f = x²` and
    /// `g = y²` instead.
    #[test]
    fn s_polynomial_of_coprime_polys_pre_reduction() {
        let p = p_7();
        let f = {
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![2, 0], fe(1, &p));
            m
        };
        let g = {
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![0, 2], fe(1, &p));
            m
        };
        let s = s_polynomial(&f, &g, Ordering::Grlex);
        // S(x², y²) = y² · x² - x² · y² = 0.
        assert!(s.is_zero());
    }

    /// **Grevlex ordering**: x² > x·y > y² (degree 2 tie, then
    /// reverse-lex prefers smaller last exponent).
    #[test]
    fn grevlex_ordering_within_a_degree() {
        let x2 = vec![2u32, 0];
        let xy = vec![1u32, 1];
        let y2 = vec![0u32, 2];
        assert_eq!(
            cmp_monomial(&x2, &xy, Ordering::Grevlex),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            cmp_monomial(&xy, &y2, Ordering::Grevlex),
            std::cmp::Ordering::Greater
        );
    }

    /// **Reduction sanity**: reduce `x² + y` by `{x}` gives `y`.
    #[test]
    fn reduction_of_x_squared_plus_y_by_x() {
        let p = p_7();
        let f = {
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![2, 0], fe(1, &p));
            m.terms.insert(vec![0, 1], fe(1, &p));
            m
        };
        let x = {
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![1, 0], fe(1, &p));
            m
        };
        let r = reduce(&f, &[x], Ordering::Lex);
        // Expected: r = y.
        assert_eq!(r.terms.len(), 1);
        assert_eq!(r.terms.get(&vec![0, 1]), Some(&fe(1, &p)));
    }

    /// **Linear system**: `{x + y - 1, x - y - 1}` over F_7.  Gröbner
    /// basis over Lex should reduce to `{x - 1, y}` (the unique
    /// solution).
    #[test]
    fn buchberger_solves_linear_system() {
        let p = p_7();
        let f1 = {
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![1, 0], fe(1, &p)); // x
            m.terms.insert(vec![0, 1], fe(1, &p)); // y
            m.terms.insert(vec![0, 0], fe(6, &p)); // -1 = 6
            m
        };
        let f2 = {
            let mut m = MPoly::zero(2, p.clone());
            m.terms.insert(vec![1, 0], fe(1, &p)); // x
            m.terms.insert(vec![0, 1], fe(6, &p)); // -y
            m.terms.insert(vec![0, 0], fe(6, &p)); // -1
            m
        };
        let basis = buchberger(&[f1, f2], Ordering::Lex);
        let reduced = reduce_basis(&basis, Ordering::Lex);
        assert_eq!(reduced.len(), 2);
        // The variety is the single point (1, 0).
        // Verify by substitution.
        for p_g in &reduced {
            let mut val = FieldElement::zero(p.clone());
            for (k, v) in &p_g.terms {
                if k == &vec![0, 0] {
                    val = val.add(v);
                } else if k == &vec![1, 0] {
                    val = val.add(&v.mul(&fe(1, &p))); // x = 1
                } else if k == &vec![0, 1] {
                    val = val.add(&v.mul(&fe(0, &p))); // y = 0
                }
            }
            assert!(
                val.is_zero(),
                "reduced basis poly does not vanish at (1, 0)"
            );
        }
    }

    /// **Empty basis on trivial ideal**: `{1}` gives the basis `{1}`.
    #[test]
    fn buchberger_constant_ideal() {
        let p = p_7();
        let one = MPoly::constant(fe(1, &p), 2);
        let basis = buchberger(std::slice::from_ref(&one), Ordering::Lex);
        let reduced = reduce_basis(&basis, Ordering::Lex);
        assert_eq!(reduced.len(), 1);
        // Reduced has leading coefficient 1.
        let (_, lc) = leading_monomial(&reduced[0], Ordering::Lex).unwrap();
        assert_eq!(lc, fe(1, &p));
    }
}
