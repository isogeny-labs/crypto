import Std.Tactic

namespace NewtonJets105

-- Bit i is the coefficient of degree i over F2.
def mul (a b : Nat) : Nat :=
  (List.range 17).foldl
    (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0

def power (a n : Nat) : Nat :=
  (List.range n).foldl (fun acc _ => mul acc a) 1

theorem lambdaFactor : mul 7 6 = 18 := by decide
theorem yFactor : mul (Nat.shiftLeft 1 3) (power 3 3) = 120 := by decide
theorem firstMixedLambdaFactor :
    mul (mul 7 6) (mul (Nat.shiftLeft 1 3) (power 3 3)) =
      mul 18 120 := by decide

-- The two equality endpoints of each lower Newton edge are coprime.
theorem yZeroEdge :
    3 * 2 + 2 * 0 = 6 ∧ 3 * 0 + 2 * 3 = 6 ∧
    3 * 4 + 2 * 1 > 6 ∧ 3 * 1 + 2 * 2 > 6 ∧
    Nat.gcd 2 3 = 1 := by decide

theorem yOneEdge :
    4 * 3 + 3 * 0 = 12 ∧ 4 * 0 + 3 * 4 = 12 ∧
    4 * 3 + 3 * 1 > 12 ∧ 4 * 2 + 3 * 2 > 12 ∧
    4 * 1 + 3 * 3 > 12 ∧ Nat.gcd 3 4 = 1 := by decide

end NewtonJets105
