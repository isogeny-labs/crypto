import Init

/-! Arithmetic and representative-count checks for the frozen two-pole screen.
    Geometric coverage, exact polynomial identities, and receipt validity are external to Lean. -/
namespace TraceMinus14DegreeEightTwoPoleCounts

def sources : List Nat := [11,202,38,80]
def carriers : List Nat := [9735,17371]
def cells : List (Nat × Nat) :=
  sources.flatMap fun b => carriers.map fun f => (b,f)

theorem eight_distinct_cells : cells.length = 8 ∧ cells.Nodup := by decide
theorem pole_pairs : (5 : Nat) * 4 / 2 = 10 := by decide
theorem two_shapes : (10 : Nat) * 2 = 20 := by decide
theorem branch_assignments : (20 : Nat) * 27 = 540 := by decide
theorem logical_candidates_per_cell : (540 : Nat) * 121 * 243 * 243 = 3858261660 := by decide
theorem logical_candidates_all_cells : (8 : Nat) * 3858261660 = 30866093280 := by decide
theorem point_controls_per_cell : (540 : Nat) * 20 = 10800 := by decide
theorem exact_pairs_represented : (20 : Nat) * 12 = 240 := by decide

end TraceMinus14DegreeEightTwoPoleCounts
