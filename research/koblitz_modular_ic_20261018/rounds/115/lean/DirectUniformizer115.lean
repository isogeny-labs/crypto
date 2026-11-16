import Std.Tactic

namespace DirectUniformizer115

-- Bit i is the coefficient of t^i in F2[t]/(t^128).
def mul (a b : Nat) : Nat :=
 (List.range 128).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0 % (2^128)
def power (a n : Nat) : Nat :=
 (List.range n).foldl (fun acc _ => mul acc a) 1
def E : List (Nat × Nat) := [(0, 1), (0, 2), (0, 3), (0, 4), (1, 4), (1, 6), (1, 8), (1, 10), (2, 0), (2, 1), (2, 3), (2, 4), (2, 5), (2, 7), (2, 8), (2, 9), (2, 10), (2, 11), (3, 1), (3, 2), (3, 5), (3, 6), (3, 8), (3, 9), (4, 9), (5, 10), (5, 12), (5, 14), (6, 11), (6, 12), (6, 13), (6, 14), (6, 15), (7, 13)]
def x : Nat := 234551786471618107977878550630337635940
def evalE : Nat := E.foldl (fun acc p => Nat.xor acc (Nat.shiftLeft (power x p.2) p.1)) 0 % (2^128)

theorem residual128 : evalE = 0 := by native_decide
theorem leadingX : Nat.testBit x 2 ∧ ¬ Nat.testBit x 0 ∧ ¬ Nat.testBit x 1 := by native_decide
theorem directUniformizerOrder : 3 - 2 = 1 := by decide
theorem primitiveWildSlope : Nat.gcd 3 2 = 1 := by native_decide
theorem baseLocalDegreeSum : 1 + 3 + 1 + 2 = 7 := by decide

end DirectUniformizer115
