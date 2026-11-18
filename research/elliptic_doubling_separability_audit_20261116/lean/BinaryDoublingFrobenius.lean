import OddDegreeSeparable

namespace EllipticDoublingSeparabilityAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open EllipticDoublingDegreeAudit EllipticTranslationAudit EllipticFieldNormAudit
open IsogenyFactorizationGuardAudit GeneralTranslationAudit MillerNormValuationAudit
open NegationPullbackAudit MillerInfinityChartAudit FunctionDivisorFinitenessAudit
open scoped EllipticFieldNormAudit CharTwo
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
attribute [local instance] generalF2CommRing
private instance binaryFunctionChar : CharP binaryCurve.FunctionField 2 :=
  charP_of_injective_algebraMap' GeneralF2 2

def binaryFrobeniusSquareNegation : binaryCurve.FunctionField →ₐ[GeneralF2] binaryCurve.FunctionField :=
  (fieldFrobenius binaryCurve).comp ((fieldFrobenius binaryCurve).comp
    ((functionConjugation binaryCurve).restrictScalars GeneralF2).toAlgHom)

theorem binary_closed_coefficients :
    binaryCurve.b₂=0 ∧ binaryCurve.b₄=0 ∧ binaryCurve.b₆=1 ∧ binaryCurve.b₈=0 := by
  decide +kernel

theorem binary_generic_coefficients :
    (genericCurve binaryCurve).a₁=0 ∧ (genericCurve binaryCurve).a₂=0 ∧
    (genericCurve binaryCurve).a₃=1 ∧ (genericCurve binaryCurve).a₄=0 ∧
    (genericCurve binaryCurve).a₆=0 := by
  exact ⟨map_zero (algebraMap GeneralF2 binaryCurve.FunctionField),
    map_zero (algebraMap GeneralF2 binaryCurve.FunctionField),
    map_one (algebraMap GeneralF2 binaryCurve.FunctionField),
    map_zero (algebraMap GeneralF2 binaryCurve.FunctionField),
    map_zero (algebraMap GeneralF2 binaryCurve.FunctionField)⟩

theorem binary_doubling_x : doublingAlgHom binaryCurve binary_delta (xFunction binaryCurve)=
    xFunction binaryCurve^4 := by
  obtain ⟨hb2,hb4,hb6,hb8⟩ := binary_closed_coefficients
  rw [doubling_field_x,actual_doubled_x,doublingRational,map_div₀,
    rational_embedding_polynomial,rational_embedding_polynomial]
  simp [doublingNumerator,doublingDenominator,hb2,hb4,hb6,hb8,map_pow]

theorem binary_generic_slope :
    (genericCurve binaryCurve).slope (xFunction binaryCurve) (xFunction binaryCurve)
      (yFunction binaryCurve) (yFunction binaryCurve)=xFunction binaryCurve^2 := by
  obtain ⟨ha1,ha2,ha3,ha4,_⟩ := binary_generic_coefficients
  rw [slope_of_Y_ne rfl (generic_not_its_negative binaryCurve binary_delta)]
  simp only [negY,ha1,ha2,ha3,ha4,zero_mul,sub_zero]
  simp

theorem binary_doubling_y : doublingAlgHom binaryCurve binary_delta (yFunction binaryCurve)=
    yFunction binaryCurve^4+1 := by
  obtain ⟨ha1,ha2,ha3,ha4,ha6⟩ := binary_generic_coefficients
  rw [doubling_field_y,doubledY,doubledGeneric,EllipticTranslationAudit.genericPoint,
    Point.add_self_of_Y_ne (generic_not_its_negative binaryCurve binary_delta)]
  change (genericCurve binaryCurve).addY _ _ _ _ = _
  rw [binary_generic_slope]
  have he := ((genericCurve binaryCurve).equation_iff _ _).mp (generic_curve_equation binaryCurve)
  simp only [ha1,ha2,ha3,ha4,ha6,zero_mul,add_zero,one_mul] at he
  have ht : (2 : binaryCurve.FunctionField)=0 := CharP.cast_eq_zero _ 2
  simp only [addY,negY,negAddY,addX,ha1,ha2,ha3,zero_mul,sub_zero]
  grobner

theorem actual_binary_doubling_map_eq :
    doublingAlgHom binaryCurve binary_delta=binaryFrobeniusSquareNegation := by
  have he : (doublingAlgHom binaryCurve binary_delta).toRingHom=
      binaryFrobeniusSquareNegation.toRingHom := by
    apply function_hom_ext binaryCurve
    · intro c
      change doublingAlgHom binaryCurve binary_delta (constantHom binaryCurve c)=
        binaryFrobeniusSquareNegation (constantHom binaryCurve c)
      rw [←function_coefficient_map]
      rw [AlgHom.commutes,AlgHom.commutes]
    · change doublingAlgHom binaryCurve binary_delta (xFunction binaryCurve)=
        binaryFrobeniusSquareNegation (xFunction binaryCurve)
      rw [binary_doubling_x]
      change xFunction binaryCurve^4=fieldFrobenius binaryCurve
        (fieldFrobenius binaryCurve (fieldNeg binaryCurve (xFunction binaryCurve)))
      rw [field_neg_x,field_frobenius_apply,field_frobenius_apply]
      change xFunction binaryCurve^4=(xFunction binaryCurve^2)^2
      ring
    · change doublingAlgHom binaryCurve binary_delta (yFunction binaryCurve)=
        binaryFrobeniusSquareNegation (yFunction binaryCurve)
      rw [binary_doubling_y]
      change yFunction binaryCurve^4+1=fieldFrobenius binaryCurve
        (fieldFrobenius binaryCurve (fieldNeg binaryCurve (yFunction binaryCurve)))
      have hn : fieldNeg binaryCurve (yFunction binaryCurve)=yFunction binaryCurve+1 := by
        have hh : linearH binaryCurve=Polynomial.C (1 : GeneralF2) := by
          simp [linearH,binaryCurve]
        rw [field_neg_y,hh,polynomial_function_constant,map_one]
        simp
      rw [hn,map_add,map_one,map_add,map_one,field_frobenius_apply,field_frobenius_apply]
      change yFunction binaryCurve^4+1=(yFunction binaryCurve^2)^2+1
      ring
  exact AlgHom.ext fun f => RingHom.congr_fun he f

theorem binary_composed_map_apply (f : binaryCurve.FunctionField) :
    binaryFrobeniusSquareNegation f=(fieldNeg binaryCurve f)^4 := by
  change ((fieldNeg binaryCurve f)^2)^2=(fieldNeg binaryCurve f)^4
  ring

theorem actual_binary_fourth_power_preimage (f : binaryCurve.FunctionField) :
    doublingAlgHom binaryCurve binary_delta (fieldNeg binaryCurve f)=f^4 := by
  rw [actual_binary_doubling_map_eq,binary_composed_map_apply,field_neg_square]

#print axioms actual_binary_doubling_map_eq
#print axioms actual_binary_fourth_power_preimage
end
end EllipticDoublingSeparabilityAudit
