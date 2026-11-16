import Std.Tactic

namespace F4Newton119

def rows_y0_e1 : List (Nat × Int) := [(0, 8), (1, 23), (4, 23), (8, 0), (16, 0)]
def rows_y0_e3 : List (Nat × Int) := [(0, 8), (1, 23), (4, 23), (8, 0), (16, 0)]
def rows_y1_e1 : List (Nat × Int) := [(0, 28), (1, 23), (4, 23), (8, 0), (16, 0)]
def rows_y1_e2 : List (Nat × Int) := [(0, 16), (1, 23), (4, 23), (8, 0), (16, 0)]

def above (rows : List (Nat × Int)) (a b c : Int) : Bool :=
 rows.all fun p => decide (a * p.2 >= b + c * Int.ofNat p.1)
def onLine (rows : List (Nat × Int)) (a b c : Int) : List Nat :=
 (rows.filter fun p => decide (a * p.2 = b + c * Int.ofNat p.1)).map Prod.fst

theorem edge_0 : above rows_y0_e1 1 (8) (-1) = true ∧ onLine rows_y0_e1 1 (8) (-1) = [0, 8] := by native_decide
theorem edge_1 : above rows_y0_e3 1 (8) (-1) = true ∧ onLine rows_y0_e3 1 (8) (-1) = [0, 8] := by native_decide
theorem edge_2 : above rows_y1_e1 1 (28) (-5) = true ∧ onLine rows_y1_e1 1 (28) (-5) = [0, 1] := by native_decide
theorem edge_3 : above rows_y1_e1 7 (184) (-23) = true ∧ onLine rows_y1_e1 7 (184) (-23) = [1, 8] := by native_decide
theorem edge_4 : above rows_y1_e2 1 (16) (-2) = true ∧ onLine rows_y1_e2 1 (16) (-2) = [0, 8] := by native_decide
theorem horizontal_y0_e1 : above rows_y0_e1 1 0 0 = true ∧ onLine rows_y0_e1 1 0 0 = [8, 16] := by native_decide
theorem horizontal_y0_e3 : above rows_y0_e3 1 0 0 = true ∧ onLine rows_y0_e3 1 0 0 = [8, 16] := by native_decide
theorem horizontal_y1_e1 : above rows_y1_e1 1 0 0 = true ∧ onLine rows_y1_e1 1 0 0 = [8, 16] := by native_decide
theorem horizontal_y1_e2 : above rows_y1_e2 1 0 0 = true ∧ onLine rows_y1_e2 1 0 0 = [8, 16] := by native_decide

end F4Newton119
