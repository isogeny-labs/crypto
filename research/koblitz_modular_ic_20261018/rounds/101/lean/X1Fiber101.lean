import Std.Tactic

namespace X1Fiber101

-- Coefficient bit i represents the coefficient of t^i in F2[t].
def mul (a b : Nat) : Nat :=
  (List.range 17).foldl
    (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0

def power (a n : Nat) : Nat :=
  (List.range n).foldl (fun acc _ => mul acc a) 1

-- The source-bound CAS/Python results supply the specialized equations;
-- these concrete lemmas check their displayed F2 polynomial identities.
theorem lambdaFactor :
    power 7 8 = Nat.xor (Nat.xor (Nat.shiftLeft 1 16) (Nat.shiftLeft 1 8)) 1 := by
  decide

theorem baseFactor :
    mul (Nat.shiftLeft 1 3) (power 3 4) =
      Nat.xor (Nat.shiftLeft 1 7) (Nat.shiftLeft 1 3) := by
  decide

theorem componentLengths : 3 * 8 * 2 = 48 ∧ 4 * 8 * 2 = 64 ∧ 48 + 64 = 112 := by
  omega

end X1Fiber101
