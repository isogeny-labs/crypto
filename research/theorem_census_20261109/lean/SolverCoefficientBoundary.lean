import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic.NormNum

/-! Arithmetic audit of the archived msolve c=2 preflight.
The source/log interpretation is documented separately. These theorems do
not claim a formal C, floating-point, libm or full parametrization semantics. -/
namespace SolverCoefficientAudit
set_option exponentiation.threshold 512

theorem advertised_power_outside_int32 :
    (2147483647 : ℕ) < 2^344 := by
  have h : 2^31 ≤ (2:ℕ)^344 := Nat.pow_le_pow_right (by decide) (by decide)
  exact lt_of_lt_of_le (by norm_num) h

theorem observed_coefficient_wrong_in_five :
    (2 : ZMod 5)^344 = 1 ∧ (-2147483648 : ZMod 5) = 2 ∧
      (2 : ZMod 5)^344 ≠ (-2147483648 : ZMod 5) := by
  decide

theorem observed_coefficient_wrong_in_five_at_first_overflow :
    (2147483647 : ℕ) < 2^31 ∧
      (2 : ZMod 5)^31 ≠ (-2147483648 : ZMod 5) := by
  decide

end SolverCoefficientAudit
