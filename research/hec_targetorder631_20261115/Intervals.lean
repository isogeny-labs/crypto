import Mathlib
namespace TargetOrder
/-- Conditional on the Weil interval and a prime-order witness, cardinality is unique. -/
theorem source_sign_unique (n : ℕ) (hlo : 230993922 ≤ n) (hhi : n ≤ 243613922)
    (hd : 59296481 ∣ n) : n = 237185924 := by
  rcases hd with ⟨k, rfl⟩
  omega
/-- The opposite-sign interval excludes any group with this prime divisor. -/
theorem opposite_excludes (n : ℕ) (hlo : 258865262 ≤ n) (hhi : n ≤ 271485262) :
    ¬ 59296481 ∣ n := by
  rintro ⟨k, hk⟩
  omega
end TargetOrder
#print axioms TargetOrder.source_sign_unique
#print axioms TargetOrder.opposite_excludes
