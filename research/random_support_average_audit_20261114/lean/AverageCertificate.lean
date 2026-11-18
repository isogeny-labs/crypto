module

public import ZeroWeightControl

public section

namespace RandomSupportAverageAudit

open Real SupportGaussianAudit InjectiveBoxAudit HildebrandCellAudit
open WalkCharacterAudit WeightedCycleCouplingAudit CycleCouplingAudit FiniteFourierUpperBoundAudit

theorem full_support_pointmass {p d : ℕ} [Fact p.Prime] (h : d ≤ p-1)
    (a : Sample (ZMod p) (d+1)) :
    (fullSupportLaw p d h a).toReal=1/(p.descFactorial (d+1) : ℝ) := by
  unfold fullSupportLaw
  rw [sample_law_pointmass,ZMod.card]

/-- Qualified Hildebrand expected random-support mixing with actual normalized
walk/support laws, exact floor and exponent, and weight-deletion controls. -/
theorem random_support_average_certificate :
    (∀ (p d : ℕ) [Fact p.Prime] (h : d ≤ p-1) (a : Sample (ZMod p) (d+1)),
      (fullSupportLaw p d h a).toReal=1/(p.descFactorial (d+1) : ℝ)) ∧
    (∀ (p d : ℕ) [Fact p.Prime], Function.Bijective
      (allSupportEquiv : (ZMod p × Sample (Choice p) d) ≃ Sample (ZMod p) (d+1))) ∧
    (∀ (p d : ℕ) [Fact p.Prime] (h : d ≤ p-1) (F : Sample (ZMod p) (d+1) → ℝ),
      expectation (fullSupportLaw p d h) F =
        expectation (uniformLaw (ZMod p)) (fun a₀ =>
          expectation (sampleLaw (Choice p) d (Eq.mpr (congrArg (fun n => d ≤ n) choice_card) h))
            (fun e => F (allSupportEquiv (a₀,e))))) ∧
    (∀ (p : ℕ) [Fact p.Prime], Function.Bijective (frequencyCharacter p)) ∧
    (∀ (p d : ℕ) [Fact p.Prime] (h : d ≤ p-1) (q : PMF (Fin (d+1)))
      (ω : ℝ), 0<ω → (∀ i, ω ≤ weight q i) → ∀ (t : ℕ), 0<t → ∀ x : ZMod p,
      (expectedDistance p d h q t x)^2 ≤ expectedSquaredDistance p d h q t x ∧
      expectedSquaredDistance p d h q t x ≤
        ((p-1 : ℕ) : ℝ)/4 *
        (sqrt (Real.pi/((8*ω^2*(t : ℝ))/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ)) ∧
    (∀ (d : ℕ), 1 ≤ d → ∀ (q : PMF (Fin (d+1))), (∀ i, 0<weight q i) →
      ∀ ε : ℝ, 0<ε → ∃ γ : ℝ, 0<γ ∧ ∃ N : ℕ, d+1 ≤ N ∧
      ∀ (p : ℕ) [Fact p.Prime] (h : d ≤ p-1), N ≤ p → ∀ x : ZMod p,
        expectedDistance p d h q ⌊γ*(p : ℝ)^((2 : ℝ)/(d : ℝ))⌋₊ x<ε) ∧
    (∀ (p d : ℕ) [Fact p.Prime] (h : d ≤ p-1) (t : ℕ) (x : ZMod p),
      (1/2 : ℝ) ≤ expectedDistance p d h (PMF.pure (0 : Fin (d+1))) t x) ∧
    (∀ d : ℕ, ¬ ∃ γ : ℝ, 0<γ ∧ ∃ N : ℕ, d+1 ≤ N ∧
      ∀ (p : ℕ) [Fact p.Prime] (h : d ≤ p-1), N ≤ p → ∀ x : ZMod p,
        expectedDistance p d h (PMF.pure (0 : Fin (d+1)))
          ⌊γ*(p : ℝ)^((2 : ℝ)/(d : ℝ))⌋₊ x<(1/4 : ℝ)) := by
  refine ⟨@full_support_pointmass,?_,@full_support_expectation,@frequency_index_bijective,?_,?_,
    @zero_weight_average_lower,strict_weight_deletion_false⟩
  · intro p d _
    exact allSupportEquiv.bijective
  · intro p d _ h q ω hω hw t ht x
    exact ⟨expected_distance_square_bound h q t x,finite_expected_squared_bound h q hω hw t ht x⟩
  · intro d hd q hq ε hε
    exact qualified_expected_support_mixing hd q hq hε

#print axioms random_support_average_certificate

end RandomSupportAverageAudit
