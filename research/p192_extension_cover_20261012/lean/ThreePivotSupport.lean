import Mathlib.Data.Sym.Card
import Mathlib.Tactic.NormNum

/-!
Arithmetic certificate for the independently replayed selected-plus-three-
residual-pivot class union. The structural unordered-support map theorem is
`P192ExtensionCover.unordered_policy_support_card_le_multichoose` in
`UnorderedSupport.lean`; the concrete union cardinality is a finite-field
input checked by the three-pivot sorted-list replay.
-/

namespace P192ExtensionCover

theorem three_pivot_three_sum_capacity :
    Nat.multichoose 39984 3 = 10654671159440 := by
  rw [Nat.multichoose_eq, Nat.choose_eq_descFactorial_div_factorial]
  norm_num [Nat.descFactorial, Nat.factorial]

theorem three_pivot_accepted_hit_bound :
    10654671159440 * (2 ^ 146 : ℕ) <
      1569275433846670190958947353987675545133315652035359680560 := by
  norm_num

end P192ExtensionCover

#print axioms P192ExtensionCover.three_pivot_three_sum_capacity
#print axioms P192ExtensionCover.three_pivot_accepted_hit_bound
