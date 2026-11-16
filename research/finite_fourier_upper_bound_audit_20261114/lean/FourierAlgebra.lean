module

public import CharacterCertificate
public import UniformChoices
public import Mathlib.Analysis.Fourier.FiniteAbelian.PontryaginDuality
public import Mathlib.Algebra.Order.BigOperators.Ring.Finset

@[expose] public section

namespace FiniteFourierUpperBoundAudit

open WalkCharacterAudit HildebrandCellAudit WeightedCycleCouplingAudit
open scoped ComplexConjugate

noncomputable def rawFourier {G : Type*} [Fintype G] [AddCommGroup G]
    (f : G → ℂ) (ψ : AddChar G ℂ) : ℂ := ∑ x, f x*ψ x

theorem character_sub_conjugate {G : Type*} [AddCommGroup G] [Finite G]
    (ψ : AddChar G ℂ) (x y : G) : ψ (x-y)=ψ x*conj (ψ y) := by
  rw [sub_eq_add_neg,ψ.map_add_eq_mul,ψ.map_neg_eq_conj]

theorem fourier_product_expansion {G : Type*} [Fintype G] [AddCommGroup G]
    (f : G → ℂ) (ψ : AddChar G ℂ) :
    rawFourier f ψ*conj (rawFourier f ψ) =
      ∑ x, ∑ y, (f x*conj (f y))*ψ (x-y) := by
  simp only [rawFourier,map_sum,map_mul,Finset.sum_mul,Finset.mul_sum,character_sub_conjugate]
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro x _
  apply Finset.sum_congr rfl
  intro y _
  ring

theorem finite_plancherel_complex {G : Type*} [Fintype G] [AddCommGroup G] (f : G → ℂ) :
    (∑ ψ : AddChar G ℂ, rawFourier f ψ*conj (rawFourier f ψ)) =
      (Fintype.card G : ℂ)*(∑ x, f x*conj (f x)) := by
  classical
  simp_rw [fourier_product_expansion]
  calc
    _ = ∑ x, ∑ y, ∑ ψ : AddChar G ℂ, (f x*conj (f y))*ψ (x-y) := by
      rw [Finset.sum_comm]
      apply Finset.sum_congr rfl
      intro x _
      rw [Finset.sum_comm]
    _ = ∑ x, ∑ y, (f x*conj (f y))*(∑ ψ : AddChar G ℂ, ψ (x-y)) := by
      simp only [← Finset.mul_sum]
    _ = ∑ x, (f x*conj (f x))*(Fintype.card G : ℂ) := by
      simp [AddChar.sum_apply_eq_ite,sub_eq_zero,mul_ite]
    _ = _ := by rw [← Finset.sum_mul,mul_comm]

theorem finite_plancherel {G : Type*} [Fintype G] [AddCommGroup G] (f : G → ℂ) :
    (∑ ψ : AddChar G ℂ, Complex.normSq (rawFourier f ψ)) =
      (Fintype.card G : ℝ)*(∑ x, Complex.normSq (f x)) := by
  have he := congrArg Complex.re (finite_plancherel_complex f)
  simpa [Complex.mul_conj] using he

theorem uniform_character_coefficient {G : Type*} [Fintype G] [AddCommGroup G] (ψ : AddChar G ℂ) :
    complexExpectation (uniformLaw G) ψ = if ψ=0 then 1 else 0 := by
  classical
  by_cases hψ : ψ=0
  · simp [complexExpectation,hψ,complex_weight_sum]
  · unfold complexExpectation
    simp only [weight,uniform_real]
    rw [← Finset.mul_sum,AddChar.sum_eq_ite]
    simp [hψ]

theorem difference_fourier {G : Type*} [Fintype G] [AddCommGroup G]
    (q r : PMF G) (ψ : AddChar G ℂ) :
    rawFourier (fun x => (weight q x : ℂ)-(weight r x : ℂ)) ψ =
      complexExpectation q ψ-complexExpectation r ψ := by
  simp only [rawFourier,complexExpectation,sub_mul,Finset.sum_sub_distrib]

end FiniteFourierUpperBoundAudit
