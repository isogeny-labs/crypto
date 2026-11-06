import Std.Tactic

namespace N83K4405Capacity

def states (k : Nat) : Nat := 83 * (k * (k - 1) / 2) + 42 * k
def request (k : Nat) : Nat := (4 * states k + 2) / 3

theorem states_step (k : Nat) : states k ≤ states (k + 1) := by
  have pair_le : k * (k - 1) ≤ (k + 1) * k :=
    Nat.mul_le_mul (by omega) (Nat.sub_le k 1)
  have div_le : k * (k - 1) / 2 ≤ (k + 1) * k / 2 :=
    Nat.div_le_div_right pair_le
  have lhs_le := Nat.mul_le_mul_left 83 div_le
  have rhs_le := Nat.mul_le_mul_left 42 (show k ≤ k + 1 by omega)
  simpa [states] using Nat.add_le_add lhs_le rhs_le

theorem request_step (k : Nat) : request k ≤ request (k + 1) := by
  unfold request
  exact Nat.div_le_div_right
    (Nat.add_le_add (Nat.mul_le_mul_left 4 (states_step k)) (Nat.le_refl 2))

theorem k4405_states : states 4405 = 805269240 := by decide
theorem k4405_request : request 4405 = 1073692320 := by decide
theorem k4406_request : request 4406 = 1074179863 := by decide
theorem k4405_last_request_in_bucket :
    request 4405 ≤ 2 ^ 30 ∧ 2 ^ 30 < request 4406 := by decide

theorem request_above_bucket_for_all_larger_k (k : Nat) (hk : 4406 ≤ k) :
    2 ^ 30 < request k := by
  have lower : ∀ j : Nat, 4406 ≤ j → request 4406 ≤ request j := by
    intro j
    induction j with
    | zero => omega
    | succ j ih =>
      intro hj
      by_cases h : 4406 ≤ j
      · exact Nat.le_trans (ih h) (request_step j)
      · have : j = 4405 := by omega
        subst j
        exact Nat.le_refl _
  exact Nat.lt_of_lt_of_le k4405_last_request_in_bucket.2 (lower k hk)

#print axioms N83K4405Capacity.k4405_states
#print axioms N83K4405Capacity.k4405_request
#print axioms N83K4405Capacity.k4406_request
#print axioms N83K4405Capacity.k4405_last_request_in_bucket
#print axioms N83K4405Capacity.states_step
#print axioms N83K4405Capacity.request_step
#print axioms N83K4405Capacity.request_above_bucket_for_all_larger_k

end N83K4405Capacity
