//! **Elliptic-curve index calculus** via Semaev summation polynomials —
//! a from-scratch implementation of the framework that gives the closest
//! published research to "subexponential ECDLP."
//!
//! ## What this module is
//!
//! A *toy-scale* implementation of the algebraic index-calculus
//! framework for elliptic-curve discrete logarithms.  It solves ECDLP
//! on small (sub-40-bit) prime-field curves by:
//!
//! 1. Building a factor base **F = {P₁, …, P_m}** of points with small
//!    x-coordinate.
//! 2. Sampling random `R = aG + bQ` and trying to *decompose* `R` as a
//!    signed sum of two factor-base elements
//!    `R = ε_i Pᵢ + ε_j P_j`, with the help of **Semaev's 3rd summation
//!    polynomial** `S₃` (closed form for short-Weierstrass curves).
//! 3. Recording each successful decomposition as a row of a linear
//!    system over `ℤ/nℤ` (where `n` is the curve order) in the
//!    unknowns `(log_G P₁, …, log_G P_m, log_G Q)`.
//! 4. Solving the system by **Gaussian elimination mod n** to recover
//!    `log_G Q`.
//!
//! The pieces above are the same architecture that Faugère, Perret,
//! Petit, Renault (2012) and Petit–Quisquater (2012) — collectively the
//! *FPPR / PQ method* — used to attack the binary-field ECDLP, and that
//! Petit–Kosters–Messeng (2016) and Amadori–Pintore–Sala (2018) adapted
//! to prime fields.
//!
//! ## What this module is **not**
//!
//! Not a threat to deployed curves.  As of 2024 the published prime-field
//! index calculus is *asymptotically slower* than generic Pollard rho:
//!
//! - Naive 2-decomposition (this module) needs ≈ `p/m` trials × `m`
//!   factor-base scans = `O(p · m / m) = O(p)` per relation, times
//!   `m` relations = `O(p · m)` total.  With `m = p^{1/2}` (the
//!   optimum), that is `O(p^{3/2})` — much worse than Pollard rho's
//!   `O(p^{1/2})`.
//! - To beat rho you would need to use higher-degree Semaev polynomials
//!   (`S_n` for `n ≥ 4`) and decompose `R` as a sum of `n − 1` factor
//!   base elements.  For prime fields, this requires solving a 0-dim
//!   polynomial system in many variables — exactly the place where
//!   Gröbner basis cost blows up.  Petit–Kosters–Messeng achieve
//!   complexity ≈ `O(p^{1/2 + ε})` *under heuristic assumptions* that
//!   are not believed to hold uniformly.  See the discussion in
//!   Galbraith, "Notes on summation polynomials" (2015), and
//!   Huang–Kiltz–Petit (Crypto 2015).
//!
//! ## The summation polynomial
//!
//! For `E: y² = x³ + ax + b` over a field `K`, Semaev's `nᵗʰ` summation
//! polynomial `S_n ∈ K[X₁, …, X_n]` has the defining property
//!
//! > `S_n(x₁, …, x_n) = 0  ⟺  ∃ y₁, …, y_n  such that
//! >        (xᵢ, yᵢ) ∈ E(K̄)  and  Σᵢ (xᵢ, yᵢ) = O` in the group law.
//!
//! There is a closed form for `S₃`:
//!
//! ```text
//! S₃(X₁, X₂, X₃) = (X₁ − X₂)² · X₃²
//!                − 2[(X₁ + X₂)(X₁X₂ + a) + 2b] · X₃
//!                + (X₁X₂ − a)² − 4b(X₁ + X₂).
//! ```
//!
//! `S₃` is symmetric in `X₁, X₂, X₃` (verify by expanding), quadratic
//! in each variable.  For higher `n` the polynomial is computed
//! recursively as the *resultant* in `X` of `S_{m+1}(X₁, …, X_m, X)`
//! and `S_{n−m+1}(X_{m+1}, …, X_n, X)`.  We implement only `S₃` here,
//! which is all that is needed for 2-decompositions.
//!
//! ## References
//!
//! - **I. Semaev**, *Summation polynomials and the discrete logarithm
//!   problem on elliptic curves*, IACR ePrint 2004/031.
//! - **C. Diem**, *On the discrete logarithm problem in elliptic
//!   curves*, Compositio Math. 147 (2011).
//! - **J.-C. Faugère, L. Perret, C. Petit, G. Renault**, *Improving
//!   the complexity of index calculus algorithms in elliptic curves
//!   over binary fields*, Eurocrypt 2012.
//! - **C. Petit, J.-J. Quisquater**, *On polynomial systems arising
//!   from a Weil descent*, Asiacrypt 2012.
//! - **S. Galbraith**, *Notes on summation polynomials*, 2015.
//! - **M.-D. Huang, M. Kiltz, C. Petit**, *Last fall degree, HFE, and
//!   Weil descent attacks on ECDLP*, Crypto 2015.
//! - **C. Petit, M. Kosters, A. Messeng**, *Algebraic approaches for
//!   the elliptic curve discrete logarithm problem over prime fields*,
//!   PKC 2016.
//! - **A. Amadori, F. Pintore, M. Sala**, *On the discrete logarithm
//!   problem for prime-field elliptic curves*, FFA 51 (2018).

use crate::ecc::curve::CurveParams;
use crate::ecc::field::FieldElement;
use crate::ecc::point::Point;
use crate::prime_hyperelliptic::ops as bitop_ops;
use crate::utils::mod_inverse;
use num_bigint::BigUint;
use num_traits::{One, Zero};

// ── Operation counters (per thread) ───────────────────────────────────
//
// The staged driver reports the group operations it charges (`group_additions`,
// `group_doublings`) and the native operations the unit does not charge
// (`*_uncharged`, AGENTS.md section 5a).  Counts are kept per thread, reset at
// the start of a staged solve and read at its end; a solve runs on one thread.
// Group operations of a scalar pass are derived from the scalars it consumes
// (doublings = bits - 1, additions = popcount - 1 for a ladder; see
// `scalar_mult_ops`), not read from the arithmetic.
thread_local! {
    static COUNTERS: std::cell::RefCell<std::collections::BTreeMap<&'static str, u64>> =
        const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
}

fn count(name: &'static str, v: u64) {
    COUNTERS.with(|c| *c.borrow_mut().entry(name).or_insert(0) += v);
}

fn reset_counters() {
    COUNTERS.with(|c| c.borrow_mut().clear());
}

fn take_counters() -> std::collections::BTreeMap<String, u64> {
    COUNTERS.with(|c| {
        c.borrow()
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    })
}

/// Group operations of a left-to-right double-and-add ladder for the scalar `k`:
/// `(additions, doublings)` = `(popcount - 1, bits - 1)`.
fn scalar_mult_ops(k: &BigUint) -> (u64, u64) {
    let bits = k.bits();
    let pop: u64 = k.iter_u64_digits().map(|d| u64::from(d.count_ones())).sum();
    (pop.saturating_sub(1), bits.saturating_sub(1))
}

// ── Semaev's 3rd summation polynomial ───────────────────────────────────

/// **Evaluate** `S₃(x₁, x₂, x₃)` on the curve `E: y² = x³ + ax + b`.
///
/// Returns `0 ∈ F_p` iff there exist `y₁, y₂, y₃ ∈ F_p` (or a quadratic
/// extension thereof) such that `(xᵢ, yᵢ) ∈ E` and `P₁ + P₂ + P₃ = O`.
///
/// Note: the result equalling zero only *implies* a sign combination
/// `ε₁P₁ + ε₂P₂ + ε₃P₃ = O` exists for some signs — the polynomial
/// is independent of the y-coordinates, which are determined only up
/// to negation.
pub fn semaev_s3(
    x1: &FieldElement,
    x2: &FieldElement,
    x3: &FieldElement,
    a: &FieldElement,
    b: &FieldElement,
) -> FieldElement {
    let (a_coef, b_coef, c_coef) = semaev_s3_in_x3(x1, x2, a, b);
    // S₃ as a quadratic in X₃:  A·X₃² + B·X₃ + C.
    a_coef.mul(&x3.mul(x3)).add(&b_coef.mul(x3)).add(&c_coef)
}

/// **Extract S₃ as a quadratic in `X₃`** given the other two x-coords
/// fixed.  Returns `(A, B, C)` such that `S₃(x₁, x₂, X₃) = A·X₃² + B·X₃ + C`.
///
/// This is the form we plug into the quadratic formula when sweeping
/// `(x₁, x₂) = (x_R, x_{F_i})` and asking "is there an x_{F_j} in the
/// factor base satisfying `S₃ = 0`?"
pub fn semaev_s3_in_x3(
    x1: &FieldElement,
    x2: &FieldElement,
    a: &FieldElement,
    b: &FieldElement,
) -> (FieldElement, FieldElement, FieldElement) {
    let p = x1.modulus.clone();
    let two = FieldElement::new(BigUint::from(2u32), p.clone());
    let four = FieldElement::new(BigUint::from(4u32), p.clone());

    let x1x2 = x1.mul(x2);
    let sum = x1.add(x2);
    let diff = x1.sub(x2);

    // A = (x₁ − x₂)²
    let coef_a = diff.mul(&diff);

    // B = −2 [(x₁ + x₂)(x₁x₂ + a) + 2b]
    let inner = sum.mul(&x1x2.add(a)).add(&two.mul(b));
    let coef_b = two.mul(&inner).neg();

    // C = (x₁x₂ − a)² − 4b(x₁ + x₂)
    let part = x1x2.sub(a);
    let part_sq = part.mul(&part);
    let coef_c = part_sq.sub(&four.mul(b).mul(&sum));

    (coef_a, coef_b, coef_c)
}

// ── Semaev's higher summation polynomials (recursive via resultant) ─────

/// **Semaev S₄ as a univariate polynomial in `X₄`**, evaluated at
/// fixed `(X₁, X₂, X₃)`.
///
/// `S_n` for `n ≥ 4` is built recursively from `S_3` via the resultant
/// identity (Semaev 2004 §3):
///
/// ```text
/// S_{m+n−2}(X₁, …, X_{m+n−2}) = Res_X(
///        S_{m+1}(X₁, …, X_m, X),
///        S_{n+1}(X_{m+1}, …, X_{m+n−2}, −X)).
/// ```
///
/// For `n = 4`, taking `m = n = 3`:
/// `S_4(X₁,X₂,X₃,X₄) = Res_X( S_3(X₁,X₂,X), S_3(X₃,X₄,X) )`
/// (using that `S_3` is invariant under `X → −X` after expansion — the
/// `−X` in the formula above is a notational convenience).
///
/// We fix `(X₁, X₂, X₃)` (turning the first `S_3` into a univariate
/// quadratic in `X`) and return `S_4(X₁,X₂,X₃,X₄)` as a quartic in
/// `X₄`: a 5-tuple `(c₀, c₁, c₂, c₃, c₄)` with
/// `S_4 = c₀ + c₁ X₄ + c₂ X₄² + c₃ X₄³ + c₄ X₄⁴`.
///
/// This is the polynomial used in 3-decompositions: given `R = aG + bQ`,
/// find `(F_i, F_j, F_k)` in the factor base with `S_4(x_R, x_i, x_j, x_k)
/// = 0`.  Solving for `x_k` over `F_p` is univariate root-finding of a
/// quartic: [`find_roots_fp_fast`] does it by `gcd(f, X^p - X)` followed by
/// equal-degree splitting (Cantor–Zassenhaus) above 20 bits, and by brute
/// force below; [`find_one_relation_s4_counted`] is the caller.
pub fn semaev_s4_in_x4(
    x1: &FieldElement,
    x2: &FieldElement,
    x3: &FieldElement,
    a: &FieldElement,
    b: &FieldElement,
) -> [FieldElement; 5] {
    // First S_3(X₁, X₂, X) = α₂ X² + α₁ X + α₀.
    let (a2, a1, a0) = semaev_s3_in_x3(x1, x2, a, b);
    // Second S_3(X₃, X₄, X) — treating X₄ as a *symbol*.  Since `S_3`
    // is quadratic in *each* variable, we expand explicitly:
    //
    //   S₃(X₃, X₄, X) =   (X₃ − X₄)² X²
    //                   − 2[(X₃ + X₄)(X₃ X₄ + a) + 2b] X
    //                   + (X₃ X₄ − a)² − 4b(X₃ + X₄).
    //
    // Coefficients are polynomials in X₄ of degree (2, 2, 2):
    //   (X₃ − X₄)²        = X₄² − 2X₃·X₄ + X₃²
    //   X₃ + X₄           = X₃ + X₄
    //   X₃ X₄ + a         = X₃·X₄ + a
    //   (X₃ + X₄)(X₃ X₄ + a)  ≡  X₃² X₄ + a X₃ + X₃ X₄² + a X₄
    //   (X₃ X₄ − a)²      = X₃² X₄² − 2 a X₃ X₄ + a²
    //   −4b(X₃ + X₄)      = −4b X₃ − 4b X₄.
    //
    // So  S₃(X₃, X₄, X) = β₂(X₄) X² + β₁(X₄) X + β₀(X₄)
    // where
    //   β₂ = X₄² − 2X₃ X₄ + X₃²,
    //   β₁ = −2 [ X₃ X₄² + (X₃² + a) X₄ + (a X₃ + 2b) ],
    //   β₀ = X₃² X₄² − (2 a X₃ + 4b) X₄ + (a² − 4b X₃).
    //
    // Then  S_4(…, X₄) = Res_X(α₂ X² + α₁ X + α₀,  β₂(X₄) X² + β₁(X₄) X + β₀(X₄)).
    //
    // For two quadratics Res = (α₂ β₀ − α₀ β₂)² − (α₂ β₁ − α₁ β₂)(α₁ β₀ − α₀ β₁).

    let p = x1.modulus.clone();
    let zero = FieldElement::zero(p.clone());
    let two = FieldElement::new(BigUint::from(2u32), p.clone());
    let four = FieldElement::new(BigUint::from(4u32), p.clone());

    // β₂(X₄) as [coef_X₄^0, coef_X₄^1, coef_X₄^2].
    let b2 = [x3.mul(x3), two.mul(x3).neg(), one(&p)];
    // β₁(X₄)  =  −2[X₃ X₄² + (X₃² + a) X₄ + (a X₃ + 2b)].
    let b1 = [
        two.mul(&a.mul(x3).add(&two.mul(b))).neg(),
        two.mul(&x3.mul(x3).add(a)).neg(),
        two.mul(x3).neg(),
    ];
    // β₀(X₄)  =  X₃² X₄² − (2 a X₃ + 4b) X₄ + (a² − 4b X₃).
    let b0 = [
        a.mul(a).sub(&four.mul(b).mul(x3)),
        two.mul(a).mul(x3).add(&four.mul(b)).neg(),
        x3.mul(x3),
    ];

    // The resultant components, each itself a polynomial in X₄:
    //   A(X₄) = α₂·β₀ − α₀·β₂
    //   B(X₄) = α₂·β₁ − α₁·β₂
    //   C(X₄) = α₁·β₀ − α₀·β₁
    // and S_4 = A² − B·C.
    let mul_polys = |p1: &[FieldElement], p2: &[FieldElement], deg: usize| -> Vec<FieldElement> {
        let mut out = vec![zero.clone(); deg + 1];
        for (i, ai) in p1.iter().enumerate() {
            for (j, bj) in p2.iter().enumerate() {
                if i + j <= deg {
                    out[i + j] = out[i + j].add(&ai.mul(bj));
                }
            }
        }
        out
    };
    let scale = |c: &FieldElement, poly: &[FieldElement]| -> Vec<FieldElement> {
        poly.iter().map(|t| t.mul(c)).collect()
    };
    let sub_polys = |p1: &[FieldElement], p2: &[FieldElement]| -> Vec<FieldElement> {
        let mut out = p1.to_vec();
        for (i, t) in p2.iter().enumerate() {
            while out.len() <= i {
                out.push(zero.clone());
            }
            out[i] = out[i].sub(t);
        }
        out
    };

    // A(X₄), B(X₄), C(X₄) are polynomials of degree ≤ 2.
    let a_poly = sub_polys(&scale(&a2, &b0), &scale(&a0, &b2));
    let b_poly = sub_polys(&scale(&a2, &b1), &scale(&a1, &b2));
    let c_poly = sub_polys(&scale(&a1, &b0), &scale(&a0, &b1));

    // S_4 = A² − B·C  ∈  F_p[X₄], degree ≤ 4.
    let a_sq = mul_polys(&a_poly, &a_poly, 4);
    let bc = mul_polys(&b_poly, &c_poly, 4);
    let s4 = sub_polys(&a_sq, &bc);

    [
        s4.first().cloned().unwrap_or(zero.clone()),
        s4.get(1).cloned().unwrap_or(zero.clone()),
        s4.get(2).cloned().unwrap_or(zero.clone()),
        s4.get(3).cloned().unwrap_or(zero.clone()),
        s4.get(4).cloned().unwrap_or(zero.clone()),
    ]
}

fn one(p: &BigUint) -> FieldElement {
    FieldElement::one(p.clone())
}

// ── Univariate root finder over F_p ─────────────────────────────────────

