import Std
namespace N83ChainOverlap
-- Arithmetic consequences used by the written support-rank proof.
-- This does not formalize the polynomial, Moore-window or exact-sequence bridges.
theorem weighted_rank_budget (n ell deficit penalty budget : Nat)
    (hrank : budget ≤ n * penalty + (2 * ell) * deficit)
    (hwidth : 2 * ell ≤ n) : budget ≤ n * (penalty + deficit) := by
  calc
    budget ≤ n * penalty + (2 * ell) * deficit := hrank
    _ ≤ n * penalty + n * deficit :=
      Nat.add_le_add_left (Nat.mul_le_mul_right deficit hwidth) _
    _ = n * (penalty + deficit) := by simp [Nat.mul_add]

theorem shifted_defect_bound (q n ell : Nat) (h : ell + 2 ≤ n) :
    q * n + 2 * ell + 2 ≤ (q + 1) * n + ell := by
  simp only [Nat.add_mul, Nat.one_mul]
  omega

#print axioms weighted_rank_budget
#print axioms shifted_defect_bound
end N83ChainOverlap
