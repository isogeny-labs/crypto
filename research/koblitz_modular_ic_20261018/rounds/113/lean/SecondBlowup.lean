import Std.Tactic

namespace SecondBlowup

-- Bit i is the coefficient of T^i in F2[T].
def mul2 (a b : Nat) : Nat :=
 (List.range 8).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0

theorem repeatedEdge : mul2 3 3 = 5 := by native_decide
-- 1+T^2=(1+T)^2; the degree-2 edge residual is inseparable.
theorem primitiveSlope : Nat.gcd 3 2 = 1 := by native_decide
theorem transformedFiber : Nat.xor (2^2) (2^3) = 12 := by native_decide
-- The second quotient has special fiber v^2+v^3.

end SecondBlowup
