import Mathlib.Analysis.Complex.ExponentialBounds
import Mathlib.Tactic

namespace XedniMinimumLiftHeight

theorem target_cases (n : ℤ) (hmod : n % 43 = 22) :
    n = -21 ∨ n = 22 ∨ n ≤ -64 ∨ 65 ≤ n := by
  omega

theorem tail_square (n : ℤ) (hmod : n % 43 = 22)
    (hne1 : n ≠ -21) (hne2 : n ≠ 22) : 4096 ≤ n ^ 2 := by
  rcases target_cases n hmod with h | h | h | h
  · exact (hne1 h).elim
  · exact (hne2 h).elim
  · nlinarith [sq_nonneg (n + 64)]
  · nlinarith [sq_nonneg (n - 65)]

theorem log_two_bounds :
    (2 / 3 : ℝ) < Real.log 2 ∧ Real.log 2 < 7 / 10 := by
  constructor
  · have h := Real.log_two_gt_d9
    linarith
  · have h := Real.log_two_lt_d9
    linarith

theorem no_tail_under_cap
    (n : ℤ) (hmod : n % 43 = 22)
    (hne1 : n ≠ -21) (hne2 : n ≠ 22)
    (hQ h65 hR : ℝ) (hQnonneg : 0 ≤ hQ)
    (h65quad : h65 = 4225 * hQ)
    (h65lower : 2893 * Real.log 2 ≤ h65 + 14)
    (hRquad : hR = (n : ℝ) ^ 2 * hQ)
    (hRupper : hR ≤ 2600 * Real.log 2 + 14) : False := by
  have hsq : (4096 : ℝ) ≤ (n : ℝ) ^ 2 := by
    exact_mod_cast tail_square n hmod hne1 hne2
  have hmin : 4096 * hQ ≤ hR := by
    rw [hRquad]
    exact mul_le_mul_of_nonneg_right hsq hQnonneg
  have hloglo := (log_two_bounds).1
  have hloghi := (log_two_bounds).2
  rw [h65quad] at h65lower
  linarith

theorem unique_minimizer
    (H : ℤ → ℕ)
    (hminus : H (-21) < 2 ^ 306)
    (h22 : 2 ^ 332 ≤ H 22)
    (htail : ∀ n : ℤ, n % 43 = 22 → n ≠ -21 → n ≠ 22 → 2 ^ 1300 < H n)
    (n : ℤ) (hmod : n % 43 = 22) (hne : n ≠ -21) :
    H (-21) < H n := by
  rcases target_cases n hmod with h | h | h | h
  · exact (hne h).elim
  · subst n
    have hpow : (2 : ℕ) ^ 306 < 2 ^ 332 :=
      pow_lt_pow_right₀ (by norm_num) (by omega)
    omega
  · have hne22 : n ≠ 22 := by omega
    have htailn := htail n hmod hne hne22
    have hpow : (2 : ℕ) ^ 306 < 2 ^ 1300 :=
      pow_lt_pow_right₀ (by norm_num) (by omega)
    omega
  · have hne22 : n ≠ 22 := by omega
    have htailn := htail n hmod hne hne22
    have hpow : (2 : ℕ) ^ 306 < 2 ^ 1300 :=
      pow_lt_pow_right₀ (by norm_num) (by omega)
    omega

#print axioms target_cases
#print axioms tail_square
#print axioms log_two_bounds
#print axioms no_tail_under_cap
#print axioms unique_minimizer

end XedniMinimumLiftHeight
