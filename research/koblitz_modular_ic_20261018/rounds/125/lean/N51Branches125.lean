import Std.Tactic

namespace N51Branches125

def rows_y0_e1 : List (Nat × Int) := [(0, -30), (1, -27), (2, -24), (3, -21), (4, -20), (5, -24), (6, -28), (7, -32), (8, -29), (9, -26), (10, -23), (11, -20), (12, -17), (13, -14), (14, -11)]
def rows_y0_e3 : List (Nat × Int) := [(0, -21), (1, -18), (2, -15), (3, -15), (4, -17), (5, -19), (6, -21), (7, -23), (8, -20), (9, -17), (10, -14), (11, -11), (12, -8), (13, -5), (14, -2)]
def rows_y1_e1 : List (Nat × Int) := [(0, 0), (1, 0), (2, 0), (3, -3), (4, -8), (5, -13), (6, -18), (7, -23), (8, -23), (9, -23), (10, -23), (11, -23), (12, -23), (13, -23), (14, -23)]
def rows_y1_e2 : List (Nat × Int) := [(0, 0), (1, 2), (2, 4), (3, 6), (4, 3), (5, -1), (6, -5), (7, -9), (8, -7), (9, -5), (10, -3), (11, -1), (12, 1), (13, 3), (14, 5)]

def above (rows : List (Nat × Int)) (e q m : Int) : Bool :=
 rows.all fun p => decide (e * p.2 + q * Int.ofNat p.1 >= m)
def tied (rows : List (Nat × Int)) (e q m : Int) : List Nat :=
 (rows.filter fun p => decide (e * p.2 + q * Int.ofNat p.1 = m)).map Prod.fst

theorem simple_y0_e1 : above rows_y0_e1 1 4 (-30) = true ∧ tied rows_y0_e1 1 4 (-30) = [0] := by native_decide
theorem seven_y0_e1 : above rows_y0_e1 7 2 (-210) = true ∧ tied rows_y0_e1 7 2 (-210) = [0, 7] := by native_decide
theorem simple_y0_e3 : above rows_y0_e3 1 2 (-21) = true ∧ tied rows_y0_e3 1 2 (-21) = [0] := by native_decide
theorem seven_y0_e3 : above rows_y0_e3 7 2 (-147) = true ∧ tied rows_y0_e3 7 2 (-147) = [0, 7] := by native_decide
theorem simple_y1_e1 : above rows_y1_e1 1 5 (0) = true ∧ tied rows_y1_e1 1 5 (0) = [0] := by native_decide
theorem seven_y1_e1 : above rows_y1_e1 7 23 (0) = true ∧ tied rows_y1_e1 7 23 (0) = [0, 7] := by native_decide
theorem simple_y1_e2 : above rows_y1_e2 1 4 (0) = true ∧ tied rows_y1_e2 1 4 (0) = [0] := by native_decide
theorem seven_y1_e2 : above rows_y1_e2 7 9 (0) = true ∧ tied rows_y1_e2 7 9 (0) = [0, 7] := by native_decide
theorem simpleLeading : (1 : Nat) ≠ 0 := by decide
theorem sevenCancellation : Nat.xor 1 1 = 0 := by decide

end N51Branches125
