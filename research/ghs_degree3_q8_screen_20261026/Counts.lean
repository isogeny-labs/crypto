import Init

/-! Arithmetic checks only. The finite-field and curve-map certificates are checked by Sage. -/

namespace GhsQ8DegreeThree

theorem pairs : 18 * 15 + 18 * 9 = 432 := by decide
theorem fourSupport : 432 * 24 = 10368 := by decide
theorem fiveSupportPerPair : 4 * 2 * (512 - 4 + 1) = 4072 := by decide
theorem fiveSupport : 432 * 4072 = 1759104 := by decide
theorem totalFunctions : 10368 + 1759104 = 1769472 := by decide
theorem subgroup251 : Nat.Coprime 3 251 := by decide
theorem subgroup257 : Nat.Coprime 3 257 := by decide

end GhsQ8DegreeThree
