import Std.Tactic

namespace LambdaFiber130

theorem linearPlaceLambdaDegree : (2 : Nat) * 8 = 16 := by native_decide
theorem quarticPlaceLambdaDegree : (4 : Nat) * 1 + 4 * 3 = 16 := by native_decide
theorem totalPlaceCount : (2 : Nat) + 2 + 4 + 4 = 12 := by native_decide
theorem weightedFiberDegree :
    (2 : Nat) * 56 * 1 + 2 * 56 * 2 + 4 * 28 * 1 + 4 * 84 * 1 = 784 := by native_decide
theorem derivativeUnits : (93 : Nat) ≠ 0 ∧ (70 : Nat) ≠ 0 ∧ (28271853 : Nat) ≠ 0 := by decide

end LambdaFiber130
