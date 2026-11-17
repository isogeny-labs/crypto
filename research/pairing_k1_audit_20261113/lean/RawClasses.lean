import UnitTarget

namespace PairingKOneAudit
noncomputable section

theorem reduce9_section : ∀ i : ZMod 3,reduce9 (i.val:ZMod 9)=i := by decide +kernel

def rightRepresentative (z : Right) : curve.Point := indexedPoint ((rightEquiv z).val:ZMod 9)

theorem right_representative_class (z : Right) : toRight (rightRepresentative z)=z := by
  apply rightEquiv.injective
  rw [rightRepresentative,right_on_index,reduce9_section]

def rawUnit (U : threeTorsion) (R : curve.Point) : BaseUnits :=
  Units.mk0 (rawRatio U R) (rawRatio_nonzero U R)

def tateClass (U : threeTorsion) (z : Right) : UnitTarget :=
  (QuotientGroup.mk' baseCubes) (rawUnit U (rightRepresentative z))

def reducedUnits (U : threeTorsion) (z : Right) : cubeRoots := finalExponentEquiv (tateClass U z)

theorem reduced_value (U : threeTorsion) (z : Right) :
    rootValue (reducedUnits U z)=tateValue U z := by
  unfold rootValue reducedUnits tateClass
  rw [finalExponentEquiv_mk]
  change (rawRatio U (rightRepresentative z))^2=tateValue U z
  rw [rawRatio_reduction,right_representative_class]

theorem reduced_left_bilinear (U V : threeTorsion) (z : Right) :
    reducedUnits (U+V) z=reducedUnits U z*reducedUnits V z := by
  apply rootValue_injective
  simpa only [rootValue_mul,reduced_value] using tate_left_bilinear U V z

theorem reduced_right_bilinear (U : threeTorsion) (z w : Right) :
    reducedUnits U (z+w)=reducedUnits U z*reducedUnits U w := by
  apply rootValue_injective
  simpa only [rootValue_mul,reduced_value] using tate_right_bilinear U z w

theorem class_left_bilinear (U V : threeTorsion) (z : Right) :
    tateClass (U+V) z=tateClass U z*tateClass V z := by
  apply finalExponentEquiv.injective
  simpa only [reducedUnits,map_mul] using reduced_left_bilinear U V z

theorem class_right_bilinear (U : threeTorsion) (z w : Right) :
    tateClass U (z+w)=tateClass U z*tateClass U w := by
  apply finalExponentEquiv.injective
  simpa only [reducedUnits,map_mul] using reduced_right_bilinear U z w

theorem class_one_implies_value_one (U : threeTorsion) (z : Right)
    (h : tateClass U z=1) : tateValue U z=1 := by
  have hh := congrArg (fun w : UnitTarget => rootValue (finalExponentEquiv w)) h
  change rootValue (reducedUnits U z)=rootValue (finalExponentEquiv 1) at hh
  rw [reduced_value,map_one,rootValue_one] at hh
  exact hh

theorem class_left_nondegenerate (U : threeTorsion) (hU : ∀ z,tateClass U z=1) : U=0 := by
  apply tate_left_nondegenerate U
  exact fun z => class_one_implies_value_one U z (hU z)

theorem class_right_nondegenerate (z : Right) (hz : ∀ U,tateClass U z=1) : z=0 := by
  apply tate_right_nondegenerate z
  exact fun U => class_one_implies_value_one U z (hz U)

theorem reduced_left_nondegenerate (U : threeTorsion) (hU : ∀ z,reducedUnits U z=1) : U=0 := by
  apply tate_left_nondegenerate U
  intro z
  have h := congrArg rootValue (hU z)
  simpa only [reduced_value,rootValue_one] using h

theorem reduced_right_nondegenerate (z : Right) (hz : ∀ U,reducedUnits U z=1) : z=0 := by
  apply tate_right_nondegenerate z
  intro U
  have h := congrArg rootValue (hz U)
  simpa only [reduced_value,rootValue_one] using h

theorem raw_class_on_actual_point (U : threeTorsion) (R : curve.Point) :
    tateClass U (toRight R)=(QuotientGroup.mk' baseCubes) (rawUnit U R) := by
  apply finalExponentEquiv.injective
  apply rootValue_injective
  change rootValue (reducedUnits U (toRight R))=
    rootValue (finalExponentEquiv ((QuotientGroup.mk' baseCubes) (rawUnit U R)))
  rw [reduced_value]
  unfold rootValue
  rw [finalExponentEquiv_mk]
  change tateValue U (toRight R)=(rawRatio U R)^2
  exact (rawRatio_reduction U R).symm

def rawAuxUnit (U : threeTorsion) (R S : curve.Point) (h : goodAux U.val R S) : BaseUnits :=
  Units.mk0 (millerLine U.val (R+S)/millerLine U.val S) (div_ne_zero h.2 h.1)

theorem raw_class_all_valid_auxiliaries (U : threeTorsion) (R S : curve.Point)
    (h : goodAux U.val R S) :
    (QuotientGroup.mk' baseCubes) (rawAuxUnit U R S h)=tateClass U (toRight R) := by
  apply finalExponentEquiv.injective
  apply rootValue_injective
  change rootValue (finalExponentEquiv ((QuotientGroup.mk' baseCubes) (rawAuxUnit U R S h)))=
    rootValue (reducedUnits U (toRight R))
  rw [reduced_value]
  unfold rootValue
  rw [finalExponentEquiv_mk]
  change (millerLine U.val (R+S)/millerLine U.val S)^2=tateValue U (toRight R)
  exact all_valid_auxiliary_actual U R S h

theorem class_right_zero (U : threeTorsion) : tateClass U 0=1 := by
  apply finalExponentEquiv.injective
  apply rootValue_injective
  change rootValue (reducedUnits U 0)=rootValue (finalExponentEquiv 1)
  rw [reduced_value,map_one,rootValue_one,tateValue_right_zero]

theorem class_restricted_trivial (U V : threeTorsion) : tateClass U (torsionToRight V)=1 := by
  rw [torsionToRight_zero,class_right_zero]

theorem class_restricted_not_nondegenerate :
    ¬∀ U : threeTorsion,(∀ V : threeTorsion,tateClass U (torsionToRight V)=1) → U=0 :=
  pullback_not_left_nondegenerate tateClass class_right_zero

#print axioms reduced_value
#print axioms raw_class_all_valid_auxiliaries
#print axioms class_left_nondegenerate
#print axioms class_right_nondegenerate
#print axioms class_restricted_not_nondegenerate

end
end PairingKOneAudit
