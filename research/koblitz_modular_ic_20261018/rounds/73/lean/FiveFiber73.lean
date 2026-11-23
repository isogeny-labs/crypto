import Init.Omega
import Mathlib.Tactic.LinearCombination

namespace FiveFiber73

/-- The six residue degrees found over F32 have total reduced degree 20. -/
theorem reduced_degree_sum : 1 + 1 + 1 + 1 + 2 + 14 = 20 := by
  omega

/-- The R65 discriminant budget and the complete degree profile force the
degree-14 branch to be unramified, conditional on the standard different
lower bound supplying the displayed nonnegative weighted budget. -/
theorem degree_fourteen_unramified
    (e₁ e₂ e₃ e₄ e₅ e₆ : ℕ)
    (h₁ : 1 ≤ e₁) (h₂ : 1 ≤ e₂) (h₃ : 1 ≤ e₃)
    (h₄ : 1 ≤ e₄) (h₅ : 1 ≤ e₅) (h₆ : 1 ≤ e₆)
    (hbudget : (e₁ - 1) + (e₂ - 1) + (e₃ - 1) + (e₄ - 1) +
      2 * (e₅ - 1) + 14 * (e₆ - 1) = 12) :
    e₆ = 1 := by
  omega

/-- In the rational z=1 fiber the two tame degree-two index-seven
branches account for the complete discriminant exponent. -/
theorem rational_tame_discriminant :
    2 * (7 - 1) + 2 * (7 - 1) = 24 := by
  omega

end FiveFiber73
