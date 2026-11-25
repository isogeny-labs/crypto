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
use std::sync::atomic::{AtomicBool, Ordering};

/// Public Semaev / IC walks: default vartime point ops.
/// `IC_IC_CT=1` restores CT for A/B.
static IC_CT: AtomicBool = AtomicBool::new(false);

pub(crate) fn refresh_ic_ct_flag() {
    IC_CT.store(
        std::env::var("IC_IC_CT").as_deref() == Ok("1"),
        Ordering::Relaxed,
    );
}

#[inline(always)]
pub(crate) fn ic_padd(p: &Point, q: &Point, a: &FieldElement) -> Point {
    if IC_CT.load(Ordering::Relaxed) {
        p.add(q, a)
    } else {
        p.add_vartime(q, a)
    }
}

#[inline(always)]
pub(crate) fn ic_smul(p: &Point, k: &BigUint, a: &FieldElement) -> Point {
    if IC_CT.load(Ordering::Relaxed) {
        p.scalar_mul(k, a)
    } else {
        p.scalar_mul_vartime(k, a)
    }
}

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

/// Stack-resident degree-≤4 polynomial over a single-word prime (`p < 2^63`).
///
/// Round-4 audit: the generic [`semaev_s4_in_x4`] path allocates dozens of
/// `FieldElement`/`BigUint` values per factor-base pair.  `Poly4Fp` keeps the
/// same resultant algebra in five `u64` limbs so S₄ sieving stays in registers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poly4Fp {
    /// Coefficients `c[0] + c[1] X + … + c[4] X⁴` reduced mod `p`.
    pub c: [u64; 5],
    pub p: u64,
}

impl Poly4Fp {
    /// Horner evaluation at `x`.
    #[inline]
    pub fn eval(&self, x: u64) -> u64 {
        let p = self.p as u128;
        let mut acc = 0u128;
        for &ci in self.c.iter().rev() {
            acc = (acc * (x as u128) + ci as u128) % p;
        }
        acc as u64
    }

    /// Lift to the heap [`FieldElement`] coefficient form used by older callers.
    pub fn to_field_elements(&self) -> [FieldElement; 5] {
        let p = BigUint::from(self.p);
        std::array::from_fn(|i| FieldElement::new(BigUint::from(self.c[i]), p.clone()))
    }

    /// `F_p`-rational roots of this quartic, without heap `FieldElement`s.
    ///
    /// For `p ≤ 2^20` brute-forces via [`Self::eval`] (S₄ IC benches).
    /// Larger odd word primes reuse the stack Cantor–Zassenhaus path
    /// through a thin FieldElement shim (same roots as [`find_roots_fp_fast`]).
    pub fn roots(&self) -> Vec<u64> {
        let p = self.p;
        if p <= (1u64 << 20) {
            let mut out = Vec::new();
            for x in 0..p {
                if self.eval(x) == 0 {
                    out.push(x);
                }
            }
            return out;
        }
        let fe = self.to_field_elements();
        find_roots_fp_fast(&fe, &BigUint::from(p))
            .into_iter()
            .map(|r| r.value.to_u64_digits().first().copied().unwrap_or(0))
            .collect()
    }
}

/// Stack-resident Semaev `S₄(X₁,X₂,X₃,X₄)` as a quartic in `X₄` for `p < 2^63`.
///
/// Bit-identical to [`semaev_s4_in_x4`] on single-word primes (see unit test).
pub fn semaev_s4_poly4fp(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> Poly4Fp {
    debug_assert!(p > 3 && (p & 1) == 1 && (p >> 63) == 0);
    #[inline(always)]
    fn add(x: u64, y: u64, p: u64) -> u64 {
        // Safe: callers keep `x, y < p < 2^63`, so `x + y` fits in `u64`.
        let s = x + y;
        if s >= p {
            s - p
        } else {
            s
        }
    }
    #[inline(always)]
    fn sub(x: u64, y: u64, p: u64) -> u64 {
        if x >= y {
            x - y
        } else {
            x.wrapping_add(p).wrapping_sub(y)
        }
    }
    #[inline(always)]
    fn mul(x: u64, y: u64, p: u64) -> u64 {
        ((x as u128 * y as u128) % (p as u128)) as u64
    }
    #[inline(always)]
    fn neg(x: u64, p: u64) -> u64 {
        if x == 0 {
            0
        } else {
            p - x
        }
    }
    #[inline(always)]
    fn sm(k: u64, x: u64, p: u64) -> u64 {
        mul(k % p, x, p)
    }

    // S₃(X₁, X₂, X) = α₂ X² + α₁ X + α₀ — same closed form as [`semaev_s3_in_x3`].
    let s = add(x1, x2, p);
    let pr = mul(x1, x2, p);
    let dx12 = sub(x1, x2, p);
    let alpha2 = mul(dx12, dx12, p);
    let alpha1 = neg(sm(2, add(mul(s, add(pr, a, p), p), sm(2, b, p), p), p), p);
    let alpha0 = sub(
        mul(sub(pr, a, p), sub(pr, a, p), p),
        sm(4, mul(b, s, p), p),
        p,
    );

    // β₂, β₁, β₀ as degree-≤2 polys in X₄ (same as FieldElement path).
    let x3sq = mul(x3, x3, p);
    let b2 = [x3sq, neg(sm(2, x3, p), p), 1u64 % p];
    let b1 = [
        neg(sm(2, add(mul(a, x3, p), sm(2, b, p), p), p), p),
        neg(sm(2, add(x3sq, a, p), p), p),
        neg(sm(2, x3, p), p),
    ];
    let b0 = [
        sub(mul(a, a, p), sm(4, mul(b, x3, p), p), p),
        neg(add(sm(2, mul(a, x3, p), p), sm(4, b, p), p), p),
        x3sq,
    ];

    let scale = |c: u64, poly: [u64; 3]| -> [u64; 3] {
        [mul(c, poly[0], p), mul(c, poly[1], p), mul(c, poly[2], p)]
    };
    let sub3 = |u: [u64; 3], v: [u64; 3]| -> [u64; 3] {
        [sub(u[0], v[0], p), sub(u[1], v[1], p), sub(u[2], v[2], p)]
    };
    let a_poly = sub3(scale(alpha2, b0), scale(alpha0, b2));
    let b_poly = sub3(scale(alpha2, b1), scale(alpha1, b2));
    let c_poly = sub3(scale(alpha1, b0), scale(alpha0, b1));

    // S₄ = A² − B·C, degree ≤ 4.
    let mut a_sq = [0u64; 5];
    let mut bc = [0u64; 5];
    for i in 0..3 {
        for j in 0..3 {
            a_sq[i + j] = add(a_sq[i + j], mul(a_poly[i], a_poly[j], p), p);
            bc[i + j] = add(bc[i + j], mul(b_poly[i], c_poly[j], p), p);
        }
    }
    let c = [
        sub(a_sq[0], bc[0], p),
        sub(a_sq[1], bc[1], p),
        sub(a_sq[2], bc[2], p),
        sub(a_sq[3], bc[3], p),
        sub(a_sq[4], bc[4], p),
    ];
    Poly4Fp { c, p }
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
    // Stack-resident u64 Shoup Cantor-Zassenhaus fast path for p < 2^63 and deg <= 8
    // (eliminates all BigUint / Vec<FieldElement> heap allocations in S3/S4 IC sieving).
    if p.bits() <= 63 && coeffs.len() <= 9 {
        if let Some(roots_u64) = find_roots_fp_u64_stack(coeffs, p) {
            return roots_u64
                .into_iter()
                .map(|r| FieldElement::new(BigUint::from(r), p.clone()))
                .collect();
        }
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
    let lead_inv = f[deg]
        .inv_vartime()
        .expect("nonzero lead is invertible mod p");
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
                if let Some(inv) = qb.inv_vartime() {
                    factors.push(vec![qc.neg().mul(&inv), qb]);
                }
            } else {
                let four = FieldElement::new(BigUint::from(4u32), p.clone());
                let disc = qb.mul(&qb).sub(&four.mul(&qa).mul(&qc));
                if let Some(root) = sqrt_mod_p(&disc.value, p) {
                    let two_inv = qa.add(&qa).inv_vartime().expect("2a invertible");
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
            if let Some(inv) = lin[1].inv_vartime() {
                roots.push(lin[0].neg().mul(&inv));
            }
        } else if lin.len() == 2 {
            // constant-only factor (should not happen after gcd)
        }
    }
    roots.dedup_by(|a, b| a.value == b.value);
    roots
}

/// Stack-resident `[u64; 9]` polynomial Cantor–Zassenhaus root-finder for odd primes
/// `2^20 < p < 2^63` and `deg(f) <= 8`, using division-free Montgomery arithmetic.
fn find_roots_fp_u64_stack(coeffs: &[FieldElement], p_big: &BigUint) -> Option<Vec<u64>> {
    let digits = p_big.to_u64_digits();
    if digits.len() != 1 {
        return None;
    }
    let p = digits[0];
    if p <= 3 || (p & 1) == 0 || (p >> 63) != 0 {
        return None;
    }

    // Single-word Montgomery context (`R = 2^64`, `-p^-1 mod 2^64`):
    let mut pinv = 1u64;
    for _ in 0..6 {
        pinv = pinv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(pinv)));
    }
    let neg_pinv = pinv.wrapping_neg();
    let r_mod_p = (((1u128 << 64) % (p as u128)) as u64) % p;
    let r2_mod_p = (((r_mod_p as u128) * (r_mod_p as u128)) % (p as u128)) as u64;

    #[inline(always)]
    fn mmul(a: u64, b: u64, p: u64, neg_pinv: u64) -> u64 {
        let t = (a as u128) * (b as u128);
        let m = (t as u64).wrapping_mul(neg_pinv);
        let u = ((t + (m as u128) * (p as u128)) >> 64) as u64;
        if u >= p {
            u - p
        } else {
            u
        }
    }
    #[inline(always)]
    fn madd(a: u64, b: u64, p: u64) -> u64 {
        let s = a + b;
        if s >= p {
            s - p
        } else {
            s
        }
    }
    #[inline(always)]
    fn msub(a: u64, b: u64, p: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + (p - b)
        }
    }
    #[inline(always)]
    fn mneg(a: u64, p: u64) -> u64 {
        if a == 0 {
            0
        } else {
            p - a
        }
    }
    #[inline]
    fn mpow(mut base: u64, mut exp: u64, p: u64, neg_pinv: u64, one_m: u64) -> u64 {
        let mut acc = one_m;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = mmul(acc, base, p, neg_pinv);
            }
            exp >>= 1;
            if exp > 0 {
                base = mmul(base, base, p, neg_pinv);
            }
        }
        acc
    }
    #[inline]
    fn minv(a_mont: u64, p: u64, neg_pinv: u64, r2: u64) -> Option<u64> {
        if a_mont == 0 {
            return None;
        }
        let a_can = mmul(a_mont, 1, p, neg_pinv);
        let (mut old_r, mut r) = (a_can, p);
        let (mut old_s, mut s) = (1i64, 0i64);
        while r != 0 {
            let q = old_r / r;
            (old_r, r) = (r, old_r - q * r);
            (old_s, s) = (s, old_s - (q as i64) * s);
        }
        if old_r != 1 {
            return None;
        }
        let inv_can = if old_s < 0 {
            (old_s + p as i64) as u64
        } else {
            old_s as u64
        };
        Some(mmul(inv_can, r2, p, neg_pinv))
    }

    #[derive(Clone, Copy)]
    struct StackPoly {
        c: [u64; 9],
        len: usize,
    }
    impl StackPoly {
        #[inline(always)]
        fn zero() -> Self {
            Self { c: [0; 9], len: 1 }
        }
        #[inline(always)]
        fn trim(&mut self) {
            while self.len > 1 && self.c[self.len - 1] == 0 {
                self.len -= 1;
            }
        }
        #[inline(always)]
        fn deg(&self) -> usize {
            self.len - 1
        }
        #[inline(always)]
        fn is_zero(&self) -> bool {
            self.len == 1 && self.c[0] == 0
        }
    }

    let poly_mul_mod_monic = |a: &StackPoly, b: &StackPoly, m: &StackPoly| -> StackPoly {
        let md = m.deg();
        let mut prod = [0u64; 18];
        for i in 0..a.len {
            let ai = a.c[i];
            if ai == 0 {
                continue;
            }
            for j in 0..b.len {
                prod[i + j] = madd(prod[i + j], mmul(ai, b.c[j], p, neg_pinv), p);
            }
        }
        let top = (a.len + b.len - 1).min(17);
        for k in (md..=top).rev() {
            let lead = prod[k];
            if lead == 0 {
                continue;
            }
            prod[k] = 0;
            let sh = k - md;
            for i in 0..md {
                prod[sh + i] = msub(prod[sh + i], mmul(lead, m.c[i], p, neg_pinv), p);
            }
        }
        let mut out = StackPoly {
            c: [0; 9],
            len: md.max(1),
        };
        out.c[..md].copy_from_slice(&prod[..md]);
        out.trim();
        out
    };

    let poly_pow_mod_monic = |mut base: StackPoly, mut exp: u64, m: &StackPoly| -> StackPoly {
        let mut acc = StackPoly::zero();
        acc.c[0] = r_mod_p;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = poly_mul_mod_monic(&acc, &base, m);
            }
            exp >>= 1;
            if exp > 0 {
                base = poly_mul_mod_monic(&base, &base, m);
            }
        }
        acc
    };

    let poly_gcd_monic = |mut a: StackPoly, mut b: StackPoly| -> Option<StackPoly> {
        a.trim();
        b.trim();
        while !b.is_zero() {
            let bd = b.deg();
            if bd == 0 {
                let mut one_p = StackPoly::zero();
                one_p.c[0] = r_mod_p;
                return Some(one_p);
            }
            let binv = minv(b.c[bd], p, neg_pinv, r2_mod_p)?;
            while !a.is_zero() && a.deg() >= bd {
                let ad = a.deg();
                let factor = mmul(a.c[ad], binv, p, neg_pinv);
                let sh = ad - bd;
                a.c[ad] = 0;
                for i in 0..bd {
                    a.c[sh + i] = msub(a.c[sh + i], mmul(factor, b.c[i], p, neg_pinv), p);
                }
                a.trim();
            }
            core::mem::swap(&mut a, &mut b);
        }
        let ad = a.deg();
        let ainv = minv(a.c[ad], p, neg_pinv, r2_mod_p)?;
        for i in 0..=ad {
            a.c[i] = mmul(a.c[i], ainv, p, neg_pinv);
        }
        Some(a)
    };

    let poly_div_exact_monic = |mut a: StackPoly, b: &StackPoly| -> StackPoly {
        let bd = b.deg();
        let mut q = StackPoly {
            c: [0; 9],
            len: a.deg() - bd + 1,
        };
        while !a.is_zero() && a.deg() >= bd {
            let ad = a.deg();
            let sh = ad - bd;
            let factor = a.c[ad]; // b is monic (`b.c[bd] == r_mod_p`)
            q.c[sh] = factor;
            a.c[ad] = 0;
            for i in 0..bd {
                a.c[sh + i] = msub(a.c[sh + i], mmul(factor, b.c[i], p, neg_pinv), p);
            }
            a.trim();
        }
        q.trim();
        q
    };

    let sqrt_mont = |n_m: u64| -> Option<u64> {
        if n_m == 0 {
            return Some(0);
        }
        if mpow(n_m, (p - 1) >> 1, p, neg_pinv, r_mod_p) != r_mod_p {
            return None;
        }
        if p & 3 == 3 {
            return Some(mpow(n_m, (p + 1) >> 2, p, neg_pinv, r_mod_p));
        }
        let s = (p - 1).trailing_zeros();
        let q = (p - 1) >> s;
        let mut z = madd(r_mod_p, r_mod_p, p);
        let minus_one = p - r_mod_p;
        while mpow(z, (p - 1) >> 1, p, neg_pinv, r_mod_p) != minus_one {
            z = madd(z, r_mod_p, p);
        }
        let mut m_ts = s;
        let mut c = mpow(z, q, p, neg_pinv, r_mod_p);
        let mut t = mpow(n_m, q, p, neg_pinv, r_mod_p);
        let mut r = mpow(n_m, (q + 1) >> 1, p, neg_pinv, r_mod_p);
        while t != r_mod_p {
            let mut i = 1u32;
            let mut tt = mmul(t, t, p, neg_pinv);
            while tt != r_mod_p {
                tt = mmul(tt, tt, p, neg_pinv);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m_ts - i - 1) {
                b = mmul(b, b, p, neg_pinv);
            }
            m_ts = i;
            c = mmul(b, b, p, neg_pinv);
            t = mmul(t, c, p, neg_pinv);
            r = mmul(r, b, p, neg_pinv);
        }
        Some(r)
    };

    let mut f = StackPoly {
        c: [0; 9],
        len: coeffs.len().max(1),
    };
    for (idx, fe) in coeffs.iter().enumerate() {
        let v = if fe.value.is_zero() {
            0u64
        } else {
            let d = fe.value.to_u64_digits();
            if d.len() > 1 {
                return None;
            }
            d[0] % p
        };
        f.c[idx] = mmul(v, r2_mod_p, p, neg_pinv);
    }
    f.trim();
    if f.is_zero() || f.deg() == 0 {
        return Some(Vec::new());
    }
    let lead_inv = minv(f.c[f.deg()], p, neg_pinv, r2_mod_p)?;
    for i in 0..f.len {
        f.c[i] = mmul(f.c[i], lead_inv, p, neg_pinv);
    }

    if f.deg() == 1 {
        let root_m = mneg(f.c[0], p);
        return Some(vec![mmul(root_m, 1, p, neg_pinv)]);
    }

    // X^p mod f(X)
    let x_poly = if f.deg() > 1 {
        let mut xp = StackPoly { c: [0; 9], len: 2 };
        xp.c[1] = r_mod_p;
        xp
    } else {
        let mut xp = StackPoly::zero();
        xp.c[0] = mneg(f.c[0], p);
        xp
    };
    let mut x_pow_p = poly_pow_mod_monic(x_poly, p, &f);
    if x_pow_p.len < 2 {
        x_pow_p.len = 2;
    }
    x_pow_p.c[1] = msub(x_pow_p.c[1], r_mod_p, p);
    x_pow_p.trim();

    let g = poly_gcd_monic(f, x_pow_p)?;
    if g.deg() == 0 {
        return Some(Vec::new());
    }

    let mut roots_m = [0u64; 8];
    let mut n_roots = 0usize;
    let mut queue = [StackPoly::zero(); 16];
    let mut q_len = 1usize;
    queue[0] = g;
    let half_pm1 = (p - 1) >> 1;
    let inv2_m = minv(madd(r_mod_p, r_mod_p, p), p, neg_pinv, r2_mod_p)?;

    while q_len > 0 {
        q_len -= 1;
        let h = queue[q_len];
        let hd = h.deg();
        if hd == 0 {
            continue;
        }
        if hd == 1 {
            roots_m[n_roots] = mneg(h.c[0], p);
            n_roots += 1;
            continue;
        }
        if hd == 2 {
            let qb = h.c[1];
            let qc = h.c[0];
            let four_qc = {
                let two_qc = madd(qc, qc, p);
                madd(two_qc, two_qc, p)
            };
            let disc = msub(mmul(qb, qb, p, neg_pinv), four_qc, p);
            if let Some(sq) = sqrt_mont(disc) {
                let nqb = mneg(qb, p);
                roots_m[n_roots] = mmul(madd(nqb, sq, p), inv2_m, p, neg_pinv);
                roots_m[n_roots + 1] = mmul(msub(nqb, sq, p), inv2_m, p, neg_pinv);
                n_roots += 2;
            }
            continue;
        }
        let mut split_opt = None;
        for c_u32 in 1u64..64 {
            let c_m = mmul(c_u32 % p, r2_mod_p, p, neg_pinv);
            let mut xc = StackPoly { c: [0; 9], len: 2 };
            xc.c[0] = c_m;
            xc.c[1] = r_mod_p;
            let mut pow = poly_pow_mod_monic(xc, half_pm1, &h);
            pow.c[0] = msub(pow.c[0], r_mod_p, p);
            pow.trim();
            if let Some(d) = poly_gcd_monic(h, pow) {
                if d.deg() > 0 && d.deg() < hd {
                    split_opt = Some(d);
                    break;
                }
            }
        }
        if let Some(d) = split_opt {
            let q_poly = poly_div_exact_monic(h, &d);
            queue[q_len] = d;
            queue[q_len + 1] = q_poly;
            q_len += 2;
        }
    }

    let mut out = Vec::with_capacity(n_roots);
    for &rm in &roots_m[..n_roots] {
        let rc = mmul(rm, 1, p, neg_pinv);
        if !out.contains(&rc) {
            out.push(rc);
        }
    }
    Some(out)
}

