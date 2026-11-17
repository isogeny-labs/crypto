import JKSSTCanonicalHeight
import JKSSTHeightDiscriminant
import Mathlib.NumberTheory.Height.NumberField

/-!
# Actual elliptic-curve counterexample to a universal height/discriminant bound

This upgrades the earlier numerical abstraction to Mathlib's bundled rational
elliptic-curve point group and the kernel-checked Néron--Tate height constructed
in `JKSSTCanonicalHeight`.

It still targets only the overbroad claim that one fixed discriminant bounds
the heights of *all* rational points/multiples on a positive-rank curve.  It
does not refute a separately specified lift-selection algorithm whose outputs
might satisfy additional restrictions.
-/

namespace JKSSTActualHeightCounterexample

open Height

variable {W : WeierstrassCurve.Affine ℚ} [W.IsElliptic]

/-- A nontorsion rational point has strictly positive Néron--Tate height. -/
theorem neronTateHeight_pos_of_not_torsion
    (P : W.Point) (hP : P ∉ AddCommGroup.torsion W.Point) :
    0 < P.neronTateHeight := by
  have hnonneg := P.neronTateHeight_nonneg
  have hne : P.neronTateHeight ≠ 0 := by
    intro hz
    exact hP ((P.neronTateHeight_eq_zero_iff_mem_torsion).mp hz)
  exact lt_of_le_of_ne hnonneg (Ne.symm hne)

/-- On one fixed rational elliptic curve, no positive constant can bound the
Néron--Tate heights of all multiples of a nontorsion point by one fixed real
discriminant parameter. -/
theorem no_uniform_fixed_discriminant_bound_on_actual_curve
    (P : W.Point) (hP : P ∉ AddCommGroup.torsion W.Point)
    (logAbsDisc : ℝ) :
    ¬ ∃ C₁ : ℝ, 0 < C₁ ∧
      ∀ n : ℕ, C₁ * (n • P).neronTateHeight ≤ logAbsDisc := by
  intro alleged
  apply JKSSTHeightDiscriminant.no_universal_fixed_discriminant_bound
    P.neronTateHeight logAbsDisc
    (neronTateHeight_pos_of_not_torsion P hP)
  obtain ⟨C₁, C₁_pos, bound⟩ := alleged
  refine ⟨C₁, C₁_pos, fun n ↦ ?_⟩
  simpa using bound n

/-- The same obstruction with the right-hand side specialized to the actual
discriminant of the fixed Mathlib Weierstrass model.  This is deliberately a
fixed-model statement: it does not identify this quantity with a global
minimal discriminant. -/
theorem no_uniform_bound_by_actual_model_discriminant
    (P : W.Point) (hP : P ∉ AddCommGroup.torsion W.Point) :
    ¬ ∃ C₁ : ℝ, 0 < C₁ ∧
      ∀ n : ℕ, C₁ * (n • P).neronTateHeight ≤
        Real.log |((W.Δ : ℚ) : ℝ)| :=
  no_uniform_fixed_discriminant_bound_on_actual_curve P hP
    (Real.log |((W.Δ : ℚ) : ℝ)|)

end JKSSTActualHeightCounterexample
