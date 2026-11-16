import Mathlib
open scoped BigOperators
namespace CompiledMultiplication
 theorem dual_contraction {R J K : Type*} [CommRing R] [Fintype J] [Fintype K]
    (a : K → R) (u : J → R) (m : J → K → R) :
    (∑ k, a k * (∑ j, u j * m j k)) =
      ∑ j, u j * (∑ k, a k * m j k) := by
  calc
    (∑ k, a k * (∑ j, u j * m j k)) =
        ∑ k, ∑ j, a k * (u j * m j k) := by simp_rw [Finset.mul_sum]
    _ = ∑ j, ∑ k, a k * (u j * m j k) := Finset.sum_comm
    _ = ∑ j, u j * (∑ k, a k * m j k) := by
      apply Finset.sum_congr rfl
      intro j _
      rw [Finset.mul_sum]
      apply Finset.sum_congr rfl
      intro k _
      ring
end CompiledMultiplication
#print axioms CompiledMultiplication.dual_contraction
