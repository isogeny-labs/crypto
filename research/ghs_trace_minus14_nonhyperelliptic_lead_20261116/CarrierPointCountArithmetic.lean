import Init

/-! Arithmetic consequences of the externally enumerated point counts.
    Lean does not count curve points or prove the Weil functional equation. -/
namespace TraceMinus14CarrierPointCountArithmetic

theorem first_four_counts :
    (3 : Int)^1 + 1 = 4 ∧ (3 : Int)^2 + 1 = 10 ∧
    (3 : Int)^3 + 1 = 28 ∧ (3 : Int)^4 + 1 = 82 := by decide

theorem fifth_power_sum : (3 : Int)^5 + 1 - 314 = -70 := by decide

theorem fifth_newton_coefficient : (-70 : Int) + 5*14 = 0 := by decide

theorem functional_tail : (3 : Int)^5 = 243 := by decide

theorem extension_point_count : (3 : Int)^5 + 1 - (-70) = 314 := by decide

end TraceMinus14CarrierPointCountArithmetic