/// Find every `F_p`-rational root of the polynomial
/// `coeffs[0] + coeffs[1]·X + coeffs[2]·X² + … + coeffs[d]·X^d`.
///
/// This is what wires Semaev's `S_4` into an actual 3-decomposition
/// sieve: fix `(x_R, x_i, x_j)` from a factor-base sweep, compute the
/// quartic in `X_k` via [`semaev_s4_in_x4`], and pass it here to read
/// off the candidate `x_k` values.
///
/// Algorithm: for fields with `p` up to roughly `2²⁰` we evaluate at
/// every element (Horner's rule, ~`p · d` field ops).  This is the
/// only context where this index-calculus module is exercised in the
/// crate — the toy curve uses `p = 271` — so brute force is more than
/// fast enough.
///
/// For cryptographically-sized `p` the right approach is
/// `gcd(poly(X), X^p − X)` (Cantor-Zassenhaus rational-factor
/// extraction) followed by equal-degree splitting; we do not
/// implement that here, but the function still returns the empty
/// vector rather than panicking so the caller can detect the gap.
pub fn find_roots_fp(coeffs: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    let mut roots = Vec::new();
    if coeffs.is_empty() || coeffs.iter().all(|c| c.is_zero()) {
        return roots;
    }
    // Strip trailing zero coefficients to get the true degree.
    let mut deg = coeffs.len() - 1;
    while deg > 0 && coeffs[deg].is_zero() {
        deg -= 1;
    }
    if deg == 0 {
        // Non-zero constant: no roots.
        return roots;
    }

    let p_bits = p.bits();
    if p_bits > 20 {
        // Out of brute-force range and no CZ wired here — caller
        // gets empty.  See doc comment.
        return roots;
    }
    let p_u64 = p.to_u64_digits().first().copied().unwrap_or(0);
    let coeffs_slice = &coeffs[..=deg];
    for v in 0..p_u64 {
        let elt = FieldElement::new(BigUint::from(v), p.clone());
        // Horner: value = c_d; value = value·X + c_{i} for i = d-1 .. 0.
        let mut value = coeffs_slice[deg].clone();
        for i in (0..deg).rev() {
            value = value.mul(&elt).add(&coeffs_slice[i]);
        }
        if value.is_zero() {
            roots.push(elt);
        }
    }
    roots
}

/// **Fast `F_p`-rational root finding** for small-degree polynomials
/// (degree ≤ 8): Cantor–Zassenhaus over `gcd(f, X^p − X)`.
///
/// Repeated-squaring computes `X^p mod f` in `O(log p)` polynomial
/// multiplications, the root part is `g = gcd(f, X^p − X)`, and
/// equal-degree splitting peels linear factors off `g`.  This is what
/// makes an S₄-based 3-decomposition relation search feasible at 16–32
/// bit primes, where [`find_roots_fp`]'s brute force would evaluate the
/// polynomial at every field element.
pub fn find_roots_fp_fast(coeffs: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    // Small primes: reuse the brute force (it is exact and trivial).
    if p.bits() <= 20 {
        return find_roots_fp(coeffs, p);
    }
    // Normalize to a canonical coefficient slice with nonzero lead.
    let mut f: Vec<FieldElement> = coeffs.to_vec();
    while let Some(last) = f.last() {
        if last.is_zero() {
            f.pop();
        } else {
            break;
        }
    }
    if f.is_empty() {
        // Zero polynomial: every element is a root; the callers here fix
        // finitely many x's, so treat it as "no specific root".
        return Vec::new();
    }
    let deg = f.len() - 1;
    if deg == 0 {
        return Vec::new();
    }
    // Monic copy for the polynomial arithmetic.
    let lead_inv = f[deg].inv().expect("nonzero lead is invertible mod p");
    let monic: Vec<FieldElement> = f.iter().map(|c| c.mul(&lead_inv)).collect();

    // X^p mod f(X) by repeated squaring.
    let x_pow_p = poly_pow_x_mod(&monic, p);
    // g = gcd(f, x_pow_p − X): the product of the linear factors.
    let mut shifted = x_pow_p.clone();
    // shifted -= X  (coefficient of X is index 1).
    while shifted.len() < 2 {
        shifted.push(FieldElement::zero(p.clone()));
    }
    shifted[1] = shifted[1].sub(&FieldElement::one(p.clone()));
    let g = poly_gcd(&monic, &shifted, p);
    let g_deg = g.len() - 1;
    if g_deg == 0 {
        return Vec::new();
    }

    // Equal-degree splitting: all roots of g are in F_p.
    let mut factors: Vec<Vec<FieldElement>> = Vec::new();
    let mut queue = vec![g.clone()];
    while let Some(h) = queue.pop() {
        let h_deg = h.len() - 1;
        if h_deg == 1 {
            factors.push(h);
            continue;
        }
        if h_deg == 2 {
            // Direct quadratic formula (all roots are F_p-rational here).
            let qa = h[2].clone();
            let qb = h[1].clone();
            let qc = h[0].clone();
            if qa.is_zero() {
                if let Some(inv) = qb.inv() {
                    factors.push(vec![qc.neg().mul(&inv), qb]);
                }
            } else {
                let four = FieldElement::new(BigUint::from(4u32), p.clone());
                let disc = qb.mul(&qb).sub(&four.mul(&qa).mul(&qc));
                if let Some(root) = sqrt_mod_p(&disc.value, p) {
                    let two_inv = qa.add(&qa).inv().expect("2a invertible");
                    let s = FieldElement::new(root, p.clone());
                    let r1 = qb.neg().add(&s).mul(&two_inv);
                    let r2 = qb.neg().sub(&s).mul(&two_inv);
                    // Emit both linear factors (x - r).
                    let one = FieldElement::one(p.clone());
                    factors.push(vec![r1.neg(), one.clone()]);
                    factors.push(vec![r2.neg(), one]);
                }
            }
            continue;
        }
        // Try gcd(h, (X + c)^((p-1)/2) − 1) for deterministic c values.
        let exp = (p - BigUint::from(1u32)) / BigUint::from(2u32);
        let mut split = None;
        for c_u32 in 1u32..64 {
            let c = FieldElement::new(BigUint::from(c_u32), p.clone());
            let h_shifted: Vec<FieldElement> = {
                // (X + c) mod h: degree < h_deg.
                let mut hs = h.clone();
                while hs.len() < 2 {
                    hs.push(FieldElement::zero(p.clone()));
                }
                hs[1] = hs[1].add(&c);
                hs
            };
            let pow = poly_pow_mod(&h_shifted, &exp, &h, p);
            let mut minus_one = pow.clone();
            while minus_one.is_empty() {
                minus_one.push(FieldElement::zero(p.clone()));
            }
            minus_one[0] = minus_one[0].sub(&FieldElement::one(p.clone()));
            let d = poly_gcd(&h, &minus_one, p);
            let d_deg = d.len() - 1;
            if d_deg > 0 && d_deg < h_deg {
                split = Some(d);
                break;
            }
        }
        match split {
            Some(d) => {
                let q = poly_div_exact(&h, &d, p);
                queue.push(d);
                queue.push(q);
            }
            None => {
                // Rare: degree > 2 that resisted the small-c sweep.  Give
                // up on this factor (its roots are missed, making the
                // relation search marginally less likely per probe, never
                // wrong).
            }
        }
    }

    let mut roots = Vec::new();
    for lin in factors {
        if lin.len() == 2 && !lin[1].is_zero() {
            // root = −c0 / c1
            if let Some(inv) = lin[1].inv() {
                roots.push(lin[0].neg().mul(&inv));
            }
        } else if lin.len() == 2 {
            // constant-only factor (should not happen after gcd)
        }
    }
    roots.dedup_by(|a, b| a.value == b.value);
    roots
}

/// Polynomial remainder `a mod modulus` over `F_p` (any nonzero lead).
fn poly_rem(a: &[FieldElement], modulus: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    let m_deg = modulus.len() - 1;
    let lead_inv = match modulus[m_deg].inv() {
        Some(v) => v,
        None => return a.to_vec(),
    };
    let mut r = a.to_vec();
    loop {
        while r.len() > 1 && r.last().map(|c| c.is_zero()).unwrap_or(true) {
            r.pop();
        }
        if r.is_empty() || r.len() - 1 < m_deg {
            break;
        }
        let factor = r[r.len() - 1].mul(&lead_inv);
        let shift = r.len() - 1 - m_deg;
        for (i, term) in modulus.iter().enumerate().take(m_deg) {
            r[shift + i] = r[shift + i].sub(&factor.mul(term));
        }
        r.pop();
    }
    let _ = p;
    if r.is_empty() {
        r.push(FieldElement::zero(p.clone()));
    }
    r
}

/// Multiply two polynomials reduced mod a monic modulus.
fn poly_mul_mod(
    a: &[FieldElement],
    b: &[FieldElement],
    modulus: &[FieldElement],
    p: &BigUint,
) -> Vec<FieldElement> {
    let a_deg = a.len().saturating_sub(1);
    let b_deg = b.len().saturating_sub(1);
    let mut acc = vec![FieldElement::zero(p.clone()); a_deg + b_deg + 1];
    for (i, ai) in a.iter().enumerate() {
        if ai.is_zero() {
            continue;
        }
        for (j, bj) in b.iter().enumerate() {
            if bj.is_zero() {
                continue;
            }
            acc[i + j] = acc[i + j].add(&ai.mul(bj));
        }
    }
    poly_rem(&acc, modulus, p)
}

/// `X^p mod f(X)` by left-to-right binary exponentiation on the residue
/// class of `X` (`f` monic).
fn poly_pow_x_mod(f: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    let zero = FieldElement::zero(p.clone());
    let one = FieldElement::one(p.clone());
    let x_res: Vec<FieldElement> = poly_rem(&[zero.clone(), one], f, p);
    let bits = p.to_radix_be(2);
    if bits.is_empty() {
        return x_res;
    }
    let mut result = x_res.clone();
    for bit in bits.iter().skip(1) {
        result = poly_mul_mod(&result, &result, f, p);
        if *bit == 1 {
            result = poly_mul_mod(&result, &x_res, f, p);
        }
    }
    result
}

/// `(poly) ^ exp mod f` for a small base polynomial.
fn poly_pow_mod(
    base: &[FieldElement],
    exp: &BigUint,
    f: &[FieldElement],
    p: &BigUint,
) -> Vec<FieldElement> {
    let zero = FieldElement::zero(p.clone());
    let mut result: Vec<FieldElement> = vec![FieldElement::one(p.clone())];
    let b = poly_rem(base, f, p);
    let bits = exp.to_radix_be(2);
    for bit in bits {
        result = poly_mul_mod(&result, &result, f, p);
        if bit == 1 {
            result = poly_mul_mod(&result, &b, f, p);
        }
    }
    let _ = zero;
    result
}

/// Euclidean GCD of two polynomials over `F_p` (inputs need not be
/// monic; the result is monic).
fn poly_gcd(a: &[FieldElement], b: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    loop {
        while a.len() > 1 && a.last().map(|c| c.is_zero()).unwrap_or(true) {
            a.pop();
        }
        while b.len() > 1 && b.last().map(|c| c.is_zero()).unwrap_or(true) {
            b.pop();
        }
        if b.len() == 1 && b[0].is_zero() {
            break;
        }
        let r = poly_rem(&a, &b, p);
        a = b.clone();
        b = r;
    }
    while a.len() > 1 && a.last().map(|c| c.is_zero()).unwrap_or(true) {
        a.pop();
    }
    if a.is_empty() {
        return vec![FieldElement::zero(p.clone())];
    }
    if let Some(inv) = a[a.len() - 1].inv() {
        for coeff in a.iter_mut() {
            *coeff = coeff.mul(&inv);
        }
    }
    a
}

/// Exact quotient `a / b` for polynomials with `b | a`.
fn poly_div_exact(a: &[FieldElement], b: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    let b_deg = b.len() - 1;
    let lead_inv = b[b_deg].inv().expect("divisor lead is invertible");
    let mut r = a.to_vec();
    let mut quotient = vec![FieldElement::zero(p.clone()); a.len().saturating_sub(b_deg)];
    while r.len() > b_deg && !(r.len() == 1 && r[0].is_zero()) {
        let r_deg = r.len() - 1;
        let factor = r[r_deg].mul(&lead_inv);
        let shift = r_deg - b_deg;
        if quotient.len() <= shift {
            quotient.resize(shift + 1, FieldElement::zero(p.clone()));
        }
        quotient[shift] = factor.clone();
        for (i, term) in b.iter().enumerate() {
            r[shift + i] = r[shift + i].sub(&factor.mul(term));
        }
        while r.len() > 1 && r.last().map(|c| c.is_zero()).unwrap_or(true) {
            r.pop();
        }
        if r.is_empty() {
            break;
        }
    }
    while quotient.len() > 1 && quotient.last().map(|c| c.is_zero()).unwrap_or(true) {
        quotient.pop();
    }
    quotient
}

// ── Tonelli–Shanks square root mod p ────────────────────────────────────

/// Solve `x² ≡ n (mod p)` for a prime `p`.  Returns `Some(x)` if `n`
/// is a quadratic residue mod p, `None` otherwise.  For `n = 0`,
/// returns `Some(0)`.
///
/// Handles every prime `p > 2`; falls back to the `(p+1)/4` shortcut
/// for `p ≡ 3 (mod 4)`.
pub fn sqrt_mod_p(n: &BigUint, p: &BigUint) -> Option<BigUint> {
    let n_mod = n % p;
    if n_mod.is_zero() {
        return Some(BigUint::zero());
    }
    // Word-sized `p`: the same algorithm on `u64`s, with the residue
    // tests as Jacobi symbols.  Same root: the shortcut and the
    // Tonelli–Shanks loop below are deterministic in `n`, `p` and the
    // first non-residue, and the word path finds the same `z`.
    if let ([n], [m]) = (
        n_mod.to_u64_digits().as_slice(),
        p.to_u64_digits().as_slice(),
    ) {
        if *m > 2 && *m % 2 == 1 && *m < 1 << 63 {
            return sqrt_mod_p_u64(*n, *m).map(BigUint::from);
        }
    }
    // Euler's criterion: n^((p-1)/2) must be 1 (QR) or p-1 (NQR).
    let p_minus_one = p - BigUint::one();
    let euler = n_mod.modpow(&(&p_minus_one >> 1), p);
    if euler != BigUint::one() {
        return None;
    }
    // Shortcut for p ≡ 3 (mod 4):  x = n^((p+1)/4) mod p.
    let four = BigUint::from(4u32);
    if (p % &four) == BigUint::from(3u32) {
        let exp = (p + BigUint::one()) >> 2;
        return Some(n_mod.modpow(&exp, p));
    }
    // General Tonelli–Shanks.  Write p − 1 = Q · 2^S with Q odd.
    let mut q = p_minus_one.clone();
    let mut s = 0u32;
    while (&q & BigUint::one()).is_zero() {
        q >>= 1;
        s += 1;
    }
    // Find a quadratic non-residue z.
    let mut z = BigUint::from(2u32);
    while z.modpow(&(&p_minus_one >> 1), p) != p_minus_one {
        z += BigUint::one();
    }
    let mut m = s;
    let mut c = z.modpow(&q, p);
    let mut t = n_mod.modpow(&q, p);
    let mut r = n_mod.modpow(&((&q + BigUint::one()) >> 1), p);
    loop {
        if t == BigUint::one() {
            return Some(r);
        }
        // Find least i, 0 < i < m, with t^(2^i) = 1.
        let mut i = 0u32;
        let mut tmp = t.clone();
        while tmp != BigUint::one() {
            tmp = (&tmp * &tmp) % p;
            i += 1;
            if i >= m {
                // n should be a QR by the Euler check; reach here only
                // if p is composite or our randomness is unlucky.  Bail.
                return None;
            }
        }
        let exp = BigUint::one() << (m - i - 1);
        let b = c.modpow(&exp, p);
        m = i;
        c = (&b * &b) % p;
        t = (t * &c) % p;
        r = (r * b) % p;
    }
}

/// [`sqrt_mod_p`] on words: `0 < n < p`, `p` an odd prime below `2^63`.
///
/// The residue tests are Jacobi symbols, a gcd-style loop, and the
/// exponentiations plain square-and-multiply: what `bitop/v1` prices a
/// `legendre` and a `field_sqrt` as.  The big-integer path's Euler
/// criterion goes through `num-bigint`'s windowed `modpow`, whose cost
/// is a fixed ~90 products whatever the size of `p`.
fn sqrt_mod_p_u64(n: u64, p: u64) -> Option<u64> {
    let mul = |a: u64, b: u64| ((a as u128 * b as u128) % p as u128) as u64;
    let pow = |mut b: u64, mut e: u64| {
        let mut acc = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                acc = mul(acc, b);
            }
            b = mul(b, b);
            e >>= 1;
        }
        acc
    };
    if crate::utils::jacobi_u64(n, p) != 1 {
        return None;
    }
    // Shortcut for p ≡ 3 (mod 4):  x = n^((p+1)/4) mod p.
    if p % 4 == 3 {
        return Some(pow(n, p / 4 + 1));
    }
    // General Tonelli–Shanks.  Write p − 1 = Q · 2^S with Q odd.
    let mut q = p - 1;
    let mut s = 0u32;
    while q.is_multiple_of(2) {
        q /= 2;
        s += 1;
    }
    // Find a quadratic non-residue z.
    let mut z = 2u64;
    while crate::utils::jacobi_u64(z, p) != -1 {
        z += 1;
    }
    let mut m = s;
    let mut c = pow(z, q);
    let mut t = pow(n, q);
    let mut r = pow(n, q.div_ceil(2));
    loop {
        if t == 1 {
            return Some(r);
        }
        // Find least i, 0 < i < m, with t^(2^i) = 1.
        let mut i = 0u32;
        let mut tmp = t;
        while tmp != 1 {
            tmp = mul(tmp, tmp);
            i += 1;
            if i >= m {
                return None;
            }
        }
        let b = pow(c, 1u64 << (m - i - 1));
        m = i;
        c = mul(b, b);
        t = mul(t, c);
        r = mul(r, b);
    }
}

// ── Factor base over E(F_p) ─────────────────────────────────────────────

/// A factor-base element.  We record both the affine point and its
/// index in the base, so when a relation fires we can build the row.
#[derive(Clone, Debug)]
pub struct FactorBaseEntry {
    /// Position in the factor base.
    pub idx: usize,
    /// Point `P_idx = (x, y)` on the curve.
    pub point: Point,
}

