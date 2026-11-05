import Std
namespace N83BinaryPairSlots
-- Only the elementary monomial slot argument, not the polynomial bridge.
theorem sum_le_length (xs : List Nat) (h : ∀ x ∈ xs, x ≤ 1) :
    xs.sum ≤ xs.length := by
  induction xs with
  | nil => simp
  | cons x xs ih =>
    have hx : x ≤ 1 := h x (by simp)
    have hs : ∀ y ∈ xs, y ≤ 1 := by
      intro y hy
      exact h y (by simp [hy])
    have hi := ih hs
    simp only [List.sum_cons, List.length_cons]
    omega

theorem no_degree_28_in_27_slots (xs : List Nat)
    (hn : xs.length = 27) (h : ∀ x ∈ xs, x ≤ 1) : xs.sum < 28 := by
  have hh := sum_le_length xs h
  omega
#print axioms sum_le_length
#print axioms no_degree_28_in_27_slots
end N83BinaryPairSlots
