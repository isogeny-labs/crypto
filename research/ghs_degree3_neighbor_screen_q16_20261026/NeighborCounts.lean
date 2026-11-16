import Init

/-!
Arithmetic identities in the finite twelve-source/four-Jacobian-model
degree-three screen. Geometry and finite-field residues are justified in
REPORT.md and replayed by Sage; they are not formalized here.
-/

namespace GhsDegreeThreeNeighbors

theorem source_model_pairs : 12 * 4 = 48 := by decide
theorem four_support_per_pair : 4 * 3 * 2 = 24 := by decide
theorem five_support_per_pair : 4 * 2 * (4096 - 4 + 1) = 32744 := by decide
theorem four_support_total : 48 * 24 = 1152 := by decide
theorem five_support_total : 48 * 32744 = 1571712 := by decide
theorem all_functions_total : 1152 + 1571712 = 1572864 := by decide
theorem subgroup_coprime_to_three : Nat.Coprime 3 1987 := by decide

end GhsDegreeThreeNeighbors
