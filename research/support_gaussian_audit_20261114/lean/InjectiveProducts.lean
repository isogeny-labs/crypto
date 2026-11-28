module

public import InjectiveSampling
public import Mathlib.Algebra.BigOperators.Ring.Finset

@[expose] public section

namespace SupportGaussianAudit

open InjectiveBoxAudit CycleCouplingAudit

theorem sample_to_function_injective {X : Type*} {d : ℕ} :
    Function.Injective (fun e : Sample X d => (fun i => e i)) := by
  intro e f h
  apply Function.Embedding.ext
  intro i
  exact congrFun h i

theorem injective_product_sum_bound {X : Type*} [Fintype X] {d : ℕ}
    (H : Fin d → X → ℝ) (hH : ∀ i x, 0 ≤ H i x) :
    (∑ e : Sample X d, ∏ i, H i (e i)) ≤ ∏ i, ∑ x, H i x := by
  classical
  calc
    _ = ∑ f ∈ (Finset.univ.image (fun e : Sample X d => (fun i => e i))), ∏ i, H i (f i) :=
      (Finset.sum_image (fun _ _ _ _ h => sample_to_function_injective h)).symm
    _ ≤ ∑ f : Fin d → X, ∏ i, H i (f i) :=
      Finset.sum_le_sum_of_subset_of_nonneg (Finset.subset_univ _)
        (fun f _ _ => Finset.prod_nonneg (fun i _ => hH i (f i)))
    _ = _ := (Fintype.prod_sum H).symm

noncomputable def injectiveProductExpectation (X : Type*) [Fintype X] (d : ℕ)
    (h : d ≤ Fintype.card X) (H : Fin d → X → ℝ) : ℝ :=
  expectation (sampleLaw X d h) (fun e => ∏ i, H i (e i))

theorem injective_product_expectation_exact {X : Type*} [Fintype X] {d : ℕ}
    (h : d ≤ Fintype.card X) (H : Fin d → X → ℝ) :
    injectiveProductExpectation X d h H =
      (∑ e : Sample X d, ∏ i, H i (e i))/((Fintype.card X).descFactorial d : ℝ) := by
  unfold injectiveProductExpectation expectation
  simp_rw [sample_law_pointmass h]
  rw [← Finset.mul_sum]
  ring

theorem injective_product_expectation_bound {X : Type*} [Fintype X] {d : ℕ}
    (h : d ≤ Fintype.card X) (H : Fin d → X → ℝ) (hH : ∀ i x, 0 ≤ H i x) :
    injectiveProductExpectation X d h H ≤
      (∏ i, ∑ x, H i x)/((Fintype.card X).descFactorial d : ℝ) := by
  rw [injective_product_expectation_exact]
  exact div_le_div_of_nonneg_right (injective_product_sum_bound H hH) (by positivity)

end SupportGaussianAudit
