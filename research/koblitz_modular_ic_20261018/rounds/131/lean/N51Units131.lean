import Std.Tactic

namespace N51Units131

theorem placeCount : (2 : Nat) + 2 + 4 + 4 = 12 := by native_decide
theorem unitCount : (2 : Nat) + 2 + 4 + 4 = 12 := by native_decide
theorem zeroResidueCount : (0 : Nat) + 0 + 0 + 0 = 0 := by native_decide
theorem weightedFiberDegree :
    (2 : Nat) * 56 + 2 * 56 * 2 + 4 * 28 + 4 * 84 = 784 := by native_decide

end N51Units131
