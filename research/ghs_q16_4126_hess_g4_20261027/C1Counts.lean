import Init

/-!
Checked finite arithmetic for the c=1 Hess genus partition. The field
classification, Hess genus formula, and exhaustive Sage/Magma checks are
external to this Lean module.
-/
set_option autoImplicit false

namespace GhsQ16C1Counts

theorem nonzero_field_elements : 16 ^ 3 - 1 = 4095 := by decide
theorem each_trace_stratum : 15 + 255 + 3825 = 4095 := by decide
theorem all_normalized_models : 2 * 4095 = 8190 := by decide
theorem all_genus_partition : 30 + 510 + 7650 = 8190 := by decide
theorem residual_partition : 10 + 0 + 2865 = 2875 := by decide
theorem genus_one_subfield_sources : 2 * 15 = 30 := by decide
theorem genus_three_trace_zero_sources : 2 * 255 = 510 := by decide
theorem genus_four_full_span_sources : 2 * 3825 = 7650 := by decide

end GhsQ16C1Counts
