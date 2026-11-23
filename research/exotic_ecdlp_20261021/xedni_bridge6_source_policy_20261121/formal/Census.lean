import Mathlib.Tactic

/-! Finite status census from the saved source-pair outcome rows.
    This module does not encode the elliptic-curve group-law replay. -/

namespace XedniBridge6SourcePolicy

def selected : List ℕ := [31, 73, 99, 141, 199, 241, 249, 289, 348, 388, 439, 481, 530, 570, 582, 624, 657, 697, 744, 786, 834, 876, 928, 968, 996, 1038, 1105, 1147, 1153, 1193, 1264, 1304, 1328, 1370, 1404, 1446, 1504, 1546, 1582, 1624, 1662, 1704]
def bridge : List Bool := [true, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, true, true, false, false, false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, true, true, false, false, false, false]
def rational : List Bool := [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]

theorem selected_count : selected.length = 42 := by decide
theorem local_bridge_count : (bridge.filter id).length = 22 := by decide
theorem first_crt_rational_count : (rational.filter id).length = 0 := by decide

#print axioms selected_count
#print axioms local_bridge_count
#print axioms first_crt_rational_count

end XedniBridge6SourcePolicy
