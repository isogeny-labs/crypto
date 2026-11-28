import Std.Tactic

namespace Fiber127

def step (s : Nat × Nat × Nat) : Nat × Nat × Nat :=
  let z := if s.2.2 % 2 = 1 then Nat.xor s.1 s.2.1 else s.1
  let a := 2 * s.2.1
  (z, (if a ≥ 128 then Nat.xor a 203 else a), s.2.2 / 2)

def gfMul (a b : Nat) : Nat :=
  ((List.range 7).foldl (fun s _ => step s) (0, a, b)).1

def polyMul (p q : List Nat) : List Nat :=
  (List.range (p.length + q.length - 1)).map fun k =>
    (List.range (k+1)).foldl
      (fun z i => Nat.xor z (gfMul (p.getD i 0) (q.getD (k-i) 0))) 0

theorem specializedFactorProduct :
  polyMul (polyMul [123, 1] (polyMul [90, 1] [90, 1]))
    [25, 38, 83, 66, 1] = [20, 34, 92, 105, 53, 118, 57, 1] := by native_decide

theorem factorDegreeSum : (1 : Nat) + 2 + 4 = 7 := by native_decide

end Fiber127