/// Build a factor base of size `target_size` by enumerating points with
/// the smallest x-coordinates `x = 1, 2, 3, …` and checking whether
/// `x³ + ax + b` is a quadratic residue mod p.  The y-coordinate is
/// extracted by [`sqrt_mod_p`].
///
/// Returns the list; the actual size is `≤ target_size` because not
/// every x has a corresponding curve point.  Statistically half of all
/// x's are QRs, so expected `2·target_size` enumerations.
pub fn build_factor_base(curve: &CurveParams, target_size: usize) -> Vec<FactorBaseEntry> {
    let mut out = Vec::with_capacity(target_size);
    let mut x = BigUint::one();
    while out.len() < target_size && x < curve.p {
        // rhs = x³ + ax + b mod p
        let xf = curve.fe(x.clone());
        let rhs_value = xf
            .mul(&xf)
            .mul(&xf)
            .add(&curve.a_fe().mul(&xf))
            .add(&curve.fe(curve.b.clone()))
            .value;
        count("sqrt_mod_p_calls_uncharged", 1);
        if let Some(y) = sqrt_mod_p(&rhs_value, &curve.p) {
            // Canonicalise: pick the smaller of (y, p − y) so the
            // factor-base entry is uniquely keyed by x.
            let y_canon = if &y * &BigUint::from(2u32) < curve.p {
                y
            } else {
                &curve.p - &y
            };
            out.push(FactorBaseEntry {
                idx: out.len(),
                point: Point::Affine {
                    x: xf,
                    y: curve.fe(y_canon),
                },
            });
        }
        x += BigUint::one();
    }
    out
}

/// Cofactor-projected factor base: for each small-x point `F` of the curve
/// the entry is `[h]F`, which lies in the prime-order subgroup `<G>` (for
/// `gcd(h, n) = 1`).  Every sum or difference of two entries then lies in
/// `<G>`, so a target `R ∈ <G>` can decompose over every pair rather than
/// the `1/h` of pairs whose torsion components cancel.  Entries are keyed by
/// the x-coordinate of `[h]F`, canonicalised by the smaller y.  Points with
/// `[h]F = O` are skipped.  Opt-in; [`build_factor_base`] is unchanged.
pub fn build_factor_base_projected(
    curve: &CurveParams,
    target_size: usize,
) -> Vec<FactorBaseEntry> {
    let mut out: Vec<FactorBaseEntry> = Vec::with_capacity(target_size);
    let mut seen = std::collections::HashSet::new();
    let a_fe = curve.a_fe();
    let h = BigUint::from(curve.h);
    let mut x = BigUint::one();
    while out.len() < target_size && x < curve.p {
        let xf = curve.fe(x.clone());
        let rhs_value = xf
            .mul(&xf)
            .mul(&xf)
            .add(&curve.a_fe().mul(&xf))
            .add(&curve.fe(curve.b.clone()))
            .value;
        count("sqrt_mod_p_calls_uncharged", 1);
        if let Some(y) = sqrt_mod_p(&rhs_value, &curve.p) {
            let (ha, hd) = scalar_mult_ops(&h);
            count("group_additions", ha);
            count("group_doublings", hd);
            let f = Point::Affine {
                x: xf,
                y: curve.fe(y),
            };
            if let Point::Affine { x: px, y: py } = f.scalar_mul(&h, &a_fe) {
                if seen.insert(px.value.clone()) {
                    let y_canon = if &py.value * &BigUint::from(2u32) < curve.p {
                        py.value.clone()
                    } else {
                        &curve.p - &py.value
                    };
                    out.push(FactorBaseEntry {
                        idx: out.len(),
                        point: Point::Affine {
                            x: px,
                            y: curve.fe(y_canon),
                        },
                    });
                }
            }
        }
        x += BigUint::one();
    }
    out
}

/// `aG + bQ` by one interleaved (Shamir) double-and-add pass over both
/// scalars, with `G + Q` precomputed: about `max(bits)` doublings and `3/4`
/// of that many additions, against two separate ladders plus a final add.
/// Variable time (public scalars).  The result equals
/// `g.scalar_mul(a) + q.scalar_mul(b)` exactly.
pub fn double_scalar_mul_vartime(
    g: &Point,
    a: &BigUint,
    q: &Point,
    b: &BigUint,
    a_fe: &FieldElement,
) -> Point {
    let gq = g.add_vartime(q, a_fe);
    let bits = a.bits().max(b.bits());
    let mut acc = Point::Infinity;
    for i in (0..bits).rev() {
        acc = acc.double_vartime(a_fe);
        match (a.bit(i), b.bit(i)) {
            (true, true) => acc = acc.add_vartime(&gq, a_fe),
            (true, false) => acc = acc.add_vartime(g, a_fe),
            (false, true) => acc = acc.add_vartime(q, a_fe),
            (false, false) => {}
        }
    }
    acc
}

/// `R = aG + bQ` for the relation search.  The default is
/// [`Point::double_scalar_mul_vartime`] (public scalars: variable time, one
/// interleaved word-Jacobian pass).  `IC_SHAMIR` selects an older path, kept
/// for the controlled comparison in `RESULTS_shamir.md`: `0` the original
/// two `scalar_mul` ladders and `add`, `1` the affine interleave, `2` two
/// `scalar_mul_vartime` ladders and `add_vartime`.  All return the same point.
fn relation_point(g: &Point, a: &BigUint, q: &Point, b: &BigUint, a_fe: &FieldElement) -> Point {
    static MODE: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    match *MODE.get_or_init(|| match std::env::var("IC_SHAMIR").as_deref() {
        Ok("0") => 0,
        Ok("1") => 1,
        Ok("2") => 2,
        _ => 3,
    }) {
        0 => {
            count("relation_points_not_counted", 1);
            g.scalar_mul(a, a_fe).add(&q.scalar_mul(b, a_fe), a_fe)
        }
        1 => {
            count("relation_points_not_counted", 1);
            double_scalar_mul_vartime(g, a, q, b, a_fe)
        }
        2 => {
            count("relation_points_not_counted", 1);
            g.scalar_mul_vartime(a, a_fe)
                .add_vartime(&q.scalar_mul_vartime(b, a_fe), a_fe)
        }
        _ => {
            // One interleaved pass: bits - 1 doublings, popcount(a | b) additions
            // counting the first set bit as a load, plus the G + Q precompute.
            let bits = a.bits().max(b.bits());
            let mut pop = 0u64;
            let (mut da, mut db) = (a.iter_u64_digits(), b.iter_u64_digits());
            loop {
                match (da.next(), db.next()) {
                    (None, None) => break,
                    (x, y) => pop += u64::from((x.unwrap_or(0) | y.unwrap_or(0)).count_ones()),
                }
            }
            count("group_doublings", bits.saturating_sub(1));
            count("group_additions", pop);
            g.double_scalar_mul_vartime(a, q, b, a_fe)
        }
    }
}

// ── Relation finding ────────────────────────────────────────────────────

/// One row of the index-calculus linear system.
///
/// Mathematically: `coef_a + coef_b · log_G(Q) ≡ Σⱼ entries[j] · log_G(F_j)
/// (mod n)`, where `entries[j] ∈ {0, ±1, ±2}` records the signed
/// multiplicity of `F_j` in the decomposition.
#[derive(Clone, Debug)]
pub struct Relation {
    /// Coefficient of `G` in `R = aG + bQ`.
    pub coef_a: BigUint,
    /// Coefficient of `Q` in `R = aG + bQ`.
    pub coef_b: BigUint,
    /// Sparse entries: `(factor_base_index, signed_multiplicity)`.
    pub entries: Vec<(usize, i64)>,
}

/// **Find one relation** by sampling random `(a, b)` and trying to
/// decompose `R = aG + bQ` as a signed sum of *two* factor-base
/// elements.  Returns `None` if `max_trials` random samples all fail.
pub fn find_one_relation(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    max_trials: usize,
) -> Option<Relation> {
    find_one_relation_counted(curve, g, q, factor_base, max_trials).map(|(rel, _)| rel)
}

/// Counted twin of [`find_one_relation`]: identical search, additionally
/// reporting how many random `(a, b)` trials were consumed so callers can
/// publish trials-per-relation yield.
pub fn find_one_relation_counted(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    max_trials: usize,
) -> Option<(Relation, usize)> {
    let mut rng = rand::rngs::OsRng;
    find_one_relation_counted_with_rng(curve, g, q, factor_base, max_trials, &mut rng)
}

/// x-coordinate → factor-base index, for O(1) lookups.
fn factor_base_x_index(
    factor_base: &[FactorBaseEntry],
) -> std::collections::HashMap<BigUint, usize> {
    let mut x_to_idx = std::collections::HashMap::new();
    for fb in factor_base {
        if let Point::Affine { x, .. } = &fb.point {
            x_to_idx.insert(x.value.clone(), fb.idx);
        }
    }
    x_to_idx
}

fn find_one_relation_counted_with_rng<R: rand::RngCore>(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    max_trials: usize,
    rng: &mut R,
) -> Option<(Relation, usize)> {
    let x_to_idx = factor_base_x_index(factor_base);
    find_one_relation_sweep_with_index(curve, g, q, factor_base, &x_to_idx, None, max_trials, rng)
}

/// The sweep's inner loop on `u64` words (small odd prime fields below 2^40): the same candidate set,
/// in the same order, as the `BigUint` loop in [`find_one_relation_sweep_with_index`].  Experimental
/// (`IC_FAST_SWEEP=1`, which also hoists the x-index); the default sweep is unchanged.  The design and
/// the equality of results were first established by an independent validator's probe.
struct WordSweep {
    p: u64,
    a: u64,
    b: u64,
    xs: Vec<Option<u64>>,
    in_fb: Vec<bool>,
}

impl WordSweep {
    fn new(curve: &CurveParams, factor_base: &[FactorBaseEntry]) -> Option<Self> {
        let word = |v: &BigUint| v.to_u64_digits().first().copied().unwrap_or(0);
        let pd = curve.p.to_u64_digits();
        if pd.len() != 1 || pd[0] <= 2 || pd[0].is_multiple_of(2) || pd[0] >= (1u64 << 40) {
            return None;
        }
        let xs: Vec<Option<u64>> = factor_base
            .iter()
            .map(|e| match &e.point {
                Point::Affine { x, .. } => Some(word(&x.value)),
                Point::Infinity => None,
            })
            .collect();
        let max_x = xs.iter().flatten().copied().max().unwrap_or(0) as usize;
        let mut in_fb = vec![false; max_x + 1];
        for x in xs.iter().flatten() {
            in_fb[*x as usize] = true;
        }
        Some(Self {
            p: pd[0],
            a: word(&curve.a),
            b: word(&curve.b),
            xs,
            in_fb,
        })
    }

    /// Candidate third coordinates `X` of `S_3(x1, x2, X) = 0` for factor-base element `i`, in the order
    /// the `BigUint` loop tries them (the `+` root, then the `-` root; one candidate in the linear case).
    fn candidates(&self, x1: u64, x2: u64) -> [Option<u64>; 2] {
        let pw = self.p;
        let m = |u: u64, v: u64| ((u as u128 * v as u128) % pw as u128) as u64;
        let ad = |u: u64, v: u64| {
            let s = u + v;
            if s >= pw {
                s - pw
            } else {
                s
            }
        };
        let sb = |u: u64, v: u64| if u >= v { u - v } else { u + pw - v };
        let ng = |u: u64| if u == 0 { 0 } else { pw - u };
        let two_b = ad(self.b, self.b);
        let four_b = ad(two_b, two_b);
        let x1x2 = m(x1, x2);
        let sum = ad(x1, x2);
        let diff = sb(x1, x2);
        let qa = m(diff, diff);
        let inner = ad(m(sum, ad(x1x2, self.a)), two_b);
        let qb = ng(ad(inner, inner));
        let part = sb(x1x2, self.a);
        let qc = sb(m(part, part), m(four_b, sum));
        let mut out = [None, None];
        if qa == 0 {
            if qb == 0 {
                return out;
            }
            if let Some(inv) = inv_mod_u64(qb, pw) {
                out[0] = Some(m(ng(qc), inv));
            }
            return out;
        }
        let four_qa = ad(ad(qa, qa), ad(qa, qa));
        let disc = sb(m(qb, qb), m(four_qa, qc));
        let sq = if disc == 0 {
            0
        } else {
            match sqrt_mod_p_u64(disc, pw) {
                Some(v) => v,
                None => return out,
            }
        };
        let Some(two_qa_inv) = inv_mod_u64(ad(qa, qa), pw) else {
            return out;
        };
        out[0] = Some(m(ad(ng(qb), sq), two_qa_inv));
        out[1] = Some(m(ad(ng(qb), ng(sq)), two_qa_inv));
        out
    }
}

/// The square-root sweep with a prebuilt x-index.  The default path builds the
/// index on every call (as before); `IC_HOIST_SWEEP=1` builds it once per solve.
fn find_one_relation_sweep_with_index<R: rand::RngCore>(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    x_to_idx: &std::collections::HashMap<BigUint, usize>,
    word: Option<&WordSweep>,
    max_trials: usize,
    rng: &mut R,
) -> Option<(Relation, usize)> {
    use num_bigint::RandBigInt;

    let a_fe = curve.a_fe();
    let b_fe = curve.fe(curve.b.clone());

    for trial in 1..=max_trials {
        let mut sample_nonzero = || loop {
            let value = rng.gen_biguint_below(&curve.n);
            if !value.is_zero() {
                break value;
            }
        };
        let a = sample_nonzero();
        let b = sample_nonzero();
        // R = aG + bQ.  Use the variable-time ladder (public k, fine).
        let r = relation_point(g, &a, q, &b, &a_fe);
        let xr = match &r {
            Point::Affine { x, .. } => x.clone(),
            Point::Infinity => continue,
        };

        if let Some(w) = word {
            let x1 = xr.value.to_u64_digits().first().copied().unwrap_or(0);
            for (i, fb_i) in factor_base.iter().enumerate() {
                let Some(x2) = w.xs[i] else { continue };
                for c in w.candidates(x1, x2).into_iter().flatten() {
                    if !((c as usize) < w.in_fb.len() && w.in_fb[c as usize]) {
                        continue;
                    }
                    let candidate = curve.fe(BigUint::from(c));
                    if let Some(rel) = try_finalise(
                        &r,
                        &fb_i.point,
                        &candidate,
                        x_to_idx,
                        factor_base,
                        curve,
                        &a,
                        &b,
                    ) {
                        return Some((rel, trial));
                    }
                }
            }
            continue;
        }

        // Sweep each factor-base element F_i; solve
        // S₃(x_R, x_{F_i}, X) = 0  for X via the quadratic formula.
        for fb_i in factor_base {
            let xi = match &fb_i.point {
                Point::Affine { x, .. } => x.clone(),
                Point::Infinity => continue,
            };
            let (qa, qb, qc) = semaev_s3_in_x3(&xr, &xi, &a_fe, &b_fe);
            // Solve qa·X² + qb·X + qc = 0 mod p.  Discriminant Δ = qb² − 4qa·qc.
            if qa.is_zero() {
                // Linear case: qb X + qc = 0 ⟹ X = −qc · qb⁻¹.
                if qb.is_zero() {
                    continue;
                }
                let inv = match qb.inv() {
                    Some(v) => v,
                    None => continue,
                };
                let candidate = qc.neg().mul(&inv);
                if let Some(rel) = try_finalise(
                    &r,
                    &fb_i.point,
                    &candidate,
                    x_to_idx,
                    factor_base,
                    curve,
                    &a,
                    &b,
                ) {
                    return Some((rel, trial));
                }
                continue;
            }
            let four = curve.fe(BigUint::from(4u32));
            let disc = qb.mul(&qb).sub(&four.mul(&qa).mul(&qc));
            let sqrt_disc = match sqrt_mod_p(&disc.value, &curve.p) {
                Some(v) => v,
                None => continue,
            };
            let two_qa_inv = match qa.add(&qa).inv() {
                Some(v) => v,
                None => continue,
            };
            for sign in [false, true] {
                let s = if sign {
                    curve.fe(&curve.p - &sqrt_disc)
                } else {
                    curve.fe(sqrt_disc.clone())
                };
                let candidate = qb.neg().add(&s).mul(&two_qa_inv);
                if let Some(rel) = try_finalise(
                    &r,
                    &fb_i.point,
                    &candidate,
                    x_to_idx,
                    factor_base,
                    curve,
                    &a,
                    &b,
                ) {
                    return Some((rel, trial));
                }
            }
        }
    }
    None
}

/// Given a candidate decomposition `R = ε_i F_i + ε_j F_j` (with
/// `x_{F_j}` resolved to `candidate_x`), try the four sign combinations
/// `(ε_i, ε_j) ∈ {±1}²` and return the resulting [`Relation`] if any
/// of them is geometrically valid (i.e. `ε_i F_i + ε_j F_j = R`).
fn try_finalise(
    r: &Point,
    f_i: &Point,
    candidate_x: &FieldElement,
    x_to_idx: &std::collections::HashMap<BigUint, usize>,
    factor_base: &[FactorBaseEntry],
    curve: &CurveParams,
    coef_a: &BigUint,
    coef_b: &BigUint,
) -> Option<Relation> {
    let j = *x_to_idx.get(&candidate_x.value)?;
    let f_j = factor_base[j].point.clone();
    let a_fe = curve.a_fe();

    // Try all four sign combinations.  For each match, the relation is
    //   aG + bQ = ε_i F_i + ε_j F_j
    // ⟹ in the unknowns y_k = log_G F_k and x = log_G Q:
    //      ε_i y_i + ε_j y_j − b x ≡ a   (mod n).
    // We store the row as entries that *multiply log_G F_*, and (coef_a,
    // coef_b) so the solver can build the augmented matrix.
    for s_i in [1i64, -1i64] {
        for s_j in [1i64, -1i64] {
            let p_i = if s_i > 0 { f_i.clone() } else { f_i.neg() };
            let p_j = if s_j > 0 { f_j.clone() } else { f_j.neg() };
            let lhs = p_i.add(&p_j, &a_fe);
            if &lhs == r {
                let i_idx = factor_base.iter().position(|fb| &fb.point == f_i)?;
                let mut entries = Vec::new();
                if i_idx == j {
                    // Special case: same factor base index, combine.
                    let combined = s_i + s_j;
                    if combined != 0 {
                        entries.push((i_idx, combined));
                    }
                } else {
                    entries.push((i_idx, s_i));
                    entries.push((j, s_j));
                }
                return Some(Relation {
                    coef_a: coef_a.clone(),
                    coef_b: coef_b.clone(),
                    entries,
                });
            }
        }
    }
    None
}

