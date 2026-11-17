import Mathlib

/-!
# Model discriminants are not minimal discriminants

For the short Weierstrass equation

`y² = x³ + A*x + B`,

the integral change of model `(x,y) ↦ (u²*x,u³*y)` replaces `(A,B)` by
`(A*u⁴,B*u⁶)` and multiplies the displayed discriminant by `u¹²`.
Consequently the discriminant of an arbitrary integral equation is unbounded
inside one rational isomorphism class.  A Lang-style input must therefore use
the minimal/isomorphism-invariant discriminant, not an arbitrary output model.

This file certifies the polynomial and rational-coordinate facts directly.
The project now constructs a canonical height separately, but does not prove
an induced point-group equivalence or invariance of that height under this
elementary model transformation.
-/

namespace JKSSTModelDiscriminant

/-- The discriminant of a short integral Weierstrass equation. -/
def shortDiscriminant (A B : ℤ) : ℤ :=
  -16 * (4 * A ^ 3 + 27 * B ^ 2)

/-- The short-Weierstrass discriminant has weight twelve. -/
theorem shortDiscriminant_scale (A B u : ℤ) :
    shortDiscriminant (A * u ^ 4) (B * u ^ 6) =
      u ^ 12 * shortDiscriminant A B := by
  simp only [shortDiscriminant]
  ring

/-- Absolute discriminant size scales by `|u|¹²`. -/
theorem natAbs_shortDiscriminant_scale (A B u : ℤ) :
    Int.natAbs (shortDiscriminant (A * u ^ 4) (B * u ^ 6)) =
      Int.natAbs u ^ 12 * Int.natAbs (shortDiscriminant A B) := by
  rw [shortDiscriminant_scale, Int.natAbs_mul, Int.natAbs_pow]

/-- The equation residual, used to state the coordinate change without
depending on a particular elliptic-curve API. -/
def shortResidual (A B x y : ℚ) : ℚ :=
  y ^ 2 - (x ^ 3 + A * x + B)

/-- The coordinate change scales the equation residual by `u⁶`. -/
theorem shortResidual_scale (A B x y u : ℚ) :
    shortResidual (A * u ^ 4) (B * u ^ 6) (u ^ 2 * x) (u ^ 3 * y) =
      u ^ 6 * shortResidual A B x y := by
  simp only [shortResidual]
  ring

/-- For nonzero rational `u`, the coordinate scaling preserves and reflects
membership in the affine short Weierstrass equation. -/
theorem shortResidual_scale_eq_zero_iff
    (A B x y u : ℚ) (hu : u ≠ 0) :
    shortResidual (A * u ^ 4) (B * u ^ 6) (u ^ 2 * x) (u ^ 3 * y) = 0 ↔
      shortResidual A B x y = 0 := by
  rw [shortResidual_scale]
  simp [hu]

/-- If the original discriminant is nonzero, integral rescalings make the
absolute model discriminant exceed every prescribed natural bound. -/
theorem scaled_discriminant_natAbs_unbounded
    (A B : ℤ) (hdisc : shortDiscriminant A B ≠ 0) (N : ℕ) :
    ∃ u : ℕ,
      N < Int.natAbs
        (shortDiscriminant (A * (u : ℤ) ^ 4) (B * (u : ℤ) ^ 6)) := by
  let d := Int.natAbs (shortDiscriminant A B)
  have d_pos : 0 < d := Int.natAbs_pos.mpr hdisc
  let u := N + 1
  have one_le_u : 1 ≤ u := by omega
  have u_le_pow : u ≤ u ^ 12 :=
    le_self_pow₀ one_le_u (by norm_num)
  have pow_le_mul : u ^ 12 ≤ u ^ 12 * d := by
    calc
      u ^ 12 = u ^ 12 * 1 := (mul_one _).symm
      _ ≤ u ^ 12 * d := Nat.mul_le_mul_left (u ^ 12) d_pos
  refine ⟨u, ?_⟩
  rw [natAbs_shortDiscriminant_scale]
  simp only [Int.natAbs_natCast]
  exact (Nat.lt_succ_self N).trans_le (u_le_pow.trans pow_le_mul)

/-- Equivalently, no finite upper bound works for every integral model in
this rescaled family. -/
theorem no_uniform_model_discriminant_natAbs_bound
    (A B : ℤ) (hdisc : shortDiscriminant A B ≠ 0) :
    ¬ ∃ N : ℕ, ∀ u : ℕ,
      Int.natAbs
        (shortDiscriminant (A * (u : ℤ) ^ 4) (B * (u : ℤ) ^ 6)) ≤ N := by
  rintro ⟨N, hN⟩
  obtain ⟨u, hu⟩ := scaled_discriminant_natAbs_unbounded A B hdisc N
  exact (not_lt_of_ge (hN u)) hu

/-- The logarithm of the absolute model discriminant is likewise unbounded.
The logarithm is taken after coercing the exact natural absolute value to
`ℝ`, so no convention at a negative integer is hidden. -/
theorem scaled_logAbs_discriminant_unbounded
    (A B : ℤ) (hdisc : shortDiscriminant A B ≠ 0) (K : ℝ) :
    ∃ u : ℕ,
      K < Real.log
        (Int.natAbs
          (shortDiscriminant (A * (u : ℤ) ^ 4) (B * (u : ℤ) ^ 6)) : ℝ) := by
  obtain ⟨N : ℕ, hN⟩ := exists_nat_gt (Real.exp K)
  obtain ⟨u, hu⟩ := scaled_discriminant_natAbs_unbounded A B hdisc N
  let D : ℕ := Int.natAbs
    (shortDiscriminant (A * (u : ℤ) ^ 4) (B * (u : ℤ) ^ 6))
  have exp_lt_D : Real.exp K < (D : ℝ) := by
    exact hN.trans (by exact_mod_cast hu)
  have D_pos : (0 : ℝ) < (D : ℝ) := (Real.exp_pos K).trans exp_lt_D
  refine ⟨u, ?_⟩
  exact (Real.lt_log_iff_exp_lt D_pos).2 exp_lt_D

end JKSSTModelDiscriminant
