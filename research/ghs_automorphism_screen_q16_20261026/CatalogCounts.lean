import Init

/-!
Finite arithmetic from the archived GF(16) genus-three model census.
The model equations, point counts, branch checks, and automorphism
classification are replayed in Sage and proved in the accompanying report;
this Lean file checks only the integer identities used in the bound.
-/

namespace GhsQ16QuotientCounts

theorem all_models_partition : 10664 = 3516 + 7148 := by decide
theorem sparse_partition : 1332 = 456 + 876 := by decide
theorem outside_partition : 7148 = 876 + 6272 := by decide
theorem direct_sparse_partition : 456 = 104 + 352 := by decide
theorem outside_sparse_gamma_zero : 876 = 876 + 0 := by decide
theorem direct_control_lifts : 3 < 12 := by decide
theorem degree_three_four_support_count : 4 * 24 = 96 := by decide
theorem degree_three_five_support_count :
    4 * 4 * 2 * (4096 - 4 + 1) = 130976 := by decide
theorem degree_three_total_count : 96 + 130976 = 131072 := by decide

end GhsQ16QuotientCounts
