import Init

/-! Arithmetic checks for twelve trace-minus-fourteen degree-five cells.
    This does not formalize the interpolation or square-obstruction search. -/
namespace GHSTraceMinus14DegreeFive

def carrierMasks : List Nat :=
  [9735, 17371, 21624, 44227, 64725, 90937,
   92153, 100118, 124586, 144548, 156795, 159256]

theorem carrier_count : carrierMasks.length = 12 := by decide

theorem assignments_per_cell :
    (243 : Nat) + 405 + 58806 + 270 + 98010 + 58801 * 243 = 14446377 := by decide

theorem total_assignments : (12 : Nat) * 14446377 = 173356524 := by decide

theorem fallback_checks : (12 : Nat) * 646811 = 7761732 := by decide

theorem source_orbit_pairs : (12 : Nat) * 5 = 60 := by decide

end GHSTraceMinus14DegreeFive
