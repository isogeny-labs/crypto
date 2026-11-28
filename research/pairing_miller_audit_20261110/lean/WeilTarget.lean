import TateClassPairing
import BaseDescent

namespace PairingMillerWeilTargetAudit
open PairingMillerCurveAudit PairingMillerTorsionAudit PairingMillerTableAudit
  PairingMillerPropertiesAudit PairingMillerUnitsTargetAudit PairingMillerTateClassAudit
  PairingMillerBaseDescentAudit

theorem omega_power_roots : ∀ a : ZMod 3,(omega^a.val)^3=1 ∧ omega^a.val≠0 := by
  decide +kernel

theorem weil_value_roots (U V : threeTorsion) : (weilValue U V)^3=1 ∧ weilValue U V≠0 := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  rw [weilValue_indexed]
  exact omega_power_roots _

def weilUnits (U V : threeTorsion) : cubeRoots :=
  ⟨Units.mk0 (weilValue U V) (weil_value_roots U V).2,by
    change (Units.mk0 (weilValue U V) (weil_value_roots U V).2)^3=1
    apply Units.ext
    change (weilValue U V)^3=1
    exact (weil_value_roots U V).1⟩

theorem weil_units_value (U V : threeTorsion) : rootValue (weilUnits U V)=weilValue U V := rfl

theorem weil_units_left_bilinear (U V W : threeTorsion) :
    weilUnits (U+V) W=weilUnits U W*weilUnits V W := by
  apply rootValue_injective
  simpa only [rootValue_mul,weil_units_value] using weil_left_bilinear U V W

theorem weil_units_right_bilinear (U V W : threeTorsion) :
    weilUnits U (V+W)=weilUnits U V*weilUnits U W := by
  apply rootValue_injective
  simpa only [rootValue_mul,weil_units_value] using weil_right_bilinear U V W

theorem weil_units_left_nondegenerate (U : threeTorsion)
    (hU : ∀ V,weilUnits U V=1) : U=0 := by
  apply weil_left_nondegenerate U
  intro V
  simpa only [weil_units_value,rootValue_one] using congrArg rootValue (hU V)

theorem weil_units_right_nondegenerate (V : threeTorsion)
    (hV : ∀ U,weilUnits U V=1) : V=0 := by
  apply weil_right_nondegenerate V
  intro U
  simpa only [weil_units_value,rootValue_one] using congrArg rootValue (hV U)

theorem weil_units_alternating (U : threeTorsion) : weilUnits U U=1 := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  apply rootValue_injective
  rw [weil_units_value,rootValue_one,weilValue_indexed]
  have hd : determinant u u=0 := by unfold determinant;ring
  rw [hd]
  rfl

theorem base_restriction_nondegeneracy_false :
    ¬∀ U : threeTorsion,BaseRational U.val →
      (∀ V : threeTorsion,BaseRational V.val → weilUnits U V=1) → U=0 := by
  intro h
  apply rational_domain_nondegeneracy_false
  intro U hU hUV
  apply h U hU
  intro V hV
  apply rootValue_injective
  simpa only [weil_units_value,rootValue_one] using hUV V hV

theorem embedding_two_conditions : Nat.Coprime 3 5 ∧ ¬3∣5-1 ∧ 3∣25-1 := by
  decide +kernel

#print axioms weil_units_left_nondegenerate
#print axioms base_restriction_nondegeneracy_false
end PairingMillerWeilTargetAudit
