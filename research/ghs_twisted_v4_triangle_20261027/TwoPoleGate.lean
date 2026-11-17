/-!
Kernel-checked finite arithmetic for the two-common-pole q=16 certificate.
The field-product characterization, genus, descent, and Jacobian map are
proved on paper in REPORT.md and checked computationally in the receipts;
they are not formalized in Lean here.
-/

namespace GHSTwoPole

set_option maxRecDepth 8192
set_option maxHeartbeats 0

theorem q16_product_count :
    (16^2 - 1) * (16 + 2) / 2 = 2295 := by decide

theorem q16_union_count :
    16^2 * (16+1) / 4 = 1088 ∧
    4095 + 1088 = 5183 ∧ 2 * 4095 = 8190 ∧
    2295 - 1088 = 1207 := by decide

theorem q32_overlap_count :
    32^2 * (32+1) / 4 = 8448 ∧
    32^3 - 1 + 8448 = 41215 := by decide

theorem q16_subgroup_order : 4 * 1033 = 4132 := by decide

private def doubleMod (r x : Nat) : Nat := (2*x) % r

theorem source_1033_double_kernel :
    ∀ x : Fin 1033, doubleMod 1033 x.val = 0 → x.val = 0 := by decide

end GHSTwoPole