// ── Semaev S₄ 3-decomposition relation search ────────────────────────

/// **Find one 3-decomposition relation** by sampling random `(a, b)` and
/// decomposing `R = aG + bQ` as a signed sum of *three* factor-base
/// elements.  For each ordered pair `(F_i, F_j)` from the factor base,
/// [`semaev_s4_in_x4`] expands the quartic whose roots are the candidate
/// `x_k` values with `x_i ± x_j ± x_k` summing to `R`; the roots come
/// from [`find_roots_fp_fast`] (Cantor–Zassenhaus).  Counted twin of the
/// 2-decomposition [`find_one_relation`]: reports trials consumed.
///
/// This is the prime-regime `decomposition` stage lever: 3-decomposition
/// relations cover three factor-base columns per row and need
/// `~ p/B³` trials per relation instead of `~ p/B²`.
pub fn find_one_relation_s4_counted(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    max_trials: usize,
) -> Option<(Relation, usize)> {
    use crate::utils::random::random_scalar;

    let a_fe = curve.a_fe();
    let b_fe = curve.fe(curve.b.clone());
    let mut x_to_idx = std::collections::HashMap::new();
    for fb in factor_base {
        if let Point::Affine { x, .. } = &fb.point {
            x_to_idx.insert(x.value.clone(), fb.idx);
        }
    }

    for trial in 1..=max_trials {
        let a = random_scalar(&curve.n);
        let b = random_scalar(&curve.n);
        let ag = g.scalar_mul(&a, &a_fe);
        let bq = q.scalar_mul(&b, &a_fe);
        let r = ag.add(&bq, &a_fe);
        let xr = match &r {
            Point::Affine { x, .. } => x.clone(),
            Point::Infinity => continue,
        };

        // Sweep factor-base pairs (i < j keeps the sweep half-size; the
        // sign resolution below covers both orders).
        for (i, fb_i) in factor_base.iter().enumerate() {
            let xi = match &fb_i.point {
                Point::Affine { x, .. } => x.clone(),
                Point::Infinity => continue,
            };
            for (j, fb_j) in factor_base.iter().enumerate().skip(i + 1) {
                let xj = match &fb_j.point {
                    Point::Affine { x, .. } => x.clone(),
                    Point::Infinity => continue,
                };
                let quartic = semaev_s4_in_x4(&xr, &xi, &xj, &a_fe, &b_fe);
                for xk in find_roots_fp_fast(&quartic, &curve.p) {
                    let k = match x_to_idx.get(&xk.value).copied() {
                        Some(k) => k,
                        None => continue,
                    };
                    if let Some(rel) = try_finalise_s4(
                        &r,
                        i,
                        &fb_i.point,
                        j,
                        &fb_j.point,
                        k,
                        &factor_base[k].point,
                        curve,
                        &a,
                        &b,
                    ) {
                        return Some((rel, trial));
                    }
                }
            }
        }
    }
    None
}

/// Sign resolution for the 3-point decomposition: try all eight sign
/// combinations `(ε_i, ε_j, ε_k) ∈ {±1}³` and accept the one with
/// `ε_i F_i + ε_j F_j + ε_k F_k = R`.
#[allow(clippy::too_many_arguments)]
fn try_finalise_s4(
    r: &Point,
    i: usize,
    f_i: &Point,
    j: usize,
    f_j: &Point,
    k: usize,
    f_k: &Point,
    curve: &CurveParams,
    coef_a: &BigUint,
    coef_b: &BigUint,
) -> Option<Relation> {
    let a_fe = curve.a_fe();
    // aG + bQ = ε_i F_i + ε_j F_j + ε_k F_k
    // ⟹ ε_i y_i + ε_j y_j + ε_k y_k − b x ≡ a (mod n).
    for s_i in [1i64, -1i64] {
        for s_j in [1i64, -1i64] {
            for s_k in [1i64, -1i64] {
                let p_i = if s_i > 0 { f_i.clone() } else { f_i.neg() };
                let p_j = if s_j > 0 { f_j.clone() } else { f_j.neg() };
                let p_k = if s_k > 0 { f_k.clone() } else { f_k.neg() };
                let lhs = p_i.add(&p_j, &a_fe).add(&p_k, &a_fe);
                if &lhs == r {
                    let mut entries: Vec<(usize, i64)> = vec![(i, s_i), (j, s_j), (k, s_k)];
                    // Merge duplicate factor-base indices.
                    entries.sort_unstable();
                    let mut merged: Vec<(usize, i64)> = Vec::new();
                    for (idx, sign) in entries {
                        if let Some(last) = merged.last_mut() {
                            if last.0 == idx {
                                last.1 += sign;
                                continue;
                            }
                        }
                        merged.push((idx, sign));
                    }
                    merged.retain(|(_, sign)| *sign != 0);
                    if merged.is_empty() {
                        return None;
                    }
                    return Some(Relation {
                        coef_a: coef_a.clone(),
                        coef_b: coef_b.clone(),
                        entries: merged,
                    });
                }
            }
        }
    }
    None
}

// ── Gaussian elimination mod n ─────────────────────────────────────────
/// **Solve** `M · y ≡ rhs (mod n)` for `y`, where `M` is given as rows
/// and `n` is prime (so every nonzero element is invertible).  Returns
/// `None` if the system is inconsistent, under-determined (any column
/// lacks a pivot), or a pivot is not invertible mod `n`.
///
/// Callers that need only some unknowns pinned (typically `log_G Q`
/// with unused factor-base columns left free) use
/// [`gaussian_eliminate_mod_n_particular`] and check
/// [`ModNSolution::determined`] for the columns they read.
///
/// `matrix` and `rhs` are left in reduced row-echelon form.
pub fn gaussian_eliminate_mod_n(
    matrix: &mut [Vec<BigUint>],
    rhs: &mut [BigUint],
    n: &BigUint,
) -> Option<Vec<BigUint>> {
    let sol = gaussian_eliminate_mod_n_particular(matrix, rhs, n)?;
    sol.determined.iter().all(|&d| d).then_some(sol.values)
}

/// A consistent solution of `M · y ≡ rhs (mod n)` from
/// [`gaussian_eliminate_mod_n_particular`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModNSolution {
    /// The particular solution with every free unknown set to zero.
    pub values: Vec<BigUint>,
    /// `determined[c]` is true exactly when `y_c` takes the same value
    /// in every solution: `c` has a pivot and its reduced row has no
    /// entry in a free column.  Only determined values are answers.
    pub determined: Vec<bool>,
    /// Rank of `M`.
    pub rank: usize,
}

/// Rank-checked Gauss–Jordan elimination mod prime `n` that tolerates
/// free unknowns.  Returns `None` only when the system is inconsistent
/// or a pivot is not invertible; otherwise the particular solution with
/// free unknowns zero and, per column, whether that unknown is
/// determined.  A value whose `determined` flag is false is arbitrary
/// and must not be reported as a logarithm.
pub fn gaussian_eliminate_mod_n_particular(
    matrix: &mut [Vec<BigUint>],
    rhs: &mut [BigUint],
    n: &BigUint,
) -> Option<ModNSolution> {
    let m = matrix.first().map(|r| r.len()).unwrap_or(0);
    let n64 = n
        .to_u64_digits()
        .first()
        .copied()
        .filter(|&v| n.bits() <= 64 && v > 1);
    // Both paths leave `matrix`/`rhs` in reduced row-echelon form with
    // pivot rows 0..rank, and return the pivot row of each column.
    let pivot_rows = match n64 {
        Some(n64) => gaussian_eliminate_mod_u64(matrix, rhs, n64)?,
        None => gaussian_eliminate_mod_biguint(matrix, rhs, n)?,
    };
    let rank = pivot_rows.iter().filter(|&&r| r != usize::MAX).count();
    // Counted for the `bitop/v1` head-to-head: one word op per zero test
    // and per determined-flag compare below.
    bitop_ops::word_op((rhs.len().saturating_sub(rank)) as u64);
    // Rows below the pivots are all-zero on the left; a nonzero
    // right-hand side there is `0 = c`.
    if rhs.iter().skip(rank).any(|v| !(v % n).is_zero()) {
        return None;
    }
    let free: Vec<usize> = (0..m).filter(|&c| pivot_rows[c] == usize::MAX).collect();
    let mut values = vec![BigUint::zero(); m];
    let mut determined = vec![false; m];
    for c in 0..m {
        let pr = pivot_rows[c];
        if pr == usize::MAX {
            continue;
        }
        values[c] = rhs[pr].clone();
        bitop_ops::word_op(free.len() as u64);
        determined[c] = free.iter().all(|&f| matrix[pr][f].is_zero());
    }
    Some(ModNSolution {
        values,
        determined,
        rank,
    })
}

/// The arbitrary-precision path of [`gaussian_eliminate_mod_n`].
fn gaussian_eliminate_mod_biguint(
    matrix: &mut [Vec<BigUint>],
    rhs: &mut [BigUint],
    n: &BigUint,
) -> Option<Vec<usize>> {
    let rows = matrix.len();
    let cols = matrix.first().map(|r| r.len()).unwrap_or(0);
    let m = cols; // number of unknowns

    let mut col = 0;
    let mut row = 0;
    let mut pivot_rows: Vec<usize> = vec![usize::MAX; m];

    while row < rows && col < m {
        // Find a row at index ≥ `row` with a non-zero entry in column `col`.
        let mut piv = None;
        for r in row..rows {
            bitop_ops::word_op(1);
            if !matrix[r][col].is_zero() {
                piv = Some(r);
                break;
            }
        }
        let piv = match piv {
            Some(p) => p,
            None => {
                col += 1;
                continue;
            }
        };
        matrix.swap(row, piv);
        rhs.swap(row, piv);
        pivot_rows[col] = row;

        // Normalise the pivot row.
        // Left of `col` the pivot row is already zero (earlier pivot
        // columns were eliminated from it, free columns were zero in
        // every row still below the diagonal), so every sweep starts
        // at `col`.
        bitop_ops::modr_inv(1);
        let inv = mod_inverse(&matrix[row][col], n)?;
        bitop_ops::modr_mul((m - col) as u64 + 1);
        for c in col..m {
            matrix[row][c] = (&matrix[row][c] * &inv) % n;
        }
        rhs[row] = (&rhs[row] * &inv) % n;

        // Eliminate column `col` in every other row.
        for r in 0..rows {
            if r == row {
                continue;
            }
            bitop_ops::word_op(1);
            if matrix[r][col].is_zero() {
                continue;
            }
            // `m − col + 1` multiply-subtracts.
            bitop_ops::modr_mul((m - col) as u64 + 1);
            bitop_ops::modr_add((m - col) as u64 + 1);
            let factor = matrix[r][col].clone();
            for c in col..m {
                let term = (&factor * &matrix[row][c]) % n;
                matrix[r][c] = (&matrix[r][c] + n - term) % n;
            }
            let term = (&factor * &rhs[row]) % n;
            rhs[r] = (&rhs[r] + n - term) % n;
        }
        row += 1;
        col += 1;
    }

    Some(pivot_rows)
}

/// [`gaussian_eliminate_mod_n`] for a word-sized modulus: the same
/// Gauss–Jordan elimination, pivot choice and in-place contract, on
/// `u64` rows with `u128` products instead of `BigUint`s.  Each row
/// update is a tight allocation-free loop, where the `BigUint` path
/// allocates on every multiply and every reduction.
///
/// Inputs are reduced mod `n` on the way in, and the reduced matrix
/// and right-hand side are written back so callers that inspect them
/// afterwards see the same echelon form.
fn gaussian_eliminate_mod_u64(
    matrix: &mut [Vec<BigUint>],
    rhs: &mut [BigUint],
    n: u64,
) -> Option<Vec<usize>> {
    let rows = matrix.len();
    let m = matrix.first().map(|r| r.len()).unwrap_or(0);
    let nb = BigUint::from(n);
    let to_u64 = |v: &BigUint| -> u64 {
        if v.bits() <= 64 {
            v.to_u64_digits().first().copied().unwrap_or(0) % n
        } else {
            (v % &nb).to_u64_digits().first().copied().unwrap_or(0)
        }
    };
    let mut a: Vec<u64> = Vec::with_capacity(rows * m);
    for r in matrix.iter() {
        a.extend(r.iter().take(m).map(to_u64));
    }
    let mut b: Vec<u64> = rhs.iter().map(to_u64).collect();
    let mulmod = |x: u64, y: u64| ((x as u128 * y as u128) % n as u128) as u64;

    let mut pivot_rows: Vec<usize> = vec![usize::MAX; m];
    let mut failed = false;
    let (mut row, mut col) = (0usize, 0usize);
    while row < rows && col < m {
        let Some(piv) = (row..rows).find(|&r| {
            bitop_ops::word_op(1);
            a[r * m + col] != 0
        }) else {
            col += 1;
            continue;
        };
        if piv != row {
            for c in col..m {
                a.swap(row * m + c, piv * m + c);
            }
            b.swap(row, piv);
            matrix.swap(row, piv);
        }
        pivot_rows[col] = row;
        bitop_ops::modr_inv(1);
        let Some(inv) = inv_mod_u64(a[row * m + col], n) else {
            failed = true;
            break;
        };
        bitop_ops::modr_mul((m - col) as u64 + 1);
        for c in col..m {
            a[row * m + c] = mulmod(a[row * m + c], inv);
        }
        b[row] = mulmod(b[row], inv);

        let (before, rest) = a.split_at_mut(row * m);
        let (prow, after) = rest.split_at_mut(m);
        let prow = &prow[col..];
        let brow = b[row];
        let eliminate = |target: &mut [u64], br: &mut u64| {
            let factor = target[col];
            bitop_ops::word_op(1);
            if factor == 0 {
                return;
            }
            // The negation, then `m − col + 1` multiply-adds.
            bitop_ops::modr_add((m - col) as u64 + 2);
            bitop_ops::modr_mul((m - col) as u64 + 1);
            let neg = n - factor;
            for (t, &p) in target[col..].iter_mut().zip(prow) {
                *t = ((*t as u128 + neg as u128 * p as u128) % n as u128) as u64;
            }
            *br = ((*br as u128 + neg as u128 * brow as u128) % n as u128) as u64;
        };
        for (r, target) in before.chunks_exact_mut(m).enumerate() {
            eliminate(target, &mut b[r]);
        }
        for (i, target) in after.chunks_exact_mut(m).enumerate() {
            eliminate(target, &mut b[row + 1 + i]);
        }
        row += 1;
        col += 1;
    }

    // Write the reduced system back, as the `BigUint` path leaves it.
    // Rows were already swapped in `matrix` alongside `a`.
    for (r, out) in matrix.iter_mut().enumerate() {
        for (c, v) in out.iter_mut().take(m).enumerate() {
            *v = BigUint::from(a[r * m + c]);
        }
    }
    for (r, out) in rhs.iter_mut().enumerate() {
        if r < b.len() {
            *out = BigUint::from(b[r]);
        }
    }
    (!failed).then_some(pivot_rows)
}

/// `a^{-1} mod n` by the extended Euclidean algorithm, `None` when
/// `gcd(a, n) ≠ 1`.
fn inv_mod_u64(a: u64, n: u64) -> Option<u64> {
    let (mut old_r, mut r) = (a as i128, n as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    if old_r != 1 {
        return None;
    }
    Some(old_s.rem_euclid(n as i128) as u64)
}

// ── End-to-end ECDLP solver via index calculus ─────────────────────────

/// **Solve `Q = x · G` for `x`** on `curve` using the Semaev-S₃ index-
/// calculus framework.  `fb_size` is the desired factor base size,
/// `extra_relations` is how many relations beyond the minimum to
/// collect, `max_trials_per_relation` bounds the trial loop.
///
/// Returns `Some(x)` on success; `None` if relation gathering or the
/// linear solve fails.  Intended for educational use on **small**
/// curves; see module docs for asymptotic complexity.
pub fn ec_index_calculus_dlp(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
) -> Option<BigUint> {
    let fb = build_factor_base(curve, fb_size);
    if fb.is_empty() {
        return None;
    }
    let target = fb.len() + extra_relations;

    let mut relations = Vec::with_capacity(target);
    while relations.len() < target {
        let rel = find_one_relation(curve, g, q, &fb, max_trials_per_relation)?;
        relations.push(rel);
    }

    // Build the linear system.  Unknowns: (y_1, …, y_m, x = log_G Q).
    // Row r:   Σ entries[j].value y_j  − coef_b · x  ≡ coef_a (mod n)
    // i.e.    entries  ‖ −coef_b   ·   (y, x)  =  coef_a.
    let m = fb.len();
    let mut matrix: Vec<Vec<BigUint>> = Vec::with_capacity(relations.len());
    let mut rhs: Vec<BigUint> = Vec::with_capacity(relations.len());
    for rel in &relations {
        let mut row = vec![BigUint::zero(); m + 1];
        for &(j, mult) in &rel.entries {
            let val = signed_mod(mult, &curve.n);
            row[j] = (&row[j] + &val) % &curve.n;
        }
        // last column: −coef_b
        let neg_b = (&curve.n - &(&rel.coef_b % &curve.n)) % &curve.n;
        row[m] = neg_b;
        matrix.push(row);
        rhs.push(rel.coef_a.clone() % &curve.n);
    }

    // Only `x` must be pinned; unused factor-base columns may stay free.
    let solution = gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, &curve.n)?;
    if !solution.determined[m] {
        return None;
    }
    let x = solution.values[m].clone();
    // Verify Q ≡ x·G independently of the linear algebra.
    let a_fe = curve.a_fe();
    let candidate = g.scalar_mul(&x, &a_fe);
    if &candidate == q {
        Some(x)
    } else {
        None
    }
}

