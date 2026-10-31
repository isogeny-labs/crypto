import Init

/-! Arithmetic checks only; the curve geometry and field search are in REPORT.md. -/

namespace GhsQ4DegreeThree

theorem pairs : 9 * 4 + 6 * 4 = 60 := by decide
theorem four_support : 60 * 24 = 1440 := by decide
theorem five_support_per_pair : 4 * 2 * (64 - 4 + 1) = 488 := by decide
theorem five_support : 60 * 488 = 29280 := by decide
theorem total_functions : 1440 + 29280 = 30720 := by decide
theorem subgroup_29 : Nat.Coprime 3 29 := by decide
theorem subgroup_31 : Nat.Coprime 3 31 := by decide

end GhsQ4DegreeThree
