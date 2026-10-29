import Init

/-! Finite enumeration arithmetic only. The map screen is checked by Sage. -/

namespace GhsQ8TailDegreeThree

theorem pairs : 27 * 27 + 18 * 15 = 999 := by decide
theorem fourSupport : 999 * 24 = 23976 := by decide
theorem fiveSupportPerPair : 4 * 2 * (512 - 4 + 1) = 4072 := by decide
theorem fiveSupport : 999 * 4072 = 4067928 := by decide
theorem totalFunctions : 23976 + 4067928 = 4091904 := by decide
theorem fourClassModels : 15 + 9 + 27 + 15 = 66 := by decide
theorem fourClassPairs : 432 + 999 = 1431 := by decide
theorem fourClassFunctions : 1769472 + 4091904 = 5861376 := by decide
theorem subgroup263 : Nat.Coprime 3 263 := by decide
theorem subgroup271 : Nat.Coprime 3 271 := by decide

end GhsQ8TailDegreeThree