fn signed_mod(v: i64, n: &BigUint) -> BigUint {
    if v >= 0 {
        BigUint::from(v as u64) % n
    } else {
        n - (BigUint::from((-v) as u64) % n)
    }
}

// ── Staged (split-timer) end-to-end driver ───────────────────────────

/// Split-stage timing report for the generic prime-field IC pipeline.
///
/// Every field is wall-clock milliseconds measured inside
/// [`ec_index_calculus_dlp_staged`]; the relation counters let callers
/// publish a trials-per-relation yield curve from the same run.
#[derive(Debug, Clone, serde::Serialize)]
pub struct EcIcStageReport {
    pub factor_base_ms: f64,
    pub relations_ms: f64,
    pub linear_algebra_ms: f64,
    pub verify_ms: f64,
    pub total_ms: f64,
    pub factor_base_size: usize,
    pub relations_collected: usize,
    pub relation_attempts_exhausted: usize,
    pub trials_total: usize,
    pub trials_per_relation_min: usize,
    pub trials_per_relation_median: f64,
    pub trials_per_relation_max: usize,
    /// Charged group operations (`group_additions`, `group_doublings`) and the
    /// native operations the unit does not charge (`*_uncharged`), counted for
    /// the default path (pair-sum table, joint relation point).  Empty when a
    /// path that does not count is used.
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub counters: std::collections::BTreeMap<String, u64>,
    /// Counters this solve names but does not emit; an arm that lists any is a
    /// lower bound at every weight of the band (AGENTS.md section 5a).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub counters_unemitted: Vec<&'static str>,
}

/// Staged twin of [`ec_index_calculus_dlp`]: same mathematics and row
/// policy, but each stage is timed separately (factor base → relations
/// → linear algebra → verify) and relation-search trials are counted.
///
/// `max_relation_attempts` bounds how many `max_trials_per_relation`
/// search calls a single relation may consume before the whole solve
/// gives up (`None`); the single-attempt original returns `None`
/// immediately where this one retries.
pub fn ec_index_calculus_dlp_staged(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
) -> Option<(BigUint, EcIcStageReport)> {
    let mut rng = rand::rngs::OsRng;
    ec_index_calculus_dlp_staged_with_rng(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        false,
        &mut rng,
    )
}

/// Run the staged prime-field IC pipeline with a reproducible relation sampler.
/// The seed controls relation sampling only; wall times remain host-dependent.
pub fn ec_index_calculus_dlp_staged_seeded(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    seed: u64,
) -> Option<(BigUint, EcIcStageReport)> {
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    ec_index_calculus_dlp_staged_with_rng(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        false,
        &mut rng,
    )
}

/// Seeded staged pipeline over the cofactor-projected factor base
/// ([`build_factor_base_projected`]); otherwise identical to
/// [`ec_index_calculus_dlp_staged_seeded`].
pub fn ec_index_calculus_dlp_staged_seeded_projected(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    seed: u64,
) -> Option<(BigUint, EcIcStageReport)> {
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    ec_index_calculus_dlp_staged_with_rng(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        true,
        &mut rng,
    )
}

/// Pair-sum table for 2-decomposition: maps the affine point `±F_i ± F_j`
/// (`i <= j`) to `(i, s_i, j, s_j)`.  Built once per solve, so a relation
/// trial is one hash lookup instead of a square-root sweep over the factor
/// base.  Experimental (`IC_SUM_TABLE=1`); the default sweep is unchanged.
pub type PairSumTableExact =
    std::collections::HashMap<(BigUint, BigUint), (usize, i64, usize, i64)>;

/// Largest factor base the compact table indexes (14 bits per index).
pub const PAIR_SUM_TABLE_MAX_FB: usize = 1 << 14;

fn x_fingerprint(x: &BigUint) -> u64 {
    // splitmix64 fold over the 32-bit limbs; only a probe key (every hit is checked against the point)
    let mut h = 0x9E37_79B9_7F4A_7C15u64;
    for limb in x.iter_u32_digits() {
        h ^= u64::from(limb);
        h = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = h;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        h = z ^ (z >> 31);
    }
    h
}

/// Compact pair-sum table: open addressing over a 64-bit fingerprint of the x coordinate plus a
/// packed `(i, s_i, j, s_j)`, 12 bytes a slot (about 20 per stored sum at load 0.6) instead of about 170 for the exact hash map. A hit
/// is checked against the factor-base points (one addition, only on a hit), so a fingerprint
/// collision is a miss, never a wrong relation; the hit set equals the exact table's (both
/// signs of every pair sum are stored there, so a hit is "x is a pair-sum x"). Entries with the
/// same x are all kept, in insertion order, and the first valid one is returned, which is the
/// exact table's first-wins choice.
pub struct PairSumTable {
    fps: Vec<u64>,
    vals: Vec<u32>,
    points: Vec<Point>,
    a_fe: FieldElement,
    entries: usize,
}

impl PairSumTable {
    fn slot(&self, fp: u64) -> usize {
        ((u128::from(fp) * self.fps.len() as u128) >> 64) as usize
    }

    fn pack(i: usize, s_i: i64, j: usize, s_j: i64) -> u32 {
        let sign = |s: i64| u32::from(s < 0);
        1 + ((((i as u32) << 14 | j as u32) << 2) | sign(s_i) << 1 | sign(s_j))
    }

    fn unpack(v: u32) -> (usize, i64, usize, i64) {
        let v = v - 1;
        let sg = |b: u32| if b == 1 { -1i64 } else { 1 };
        (
            (v >> 16) as usize,
            sg((v >> 1) & 1),
            ((v >> 2) & 0x3FFF) as usize,
            sg(v & 1),
        )
    }

    fn insert(&mut self, x: &BigUint, key: (usize, i64, usize, i64)) {
        let fp = x_fingerprint(x);
        let mut at = self.slot(fp);
        while self.vals[at] != 0 {
            at += 1;
            if at == self.fps.len() {
                at = 0;
            }
        }
        self.fps[at] = fp;
        self.vals[at] = Self::pack(key.0, key.1, key.2, key.3);
        self.entries += 1;
    }

    /// Number of stored sums.
    pub fn len(&self) -> usize {
        self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries == 0
    }

    /// Bytes held by the table's slots.
    pub fn slot_bytes(&self) -> usize {
        self.fps.len() * 12
    }

    /// The decomposition `r = s_i F_i + s_j F_j` of the affine point `(x, y)`, if it is a pair sum.
    /// Among entries with the same x it returns the first one stored for exactly `(x, y)`, as the
    /// exact table does, and only if there is none the first one for `(x, -y)` with both signs flipped.
    pub fn find(&self, x: &BigUint, y: &BigUint) -> Option<(usize, i64, usize, i64)> {
        let fp = x_fingerprint(x);
        let mut at = self.slot(fp);
        let mut flipped = None;
        while self.vals[at] != 0 {
            if self.fps[at] == fp {
                let (i, s_i, j, s_j) = Self::unpack(self.vals[at]);
                let pi = if s_i > 0 {
                    self.points[i].clone()
                } else {
                    self.points[i].neg_vartime()
                };
                let pj = if s_j > 0 {
                    self.points[j].clone()
                } else {
                    self.points[j].neg_vartime()
                };
                if let Point::Affine { x: sx, y: sy } = pi.add_vartime(&pj, &self.a_fe) {
                    if &sx.value == x {
                        if &sy.value == y {
                            return Some((i, s_i, j, s_j));
                        }
                        flipped.get_or_insert((i, -s_i, j, -s_j));
                    }
                }
            }
            at += 1;
            if at == self.fps.len() {
                at = 0;
            }
        }
        flipped
    }
}

pub fn build_pair_sum_table(curve: &CurveParams, factor_base: &[FactorBaseEntry]) -> PairSumTable {
    assert!(
        factor_base.len() <= PAIR_SUM_TABLE_MAX_FB,
        "compact pair-sum table indexes at most {PAIR_SUM_TABLE_MAX_FB} factor-base points"
    );
    let a_fe = curve.a_fe();
    let f = factor_base.len();
    let slots = (f * f * 2 * 5 / 3).max(16);
    let mut table = PairSumTable {
        fps: vec![0; slots],
        vals: vec![0; slots],
        points: factor_base.iter().map(|e| e.point.clone()).collect(),
        a_fe: a_fe.clone(),
        entries: 0,
    };
    let mut put = |pt: &Point, key: (usize, i64, usize, i64)| {
        if let Point::Affine { x, .. } = pt {
            table.insert(&x.value, key);
        }
    };
    for (i, fi) in factor_base.iter().enumerate() {
        for (j, fj) in factor_base.iter().enumerate().skip(i) {
            let sum = fi.point.add_vartime(&fj.point, &a_fe);
            put(&sum, (i, 1, j, 1));
            put(&sum.neg_vartime(), (i, -1, j, -1));
            count("group_additions", 1);
            count("table_insert_attempts_uncharged", 2);
            count("point_negations_uncharged", 1);
            if i != j {
                let dif = fi.point.add_vartime(&fj.point.neg_vartime(), &a_fe);
                put(&dif, (i, 1, j, -1));
                put(&dif.neg_vartime(), (i, -1, j, 1));
                count("group_additions", 1);
                count("table_insert_attempts_uncharged", 2);
                count("point_negations_uncharged", 2);
            }
        }
    }
    table
}

/// Exact pair-sum table (the reference the compact table is tested against; about 170 bytes an entry).
pub fn build_pair_sum_table_exact(
    curve: &CurveParams,
    factor_base: &[FactorBaseEntry],
) -> PairSumTableExact {
    let a_fe = curve.a_fe();
    let mut table = PairSumTableExact::with_capacity(factor_base.len() * factor_base.len() * 2);
    let mut put = |pt: &Point, key: (usize, i64, usize, i64)| {
        if let Point::Affine { x, y } = pt {
            table
                .entry((x.value.clone(), y.value.clone()))
                .or_insert(key);
        }
    };
    // Negation is free on a curve: -(F_i + F_j) = (-F_i) + (-F_j) and -(F_i - F_j) = (-F_i) + F_j, so two
    // (variable-time) additions per pair give all four sign combinations.
    for (i, fi) in factor_base.iter().enumerate() {
        for (j, fj) in factor_base.iter().enumerate().skip(i) {
            let sum = fi.point.add_vartime(&fj.point, &a_fe);
            put(&sum, (i, 1, j, 1));
            put(&sum.neg_vartime(), (i, -1, j, -1));
            count("group_additions", 1);
            count("table_insert_attempts_uncharged", 2);
            count("point_negations_uncharged", 1);
            if i != j {
                let dif = fi.point.add_vartime(&fj.point.neg_vartime(), &a_fe);
                put(&dif, (i, 1, j, -1));
                put(&dif.neg_vartime(), (i, -1, j, 1));
                count("group_additions", 1);
                count("table_insert_attempts_uncharged", 2);
                count("point_negations_uncharged", 2);
            }
        }
    }
    table
}

/// Same sampling as [`find_one_relation_counted_with_rng`] (one trial = one
/// random `R = aG + bQ`), but the decomposition test is a table lookup.
fn find_one_relation_table_with_rng<R: rand::RngCore>(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    table: &PairSumTable,
    max_trials: usize,
    rng: &mut R,
) -> Option<(Relation, usize)> {
    use num_bigint::RandBigInt;
    let a_fe = curve.a_fe();
    for trial in 1..=max_trials {
        let mut sample_nonzero = || loop {
            let value = rng.gen_biguint_below(&curve.n);
            if !value.is_zero() {
                break value;
            }
        };
        let a = sample_nonzero();
        let b = sample_nonzero();
        count("random_scalars_drawn_uncharged", 2);
        let r = relation_point(g, &a, q, &b, &a_fe);
        let key = match &r {
            Point::Affine { x, y } => (x.value.clone(), y.value.clone()),
            Point::Infinity => continue,
        };
        count("table_probes_uncharged", 1);
        if let Some((i, s_i, j, s_j)) = table.find(&key.0, &key.1) {
            let mut entries = Vec::new();
            if i == j {
                let combined = s_i + s_j;
                if combined != 0 {
                    entries.push((i, combined));
                }
            } else {
                entries.push((i, s_i));
                entries.push((j, s_j));
            }
            return Some((
                Relation {
                    coef_a: a,
                    coef_b: b,
                    entries,
                },
                trial,
            ));
        }
    }
    None
}

/// r-adding walk state for relation sampling (experimental, `IC_WALK=1` with `IC_SUM_TABLE=1`).
/// Each step adds one of `WALK_ARMS` precomputed points `a_k G + b_k Q` to the current point,
/// tracking (a, b) mod n, so a trial costs one point addition instead of two scalar multiplications.
struct SampleWalk {
    r: Point,
    a: BigUint,
    b: BigUint,
    arms: Vec<(Point, BigUint, BigUint)>,
    steps: usize,
}

const WALK_ARMS: usize = 32;
/// A walk whose step depends only on x enters a cycle after about sqrt(n) steps, so it is
/// restarted from fresh random coefficients every `WALK_RESTART` steps.
const WALK_RESTART: usize = 32;

impl SampleWalk {
    fn new<R: rand::RngCore>(curve: &CurveParams, g: &Point, q: &Point, rng: &mut R) -> Self {
        use num_bigint::RandBigInt;
        let a_fe = curve.a_fe();
        let nonzero = |rng: &mut R| loop {
            let v = rng.gen_biguint_below(&curve.n);
            if !v.is_zero() {
                break v;
            }
        };
        let mut arms = Vec::with_capacity(WALK_ARMS);
        for _ in 0..WALK_ARMS {
            let (a, b) = (nonzero(rng), nonzero(rng));
            let pt = relation_point(g, &a, q, &b, &a_fe);
            arms.push((pt, a, b));
        }
        let (a, b) = (nonzero(rng), nonzero(rng));
        let r = relation_point(g, &a, q, &b, &a_fe);
        SampleWalk {
            r,
            a,
            b,
            arms,
            steps: 0,
        }
    }

    fn restart<R: rand::RngCore>(
        &mut self,
        curve: &CurveParams,
        g: &Point,
        q: &Point,
        rng: &mut R,
    ) {
        use num_bigint::RandBigInt;
        let a_fe = curve.a_fe();
        let nonzero = |rng: &mut R| loop {
            let v = rng.gen_biguint_below(&curve.n);
            if !v.is_zero() {
                break v;
            }
        };
        let (a, b) = (nonzero(rng), nonzero(rng));
        self.r = relation_point(g, &a, q, &b, &a_fe);
        self.a = a;
        self.b = b;
        self.steps = 0;
    }

    fn step(&mut self, curve: &CurveParams) {
        let a_fe = curve.a_fe();
        self.steps += 1;
        let k = match &self.r {
            Point::Affine { x, .. } => {
                (x.value.iter_u64_digits().next().unwrap_or(0) as usize) % WALK_ARMS
            }
            Point::Infinity => 0,
        };
        let (pt, ak, bk) = &self.arms[k];
        self.r = self.r.add_vartime(pt, &a_fe);
        self.a = (&self.a + ak) % &curve.n;
        self.b = (&self.b + bk) % &curve.n;
    }
}

/// Table lookup on an r-adding walk instead of fresh random (a, b).
fn find_one_relation_table_walk<R: rand::RngCore>(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    table: &PairSumTable,
    walk: &mut SampleWalk,
    max_trials: usize,
    rng: &mut R,
) -> Option<(Relation, usize)> {
    for trial in 1..=max_trials {
        if walk.steps >= WALK_RESTART || matches!(walk.r, Point::Infinity) {
            walk.restart(curve, g, q, rng);
        }
        walk.step(curve);
        let key = match &walk.r {
            Point::Affine { x, y } => (x.value.clone(), y.value.clone()),
            Point::Infinity => continue,
        };
        if let Some((i, s_i, j, s_j)) = table.find(&key.0, &key.1) {
            let mut entries = Vec::new();
            if i == j {
                let combined = s_i + s_j;
                if combined != 0 {
                    entries.push((i, combined));
                }
            } else {
                entries.push((i, s_i));
                entries.push((j, s_j));
            }
            return Some((
                Relation {
                    coef_a: walk.a.clone(),
                    coef_b: walk.b.clone(),
                    entries,
                },
                trial,
            ));
        }
    }
    None
}

