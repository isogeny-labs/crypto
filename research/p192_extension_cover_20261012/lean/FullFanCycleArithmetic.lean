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

#print axioms full_fan_cycle_dimension_arithmetic
#print axioms full_fan_implicit_rectangle_count
#print axioms full_fan_active_edge_count
#print axioms non_source_cycle_dimension_arithmetic
#print axioms non_source_implicit_rectangle_count
#print axioms non_source_active_edge_count
#print axioms full_rectangle_accepted_capacity_bound

end P192ExtensionCover
