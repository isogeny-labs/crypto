import Std.Tactic

namespace BranchValuations

-- Input supports are rebound to the source-bound result by verify110.py.
def n0 : List (Nat × Nat) := [(35, 2)]
def n1 : List (Nat × Nat) := [(28, 2)]
def firstOrder (p : List (Nat × Nat)) : Option Nat :=
  (p.filter fun x => x.2 != 0).map Prod.fst |>.head?

theorem y0Order : firstOrder n0 = some 35 := by native_decide
theorem y1Order : firstOrder n1 = some 28 := by native_decide
theorem branchUniformizer : 10 * 7 - 3 * 23 = 1 := by decide
theorem localDegreeContribution : 2 * 35 + 2 * 28 = 126 := by decide
theorem initialCancellation : Nat.xor 1 1 = 0 := by native_decide

end BranchValuations
