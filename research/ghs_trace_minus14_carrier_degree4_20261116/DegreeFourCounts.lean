import Init

/-! Arithmetic checks for the corrected twelve-carrier degree-four census.
    The map normalization and Sage elimination are not formalized here. -/
namespace GHSTraceMinus14DegreeFour

def carrierMasks : List Nat :=
  [9735, 17371, 21624, 44227, 64725, 90937, 92153,
   100118, 124586, 144548, 156795, 159256]

theorem carrier_count : carrierMasks.length = 12 := by decide

theorem assignments_per_carrier :
    (243 : Nat) + 243 + 405 + 58806 = 59697 := by decide

theorem total_assignments : (12 : Nat) * 59697 = 716364 := by decide

end GHSTraceMinus14DegreeFour
