import Std.Tactic

namespace F4Newton122

def rows_y0_e3 : List (Nat × Int) := [(0, 4), (1, 2), (4, 11), (8, 0), (16, 24)]

def above (rows : List (Nat × Int)) (a b c : Int) : Bool :=
 rows.all fun p => decide (a * p.2 >= b + c * Int.ofNat p.1)
def onLine (rows : List (Nat × Int)) (a b c : Int) : List Nat :=
 (rows.filter fun p => decide (a * p.2 = b + c * Int.ofNat p.1)).map Prod.fst

theorem edge_0 : above rows_y0_e3 1 (4) (-2) = true ∧ onLine rows_y0_e3 1 (4) (-2) = [0, 1] := by native_decide
theorem edge_1 : above rows_y0_e3 7 (16) (-2) = true ∧ onLine rows_y0_e3 7 (16) (-2) = [1, 8] := by native_decide
theorem edge_2 : above rows_y0_e3 1 (-24) (3) = true ∧ onLine rows_y0_e3 1 (-24) (3) = [8, 16] := by native_decide

end F4Newton122
