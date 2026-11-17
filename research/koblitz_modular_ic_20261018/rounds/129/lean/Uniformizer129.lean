import Std.Tactic

namespace Uniformizer129

def step (s : Nat × Nat × Nat) : Nat × Nat × Nat :=
  let z := if s.2.2 % 2 = 1 then Nat.xor s.1 s.2.1 else s.1
  let a := 2 * s.2.1
  (z, (if a ≥ 128 then Nat.xor a 203 else a), s.2.2 / 2)
def gfMul (a b : Nat) : Nat :=
  ((List.range 7).foldl (fun s _ => step s) (0, a, b)).1

theorem leadingEquation : Nat.xor (gfMul 84 6) 110 = 0 := by native_decide
theorem transverseUnit : (84 : Nat) ≠ 0 := by decide
theorem branchDegreeSum : (7 : Nat) * 1 + 7 * 2 + 28 * 1 = 49 := by native_decide
theorem relativeDegreeSum : (1 : Nat) * 1 + 1 * 2 + 4 * 1 = 7 := by native_decide

end Uniformizer129
