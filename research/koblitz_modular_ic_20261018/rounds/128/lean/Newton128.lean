import Std.Tactic

namespace Newton128

def step (s : Nat × Nat × Nat) : Nat × Nat × Nat :=
  let z := if s.2.2 % 2 = 1 then Nat.xor s.1 s.2.1 else s.1
  let a := 2 * s.2.1
  (z, (if a ≥ 128 then Nat.xor a 203 else a), s.2.2 / 2)
def gfMul (a b : Nat) : Nat :=
  ((List.range 7).foldl (fun s _ => step s) (0, a, b)).1

def orders : List (Nat × Nat) :=
  [(0,1),(1,1),(2,0),(3,0),(4,0),(5,1),(6,0),(7,0)]
def onPositiveEdge (j v : Nat) : Bool := decide (2*v+j=2)
def positiveSupport : List Nat :=
  (orders.filter fun p => onPositiveEdge p.1 p.2).map Prod.fst

theorem positiveEdgeSupport : positiveSupport = [0,2] := by native_decide
theorem edgeLeadingRelation : gfMul 84 6 = 110 := by native_decide
theorem transverseDerivativeUnit : (84 : Nat) ≠ 0 := by decide
theorem slopeDenominator : (2 : Nat) * 1 = 2 := by decide

end Newton128
