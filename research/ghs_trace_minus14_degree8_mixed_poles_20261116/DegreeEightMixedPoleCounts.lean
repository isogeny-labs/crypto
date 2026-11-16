import Init

/-! Arithmetic and representative counts for the frozen mixed-pole screen.
    Polynomial elimination, geometry, and source-bound receipts remain external. -/
namespace TraceMinus14DegreeEightMixedPoleCounts

def sources : List Nat := [11,202,38,80]
def carriers : List Nat := [9735,17371]
def cells : List (Nat × Nat) :=
  sources.flatMap fun b => carriers.map fun f => (b,f)

theorem eight_distinct_cells : cells.length = 8 ∧ cells.Nodup := by decide
theorem branch_assignments_all_poles : (3 : Nat)^5 = 243 := by decide
theorem branch_assignments_except_one : (5 : Nat) * 3^4 = 405 := by decide
theorem nonbranch_choices : (243 : Nat) - 1 = 242 := by decide
theorem square_leading_coefficients : (243 : Nat) / 2 = 121 := by decide
theorem first_shape_candidates : (243 : Nat) * 242 = 58806 := by decide
theorem mixed_shape_candidates : (405 : Nat) * 242 * 121 = 11859210 := by decide
theorem per_cell_candidates : (3 : Nat) * 58806 + 3 * 11859210 = 35754048 := by decide
theorem all_cell_candidates : (8 : Nat) * 35754048 = 286032384 := by decide
theorem point_controls_mixed_shape : (405 : Nat) * (242 * 10 - 10) = 976050 := by decide
theorem exact_pairs_represented : (20 : Nat) * 12 = 240 := by decide

/- A signature is (exponent of L, number of distinct quadratic branch poles,
   exponent of the one nonbranch linear factor). The geometric transformation
   rule is external; these theorems check its arithmetic closure only. -/
def mixedShapeSignatures : List (Nat × Nat × Nat) :=
  [(1,0,2),(1,0,4),(1,0,6),(1,1,2),
   (1,1,4),(3,1,2),(3,0,4),(5,0,2)]

def branchSwap (s : Nat × Nat × Nat) : Nat × Nat × Nat :=
  (8 - (s.1 + 2*s.2.1 + s.2.2),s.2.1,s.2.2)

theorem eight_distinct_mixed_shapes :
    mixedShapeSignatures.length = 8 ∧ mixedShapeSignatures.Nodup := by decide

theorem mixed_shapes_swap_closed :
    ∀ s, s ∈ mixedShapeSignatures → branchSwap s ∈ mixedShapeSignatures := by decide

theorem branch_swap_involution_on_shapes :
    ∀ s, s ∈ mixedShapeSignatures → branchSwap (branchSwap s) = s := by decide

end TraceMinus14DegreeEightMixedPoleCounts
