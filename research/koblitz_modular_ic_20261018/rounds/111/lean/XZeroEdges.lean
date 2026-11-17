import Std.Tactic

namespace XZeroEdges

-- Bit i is the coefficient of T^i in F2[T].
def mul2 (a b : Nat) : Nat :=
 (List.range 8).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0

def cube (a : Nat) : Nat := mul2 (mul2 a a) a

theorem y0SecondEdgeFactor : mul2 3 7 = 9 := by native_decide
-- 1+T^3=(1+T)(1+T+T^2); the slope is 1/3.
theorem y1RepeatedEdge : cube 3 = 15 := by native_decide
-- 1+T+T^2+T^3=(1+T)^3, a repeated edge root.
theorem xZeroBaseFiber : Nat.xor (Nat.xor (Nat.xor (2^4) (2^5)) (2^6)) (2^7) = 240 := by native_decide

end XZeroEdges
