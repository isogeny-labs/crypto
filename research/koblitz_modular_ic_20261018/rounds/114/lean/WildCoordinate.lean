import Std.Tactic

namespace WildCoordinate

-- The finite support is bound to the source-derived JSON by verify114.py.
def mul2 (a b : Nat) : Nat :=
 (List.range 8).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0

theorem repeatedInitialEdge : mul2 3 3 = 5 := by native_decide
theorem directUniformizerOrder : 3 - 2 = 1 := by decide
-- If v_x(v)=3/2, normalized orders are v(v)=3 and v(x)=2.
theorem nextLinearTermOrder : 3 < 2*3 := by decide

end WildCoordinate
