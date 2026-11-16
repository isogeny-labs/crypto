module

public import AffineWalk

public section

namespace CycleCouplingAudit

theorem deterministic_never_mixes :
    ¬ ∃ T : ℕ, ∀ t : ℕ, T ≤ t →
      abs (expectation (fairLaw deterministicStep t (0 : ZMod 2))
        (eventIndicator {(t : ZMod 2)}) - (1 : ℝ)/2) ≤ 1/4 := by
  rintro ⟨T,h⟩
  have ht := h T le_rfl
  rw [deterministic_nonmixing] at ht
  norm_num at ht

/-- Actual independent laws, coupling, marginals and paper-normalized event bounds.
The qualified k=2/equalweight case is proved; no static rho or larger-support law is substituted. -/
theorem cycle_coupling_certificate :
    (fairCoin false = (2 : ENNReal)⁻¹ ∧ fairCoin true = (2 : ENNReal)⁻¹) ∧
    (∀ (n : ℕ) [NeZero n] (t : ℕ) (f : ZMod n → ℝ) (x : ZMod n),
      expectation (fairLaw walkStep t x) f = average walkStep t f x) ∧
    (∀ (n : ℕ) [NeZero n] (t : ℕ) (f : ZMod n → ℝ) (x y : ZMod n),
      average coupleStep t (fun s => f s.1) (x,y) = average walkStep t f x ∧
      average coupleStep t (fun s => f s.2) (x,y) = average walkStep t f y) ∧
    (∀ (n : ℕ) [NeZero n], 2 ≤ n → ∀ s : ZMod n × ZMod n,
      (pairPotential (coupleStep false s) + pairPotential (coupleStep true s))/2 =
        pairPotential s - mismatch s) ∧
    (∀ (n : ℕ) [NeZero n], 2 ≤ n → ∀ t : ℕ, 0 < t →
      ∀ (x : ZMod n) (A : Finset (ZMod n)),
      abs (walkProbability t x A - (A.card : ℝ)/n) ≤ (n : ℝ)^2/(4*t)) ∧
    (∀ (p : ℕ) [Fact p.Prime] (a b : ZMod p), a ≠ b →
      ∀ t : ℕ, 0 < t → ∀ (x : ZMod p) (A : Finset (ZMod p)),
      abs (twoProbability a b t x A - (A.card : ℝ)/p) ≤ (p : ℝ)^2/(4*t)) ∧
    (∀ t : ℕ, abs (expectation (fairLaw deterministicStep t (0 : ZMod 2))
      (eventIndicator {(t : ZMod 2)}) - (1 : ℝ)/2) = 1/2) ∧
    (¬ ∃ T : ℕ, ∀ t : ℕ, T ≤ t →
      abs (expectation (fairLaw deterministicStep t (0 : ZMod 2))
        (eventIndicator {(t : ZMod 2)}) - (1 : ℝ)/2) ≤ 1/4) ∧
    (¬ abs (walkProbability 0 (0 : ZMod 2) {0} - (1 : ℝ)/2) ≤ (2 : ℝ)^2/(4*0)) := by
  refine ⟨⟨rfl,rfl⟩,?_,?_,?_,?_,?_,deterministic_nonmixing,
    deterministic_never_mixes,time_zero_counter⟩
  · intro n _ t f x
    exact actual_expectation walkStep t f x
  · intro n _ t f x y
    exact ⟨left_margin t f x y,right_margin t f x y⟩
  · intro n _ hn s
    exact pair_drift hn s
  · intro n _ hn t ht x A
    exact cyclic_mixing_bound hn ht x A
  · intro p _ a b hab t ht x A
    exact two_support_mixing a b hab ht x A

#print axioms deterministic_never_mixes
#print axioms cycle_coupling_certificate

end CycleCouplingAudit
