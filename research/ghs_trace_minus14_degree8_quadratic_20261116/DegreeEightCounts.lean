import Init

/-! Arithmetic checks for the three frozen degree-eight shape screens.
    Exact cover elimination and squarehood remain external to Lean. -/
namespace TraceMinus14DegreeEightCounts

def sourceRepresentatives : List Nat := [11,202,38,80]
def carrierRepresentatives : List Nat := [9735,17371]
def cells : List (Nat × Nat) :=
  sourceRepresentatives.flatMap fun b => carrierRepresentatives.map fun f => (b,f)

theorem cell_count : cells.length = 8 ∧ cells.Nodup := by decide

theorem quadratic_specializations_per_cell : (243 : Nat) * 243 = 59049 := by decide

theorem quadratic_specializations_all_cells : (8 : Nat) * 59049 = 472392 := by decide

theorem linear_parameter_pairs_per_cell : (243 : Nat) * 242 = 58806 := by decide

theorem linear_parameter_pairs_all_cells : (8 : Nat) * 58806 = 470448 := by decide

theorem source_and_carrier_pairs : (20 : Nat) * 12 = 240 := by decide

end TraceMinus14DegreeEightCounts
