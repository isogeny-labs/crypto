import SolvingDegreeBound

/-! Exact degree-three control for the same criterion used by the census.
The copied SolvingDegreeBound is unchanged historical proof code. -/
namespace TheoremAudit
open MvPolynomial SolvingDegreeBound
open SolvingDegreeBound.Toy
local notation "R" => MvPolynomial (Fin 2) (ZMod 2)

noncomputable def controlRows : Set R :=
  {e0,e1,f, X 0*e0,X 1*e0,X 0*e1,X 1*e1,X 0*f,X 1*f}

theorem control_X0_in_degree3_span :
    (X 0:R) ∈ Submodule.span (ZMod 2) controlRows := by
  have hi : (X 0:R)=X 0*f+X 1*e0+e0 := by
    simp only [f,e0]
    linear_combination (-(X 0^2*X 1+X 0^2+X 0*X 1:R))*two_eq_zero
  rw [hi]
  apply Submodule.add_mem
  · apply Submodule.add_mem
    · apply Submodule.subset_span; simp [controlRows]
    · apply Submodule.subset_span; simp [controlRows]
  · apply Submodule.subset_span; simp [controlRows]

theorem control_X1_in_degree3_span :
    (X 1:R) ∈ Submodule.span (ZMod 2) controlRows := by
  have hi : (X 1:R)=X 1*f+X 0*e1+e1 := by
    simp only [f,e1]
    linear_combination (-(X 1^2*X 0+X 1^2+X 0*X 1:R))*two_eq_zero
  rw [hi]
  apply Submodule.add_mem
  · apply Submodule.add_mem
    · apply Submodule.subset_span; simp [controlRows]
    · apply Submodule.subset_span; simp [controlRows]
  · apply Submodule.subset_span; simp [controlRows]

theorem control_variables_are_gb :
    IsGB MonomialOrder.degLex ({X 0,X 1}:Set R) (Ideal.span (F:Set R)) := by
  constructor
  · intro g hg
    rcases hg with rfl | hg
    · exact X0_mem
    · have : g=X 1 := hg
      subst g
      exact X1_mem
  · intro g hg hgn
    let m : MonomialOrder (Fin 2) := MonomialOrder.degLex
    have hd : m.degree g ≠ 0 := by
      intro hd
      have he := m.eq_C_of_degree_eq_zero hd
      have hc := m.leadingCoeff_ne_zero_iff.mpr hgn
      apply one_not_mem
      have hi : (1:R)=C (m.leadingCoeff g)⁻¹*g := by
        calc
          1 = C (m.leadingCoeff g)⁻¹ * C (m.leadingCoeff g) := by
            rw [← C_mul,inv_mul_cancel₀ hc,C_1]
          _ = C (m.leadingCoeff g)⁻¹ * g :=
            congrArg (fun z => C (m.leadingCoeff g)⁻¹*z) he.symm
      rw [hi]
      exact Ideal.mul_mem_left _ _ hg
    have hex : ∃ i : Fin 2, (m.degree g) i ≠ 0 := by
      by_contra hn
      push_neg at hn
      exact hd (Finsupp.ext hn)
    obtain ⟨i,hi⟩ := hex
    refine ⟨X i,?_,X_ne_zero _,?_⟩
    · fin_cases i <;> simp
    · rw [MonomialOrder.degree_X]
      intro j
      by_cases hj : j=i
      · subst j; simpa using Nat.one_le_iff_ne_zero.mpr hi
      · simp [Finsupp.single_apply,hj]

theorem control_degree3_rows_suffice :
    ∃ G : Set R, G ⊆ Submodule.span (ZMod 2) controlRows ∧
      IsGB MonomialOrder.degLex G (Ideal.span (F:Set R)) := by
  refine ⟨{X 0,X 1},?_,control_variables_are_gb⟩
  intro g hg
  rcases hg with rfl | hg
  · exact control_X0_in_degree3_span
  · have : g=X 1 := hg
    subst g
    exact control_X1_in_degree3_span

theorem control_all_rows_degree_le_three :
    ∀ g ∈ controlRows, g.totalDegree ≤ 3 := by
  have hp : ∀ i : Fin 2, (X i^2:R).totalDegree ≤ 2 := by
    intro i
    simpa using totalDegree_pow (X i:R) 2
  have he0 : e0.totalDegree ≤ 2 :=
    (totalDegree_add _ _).trans (max_le (hp 0) (by simp))
  have he1 : e1.totalDegree ≤ 2 :=
    (totalDegree_add _ _).trans (max_le (hp 1) (by simp))
  have hxy : (X 0*X 1:R).totalDegree ≤ 2 := by
    simpa using totalDegree_mul (X 0:R) (X 1)
  have hf : f.totalDegree ≤ 2 := by
    exact (totalDegree_add _ _).trans
      (max_le ((totalDegree_add _ _).trans (max_le hxy (by simp))) (by simp))
  have hmul : ∀ i : Fin 2, ∀ g : R, g.totalDegree ≤ 2 → (X i*g).totalDegree ≤ 3 := by
    intro i g hg
    have hh := totalDegree_mul (X i:R) g
    simp only [totalDegree_X] at hh
    omega
  intro g hg
  simp only [controlRows,Set.mem_insert_iff,Set.mem_singleton_iff] at hg
  rcases hg with rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl
  · omega
  · omega
  · omega
  · exact hmul 0 e0 he0
  · exact hmul 1 e0 he0
  · exact hmul 0 e1 he1
  · exact hmul 1 e1 he1
  · exact hmul 0 f hf
  · exact hmul 1 f hf

-- Together with the pre-existing lower bound this gives a known threshold:
-- no GB in degree-two generator span, a GB in explicit degree-three rows.
theorem control_degree2_rejected :
    ¬ ∃ G : Set R, G ⊆ Submodule.span (ZMod 2) (F:Set R) ∧
      IsGB MonomialOrder.degLex G (Ideal.span (F:Set R)) :=
  toy_solving_degree_ge_three

end TheoremAudit
