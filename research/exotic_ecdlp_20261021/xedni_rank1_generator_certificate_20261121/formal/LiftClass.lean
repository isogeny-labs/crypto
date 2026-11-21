import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! The integer congruence and height step for a primitive rank-one lift. -/

namespace XedniRankOneLiftClass

theorem reduction_preserves_multiple {A B : Type*}
    [AddCommGroup A] [AddCommGroup B]
    (red : A →+ B) (Q : A) (n : ℤ) :
    red (n • Q) = n • red Q := by simp

theorem target22_min_abs (n : ℤ) (h : n % 43 = 22) :
    21 ≤ |n| := by
  have hsplit : n ≤ -21 ∨ 21 ≤ n := by omega
  rcases hsplit with hneg | hpos
  · rw [abs_of_nonpos (by omega)]
    omega
  · rw [abs_of_nonneg (by omega)]
    omega

theorem target22_min_square (n : ℤ) (h : n % 43 = 22) :
    (441 : ℤ) ≤ n^2 := by
  have hsplit : n ≤ -21 ∨ 21 ≤ n := by omega
  rcases hsplit with hneg | hpos
  · nlinarith [sq_nonneg (n + 21)]
  · nlinarith [sq_nonneg (n - 21)]

theorem target22_height_lower_bound (n : ℤ) (hmod : n % 43 = 22)
    (hQ : ℝ) (hnonneg : 0 ≤ hQ) :
    (441 : ℝ) * hQ ≤ (n : ℝ)^2 * hQ := by
  have hsq : (441 : ℝ) ≤ (n : ℝ)^2 := by
    exact_mod_cast target22_min_square n hmod
  exact mul_le_mul_of_nonneg_right hsq hnonneg

theorem minus21_is_target_class : (-21 : ℤ) % 43 = 22 := by norm_num

#print axioms reduction_preserves_multiple
#print axioms target22_min_abs
#print axioms target22_min_square
#print axioms target22_height_lower_bound
#print axioms minus21_is_target_class

end XedniRankOneLiftClass
