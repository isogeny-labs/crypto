import Init

/-! Arithmetic only. Sage certifies the exact finite-field objects and counts. -/

namespace GhsQ8NewQuarticClasses

theorem normalizedOrdinary : 2 * (512 - 1) = 1022 := by decide
theorem directB : 2 * (64 - 1) = 126 := by decide
theorem directClosure : 126 + 198 = 324 := by decide
theorem outsideDirectClosure : 684 + 14 = 698 := by decide
theorem partition : 324 + 698 = 1022 := by decide
theorem newClassSourceCount : 9 + 18 + 9 + 9 + 9 = 54 := by decide
theorem order548 : 4 * 137 = 548 := by decide
theorem derivative548 : 6 + 3 * 35 = 111 := by decide
theorem simpleRoot548 : Nat.Coprime 137 111 := by decide
theorem pointCount548 : 513 + 3 * 35 = 618 := by decide

end GhsQ8NewQuarticClasses
