import RawClasses

namespace PairingKOneAudit

theorem torsionHom_as_nsmul (i : ZMod 3) : torsionHom i=i.val • torsionHom 1 := by
  apply Subtype.ext
  change torsionPoint i=i.val • torsionPoint 1
  simp only [torsionPoint,show (1:ZMod 3).val=1 by decide +kernel,one_nsmul]

/-- Every alternating biadditive map on the actual rational torsion is trivial. -/
theorem alternating_pairing_trivial {A : Type*} [AddCommGroup A]
    (b : threeTorsion →+ threeTorsion →+ A) (ha : ∀ U,b U U=0)
    (U V : threeTorsion) : b U V=0 := by
  obtain ⟨i,rfl⟩ := torsionHom_bijective.surjective U
  obtain ⟨j,rfl⟩ := torsionHom_bijective.surjective V
  rw [torsionHom_as_nsmul i,torsionHom_as_nsmul j]
  simp only [map_nsmul,AddMonoidHom.nsmul_apply,ha,nsmul_zero]

theorem alternating_pairing_not_nondegenerate {A : Type*} [AddCommGroup A]
    (b : threeTorsion →+ threeTorsion →+ A) (ha : ∀ U,b U U=0) :
    ¬∀ U : threeTorsion,(∀ V,b U V=0) → U=0 := by
  intro h
  exact torsionHom_one_ne_zero (h (torsionHom 1) (alternating_pairing_trivial b ha _))

#print axioms alternating_pairing_trivial
#print axioms alternating_pairing_not_nondegenerate
end PairingKOneAudit
