import Std.Tactic

namespace BranchCensus123

def baseRamification : List Nat := [1, 3, 1, 2]
def localDegree (e : Nat) : Nat := 2 * e + 2 * 7 * e
theorem baseDegree : baseRamification.sum = 7 := by native_decide
theorem totalDegree : (baseRamification.map localDegree).sum = 112 := by native_decide
theorem eightPlaces : baseRamification.length * 2 = 8 := by native_decide
theorem localSplits : 2 * (1 + 7) = 16 := by native_decide

end BranchCensus123
