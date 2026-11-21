import Mathlib.Analysis.Complex.ExponentialBounds
import Mathlib.Tactic

namespace XedniExplicitHeightExclusion

theorem log_two_bounds :
    (2 / 3 : ℝ) < Real.log 2 ∧ Real.log 2 < 7 / 10 := by
  constructor
  · have h := Real.log_two_gt_d9
    linarith
  · have h := Real.log_two_lt_d9
    linarith

theorem target_min_square (n : ℤ) (hmod : n % 43 = 22) : 441 ≤ n ^ 2 := by
  have hsplit : n ≤ -21 ∨ 21 ≤ n := by omega
  rcases hsplit with hneg | hpos
  · nlinarith [sq_nonneg (n + 21)]
  · nlinarith [sq_nonneg (n - 21)]

theorem silverman_formula_lt14
    (hDelta hJinf hB2 hJ : ℝ)
    (hD0 : 0 ≤ hDelta) (hJi0 : 0 ≤ hJinf)
    (hB0 : 0 ≤ hB2) (hJ0 : 0 ≤ hJ)
    (hD : hDelta < 22) (hJi : hJinf < 7)
    (hB : hB2 < 4) (hJb : hJ < 21) :
    max (abs (2 * (-hJ / 24 -
      (hDelta / 12 + hJinf / 12 + hB2 / 2 + Real.log 2 / 2) - 961 / 1000)))
      (abs (2 * ((hDelta / 12 + hJinf / 12 + hB2 / 2 + Real.log 2 / 2) + 107 / 100)))
      < 14 := by
  have hloglo := (log_two_bounds).1
  have hloghi := (log_two_bounds).2
  apply max_lt
  · rw [abs_lt]
    constructor <;> linarith
  · rw [abs_lt]
    constructor <;> linarith

theorem no_lift_under_cap
    (n : ℤ) (hmod : n % 43 = 22) (hQ h65 hR : ℝ)
    (hQnonneg : 0 ≤ hQ)
    (h65quad : h65 = 4225 * hQ)
    (h65lower : 2893 * Real.log 2 ≤ h65 + 14)
    (hRquad : hR = (n : ℝ) ^ 2 * hQ)
    (hRupper : hR ≤ 260 * Real.log 2 + 14) : False := by
  have hsq : (441 : ℝ) ≤ (n : ℝ) ^ 2 := by
    exact_mod_cast target_min_square n hmod
  have hmin : 441 * hQ ≤ hR := by
    rw [hRquad]
    exact mul_le_mul_of_nonneg_right hsq hQnonneg
  have hloglo := (log_two_bounds).1
  have hloghi := (log_two_bounds).2
  rw [h65quad] at h65lower
  linarith

#print axioms log_two_bounds
#print axioms target_min_square
#print axioms silverman_formula_lt14
#print axioms no_lift_under_cap

end XedniExplicitHeightExclusion
