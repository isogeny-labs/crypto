import TwoRoots

namespace WeilTwoNondegeneracyAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open MillerPrincipalIdealAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem two_vertical_divisor (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (ht : Point.some x y h+Point.some x y h=0) :
    functionDivisor W (verticalFunction W x) (vertical_nonzero W x)=
      torsionDivisor W (.some x y h) 2 := by
  have hn : -(Point.some x y h)=Point.some x y h := (eq_neg_of_add_eq_zero_left ht).symm
  ext R
  have he:=congrArg (fun D : W.Point →₀ ℤ=>D R) (vertical_principal_point_divisor W h)
  change pointOrder W (verticalFunction W x) R=_ at he ⊢
  rw [verticalDivisor,hn] at he
  rw [←Finsupp.single_add] at he
  simpa [torsionDivisor,Finsupp.smul_apply,zsmul_eq_mul] using he

omit [DecidableEq K] in
theorem vertical_quotient_near_one (W : WeierstrassCurve.Affine K) (u x : K) :
    functionValuation W (verticalFunction W u/verticalFunction W x-1)<1 := by
  have h:=near_one_div (functionValuation W) (vertical_normalized W u) (vertical_normalized W x)
  have he : (parameter W^2*verticalFunction W u)/(parameter W^2*verticalFunction W x)=
      verticalFunction W u/verticalFunction W x := by
    field_simp [(parameter_is_uniformizer W).ne_zero,vertical_nonzero W x]
  rwa [he] at h

def verticalQuotientRegular (W : WeierstrassCurve.Affine K) (u x : K) : RegularRing W 0 :=
  ⟨verticalFunction W u/verticalFunction W x,
    (near_one_value (functionValuation W) (vertical_quotient_near_one W u x)).le⟩

omit [DecidableEq K] in
theorem vertical_quotient_residue (W : WeierstrassCurve.Affine K) (u x : K) :
    regularEvaluation W 0 (verticalQuotientRegular W u x)=1 := by
  apply (regular_evaluation_difference W 0 _ 1).mpr
  change functionValuation W (verticalFunction W u/verticalFunction W x-constantHom W 1)<1
  simpa only [map_one] using vertical_quotient_near_one W u x

theorem vertical_auxiliary_scalar (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (x y u v : K) (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v) (c : K)
    (he : auxiliaryQuotient W hΔ (.some x y hP) (.some u v hQ)
      (verticalFunction W x) (verticalFunction W u)=constantHom W c) :
    c*(x-u)=u-x := by
  let P : W.Point:=.some x y hP
  let Q : W.Point:=.some u v hQ
  let f:=verticalFunction W x
  let g:=verticalFunction W u
  let F : RegularRing W Q:=coordinateRegular W hQ (WeierstrassCurve.Affine.CoordinateRing.XClass W x)
  let G : RegularRing W P:=coordinateRegular W hP (WeierstrassCurve.Affine.CoordinateRing.XClass W u)
  have hF : (F : W.FunctionField)=f := coordinate_regular_coe W hQ _
  have hG : (G : W.FunctionField)=g := coordinate_regular_coe W hP _
  have hneg : fieldNeg W g=g := by
    change fieldNeg W (verticalFunction W u)=verticalFunction W u
    rw [vertical_formula,map_sub,field_neg_x,negation_constant]
  let A:=infinityTranslatedRegular W hΔ Q F
  let B:=infinityTranslatedRegular W hΔ (-P) (negatedRegularAt W P G)
  let L:=verticalQuotientRegular W u x
  have hA : (A : W.FunctionField)=generalTranslation W hΔ Q f := by
    change generalTranslation W hΔ Q (F : W.FunctionField)=_;rw [hF]
  have hB : (B : W.FunctionField)=generalTranslation W hΔ (-P) (fieldNeg W g) := by
    change generalTranslation W hΔ (-P) (fieldNeg W (G : W.FunctionField))=_;rw [hG]
  have hcross : generalTranslation W hΔ Q f*fieldNeg W g=
      constantHom W c*(f*generalTranslation W hΔ (-P) (fieldNeg W g)) :=
    (div_eq_iff (auxiliary_denominator_nonzero W hΔ P (vertical_nonzero W x) (vertical_nonzero W u))).mp he
  have hr : A*L=regularConstant W 0 c*B := by
    apply Subtype.ext
    change (A : W.FunctionField)*(L : W.FunctionField)=constantHom W c*(B : W.FunctionField)
    rw [hA,hB]
    change generalTranslation W hΔ Q f*(g/f)=_
    have hc : generalTranslation W hΔ Q f*g=
        constantHom W c*(f*generalTranslation W hΔ (-P) (fieldNeg W g)) := by
      simpa only [hneg] using hcross
    rw [←mul_div_assoc,hc]
    field_simp [show f≠0 from vertical_nonzero W x]
  have hAv : regularEvaluation W 0 A=u-x := by
    rw [infinity_translated_residue]
    exact (coordinate_regular_evaluation W hQ _).trans (point_eval_XClass W hQ.1 x)
  have hBv : regularEvaluation W 0 B=x-u := by
    rw [infinity_translated_residue,negated_regular_at_residue]
    exact (coordinate_regular_evaluation W hP _).trans (point_eval_XClass W hP.1 u)
  have hv:=congrArg (regularEvaluation W 0) hr
  rw [map_mul,map_mul,regular_eval_constant,hAv,hBv,vertical_quotient_residue,mul_one] at hv
  exact hv.symm

theorem two_vertical_auxiliary [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x y u v : K)
    (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v)
    (htP : Point.some x y hP+Point.some x y hP=0)
    (htQ : Point.some u v hQ+Point.some u v hQ=0) (hx : x≠u) :
    auxiliaryQuotient W hΔ (.some x y hP) (.some u v hQ)
      (verticalFunction W x) (verticalFunction W u)=constantHom W (-1) := by
  obtain ⟨c,_,he⟩:=auxiliary_scalar W hΔ (.some x y hP) (.some u v hQ) 2
    (vertical_nonzero W x) (vertical_nonzero W u)
    (two_vertical_divisor W hP htP) (two_vertical_divisor W hQ htQ)
  have hv:=vertical_auxiliary_scalar W hΔ x y u v hP hQ c he
  have hc : c= -1 := by
    apply mul_right_cancel₀ (sub_ne_zero.mpr hx)
    calc
      c*(x-u)=u-x := hv
      _ = -1*(x-u) := by ring
  rwa [hc] at he

#print axioms two_vertical_divisor
#print axioms vertical_auxiliary_scalar
#print axioms two_vertical_auxiliary
end
end WeilTwoNondegeneracyAudit
