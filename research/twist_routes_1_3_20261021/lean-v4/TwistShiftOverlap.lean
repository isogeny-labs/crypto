import Mathlib.Algebra.BigOperators.Group.Finset.Basic
import Mathlib.Algebra.BigOperators.Ring.Finset
import Mathlib.Data.Fintype.Card
import Mathlib.Tactic.Ring
import Mathlib.Tactic.NormNum

/- Exact finite sign/permutation identity. The Legendre-symbol, projective
   genus-two and Weil-bound bridges are deliberately not claimed here. -/
namespace TwistShiftOverlap
open scoped BigOperators

theorem mask_expansion {K : Type*} [CommRing K] (x y : K) :
    (1-x)*(1+y) = 1-x+y-x*y := by ring

theorem sign_mask (x y : ℤ) (hx : x = 1 ∨ x = -1)
    (hy : y = 1 ∨ y = -1) :
    (1-x)*(1+y) = 4*(if x = -1 ∧ y = 1 then 1 else 0) := by
  rcases hx with hx | hx <;> rcases hy with hy | hy <;> simp [hx, hy]

theorem finite_overlap {ι : Type*} [Fintype ι] (χ : ι → ℤ)
    (σ : Equiv.Perm ι) (hχ : ∀ i, χ i = 1 ∨ χ i = -1) :
    4*(∑ i, if χ i = -1 ∧ χ (σ i) = 1 then (1 : ℤ) else 0) =
      (Fintype.card ι : ℤ) - ∑ i, χ i * χ (σ i) := by
  classical
  rw [Finset.mul_sum]
  calc
    (∑ i, 4*(if χ i = -1 ∧ χ (σ i) = 1 then (1 : ℤ) else 0)) =
        ∑ i, (1-χ i)*(1+χ (σ i)) := by
      apply Finset.sum_congr rfl
      intro i _
      exact (sign_mask (χ i) (χ (σ i)) (hχ i) (hχ (σ i))).symm
    _ = ∑ i, (1-χ i+χ (σ i)-χ i*χ (σ i)) := by
      apply Finset.sum_congr rfl
      intro i _
      exact mask_expansion (χ i) (χ (σ i))
    _ = (Fintype.card ι : ℤ) - ∑ i, χ i*χ (σ i) := by
      simp only [Finset.sum_sub_distrib, Finset.sum_add_distrib]
      rw [Equiv.sum_comp (σ : ι ≃ ι) χ]
      simp

end TwistShiftOverlap
