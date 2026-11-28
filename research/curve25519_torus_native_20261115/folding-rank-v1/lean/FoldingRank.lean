import Lean

namespace FoldingRank

/- Conditional integer gate only: no native rank, geometry or decoder theorem. -/
theorem root_budget_degree_bound
    (D a L m : Nat) (hL : L ≤ m) (hbudget : a * m ≤ 6)
    (hroot : D + 4 < a * L) : D ≤ 1 := by
  have h : D + 4 < 6 :=
    Nat.lt_of_lt_of_le hroot (Nat.le_trans (Nat.mul_le_mul_left a hL) hbudget)
  have hs : D + 5 ≤ 1 + 5 := h
  exact Nat.le_of_add_le_add_right hs

#print axioms root_budget_degree_bound

end FoldingRank
