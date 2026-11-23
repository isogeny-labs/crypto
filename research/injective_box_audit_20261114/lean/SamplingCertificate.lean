module

public import PrimeFieldBoxes

public section

namespace InjectiveBoxAudit

open HildebrandCellAudit

/-- Exact corrected finite counting core. No asymptotic approximation or independence
of coordinates is assumed; actual source laws and deleted-index controls are bound. -/
theorem injective_box_certificate :
    (∀ (T : Type) [Fintype T] [Nonempty T] (A : Finset T),
      uniformEvent A=(A.card : ℝ)/(Fintype.card T : ℝ) ∧ 0 ≤ uniformEvent A ∧ uniformEvent A ≤ 1) ∧
    (∀ (X : Type) [Fintype X] (d : ℕ),
      Fintype.card (Sample X d)=(Fintype.card X).descFactorial d) ∧
    (∀ (X : Type) [Fintype X] (d : ℕ) (h : d ≤ Fintype.card X),
      (∀ e : Sample X d, (sampleLaw X d h e).toReal=1/((Fintype.card X).descFactorial d : ℝ)) ∧
      ∀ B : Fin d → Finset X,
      boxProbability X d h B=((boxEvent B).card : ℝ)/((Fintype.card X).descFactorial d : ℝ) ∧
      boxProbability X d h B ≤ (∏ i, (B i).card : ℕ)/((Fintype.card X).descFactorial d : ℝ)) ∧
    (∀ (X : Type) [Fintype X] (d : ℕ), Fintype.card X<d → IsEmpty (PMF (Sample X d))) ∧
    (∀ (S : Finset ℤ) (L U : ℝ), L ≤ U →
      (∀ z ∈ S, L ≤ (z : ℝ) ∧ (z : ℝ) ≤ U) → (S.card : ℝ) ≤ U-L+1) ∧
    (∀ (p d : ℕ) [Fact p.Prime] (a₀ : ZMod p) (h : d ≤ p-1),
      Fintype.card (FullSupports p d a₀)=(p-1).descFactorial d ∧
      sourceLaw p d a₀ h=@uniformLaw (FullSupports p d a₀) inferInstance (full_support_nonempty a₀ h)) ∧
    (∀ (p d : ℕ) [Fact p.Prime] (a₀ j : ZMod p), j ≠ 0 → ∀ h : d ≤ p-1,
      ∀ (L U : Fin d → ℝ), (∀ i, L i ≤ U i) →
      sourceProbability p d a₀ j h L U ≤
        (∏ i, ((p : ℝ)*(U i-L i)+1))/((p-1).descFactorial d : ℝ)) ∧
    (∀ (p : ℕ) [Fact p.Prime], 3 ≤ p →
      (coordinateBox (0 : ZMod p) 0 0).card=p-1 ∧
      ¬ ((coordinateBox (0 : ZMod p) 0 0).card : ℝ) ≤ (p : ℝ)*(0-0)+1) ∧
    (∀ (p : ℕ) [Fact p.Prime] (hp : 3 ≤ p) (a₀ : ZMod p),
      sourceProbability p 1 a₀ 0 (one_dimension_fits hp) (fun _ => 0) (fun _ => 0)=1 ∧
      ¬ sourceProbability p 1 a₀ 0 (one_dimension_fits hp) (fun _ => 0) (fun _ => 0) ≤
        1/((p-1 : ℕ) : ℝ)) := by
  refine ⟨?_,?_,?_,?_,integer_set_card_bound,?_,?_,?_,?_⟩
  · intro T _ _ A
    exact ⟨uniform_event_formula A,uniform_event_nonneg A,uniform_event_le_one A⟩
  · intro X _ d
    exact sample_card
  · intro X _ d h
    exact ⟨sample_law_pointmass h,fun B => ⟨box_probability_exact h B,box_probability_bound h B⟩⟩
  · intro X _ d h
    exact no_sampler_if_too_many h
  · intro p d _ a₀ h
    exact ⟨full_support_card a₀,actual_uniform_source_law a₀ h⟩
  · intro p d _ a₀ j hj h L U hLU
    exact source_probability_bound a₀ j hj h L U hLU
  · intro p _ hp
    exact ⟨zero_index_card,zero_index_bound_false hp⟩
  · intro p _ hp a₀
    exact ⟨zero_index_probability a₀ (one_dimension_fits hp),zero_index_probability_bound_false hp a₀⟩

#print axioms injective_box_certificate

end InjectiveBoxAudit
