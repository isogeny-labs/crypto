import Mathlib.GroupTheory.QuotientGroup.Defs

namespace TwistQuotientCycle

/- Faithful abstract quotient descent. Concrete curve groups, morphisms,
   field of definition, kernel inclusion and map degree are external bridges. -/
variable {A : Type*} [AddCommGroup A]

theorem quotient_descent (H K : AddSubgroup A) (a : A →+ A)
    (h : H ≤ K.comap a) :
    ∃ b : A ⧸ H →+ A ⧸ K,
      ∀ x, b (QuotientAddGroup.mk' H x) = QuotientAddGroup.mk' K (a x) := by
  exact ⟨QuotientAddGroup.map H K a h, fun _ => rfl⟩

theorem quotient_descent_unique (H K : AddSubgroup A) (a : A →+ A)
    (b c : A ⧸ H →+ A ⧸ K)
    (hb : ∀ x, b (QuotientAddGroup.mk' H x) = QuotientAddGroup.mk' K (a x))
    (hc : ∀ x, c (QuotientAddGroup.mk' H x) = QuotientAddGroup.mk' K (a x)) :
    b = c := by
  apply AddMonoidHom.ext
  intro z
  refine QuotientAddGroup.induction_on z ?_
  intro x
  exact (hb x).trans (hc x).symm

end TwistQuotientCycle
