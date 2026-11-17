import WeightedGeometric

namespace MarkovGreenUpperAudit
noncomputable section
open scoped BigOperators
open MarkovGreenBoundAudit
variable {S : Type*} [Fintype S] [DecidableEq S] [Nonempty S]

theorem source_rhs_envelope (P : Matrix S S ℝ) (T : ℕ) (c d : ℝ)
    (hd : 0≤d) (hd1 : d<1)
    (he : ∀ j : ℕ, j≤T → ∀ x y : S, (P^j) x y ≤ c+d^j) :
    sourceRHS P T ≤ c*((T : ℝ)+1)*((T : ℝ)+2)+2/(1-d)^2 := by
  have hm (j : ℕ) (hj : j≤T) : maxEntry P j ≤ c+d^j :=
    Finset.sup'_le _ _ (fun xy _ => he j hj xy.1 xy.2)
  have hb : sourceRHS P T ≤
      2*(∑ j ∈ Finset.range (T+1), ((j : ℝ)+1)*(c+d^j)) := by
    unfold sourceRHS
    apply mul_le_mul_of_nonneg_left _ (by norm_num)
    apply Finset.sum_le_sum
    intro j hj
    exact mul_le_mul_of_nonneg_left (hm j (by simpa using Finset.mem_range.mp hj)) (by positivity)
  rw [envelope_weight_sum] at hb
  have hg := weighted_geometric_bound d hd hd1 (T+1)
  push_cast at hb
  have hnum : c*((T : ℝ)+1)*((T : ℝ)+2)+2/(1-d)^2 =
      c*((T : ℝ)+1)*((T : ℝ)+1+1)+2*(1/(1-d)^2) := by ring
  rw [hnum]
  exact hb.trans (add_le_add le_rfl (mul_le_mul_of_nonneg_left hg (by norm_num : (0 : ℝ)≤2)))

/-- Actual Green maxima inherit the finite numerical bound from their actual kernels. -/
theorem finite_green_envelope (P : Matrix S S ℝ) (hP : P ∈ doublyStochastic ℝ S)
    (T : ℕ) (c d : ℝ) (hd : 0≤d) (hd1 : d<1)
    (he : ∀ j : ℕ, j≤T → ∀ x y : S, (P^j) x y ≤ c+d^j) :
    rowMaximum P T ≤ c*((T : ℝ)+1)*((T : ℝ)+2)+2/(1-d)^2 ∧
    columnMaximum P T ≤ c*((T : ℝ)+1)*((T : ℝ)+2)+2/(1-d)^2 := by
  have h := qualified_green_bound P hP T
  have hb := source_rhs_envelope P T c d hd hd1 he
  exact ⟨h.1.trans hb, h.2.trans hb⟩

#print axioms source_rhs_envelope
#print axioms finite_green_envelope
end
end MarkovGreenUpperAudit
