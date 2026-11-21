import AuxiliaryTangents

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit FinitePointEvaluationAudit GeneralTranslationAudit
open BKThreeNormalizationAudit ThreeTangentPairingAudit
variable {K : Type*} [Field K] [DecidableEq K]

def infinityTranslatedRegular (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (r : RegularRing W Q) : RegularRing W 0 :=
  ⟨generalTranslation W hΔ Q r,by
    change localFunctionValuation W 0 (generalTranslation W hΔ Q r)≤1
    rw [general_translation_valuation,zero_add];exact r.property⟩

theorem infinity_translated_residue (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (r : RegularRing W Q) :
    regularEvaluation W 0 (infinityTranslatedRegular W hΔ Q r)=regularEvaluation W Q r := by
  apply (regular_evaluation_difference W 0 _ _).mpr
  change localFunctionValuation W 0
    (generalTranslation W hΔ Q r-constantHom W (regularEvaluation W Q r))<1
  rw [←general_translation_constant W hΔ Q,←map_sub,general_translation_valuation,zero_add]
  exact (regular_evaluation_difference W Q r _).mp rfl

def negatedRegularAt (W : WeierstrassCurve.Affine K) (P : W.Point)
    (r : RegularRing W P) : RegularRing W (-P) :=
  regularNegation W (-P) ⟨r,by simpa only [neg_neg] using r.property⟩

omit [DecidableEq K] in
theorem negated_regular_at_residue (W : WeierstrassCurve.Affine K) (P : W.Point)
    (r : RegularRing W P) :
    regularEvaluation W (-P) (negatedRegularAt W P r)=regularEvaluation W P r := by
  apply (regular_evaluation_difference W (-P) _ _).mpr
  change localFunctionValuation W (-P)
    (fieldNeg W r-constantHom W (regularEvaluation W P r))<1
  rw [←negation_constant,←map_sub,local_negation_value,neg_neg]
  exact (regular_evaluation_difference W P r _).mp rfl

/-- Actual infinity residue computes the scalar of any two y-monic lines. -/
theorem line_auxiliary_scalar_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (x y u v m s : K) (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v) (c : K)
    (he : auxiliaryQuotient W hΔ (.some x y hP) (.some u v hQ)
      (lineFunction W x y m) (lineFunction W u v s)=constantHom W c) :
    c*(y-v-s*(x-u))= -(v-y-m*(u-x)) := by
  let P : W.Point := .some x y hP
  let Q : W.Point := .some u v hQ
  let f := lineFunction W x y m
  let g := lineFunction W u v s
  let F : RegularRing W Q := coordinateRegular W hQ (MillerPrincipalIdealAudit.lineClass W x y m)
  let G : RegularRing W P := coordinateRegular W hP (MillerPrincipalIdealAudit.lineClass W u v s)
  have hF : (F : W.FunctionField)=f := coordinate_regular_coe W hQ _
  have hG : (G : W.FunctionField)=g := coordinate_regular_coe W hP _
  let A := infinityTranslatedRegular W hΔ Q F
  let B := infinityTranslatedRegular W hΔ (-P) (negatedRegularAt W P G)
  let L := negatedLineQuotientRegular W u v s x y m
  have hA : (A : W.FunctionField)=generalTranslation W hΔ Q f := by
    change generalTranslation W hΔ Q (F : W.FunctionField)=_
    rw [hF]
  have hB : (B : W.FunctionField)=generalTranslation W hΔ (-P) (fieldNeg W g) := by
    change generalTranslation W hΔ (-P) (fieldNeg W (G : W.FunctionField))=_
    rw [hG]
  have hcross : generalTranslation W hΔ Q f*fieldNeg W g=
      constantHom W c*(f*generalTranslation W hΔ (-P) (fieldNeg W g)) :=
    (div_eq_iff (auxiliary_denominator_nonzero W hΔ P
      (MillerPrincipalIdealAudit.line_function_nonzero W x y m)
      (MillerPrincipalIdealAudit.line_function_nonzero W u v s))).mp he
  have hr : A*L=regularConstant W 0 c*B := by
    apply Subtype.ext
    change (A : W.FunctionField)*(L : W.FunctionField)=constantHom W c*(B : W.FunctionField)
    rw [hA,hB]
    change generalTranslation W hΔ Q f*(fieldNeg W g/f)=_
    rw [←mul_div_assoc,hcross]
    have hf : f≠0 := MillerPrincipalIdealAudit.line_function_nonzero W x y m
    field_simp [hf]
  have hAv : regularEvaluation W 0 A=v-y-m*(u-x) := by
    rw [infinity_translated_residue]
    exact (coordinate_regular_evaluation W hQ _).trans (MillerPrincipalIdealAudit.line_evaluation W hQ.1 x y m)
  have hBv : regularEvaluation W 0 B=y-v-s*(x-u) := by
    rw [infinity_translated_residue,negated_regular_at_residue]
    exact (coordinate_regular_evaluation W hP _).trans (MillerPrincipalIdealAudit.line_evaluation W hP.1 u v s)
  have hv := congrArg (regularEvaluation W 0) hr
  rw [map_mul,map_mul,regular_eval_constant,hAv,hBv,negated_line_quotient_residue] at hv
  simpa only [mul_neg,mul_one] using hv.symm

/-- The actual tangent ratio is the actual auxiliary quotient, for distinct abscissae. -/
theorem tangent_auxiliary_quotient [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x y u v : K)
    (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v)
    (htP : (3 : ℕ) • Point.some x y hP=0) (htQ : (3 : ℕ) • Point.some u v hQ=0)
    (hx : x≠u) :
    auxiliaryQuotient W hΔ (.some x y hP) (.some u v hQ)
      (tangentFunction W x y) (tangentFunction W u v)=
      constantHom W (tangentPair W (.some x y hP) (.some u v hQ)) := by
  obtain ⟨c,hc,he⟩ := auxiliary_scalar W hΔ (.some x y hP) (.some u v hQ) 3
    (tangent_function_nonzero W x y) (tangent_function_nonzero W u v)
    (tangent_function_divisor W hP htP) (tangent_function_divisor W hQ htQ)
  have hv := line_auxiliary_scalar_value W hΔ x y u v (tangentSlope W x y)
    (tangentSlope W u v) hP hQ c he
  have hn := (tangentPair_outside_domain W hΔ x y u v hP hQ htP htQ hx).2
  have hc' : c=tangentPair W (.some x y hP) (.some u v hQ) := by
    change c=(if x=u then 1 else -tangentValue W x y u v/tangentValue W u v x y)
    rw [ite_eq_right hx]
    exact (eq_div_iff hn).mpr hv
  simpa only [hc'] using he

#print axioms line_auxiliary_scalar_value
#print axioms tangent_auxiliary_quotient
end
end WeilAuxiliaryRatioAudit
