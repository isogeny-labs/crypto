import Std
namespace N83ChainSlots
-- Elementary slot arithmetic only; polynomial bridges are not formalized.
theorem sum_le_cap (cap : Nat) (xs : List Nat)
    (h : ∀ x ∈ xs, x ≤ cap) : xs.sum ≤ cap * xs.length := by
  induction xs with
  | nil => simp
  | cons x xs ih =>
    have hx := h x (by simp)
    have hs : ∀ y ∈ xs, y ≤ cap := by
      intro y hy
      exact h y (by simp [hy])
    have hi := ih hs
    simp only [List.sum_cons, List.length_cons, Nat.mul_succ]
    omega

theorem cubic_slots (xs : List Nat) (h : ∀ x ∈ xs, x ≤ 2) :
    xs.sum ≤ 2 * xs.length := sum_le_cap 2 xs h
#print axioms sum_le_cap
#print axioms cubic_slots
end N83ChainSlots
