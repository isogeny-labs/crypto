import GreenCertificate

namespace MarkovGreenUpperAudit
noncomputable section
open scoped BigOperators

def weightedGeometric (d : ℝ) (n : ℕ) : ℝ :=
  ∑ j ∈ Finset.range n, ((j : ℝ)+1)*d^j

theorem weighted_geometric_identity (d : ℝ) (n : ℕ) :
    (1-d)^2*weightedGeometric d n = 1-((n : ℝ)+1)*d^n+(n : ℝ)*d^(n+1) := by
  induction n with
  | zero => simp [weightedGeometric]
  | succ n ih =>
    rw [weightedGeometric, Finset.sum_range_succ]
    change (1-d)^2*(weightedGeometric d n+((n : ℝ)+1)*d^n) = _
    rw [mul_add, ih]
    push_cast
    simp only [pow_succ]
    ring

theorem weighted_geometric_bound (d : ℝ) (hd : 0≤d) (hd1 : d<1) (n : ℕ) :
    weightedGeometric d n ≤ 1/(1-d)^2 := by
  have hp := pow_nonneg hd n
  have hn : 0≤(n : ℝ) := Nat.cast_nonneg n
  have hr : 0≤(n : ℝ)*(1-d)+1 := by positivity
  have ha := mul_nonneg hp hr
  have hm : (1-d)^2*weightedGeometric d n ≤ 1 := by
    rw [weighted_geometric_identity, pow_succ]
    nlinarith
  apply (le_div_iff₀ (sq_pos_of_pos (sub_pos.mpr hd1))).mpr
  nlinarith

theorem linear_weight_sum (n : ℕ) :
    2*(∑ j ∈ Finset.range n, ((j : ℝ)+1)) = (n : ℝ)*((n : ℝ)+1) := by
  induction n with
  | zero => simp
  | succ n ih =>
    rw [Finset.sum_range_succ]
    push_cast
    nlinarith

theorem envelope_weight_sum (c d : ℝ) (n : ℕ) :
    2*(∑ j ∈ Finset.range n, ((j : ℝ)+1)*(c+d^j)) =
      c*(n : ℝ)*((n : ℝ)+1)+2*weightedGeometric d n := by
  calc
    _ = c*(2*(∑ j ∈ Finset.range n, ((j : ℝ)+1)))+2*weightedGeometric d n := by
      simp_rw [mul_add]
      rw [Finset.sum_add_distrib, ← Finset.sum_mul]
      unfold weightedGeometric
      ring
    _ = _ := by rw [linear_weight_sum]; ring

theorem weighted_zero_terms (d : ℝ) : weightedGeometric d 0 = 0 := by
  simp [weightedGeometric]

theorem weighted_one_term (d : ℝ) : weightedGeometric d 1 = 1 := by
  simp [weightedGeometric]

theorem weighted_zero_ratio (n : ℕ) : weightedGeometric 0 (n+1) = 1 := by
  induction n with
  | zero => exact weighted_one_term 0
  | succ n ih =>
    simp only [weightedGeometric, Finset.sum_range_succ] at ih ⊢
    simp at ih ⊢
    exact ih

#print axioms weighted_geometric_identity
#print axioms weighted_geometric_bound
#print axioms envelope_weight_sum
end
end MarkovGreenUpperAudit
