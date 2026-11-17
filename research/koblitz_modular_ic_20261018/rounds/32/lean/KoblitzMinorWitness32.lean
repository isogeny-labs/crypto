import Mathlib.Algebra.BigOperators.Ring.Finset
import Mathlib.Algebra.Ring.Hom.Defs

/-! A unit-ideal witness survives every field-valued branch. These lemmas
do not import the native Jacobian arrays or prove the scheme's dimension. -/
open scoped BigOperators
namespace KoblitzMinorWitness32
variable {I R K : Type*} [Fintype I] [CommRing R] [CommRing K] [Nontrivial K]

theorem specialize_unit_identity (a m : I → R) (φ : R →+* K)
    (h : ∑ i, a i * m i = 1) : ∑ i, φ (a i) * φ (m i) = 1 := by
  simpa only [map_sum, map_mul, map_one] using congrArg φ h

theorem some_minor_nonzero_on_every_branch (a m : I → R) (φ : R →+* K)
    (h : ∑ i, a i * m i = 1) : ∃ i, φ (m i) ≠ 0 := by
  classical
  by_contra hn
  have hz : ∀ i, φ (m i) = 0 := by
    intro i
    by_contra hi
    exact hn ⟨i,hi⟩
  have he := specialize_unit_identity a m φ h
  simp [hz] at he

end KoblitzMinorWitness32
