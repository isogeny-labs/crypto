import Init

/-! Arithmetic and representative counts for the frozen sparse degree-eight screen.
    Sage's geometric and polynomial checks are external to Lean. -/
namespace TraceMinus14DegreeEightSparseCounts

def sources : List Nat := [11,202,38,80]
def carriers : List Nat := [9735,17371]
def cells : List (Nat × Nat) :=
  sources.flatMap fun b => carriers.map fun f => (b,f)

theorem eight_distinct_cells : cells.length = 8 ∧ cells.Nodup := by decide
theorem all_branch_assignments : (3 : Nat)^5 = 243 := by decide
theorem four_branch_assignments : (3 : Nat)^4 = 81 := by decide
theorem one_pole_assignments_per_shape : (5 : Nat) * 81 = 405 := by decide
theorem one_pole_candidates_per_shape : (405 : Nat) * 121 = 49005 := by decide
theorem logical_candidates_per_cell : (4 : Nat) * 243 + 4 * 49005 = 196992 := by decide
theorem logical_candidates_all_cells : (8 : Nat) * 196992 = 1575936 := by decide
theorem point_controls_per_cell : (4 : Nat) * 405 * 10 = 16200 := by decide
theorem exact_pairs_represented : (20 : Nat) * 12 = 240 := by decide

end TraceMinus14DegreeEightSparseCounts
