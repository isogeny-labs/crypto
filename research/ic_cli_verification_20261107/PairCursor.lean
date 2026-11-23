import Lean.Elab.Tactic.Omega

/-!
Resumability arithmetic for the fixed-workflow pair table.

`indexcalc_fixed.py` stores a cursor `(i, j)` and a rolling checksum. Before
resuming it recomputes the number of candidates already considered as

    considered = i*size - i*(i-1)/2 + (j-i)

This file models the same quantity recursively (avoiding the nonlinear closed
form) and proves the two properties the resume path depends on:

* each candidate step increases `considered` by exactly one;
* finishing a row and moving to the next row preserves `considered`.

A Python test separately checks that the closed form above equals the
recursive model for every cursor with `size <= 30`.
-/

namespace PairCursor

/-- Number of candidates in rows `0 .. i-1`, where row `k` has `size-k`
entries. -/
def rowPrefix : Nat → Nat → Nat
  | _, 0 => 0
  | size, i + 1 => rowPrefix size i + (size - i)

/-- Candidates considered after finishing rows before `i` and advancing `j`
entries in row `i`. -/
def considered (size i j : Nat) : Nat := rowPrefix size i + (j - i)

/-- Every candidate in the row keeps the cursor in the same row. -/
theorem considered_step (size i j : Nat) (hij : i ≤ j) :
    considered size i (j + 1) = considered size i j + 1 := by
  unfold considered
  have h : j + 1 - i = (j - i) + 1 := by omega
  rw [h]
  omega

/-- The last candidate of a row and the first candidate of the next row are
the same resume point. -/
theorem considered_row_break (size i : Nat) (_hi : i < size) :
    considered size (i + 1) (i + 1) = considered size i size := by
  simp [considered, rowPrefix]

/-- Starting at the beginning considers nothing. -/
theorem considered_zero (size : Nat) : considered size 0 0 = 0 := by
  simp [considered, rowPrefix]

/-- Finishing the table considers exactly the row prefix at `size`. -/
theorem considered_total (size : Nat) : considered size size size = rowPrefix size size := by
  simp [considered]

/-- The prefix is monotone in the row index. -/
theorem rowPrefix_mono (size i j : Nat) (hij : i ≤ j) :
    rowPrefix size i ≤ rowPrefix size j := by
  induction j with
  | zero =>
      have : i = 0 := by omega
      simp [this]
  | succ j ih =>
      by_cases h : i ≤ j
      · have := ih h
        simp [rowPrefix]
        omega
      · have : i = j + 1 := by omega
        simp [this]

end PairCursor

#print axioms PairCursor.considered_step
#print axioms PairCursor.considered_row_break
#print axioms PairCursor.considered_zero
#print axioms PairCursor.considered_total
#print axioms PairCursor.rowPrefix_mono
