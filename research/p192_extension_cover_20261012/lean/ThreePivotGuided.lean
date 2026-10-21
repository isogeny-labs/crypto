import Mathlib

/-!
The concrete 64-class support and admitted-cell counts are independently
replayed over the P-192 curve. These theorems certify the consequent
arithmetic and the known-scalar logarithm equation for any successful hit.
-/

namespace P192ExtensionCover

theorem three_pivot_64_class_three_sum_capacity :
    64 * 65 * 66 / 6 = 45760 := by
  norm_num

theorem three_pivot_64_class_guided_hit_bound :
    11531 * (2 ^ 176 : ℕ) <
      1569275433846670190958947353987675545133315652035359680560 := by
  norm_num

theorem three_pivot_guided_scalar_relation {G K : Type*}
    [AddCommGroup G] [AddCommGroup K] (log : G →+ K)
    (generator b₀ b₁ b₂ : G) (a : ℕ)
    (hit : b₀ + b₁ + b₂ = a • (-(generator + generator))) :
    log b₀ + log b₁ + log b₂ =
      a • (-(log generator + log generator)) := by
  calc
    log b₀ + log b₁ + log b₂ = log (b₀ + b₁ + b₂) := by simp
    _ = log (a • (-(generator + generator))) := congrArg log hit
    _ = a • (-(log generator + log generator)) := by simp

#print axioms three_pivot_64_class_three_sum_capacity
#print axioms three_pivot_64_class_guided_hit_bound
#print axioms three_pivot_guided_scalar_relation

end P192ExtensionCover
