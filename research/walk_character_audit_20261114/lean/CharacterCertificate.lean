module

public import SpectralContraction

public section

namespace WalkCharacterAudit

open CycleCouplingAudit WeightedCycleCouplingAudit

/-- Actual iid-walk Fourier law and quantitative phase contraction, with
the zero-frequency, deleted-positive-weight and deleted-prime controls. -/
theorem walk_character_certificate :
    (∀ (I S : Type) [Fintype I] [Fintype S] (q : PMF I) (k : I → PMF S) (f : S → ℂ),
      complexExpectation (q.bind k) f=∑ i, (weight q i : ℂ)*complexExpectation (k i) f) ∧
    (∀ (I G : Type) [Fintype I] [Fintype G] [AddCommGroup G]
      (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) (t : ℕ) (x : G),
      complexExpectation (kernelLaw q (fun i y => y+a i) t x) ψ=(characterEigenvalue q a ψ)^t*ψ x ∧
      Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x) ψ)=
        (Complex.normSq (characterEigenvalue q a ψ))^t) ∧
    (∀ (I : Type) [Fintype I] (q : PMF I) (z : I → ℂ),
      (∀ i, Complex.normSq (z i)=1) →
      1-Complex.normSq (complexExpectation q z)=
        (1/2 : ℝ)*(∑ i, ∑ j, weight q i*weight q j*Complex.normSq (z i-z j))) ∧
    (∀ (I G : Type) [Fintype I] [AddCommGroup G] [Finite G]
      (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) (i j : I), i ≠ j →
      Complex.normSq (characterEigenvalue q a ψ) ≤
        1-2*weight q i*weight q j*(1-(ψ (a i-a j)).re)) ∧
    (∀ (I G : Type) [Fintype I] [AddCommGroup G] [Finite G]
      (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) (i j : I), i ≠ j →
      0<weight q i → 0<weight q j → ∀ δ : ℝ,
      1-2*weight q i*weight q j*δ<Complex.normSq (characterEigenvalue q a ψ) →
        1-δ<(ψ (a i-a j)).re) ∧
    (∀ (p : ℕ) [NeZero p] (j x : ZMod p),
      (frequencyCharacter p j x).re=Real.cos (2*Real.pi*((x*j).val : ℝ)/(p : ℝ))) ∧
    (∀ (I : Type) [Fintype I] (p : ℕ) [Fact p.Prime]
      (q : PMF I) (a : I → ZMod p), Function.Injective a → ∀ j : ZMod p, j ≠ 0 →
      ∀ (u v : I), u ≠ v → 0<weight q u → 0<weight q v →
      Complex.normSq (characterEigenvalue q a (frequencyCharacter p j))<1) ∧
    (∀ (I : Type) [Fintype I] (p : ℕ) [NeZero p] (q : PMF I) (a : I → ZMod p),
      characterEigenvalue q a (frequencyCharacter p 0)=1 ∧ ∀ (t : ℕ) (x : ZMod p),
      Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x) (frequencyCharacter p 0))=1) ∧
    (weight (PMF.pure false) false=1 ∧ weight (PMF.pure false) true=0) ∧
    (∀ (p : ℕ) [Fact p.Prime], Function.Injective (twoSupports p) ∧
      characterEigenvalue (PMF.pure false) (twoSupports p) (frequencyCharacter p 1)=1 ∧
      ¬ Complex.normSq (characterEigenvalue (PMF.pure false) (twoSupports p) (frequencyCharacter p 1))<1) ∧
    ((∀ b : Bool, 0<weight fairCoin b) ∧ Function.Injective evenSupports ∧ (2 : ZMod 4) ≠ 0 ∧
      characterEigenvalue fairCoin evenSupports (frequencyCharacter 4 2)=1) := by
  refine ⟨@complex_expectation_bind,?_,@unit_phase_variance,@actual_character_pair_bound,
    @actual_character_phase_alignment,@frequency_character_cosine,?_,?_,pure_false_weights,
    zero_weight_counter,⟨fair_weights_positive,composite_frequency_counter⟩⟩
  · intro I G _ _ _ q a ψ t x
    exact ⟨actual_character_walk q a ψ t x,actual_character_squared_moment q a ψ t x⟩
  · intro I _ p _ q a ha j hj u v huv hu hv
    exact prime_frequency_strict q a ha j hj u v huv hu hv
  · intro I _ p _ q a
    refine ⟨zero_frequency_eigenvalue q a,?_⟩
    intro t x
    rw [actual_character_squared_moment,zero_frequency_eigenvalue]
    simp

#print axioms walk_character_certificate

end WalkCharacterAudit
