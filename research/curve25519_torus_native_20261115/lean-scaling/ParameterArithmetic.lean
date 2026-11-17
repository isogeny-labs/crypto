/- Actual-parameter integer checks only; no point count or solver bound. -/
set_option autoImplicit false
namespace Curve25519ScalingParameters
def scalarOrder : Nat := 2^252 + 27742317777372353535851937790883648493
def torusOrder : Nat := 60824497 * 213156431
def classLowerBound : Nat := (torusOrder - 1) / 2

theorem sixth_root_lower : (2^42)^6 ≤ scalarOrder := by decide
theorem sixth_root_upper : scalarOrder < (2^42 + 1)^6 := by decide
theorem current_base_above_sixth_root : scalarOrder < classLowerBound^6 := by decide
theorem current_budget_square_above_order : scalarOrder < classLowerBound^5 := by decide

#print axioms sixth_root_lower
#print axioms sixth_root_upper
#print axioms current_base_above_sixth_root
#print axioms current_budget_square_above_order
end Curve25519ScalingParameters
