module

public import NormalizationControl

public section

namespace FiniteFourierUpperBoundAudit

open WalkCharacterAudit HildebrandCellAudit WeightedCycleCouplingAudit

/-- Finite-abelian Upper Bound Lemma from derived character Plancherel and
actual PMF distances, with the actual walk law and sharp normalization control. -/
theorem finite_fourier_upper_bound_certificate :
    (∀ (G : Type) [Fintype G] [AddCommGroup G] (f : G → ℂ),
      (∑ ψ : AddChar G ℂ, Complex.normSq (rawFourier f ψ)) =
        (Fintype.card G : ℝ)*(∑ x, Complex.normSq (f x))) ∧
    (∀ (S : Type) [Fintype S] (q r : PMF S),
      totalVariation q r=l1Distance q r/2 ∧ 0 ≤ totalVariation q r ∧ totalVariation q r ≤ 1 ∧
      (∀ A : Finset S, abs (eventMass q A-eventMass r A) ≤ totalVariation q r) ∧
      eventMass q (positiveEvent q r)-eventMass r (positiveEvent q r)=totalVariation q r) ∧
    (∀ (G : Type) [Fintype G] [AddCommGroup G] (q r : PMF G),
      differenceEnergy q r=(Fintype.card G : ℝ)*(∑ x, (weight q x-weight r x)^2) ∧
      (l1Distance q r)^2 ≤ differenceEnergy q r ∧
      (totalVariation q r)^2 ≤ (1/4 : ℝ)*differenceEnergy q r) ∧
    (∀ (G : Type) [Fintype G] [AddCommGroup G] (x : G),
      (uniformLaw G x).toReal=1/(Fintype.card G : ℝ)) ∧
    (∀ (G : Type) [Fintype G] [AddCommGroup G] (A : Finset G),
      eventMass (uniformLaw G) A=(A.card : ℝ)/(Fintype.card G : ℝ)) ∧
    (∀ (G : Type) [Fintype G] [AddCommGroup G] (ψ : AddChar G ℂ),
      (ψ=0 → complexExpectation (uniformLaw G) ψ=1) ∧
      (ψ ≠ 0 → complexExpectation (uniformLaw G) ψ=0)) ∧
    (∀ (G : Type) [Fintype G] [AddCommGroup G] (q : PMF G),
      differenceEnergy q (uniformLaw G)=nontrivialEnergy q ∧
      (totalVariation q (uniformLaw G))^2 ≤ (1/4 : ℝ)*nontrivialEnergy q) ∧
    (∀ (I G : Type) [Fintype I] [Fintype G] [AddCommGroup G]
      (q : PMF I) (a : I → G) (t : ℕ) (x : G),
      (totalVariation (kernelLaw q (fun i y => y+a i) t x) (uniformLaw G))^2 ≤
        (1/4 : ℝ)*walkSpectralEnergy q a t ∧
      ∀ A : Finset G,
      (abs (eventMass (kernelLaw q (fun i y => y+a i) t x) A-(A.card : ℝ)/(Fintype.card G : ℝ)))^2 ≤
        (1/4 : ℝ)*walkSpectralEnergy q a t) ∧
    (l1Distance (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2))=1 ∧
      totalVariation (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2))=1/2 ∧
      nontrivialEnergy (PMF.pure (0 : ZMod 2))=1) ∧
    ((totalVariation (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2)))^2 =
      (1/4 : ℝ)*nontrivialEnergy (PMF.pure (0 : ZMod 2))) ∧
    (¬ (l1Distance (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2)))^2 ≤
      (1/4 : ℝ)*nontrivialEnergy (PMF.pure (0 : ZMod 2))) ∧
    (∀ c : ℝ, (∀ q : PMF (ZMod 2), (totalVariation q (uniformLaw (ZMod 2)))^2 ≤
      c*nontrivialEnergy q) → (1/4 : ℝ) ≤ c) := by
  classical
  refine ⟨@finite_plancherel,?_,?_,?_,@uniform_event_mass,?_,?_,?_,
    ⟨two_point_full_distance,two_point_half_distance,two_point_spectral_energy⟩,
    correct_normalization_equality,mixed_normalization_bound_false,quarter_constant_necessary⟩
  · intro S _ q r
    exact ⟨rfl,total_variation_nonneg q r,total_variation_le_one q r,
      event_difference_le_total_variation q r,positive_event_attains q r⟩
  · intro G _ _ q r
    exact ⟨difference_energy_parseval q r,l1_fourier_bound q r,total_variation_fourier_bound q r⟩
  · intro G _ _ x
    exact uniform_real G x
  · intro G _ _ ψ
    constructor
    · intro h
      rw [uniform_character_coefficient]
      simp [h]
    · intro h
      rw [uniform_character_coefficient]
      simp [h]
  · intro G _ _ q
    exact ⟨uniform_difference_energy q,uniform_total_variation_bound q⟩
  · intro I G _ _ _ q a t x
    exact ⟨actual_walk_upper_bound q a t x,actual_walk_event_upper_bound q a t x⟩

#print axioms finite_fourier_upper_bound_certificate

end FiniteFourierUpperBoundAudit