fn ec_index_calculus_dlp_staged_with_rng<R: rand::RngCore>(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    project_cofactor: bool,
    rng: &mut R,
) -> Option<(BigUint, EcIcStageReport)> {
    use std::time::Instant;
    reset_counters();
    let total_started = Instant::now();

    let fb_started = Instant::now();
    let fb = if project_cofactor {
        build_factor_base_projected(curve, fb_size)
    } else {
        build_factor_base(curve, fb_size)
    };
    let factor_base_ms = fb_started.elapsed().as_secs_f64() * 1e3;
    if fb.is_empty() {
        return None;
    }
    let mut target = fb.len() + extra_relations;

    let relations_started = Instant::now();
    // Experimental: IC_SUM_TABLE=1 swaps the per-trial sweep for a pair-sum table lookup.
    // Table construction is charged to the relation-collection stage.
    // Default: the table for a small factor base (its build is O(fb^2) point
    // additions and it loses from fb about 128); `IC_SUM_TABLE=1` forces it,
    // `IC_SUM_TABLE=0` selects the per-trial sweep.
    let use_table = match std::env::var("IC_SUM_TABLE").as_deref() {
        Ok("1") => true,
        Ok("0") => false,
        _ => fb.len() <= 64,
    };
    if !use_table {
        // the per-trial sweep's square roots and Semaev evaluations are not counted
        count("sweep_collector_work_not_counted", 1);
    }
    let sum_table = if use_table {
        Some(build_pair_sum_table(curve, &fb))
    } else {
        None
    };
    // Experimental: IC_HOIST_SWEEP=1 builds the sweep's x-index once per solve.
    let fast_sweep = std::env::var("IC_FAST_SWEEP").as_deref() == Ok("1");
    let hoisted_index = if fast_sweep || std::env::var("IC_HOIST_SWEEP").as_deref() == Ok("1") {
        Some(factor_base_x_index(&fb))
    } else {
        None
    };
    let word_sweep = if fast_sweep {
        WordSweep::new(curve, &fb)
    } else {
        None
    };
    // Experimental: IC_WALK=1 (with IC_SUM_TABLE=1) samples along an r-adding walk.
    let mut walk = if sum_table.is_some() && std::env::var("IC_WALK").as_deref() == Ok("1") {
        Some(SampleWalk::new(curve, g, q, rng))
    } else {
        None
    };
    let mut relations = Vec::with_capacity(target);
    let mut attempts_exhausted = 0usize;
    let mut trials_per_relation: Vec<usize> = Vec::with_capacity(target);
    let mut trials_total = 0usize;
    let mut retries = 0usize;
    let mut relations_ms;
    let (m, solution, linear_algebra_ms) = loop {
        while relations.len() < target {
            let mut failed_trials = 0usize;
            let found = 'attempt: {
                for _ in 0..max_relation_attempts {
                    let attempt = match &sum_table {
                        Some(t) if walk.is_some() => find_one_relation_table_walk(
                            curve,
                            g,
                            q,
                            t,
                            walk.as_mut().expect("walk checked"),
                            max_trials_per_relation,
                            rng,
                        ),
                        Some(t) => find_one_relation_table_with_rng(
                            curve,
                            g,
                            q,
                            t,
                            max_trials_per_relation,
                            rng,
                        ),
                        None => match &hoisted_index {
                            Some(ix) => find_one_relation_sweep_with_index(
                                curve,
                                g,
                                q,
                                &fb,
                                ix,
                                word_sweep.as_ref(),
                                max_trials_per_relation,
                                rng,
                            ),
                            None => find_one_relation_counted_with_rng(
                                curve,
                                g,
                                q,
                                &fb,
                                max_trials_per_relation,
                                rng,
                            ),
                        },
                    };
                    if let Some(found) = attempt {
                        break 'attempt Some(found);
                    }
                    attempts_exhausted += 1;
                    failed_trials += max_trials_per_relation;
                }
                None
            };
            let (rel, trials) = found?;
            trials_total += failed_trials + trials;
            trials_per_relation.push(failed_trials + trials);
            relations.push(rel);
        }
        relations_ms = relations_started.elapsed().as_secs_f64() * 1e3;

        // Build the linear system.  Unknowns: (y_1, …, y_m, x = log_G Q).
        // Row r:   Σ entries[j].value y_j  − coef_b · x  ≡ coef_a (mod n)
        // i.e.    entries  ‖ −coef_b   ·   (y, x)  =  coef_a.
        let m = fb.len();
        let mut matrix: Vec<Vec<BigUint>> = Vec::with_capacity(relations.len());
        let mut rhs: Vec<BigUint> = Vec::with_capacity(relations.len());
        for rel in &relations {
            let mut row = vec![BigUint::zero(); m + 1];
            for &(j, mult) in &rel.entries {
                let val = signed_mod(mult, &curve.n);
                row[j] = (&row[j] + &val) % &curve.n;
            }
            // last column: −coef_b
            let neg_b = (&curve.n - &(&rel.coef_b % &curve.n)) % &curve.n;
            row[m] = neg_b;
            matrix.push(row);
            rhs.push(rel.coef_a.clone() % &curve.n);
        }

        let la_started = Instant::now();
        // The elimination charges its modular operations to the `bitop/v1` tally; the
        // difference across the call is what this solve spent (summed over every retry).
        let tally_before = bitop_ops::snapshot();
        let eliminated = gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, &curve.n);
        let spent = bitop_ops::snapshot().since(&tally_before).total();
        count(
            "elimination_modular_multiplications_uncharged",
            spent.get(bitop_ops::Kind::ModrMul),
        );
        count(
            "elimination_modular_additions_uncharged",
            spent.get(bitop_ops::Kind::ModrAdd),
        );
        count(
            "elimination_modular_inversions_uncharged",
            spent.get(bitop_ops::Kind::ModrInv),
        );
        count(
            "elimination_word_operations_uncharged",
            spent.get(bitop_ops::Kind::WordOp),
        );
        let solution = eliminated?;
        if solution.determined[m] {
            break (m, solution, la_started.elapsed().as_secs_f64() * 1e3);
        }
        // Rank-deficient (the target is not pinned down): collect two more
        // relations and eliminate again, so a small `extra_relations` is safe.
        retries += 1;
        if retries > 8 {
            return None;
        }
        target += 2;
    };
    let x = solution.values[m].clone();

    let verify_started = Instant::now();
    let a_fe = curve.a_fe();
    let (va, vd) = scalar_mult_ops(&x);
    count("group_additions", va);
    count("group_doublings", vd);
    let candidate = g.scalar_mul(&x, &a_fe);
    let verified = &candidate == q;
    let verify_ms = verify_started.elapsed().as_secs_f64() * 1e3;
    if !verified {
        return None;
    }

    let mut sorted_trials = trials_per_relation.clone();
    sorted_trials.sort_unstable();
    let median = if sorted_trials.is_empty() {
        0.0
    } else if sorted_trials.len() % 2 == 1 {
        sorted_trials[sorted_trials.len() / 2] as f64
    } else {
        (sorted_trials[sorted_trials.len() / 2 - 1] + sorted_trials[sorted_trials.len() / 2]) as f64
            / 2.0
    };
    let report = EcIcStageReport {
        factor_base_ms,
        relations_ms,
        linear_algebra_ms,
        verify_ms,
        total_ms: total_started.elapsed().as_secs_f64() * 1e3,
        factor_base_size: m,
        relations_collected: relations.len(),
        relation_attempts_exhausted: attempts_exhausted,
        trials_total,
        trials_per_relation_min: sorted_trials.first().copied().unwrap_or(0),
        trials_per_relation_median: median,
        trials_per_relation_max: sorted_trials.last().copied().unwrap_or(0),
        counters: take_counters(),
        counters_unemitted: Vec::new(),
    };
    Some((x, report))
}

/// Staged S₄ 3-decomposition twin of [`ec_index_calculus_dlp_staged`]:
/// identical report shape, relations come from
/// [`find_one_relation_s4_counted`] (three factor-base points per row).
pub fn ec_index_calculus_dlp_s4_staged(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
) -> Option<(BigUint, EcIcStageReport)> {
    use std::time::Instant;
    let total_started = Instant::now();

    let fb_started = Instant::now();
    let fb = build_factor_base(curve, fb_size);
    let factor_base_ms = fb_started.elapsed().as_secs_f64() * 1e3;
    if fb.is_empty() {
        return None;
    }
    let target = fb.len() + extra_relations;

    let relations_started = Instant::now();
    let mut relations = Vec::with_capacity(target);
    let mut attempts_exhausted = 0usize;
    let mut trials_per_relation: Vec<usize> = Vec::with_capacity(target);
    let mut trials_total = 0usize;
    while relations.len() < target {
        let mut failed_trials = 0usize;
        let found = 'attempt: {
            for _ in 0..max_relation_attempts {
                if let Some(found) =
                    find_one_relation_s4_counted(curve, g, q, &fb, max_trials_per_relation)
                {
                    break 'attempt Some(found);
                }
                attempts_exhausted += 1;
                failed_trials += max_trials_per_relation;
            }
            None
        };
        let (rel, trials) = found?;
        trials_total += failed_trials + trials;
        trials_per_relation.push(failed_trials + trials);
        relations.push(rel);
    }
    let relations_ms = relations_started.elapsed().as_secs_f64() * 1e3;

    let m = fb.len();
    let mut matrix: Vec<Vec<BigUint>> = Vec::with_capacity(relations.len());
    let mut rhs: Vec<BigUint> = Vec::with_capacity(relations.len());
    for rel in &relations {
        let mut row = vec![BigUint::zero(); m + 1];
        for &(j, mult) in &rel.entries {
            let val = signed_mod(mult, &curve.n);
            row[j] = (&row[j] + &val) % &curve.n;
        }
        let neg_b = (&curve.n - &(&rel.coef_b % &curve.n)) % &curve.n;
        row[m] = neg_b;
        matrix.push(row);
        rhs.push(rel.coef_a.clone() % &curve.n);
    }

    let la_started = Instant::now();
    let solution = gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, &curve.n)?;
    if !solution.determined[m] {
        return None;
    }
    let linear_algebra_ms = la_started.elapsed().as_secs_f64() * 1e3;
    let x = solution.values[m].clone();

    let verify_started = Instant::now();
    let a_fe = curve.a_fe();
    let candidate = g.scalar_mul(&x, &a_fe);
    let verified = &candidate == q;
    let verify_ms = verify_started.elapsed().as_secs_f64() * 1e3;
    if !verified {
        return None;
    }

    let mut sorted_trials = trials_per_relation.clone();
    sorted_trials.sort_unstable();
    let median = if sorted_trials.is_empty() {
        0.0
    } else if sorted_trials.len() % 2 == 1 {
        sorted_trials[sorted_trials.len() / 2] as f64
    } else {
        (sorted_trials[sorted_trials.len() / 2 - 1] + sorted_trials[sorted_trials.len() / 2]) as f64
            / 2.0
    };
    let report = EcIcStageReport {
        factor_base_ms,
        relations_ms,
        linear_algebra_ms,
        verify_ms,
        total_ms: total_started.elapsed().as_secs_f64() * 1e3,
        factor_base_size: m,
        relations_collected: relations.len(),
        relation_attempts_exhausted: attempts_exhausted,
        trials_total,
        trials_per_relation_min: sorted_trials.first().copied().unwrap_or(0),
        trials_per_relation_median: median,
        trials_per_relation_max: sorted_trials.last().copied().unwrap_or(0),
        counters: Default::default(),
        counters_unemitted: Vec::new(),
    };
    Some((x, report))
}

// ── Baseline: textbook Pollard rho for ECDLP ───────────────────────────

