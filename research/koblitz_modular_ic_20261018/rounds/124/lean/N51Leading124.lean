import Std.Tactic

namespace N51Leading124

def rows_y0_e1 : List (Nat × Int) := [(0, -53), (1, -43), (2, -32), (3, -23), (4, -13), (5, -1), (6, 7), (7, 17), (8, 27), (9, 39), (10, 47), (11, 57), (12, 68), (13, 77), (14, 87)]
def rows_y0_e3 : List (Nat × Int) := [(0, -44), (1, -37), (2, -27), (3, -23), (4, -16), (5, -3), (6, -2), (7, 5), (8, 12), (9, 25), (10, 26), (11, 33), (12, 43), (13, 47), (14, 54)]
def rows_y1_e1 : List (Nat × Int) := [(0, -23), (1, -23), (2, -12), (3, -23), (4, -23), (5, -1), (6, -23), (7, -23), (8, -23), (9, -1), (10, -23), (11, -23), (12, -12), (13, -23), (14, -23)]
def rows_y1_e2 : List (Nat × Int) := [(0, -23), (1, -23), (2, -14), (3, -23), (4, -23), (5, -5), (6, -23), (7, -23), (8, -23), (9, -5), (10, -23), (11, -23), (12, -14), (13, -23), (14, -23)]

def above (rows : List (Nat × Int)) (k m : Int) : Bool :=
 rows.all fun p => decide (p.2 - k * Int.ofNat p.1 >= m)
def tied (rows : List (Nat × Int)) (k m : Int) : List Nat :=
 (rows.filter fun p => decide (p.2 - k * Int.ofNat p.1 = m)).map Prod.fst
def alphaPower (a j : Nat) : Nat :=
 if j % 3 = 0 then 1 else if j % 3 = 1 then a else Nat.xor a 1
def leadingSum (a : Nat) (js : List Nat) : Nat :=
 js.foldl (fun z j => Nat.xor z (alphaPower a j)) 0

theorem weighted_y0_e1 : above rows_y0_e1 10 (-53) = true ∧ tied rows_y0_e1 10 (-53) = [0, 1, 3, 4, 6, 7, 8, 10, 11, 13, 14] := by native_decide
theorem weighted_y0_e3 : above rows_y0_e3 7 (-44) = true ∧ tied rows_y0_e3 7 (-44) = [0, 1, 3, 4, 6, 7, 8, 10, 11, 13, 14] := by native_decide
theorem weighted_y1_e1 : above rows_y1_e1 0 (-23) = true ∧ tied rows_y1_e1 0 (-23) = [0, 1, 3, 4, 6, 7, 8, 10, 11, 13, 14] := by native_decide
theorem weighted_y1_e2 : above rows_y1_e2 0 (-23) = true ∧ tied rows_y1_e2 0 (-23) = [0, 1, 3, 4, 6, 7, 8, 10, 11, 13, 14] := by native_decide
def commonTies : List Nat := [0, 1, 3, 4, 6, 7, 8, 10, 11, 13, 14]
theorem f4Cancellation : leadingSum 2 commonTies = 0 ∧ leadingSum 3 commonTies = 0 := by native_decide

def mul2 (a b : Nat) : Nat :=
 (List.range 15).foldl (fun z j => if Nat.testBit b j then Nat.xor z (Nat.shiftLeft a j) else z) 0
def power2 (a n : Nat) : Nat :=
 (List.range n).foldl (fun z _ => mul2 z a) 1
def tiesPolynomial : Nat :=
 commonTies.foldl (fun z j => Nat.xor z (Nat.shiftLeft 1 j)) 0
theorem seventhPower : tiesPolynomial = power2 7 7 := by native_decide
-- The bit polynomial of the 11 tied powers is (1+T+T^2)^7.

end N51Leading124
