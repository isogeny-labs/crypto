import Mathlib.RingTheory.Valuation.Basic
import Mathlib.Tactic.FieldSimp

/-!
# Fraction-field pole of the common 2+2 interpolant

This is the rational-function step after a projective infinity stalk and its
valuation have been supplied. It does not construct that stalk or identify the
source residual divisor.
-/

set_option autoImplicit false

namespace Common22FractionPole

variable {K Γ : Type*} [Field K] [LinearOrderedCommGroupWithZero Γ]

/-- If the infinity chart has `w²=z*u` with a valuation-unit `u`, and the
cleared interpolant satisfies `z³*g=G`, then a unit `G` makes the pole of `g`
equal to six copies of the pole of `w`. -/
theorem unit_chart_pole (v : Valuation K Γ) (w z u g G : K)
    (hw : w ≠ 0) (hchart : w ^ 2 = z * u) (hu : v u = 1)
    (hg : z ^ 3 * g = G) (hG : v G = 1) :
    v g = (v w)⁻¹ ^ 6 := by
  have hvw : v w ≠ 0 := (Valuation.ne_zero_iff v).mpr hw
  have hvz : v z = (v w) ^ 2 := by
    have h := congrArg v hchart
    simp only [map_pow, Valuation.map_mul, hu, mul_one] at h
    exact h.symm
  have hvg : (v z) ^ 3 * v g = 1 := by
    have h := congrArg v hg
    simpa only [map_pow, Valuation.map_mul, hG] using h
  rw [hvz, ← pow_mul] at hvg
  have hvg' : v g * (v w) ^ 6 = 1 := by simpa [mul_comm] using hvg
  simpa only [inv_pow] using
    (mul_eq_one_iff_eq_inv₀ (pow_ne_zero 6 hvw)).mp hvg'

/-- In the degree-drop branch `G` has one uniformizer factor. The cleared
interpolant therefore has pole order five. -/
theorem linear_chart_pole (v : Valuation K Γ) (w z u g G : K)
    (hw : w ≠ 0) (hchart : w ^ 2 = z * u) (hu : v u = 1)
    (hg : z ^ 3 * g = G) (hG : v G = v w) :
    v g = (v w)⁻¹ ^ 5 := by
  have hvw : v w ≠ 0 := (Valuation.ne_zero_iff v).mpr hw
  have hvz : v z = (v w) ^ 2 := by
    have h := congrArg v hchart
    simp only [map_pow, Valuation.map_mul, hu, mul_one] at h
    exact h.symm
  have hvg : (v z) ^ 3 * v g = v w := by
    have h := congrArg v hg
    simpa only [map_pow, Valuation.map_mul, hG] using h
  rw [hvz, ← pow_mul] at hvg
  have : (v w) ^ 6 * ((v w)⁻¹ ^ 5) = v w := by
    field_simp [hvw]
  have hvg' : (v w) ^ 6 * v g = v w := by simpa using hvg
  exact (mul_left_cancel₀ (pow_ne_zero 6 hvw)) (hvg'.trans this.symm)

end Common22FractionPole

#print axioms Common22FractionPole.unit_chart_pole
#print axioms Common22FractionPole.linear_chart_pole
