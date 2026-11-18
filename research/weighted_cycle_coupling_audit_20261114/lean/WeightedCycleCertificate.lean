module

public import SourceAndControl

public section

namespace WeightedCycleCouplingAudit

open CycleCouplingAudit

/-- Qualified uniform-support quadratic mixing, with an actual prime-deletion counterexample.
The stronger random-support exponent and static rho/collision laws are separate. -/
theorem weighted_cycle_certificate :
    (∀ (I : Type) [Fintype I] (q : PMF I),
      labelPair q = q.bind (fun i => q.map (fun j => (i,j)))) ∧
    (∀ (I : Type) [Fintype I] (n : ℕ) [NeZero n] (q : PMF I) (g : I → ZMod n)
      (t : ℕ) (f : ZMod n → ℝ) (x : ZMod n),
      expectation (kernelLaw q (incrementStep g) t x) f = weightedAverage q (incrementStep g) t f x) ∧
    (∀ (I : Type) [Fintype I] (n : ℕ) (q : PMF I) (g : I → ZMod n) (a b : I)
      (t : ℕ) (f : ZMod n → ℝ) (x y : ZMod n),
      weightedAverage (labelPair q) (coupledLabelStep g a b) t (fun s => f s.1) (x,y) =
        weightedAverage q (incrementStep g) t f x ∧
      weightedAverage (labelPair q) (coupledLabelStep g a b) t (fun s => f s.2) (x,y) =
        weightedAverage q (incrementStep g) t f y) ∧
    (∀ (I : Type) [Fintype I] (p : ℕ) [Fact p.Prime] (q : PMF I)
      (g : I → ZMod p) (a b : I), g a ≠ g b → ∀ s : ZMod p × ZMod p,
      pairMean q (fun z => normalizedPotential g a b (coupledLabelStep g a b z s)) =
        normalizedPotential g a b s-(2*weight q a*weight q b)*mismatch s) ∧
    (∀ (I : Type) [Fintype I] (p : ℕ) [Fact p.Prime] (q : PMF I)
      (g : I → ZMod p) (a b : I), 0 < weight q a → 0 < weight q b → g a ≠ g b →
      ∀ t : ℕ, 0 < t → ∀ (x : ZMod p) (A : Finset (ZMod p)),
      abs (probability q g t x A-(A.card : ℝ)/p) ≤ (p : ℝ)^2/(8*weight q a*weight q b*t)) ∧
    (∀ (I : Type) [Fintype I] [Nontrivial I] (w : I → NNReal) (hw : ∑ i, w i = 1),
      (∀ i, 0 < w i) → ∀ ε : ℝ, 0 < ε → ∃ γ : ℝ, 0 < γ ∧
      ∀ (p : ℕ) [Fact p.Prime] (g : I → ZMod p), Function.Injective g →
      ∀ t : ℕ, γ*(p : ℝ)^2 ≤ t → ∀ (x : ZMod p) (A : Finset (ZMod p)),
      abs (probability (labelLaw w hw) g t x A-(A.card : ℝ)/p) < ε) ∧
    Function.Injective compositeIncrements ∧
    (∀ i : Bool, weight fairCoin i = 1/2) ∧
    evenEvent.card = 2 ∧
    (∀ t : ℕ, abs (probability fairCoin compositeIncrements t 0 evenEvent-
      (evenEvent.card : ℝ)/4) = 1/2) ∧
    (¬ ∃ γ : ℝ, 0 < γ ∧ ∀ t : ℕ, γ*(4 : ℝ)^2 ≤ t →
      ∀ (x : ZMod 4) (A : Finset (ZMod 4)),
      abs (probability fairCoin compositeIncrements t x A-(A.card : ℝ)/4) < 1/4) := by
  refine ⟨?_,?_,?_,?_,?_,?_,composite_distinct,fair_weights,?_,
    composite_event_distance,composite_no_quadratic_constant⟩
  · intro I _ q
    exact label_pair_eq_bind q
  · intro I _ n _ q g t f x
    exact actual_weighted_expectation q (incrementStep g) t f x
  · intro I _ n q g a b t f x y
    exact ⟨weighted_left_margin q g a b t f x y,weighted_right_margin q g a b t f x y⟩
  · intro I _ p _ q g a b hg s
    exact normalized_drift q g a b hg s
  · intro I _ p _ q g a b ha hb hg t ht x A
    exact prime_weighted_mixing q g a b ha hb hg ht x A
  · intro I _ _ w hw hp ε hε
    exact hildebrand_theorem_two w hw hp ε hε
  · decide +kernel

#print axioms weighted_cycle_certificate

end WeightedCycleCouplingAudit
