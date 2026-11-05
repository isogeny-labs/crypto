import Std.Tactic

namespace NuOrders102

-- The source-bound R102 checker supplies these 105 coefficient orders.
def orders : List Int := [-9, -7, -6, -6, -9, -7, -6, -9, -5, -7, -7, -9, -7, -5, -6, -6, -6, -6, -5, -5, -2, -9, -6, -7, -7, -9, -7, -5, -9, -7, -2, -1, -9, -7, -6, -4, -4, -4, -4, -3, -2, -2, -9, -7, -3, -3, -9, -7, -6, -9, -6, -7, -7, -9, -7, -5, -9, -7, -4, -4, -9, -7, -6, -4, -4, -4, -4, -3, -2, -2, -9, -7, -4, -4, -9, -7, -6, -9, -6, -7, -7, -9, -7, -5, -6, -6, -6, -6, -5, -5, -2, -9, -5, -7, -7, -9, -7, -5, -9, -7, -6, -6, -9, -7, -6]

theorem count : orders.length = 105 := by decide

theorem lowerBound : ∀ v ∈ orders, (-9 : Int) ≤ v := by decide

theorem worstOccurs : (-9 : Int) ∈ orders := by decide

theorem clearedByNine : ∀ v ∈ orders, 0 ≤ v + 9 := by
  intro v hv
  have h := lowerBound v hv
  omega

theorem eightDoesNotClear : ∃ v ∈ orders, v + 8 < 0 := by
  exact ⟨-9, worstOccurs, by omega⟩

end NuOrders102
