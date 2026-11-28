module

public import DiscreteGaussian
public import InjectiveProducts
public import CenteredContraction

public section

namespace SupportGaussianAudit

open Real InjectiveBoxAudit WalkCharacterAudit WeightedCycleCouplingAudit

/-- Actual convergent Gaussian counter to a printed intermediate equality,
its valid repair, and actual without-replacement and walk-moment bounds. -/
theorem support_gaussian_certificate :
    (∀ a : ℝ, 0<a → Summable (gaussianTerm a) ∧
      Summable (fun v : ℕ × ℕ => exp (-a*((v.1 : ℝ)^2+(v.2 : ℝ)^2))) ∧
      doubleGaussian a=(theta a)^2 ∧ doubleGaussian a<1/(1-exp (-a))^2 ∧
      ¬ doubleGaussian a=1/(1-exp (-a))^2) ∧
    (∀ (p t : ℕ) (ε c B : ℝ), 0<p → 0<t → 0<ε → 0<c →
      printedExponentThree p t ε c=(t : ℝ)*c*(ε/2)/(4*(p : ℝ)) ∧
      0<printedExponentThree p t ε c ∧
      B+printedSeriesCoefficient p ε*doubleGaussian (printedExponentThree p t ε c) <
        B+printedSeriesCoefficient p ε/(1-exp (-printedExponentThree p t ε c))^2 ∧
      B+printedSeriesCoefficient p ε*doubleGaussian (printedExponentThree p t ε c) ≤
        B+printedSeriesCoefficient p ε/(1-exp (-printedExponentThree p t ε c))^2 ∧
      ¬ B+printedSeriesCoefficient p ε*doubleGaussian (printedExponentThree p t ε c) =
        B+printedSeriesCoefficient p ε/(1-exp (-printedExponentThree p t ε c))^2) ∧
    (∀ a : ℝ, 0<a → Summable (gaussianInteger a) ∧
      (∑' z : ℤ, gaussianInteger a z) ≤ sqrt (Real.pi/a) ∧
      ∀ S : Finset ℤ, 0 ∉ S → (∑ z ∈ S, exp (-a*(z : ℝ)^2)) ≤ sqrt (Real.pi/a)) ∧
    (¬ Summable (gaussianTerm 0)) ∧
    (¬ (∑ z ∈ ({1} : Finset ℤ), exp (-(0 : ℝ)*(z : ℝ)^2)) ≤ sqrt (Real.pi/(0 : ℝ))) ∧
    (∀ (X : Type) [Fintype X] (d : ℕ) (h : d ≤ Fintype.card X)
      (H : Fin d → X → ℝ), (∀ i x, 0 ≤ H i x) →
      injectiveProductExpectation X d h H =
        (∑ e : Sample X d, ∏ i, H i (e i))/((Fintype.card X).descFactorial d : ℝ) ∧
      injectiveProductExpectation X d h H ≤
        (∏ i, ∑ x, H i x)/((Fintype.card X).descFactorial d : ℝ)) ∧
    (∀ (I : Type) [Fintype I] [Nonempty I] (q : PMF I), (∀ i, 0<weight q i) →
      ∃ ω : ℝ, 0<ω ∧ (∀ i, ω ≤ weight q i) ∧
      ∀ (p : ℕ) [Fact p.Prime] (a : I → ZMod p) (r : I) (j : ZMod p)
        (t : ℕ) (x : ZMod p),
      Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x)
        (frequencyCharacter p j)) ≤ exp (-(8*ω^2*(t : ℝ))*centeredRadius a r j)) := by
  classical
  refine ⟨?_,?_,?_,gaussian_zero_not_summable,zero_exponent_finite_bound_false,?_,
    @uniform_weight_centered_decay⟩
  · intro a ha
    have hs := double_gaussian_strict ha
    rw [geometricClosed,div_pow,one_pow] at hs
    exact ⟨gaussian_summable ha,double_gaussian_summable ha,double_gaussian_factorization ha,
      hs,literal_gaussian_geometric_equality_false ha⟩
  · intro p t ε c B hp ht hε hc
    have hs := printed_series_step_strict hp ht hε hc B
    exact ⟨printed_exponent_three p t ε c,printed_exponent_positive hp ht hε hc,
      hs,hs.le,printed_series_equality_false hp ht hε hc B⟩
  · intro a ha
    exact ⟨gaussian_integer_summable ha,gaussian_integer_bound ha,finite_integer_gaussian_bound ha⟩
  · intro X _ d h H hH
    exact ⟨injective_product_expectation_exact h H,injective_product_expectation_bound h H hH⟩

#print axioms support_gaussian_certificate

end SupportGaussianAudit
