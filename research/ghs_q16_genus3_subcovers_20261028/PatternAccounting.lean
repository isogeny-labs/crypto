/-!
Checked finite arithmetic for the four-branch, degree-four search.

The geometric statement about Artin-Schreier pole conductors and the
finite-field calculations are proved/executed separately in REPORT.md,
search.py, and automorphisms.py. This file does not formalize the field
geometry or certify the Sage output.
-/

namespace GHSQ16GenusThree

theorem exponent_pattern_accounting :
    12 + 12 + 6 = 30 ∧ 12 * 30 * 4095 = 1474200 := by decide

theorem degree_three_pattern_accounting :
    24 + 4 + 4 = 32 ∧ 60 * 24 = 1440 := by decide

theorem degree_three_full_catalog_accounting :
    10664 * 24 = 255936 ∧ 60 ≤ 255936 ∧ 36 + 24 = 60 := by decide

theorem degree_four_full_catalog_accounting :
    10664 * 6 = 63984 ∧ 1020 ≤ 63984 ∧ 510 * 2 = 1020 := by decide

theorem degree_five_branch_pattern_count :
    4 * 3 * 2 = 24 ∧ 4 + 1 = 5 := by decide

theorem nonzero_trace_source_subgroup_order :
    4090 = 10 * 409 ∧ 409 > 1 := by decide

theorem base_isogeny_cubic_trace_gate :
    (7 : Int)^3 - 48 * 7 = 7 ∧
    (-7 : Int)^3 - 48 * (-7) = -7 ∧
    (1 : Int)^3 - 48 * 1 = -47 ∧
    (-1 : Int)^3 - 48 * (-1) = 47 := by decide

theorem trace_zero_factor_orders :
    10 * 409 = 4090 ∧ 24 * 171 = 4104 ∧
    18 * 225 = 4050 ∧ 16 * 259 = 4144 := by decide

theorem branch_symmetry_accounting :
    4 * 3 = 12 ∧ 3 * 12 = 36 := by decide

theorem genus_three_subgroup_order_3974 :
    3974 = 2 * 1987 ∧ 1987 > 1 := by decide

theorem genus_three_subgroup_order_3978 :
    3978 = 17 * 234 ∧ 17 > 1 := by decide

theorem genus_three_subgroup_order_4178 :
    4178 = 2 * 2089 ∧ 2089 > 1 := by decide

end GHSQ16GenusThree
