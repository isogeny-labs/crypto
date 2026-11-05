import Std.Tactic

namespace F4Newton121

def rows_y0_e1 : List (Nat × Int) := [(0, 6), (1, 2), (4, 11), (8, 0), (16, 24)]
def rows_y0_e3 : List (Nat × Int) := [(0, 8), (1, 9), (4, 15), (8, 0), (16, 16)]

def above (rows : List (Nat × Int)) (a b c : Int) : Bool :=
 rows.all fun p => decide (a * p.2 >= b + c * Int.ofNat p.1)
def onLine (rows : List (Nat × Int)) (a b c : Int) : List Nat :=
 (rows.filter fun p => decide (a * p.2 = b + c * Int.ofNat p.1)).map Prod.fst

theorem edge_0 : above rows_y0_e1 1 (6) (-4) = true ∧ onLine rows_y0_e1 1 (6) (-4) = [0, 1] := by native_decide
theorem edge_1 : above rows_y0_e1 7 (16) (-2) = true ∧ onLine rows_y0_e1 7 (16) (-2) = [1, 8] := by native_decide
theorem edge_2 : above rows_y0_e1 1 (-24) (3) = true ∧ onLine rows_y0_e1 1 (-24) (3) = [8, 16] := by native_decide
theorem edge_3 : above rows_y0_e3 1 (8) (-1) = true ∧ onLine rows_y0_e3 1 (8) (-1) = [0, 8] := by native_decide
theorem edge_4 : above rows_y0_e3 1 (-16) (2) = true ∧ onLine rows_y0_e3 1 (-16) (2) = [8, 16] := by native_decide

end F4Newton121
