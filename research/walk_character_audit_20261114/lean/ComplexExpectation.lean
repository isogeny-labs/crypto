module

public import WeightedKernel
public import Mathlib.Analysis.Fourier.FiniteAbelian.Orthogonality
public import Mathlib.Analysis.SpecialFunctions.Complex.CircleAddChar

@[expose] public section

namespace WalkCharacterAudit

open CycleCouplingAudit WeightedCycleCouplingAudit

noncomputable def complexExpectation {S : Type*} [Fintype S] (q : PMF S) (f : S → ℂ) : ℂ :=
  ∑ s, (weight q s : ℂ)*f s

theorem complex_expectation_re {S : Type*} [Fintype S] (q : PMF S) (f : S → ℂ) :
    (complexExpectation q f).re=expectation q (fun s => (f s).re) := by
  simp [complexExpectation,expectation,weight]

theorem complex_expectation_im {S : Type*} [Fintype S] (q : PMF S) (f : S → ℂ) :
    (complexExpectation q f).im=expectation q (fun s => (f s).im) := by
  simp [complexExpectation,expectation,weight]

theorem complex_expectation_pure {S : Type*} [Fintype S] (s : S) (f : S → ℂ) :
    complexExpectation (PMF.pure s) f=f s := by
  apply Complex.ext
  · rw [complex_expectation_re,expectation_pure]
  · rw [complex_expectation_im,expectation_pure]

theorem complex_expectation_bind {I S : Type*} [Fintype I] [Fintype S]
    (q : PMF I) (k : I → PMF S) (f : S → ℂ) :
    complexExpectation (q.bind k) f = ∑ i, (weight q i : ℂ)*complexExpectation (k i) f := by
  apply Complex.ext
  · simpa [complexExpectation,expectation,weight] using expectation_bind q k (fun s => (f s).re)
  · simpa [complexExpectation,expectation,weight] using expectation_bind q k (fun s => (f s).im)

theorem complex_weight_sum {I : Type*} [Fintype I] (q : PMF I) :
    ∑ i, (weight q i : ℂ)=1 := by exact_mod_cast weight_sum q

end WalkCharacterAudit
