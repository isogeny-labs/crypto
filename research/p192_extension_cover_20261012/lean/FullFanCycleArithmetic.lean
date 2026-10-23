import Mathlib.Data.Nat.Choose.Basic
import Mathlib.Tactic.NormNum

/-!
Arithmetic consequences of the independently replayed complete-fan
incidence counts.  The field-point labels, graph connectivity, and sparse
matrix rank are finite-field inputs checked by the producer and replay.
-/

namespace P192ExtensionCover

theorem full_fan_cycle_dimension_arithmetic :
    (15034 : ℕ) - (3 + 8718) + 1 = 6314 := by
  norm_num

theorem full_fan_implicit_rectangle_count :
    Nat.choose 2574 2 + Nat.choose 2552 2 + Nat.choose 2485 2 = 9652897 := by
  norm_num [Nat.choose_two_right]

theorem full_fan_active_edge_count :
    2 * (2574 + 2552 + 2485 - 3 * 1295) + 3 * 1295 = 11337 := by
  norm_num

theorem non_source_cycle_dimension_arithmetic :
    (15028 : ℕ) - (3 + 8717) + 1 = 6309 := by
  norm_num

theorem non_source_implicit_rectangle_count :
    Nat.choose 2571 2 + Nat.choose 2550 2 + Nat.choose 2483 2 = 9635113 := by
  norm_num [Nat.choose_two_right]

theorem non_source_active_edge_count :
    2 * (2571 + 2550 + 2483 - 3 * 1293) + 3 * 1293 = 11329 := by
  norm_num

theorem full_rectangle_accepted_capacity_bound :
    9652897 * (2 ^ 166 : ℕ) <
      1569275433846670190958947353987675545133315652035359680560 := by
  norm_num

/-- Three pivot fans and sixteen shared endpoints give 48 cuts and 360
presentations; the connected incidence graph has thirty cycle rows. -/
theorem triple_star_16_cut_count : 3 * 16 = 48 := by
  norm_num

theorem triple_star_16_rectangle_count :
    3 * Nat.choose 16 2 = 360 := by
  norm_num [Nat.choose_two_right]

theorem triple_star_16_cycle_dimension_arithmetic :
    (48 : ℕ) - (3 + 16) + 1 = 30 := by
  norm_num

/-- With 48 cuts, at most three edges per endpoint, and one connected
incidence component, the graph cycle rank cannot exceed thirty. -/
theorem connected_48_cut_three_pivot_cycle_upper_bound
    {endpoints rank : ℕ}
    (hcapacity : 48 ≤ 3 * endpoints)
    (hcycle : rank + (3 + endpoints) = 49) :
    rank ≤ 30 := by
  omega

theorem triple_star_ideal_uniform_support_bound :
    360 * (2 ^ 181 : ℕ) <
      1569275433846670190958947353987675545133315652035359680560 := by
  norm_num

/-- The character-selected 128 admitted targets occupy less than a
`2⁻¹⁸²` share of the accepted-set lower bound under an ideal-uniform law. -/
theorem character_star_ideal_uniform_support_bound :
    128 * (2 ^ 182 : ℕ) <
      1569275433846670190958947353987675545133315652035359680560 := by
  norm_num

#print axioms full_fan_cycle_dimension_arithmetic
#print axioms full_fan_implicit_rectangle_count
#print axioms full_fan_active_edge_count
#print axioms non_source_cycle_dimension_arithmetic
#print axioms non_source_implicit_rectangle_count
#print axioms non_source_active_edge_count
#print axioms full_rectangle_accepted_capacity_bound
#print axioms triple_star_16_cut_count
#print axioms triple_star_16_rectangle_count
#print axioms triple_star_16_cycle_dimension_arithmetic
#print axioms connected_48_cut_three_pivot_cycle_upper_bound
#print axioms triple_star_ideal_uniform_support_bound
#print axioms character_star_ideal_uniform_support_bound

end P192ExtensionCover
