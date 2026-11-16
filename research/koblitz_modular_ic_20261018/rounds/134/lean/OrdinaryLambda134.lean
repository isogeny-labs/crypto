import Std.Tactic

namespace OrdinaryLambda134

theorem p11LinearLambdaDegrees : (4 : Nat) * 4 = 16 := by native_decide
theorem p11QuinticLambdaDegrees : (4 : Nat) * 1 + 4 * 3 = 16 := by native_decide
theorem p22QuadraticLambdaDegrees : (4 : Nat) * 4 = 16 := by native_decide
theorem p22QuarticLambdaDegrees : (2 : Nat) * 2 + 2 * 6 = 16 := by native_decide
theorem p11Weighted : (4 : Nat) * 44 + 4 * 55 + 4 * 165 = 1056 := by native_decide
theorem p22Weighted : (4 : Nat) * 176 + 2 * 176 + 2 * 528 = 2112 := by native_decide
theorem totalWeighted : (1056 : Nat) + 2112 = 3168 := by native_decide
theorem ordinaryPlaceCount : (4 : Nat) + 8 + 4 + 4 = 20 := by native_decide

end OrdinaryLambda134
