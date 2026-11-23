import Std.Tactic

namespace GNewton118

def rows_y0_e1 : List (Nat × Int) := [(0, -160), (1, -127), (4, -97), (8, -80), (16, 0)]
def rows_y0_e3 : List (Nat × Int) := [(0, -112), (1, -82), (4, -61), (8, -56), (16, 0)]
def rows_y1_e1 : List (Nat × Int) := [(0, 0), (1, 23), (4, 23), (8, 0), (16, 0)]
def rows_y1_e2 : List (Nat × Int) := [(0, 0), (1, 23), (4, 23), (8, 0), (16, 0)]

def onOrAbove (rows : List (Nat × Int)) (base slope : Int) : Bool :=
 rows.all fun p => decide (base + slope * Int.ofNat p.1 <= p.2)
def onLine (rows : List (Nat × Int)) (base slope : Int) : List Nat :=
 (rows.filter fun p => p.2 == base + slope * Int.ofNat p.1).map Prod.fst

theorem edge_y0_e1 : onOrAbove rows_y0_e1 (-160) 10 = true ∧ onLine rows_y0_e1 (-160) 10 = [0, 8, 16] := by native_decide
theorem edge_y0_e3 : onOrAbove rows_y0_e3 (-112) 7 = true ∧ onLine rows_y0_e3 (-112) 7 = [0, 8, 16] := by native_decide
theorem edge_y1_e1 : onOrAbove rows_y1_e1 (0) 0 = true ∧ onLine rows_y1_e1 (0) 0 = [0, 8, 16] := by native_decide
theorem edge_y1_e2 : onOrAbove rows_y1_e2 (0) 0 = true ∧ onLine rows_y1_e2 (0) 0 = [0, 8, 16] := by native_decide

def mul2 (a b : Nat) : Nat :=
 (List.range 17).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0
def power2 (a n : Nat) : Nat :=
 (List.range n).foldl (fun acc _ => mul2 acc a) 1
theorem repeatedEdgePolynomial : power2 7 8 = 65793 := by native_decide
-- (1+T+T^2)^8 = 1+T^8+T^16 over F2.

end GNewton118
