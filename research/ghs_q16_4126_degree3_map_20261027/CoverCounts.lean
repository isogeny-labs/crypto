import Init

/-!
Checked arithmetic consequences of the external order-4126 finite
certificate. The geometric degree-three classification, field residue
elimination, and exhaustive Sage run are external to this Lean file.
-/

set_option autoImplicit false

namespace GhsQ16Order4126CoverCounts

theorem exact_pairs : 108 * 140 = 15120 := by decide
theorem four_per_pair : 4 * 3 * 2 = 24 := by decide
theorem four_assignments : 15120 * 24 = 362880 := by decide
theorem hidden_points : 4096 - 4 + 1 = 4093 := by decide
theorem five_per_pair : 4 * 2 * 4093 = 32744 := by decide
theorem five_assignments : 15120 * 32744 = 495089280 := by decide
theorem one_source_four : 140 * 24 = 3360 := by decide
theorem one_source_five : 140 * 32744 = 4584160 := by decide
theorem source_order : 4126 = 2 * 2063 := by decide
theorem model_partition : 4 + 136 = 140 := by decide

end GhsQ16Order4126CoverCounts
