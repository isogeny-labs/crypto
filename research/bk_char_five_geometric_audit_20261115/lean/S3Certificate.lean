module
public import S3Completion

@[expose] public section

/-!
# Polynomial certificate form of Semaev's third summation polynomial

This file packages the scalar-valued `SemaevS3.S3` formula as a univariate
polynomial in its third coordinate.  It proves the construction's exact
evaluation, quadratic degree bound, generic nonvanishing, and the forward
zero-sum implication over the actual Mathlib elliptic-curve point group.

This is the base certificate for a future resultant recursion.  It is not a
construction of the general even-arity summation polynomial.
-/

open Polynomial

namespace SemaevS3

variable {K : Type*} [Field K]

/-- Semaev's third summation polynomial, univariate in its third input. -/
noncomputable def S3Polynomial (a b x₁ x₂ : K) : K[X] :=
  C ((x₁ - x₂) ^ 2) * X ^ 2
    - C (2 * ((x₁ + x₂) * (x₁ * x₂ + a) + 2 * b)) * X
    + C ((x₁ * x₂ - a) ^ 2 - 4 * b * (x₁ + x₂))

/-- Evaluating the polynomial package recovers the scalar `S3` formula. -/
theorem eval_S3Polynomial (a b x₁ x₂ x₃ : K) :
    (S3Polynomial a b x₁ x₂).eval x₃ = S3 a b x₁ x₂ x₃ := by
  simp [S3Polynomial, S3]

/-- The univariate `S3` package has degree at most two. -/
theorem natDegree_S3Polynomial_le_two (a b x₁ x₂ : K) :
    (S3Polynomial a b x₁ x₂).natDegree ≤ 2 := by
  unfold S3Polynomial
  calc
    (C ((x₁ - x₂) ^ 2) * X ^ 2
        - C (2 * ((x₁ + x₂) * (x₁ * x₂ + a) + 2 * b)) * X
        + C ((x₁ * x₂ - a) ^ 2 - 4 * b * (x₁ + x₂))).natDegree ≤
        max
          (C ((x₁ - x₂) ^ 2) * X ^ 2
            - C (2 * ((x₁ + x₂) * (x₁ * x₂ + a) + 2 * b)) * X).natDegree
          (C ((x₁ * x₂ - a) ^ 2 - 4 * b * (x₁ + x₂))).natDegree :=
      Polynomial.natDegree_add_le _ _
    _ ≤ 2 := by
      apply max_le
      · calc
          (C ((x₁ - x₂) ^ 2) * X ^ 2
              - C (2 * ((x₁ + x₂) * (x₁ * x₂ + a) + 2 * b)) * X).natDegree ≤
              max (C ((x₁ - x₂) ^ 2) * X ^ 2).natDegree
                (C (2 * ((x₁ + x₂) * (x₁ * x₂ + a) + 2 * b)) * X).natDegree :=
            Polynomial.natDegree_sub_le _ _
          _ ≤ 2 := by
            apply max_le
            · exact Polynomial.natDegree_C_mul_X_pow_le _ 2
            · exact le_trans
                (by simpa only [pow_one] using
                  (Polynomial.natDegree_C_mul_X_pow_le
                    (2 * ((x₁ + x₂) * (x₁ * x₂ + a) + 2 * b)) 1))
                (by omega)
      · exact le_trans
          (by simpa only [pow_zero, mul_one] using
            (Polynomial.natDegree_C_mul_X_pow_le
              ((x₁ * x₂ - a) ^ 2 - 4 * b * (x₁ + x₂)) 0))
          (by omega)

/-- Distinct first two inputs give a nonzero quadratic coefficient, hence a
nonzero specialized polynomial. -/
theorem S3Polynomial_ne_zero_of_ne (a b : K) {x₁ x₂ : K} (h : x₁ ≠ x₂) :
    S3Polynomial a b x₁ x₂ ≠ 0 := by
  intro hzero
  have hcoeff := congrArg (fun p : K[X] => p.coeff 2) hzero
  have hcoeffFormula :
      (S3Polynomial a b x₁ x₂).coeff 2 = (x₁ - x₂) ^ 2 := by
    unfold S3Polynomial
    rw [Polynomial.coeff_add, Polynomial.coeff_sub,
      Polynomial.coeff_C_mul_X_pow, Polynomial.coeff_C_mul_X,
      Polynomial.coeff_C]
    simp
  have hlead : (x₁ - x₂) ^ 2 = 0 := by
    rw [hcoeffFormula, Polynomial.coeff_zero] at hcoeff
    exact hcoeff
  exact (pow_ne_zero 2 (sub_ne_zero.mpr h)) hlead

/-- A genuine affine three-point zero sum forces the corresponding `S3`
polynomial to vanish at the third x-coordinate. -/
theorem S3Polynomial_vanishes_of_affine_zero_sum [DecidableEq K]
    (W : WeierstrassCurve.Affine K)
    (ha₁ : W.a₁ = 0) (ha₂ : W.a₂ = 0) (ha₃ : W.a₃ = 0)
    (h2 : (2 : K) ≠ 0)
    {x₁ y₁ x₂ y₂ x₃ y₃ : K}
    (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (h₃ : W.Nonsingular x₃ y₃)
    (hsum : WeierstrassCurve.Affine.Point.some x₁ y₁ h₁ +
      WeierstrassCurve.Affine.Point.some x₂ y₂ h₂ +
      WeierstrassCurve.Affine.Point.some x₃ y₃ h₃ = 0) :
    (S3Polynomial W.a₄ W.a₆ x₁ x₂).eval x₃ = 0 := by
  rw [eval_S3Polynomial]
  apply (TheoremAudit.S3_pointX_iff W ha₁ ha₂ ha₃ h2 h₁ h₂ x₃).mpr
  left
  exact (TheoremAudit.zero_sum_pointX_iff W
    (WeierstrassCurve.Affine.Point.some x₁ y₁ h₁ +
      WeierstrassCurve.Affine.Point.some x₂ y₂ h₂) x₃).mp
    ⟨WeierstrassCurve.Affine.Point.some x₃ y₃ h₃, rfl, hsum⟩

end SemaevS3
