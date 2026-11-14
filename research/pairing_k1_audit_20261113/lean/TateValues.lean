import MillerData

namespace PairingKOneAudit
noncomputable section

def tateValue (U : threeTorsion) (z : Right) : F :=
  indexValue (torsionEquiv.symm U) (rightEquiv z)

theorem tateValue_indexed (u : ZMod 3) (j : ZMod 9) :
    tateValue (torsionHom u) (toRight (indexedPoint j))=indexValue u (reduce9 j) := by
  have hu : torsionEquiv.symm (torsionHom u)=u := torsionEquiv.symm_apply_apply u
  simp only [tateValue,hu,right_on_index]

theorem tateValue_actual (U : threeTorsion) (R : curve.Point) :
    tateValue U (toRight R)=reducedTateMiller U.val R := by
  obtain ⟨u,rfl⟩ := torsionHom_bijective.surjective U
  obtain ⟨j,rfl⟩ := indexHom_surjective R
  exact (tateValue_indexed u j).trans (tate_table u j).symm

theorem all_support_actual (U : threeTorsion) (R : curve.Point) :
    goodAux U.val R (auxiliary U.val R) := by
  obtain ⟨u,rfl⟩ := torsionHom_bijective.surjective U
  obtain ⟨j,rfl⟩ := indexHom_surjective R
  exact chosen_auxiliary_support u j

theorem all_valid_auxiliary_actual (U : threeTorsion) (R S : curve.Point)
    (h : goodAux U.val R S) :
    (millerLine U.val (R+S)/millerLine U.val S)^2=tateValue U (toRight R) := by
  obtain ⟨u,rfl⟩ := torsionHom_bijective.surjective U
  obtain ⟨j,rfl⟩ := indexHom_surjective R
  obtain ⟨s,rfl⟩ := indexHom_surjective S
  change (millerLine (torsionPoint u) (indexedPoint j+indexedPoint s)/
    millerLine (torsionPoint u) (indexedPoint s))^2=
      tateValue (torsionHom u) (toRight (indexedPoint j))
  rw [tateValue_indexed]
  exact all_valid_auxiliary_values u j s h

theorem tate_left_bilinear (U V : threeTorsion) (z : Right) :
    tateValue (U+V) z=tateValue U z*tateValue V z := by
  unfold tateValue
  rw [map_add,indexValue_left_add]

theorem tate_right_bilinear (U : threeTorsion) (z w : Right) :
    tateValue U (z+w)=tateValue U z*tateValue U w := by
  unfold tateValue
  rw [map_add,indexValue_right_add]

theorem tate_left_nondegenerate (U : threeTorsion) (hU : ∀ z,tateValue U z=1) : U=0 := by
  have h : indexValue (torsionEquiv.symm U) 1=1 := by
    simpa only [tateValue,right_on_G] using hU (toRight G)
  have hz := (indexValue_one_iff _).mp h
  apply torsionEquiv.symm.injective
  simpa only [map_zero] using hz

theorem tate_right_nondegenerate (z : Right) (hz : ∀ U,tateValue U z=1) : z=0 := by
  have hu : torsionEquiv.symm (torsionHom 1)=1 := torsionEquiv.symm_apply_apply 1
  have h : indexValue 1 (rightEquiv z)=1 := by
    simpa only [tateValue,hu] using hz (torsionHom 1)
  have hh := (indexValue_one_left_iff _).mp h
  apply rightEquiv.injective
  simpa only [map_zero] using hh

theorem tateValue_cube (U : threeTorsion) (z : Right) : (tateValue U z)^3=1 :=
  indexValue_cube _ _
theorem tateValue_nonzero (U : threeTorsion) (z : Right) : tateValue U z≠0 :=
  indexValue_nonzero _ _

theorem tateValue_generator : tateValue (torsionHom 1) (toRight G)=2 := by
  rw [tateValue_actual]
  have hp : (torsionHom 1).val=(3:ℕ) • G := by
    change torsionPoint 1=(3:ℕ) • G
    simp only [torsionPoint,show (1:ZMod 3).val=1 by decide +kernel,one_nsmul]
  rw [hp]
  exact tate_generator_value

theorem tateValue_right_zero (U : threeTorsion) : tateValue U 0=1 := by
  simp [tateValue,indexValue]

theorem tate_restricted_trivial (U V : threeTorsion) : tateValue U (torsionToRight V)=1 := by
  rw [torsionToRight_zero,tateValue_right_zero]

theorem tate_restricted_not_nondegenerate :
    ¬∀ U : threeTorsion,(∀ V : threeTorsion,tateValue U (torsionToRight V)=1) → U=0 :=
  pullback_not_left_nondegenerate tateValue tateValue_right_zero

def weilValue (U V : threeTorsion) : F := weilMiller U.val V.val

theorem weil_all_one (U V : threeTorsion) : weilValue U V=1 := by
  obtain ⟨u,rfl⟩ := torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩ := torsionHom_bijective.surjective V
  exact weil_table u v

theorem weil_support_actual (U V : threeTorsion) :
    millerLine V.val (U.val+G)≠0 ∧ millerLine V.val G≠0 ∧
    millerLine U.val (-G)≠0 ∧ millerLine U.val (V.val-G)≠0 := by
  obtain ⟨u,rfl⟩ := torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩ := torsionHom_bijective.surjective V
  exact weil_support_table u v

theorem weil_not_nondegenerate : ¬∀ U : threeTorsion,(∀ V,weilValue U V=1) → U=0 := by
  intro h
  exact torsionHom_one_ne_zero (h (torsionHom 1) (weil_all_one _))

def rawRatio (U : threeTorsion) (R : curve.Point) : F :=
  millerLine U.val (R+auxiliary U.val R)/millerLine U.val (auxiliary U.val R)

theorem rawRatio_nonzero (U : threeTorsion) (R : curve.Point) : rawRatio U R≠0 :=
  div_ne_zero (all_support_actual U R).2 (all_support_actual U R).1

theorem rawRatio_reduction (U : threeTorsion) (R : curve.Point) :
    (rawRatio U R)^2=tateValue U (toRight R) := (tateValue_actual U R).symm

#print axioms tateValue_actual
#print axioms tate_left_nondegenerate
#print axioms tate_right_nondegenerate
#print axioms tate_restricted_not_nondegenerate
#print axioms weil_not_nondegenerate

end
end PairingKOneAudit