/// **Pollard rho** for the elliptic-curve discrete log.  Walks a single
/// chain `R_{i+1} = f(R_i)` where `f` is a partition-based pseudo-random
/// step (Teske's r-adding walk with r = 3 branches: G, Q, or G+Q each
/// chosen by `x_R mod 3`).  Detects collision via Floyd's tortoise/hare.
///
/// Included here as a baseline against which the index-calculus driver
/// is compared.  Pollard rho on a prime-field curve of `n`-bit order
/// runs in expected `≈ √(πn/2)` group operations; the IC driver above
/// is asymptotically *worse* in this 2-decomposition regime
/// (`O(p · m) = O(p^{3/2})`).  Watching the two timings diverge on
/// progressively larger curves is the headline "good news for prime
/// curve security" of this module.
pub fn pollard_rho_ecdlp(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    max_steps: usize,
) -> Option<BigUint> {
    let a_fe = curve.a_fe();
    // Three precomputed step points: G, Q, G+Q.
    let g_plus_q = g.add(q, &a_fe);

    let step = |r: &Point, alpha: &BigUint, beta: &BigUint| -> (Point, BigUint, BigUint) {
        let branch = match r {
            Point::Affine { x, .. } => (&x.value % BigUint::from(3u32))
                .to_u32_digits()
                .first()
                .copied()
                .unwrap_or(0),
            Point::Infinity => 0,
        };
        match branch {
            0 => (
                r.add(g, &a_fe),
                (alpha + BigUint::one()) % &curve.n,
                beta.clone(),
            ),
            1 => (
                r.add(&g_plus_q, &a_fe),
                (alpha + BigUint::one()) % &curve.n,
                (beta + BigUint::one()) % &curve.n,
            ),
            _ => (
                r.add(q, &a_fe),
                alpha.clone(),
                (beta + BigUint::one()) % &curve.n,
            ),
        }
    };

    // Two walkers (Floyd).  Start from R₀ = G + Q so we have a known
    // (α, β) decomposition right away.
    let mut t_r = g.add(q, &a_fe);
    let mut t_a = BigUint::one();
    let mut t_b = BigUint::one();
    let mut h_r = t_r.clone();
    let mut h_a = t_a.clone();
    let mut h_b = t_b.clone();

    for _ in 0..max_steps {
        // Tortoise: one step.
        let (rn, an, bn) = step(&t_r, &t_a, &t_b);
        t_r = rn;
        t_a = an;
        t_b = bn;
        // Hare: two steps.
        let (rn, an, bn) = step(&h_r, &h_a, &h_b);
        let (rn, an, bn) = step(&rn, &an, &bn);
        h_r = rn;
        h_a = an;
        h_b = bn;

        if t_r == h_r {
            // Collision: α_t G + β_t Q  =  α_h G + β_h Q
            //  ⟹  (α_t − α_h) ≡ (β_h − β_t) · x  (mod n).
            let n = &curve.n;
            let lhs = (&t_a + n - &h_a) % n;
            let rhs = (&h_b + n - &t_b) % n;
            if rhs.is_zero() {
                return None;
            }
            let inv = mod_inverse(&rhs, n)?;
            return Some((lhs * inv) % n);
        }
    }
    None
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    /// Cantor–Zassenhaus root finder agrees with brute force on a prime
    /// past the brute-force cutoff (p = 1048568 + 3 has 21 bits) for a
    /// spread of quartics with 0-4 roots.
    #[test]
    fn fast_roots_match_brute_force_past_cutoff() {
        let p = BigUint::from(1_048_571u32);
        // Construct (X - r1)(X - r2)... products for various root sets.
        for roots in [
            vec![3u32],
            vec![3u32, 700_001],
            vec![1u32, 2, 3],
            vec![5u32, 90_000, 1_000_003, 77],
            vec![],
        ] {
            // poly = prod (X - r) as coefficients (index = power).
            let mut poly: Vec<BigUint> = vec![BigUint::from(1u32)]; // X^0
            for r in &roots {
                // multiply by (X - r) mod p: coefficient of X^k is
                // c_{k-1} - r·c_k (with c_{-1} = c_len = 0).
                let mut fixed = vec![BigUint::from(0u32); poly.len() + 1];
                for k in 0..=poly.len() {
                    let prev = if k > 0 {
                        poly[k - 1].clone()
                    } else {
                        BigUint::from(0u32)
                    };
                    let cur = poly.get(k).cloned().unwrap_or(BigUint::from(0u32));
                    let term = (BigUint::from(*r) * cur) % &p;
                    let neg = if term.is_zero() {
                        BigUint::from(0u32)
                    } else {
                        &p - &term
                    };
                    fixed[k] = (prev + neg) % &p;
                }
                poly = fixed;
            }
            let coeffs: Vec<FieldElement> = poly
                .iter()
                .map(|c| FieldElement::new(c % &p, p.clone()))
                .collect();
            let found = find_roots_fp_fast(&coeffs, &p);
            let mut found_sorted: Vec<BigUint> = found.iter().map(|f| f.value.clone()).collect();
            found_sorted.sort();
            let mut expect: Vec<BigUint> = roots.iter().map(|r| BigUint::from(*r)).collect();
            expect.sort();
            expect.dedup();
            assert_eq!(found_sorted, expect, "roots {roots:?}");
        }
    }

    /// `Point::double_scalar_mul_vartime` equals two ladders plus an add,
    /// including `Q = -G` and zero scalars.
    #[test]
    fn point_double_scalar_mul_vartime_matches_two_ladders() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
        let neg_g = g.scalar_mul(&(&curve.n - BigUint::one()), &a_fe);
        for other in [&q, &neg_g, &g] {
            for a in 0u32..60 {
                for b in 0u32..60 {
                    let (a, b) = (BigUint::from(a * 5), BigUint::from(b * 13 + (a % 3)));
                    let want = g
                        .scalar_mul(&a, &a_fe)
                        .add(&other.scalar_mul(&b, &a_fe), &a_fe);
                    assert_eq!(g.double_scalar_mul_vartime(&a, other, &b, &a_fe), want);
                }
            }
        }
    }

    /// With no surplus relations the system is sometimes rank-deficient
    /// (about one run in ten on this fixture before the retry); the staged
    /// driver now collects more relations and still returns the log.
    #[test]
    fn staged_retries_a_rank_deficient_system() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let x_truth = BigUint::from(190u32);
        let q = g.scalar_mul(&x_truth, &a_fe);
        let mut retried = 0;
        for seed in 0..120u64 {
            let (x, report) =
                ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, 8, 0, 5000, 64, seed)
                    .expect("the retry recovers a rank-deficient system");
            assert_eq!(g.scalar_mul(&x, &a_fe), q);
            if report.relations_collected > report.factor_base_size {
                retried += 1;
            }
        }
        assert!(retried > 0, "no seed exercised the retry path");
    }

    /// Above one word, `double_scalar_mul_vartime` equals two ladders plus an
    /// add on secp256k1 and P-256, including `Q = -G` and a zero scalar.
    #[test]
    fn point_double_scalar_mul_vartime_matches_two_ladders_multiword() {
        for curve in [CurveParams::secp256k1(), CurveParams::p256()] {
            let a_fe = curve.a_fe();
            let g = curve.generator();
            let q = g.scalar_mul(&BigUint::from(0x1234_5678_9abc_def0_u64), &a_fe);
            let neg_g = g.scalar_mul(&(&curve.n - BigUint::one()), &a_fe);
            let big = |seed: u64| {
                (BigUint::from(seed) << 200u32)
                    + BigUint::from(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15))
            };
            for other in [&q, &neg_g] {
                for (i, j) in [(1u64, 2u64), (3, 0), (0, 5), (7, 7), (11, 13), (255, 4097)] {
                    let (k, l) = (big(i), big(j));
                    let want = g
                        .scalar_mul(&k, &a_fe)
                        .add(&other.scalar_mul(&l, &a_fe), &a_fe);
                    assert_eq!(g.double_scalar_mul_vartime(&k, other, &l, &a_fe), want);
                }
            }
        }
    }

    /// The ladder-operation derivation and the staged driver's counters agree with
    /// the trial counts they accompany.
    #[test]
    fn staged_counters_are_consistent_with_the_trials() {
        // 13 = 0b1101: bits 4, popcount 3 -> 2 additions, 3 doublings.
        assert_eq!(scalar_mult_ops(&BigUint::from(13u32)), (2, 3));
        assert_eq!(scalar_mult_ops(&BigUint::from(1u32)), (0, 0));
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
        let (_, report) =
            ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, 8, 2, 5000, 64, 7).expect("solves");
        let c = &report.counters;
        let get = |k: &str| c.get(k).copied().unwrap_or(0);
        // one table probe per trial that did not land on the identity; two scalars per trial
        assert!(get("table_probes_uncharged") <= report.trials_total as u64);
        assert!(get("table_probes_uncharged") + 2 >= report.trials_total as u64);
        assert_eq!(
            get("random_scalars_drawn_uncharged"),
            2 * report.trials_total as u64
        );
        // the pair table of fb = 8: 36 unordered pairs, 28 off the diagonal
        assert_eq!(get("table_insert_attempts_uncharged"), 2 * 36 + 2 * 28);
        assert!(get("group_additions") > 0 && get("group_doublings") > 0);
        assert_eq!(get("relation_points_not_counted"), 0);
        // the elimination's modular operations are emitted now, and are not zero
        assert!(report.counters_unemitted.is_empty());
        assert!(get("elimination_modular_multiplications_uncharged") > 0);
        assert!(get("elimination_modular_inversions_uncharged") > 0);
        assert!(get("elimination_word_operations_uncharged") > 0);
    }

    /// Shamir interleaving equals two separate ladders plus an add.
    #[test]
    fn double_scalar_mul_matches_two_ladders() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
        for a in 0u32..40 {
            for b in 0u32..40 {
                let (a, b) = (BigUint::from(a * 7 + 1), BigUint::from(b * 11 + 3));
                let want = g.scalar_mul(&a, &a_fe).add(&q.scalar_mul(&b, &a_fe), &a_fe);
                assert_eq!(double_scalar_mul_vartime(&g, &a, &q, &b, &a_fe), want);
            }
        }
        // Q = -G makes G + Q the identity; the pass must still be exact.
        let neg_g = g.scalar_mul(&(&curve.n - BigUint::one()), &a_fe);
        for (a, b) in [(5u32, 3u32), (9, 9), (1, 0), (0, 4)] {
            let (a, b) = (BigUint::from(a), BigUint::from(b));
            let want = g
                .scalar_mul(&a, &a_fe)
                .add(&neg_g.scalar_mul(&b, &a_fe), &a_fe);
            assert_eq!(double_scalar_mul_vartime(&g, &a, &neg_g, &b, &a_fe), want);
        }
    }

    /// The S₄ 3-decomposition relation search recovers the same DLP as the
    /// 2-decomposition driver on the tiny curve, with three-entry rows.
    #[test]
    fn s4_relation_search_solves_and_uses_three_entries() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
        let fb = build_factor_base(&curve, 10);
        assert!(!fb.is_empty());
        let (x, _trials) = (0..64)
            .find_map(|_| find_one_relation_s4_counted(&curve, &g, &q, &fb, 5000))
            .expect("an S4 relation exists within 64 bounded attempts");
        let _ = x; // one relation suffices; full solve below.

        // Compare the relation formats with ample random-row redundancy;
        // a small raw-row surplus is not a matrix-rank guarantee.
        let extra_relations = 64;
        let staged = ec_index_calculus_dlp_s4_staged(&curve, &g, &q, 10, extra_relations, 5000, 64);
        let (s4_x, report) = staged.expect("S4 staged solve succeeds on the tiny curve");
        assert_eq!(g.scalar_mul(&s4_x, &a_fe), q);
        let plain = ec_index_calculus_dlp(&curve, &g, &q, 10, extra_relations, 5000);
        assert_eq!(s4_x, plain.expect("2-decomp solve also succeeds"));
        assert_eq!(report.relations_collected, 10 + extra_relations);
        assert!(report.trials_per_relation_median >= 1.0);
    }

    /// Staged twin recovers the same log as the plain driver on the tiny
    /// curve and reports split stage timers whose sum is consistent with
    /// the total (factor_base + relations + LA each separately positive).
    #[test]
    fn staged_dlp_recovers_and_splits_stages() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
        // Stage accounting needs a successful solve. Four extra random
        // relation rows can leave the matrix rank deficient; retain ample
        // redundancy in this public known-answer fixture.
        let extra_relations = 64;
        let plain = ec_index_calculus_dlp(&curve, &g, &q, 8, extra_relations, 5000);
        let staged = ec_index_calculus_dlp_staged(&curve, &g, &q, 8, extra_relations, 5000, 64);
        let (staged_x, report) = staged.expect("staged solve succeeds on the tiny curve");
        let plain_x = plain.expect("plain solve succeeds on the tiny curve");
        assert_eq!(staged_x, plain_x);
        assert_eq!(g.scalar_mul(&staged_x, &a_fe), q);
        assert_eq!(report.factor_base_size, 8);
        assert_eq!(report.relations_collected, 8 + extra_relations);
        assert!(report.factor_base_ms >= 0.0);
        assert!(report.relations_ms >= 0.0);
        assert!(report.linear_algebra_ms >= 0.0);
        assert!(report.verify_ms >= 0.0);
        assert!(report.total_ms >= report.relations_ms);
        assert!(report.trials_total >= report.relations_collected);
        assert!(report.trials_per_relation_median >= 1.0);
        assert!(
            report.trials_total
                >= report.relations_collected + report.relation_attempts_exhausted * 64
        );
    }

    /// **Small curve for tests**: y² = x³ + x + 19 (mod 271).  Curve
    /// has 281 points (prime), so cofactor 1 and `G = (3, 7)` is a
    /// generator of the full group.
    fn tiny_curve() -> CurveParams {
        CurveParams {
            name: "test-curve-271",
            p: BigUint::from(271u32),
            a: BigUint::from(1u32),
            b: BigUint::from(19u32),
            gx: BigUint::from(3u32),
            gy: BigUint::from(7u32),
            n: BigUint::from(281u32),
            h: 1,
        }
    }

    /// Sum of signed multiples of factor-base points, by the plain group law.
    fn fb_combination(
        curve: &CurveParams,
        fb: &[FactorBaseEntry],
        entries: &[(usize, i64)],
    ) -> Point {
        let a_fe = curve.a_fe();
        let mut acc = Point::Infinity;
        for &(idx, m) in entries {
            let base = if m > 0 {
                fb[idx].point.clone()
            } else {
                fb[idx].point.neg()
            };
            let term = base.scalar_mul(&BigUint::from(m.unsigned_abs()), &a_fe);
            acc = acc.add(&term, &a_fe);
        }
        acc
    }

    /// The pair-sum table (two variable-time additions per pair, negation
    /// free) holds exactly the keys of the plain four-addition build, and
    /// every entry is a true decomposition `s_i F_i + s_j F_j`.
    #[test]
    fn pair_sum_table_is_exactly_the_four_sign_combinations() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let fb = build_factor_base(&curve, 12);
        let table = build_pair_sum_table_exact(&curve, &fb);
        let mut reference = std::collections::HashSet::new();
        for i in 0..fb.len() {
            for j in i..fb.len() {
                for s_i in [1i64, -1] {
                    for s_j in [1i64, -1] {
                        let pi = if s_i > 0 {
                            fb[i].point.clone()
                        } else {
                            fb[i].point.neg()
                        };
                        let pj = if s_j > 0 {
                            fb[j].point.clone()
                        } else {
                            fb[j].point.neg()
                        };
                        if let Point::Affine { x, y } = pi.add(&pj, &a_fe) {
                            reference.insert((x.value.clone(), y.value.clone()));
                        }
                    }
                }
            }
        }
        let keys: std::collections::HashSet<_> = table.keys().cloned().collect();
        assert_eq!(
            keys, reference,
            "table keys differ from the four-addition build"
        );
        for ((x, y), &(i, s_i, j, s_j)) in &table {
            let pi = if s_i > 0 {
                fb[i].point.clone()
            } else {
                fb[i].point.neg()
            };
            let pj = if s_j > 0 {
                fb[j].point.clone()
            } else {
                fb[j].point.neg()
            };
            match pi.add(&pj, &a_fe) {
                Point::Affine { x: x2, y: y2 } => {
                    assert_eq!(
                        (&x2.value, &y2.value),
                        (x, y),
                        "entry is not its own decomposition"
                    )
                }
                Point::Infinity => panic!("an infinity sum was stored"),
            }
        }
    }

    /// The compact table answers exactly like the exact one: the same hit set over every point of
    /// the curve's subgroup, and each answer is a true decomposition of the queried point.
    #[test]
    fn compact_pair_sum_table_matches_the_exact_table() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = Point::Affine {
            x: FieldElement::new(curve.gx.clone(), curve.p.clone()),
            y: FieldElement::new(curve.gy.clone(), curve.p.clone()),
        };
        let fb = build_factor_base(&curve, 12);
        let exact = build_pair_sum_table_exact(&curve, &fb);
        let compact = build_pair_sum_table(&curve, &fb);
        assert!(compact.slot_bytes() < exact.len() * 170);
        let mut hits = 0usize;
        let mut k = BigUint::from(1u32);
        let mut cur = g.clone();
        while k < curve.n {
            if let Point::Affine { x, y } = &cur {
                let key = (x.value.clone(), y.value.clone());
                let e = exact.get(&key).copied();
                let c = compact.find(&key.0, &key.1);
                assert_eq!(e.is_some(), c.is_some(), "hit sets differ at k = {k}");
                if let Some((i, s_i, j, s_j)) = c {
                    hits += 1;
                    assert_eq!(
                        e,
                        Some((i, s_i, j, s_j)),
                        "different decomposition at k = {k}"
                    );
                    let pi = if s_i > 0 {
                        fb[i].point.clone()
                    } else {
                        fb[i].point.neg()
                    };
                    let pj = if s_j > 0 {
                        fb[j].point.clone()
                    } else {
                        fb[j].point.neg()
                    };
                    assert_eq!(pi.add(&pj, &a_fe), cur);
                }
            }
            cur = cur.add(&g, &a_fe);
            k += 1u32;
        }
        assert!(hits > 0, "no pair sum was hit");
        let packed = PairSumTable::pack(11, -1, 7, 1);
        assert_eq!(PairSumTable::unpack(packed), (11, -1, 7, 1));
        let top = PairSumTable::pack(PAIR_SUM_TABLE_MAX_FB - 1, -1, PAIR_SUM_TABLE_MAX_FB - 1, -1);
        assert_eq!(
            PairSumTable::unpack(top),
            (PAIR_SUM_TABLE_MAX_FB - 1, -1, PAIR_SUM_TABLE_MAX_FB - 1, -1)
        );
    }

    /// Every relation found by the table lookup and by the restarted walk
    /// satisfies `a G + b Q = sum of its signed factor-base terms`; the walk
    /// keeps `r = a G + b Q` through steps and restarts.
    #[test]
    fn table_and_walk_relations_are_true_identities() {
        use rand::SeedableRng;
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = Point::Affine {
            x: FieldElement::new(curve.gx.clone(), curve.p.clone()),
            y: FieldElement::new(curve.gy.clone(), curve.p.clone()),
        };
        let q = g.scalar_mul(&BigUint::from(57u32), &a_fe);
        let fb = build_factor_base(&curve, 12);
        let table = build_pair_sum_table(&curve, &fb);
        let check = |rel: &Relation| {
            let lhs = g
                .scalar_mul(&rel.coef_a, &a_fe)
                .add(&q.scalar_mul(&rel.coef_b, &a_fe), &a_fe);
            assert_eq!(lhs, fb_combination(&curve, &fb, &rel.entries));
        };
        let mut rng = rand::rngs::StdRng::seed_from_u64(7);
        for _ in 0..20 {
            let (rel, _) =
                find_one_relation_table_with_rng(&curve, &g, &q, &table, 100_000, &mut rng)
                    .expect("table relation");
            check(&rel);
        }
        let mut walk = SampleWalk::new(&curve, &g, &q, &mut rng);
        for _ in 0..200 {
            let (rel, _) =
                find_one_relation_table_walk(&curve, &g, &q, &table, &mut walk, 100_000, &mut rng)
                    .expect("walk relation");
            check(&rel);
            let on_curve = g
                .scalar_mul(&walk.a, &a_fe)
                .add(&q.scalar_mul(&walk.b, &a_fe), &a_fe);
            assert_eq!(walk.r, on_curve, "walk lost r = aG + bQ");
        }
    }

    /// The `u64` word sweep returns exactly the relations, and the same trial
    /// counts, as the `BigUint` sweep when both consume the same random stream.
    #[test]
    fn word_sweep_matches_the_biguint_sweep() {
        use rand::SeedableRng;
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = Point::Affine {
            x: FieldElement::new(curve.gx.clone(), curve.p.clone()),
            y: FieldElement::new(curve.gy.clone(), curve.p.clone()),
        };
        let q = g.scalar_mul(&BigUint::from(101u32), &a_fe);
        for fb_size in [6usize, 12] {
            let fb = build_factor_base(&curve, fb_size);
            let x_to_idx = factor_base_x_index(&fb);
            let word = WordSweep::new(&curve, &fb).expect("271 is a word prime");
            let mut rng_a = rand::rngs::StdRng::seed_from_u64(11 + fb_size as u64);
            let mut rng_b = rand::rngs::StdRng::seed_from_u64(11 + fb_size as u64);
            for _ in 0..40 {
                let slow = find_one_relation_sweep_with_index(
                    &curve, &g, &q, &fb, &x_to_idx, None, 100_000, &mut rng_a,
                )
                .expect("BigUint sweep relation");
                let fast = find_one_relation_sweep_with_index(
                    &curve,
                    &g,
                    &q,
                    &fb,
                    &x_to_idx,
                    Some(&word),
                    100_000,
                    &mut rng_b,
                )
                .expect("word sweep relation");
                assert_eq!(slow.1, fast.1, "trial counts differ");
                assert_eq!(slow.0.coef_a, fast.0.coef_a);
                assert_eq!(slow.0.coef_b, fast.0.coef_b);
                assert_eq!(slow.0.entries, fast.0.entries);
            }
        }
    }

    /// **Semaev S₃ is symmetric** in its three variables (verify on a
    /// handful of random samples).
    /// The word-sized elimination returns the same solution and leaves
    /// the same reduced system behind as the `BigUint` path, on full-rank,
    /// rank-deficient and overdetermined systems.
    #[test]
    fn u64_elimination_matches_biguint_path() {
        let mut s = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        for &n in &[7u64, 65_537, 1_000_000_007, 0xFFFF_FFFF_FFFF_FFC5] {
            for &(rows, cols, sparsity) in
                &[(6usize, 6usize, 1u64), (12, 8, 3), (8, 8, 2), (5, 9, 1)]
            {
                let nb = BigUint::from(n);
                let mut mat: Vec<Vec<BigUint>> = (0..rows)
                    .map(|_| {
                        (0..cols)
                            .map(|_| {
                                let v = next();
                                if v % sparsity == 0 {
                                    BigUint::from(v % n)
                                } else {
                                    BigUint::zero()
                                }
                            })
                            .collect()
                    })
                    .collect();
                // Make one row a combination of two others (rank drop).
                if rows > 3 {
                    let combo: Vec<BigUint> = (0..cols)
                        .map(|c| (&mat[0][c] + &mat[1][c] * 3u32) % &nb)
                        .collect();
                    mat[rows - 1] = combo;
                }
                let rhs: Vec<BigUint> = (0..rows).map(|_| BigUint::from(next() % n)).collect();
                let (mut m1, mut r1) = (mat.clone(), rhs.clone());
                let (mut m2, mut r2) = (mat.clone(), rhs.clone());
                let fast = gaussian_eliminate_mod_u64(&mut m1, &mut r1, n);
                let slow = gaussian_eliminate_mod_biguint(&mut m2, &mut r2, &nb);
                assert_eq!(fast, slow, "n={n} {rows}x{cols}");
                if slow.is_some() {
                    assert_eq!(m1, m2, "reduced matrix, n={n} {rows}x{cols}");
                    assert_eq!(r1, r2, "reduced rhs, n={n} {rows}x{cols}");
                }
            }
        }
        // Non-prime modulus with a non-invertible pivot: both refuse.
        let mut m = vec![vec![BigUint::from(2u32)]];
        let mut r = vec![BigUint::from(1u32)];
        assert!(gaussian_eliminate_mod_n(&mut m, &mut r, &BigUint::from(4u32)).is_none());
    }

    #[test]
    fn s3_symmetric() {
        let curve = tiny_curve();
        let a = curve.a_fe();
        let b = curve.fe(curve.b.clone());
        for (x1, x2, x3) in [(1u32, 2u32, 3u32), (5, 7, 11), (100, 150, 200)] {
            let f1 = curve.fe(BigUint::from(x1));
            let f2 = curve.fe(BigUint::from(x2));
            let f3 = curve.fe(BigUint::from(x3));
            let v123 = semaev_s3(&f1, &f2, &f3, &a, &b);
            let v132 = semaev_s3(&f1, &f3, &f2, &a, &b);
            let v213 = semaev_s3(&f2, &f1, &f3, &a, &b);
            let v231 = semaev_s3(&f2, &f3, &f1, &a, &b);
            let v312 = semaev_s3(&f3, &f1, &f2, &a, &b);
            let v321 = semaev_s3(&f3, &f2, &f1, &a, &b);
            assert_eq!(v123, v132);
            assert_eq!(v123, v213);
            assert_eq!(v123, v231);
            assert_eq!(v123, v312);
            assert_eq!(v123, v321);
        }
    }

    /// **Semaev S₃ vanishes on a triple of collinear points**.  If
    /// `P₁ + P₂ + P₃ = O` on `E`, then `S₃(x_{P₁}, x_{P₂}, x_{P₃}) = 0`.
    #[test]
    fn s3_vanishes_on_collinear() {
        let curve = tiny_curve();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let b_fe = curve.fe(curve.b.clone());

        // Pick P, Q on the curve, compute R = −(P + Q) = P+Q reflected.
        // Then P + Q + R = O, so S₃ should vanish on (x_P, x_Q, x_R).
        let p1 = g.scalar_mul(&BigUint::from(2u32), &a_fe);
        let p2 = g.scalar_mul(&BigUint::from(5u32), &a_fe);
        let p3 = p1.add(&p2, &a_fe).neg();

        let (x1, x2, x3) = (
            match &p1 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p2 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p3 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
        );
        let s = semaev_s3(&x1, &x2, &x3, &a_fe, &b_fe);
        assert!(s.is_zero(), "S₃ should vanish on collinear x-coords");
    }

    /// **Tonelli–Shanks** is invertible: `sqrt(n)² ≡ n (mod p)` for
    /// every quadratic residue `n`.
    #[test]
    fn tonelli_shanks_invertible() {
        // p = 17 ≡ 1 mod 4, exercises the full Tonelli–Shanks path.
        let p = BigUint::from(17u32);
        for n in 1u32..17 {
            let nn = BigUint::from(n);
            if let Some(r) = sqrt_mod_p(&nn, &p) {
                let sq = (&r * &r) % &p;
                assert_eq!(sq, nn, "n = {}", n);
            }
        }
        // p = 263 ≡ 3 mod 4, takes the shortcut path.
        let p = BigUint::from(263u32);
        for n in 1u32..263 {
            let nn = BigUint::from(n);
            if let Some(r) = sqrt_mod_p(&nn, &p) {
                let sq = (&r * &r) % &p;
                assert_eq!(sq, nn);
            }
        }
    }

    /// **Tonelli–Shanks rejects non-residues** properly.
    #[test]
    fn tonelli_shanks_rejects_nqr() {
        let p = BigUint::from(13u32);
        // QR mod 13 are {1, 3, 4, 9, 10, 12}; NQR are {2, 5, 6, 7, 8, 11}.
        for nqr in [2u32, 5, 6, 7, 8, 11] {
            assert!(sqrt_mod_p(&BigUint::from(nqr), &p).is_none(), "{}", nqr);
        }
    }

    /// **Factor base** contains points actually on the curve.
    #[test]
    fn factor_base_on_curve() {
        let curve = tiny_curve();
        let fb = build_factor_base(&curve, 20);
        assert!(!fb.is_empty());
        assert!(fb.len() <= 20);
        for entry in &fb {
            assert!(
                curve.is_on_curve(&entry.point),
                "{}: point off curve",
                entry.idx,
            );
        }
    }

    /// **Linear solver** recovers a known solution on a small system.
    #[test]
    fn gaussian_solves_small_system() {
        // 2x + 3y = 7  ;  x + y = 3   (mod 11)
        //   ⟹  x = 2, y = 1.
        let n = BigUint::from(11u32);
        let mut m = vec![
            vec![BigUint::from(2u32), BigUint::from(3u32)],
            vec![BigUint::from(1u32), BigUint::from(1u32)],
        ];
        let mut r = vec![BigUint::from(7u32), BigUint::from(3u32)];
        let sol = gaussian_eliminate_mod_n(&mut m, &mut r, &n).expect("solvable");
        assert_eq!(sol[0], BigUint::from(2u32));
        assert_eq!(sol[1], BigUint::from(1u32));
    }

    /// Moduli exercising both elimination paths: `101` takes the `u64`
    /// path, the Mersenne prime `2^89 − 1` the `BigUint` one.
    fn both_path_moduli() -> [BigUint; 2] {
        [
            BigUint::from(101u32),
            (BigUint::one() << 89usize) - BigUint::one(),
        ]
    }

    fn dense_system(
        n: &BigUint,
        rows: &[&[(usize, u64)]],
        rhs: &[u64],
        cols: usize,
    ) -> (Vec<Vec<BigUint>>, Vec<BigUint>) {
        let m = rows
            .iter()
            .map(|r| {
                let mut d = vec![BigUint::zero(); cols];
                for &(c, v) in r.iter() {
                    d[c] = BigUint::from(v) % n;
                }
                d
            })
            .collect();
        (m, rhs.iter().map(|&v| BigUint::from(v) % n).collect())
    }

    /// Under-determined system from the hyperelliptic sparse-solver
    /// tests: `x9` (and others) have no pivot, so the strict solver
    /// must refuse rather than return a particular solution with the
    /// free unknowns zeroed.
    #[test]
    fn gaussian_rejects_underdetermined_system() {
        for n in both_path_moduli() {
            let rows: [&[(usize, u64)]; 2] =
                [&[(0, 48), (1, 46), (4, 78), (9, 65)], &[(8, 16), (9, 26)]];
            let (mut m, mut r) = dense_system(&n, &rows, &[32, 28], 10);
            assert_eq!(
                gaussian_eliminate_mod_n(&mut m, &mut r, &n),
                None,
                "n = {n}"
            );

            let (mut m, mut r) = dense_system(&n, &rows, &[32, 28], 10);
            let p = gaussian_eliminate_mod_n_particular(&mut m, &mut r, &n).expect("consistent");
            assert_eq!(p.rank, 2);
            assert!(
                p.determined.iter().all(|&d| !d),
                "n = {n}: {:?}",
                p.determined
            );
        }
    }

    /// A pivoted column whose reduced row still touches a free column
    /// is not determined; one isolated from free columns is.
    #[test]
    fn gaussian_particular_flags_only_pinned_columns() {
        for n in both_path_moduli() {
            // x0 + x1 = 5 ; x2 = 7  over three unknowns.
            let rows: [&[(usize, u64)]; 2] = [&[(0, 1), (1, 1)], &[(2, 1)]];
            let (mut m, mut r) = dense_system(&n, &rows, &[5, 7], 3);
            let p = gaussian_eliminate_mod_n_particular(&mut m, &mut r, &n).expect("consistent");
            assert_eq!(p.determined, vec![false, false, true], "n = {n}");
            assert_eq!(p.values[2], BigUint::from(7u32));
        }
    }

    /// `x0 = 1` and `x0 = 2` together: inconsistent on both paths.
    #[test]
    fn gaussian_rejects_inconsistent_system() {
        for n in both_path_moduli() {
            let rows: [&[(usize, u64)]; 2] = [&[(0, 1)], &[(0, 1)]];
            let (mut m, mut r) = dense_system(&n, &rows, &[1, 2], 1);
            assert_eq!(gaussian_eliminate_mod_n(&mut m, &mut r, &n), None);
            let (mut m, mut r) = dense_system(&n, &rows, &[1, 2], 1);
            assert_eq!(
                gaussian_eliminate_mod_n_particular(&mut m, &mut r, &n),
                None
            );
        }
    }

    /// Square full-rank system with a planted solution still solves,
    /// identically on both paths; an extra dependent row is accepted.
    #[test]
    fn gaussian_solves_square_full_rank_system() {
        let planted = [3u64, 14, 15, 92, 65];
        let rows: [&[(usize, u64)]; 6] = [
            &[(0, 2), (1, 7), (4, 1)],
            &[(1, 5), (2, 9)],
            &[(0, 1), (2, 4), (3, 3)],
            &[(3, 8), (4, 6)],
            &[(0, 11), (4, 13)],
            // Sum of the first two rows: consistent and dependent.
            &[(0, 2), (1, 12), (2, 9), (4, 1)],
        ];
        for n in both_path_moduli() {
            let rhs: Vec<u64> = rows
                .iter()
                .map(|r| {
                    let v = r.iter().fold(BigUint::zero(), |acc, &(c, a)| {
                        acc + BigUint::from(a) * BigUint::from(planted[c])
                    });
                    (v % &n).to_u64_digits().first().copied().unwrap_or(0)
                })
                .collect();
            for take in [5, 6] {
                let (mut m, mut r) = dense_system(&n, &rows[..take], &rhs[..take], 5);
                let sol = gaussian_eliminate_mod_n(&mut m, &mut r, &n).expect("full rank");
                let want: Vec<BigUint> = planted.iter().map(|&v| BigUint::from(v)).collect();
                assert_eq!(sol, want, "n = {n}, rows = {take}");
            }
        }
    }

    /// **End-to-end ECDLP** on a small curve: solve `Q = x·G` via the
    /// index-calculus driver, verify the recovered `x` against direct
    /// search.
    #[test]
    fn ec_index_calculus_small_dlp() {
        let curve = tiny_curve();
        let g = curve.generator();
        let a_fe = curve.a_fe();

        // Plant x = 47 (smallish, well within the curve order 281).
        let x_truth = BigUint::from(47u32);
        let q = g.scalar_mul(&x_truth, &a_fe);

        // Drive IC.  Tiny factor base + generous trial budget.
        let recovered = ec_index_calculus_dlp(&curve, &g, &q, 12, 4, 5_000);
        let recovered = recovered.expect("IC should recover x on this small curve");
        let recheck = g.scalar_mul(&recovered, &a_fe);
        assert_eq!(recheck, q);
        assert!(recovered < curve.n);
    }

    /// **S₄ closed form sanity check**: evaluate `S_4` at four x-coords
    /// of points that sum to the identity.  Should be zero.
    #[test]
    fn s4_vanishes_on_four_collinear() {
        let curve = tiny_curve();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let b_fe = curve.fe(curve.b.clone());

        // P₁ + P₂ + P₃ + P₄ = O.  Construct: P₁, P₂, P₃ arbitrary, then
        // P₄ = −(P₁ + P₂ + P₃).
        let p1 = g.scalar_mul(&BigUint::from(2u32), &a_fe);
        let p2 = g.scalar_mul(&BigUint::from(7u32), &a_fe);
        let p3 = g.scalar_mul(&BigUint::from(13u32), &a_fe);
        let p4 = p1.add(&p2, &a_fe).add(&p3, &a_fe).neg();
        let (x1, x2, x3, x4) = (
            match &p1 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p2 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p3 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p4 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
        );
        // Evaluate S_4(x₁, x₂, x₃, X₄) at X₄ = x₄.
        let coeffs = semaev_s4_in_x4(&x1, &x2, &x3, &a_fe, &b_fe);
        let mut value = FieldElement::zero(curve.p.clone());
        let mut x4_pow = FieldElement::one(curve.p.clone());
        for c in coeffs.iter() {
            value = value.add(&c.mul(&x4_pow));
            x4_pow = x4_pow.mul(&x4);
        }
        assert!(
            value.is_zero(),
            "S₄ should vanish on collinear-sum x-coords"
        );
    }

    /// **Root finder sanity**: build a polynomial with known roots
    /// and recover them.
    #[test]
    fn find_roots_fp_round_trip() {
        let p = BigUint::from(271u32);
        let r1 = FieldElement::new(BigUint::from(3u32), p.clone());
        let r2 = FieldElement::new(BigUint::from(11u32), p.clone());
        let r3 = FieldElement::new(BigUint::from(200u32), p.clone());
        // (X − r1)(X − r2)(X − r3) over F_271 (signs absorbed via .neg()).
        let mr1 = r1.neg();
        let mr2 = r2.neg();
        let mr3 = r3.neg();
        // Expand step by step.
        //   (X + mr1)(X + mr2) = X² + (mr1 + mr2)X + mr1·mr2
        let c0_a = mr1.mul(&mr2);
        let c1_a = mr1.add(&mr2);
        // Multiply that by (X + mr3):
        //   X³ + (mr3 + c1_a) X² + (mr3·c1_a + c0_a) X + mr3·c0_a
        let c0 = mr3.mul(&c0_a);
        let c1 = mr3.mul(&c1_a).add(&c0_a);
        let c2 = mr3.add(&c1_a);
        let c3 = FieldElement::one(p.clone());
        let coeffs = vec![c0, c1, c2, c3];
        let mut roots = find_roots_fp(&coeffs, &p);
        roots.sort_by_key(|r| r.value.clone());
        let mut expected = vec![r1, r2, r3];
        expected.sort_by_key(|r| r.value.clone());
        assert_eq!(roots, expected);
    }

    /// **S_4 + root finder, end-to-end 3-decomposition oracle**.
    ///
    /// Given three factor-base x-coords `(x₁, x₂, x₃)` and the curve's
    /// `(a, b)`, compute `S_4(x₁, x₂, x₃, X₄)` as a quartic and read off
    /// every candidate `x₄ ∈ F_p` whose curve point closes the relation
    /// `±P₁ ± P₂ ± P₃ ± P₄ = O`. This is the binary-search step that
    /// turns the (currently unused) `semaev_s4_in_x4` infrastructure
    /// into something a relation collector can actually call.
    #[test]
    fn s4_root_finder_recovers_x4() {
        let curve = tiny_curve();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let b_fe = curve.fe(curve.b.clone());

        let p1 = g.scalar_mul(&BigUint::from(2u32), &a_fe);
        let p2 = g.scalar_mul(&BigUint::from(7u32), &a_fe);
        let p3 = g.scalar_mul(&BigUint::from(13u32), &a_fe);
        let p4 = p1.add(&p2, &a_fe).add(&p3, &a_fe).neg();
        let (x1, x2, x3, x4) = (
            match &p1 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p2 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p3 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
            match &p4 {
                Point::Affine { x, .. } => x.clone(),
                _ => panic!(),
            },
        );
        let coeffs = semaev_s4_in_x4(&x1, &x2, &x3, &a_fe, &b_fe);
        let coeffs_vec: Vec<FieldElement> = coeffs.to_vec();
        let roots = find_roots_fp(&coeffs_vec, &curve.p);
        assert!(
            roots.contains(&x4),
            "the true x₄ = {:?} should be among S₄-roots {:?}",
            x4.value,
            roots.iter().map(|r| r.value.clone()).collect::<Vec<_>>()
        );
        // Cross-check: every root r satisfies S_4(x_1, x_2, x_3, r) = 0,
        // i.e. evaluating the quartic at r gives zero.
        for r in &roots {
            let mut val = coeffs[4].clone();
            for i in (0..4).rev() {
                val = val.mul(r).add(&coeffs[i]);
            }
            assert!(val.is_zero());
        }
    }

    /// **Pollard rho** as a sanity baseline: recover the same `x` and
    /// confirm both algorithms agree.
    #[test]
    fn pollard_rho_recovers_small_dlp() {
        let curve = tiny_curve();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let x_truth = BigUint::from(47u32);
        let q = g.scalar_mul(&x_truth, &a_fe);
        let recovered =
            pollard_rho_ecdlp(&curve, &g, &q, 200_000).expect("rho should recover small DLP");
        let recheck = g.scalar_mul(&recovered, &a_fe);
        assert_eq!(recheck, q);
    }

    /// **IC vs rho on a 10-bit curve**: both must find the same `x`,
    /// even when we don't tell them what it is.  This is the most
    /// substantive test in the module — we run *real* end-to-end
    /// ECDLP via two completely different algorithms and check they
    /// agree.
    ///
    /// Empirically on the 10-bit curve (`|E| = 281`), the index-calculus
    /// driver takes a few thousand trials per relation × ~16 relations,
    /// while rho terminates in ≈ √(πn/2) ≈ 21 group ops.  The asymmetry
    /// is exactly what the published research predicts: 2-decomposition
    /// IC is strictly worse than rho on prime-field curves.
    #[test]
    fn ic_and_rho_agree_on_random_dlp() {
        use crate::utils::random::random_scalar;
        let curve = tiny_curve();
        let g = curve.generator();
        let a_fe = curve.a_fe();

        let x_truth = random_scalar(&curve.n);
        let q = g.scalar_mul(&x_truth, &a_fe);

        let ic = ec_index_calculus_dlp(&curve, &g, &q, 14, 6, 10_000);
        let rho = pollard_rho_ecdlp(&curve, &g, &q, 500_000);
        let ic = ic.expect("IC should recover the DLP on this small curve");
        let rho = rho.expect("rho should recover the DLP on this small curve");

        // Both [ic]G and [rho]G must equal Q, regardless of whether
        // they're the same scalar (mod n).  This is the right
        // assertion because ECDLP solutions are unique mod n.
        assert_eq!(g.scalar_mul(&ic, &a_fe), q);
        assert_eq!(g.scalar_mul(&rho, &a_fe), q);
        // And they should in fact match each other mod n.
        assert_eq!(ic % &curve.n, rho % &curve.n);
    }
}
