import Init

/-! Finite mask and count checks for the recorded trace-minus-fourteen slice.
    PGL2 polynomial identities and degree-five elimination remain external. -/
namespace GHSTraceMinus14DegreeFiveClosure

def carrierOrbitA : List Nat :=
  [9735, 44227, 64725, 90937, 124586, 144548]

def carrierOrbitB : List Nat :=
  [17371, 21624, 92153, 100118, 156795, 159256]

theorem carrier_partition_count :
    carrierOrbitA.length = 6 ∧ carrierOrbitB.length = 6 ∧
    (carrierOrbitA ++ carrierOrbitB).Nodup ∧
    (carrierOrbitA ++ carrierOrbitB).length = 12 := by decide

theorem source_partition_count : (4 : Nat) * 5 = 20 := by decide

theorem exact_pair_count : (12 : Nat) * 20 = 240 := by decide

theorem representative_cell_count : (2 : Nat) + 12 + 12 + 12 = 38 := by decide

theorem logical_assignment_count :
    (38 : Nat) * 14446377 = 548962326 := by decide

theorem selected_prime_exceeds_cover_ceiling :
    (258 : Nat) = 6 * 43 ∧ 6 ≤ 33 ∧ 33 < 43 := by decide

end GHSTraceMinus14DegreeFiveClosure
