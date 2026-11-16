import BaseDescent

namespace PairingMillerTateQuotientAudit
open PairingMillerCurveAudit PairingMillerTorsionAudit PairingMillerPropertiesAudit

def pointOfEquation (x y : F25) (h : y^2=x^3+1) : curve.Point := .some x y (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  obtain ⟨h1,h2,h3,h4,h6⟩:=BKActualThreeAudit.coefficients (L:=F25)
  simpa only [h1,h2,h3,h4,h6,zero_mul,zero_add,mul_zero,add_zero] using h)

/-- Complete actual point check over F25, with the true curve equation and
 group operation. No Hasse bound or abstract group classification is assumed. -/
theorem six_kills_coordinates : ∀ x y : F25,
    if h : y^2=x^3+1 then (6:ℕ) • pointOfEquation x y h=0 else True := by
  decide +kernel

theorem six_kills_all (R : curve.Point) : (6:ℕ) • R=0 := by
  cases R with
  | zero => rfl
  | some x y h =>
    have he:=BKActualThreeAudit.actual_equation h
    simpa only [dite_eq_left he,pointOfEquation] using six_kills_coordinates x y

def threeMultiples : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 3 : curve.Point→+curve.Point).range
abbrev Right := curve.Point ⧸ threeMultiples

def toRight : curve.Point→+Right := QuotientAddGroup.mk' threeMultiples
def torsionToRight : threeTorsion→+Right := toRight.comp threeTorsion.subtype

theorem right_three (R : curve.Point) : (3:ℕ) • toRight R=0 := by
  rw [←map_nsmul]
  exact (QuotientAddGroup.eq_zero_iff _).mpr ⟨R,rfl⟩

theorem torsionToRight_injective : Function.Injective torsionToRight := by
  apply (injective_iff_map_eq_zero torsionToRight).mpr
  intro U hU
  obtain ⟨R,hR⟩:=(QuotientAddGroup.eq_zero_iff U.val).mp hU
  change (3:ℕ) • R=U.val at hR
  have h2 : (2:ℕ) • U.val=0 := by
    rw [←hR,smul_smul]
    exact six_kills_all R
  have h3 : (3:ℕ) • U.val=0 := U.property
  have hz : U.val=0 := by
    rw [show (3:ℕ)=2+1 by decide,add_nsmul,h2,one_nsmul,zero_add] at h3
    exact h3
  exact Subtype.ext hz

theorem torsionToRight_surjective : Function.Surjective torsionToRight := by
  intro z
  refine QuotientAddGroup.induction_on z (fun R=>?_)
  have h4 : (3:ℕ) • ((4:ℕ) • R)=0 := by
    rw [smul_smul]
    have h:=congrArg (fun T : curve.Point=>(2:ℕ) • T) (six_kills_all R)
    norm_num only [smul_smul,nsmul_zero] at h ⊢
    exact h
  refine ⟨⟨(4:ℕ) • R,h4⟩,?_⟩
  change toRight ((4:ℕ) • R)=toRight R
  rw [map_nsmul,show (4:ℕ)=3+1 by decide,add_nsmul,right_three,one_nsmul,zero_add]

noncomputable def torsionQuotientEquiv : threeTorsion≃+Right :=
  AddEquiv.ofBijective torsionToRight ⟨torsionToRight_injective,torsionToRight_surjective⟩

theorem actual_right_cardinality : Nat.card Right=9 := by
  rw [Nat.card_congr torsionQuotientEquiv.toEquiv.symm]
  exact actual_torsion_cardinality

/-- Evaluate on the unique actual 3-torsion representative of a quotient
class. This representative map is proved from the actual point operations. -/
noncomputable def tateQuotientValue (U : threeTorsion) (z : Right) : F25 :=
  tateValue U (torsionQuotientEquiv.symm z)

theorem quotient_value_on_torsion (U V : threeTorsion) :
    tateQuotientValue U (torsionToRight V)=tateValue U V := by
  exact congrArg (tateValue U) (torsionQuotientEquiv.symm_apply_apply V)

theorem quotient_left_bilinear (U V : threeTorsion) (z : Right) :
    tateQuotientValue (U+V) z=tateQuotientValue U z*tateQuotientValue V z :=
  tate_left_bilinear U V _

theorem quotient_right_bilinear (U : threeTorsion) (z w : Right) :
    tateQuotientValue U (z+w)=tateQuotientValue U z*tateQuotientValue U w := by
  unfold tateQuotientValue
  rw [map_add,tate_right_bilinear]

theorem quotient_left_nondegenerate (U : threeTorsion)
    (hU : ∀ z,tateQuotientValue U z=1) : U=0 := by
  apply tate_left_nondegenerate U
  intro V
  simpa only [quotient_value_on_torsion] using hU (torsionToRight V)

theorem quotient_right_nondegenerate (z : Right)
    (hz : ∀ U,tateQuotientValue U z=1) : z=0 := by
  have h : torsionQuotientEquiv.symm z=0 :=
    tate_right_nondegenerate _ hz
  apply torsionQuotientEquiv.symm.injective
  simpa only [map_zero] using h

#print axioms six_kills_all
#print axioms quotient_left_nondegenerate
#print axioms quotient_right_nondegenerate
end PairingMillerTateQuotientAudit
