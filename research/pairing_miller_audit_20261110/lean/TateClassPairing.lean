import TateQuotient
import UnitsTarget

namespace PairingMillerTateClassAudit
open PairingMillerCurveAudit PairingMillerExpressionsAudit PairingMillerTorsionAudit
  PairingMillerTableAudit PairingMillerPropertiesAudit PairingMillerTateQuotientAudit
  PairingMillerUnitsTargetAudit

def rawRatio (U V : threeTorsion) : F25 :=
  millerLine U.val (V.val+auxiliary)/millerLine U.val auxiliary

theorem rawRatio_nonzero (U V : threeTorsion) : rawRatio U V≠0 := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  exact div_ne_zero (all_evaluations_nonzero v u).1 (all_evaluations_nonzero v u).2.1

def rawUnit (U V : threeTorsion) : ExtUnits := Units.mk0 (rawRatio U V) (rawRatio_nonzero U V)

/-- The unreduced target is the actual field-unit quotient by cubes. -/
noncomputable def tateClass (U : threeTorsion) (z : Right) : ExtTarget :=
  (QuotientGroup.mk' extCubes) (rawUnit U (torsionQuotientEquiv.symm z))

noncomputable def reducedUnits (U : threeTorsion) (z : Right) : cubeRoots :=
  finalExponentEquiv (tateClass U z)

def rootValue (z : cubeRoots) : F25 := z.val.val
theorem rootValue_mul (z w : cubeRoots) : rootValue (z*w)=rootValue z*rootValue w := rfl
theorem rootValue_one : rootValue 1=1 := rfl
theorem rootValue_injective : Function.Injective rootValue :=
  fun _ _ h=>Subtype.ext (Units.ext h)

theorem reduced_value (U : threeTorsion) (z : Right) :
    rootValue (reducedUnits U z)=tateQuotientValue U z := by
  unfold rootValue reducedUnits tateClass
  rw [finalExponentEquiv_mk]
  rfl

theorem reduced_left_bilinear (U V : threeTorsion) (z : Right) :
    reducedUnits (U+V) z=reducedUnits U z*reducedUnits V z := by
  apply rootValue_injective
  simpa only [rootValue_mul,reduced_value] using quotient_left_bilinear U V z

theorem reduced_right_bilinear (U : threeTorsion) (z w : Right) :
    reducedUnits U (z+w)=reducedUnits U z*reducedUnits U w := by
  apply rootValue_injective
  simpa only [rootValue_mul,reduced_value] using quotient_right_bilinear U z w

theorem class_left_bilinear (U V : threeTorsion) (z : Right) :
    tateClass (U+V) z=tateClass U z*tateClass V z := by
  apply finalExponentEquiv.injective
  simpa only [reducedUnits,map_mul] using reduced_left_bilinear U V z

theorem class_right_bilinear (U : threeTorsion) (z w : Right) :
    tateClass U (z+w)=tateClass U z*tateClass U w := by
  apply finalExponentEquiv.injective
  simpa only [reducedUnits,map_mul] using reduced_right_bilinear U z w

theorem class_one_implies_value_one (U : threeTorsion) (z : Right)
    (h : tateClass U z=1) : tateQuotientValue U z=1 := by
  have hh:=congrArg (fun w : ExtTarget=>rootValue (finalExponentEquiv w)) h
  change rootValue (reducedUnits U z)=rootValue (finalExponentEquiv 1) at hh
  rw [reduced_value,map_one,rootValue_one] at hh
  exact hh

theorem class_left_nondegenerate (U : threeTorsion)
    (hU : ∀ z,tateClass U z=1) : U=0 := by
  apply quotient_left_nondegenerate U
  exact fun z=>class_one_implies_value_one U z (hU z)

theorem class_right_nondegenerate (z : Right)
    (hz : ∀ U,tateClass U z=1) : z=0 := by
  apply quotient_right_nondegenerate z
  exact fun U=>class_one_implies_value_one U z (hz U)

theorem reduced_left_nondegenerate (U : threeTorsion)
    (hU : ∀ z,reducedUnits U z=1) : U=0 := by
  apply quotient_left_nondegenerate U
  intro z
  have h:=congrArg rootValue (hU z)
  simpa only [reduced_value,rootValue_one] using h

theorem reduced_right_nondegenerate (z : Right)
    (hz : ∀ U,reducedUnits U z=1) : z=0 := by
  apply quotient_right_nondegenerate z
  intro U
  have h:=congrArg rootValue (hz U)
  simpa only [reduced_value,rootValue_one] using h

theorem raw_class_on_torsion (U V : threeTorsion) :
    tateClass U (torsionToRight V)=(QuotientGroup.mk' extCubes) (rawUnit U V) := by
  have hv : torsionQuotientEquiv.symm (torsionToRight V)=V :=
    torsionQuotientEquiv.symm_apply_apply V
  unfold tateClass
  rw [hv]

#print axioms class_left_nondegenerate
#print axioms reduced_left_nondegenerate
#print axioms actual_target_cardinalities
end PairingMillerTateClassAudit
