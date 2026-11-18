import Std.Tactic

namespace BlowupEdges

-- Bit i is the coefficient of W^i over F2.
def mul2 (a b : Nat) : Nat :=
 (List.range 8).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0

theorem repeatedSlopeOneResidual : mul2 3 3 = 5 := by native_decide
-- 1+W^2=(1+W)^2; its root remains repeated.
theorem simpleSlopeFiveResidual : Nat.testBit 3 0 ∧ Nat.testBit 3 1 := by native_decide
-- The other residual is 1+W and has a simple root.
theorem transformedFiber : Nat.testBit 8 3 ∧ ¬ Nat.testBit 8 0 ∧ ¬ Nat.testBit 8 1 ∧ ¬ Nat.testBit 8 2 := by native_decide
-- After dividing by x^3, the special fiber is W^3.

end BlowupEdges
