import HeightBridge
import Mathlib.RingTheory.Polynomial.RationalRoot

namespace XedniDuplicationTorsionBridge

open Polynomial Height XedniDuplicationPointBridge XedniDuplicationHeightBridge

private theorem no_integer_cubic_root (z : ℤ) :
    z ^ 3 - 47 * z ^ 2 + 751 * z - 3997 ≠ 0 := by
  intro hz
  have hz3 : (z : ZMod 3) ^ 3 - 47 * (z : ZMod 3) ^ 2 +
      751 * (z : ZMod 3) - 3997 = 0 := by
    simpa only [Int.cast_sub, Int.cast_add, Int.cast_mul, Int.cast_pow,
      Int.cast_ofNat, Int.cast_zero] using congrArg (fun v : ℤ => (v : ZMod 3)) hz
  have hnone : ∀ w : ZMod 3, w ^ 3 - 47 * w ^ 2 + 751 * w - 3997 ≠ 0 := by
    intro w
    fin_cases w <;> decide
  exact hnone _ hz3

private noncomputable def cubic : ℤ[X] :=
  X ^ 3 + C (-47) * X ^ 2 + C 751 * X + C (-3997)

private theorem cubic_monic : cubic.Monic := by
  unfold cubic
  monicity!

theorem no_rational_cubic_root (x : ℚ) :
    x ^ 3 - 47 * x ^ 2 + 751 * x - 3997 ≠ 0 := by
  intro hx
  have hr : aeval x cubic = 0 := by
    simp [cubic, Polynomial.aeval_def]
    nlinarith [hx]
  obtain ⟨z, hz, _⟩ := exists_integer_of_is_root_of_monic cubic_monic hr
  have hz' : z ^ 3 - 47 * z ^ 2 + 751 * z - 3997 = 0 := by
    have hzr : (z : ℚ) ^ 3 - 47 * (z : ℚ) ^ 2 + 751 * (z : ℚ) - 3997 = 0 := by
      rw [hz] at hx
      simpa using hx
    exact_mod_cast hzr
  exact no_integer_cubic_root z hz'

theorem no_affine_two_torsion {x y : ℚ} (h : FixedCurve.Nonsingular x y) :
    y ≠ FixedCurve.negY x y := by
  intro heq
  have hy : y = 0 := by
    simp [FixedCurve, WeierstrassCurve.Affine.negY] at heq
    linarith
  have hcurve := h.1
  rw [WeierstrassCurve.Affine.equation_iff] at hcurve
  have hx : x ^ 3 - 47 * x ^ 2 + 751 * x - 3997 = 0 := by
    norm_num [FixedCurve, hy] at hcurve
    linarith
  exact no_rational_cubic_root x hx

theorem all_point_abs_height_defect (P : FixedCurve.Point) :
    |(P + P).naiveHeight - 4 * P.naiveHeight| ≤ 53 * Real.log 2 := by
  cases P with
  | zero =>
      simp only [← WeierstrassCurve.Affine.Point.zero_def, zero_add]
      norm_num [WeierstrassCurve.Affine.Point.naiveHeight_eq_logHeight₁,
        WeierstrassCurve.Affine.Point.xRep, Rat.logHeight₁_eq_log_max]
      exact Real.log_nonneg (by norm_num)
  | some x y h =>
      have hn := no_affine_two_torsion h
      simpa [WeierstrassCurve.Affine.Point.naiveHeight_eq_logHeight₁] using
        point_affine_abs_height_defect h hn

#print axioms no_integer_cubic_root
#print axioms no_rational_cubic_root
#print axioms no_affine_two_torsion
#print axioms all_point_abs_height_defect

end XedniDuplicationTorsionBridge
