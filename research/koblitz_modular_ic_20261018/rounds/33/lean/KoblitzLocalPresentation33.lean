import Mathlib.RingTheory.Ideal.Defs

/-! A unit coefficient eliminates a redundant equation. This universal ring
lemma does not assert the existence of the native local unit witness. -/
namespace KoblitzLocalPresentation33
variable {R : Type*} [CommRing R]

theorem unit_coefficient_membership (I : Ideal R) (h f : R)
    (hh : IsUnit h) (hf : h * f ∈ I) : f ∈ I := by
  rcases hh with ⟨u, rfl⟩
  have hm := I.mul_mem_left (↑(u⁻¹) : R) hf
  simpa only [← mul_assoc, Units.inv_mul, one_mul] using hm

theorem remove_redundant_equation (I : Ideal R) (h f t : R)
    (hh : IsUnit h) (ht : t ∈ I) (relation : h * f + t = 0) : f ∈ I := by
  apply unit_coefficient_membership I h f hh
  have he : h * f = -t := eq_neg_of_add_eq_zero_left relation
  rw [he]
  exact I.neg_mem ht

end KoblitzLocalPresentation33
