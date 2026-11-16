import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Algebraic accounting for the genus2 balanced prototype. The geometry,
valuations and canonical-representative theorem are inputs from Galbraith
ch10, not constructed here. -/
set_option autoImplicit false
namespace BalancedDivisors

theorem effective_composition_degree (m₁ m₂ n₁ n₂ r : Int) :
    (m₁+m₂-2*r) + (n₁+n₂+r) +
      (2-m₁-n₁+2-m₂-n₂+r) = 4 := by grind

theorem inverse_infinity_involution (m n : Int) :
    2-m-(2-m-n) = n := by grind

theorem inverse_infinity_range (m n : Int)
    (hm : 0 ≤ m) (hn : 0 ≤ n) (hbound : n ≤ 2-m) :
    0 ≤ 2-m-n ∧ 2-m-n ≤ 2-m := by grind

theorem positive_branch_reduction (n m r : Int) :
    n - (3-(m+r)) - r = n+m-3 := by grind

theorem negative_branch_reduction (n r : Int) :
    n - (-3) - r = n+3-r := by grind

open Lean.Grind
theorem infinity_polynomial_square {K : Type} [CommRing K]
    (c₀ c₁ c₂ x : K) :
    (x^3+c₂*x^2+c₁*x+c₀)^2 =
      x^6 + 2*c₂*x^5 + (c₂^2+2*c₁)*x^4 + (2*c₀+2*c₂*c₁)*x^3 +
      (c₁^2+2*c₂*c₀)*x^2 + 2*c₁*c₀*x + c₀^2 := by grind

end BalancedDivisors
#print axioms BalancedDivisors.effective_composition_degree
#print axioms BalancedDivisors.inverse_infinity_involution
#print axioms BalancedDivisors.inverse_infinity_range
#print axioms BalancedDivisors.positive_branch_reduction
#print axioms BalancedDivisors.negative_branch_reduction
#print axioms BalancedDivisors.infinity_polynomial_square
