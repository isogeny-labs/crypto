import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.Ring

/- Algebraic steps of the symbolic graph reducer. These are field identities,
   not a proof of the Rust implementation or the entire forest invariant. -/
namespace PencilSymbolicGraph

variable {K : Type*} [Field K]

theorem pivot_equivalence (a b c x y : K) (ha : a ≠ 0) :
    a * x + b * y + c = 0 ↔ x = -(b * y + c) / a := by
  rw [eq_div_iff ha]
  constructor <;> intro h <;> linear_combination h

theorem substitution (a b c s t u v r q : K) :
    a * (s * r + u) + b * (t * q + v) + c =
      (a * s) * r + (b * t) * q + (a * u + b * v + c) := by
  ring

theorem balanced_cycle (a b c r : K) (h : a + b = 0) :
    a * r + b * r + c = 0 ↔ c = 0 := by
  have he : a * r + b * r = 0 := by
    calc
      a * r + b * r = (a + b) * r := by ring
      _ = 0 := by rw [h, zero_mul]
  rw [he, zero_add]

theorem unbalanced_cycle (a b c r : K) (h : a + b ≠ 0) :
    a * r + b * r + c = 0 ↔ r = -c / (a + b) := by
  have he : a * r + b * r + c = (a + b) * r + 0 * (0 : K) + c := by ring
  rw [he, pivot_equivalence (a + b) 0 c r 0 h]
  simp

end PencilSymbolicGraph

#print axioms PencilSymbolicGraph.pivot_equivalence
#print axioms PencilSymbolicGraph.substitution
#print axioms PencilSymbolicGraph.balanced_cycle
#print axioms PencilSymbolicGraph.unbalanced_cycle