/// Polynomial remainder `a mod modulus` over `F_p` (any nonzero lead).
///
/// A monic modulus — what every call in the root finder passes — needs no
/// inversion at all, which matters: the reduction runs once per squaring
/// in `X^p mod f` and in `(X + c)^((p−1)/2) mod f`.
fn poly_rem(a: &[FieldElement], modulus: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    let m_deg = modulus.len() - 1;
    let monic = modulus[m_deg].value.is_one();
    let lead_inv = if monic {
        None
    } else {
        match modulus[m_deg].inv_vartime() {
            Some(v) => Some(v),
            None => return a.to_vec(),
        }
    };
    let mut r = a.to_vec();
    loop {
        while r.len() > 1 && r.last().map(|c| c.is_zero()).unwrap_or(true) {
            r.pop();
        }
        if r.is_empty() || r.len() - 1 < m_deg {
            break;
        }
        let factor = match &lead_inv {
            Some(inv) => r[r.len() - 1].mul(inv),
            None => r[r.len() - 1].clone(),
        };
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
    if let Some(inv) = a[a.len() - 1].inv_vartime() {
        for coeff in a.iter_mut() {
            *coeff = coeff.mul(&inv);
        }
    }
    a
}

/// Exact quotient `a / b` for polynomials with `b | a`.
fn poly_div_exact(a: &[FieldElement], b: &[FieldElement], p: &BigUint) -> Vec<FieldElement> {
    let b_deg = b.len() - 1;
    let lead_inv = b[b_deg].inv_vartime().expect("divisor lead is invertible");
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

/// x-coordinate → factor-base index, for O(1) lookups.  Depends only on the
/// factor base, so callers that draw many relations from one factor base build
/// it once and pass it to [`find_one_relation_with_index`].
pub(crate) fn factor_base_x_index(
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
    let word = if std::env::var("IC_FAST_SWEEP").as_deref() == Ok("0") {
        None
    } else {
        WordSweep::new(curve, factor_base)
    };
    find_one_relation_sweep_with_index(
        curve,
        g,
        q,
        factor_base,
        &x_to_idx,
        word.as_ref(),
        max_trials,
        rng,
    )
}

/// [`find_one_relation`] with a prebuilt x-index (from [`factor_base_x_index`]
/// over the same `factor_base`): the same search on the same random stream,
/// without rebuilding the index on every call.
pub(crate) fn find_one_relation_with_index(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    x_to_idx: &std::collections::HashMap<BigUint, usize>,
    max_trials: usize,
) -> Option<Relation> {
    let mut rng = rand::rngs::OsRng;
    find_one_relation_sweep_with_index(
        curve,
        g,
        q,
        factor_base,
        x_to_idx,
        None,
        max_trials,
        &mut rng,
    )
    .map(|(rel, _)| rel)
}

/// The sweep's inner loop on `u64` words (small odd prime fields below 2^40): the same candidate set,
/// in the same order, as the `BigUint` loop in [`find_one_relation_sweep_with_index`].  Enabled by
/// default when [`WordSweep::new`] succeeds; `IC_FAST_SWEEP=0` selects the BigUint sweep.  The design
/// and the equality of results were first established by an independent validator's probe.
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

/// The square-root sweep with a prebuilt x-index.  The solver builds the index
/// once per factor base; `IC_HOIST_SWEEP=0` restores the old per-attempt rebuild
/// (via [`find_one_relation_counted_with_rng`]) for A/A comparison.
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
                let inv = match qb.inv_vartime() {
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
            let two_qa_inv = match qa.add(&qa).inv_vartime() {
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
            let lhs = ic_padd(&p_i, &p_j, &a_fe);
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

    // Round-4: single-word primes use stack Poly4Fp + u64 root-finding.
    // Round-6: JSF/mont double-scalar for R; Poly4Fp::roots / u64 x-index.
    // Round-9: default probes Semaev poly only at factor-base x-coords
    // (O(fb) evals/pair) instead of enumerating all roots in F_p then
    // filtering — wins when fb ≪ p.  `IC_S4_FB_PROBE=0` restores roots().
    let p_u64 = curve.p.to_u64_digits();
    let fast_u64 = matches!(p_u64.as_slice(), [p] if *p > 3 && (*p & 1) == 1 && (*p >> 63) == 0);
    let (a_u64, b_u64, p_word) = if fast_u64 {
        (
            a_fe.value.to_u64_digits().first().copied().unwrap_or(0),
            b_fe.value.to_u64_digits().first().copied().unwrap_or(0),
            p_u64[0],
        )
    } else {
        (0, 0, 0)
    };
    let fb_probe = std::env::var("IC_S4_FB_PROBE").as_deref() != Ok("0");
    let fb_xs_u64: Option<Vec<(u64, usize)>> = if fast_u64 {
        let mut v = Vec::with_capacity(factor_base.len());
        for fb in factor_base {
            if let Point::Affine { x, .. } = &fb.point {
                if let Some(xu) = x.value.to_u64_digits().first().copied() {
                    v.push((xu, fb.idx));
                }
            }
        }
        Some(v)
    } else {
        None
    };
    let x_to_idx_u64: Option<std::collections::HashMap<u64, usize>> = if fast_u64 && !fb_probe {
        let mut m = std::collections::HashMap::with_capacity(factor_base.len());
        for &(xu, idx) in fb_xs_u64.as_ref().expect("word fb") {
            m.insert(xu, idx);
        }
        Some(m)
    } else {
        None
    };

    for trial in 1..=max_trials {
        let a = random_scalar(&curve.n);
        let b = random_scalar(&curve.n);
        let r = g.double_scalar_mul_vartime(&a, q, &b, &a_fe);
        let xr = match &r {
            Point::Affine { x, .. } => x.clone(),
            Point::Infinity => continue,
        };
        let xr_u = if fast_u64 {
            xr.value.to_u64_digits().first().copied().unwrap_or(0)
        } else {
            0
        };

        // Sweep factor-base pairs (i < j keeps the sweep half-size; the
        // sign resolution below covers both orders).
        if fast_u64 {
            let fb_xs = fb_xs_u64.as_ref().expect("word fb");
            for (i, fb_i) in factor_base.iter().enumerate() {
                let xi_u = match &fb_i.point {
                    Point::Affine { x, .. } => {
                        x.value.to_u64_digits().first().copied().unwrap_or(0)
                    }
                    Point::Infinity => continue,
                };
                for (j, fb_j) in factor_base.iter().enumerate().skip(i + 1) {
                    let xj_u = match &fb_j.point {
                        Point::Affine { x, .. } => {
                            x.value.to_u64_digits().first().copied().unwrap_or(0)
                        }
                        Point::Infinity => continue,
                    };
                    let poly = semaev_s4_poly4fp(xr_u, xi_u, xj_u, a_u64, b_u64, p_word);
                    if fb_probe {
                        for &(xk, k) in fb_xs {
                            if poly.eval(xk) != 0 {
                                continue;
                            }
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
                    } else {
                        let idx_u = x_to_idx_u64.as_ref().expect("word index");
                        for xk in poly.roots() {
                            let k = match idx_u.get(&xk).copied() {
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
        } else {
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
    }
    None
}

/// Solve `A·X² + B·X + C ≡ 0 (mod p)` over `F_p`, returning its
/// `F_p`-rational roots (zero, one, or two of them).
///
/// Variable-time: the coefficients come from public factor-base data, and
/// every inversion here is on a public value.  `A = 0` is handled as the
/// linear case (the S₃ quadratic degenerates there when the two fixed
/// abscissae coincide).
fn quadratic_roots_fp(
    qa: &FieldElement,
    qb: &FieldElement,
    qc: &FieldElement,
    p: &BigUint,
) -> Vec<FieldElement> {
    if qa.is_zero() {
        if qb.is_zero() {
            return Vec::new();
        }
        return match qb.inv_vartime() {
            Some(inv) => vec![qc.neg().mul(&inv)],
            None => Vec::new(),
        };
    }
    let four = FieldElement::new(BigUint::from(4u32), p.clone());
    let disc = qb.mul(qb).sub(&four.mul(qa).mul(qc));
    let Some(root) = sqrt_mod_p(&disc.value, p) else {
        return Vec::new();
    };
    let two_a_inv = match qa.add(qa).inv_vartime() {
        Some(v) => v,
        None => return Vec::new(),
    };
    let s = FieldElement::new(root, p.clone());
    let r1 = qb.neg().add(&s).mul(&two_a_inv);
    let r2 = qb.neg().sub(&s).mul(&two_a_inv);
    if r1.value == r2.value {
        vec![r1]
    } else {
        vec![r1, r2]
    }
}

/// **Find one 3-decomposition relation through an S₃-anchored sweep.**
///
/// This finds exactly the same relation family as
/// [`find_one_relation_s4_counted`] — a signed triple of factor-base
/// points summing to `R = aG + bQ` — but never evaluates the quartic
/// `S₄` or runs Cantor–Zassenhaus over `F_p`.
///
/// Anchor a factor-base point `F_i` (both signs, since the base stores one
/// representative per `±` pair), form `R′ = R ∓ F_i`, and decompose `R′`
/// as a **two**-point sum: for every `x_j` in the base solve the quadratic
/// `S₃(x_j, X, x_{R′}) = 0` and keep the roots that are factor-base
/// abscissae.  `S₄(x_R, x_i, x_j, x_k) = 0` holds exactly when such a
/// signed triple exists, so the yield per trial is the same `~ 2B³/p`
/// relations as the quartic sweep (`B²/2` pairs × ≤4 roots × `B/p`), while
/// each candidate costs one quadratic solve (a few multiplications, one
/// square root) instead of a quartic plus a full `X^p mod f` and
/// equal-degree splitting.  The equivalence is not heuristic: the census
/// test `s3_anchor_sweep_matches_the_quartic_triple_census` checks both
/// oracles against a brute-force sign census on the 16-bit rung.
///
/// Counted: returns the number of random `(a, b)` trials consumed.
pub fn find_one_relation_s4_via_s3_counted(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    max_trials: usize,
) -> Option<(Relation, usize)> {
    use crate::utils::random::random_scalar;

    let a_fe = curve.a_fe();
    let b_fe = curve.fe(curve.b.clone());
    let p = curve.p.clone();
    let mut x_to_idx: std::collections::HashMap<BigUint, usize> = std::collections::HashMap::new();
    for fb in factor_base {
        if let Point::Affine { x, .. } = &fb.point {
            x_to_idx.insert(x.value.clone(), fb.idx);
        }
    }
    let negatives: Vec<Point> = factor_base.iter().map(|fb| fb.point.neg()).collect();

    for trial in 1..=max_trials {
        let a = random_scalar(&curve.n);
        let b = random_scalar(&curve.n);
        let ag = g.scalar_mul(&a, &a_fe);
        let bq = q.scalar_mul(&b, &a_fe);
        let r = ag.add(&bq, &a_fe);
        if matches!(r, Point::Infinity) {
            continue;
        }
        for (i, fb_i) in factor_base.iter().enumerate() {
            // R − F_i and R + F_i: one representative covers both signs.
            for r_prime in [r.add(&negatives[i], &a_fe), r.add(&fb_i.point, &a_fe)] {
                let x_prime = match &r_prime {
                    Point::Affine { x, .. } => x.clone(),
                    Point::Infinity => continue,
                };
                for (j, fb_j) in factor_base.iter().enumerate() {
                    let xj = match &fb_j.point {
                        Point::Affine { x, .. } => x,
                        Point::Infinity => continue,
                    };
                    let (qa, qb, qc) = semaev_s3_in_x3(xj, &x_prime, &a_fe, &b_fe);
                    for xk in quadratic_roots_fp(&qa, &qb, &qc, &p) {
                        let Some(&k) = x_to_idx.get(&xk.value) else {
                            continue;
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
    }
    None
}

/// Word-sized modular arithmetic for the MITM fast path (`p < 2^63`).
#[inline]
fn add_mod_u64(a: u64, b: u64, p: u64) -> u64 {
    let s = a as u128 + b as u128;
    (if s >= p as u128 { s - p as u128 } else { s }) as u64
}

#[inline]
fn sub_mod_u64(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}

#[inline]
fn mul_mod_u64(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

/// Extended-Euclid inverse modulo a prime `p` (no exponentiation).
fn inv_prime_u64(a: u64, p: u64) -> u64 {
    let (mut t, mut newt) = (0i128, 1i128);
    let (mut r, mut newr) = (p as i128, (a % p) as i128);
    while newr != 0 {
        let q = r / newr;
        let (t2, r2) = (t - q * newt, r - q * newr);
        t = newt;
        newt = t2;
        r = newr;
        newr = r2;
    }
    (t.rem_euclid(p as i128)) as u64
}

/// Montgomery batch inversion over a prime `p`: one extended-Euclid inverse
/// and `3n` multiplications for `n` nonzero inputs. Zero inputs must be
/// replaced by 1 by the caller (their inverses are unused).
fn batch_inv_prime_u64(vals: &[u64], p: u64) -> Vec<u64> {
    let n = vals.len();
    let mut out = vec![0u64; n];
    if n == 0 {
        return out;
    }
    let mut prefix = Vec::with_capacity(n + 1);
    prefix.push(1u64);
    for &v in vals {
        let last = *prefix.last().unwrap();
        prefix.push(mul_mod_u64(last, v, p));
    }
    let mut acc = inv_prime_u64(prefix[n], p);
    for i in (0..n).rev() {
        out[i] = mul_mod_u64(acc, prefix[i], p);
        acc = mul_mod_u64(acc, vals[i], p);
    }
    out
}

/// Affine `(x1,y1) + (x2,y2)` over `F_p`, `p < 2^63`, with `u128`
/// products. Returns `None` for the point at infinity.
fn add_point_u64(p: u64, a: u64, x1: u64, y1: u64, x2: u64, y2: u64) -> Option<(u64, u64)> {
    if x1 == x2 {
        if add_mod_u64(y1, y2, p) == 0 {
            return None;
        }
        let num = add_mod_u64(mul_mod_u64(3, mul_mod_u64(x1, x1, p), p), a, p);
        let den = inv_prime_u64(add_mod_u64(y1, y1, p), p);
        let lam = mul_mod_u64(num, den, p);
        let x3 = sub_mod_u64(sub_mod_u64(mul_mod_u64(lam, lam, p), x1, p), x2, p);
        let y3 = sub_mod_u64(mul_mod_u64(lam, sub_mod_u64(x1, x3, p), p), y1, p);
        Some((x3, y3))
    } else {
        let lam = mul_mod_u64(
            sub_mod_u64(y2, y1, p),
            inv_prime_u64(sub_mod_u64(x2, x1, p), p),
            p,
        );
        let x3 = sub_mod_u64(sub_mod_u64(mul_mod_u64(lam, lam, p), x1, p), x2, p);
        let y3 = sub_mod_u64(mul_mod_u64(lam, sub_mod_u64(x1, x3, p), p), y1, p);
        Some((x3, y3))
    }
}

/// **Pair-sum table** for the meet-in-the-middle 3-decomposition oracle.
///
/// Built once per factor base: every unordered pair `{F_i, F_j}`, `i ≤ j`,
/// contributes its sum keyed by the `x`-coordinate of the sum.  The `y`
/// sign is not stored: the probe tests both `R − F_k` and `R + F_k` and
/// [`try_finalise_s4`] resolves the eight sign patterns in the group, so
/// an `x` collision (two pair sums `±S`) only costs one failed verify.
pub struct WordPairSumTable {
    by_x: std::collections::HashMap<BigUint, Vec<(u32, u32)>>,
    /// Word-sized mirror of `by_x` for `p < 2^63`; empty otherwise.
    by_x_u64: std::collections::HashMap<u64, Vec<(u32, u32)>>,
    /// Word-sized factor-base points (same indices as `factor_base`).
    fb_u64: Vec<Option<(u64, u64)>>,
    /// `p` when the word-sized fast path applies.
    p_u64: Option<u64>,
    /// `a` when the word-sized fast path applies.
    a_u64: Option<u64>,
    /// Number of non-identity pair sums stored.
    pub pairs: usize,
    /// Number of group additions spent building the table.
    pub build_additions: usize,
}

impl WordPairSumTable {
    /// `true` when the table was built in word arithmetic.
    pub fn uses_word_path(&self) -> bool {
        !self.by_x_u64.is_empty() && self.by_x.is_empty()
    }

    /// Flattened `(sum abscissa, i, j)` list.  Word keys when the word path
    /// ran, otherwise the BigUint keys reduced to one word (the caller is
    /// only here for `p < 2^64`).
    pub fn entries_word(&self) -> Vec<(u64, u32, u32)> {
        if self.uses_word_path() {
            self.by_x_u64
                .iter()
                .flat_map(|(k, v)| v.iter().map(move |&(i, j)| (*k, i, j)))
                .collect()
        } else {
            self.by_x
                .iter()
                .flat_map(|(k, v)| {
                    let key = k.to_u64_digits().first().copied().unwrap_or(0);
                    v.iter().map(move |&(i, j)| (key, i, j))
                })
                .collect()
        }
    }
}

/// Build the pair-sum table for `factor_base` (`B(B+1)/2` additions).
pub fn build_word_pair_sum_table(
    curve: &CurveParams,
    factor_base: &[FactorBaseEntry],
) -> WordPairSumTable {
    let a_fe = curve.a_fe();
    let n = factor_base.len();
    let p_u64 = if curve.p.bits() <= 63 {
        curve.p.to_u64_digits().first().copied()
    } else {
        None
    };
    let a_u64 = if p_u64.is_some() {
        curve.a.to_u64_digits().first().copied()
    } else {
        None
    };
    let fast = p_u64.is_some() && a_u64.is_some();
    let mut by_x: std::collections::HashMap<BigUint, Vec<(u32, u32)>> =
        std::collections::HashMap::with_capacity(n * (n + 1) / 2);
    let mut by_x_u64: std::collections::HashMap<u64, Vec<(u32, u32)>> =
        std::collections::HashMap::with_capacity(if fast { n * (n + 1) / 2 } else { 0 });
    let fb_u64: Vec<Option<(u64, u64)>> = if fast {
        factor_base
            .iter()
            .map(|fb| match &fb.point {
                Point::Affine { x, y } => Some((
                    x.value.to_u64_digits().first().copied()?,
                    y.value.to_u64_digits().first().copied()?,
                )),
                _ => None,
            })
            .collect()
    } else {
        vec![None; n]
    };
    let mut pairs = 0usize;
    match (p_u64, a_u64) {
        (Some(p), Some(a_u64)) => {
            // Word-sized build: the same `B(B+1)/2` sums, but in u64 with one
            // extended-Euclid inversion per pair.  The BigUint affine add
            // spends a modular exponentiation on every pair, which is why
            // the table stage dominated the 36-bit rung.
            let mut flat: Vec<(u64, u32, u32)> = Vec::with_capacity(n * (n + 1) / 2);
            for i in 0..n {
                let Some((xi, yi)) = fb_u64[i] else { continue };
                for j in i..n {
                    let Some((xj, yj)) = fb_u64[j] else { continue };
                    if let Some((x3, _)) = add_point_u64(p, a_u64, xi, yi, xj, yj) {
                        flat.push((x3, i as u32, j as u32));
                    }
                }
            }
            pairs = flat.len();
            for (key, i, j) in flat {
                by_x_u64.entry(key).or_default().push((i, j));
            }
        }
        _ => {
            for i in 0..n {
                for j in i..n {
                    if let Point::Affine { x, .. } =
                        factor_base[i].point.add(&factor_base[j].point, &a_fe)
                    {
                        by_x.entry(x.value.clone())
                            .or_default()
                            .push((i as u32, j as u32));
                        pairs += 1;
                    }
                }
            }
        }
    }
    WordPairSumTable {
        by_x,
        by_x_u64,
        fb_u64,
        p_u64,
        a_u64,
        pairs,
        build_additions: pairs,
    }
}

/// **Meet-in-the-middle 3-decomposition**: the third factor-base point is
/// enumerated (both signs), and the remaining two-point sum is answered by
/// one hash probe into [`WordPairSumTable`] instead of the `B²` pair sweep of
/// [`find_one_relation_s4_via_s3_counted`].
///
/// Cost per random `(a, b)` draw is `2B` probes, so a relation costs
/// `≈ p/B²` probes after the `B²` table is built; `B` relations then cost
/// `≈ B² + p/B` — balanced at `B ≈ p^{1/4}` for `≈ p^{3/4}` total, against
/// the `≈ p` of every enumeration sweep.  The decided relation family is
/// identical (three signed factor-base points summing to `R`), so the
/// yield per draw is unchanged and correctness is checked by the same
/// in-group finaliser.
pub fn find_one_relation_s4_via_mitm_counted(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    factor_base: &[FactorBaseEntry],
    table: &WordPairSumTable,
    max_trials: usize,
) -> Option<(Relation, usize)> {
    use crate::utils::random::random_scalar;

    let a_fe = curve.a_fe();
    let negatives: Vec<Point> = factor_base.iter().map(|fb| fb.point.neg()).collect();

    for trial in 1..=max_trials {
        let a = random_scalar(&curve.n);
        let b = random_scalar(&curve.n);
        let r = g.scalar_mul(&a, &a_fe).add(&q.scalar_mul(&b, &a_fe), &a_fe);
        if matches!(r, Point::Infinity) {
            continue;
        }
        // Word-sized probe path (`p < 2^63`): 2B u64 additions per draw,
        // exact same relation family; every hit is still finalised and
        // re-added in the BigUint group before it is accepted.
        if let (Some(p), Some(a_u64)) = (table.p_u64, table.a_u64) {
            if !table.by_x_u64.is_empty() {
                if let Point::Affine { x, y } = &r {
                    let rx = x.value.to_u64_digits().first().copied();
                    let ry = y.value.to_u64_digits().first().copied();
                    if let (Some(rx), Some(ry)) = (rx, ry) {
                        // R + F_k and R - F_k share the denominator
                        // x(F_k) - x(R); invert all of them at once.
                        let nfb = factor_base.len();
                        let mut dens = Vec::with_capacity(nfb);
                        let mut raw = Vec::with_capacity(nfb);
                        for k in 0..nfb {
                            let d = match table.fb_u64[k] {
                                Some((fx, _)) => sub_mod_u64(fx, rx, p),
                                None => 0,
                            };
                            raw.push(d);
                            dens.push(if d == 0 { 1 } else { d });
                        }
                        let invs = batch_inv_prime_u64(&dens, p);
                        for k in 0..nfb {
                            let Some((fx, fy)) = table.fb_u64[k] else {
                                continue;
                            };
                            let mut cand = [0u64; 2];
                            let mut nc = 0usize;
                            if raw[k] == 0 {
                                if let Some((cx, _)) =
                                    add_point_u64(p, a_u64, rx, ry, fx, sub_mod_u64(p, fy, p))
                                {
                                    cand[nc] = cx;
                                    nc += 1;
                                }
                                if let Some((cx, _)) = add_point_u64(p, a_u64, rx, ry, fx, fy) {
                                    cand[nc] = cx;
                                    nc += 1;
                                }
                            } else {
                                let inv = invs[k];
                                let lam_p = mul_mod_u64(sub_mod_u64(fy, ry, p), inv, p);
                                let lam_m =
                                    mul_mod_u64(sub_mod_u64(sub_mod_u64(0, fy, p), ry, p), inv, p);
                                cand[0] = sub_mod_u64(
                                    sub_mod_u64(mul_mod_u64(lam_m, lam_m, p), rx, p),
                                    fx,
                                    p,
                                );
                                cand[1] = sub_mod_u64(
                                    sub_mod_u64(mul_mod_u64(lam_p, lam_p, p), rx, p),
                                    fx,
                                    p,
                                );
                                nc = 2;
                            }
                            for &cx in &cand[..nc] {
                                let Some(pairs) = table.by_x_u64.get(&cx) else {
                                    continue;
                                };
                                for &(i, j) in pairs {
                                    let (i, j) = (i as usize, j as usize);
                                    if let Some(rel) = try_finalise_s4(
                                        &r,
                                        i,
                                        &factor_base[i].point,
                                        j,
                                        &factor_base[j].point,
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
                        continue;
                    }
                }
            }
        }
        for k in 0..factor_base.len() {
            // R ∓ F_k: both anchor signs, so the table only needs one
            // orientation of each pair sum.
            for s in [
                r.add(&negatives[k], &a_fe),
                r.add(&factor_base[k].point, &a_fe),
            ] {
                let Point::Affine { x, .. } = s else {
                    continue;
                };
                let Some(pairs) = table.by_x.get(&x.value) else {
                    continue;
                };
                for &(i, j) in pairs {
                    let (i, j) = (i as usize, j as usize);
                    if let Some(rel) = try_finalise_s4(
                        &r,
                        i,
                        &factor_base[i].point,
                        j,
                        &factor_base[j].point,
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
                let lhs = ic_padd(&ic_padd(&p_i, &p_j, &a_fe), &p_k, &a_fe);
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
    gaussian_eliminate_mod_n_particular_pivots(matrix, rhs, n).map(|(sol, _)| sol)
}

/// The word-sized modulus of `n`, when the `u64` elimination path applies.
fn word_modulus(n: &BigUint) -> Option<u64> {
    n.to_u64_digits()
        .first()
        .copied()
        .filter(|&v| n.bits() <= 64 && v > 1)
}

/// Gauss–Jordan on whichever path fits `n`.  Both paths leave
/// `matrix`/`rhs` in reduced row-echelon form with pivot rows
/// `0..rank`, and return the pivot row of each column (`usize::MAX`
/// for a free column); `None` on a non-invertible pivot.
fn gaussian_eliminate_mod_dispatch(
    matrix: &mut [Vec<BigUint>],
    rhs: &mut [BigUint],
    n: &BigUint,
) -> Option<Vec<usize>> {
    match word_modulus(n) {
        Some(n64) => gaussian_eliminate_mod_u64(matrix, rhs, n64),
        None => gaussian_eliminate_mod_biguint(matrix, rhs, n),
    }
}

/// [`gaussian_eliminate_mod_n_particular`], also returning the pivot
/// row of each column, so a caller can keep the echelon form.
fn gaussian_eliminate_mod_n_particular_pivots(
    matrix: &mut [Vec<BigUint>],
    rhs: &mut [BigUint],
    n: &BigUint,
) -> Option<(ModNSolution, Vec<usize>)> {
    let m = matrix.first().map(|r| r.len()).unwrap_or(0);
    let pivot_rows = gaussian_eliminate_mod_dispatch(matrix, rhs, n)?;
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
    Some((
        ModNSolution {
            values,
            determined,
            rank,
        },
        pivot_rows,
    ))
}

// ── Sparse relation-matrix linear algebra mod n ────────────────────────

/// Which relation-matrix engine a staged driver runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaEngine {
    /// Dense Gauss–Jordan over `F_n`: the reference path, `O(rows·cols²)`.
    Dense,
    /// Sparse Markowitz-ordered Gauss–Jordan over `F_n`.
    Sparse,
}

impl LaEngine {
    pub fn name(self) -> &'static str {
        match self {
            LaEngine::Dense => "dense_gauss_jordan_mod_n",
            LaEngine::Sparse => "sparse_markowitz_gauss_jordan_mod_n",
        }
    }
}

/// Shape and counted work of one relation-matrix solve.
///
/// `nnz_in` is the number of nonzero entries of the relation matrix as
/// collected (`≈ (m+2)·relations` for an m-summand IC row), `nnz_peak` the
/// largest nonzero count any intermediate matrix reached (fill-in), and
/// `muladds` the number of modular multiply-adds the elimination performed.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct LaStats {
    pub engine: String,
    pub rows: usize,
    pub cols: usize,
    pub nnz_in: usize,
    pub nnz_peak: usize,
    pub rank: usize,
    pub muladds: u64,
}

/// A sparse row: `column → coefficient mod n`, no zero entries.
pub type SparseRow = std::collections::BTreeMap<usize, BigUint>;

/// Build a sparse row from `(column, coefficient)` pairs reduced mod `n`.
pub fn sparse_row_from_entries(entries: &[(usize, BigUint)], n: &BigUint) -> SparseRow {
    let mut row = SparseRow::new();
    for (col, value) in entries {
        let v = value % n;
        if v.is_zero() {
            continue;
        }
        let slot = row.entry(*col).or_insert_with(BigUint::zero);
        *slot = (&*slot + &v) % n;
        if slot.is_zero() {
            row.remove(col);
        }
    }
    row
}

/// Sparse Gauss–Jordan elimination of `M · y ≡ rhs (mod n)` with Markowitz
/// pivot ordering, tolerating free unknowns.
///
/// Pivot selection is the standard sparsity heuristic: take the row with the
/// fewest entries, and inside it the column with the fewest entries, so the
/// elimination creates as little fill-in as possible.  The result is the same
/// solution and the same `determined` flags as
/// [`gaussian_eliminate_mod_n_particular`] — free unknowns are set to zero —
/// but the cost tracks the nonzero count instead of `rows·cols²`.
///
/// Returns `None` when the system is inconsistent.  `rows` and `rhs` are
/// consumed (the caller has no further use for the relation matrix).
pub fn sparse_gaussian_eliminate_mod_n(
    mut rows: Vec<SparseRow>,
    mut rhs: Vec<BigUint>,
    n: &BigUint,
    cols: usize,
) -> Option<(ModNSolution, LaStats)> {
    use std::collections::BTreeSet;

    let n_rows = rows.len();
    let nnz_in: usize = rows.iter().map(|r| r.len()).sum();
    let mut stats = LaStats {
        engine: LaEngine::Sparse.name().into(),
        rows: n_rows,
        cols,
        nnz_in,
        nnz_peak: nnz_in,
        rank: 0,
        muladds: 0,
    };
    for v in rhs.iter_mut() {
        *v %= n;
    }

    // Column → rows containing it, kept current so pivot selection and
    // elimination touch only the affected rows.
    let mut col_rows: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); cols];
    for (r, row) in rows.iter().enumerate() {
        for &c in row.keys() {
            col_rows[c].insert(r);
        }
    }

    let mut pivot_of_col: Vec<usize> = vec![usize::MAX; cols];
    let mut is_pivot_row = vec![false; n_rows];
    let mut live: BTreeSet<usize> = (0..n_rows).collect();

    loop {
        // Markowitz: the live row with the fewest entries, breaking ties by
        // index; inside it, the column with the fewest entries.
        let mut best: Option<(usize, usize, usize)> = None; // (row_deg, row, col)
        for &r in &live {
            let deg = rows[r].len();
            if deg == 0 {
                continue;
            }
            if best.is_none_or(|(d, _, _)| deg < d) {
                let mut pick: Option<(usize, usize)> = None;
                for &c in rows[r].keys() {
                    let cdeg = col_rows[c].len();
                    if pick.is_none_or(|(d2, _)| cdeg < d2) {
                        pick = Some((cdeg, c));
                    }
                }
                let (cdeg, c) = pick?;
                let _ = cdeg;
                best = Some((deg, r, c));
            }
        }
        let Some((_, r, c)) = best else { break };
        live.remove(&r);
        is_pivot_row[r] = true;

        // Normalise the pivot row so the pivot coefficient is one.
        let lead = rows[r][&c].clone();
        let lead_inv = mod_inverse(&lead, n)?;
        {
            let row = rows[r].clone();
            for (col, value) in row {
                let v = (value * &lead_inv) % n;
                if v.is_zero() {
                    rows[r].remove(&col);
                    col_rows[col].remove(&r);
                } else {
                    rows[r].insert(col, v);
                }
            }
            rhs[r] = (&rhs[r] * &lead_inv) % n;
        }
        pivot_of_col[c] = r;

        // Eliminate the pivot column from every other row that carries it.
        let targets: Vec<usize> = col_rows[c].iter().copied().filter(|&o| o != r).collect();
        let pivot_row = rows[r].clone();
        let pivot_rhs = rhs[r].clone();
        for o in targets {
            let factor = match rows[o].get(&c) {
                Some(v) if !v.is_zero() => v.clone(),
                _ => continue,
            };
            stats.muladds += pivot_row.len() as u64;
            for (col, value) in &pivot_row {
                if *col == c {
                    continue;
                }
                let prod = (&factor * value) % n;
                let existing = rows[o].get(col).cloned();
                let new = match &existing {
                    Some(cur) => (cur + n - prod) % n,
                    None => (n - prod) % n,
                };
                if new.is_zero() {
                    if existing.is_some() {
                        rows[o].remove(col);
                        col_rows[*col].remove(&o);
                    }
                } else {
                    // Fill-in: a column row `o` did not carry must be
                    // registered so a later pivot on it reaches this row.
                    if existing.is_none() {
                        col_rows[*col].insert(o);
                    }
                    rows[o].insert(*col, new);
                }
            }
            rows[o].remove(&c);
            col_rows[c].remove(&o);
            let prod = (&factor * &pivot_rhs) % n;
            rhs[o] = (&rhs[o] + n - prod) % n;
        }
        stats.nnz_peak = stats
            .nnz_peak
            .max(rows.iter().map(|x| x.len()).sum::<usize>());
    }

    // Rows that are now empty with a nonzero right-hand side are `0 = c`.
    for r in 0..n_rows {
        if rows[r].is_empty() && !rhs[r].is_zero() {
            return None;
        }
    }

    let pivot_cols: Vec<usize> = (0..cols)
        .filter(|&c| pivot_of_col[c] != usize::MAX)
        .collect();
    stats.rank = pivot_cols.len();
    let free: Vec<usize> = (0..cols)
        .filter(|&c| pivot_of_col[c] == usize::MAX)
        .collect();
    let mut values = vec![BigUint::zero(); cols];
    let mut determined = vec![false; cols];
    for &c in &pivot_cols {
        let r = pivot_of_col[c];
        values[c] = rhs[r].clone();
        determined[c] = free.iter().all(|&f| !rows[r].contains_key(&f));
    }
    let _ = is_pivot_row;
    Some((
        ModNSolution {
            values,
            determined,
            rank: stats.rank,
        },
        stats,
    ))
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

// ── Incremental elimination across rank-deficient retries ───────────────

/// Arithmetic mod `n` for [`Echelon`], in the word the elimination path
/// for `n` uses.
trait EchelonField {
    type E: Clone;
    fn lift_big(&self, v: &BigUint) -> Self::E;
    fn to_big(&self, v: &Self::E) -> BigUint;
    fn is_zero(v: &Self::E) -> bool;
    /// `t − f·p mod n`.
    fn sub_mul(&self, t: &Self::E, f: &Self::E, p: &Self::E) -> Self::E;
}

/// A modulus of at most 64 bits, as [`gaussian_eliminate_mod_u64`] uses.
struct WordField(u64);

impl EchelonField for WordField {
    type E = u64;
    fn lift_big(&self, v: &BigUint) -> u64 {
        (v % self.0).to_u64_digits().first().copied().unwrap_or(0)
    }
    fn to_big(&self, v: &u64) -> BigUint {
        BigUint::from(*v)
    }
    fn is_zero(v: &u64) -> bool {
        *v == 0
    }
    fn sub_mul(&self, t: &u64, f: &u64, p: &u64) -> u64 {
        let n = self.0 as u128;
        ((*t as u128 + (n - *f as u128) * *p as u128) % n) as u64
    }
}

/// Any modulus, as [`gaussian_eliminate_mod_biguint`] uses.
struct BigField(BigUint);

impl EchelonField for BigField {
    type E = BigUint;
    fn lift_big(&self, v: &BigUint) -> BigUint {
        v % &self.0
    }
    fn to_big(&self, v: &BigUint) -> BigUint {
        v.clone()
    }
    fn is_zero(v: &BigUint) -> bool {
        v.is_zero()
    }
    fn sub_mul(&self, t: &BigUint, f: &BigUint, p: &BigUint) -> BigUint {
        let n = &self.0;
        (t + n - (f * p) % n) % n
    }
}

/// The reduced row-echelon form of a consistent system
/// `M · y ≡ rhs (mod n)`: its pivot rows only (every other row reduced
/// to `0 = 0`), each normalised and zero in every other pivot column.
struct Echelon<F: EchelonField> {
    field: F,
    /// Number of unknowns.
    m: usize,
    rows: Vec<Vec<F::E>>,
    rhs: Vec<F::E>,
    /// `pivot_row[c]`: index into `rows` of column `c`'s pivot, or
    /// `usize::MAX` for a free column.
    pivot_row: Vec<usize>,
}

/// What [`EchelonState::extend`] found.
enum EchelonExtend {
    /// The rows so far are consistent; the solution of all of them.
    Solved(ModNSolution),
    /// Some new row contradicts the system.
    Inconsistent,
    /// A pivot was not invertible (composite `n` only); the state is
    /// unusable and the caller eliminates from scratch.
    NonInvertible,
}

impl<F: EchelonField> Echelon<F> {
    fn from_reduced(
        field: F,
        matrix: &[Vec<BigUint>],
        rhs: &[BigUint],
        pivot_row: Vec<usize>,
    ) -> Self {
        let rank = pivot_row.iter().filter(|&&r| r != usize::MAX).count();
        let rows = matrix[..rank]
            .iter()
            .map(|r| r.iter().map(|v| field.lift_big(v)).collect())
            .collect();
        let rhs = rhs[..rank].iter().map(|v| field.lift_big(v)).collect();
        Self {
            field,
            m: pivot_row.len(),
            rows,
            rhs,
            pivot_row,
        }
    }

    /// Adds rows to the system: reduces each against the existing
    /// pivots, runs Gauss–Jordan on the reduced block alone, then clears
    /// the block's new pivot columns from the existing pivot rows.  The
    /// result is the reduced row-echelon form of all rows so far, which
    /// is unique for prime `n`, so the solution and `determined` flags
    /// equal those of a full elimination of every row.  Charges its
    /// modular operations to `bitop/v1` as the full elimination does.
    fn extend(
        &mut self,
        matrix: Vec<Vec<BigUint>>,
        rhs: Vec<BigUint>,
        n: &BigUint,
    ) -> EchelonExtend {
        let m = self.m;
        let field = &self.field;
        let old_pivots: Vec<(usize, usize)> = (0..m)
            .filter(|&c| self.pivot_row[c] != usize::MAX)
            .map(|c| (c, self.pivot_row[c]))
            .collect();
        // 1. Reduce each new row against the existing pivot rows.  A
        //    pivot row is zero left of its pivot, so each sweep starts
        //    at the pivot column.
        let mut block: Vec<Vec<F::E>> = Vec::with_capacity(matrix.len());
        let mut block_rhs: Vec<F::E> = Vec::with_capacity(matrix.len());
        for (row, b) in matrix.iter().zip(&rhs) {
            let mut row: Vec<F::E> = row.iter().map(|v| field.lift_big(v)).collect();
            let mut b = field.lift_big(b);
            for &(c, pr) in &old_pivots {
                bitop_ops::word_op(1);
                if F::is_zero(&row[c]) {
                    continue;
                }
                bitop_ops::modr_mul((m - c) as u64 + 1);
                bitop_ops::modr_add((m - c) as u64 + 1);
                let f = row[c].clone();
                let p = &self.rows[pr];
                for k in c..m {
                    row[k] = field.sub_mul(&row[k], &f, &p[k]);
                }
                b = field.sub_mul(&b, &f, &self.rhs[pr]);
            }
            block.push(row);
            block_rhs.push(b);
        }
        if block.is_empty() {
            return EchelonExtend::Solved(self.solution());
        }
        // 2. Gauss–Jordan on the block: it is zero in every old pivot
        //    column, so its pivots are new columns.
        let mut bm: Vec<Vec<BigUint>> = block
            .iter()
            .map(|r| r.iter().map(|v| field.to_big(v)).collect())
            .collect();
        let mut bb: Vec<BigUint> = block_rhs.iter().map(|v| field.to_big(v)).collect();
        let Some(block_pivots) = gaussian_eliminate_mod_dispatch(&mut bm, &mut bb, n) else {
            return EchelonExtend::NonInvertible;
        };
        let block_rank = block_pivots.iter().filter(|&&r| r != usize::MAX).count();
        bitop_ops::word_op((bb.len() - block_rank) as u64);
        if bb.iter().skip(block_rank).any(|v| !(v % n).is_zero()) {
            return EchelonExtend::Inconsistent;
        }
        let new_rows: Vec<Vec<F::E>> = bm[..block_rank]
            .iter()
            .map(|r| r.iter().map(|v| field.lift_big(v)).collect())
            .collect();
        let new_rhs: Vec<F::E> = bb[..block_rank].iter().map(|v| field.lift_big(v)).collect();
        // 3. Clear each new pivot column from the old pivot rows.  A new
        //    pivot row is zero left of its pivot, in every old pivot
        //    column and in every other new pivot column, so the old rows
        //    stay reduced.
        let new_pivots: Vec<(usize, usize)> = (0..m)
            .filter(|&c| block_pivots[c] != usize::MAX)
            .map(|c| (c, block_pivots[c]))
            .collect();
        for &(c, br) in &new_pivots {
            let p = &new_rows[br];
            for (row, b) in self.rows.iter_mut().zip(self.rhs.iter_mut()) {
                bitop_ops::word_op(1);
                if F::is_zero(&row[c]) {
                    continue;
                }
                bitop_ops::modr_mul((m - c) as u64 + 1);
                bitop_ops::modr_add((m - c) as u64 + 1);
                let f = row[c].clone();
                for k in c..m {
                    row[k] = field.sub_mul(&row[k], &f, &p[k]);
                }
                *b = field.sub_mul(b, &f, &new_rhs[br]);
            }
        }
        let base = self.rows.len();
        for &(c, br) in &new_pivots {
            self.pivot_row[c] = base + br;
        }
        self.rows.extend(new_rows);
        self.rhs.extend(new_rhs);
        EchelonExtend::Solved(self.solution())
    }

    /// As [`gaussian_eliminate_mod_n_particular_pivots`] reads its
    /// solution off the reduced form, with the same word-op charge.
    fn solution(&self) -> ModNSolution {
        let m = self.m;
        let free: Vec<usize> = (0..m)
            .filter(|&c| self.pivot_row[c] == usize::MAX)
            .collect();
        let mut values = vec![BigUint::zero(); m];
        let mut determined = vec![false; m];
        for c in 0..m {
            let pr = self.pivot_row[c];
            if pr == usize::MAX {
                continue;
            }
            values[c] = self.field.to_big(&self.rhs[pr]);
            bitop_ops::word_op(free.len() as u64);
            determined[c] = free.iter().all(|&f| F::is_zero(&self.rows[pr][f]));
        }
        ModNSolution {
            values,
            determined,
            rank: self.rows.len(),
        }
    }
}

/// [`Echelon`] in the word [`gaussian_eliminate_mod_n_particular`]
/// would use for `n`.  Kept across the rank-deficient retries of the
/// staged solver, so each retry reduces only the new relations instead
/// of eliminating the whole rebuilt matrix again.
enum EchelonState {
    Word(Echelon<WordField>),
    Big(Echelon<BigField>),
}

impl EchelonState {
    /// From a matrix and right-hand side left in reduced form by
    /// [`gaussian_eliminate_mod_n_particular_pivots`], with its pivots.
    fn from_reduced(
        matrix: &[Vec<BigUint>],
        rhs: &[BigUint],
        pivot_row: Vec<usize>,
        n: &BigUint,
    ) -> Self {
        match word_modulus(n) {
            Some(n64) => Self::Word(Echelon::from_reduced(
                WordField(n64),
                matrix,
                rhs,
                pivot_row,
            )),
            None => Self::Big(Echelon::from_reduced(
                BigField(n.clone()),
                matrix,
                rhs,
                pivot_row,
            )),
        }
    }

    fn extend(
        &mut self,
        matrix: Vec<Vec<BigUint>>,
        rhs: Vec<BigUint>,
        n: &BigUint,
    ) -> EchelonExtend {
        match self {
            Self::Word(e) => e.extend(matrix, rhs, n),
            Self::Big(e) => e.extend(matrix, rhs, n),
        }
    }
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

    let x_to_idx = factor_base_x_index(&fb);
    let mut relations = Vec::with_capacity(target);
    while relations.len() < target {
        let rel =
            find_one_relation_with_index(curve, g, q, &fb, &x_to_idx, max_trials_per_relation)?;
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
    // Verify Q ≡ x·G independently of the linear algebra (public `x`: vartime).
    let a_fe = curve.a_fe();
    let candidate = g.scalar_mul_vartime(&x, &a_fe);
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

/// Assemble the relation matrix `[ M | −b ] · (y, x) = a` and solve it.
///
/// Row `r` of the system produced by a relation `R_r = a_r·G + b_r·Q =
/// Σ ε_{r,j} F_j` is `Σ ε_{r,j} y_j − b_r·x ≡ a_r (mod n)`, so the matrix has
/// one column per factor-base point plus the target column.  `Dense` builds
/// the full `rows × cols` matrix; `Sparse` keeps only the nonzero entries
/// (an m-summand row has at most `m + 2` of them).
///
/// Both engines return the same solution and the same `determined` flags;
/// the returned [`LaStats`] records which one ran, the input nonzero count
/// (a property of the relation set, so equal for both) and the peak nonzero
/// count (fill-in for the sparse engine, `rows·cols` for dense).
fn solve_relation_matrix(
    relations: &[Relation],
    cols: usize,
    n: &BigUint,
    engine: LaEngine,
) -> Option<(ModNSolution, LaStats)> {
    let rows = relations.len();
    match engine {
        LaEngine::Dense => {
            let mut matrix: Vec<Vec<BigUint>> = Vec::with_capacity(rows);
            let mut rhs: Vec<BigUint> = Vec::with_capacity(rows);
            let mut nnz_in = 0usize;
            for rel in relations {
                let mut row = vec![BigUint::zero(); cols];
                for &(j, mult) in &rel.entries {
                    let val = signed_mod(mult, n);
                    if !val.is_zero() {
                        nnz_in += 1;
                    }
                    row[j] = (&row[j] + &val) % n;
                }
                let neg_b = (n - &(&rel.coef_b % n)) % n;
                if !neg_b.is_zero() {
                    nnz_in += 1;
                }
                row[cols - 1] = (&row[cols - 1] + &neg_b) % n;
                matrix.push(row);
                rhs.push(rel.coef_a.clone() % n);
            }
            let solution = gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, n)?;
            let stats = LaStats {
                engine: engine.name().into(),
                rows,
                cols,
                nnz_in,
                nnz_peak: rows * cols,
                rank: solution.rank,
                muladds: 0,
            };
            Some((solution, stats))
        }
        LaEngine::Sparse => {
            let mut sparse_rows: Vec<SparseRow> = Vec::with_capacity(rows);
            let mut rhs: Vec<BigUint> = Vec::with_capacity(rows);
            for rel in relations {
                let mut entries: Vec<(usize, BigUint)> = rel
                    .entries
                    .iter()
                    .map(|&(j, mult)| (j, signed_mod(mult, n)))
                    .collect();
                let neg_b = (n - &(&rel.coef_b % n)) % n;
                entries.push((cols - 1, neg_b));
                sparse_rows.push(sparse_row_from_entries(&entries, n));
                rhs.push(rel.coef_a.clone() % n);
            }
            sparse_gaussian_eliminate_mod_n(sparse_rows, rhs, n, cols)
        }
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
    /// Pair-sum table build (meet-in-the-middle oracle); `0.0` otherwise.
    pub pair_table_ms: f64,
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
    /// Shape and counted work of the relation-matrix solve, when the
    /// driver reports it (the S4 staged drivers do).
    pub la: Option<LaStats>,
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
        FactorBaseChoice::SmallX,
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
        FactorBaseChoice::SmallX,
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
        FactorBaseChoice::Projected,
        &mut rng,
    )
}

/// Which factor base the staged pipeline uses.
#[derive(Clone, Debug)]
pub enum FactorBaseChoice {
    /// [`build_factor_base`]: the points with the smallest abscissae.
    SmallX,
    /// [`build_factor_base_projected`].
    Projected,
    /// A factor base the caller built (e.g. [`build_factor_base_from_xs`]).
    Given(Vec<FactorBaseEntry>),
}

/// The factor base whose abscissae are the given `xs` (in order): every `x`
/// with `x³ + ax + b` a square becomes one entry, its `y` canonicalised to the
/// smaller of `y, p − y` as in [`build_factor_base`]; other `x` (and repeats)
/// are skipped. Used to compare structured abscissa sets (a Kummer coset, a
/// random set) against the small-x default (EXP-PKM-IC-001).
pub fn build_factor_base_from_xs(curve: &CurveParams, xs: &[BigUint]) -> Vec<FactorBaseEntry> {
    let mut out: Vec<FactorBaseEntry> = Vec::with_capacity(xs.len());
    let mut seen = std::collections::HashSet::new();
    for x in xs {
        let x = x % &curve.p;
        if !seen.insert(x.clone()) {
            continue;
        }
        let xf = curve.fe(x);
        let rhs_value = xf
            .mul(&xf)
            .mul(&xf)
            .add(&curve.a_fe().mul(&xf))
            .add(&curve.fe(curve.b.clone()))
            .value;
        count("sqrt_mod_p_calls_uncharged", 1);
        if let Some(y) = sqrt_mod_p(&rhs_value, &curve.p) {
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
    }
    out
}

/// Seeded staged pipeline over a factor base the caller built; otherwise
/// identical to [`ec_index_calculus_dlp_staged_seeded`].
#[allow(clippy::too_many_arguments)]
pub fn ec_index_calculus_dlp_staged_seeded_with_factor_base(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb: Vec<FactorBaseEntry>,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    seed: u64,
) -> Option<(BigUint, EcIcStageReport)> {
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let fb_size = fb.len();
    ec_index_calculus_dlp_staged_with_rng(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        FactorBaseChoice::Given(fb),
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
/// packed `(i, s_i, j, s_j)`: 12 bytes a slot at load 0.6 and one slot per +/- pair of sums (`+S`
/// and `-S` share x), about 20 bytes a pair against about 340 for the exact hash map. A hit is
/// checked against the factor-base points (one addition, only on a hit), so a fingerprint
/// collision is a miss, never a wrong relation; the hit set equals the exact table's, and among
/// sums with the same x the earliest inserted answers, as in the exact table.
pub struct CompactPairSumTable {
    fps: Vec<u64>,
    vals: Vec<u32>,
    points: Vec<Point>,
    a_fe: FieldElement,
    entries: usize,
}

impl CompactPairSumTable {
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
    /// A slot holds one of the two signs of a pair sum (`+S` and `-S` share x); the first verified
    /// slot in insertion order answers, with both signs flipped when its y is the other one, which is
    /// the entry the exact table (both signs stored, first insertion wins) returns.
    pub fn find(&self, x: &BigUint, y: &BigUint) -> Option<(usize, i64, usize, i64)> {
        let fp = x_fingerprint(x);
        let mut at = self.slot(fp);
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
                        return Some(if &sy.value == y {
                            (i, s_i, j, s_j)
                        } else {
                            (i, -s_i, j, -s_j)
                        });
                    }
                }
            }
            at += 1;
            if at == self.fps.len() {
                at = 0;
            }
        }
        None
    }
}

pub fn build_compact_pair_sum_table(
    curve: &CurveParams,
    factor_base: &[FactorBaseEntry],
) -> CompactPairSumTable {
    assert!(
        factor_base.len() <= PAIR_SUM_TABLE_MAX_FB,
        "compact pair-sum table indexes at most {PAIR_SUM_TABLE_MAX_FB} factor-base points"
    );
    let a_fe = curve.a_fe();
    let f = factor_base.len();
    let slots = (f * f * 5 / 3).max(16);
    let mut table = CompactPairSumTable {
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
            // one slot serves both signs (+S and -S share x); the counters keep the exact table's
            // accounting (two insert attempts and the negations per sum) so counts stay comparable
            let sum = fi.point.add_vartime(&fj.point, &a_fe);
            put(&sum, (i, 1, j, 1));
            count("group_additions", 1);
            count("table_insert_attempts_uncharged", 2);
            count("point_negations_uncharged", 1);
            if i != j {
                let dif = fi.point.add_vartime(&fj.point.neg_vartime(), &a_fe);
                put(&dif, (i, 1, j, -1));
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
    table: &CompactPairSumTable,
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
    table: &CompactPairSumTable,
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
    fb_choice: FactorBaseChoice,
    rng: &mut R,
) -> Option<(BigUint, EcIcStageReport)> {
    use std::time::Instant;
    refresh_ic_ct_flag();
    reset_counters();
    let total_started = Instant::now();

    let fb_started = Instant::now();
    let fb = match fb_choice {
        FactorBaseChoice::SmallX => build_factor_base(curve, fb_size),
        FactorBaseChoice::Projected => build_factor_base_projected(curve, fb_size),
        FactorBaseChoice::Given(fb) => fb,
    };
    let factor_base_ms = fb_started.elapsed().as_secs_f64() * 1e3;
    if fb.is_empty() {
        return None;
    }
    let mut target = fb.len() + extra_relations;

    let relations_started = Instant::now();
    // Experimental: IC_SUM_TABLE=1 swaps the per-trial sweep for a pair-sum table lookup.
    // Table construction is charged to the relation-collection stage.
    // Default: the table while it wins wall time.  Refined A/B
    // (`examples/ic_fb_crossover_refine.rs`):
    //   16-bit: table@256 (~0.71×), sweep@320 (~1.17×)
    //   18-bit: table@384 (~0.50×), tie@512, sweep@640
    //   20-bit: table through fb=768 (~0.56×)
    // `IC_SUM_TABLE=1` forces the table, `IC_SUM_TABLE=0` the per-trial sweep.
    let use_table = match std::env::var("IC_SUM_TABLE").as_deref() {
        Ok("1") => true,
        Ok("0") => false,
        _ => {
            let n = fb.len();
            let pb = curve.p.bits();
            n <= 256 || (n <= 512 && pb >= 18) || (n <= 768 && pb >= 20)
        }
    };
    if !use_table {
        // the per-trial sweep's square roots and Semaev evaluations are not counted
        count("sweep_collector_work_not_counted", 1);
    }
    let sum_table = if use_table {
        Some(build_compact_pair_sum_table(curve, &fb))
    } else {
        None
    };
    // The sweep's x-index depends only on the factor base, so it is built once
    // per solve (and only when the sweep is used).  `IC_HOIST_SWEEP=0` restores
    // the old per-attempt rebuild for A/A comparison (ignored with
    // `IC_FAST_SWEEP=1`, whose word loop always takes the hoisted index);
    // `IC_HOIST_SWEEP=1` is accepted and is the default.
    // Default: word sweep when p fits u64 (`IC_FAST_SWEEP=0` forces BigUint sweep).
    let word_sweep = if !use_table && std::env::var("IC_FAST_SWEEP").as_deref() != Ok("0") {
        WordSweep::new(curve, &fb)
    } else {
        None
    };
    let fast_sweep = word_sweep.is_some();
    let hoist_sweep = fast_sweep || std::env::var("IC_HOIST_SWEEP").as_deref() != Ok("0");
    let hoisted_index = if !use_table && hoist_sweep {
        Some(factor_base_x_index(&fb))
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
    // Reduced echelon form kept across rank-deficient retries, and how many
    // relations it covers.
    let mut echelon: Option<EchelonState> = None;
    let mut eliminated_rows = 0usize;
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
        let build = |rels: &[Relation]| {
            let mut matrix: Vec<Vec<BigUint>> = Vec::with_capacity(rels.len());
            let mut rhs: Vec<BigUint> = Vec::with_capacity(rels.len());
            for rel in rels {
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
            (matrix, rhs)
        };

        // A retry keeps the reduced echelon form of the earlier relations and
        // reduces only the new ones against it (same reduced form, so the same
        // solution and `determined` flags as eliminating every row again).
        let (mut matrix, mut rhs) =
            build(&relations[echelon.as_ref().map_or(0, |_| eliminated_rows)..]);

        let la_started = Instant::now();
        // The elimination charges its modular operations to the `bitop/v1` tally; the
        // difference across the call is what this solve spent (summed over every retry).
        let tally_before = bitop_ops::snapshot();
        let eliminated: Option<ModNSolution> = 'solve: {
            if let Some(state) = echelon.as_mut() {
                match state.extend(matrix, rhs, &curve.n) {
                    EchelonExtend::Solved(sol) => break 'solve Some(sol),
                    EchelonExtend::Inconsistent => break 'solve None,
                    // Composite modulus only: eliminate every row again, as before.
                    EchelonExtend::NonInvertible => {
                        echelon = None;
                        (matrix, rhs) = build(&relations);
                    }
                }
            }
            gaussian_eliminate_mod_n_particular_pivots(&mut matrix, &mut rhs, &curve.n).map(
                |(sol, pivots)| {
                    if !sol.determined[m] {
                        echelon = Some(EchelonState::from_reduced(&matrix, &rhs, pivots, &curve.n));
                    }
                    sol
                },
            )
        };
        eliminated_rows = relations.len();
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
    // Public recovered `x`: Word/specialised vartime ladder (~13× vs CT on word primes).
    let candidate = g.scalar_mul_vartime(&x, &a_fe);
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
        pair_table_ms: 0.0,
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
        la: None,
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
    ec_index_calculus_dlp_s4_staged_with(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        |_fb| find_one_relation_s4_counted,
        LaEngine::Dense,
    )
}

/// Staged S₄ 3-decomposition driver with an explicit linear-algebra engine.
///
/// The relation family, row policy and verification are those of
/// [`ec_index_calculus_dlp_s4_staged`]; `la_engine` picks between dense
/// Gauss–Jordan and the sparse Markowitz elimination, which decide the same
/// system and return the same scalar.  The report carries the matrix shape,
/// nonzero counts and rank of whichever engine ran.
#[allow(clippy::too_many_arguments)]
pub fn ec_index_calculus_dlp_s4_staged_with_la(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    la_engine: LaEngine,
) -> Option<(BigUint, EcIcStageReport)> {
    ec_index_calculus_dlp_s4_staged_with(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        |_fb| find_one_relation_s4_counted,
        la_engine,
    )
}

/// Staged S₄ 3-decomposition driver on the S₃-anchored sweep
/// ([`find_one_relation_s4_via_s3_counted`]).
///
/// Same relation family, same report shape and same final `[x]G = Q`
/// check as [`ec_index_calculus_dlp_s4_staged`]; only the decomposition
/// oracle differs.  Use it for the per-relation cost lever: the two
/// drivers are interchangeable, so a paired run on one seed measures the
/// oracle alone.
pub fn ec_index_calculus_dlp_s4_s3_staged(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
) -> Option<(BigUint, EcIcStageReport)> {
    ec_index_calculus_dlp_s4_staged_with(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        |_fb| find_one_relation_s4_via_s3_counted,
        LaEngine::Dense,
    )
}

/// Anchored-sweep staged driver with an explicit linear-algebra engine.
#[allow(clippy::too_many_arguments)]
pub fn ec_index_calculus_dlp_s4_s3_staged_with_la(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    la_engine: LaEngine,
) -> Option<(BigUint, EcIcStageReport)> {
    ec_index_calculus_dlp_s4_staged_with(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        |_fb| find_one_relation_s4_via_s3_counted,
        la_engine,
    )
}

/// Staged S₄ 3-decomposition driver on the **pair-sum meet-in-the-middle**
/// oracle ([`find_one_relation_s4_via_mitm_counted`]).
///
/// Same relation family, report shape and `[x]G = Q` check as the other
/// staged S₄ drivers.  The pair-sum table is built once, inside the
/// measured preprocessing stage (`pair_table_ms`), so the relation stage
/// pays `2B` hash probes per draw instead of the `B²` pair sweep.
pub fn ec_index_calculus_dlp_s4_mitm_staged(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
) -> Option<(BigUint, EcIcStageReport)> {
    ec_index_calculus_dlp_s4_staged_with(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        |fb| {
            let table = build_word_pair_sum_table(curve, fb);
            move |curve: &CurveParams,
                  g: &Point,
                  q: &Point,
                  fb: &[FactorBaseEntry],
                  max_trials: usize| {
                find_one_relation_s4_via_mitm_counted(curve, g, q, fb, &table, max_trials)
            }
        },
        LaEngine::Dense,
    )
}

/// Staged S₄ MITM driver with an explicit linear-algebra engine.
///
/// Same oracle and preprocessing accounting as
/// [`ec_index_calculus_dlp_s4_mitm_staged`]; `la_engine` picks between dense
/// Gauss–Jordan and the sparse Markowitz elimination on the same system.
#[allow(clippy::too_many_arguments)]
pub fn ec_index_calculus_dlp_s4_mitm_staged_with_la(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    la_engine: LaEngine,
) -> Option<(BigUint, EcIcStageReport)> {
    ec_index_calculus_dlp_s4_staged_with(
        curve,
        g,
        q,
        fb_size,
        extra_relations,
        max_trials_per_relation,
        max_relation_attempts,
        |fb| {
            let table = build_word_pair_sum_table(curve, fb);
            move |curve: &CurveParams,
                  g: &Point,
                  q: &Point,
                  fb: &[FactorBaseEntry],
                  max_trials: usize| {
                find_one_relation_s4_via_mitm_counted(curve, g, q, fb, &table, max_trials)
            }
        },
        la_engine,
    )
}

/// Shared body of the staged S₄ drivers.  `prepare` builds the oracle from
/// the finished factor base and returns its finder: the identity for the
/// stateless oracles, the pair-sum table closed over for
/// meet-in-the-middle.  Everything else — factor base, relation target,
/// linear algebra, `[x]G = Q` — is identical.
#[allow(clippy::too_many_arguments)]
fn ec_index_calculus_dlp_s4_staged_with<P, F>(
    curve: &CurveParams,
    g: &Point,
    q: &Point,
    fb_size: usize,
    extra_relations: usize,
    max_trials_per_relation: usize,
    max_relation_attempts: usize,
    prepare: P,
    la_engine: LaEngine,
) -> Option<(BigUint, EcIcStageReport)>
where
    P: FnOnce(&[FactorBaseEntry]) -> F,
    F: FnMut(&CurveParams, &Point, &Point, &[FactorBaseEntry], usize) -> Option<(Relation, usize)>,
{
    use std::time::Instant;
    refresh_ic_ct_flag();
    let total_started = Instant::now();

    let fb_started = Instant::now();
    let fb = build_factor_base(curve, fb_size);
    let factor_base_ms = fb_started.elapsed().as_secs_f64() * 1e3;
    if fb.is_empty() {
        return None;
    }
    let target = fb.len() + extra_relations;

    // Oracle preprocessing (pair-sum table for meet-in-the-middle; the
    // identity closure for the stateless oracles) is charged separately
    // from relation collection.
    let prepare_started = Instant::now();
    let mut finder = prepare(&fb);
    let pair_table_ms = prepare_started.elapsed().as_secs_f64() * 1e3;

    let relations_started = Instant::now();
    let mut relations = Vec::with_capacity(target);
    let mut attempts_exhausted = 0usize;
    let mut trials_per_relation: Vec<usize> = Vec::with_capacity(target);
    let mut trials_total = 0usize;
    while relations.len() < target {
        let mut failed_trials = 0usize;
        let found = 'attempt: {
            for _ in 0..max_relation_attempts {
                if let Some(found) = finder(curve, g, q, &fb, max_trials_per_relation) {
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
    let la_started = Instant::now();
    let (solution, la_stats) = solve_relation_matrix(&relations, m + 1, &curve.n, la_engine)?;
    if !solution.determined[m] {
        return None;
    }
    let linear_algebra_ms = la_started.elapsed().as_secs_f64() * 1e3;
    let x = solution.values[m].clone();

    let verify_started = Instant::now();
    let a_fe = curve.a_fe();
    // Public recovered `x`: vartime (matches staged S₃ verify).
    let candidate = g.scalar_mul_vartime(&x, &a_fe);
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
        pair_table_ms,
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
        la: Some(la_stats),
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

    /// The word-sized pair table must contain exactly the sums the BigUint
    /// affine add produces -- same abscissae, same index pairs.  The table
    /// build used to call `Point::add` for every pair (one modular
    /// exponentiation each); the u64 path is only a faster way to the same
    /// `B(B+1)/2` sums.
    #[test]
    fn word_sized_pair_table_matches_biguint_build() {
        use crate::cryptanalysis::research_bench::bench_curves_a_minus_3;

        let (_, curve) = bench_curves_a_minus_3()
            .into_iter()
            .find(|(bits, c)| *bits == 16 && c.name.contains("p256"))
            .expect("16-bit a=-3 fixture");
        let fb = build_factor_base(&curve, 20);
        let a_fe = curve.a_fe();
        let table = build_word_pair_sum_table(&curve, &fb);
        assert!(!table.by_x_u64.is_empty(), "word path took the table");
        assert!(
            table.by_x.is_empty(),
            "BigUint mirror unused on the word path"
        );

        let mut want: Vec<(u64, u32, u32)> = Vec::new();
        for i in 0..fb.len() {
            for j in i..fb.len() {
                if let Point::Affine { x, .. } = fb[i].point.add(&fb[j].point, &a_fe) {
                    want.push((
                        x.value.to_u64_digits().first().copied().expect("one word"),
                        i as u32,
                        j as u32,
                    ));
                }
            }
        }
        let mut got: Vec<(u64, u32, u32)> = table
            .by_x_u64
            .iter()
            .flat_map(|(k, v)| v.iter().map(move |&(i, j)| (*k, i, j)))
            .collect();
        want.sort_unstable();
        got.sort_unstable();
        assert_eq!(got, want, "word table differs from the BigUint build");
        assert_eq!(table.pairs, want.len());
    }

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

    /// The S₃-anchored sweep and the quartic/CZ sweep must decide the
    /// *same* S₄ triple family.  Checked against a brute-force sign census
    /// on a 16-bit deployed-shape curve: every triple either oracle admits
    /// re-adds to `R` under some sign pattern, and the two sets agree.
    #[test]
    fn s3_anchor_sweep_matches_the_quartic_triple_census() {
        use crate::cryptanalysis::research_bench::bench_curves_a_minus_3;
        use std::collections::BTreeSet;

        let Some((_, curve)) = bench_curves_a_minus_3()
            .into_iter()
            .find(|(b, c)| *b == 16 && c.name.contains("p256"))
        else {
            panic!("16-bit a=-3 bench curve missing");
        };
        let a_fe = curve.a_fe();
        let b_fe = curve.fe(curve.b.clone());
        let fb = build_factor_base(&curve, 20);
        assert_eq!(fb.len(), 20);

        // A target that is *known* to have at least one triple.
        let r = fb[0]
            .point
            .add(&fb[1].point, &a_fe)
            .add(&fb[2].point, &a_fe);

        let mut x_to_idx: std::collections::HashMap<BigUint, usize> =
            std::collections::HashMap::new();
        for f in &fb {
            if let Point::Affine { x, .. } = &f.point {
                x_to_idx.insert(x.value.clone(), f.idx);
            }
        }
        let one = BigUint::one();
        let finalises = |i: usize, j: usize, k: usize| {
            try_finalise_s4(
                &r,
                i,
                &fb[i].point,
                j,
                &fb[j].point,
                k,
                &fb[k].point,
                &curve,
                &one,
                &one,
            )
            .is_some()
        };

        // Brute-force census: every triple with a sign pattern summing to R.
        let mut census: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
        for i in 0..fb.len() {
            for j in 0..fb.len() {
                for k in 0..fb.len() {
                    if finalises(i, j, k) {
                        let mut t = [i, j, k];
                        t.sort_unstable();
                        census.insert((t[0], t[1], t[2]));
                    }
                }
            }
        }
        assert!(
            census.contains(&(0, 1, 2)),
            "the planted triple must be in the census"
        );

        // Quartic + Cantor–Zassenhaus enumeration.
        let xr = match &r {
            Point::Affine { x, .. } => x.clone(),
            Point::Infinity => unreachable!(),
        };
        let mut quartic: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
        for i in 0..fb.len() {
            let xi = match &fb[i].point {
                Point::Affine { x, .. } => x.clone(),
                Point::Infinity => continue,
            };
            for j in (i + 1)..fb.len() {
                let xj = match &fb[j].point {
                    Point::Affine { x, .. } => x.clone(),
                    Point::Infinity => continue,
                };
                let q = semaev_s4_in_x4(&xr, &xi, &xj, &a_fe, &b_fe);
                for xk in find_roots_fp_fast(&q, &curve.p) {
                    if let Some(&k) = x_to_idx.get(&xk.value) {
                        if finalises(i, j, k) {
                            let mut t = [i, j, k];
                            t.sort_unstable();
                            quartic.insert((t[0], t[1], t[2]));
                        }
                    }
                }
            }
        }

        // S₃-anchored enumeration (the production oracle).
        let mut anchored: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
        for i in 0..fb.len() {
            for r_prime in [r.add(&fb[i].point.neg(), &a_fe), r.add(&fb[i].point, &a_fe)] {
                let x_prime = match &r_prime {
                    Point::Affine { x, .. } => x.clone(),
                    Point::Infinity => continue,
                };
                for j in 0..fb.len() {
                    let xj = match &fb[j].point {
                        Point::Affine { x, .. } => x,
                        Point::Infinity => continue,
                    };
                    let (qa, qb, qc) = semaev_s3_in_x3(xj, &x_prime, &a_fe, &b_fe);
                    for xk in quadratic_roots_fp(&qa, &qb, &qc, &curve.p) {
                        if let Some(&k) = x_to_idx.get(&xk.value) {
                            if finalises(i, j, k) {
                                let mut t = [i, j, k];
                                t.sort_unstable();
                                anchored.insert((t[0], t[1], t[2]));
                            }
                        }
                    }
                }
            }
        }

        assert_eq!(
            anchored, census,
            "S₃-anchored sweep must find exactly the sign-verified triples"
        );
        // The CZ path may miss a root it fails to split (documented), but
        // every triple it does emit must be a real one, and it must find
        // the planted triple.
        assert!(
            quartic.is_subset(&census),
            "quartic sweep emitted a triple that does not re-add: {:?}",
            quartic.difference(&census).collect::<Vec<_>>()
        );
        assert!(quartic.contains(&(0, 1, 2)));
    }

    /// The two staged drivers are drop-in equivalents: same recovered log,
    /// same relation count, three-entry rows, and the anchored sweep never
    /// costs more trials per relation than the quartic one on this fixture.
    #[test]
    fn s3_staged_driver_matches_the_quartic_driver() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
        let extra_relations = 64;
        let quartic =
            ec_index_calculus_dlp_s4_staged(&curve, &g, &q, 10, extra_relations, 5000, 64)
                .expect("quartic driver solves the tiny curve");
        let anchored =
            ec_index_calculus_dlp_s4_s3_staged(&curve, &g, &q, 10, extra_relations, 5000, 64)
                .expect("anchored driver solves the tiny curve");
        assert_eq!(quartic.0, anchored.0);
        assert_eq!(g.scalar_mul(&anchored.0, &a_fe), q);
        assert_eq!(
            quartic.1.relations_collected,
            anchored.1.relations_collected
        );
        assert!(anchored.1.trials_total >= anchored.1.relations_collected);
    }

    /// Sparse and dense elimination decide the same system: same rank, same
    /// determined columns, same value wherever the column is determined.
    /// Covers full-rank, surplus and rank-deficient (free column) systems.
    #[test]
    fn sparse_and_dense_la_agree_on_random_systems() {
        let n = BigUint::from(1_000_003u32);
        let mut state = 0x2545_F491_4F6C_DD1Du64;
        let mut next = |bound: u64| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % bound
        };
        for &(rows, cols) in &[(12usize, 8usize), (20, 12), (10, 12), (40, 40)] {
            let mut dense: Vec<Vec<BigUint>> = Vec::new();
            let mut sparse: Vec<SparseRow> = Vec::new();
            let mut rhs: Vec<BigUint> = Vec::new();
            // A planted solution; rhs = A·x_true keeps the system consistent
            // even when the row set is rank deficient.
            let x_true: Vec<BigUint> = (0..cols).map(|_| BigUint::from(next(997))).collect();
            for _ in 0..rows {
                let mut entries: Vec<(usize, BigUint)> = Vec::new();
                let deg = 1 + next(3) as usize;
                for _ in 0..deg {
                    entries.push((next(cols as u64) as usize, BigUint::from(1 + next(996))));
                }
                let row = sparse_row_from_entries(&entries, &n);
                let mut acc = BigUint::zero();
                for (c, v) in &row {
                    acc = (&acc + v * &x_true[*c]) % &n;
                }
                rhs.push(acc);
                sparse.push(row);
                let mut dense_row = vec![BigUint::zero(); cols];
                for (c, v) in sparse.last().unwrap() {
                    dense_row[*c] = v.clone();
                }
                dense.push(dense_row);
            }
            let mut dense_rhs = rhs.clone();
            let dense_sol = gaussian_eliminate_mod_n_particular(&mut dense, &mut dense_rhs, &n)
                .expect("consistent");
            let (sparse_sol, stats) =
                sparse_gaussian_eliminate_mod_n(sparse.clone(), rhs.clone(), &n, cols)
                    .expect("consistent");
            assert_eq!(
                dense_sol.rank, sparse_sol.rank,
                "rank mismatch on {rows}x{cols}"
            );
            assert_eq!(stats.rank, sparse_sol.rank);
            assert_eq!(stats.rows, rows);
            assert_eq!(stats.cols, cols);
            assert!(
                stats.nnz_in < rows * cols,
                "sparse input should be sparse: {} of {}",
                stats.nnz_in,
                rows * cols
            );
            for c in 0..cols {
                assert_eq!(
                    dense_sol.determined[c], sparse_sol.determined[c],
                    "determined flag differs at column {c} on {rows}x{cols}"
                );
                if dense_sol.determined[c] {
                    assert_eq!(
                        sparse_sol.values[c], x_true[c],
                        "sparse value is not the planted one at {c} on {rows}x{cols}"
                    );
                    assert_eq!(
                        dense_sol.values[c], x_true[c],
                        "dense value is not the planted one at {c} on {rows}x{cols}"
                    );
                }
            }
            // An inconsistent system must be rejected by both.
            let mut bad_rhs = rhs.clone();
            bad_rhs[0] = (&bad_rhs[0] + BigUint::one()) % &n;
            let mut dense2: Vec<Vec<BigUint>> = sparse
                .iter()
                .map(|row| {
                    let mut v = vec![BigUint::zero(); cols];
                    for (c, x) in row {
                        v[*c] = x.clone();
                    }
                    v
                })
                .collect();
            let mut dense2_rhs = bad_rhs.clone();
            let dense_bad = gaussian_eliminate_mod_n_particular(&mut dense2, &mut dense2_rhs, &n);
            let sparse_bad = sparse_gaussian_eliminate_mod_n(sparse.clone(), bad_rhs, &n, cols);
            // The engines must agree on acceptance, whatever the perturbed
            // system turns out to be; when the surplus rows are independent
            // it is genuinely inconsistent and both must reject it.
            assert_eq!(
                dense_bad.is_some(),
                sparse_bad.is_some(),
                "engines disagree on consistency ({rows}x{cols})"
            );
        }
    }

    /// Both engines, on the *same* relations: identical rank, flags, values
    /// and input nonzero count; the sparse engine keeps the input sparse.
    #[test]
    fn la_engines_agree_on_the_same_relation_set() {
        let n = BigUint::from(65_447u32);
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = |bound: u64| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % bound
        };
        let cols = 12usize;
        let x_true: Vec<BigUint> = (0..cols).map(|_| BigUint::from(next(65_000))).collect();
        let mut relations: Vec<Relation> = Vec::new();
        for _ in 0..40 {
            // Three factor-base columns and the target column, as a 3-summand
            // IC row has.
            let mut entries: Vec<(usize, i64)> = Vec::new();
            for _ in 0..3 {
                let idx = next((cols - 1) as u64) as usize;
                entries.push((idx, if next(2) == 0 { 1 } else { -1 }));
            }
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
            merged.retain(|(_, s)| *s != 0);
            // Choose a and b so the row is consistent with x_true.
            let mut acc = BigUint::zero();
            for &(j, s) in &merged {
                let term = &x_true[j] * BigUint::from(s.unsigned_abs());
                acc = if s > 0 {
                    (&acc + &term) % &n
                } else {
                    (&acc + &n - (&term % &n)) % &n
                };
            }
            let b = BigUint::from(next(65_000));
            let target = &x_true[cols - 1];
            let rhs = (&acc + &n - (&b * target) % &n) % &n;
            relations.push(Relation {
                coef_a: rhs,
                coef_b: b,
                entries: merged,
            });
        }
        let (dense_sol, dense_stats) =
            solve_relation_matrix(&relations, cols, &n, LaEngine::Dense).expect("dense solves");
        let (sparse_sol, sparse_stats) =
            solve_relation_matrix(&relations, cols, &n, LaEngine::Sparse).expect("sparse solves");
        assert_eq!(dense_stats.nnz_in, sparse_stats.nnz_in);
        assert_eq!(dense_stats.rank, sparse_stats.rank);
        assert_eq!(dense_stats.rows, sparse_stats.rows);
        assert_eq!(dense_stats.cols, sparse_stats.cols);
        assert!(sparse_stats.nnz_peak <= dense_stats.nnz_peak);
        for c in 0..cols {
            assert_eq!(dense_sol.determined[c], sparse_sol.determined[c]);
            if dense_sol.determined[c] {
                assert_eq!(dense_sol.values[c], sparse_sol.values[c]);
                assert_eq!(dense_sol.values[c], x_true[c]);
            }
        }
    }

    /// The sparse engine recovers the same scalar as the dense one end to end.
    #[test]
    fn sparse_la_recovers_the_same_log_as_dense() {
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = curve.generator();
        let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
        let extra = 64;
        let dense = ec_index_calculus_dlp_s4_s3_staged_with_la(
            &curve,
            &g,
            &q,
            10,
            extra,
            5000,
            64,
            LaEngine::Dense,
        )
        .expect("dense driver solves the tiny curve");
        let sparse = ec_index_calculus_dlp_s4_s3_staged_with_la(
            &curve,
            &g,
            &q,
            10,
            extra,
            5000,
            64,
            LaEngine::Sparse,
        )
        .expect("sparse driver solves the tiny curve");
        assert_eq!(dense.0, sparse.0);
        assert_eq!(g.scalar_mul(&sparse.0, &a_fe), q);
        let la = sparse.1.la.expect("the S4 driver reports LA stats");
        assert_eq!(la.engine, "sparse_markowitz_gauss_jordan_mod_n");
        assert_eq!(la.rows, sparse.1.relations_collected);
        assert_eq!(la.cols, 11);
        assert_eq!(la.rank, 11);
        assert!(la.nnz_in < la.rows * la.cols);
        let dense_la = dense.1.la.expect("dense LA stats");
        assert_eq!(dense_la.engine, "dense_gauss_jordan_mod_n");
        assert_eq!(dense_la.rank, la.rank);
        assert!(dense_la.nnz_in < dense_la.rows * dense_la.cols);
        // Fill-in on this relation set is far below the dense footprint.
        assert!(la.nnz_peak < la.rows * la.cols);
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
        let compact = build_compact_pair_sum_table(&curve, &fb);
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
        let packed = CompactPairSumTable::pack(11, -1, 7, 1);
        assert_eq!(CompactPairSumTable::unpack(packed), (11, -1, 7, 1));
        let top =
            CompactPairSumTable::pack(PAIR_SUM_TABLE_MAX_FB - 1, -1, PAIR_SUM_TABLE_MAX_FB - 1, -1);
        assert_eq!(
            CompactPairSumTable::unpack(top),
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
        let table = build_compact_pair_sum_table(&curve, &fb);
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

    /// Feeds `rows` to an [`EchelonState`] two at a time after the first
    /// `start`, checking after every step that the incremental solution
    /// equals a full elimination of every row so far, and that the
    /// incremental `bitop/v1` charge (first elimination plus each
    /// extension) is at most the summed charge of re-eliminating.
    fn check_incremental_matches_full(
        rows: &[Vec<BigUint>],
        rhs: &[BigUint],
        start: usize,
        n: &BigUint,
    ) -> usize {
        let full = |k: usize| {
            let (mut mm, mut rr) = (rows[..k].to_vec(), rhs[..k].to_vec());
            let before = bitop_ops::snapshot();
            let out = gaussian_eliminate_mod_n_particular_pivots(&mut mm, &mut rr, n);
            let spent = bitop_ops::snapshot().since(&before).total();
            (out, mm, rr, spent)
        };
        let (first, mm, rr, spent) = full(start);
        let (sol, pivots) = first.expect("consistent start");
        let mut state = EchelonState::from_reduced(&mm, &rr, pivots, n);
        let mut last = sol;
        let (mut inc_mul, mut full_mul) = (
            spent.get(bitop_ops::Kind::ModrMul),
            spent.get(bitop_ops::Kind::ModrMul),
        );
        let mut steps = 0;
        let mut k = start;
        while k < rows.len() {
            let k2 = (k + 2).min(rows.len());
            let before = bitop_ops::snapshot();
            let got = state.extend(rows[k..k2].to_vec(), rhs[k..k2].to_vec(), n);
            inc_mul += bitop_ops::snapshot()
                .since(&before)
                .total()
                .get(bitop_ops::Kind::ModrMul);
            let (want, _, _, spent) = full(k2);
            full_mul += spent.get(bitop_ops::Kind::ModrMul);
            match (got, want) {
                (EchelonExtend::Solved(g), Some((w, _))) => {
                    assert_eq!(g, w, "n = {n}, rows = {k2}");
                    last = g;
                }
                (EchelonExtend::Inconsistent, None) => return steps,
                _ => panic!("n = {n}, rows = {k2}: incremental and full disagree"),
            }
            k = k2;
            steps += 1;
        }
        assert!(last.rank <= rows[0].len());
        assert!(
            inc_mul <= full_mul,
            "n = {n}: incremental {inc_mul} > full {full_mul} multiplications"
        );
        steps
    }

    /// Retries that extend the kept echelon form give exactly the
    /// solution, `determined` flags and rank of eliminating every row
    /// again, on random consistent rank-deficient systems over both
    /// elimination paths, and detect a contradicting new row.
    #[test]
    fn incremental_echelon_matches_full_elimination() {
        let mut s = 0x2545_F491_4F6C_DD1Du64;
        let mut next = move || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        for n in both_path_moduli() {
            for trial in 0..12 {
                let cols = 6 + trial % 5;
                let y: Vec<BigUint> = (0..cols).map(|_| BigUint::from(next()) % &n).collect();
                let mut rows: Vec<Vec<BigUint>> = Vec::new();
                let mut rhs = Vec::new();
                for r in 0..cols + 6 {
                    let mut row = vec![BigUint::zero(); cols];
                    // Sparse rows; every fourth repeats an earlier one scaled,
                    // so ranks lag the row count.
                    if r % 4 == 3 {
                        let f = BigUint::from(next() % 7 + 1);
                        row = rows[r - 2]
                            .iter()
                            .map(|v: &BigUint| (v * &f) % &n)
                            .collect();
                    } else {
                        for _ in 0..3 {
                            let c = (next() % cols as u64) as usize;
                            row[c] = BigUint::from(next()) % &n;
                        }
                    }
                    let b = row
                        .iter()
                        .zip(&y)
                        .fold(BigUint::zero(), |acc, (a, x)| (acc + a * x) % &n);
                    rows.push(row);
                    rhs.push(b);
                }
                let steps = check_incremental_matches_full(&rows, &rhs, cols / 2, &n);
                assert!(steps > 0);
                // A contradicting row: a copy of row 0 with a shifted rhs.
                let mut bad_rows = rows[..cols].to_vec();
                let mut bad_rhs = rhs[..cols].to_vec();
                bad_rows.push(rows[0].clone());
                bad_rhs.push((&rhs[0] + 1u32) % &n);
                check_incremental_matches_full(&bad_rows, &bad_rhs, cols - 1, &n);
            }
        }
    }

    /// The same, on the staged solver's own systems: real relations of
    /// the tiny curve, the matrix built as the staged loop builds it.
    #[test]
    fn incremental_echelon_matches_full_on_curve_relations() {
        use rand::SeedableRng;
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = Point::Affine {
            x: FieldElement::new(curve.gx.clone(), curve.p.clone()),
            y: FieldElement::new(curve.gy.clone(), curve.p.clone()),
        };
        let q = g.scalar_mul(&BigUint::from(57u32), &a_fe);
        for fb_size in [6usize, 12] {
            let fb = build_factor_base(&curve, fb_size);
            let table = build_compact_pair_sum_table(&curve, &fb);
            let mut rng = rand::rngs::StdRng::seed_from_u64(31 + fb_size as u64);
            let m = fb.len();
            let mut rows = Vec::new();
            let mut rhs = Vec::new();
            for _ in 0..m + 8 {
                let (rel, _) =
                    find_one_relation_table_with_rng(&curve, &g, &q, &table, 100_000, &mut rng)
                        .expect("table relation");
                let mut row = vec![BigUint::zero(); m + 1];
                for &(j, mult) in &rel.entries {
                    let val = signed_mod(mult, &curve.n);
                    row[j] = (&row[j] + &val) % &curve.n;
                }
                row[m] = (&curve.n - &(&rel.coef_b % &curve.n)) % &curve.n;
                rows.push(row);
                rhs.push(rel.coef_a.clone() % &curve.n);
            }
            let steps = check_incremental_matches_full(&rows, &rhs, m / 2, &curve.n);
            assert!(steps > 0);
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

    /// The sweep with the x-index built once per factor base (the solver's
    /// default) returns exactly the relations, and trial counts, of the old
    /// per-attempt rebuild (`IC_HOIST_SWEEP=0`) on the same random stream, with
    /// a factor base above the sum-table cutoff (fb > 64 selects the sweep).
    #[test]
    fn hoisted_sweep_index_matches_per_attempt_rebuild() {
        use rand::SeedableRng;
        let curve = tiny_curve();
        let a_fe = curve.a_fe();
        let g = Point::Affine {
            x: FieldElement::new(curve.gx.clone(), curve.p.clone()),
            y: FieldElement::new(curve.gy.clone(), curve.p.clone()),
        };
        let q = g.scalar_mul(&BigUint::from(101u32), &a_fe);
        let fb = build_factor_base(&curve, 80);
        assert!(
            fb.len() > 64,
            "fb {} does not reach the sweep path",
            fb.len()
        );
        let x_to_idx = factor_base_x_index(&fb);
        let mut rng_a = rand::rngs::StdRng::seed_from_u64(2026);
        let mut rng_b = rand::rngs::StdRng::seed_from_u64(2026);
        for _ in 0..(fb.len() + 10) {
            let per_attempt =
                find_one_relation_counted_with_rng(&curve, &g, &q, &fb, 100_000, &mut rng_a)
                    .expect("per-attempt sweep relation");
            let hoisted = find_one_relation_sweep_with_index(
                &curve, &g, &q, &fb, &x_to_idx, None, 100_000, &mut rng_b,
            )
            .expect("hoisted sweep relation");
            assert_eq!(per_attempt.1, hoisted.1, "trial counts differ");
            assert_eq!(per_attempt.0.coef_a, hoisted.0.coef_a);
            assert_eq!(per_attempt.0.coef_b, hoisted.0.coef_b);
            assert_eq!(per_attempt.0.entries, hoisted.0.entries);
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

    /// Round-4: stack [`Poly4Fp`] matches heap [`semaev_s4_in_x4`] bit-for-bit.
    #[test]
    fn poly4fp_matches_heap_s4_on_tiny_curve() {
        let curve = tiny_curve();
        let p = curve.p.to_u64_digits()[0];
        let a = curve
            .a_fe()
            .value
            .to_u64_digits()
            .first()
            .copied()
            .unwrap_or(0);
        let b = curve.b.to_u64_digits().first().copied().unwrap_or(0);
        let a_fe = curve.a_fe();
        let b_fe = curve.fe(curve.b.clone());
        for (x1, x2, x3) in [(1u64, 2, 3), (5, 8, 13), (17, 19, 23)] {
            let heap = semaev_s4_in_x4(
                &FieldElement::new(BigUint::from(x1), curve.p.clone()),
                &FieldElement::new(BigUint::from(x2), curve.p.clone()),
                &FieldElement::new(BigUint::from(x3), curve.p.clone()),
                &a_fe,
                &b_fe,
            );
            let stack = semaev_s4_poly4fp(x1, x2, x3, a, b, p);
            for i in 0..5 {
                let hv = heap[i].value.to_u64_digits().first().copied().unwrap_or(0);
                assert_eq!(hv, stack.c[i], "coef {i} x=({x1},{x2},{x3})");
            }
            // Root sets agree.
            let heap_roots = find_roots_fp_fast(&heap, &curve.p);
            let stack_roots = find_roots_fp_fast(&stack.to_field_elements(), &curve.p);
            let mut h: Vec<_> = heap_roots.iter().map(|r| r.value.clone()).collect();
            let mut s: Vec<_> = stack_roots.iter().map(|r| r.value.clone()).collect();
            h.sort();
            s.sort();
            assert_eq!(h, s);
        }
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

    /// The meet-in-the-middle oracle decides the same relation family as
    /// the S₃-anchored sweep: on one 16-bit a = −3 fixture both staged
    /// drivers recover the same scalar, every relation re-added in the
    /// group by the shared finaliser.
    #[test]
    fn mitm_driver_agrees_with_s3_driver() {
        use crate::cryptanalysis::research_bench::bench_curves_a_minus_3;
        use crate::utils::random::random_scalar;

        let (_, curve) = bench_curves_a_minus_3()
            .into_iter()
            .find(|(bits, _)| *bits == 16)
            .expect("16-bit a=-3 fixture");
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let x_truth = random_scalar(&curve.n);
        let q = g.scalar_mul(&x_truth, &a_fe);

        let s3 = ec_index_calculus_dlp_s4_s3_staged(&curve, &g, &q, 120, 8, 200_000, 4)
            .expect("S3-anchored driver should recover the 16-bit log");
        let mitm = ec_index_calculus_dlp_s4_mitm_staged(&curve, &g, &q, 120, 8, 200_000, 4)
            .expect("MITM driver should recover the 16-bit log");
        assert_eq!(s3.0, mitm.0);
        assert_eq!(g.scalar_mul(&mitm.0, &a_fe), q);
        assert_eq!(mitm.1.relations_collected, 128);
        assert!(mitm.1.pair_table_ms >= 0.0);
    }
}
