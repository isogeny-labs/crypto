import Std.Tactic

namespace LeadingResidue104

-- Bit i is the coefficient of t^i in F2[t]. These concrete lemmas
-- check the factors independently of the source-bound CAS transcript.
def mul (a b : Nat) : Nat :=
  (List.range 25).foldl
    (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0

def power (a n : Nat) : Nat :=
  (List.range n).foldl (fun acc _ => mul acc a) 1

def q : Nat := 7  -- t^2+t+1
def p : Nat := 28123  -- t^14+t^13+t^11+t^10+t^8+t^7+t^6+t^4+t^3+t+1
def yFactor : Nat := Nat.xor (Nat.shiftLeft 1 6) (Nat.shiftLeft 1 2)

theorem lambdaSocleFactor : power q 7 = p := by decide
theorem lambdaFiberFactor : power q 8 = Nat.xor (Nat.xor (Nat.shiftLeft 1 16) (Nat.shiftLeft 1 8)) 1 := by decide
theorem baseSocleFactor : mul (Nat.shiftLeft 1 2) (power 3 4) = yFactor := by decide
theorem baseFiberFactor : mul (Nat.shiftLeft 1 3) (power 3 4) =
    Nat.xor (Nat.shiftLeft 1 7) (Nat.shiftLeft 1 3) := by decide

-- The leading term lies in the standard-basis box, and the fiber dimensions
-- are the two local lengths 3*8*2 and 4*8*2.
theorem standardBounds : 14 < 16 ∧ 6 < 7 ∧ 3 * 8 * 2 + 4 * 8 * 2 = 112 := by omega

end LeadingResidue104
