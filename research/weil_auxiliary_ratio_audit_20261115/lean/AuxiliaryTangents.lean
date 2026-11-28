import AuxiliaryLeading

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit MillerPrincipalIdealAudit
open NegationPullbackAudit FinitePointEvaluationAudit GeneralTranslationAudit
open BKThreeNormalizationAudit ThreeTorsionRankAudit.General ThreeTangentPairingAudit
variable {K : Type*} [Field K]

/-- Actual coordinate function placed in the actual finite-point regular ring. -/
def coordinateRegular (W : WeierstrassCurve.Affine K) {u v : K}
    (h : W.Nonsingular u v) (a : W.CoordinateRing) : RegularRing W (.some u v h) :=
  finiteLocalEquiv W h (algebraMap W.CoordinateRing (PointLocalRing W u v h.1) a)

theorem coordinate_regular_coe (W : WeierstrassCurve.Affine K) {u v : K}
    (h : W.Nonsingular u v) (a : W.CoordinateRing) :
    (coordinateRegular W h a : W.FunctionField)=algebraMap W.CoordinateRing W.FunctionField a :=
  point_to_field_map W h.1 a

theorem coordinate_regular_evaluation (W : WeierstrassCurve.Affine K) {u v : K}
    (h : W.Nonsingular u v) (a : W.CoordinateRing) :
    regularEvaluation W (.some u v h) (coordinateRegular W h a)=pointEval W u v h.1 a := by
  change finiteRegularEval W h (finiteLocalEquiv W h
    (algebraMap W.CoordinateRing (PointLocalRing W u v h.1) a))=_
  rw [finite_regular_eval_local,local_point_map]

variable [DecidableEq K]

theorem tangent_slope_agrees (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y h=0) :
    W.slope x x y y=tangentSlope W x y := by
  rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl (three_tangent_nonvertical W h h3)]
  unfold tangentSlope numerator denominator WeierstrassCurve.Affine.negY
  congr 1;ring

def tangentFunction (W : WeierstrassCurve.Affine K) (x y : K) : W.FunctionField :=
  MillerNormValuationAudit.lineFunction W x y (tangentSlope W x y)

omit [DecidableEq K] in
theorem tangent_function_nonzero (W : WeierstrassCurve.Affine K) (x y : K) :
    tangentFunction W x y≠0 := line_function_nonzero W x y (tangentSlope W x y)

theorem tangent_function_divisor (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y h=0) :
    functionDivisor W (tangentFunction W x y) (tangent_function_nonzero W x y)=
      torsionDivisor W (.some x y h) 3 := by
  ext R
  have he := congrArg (fun D : W.Point →₀ ℤ => D R)
    (three_tangent_principal_point_divisor W h h3)
  change pointOrder W (MillerNormValuationAudit.lineFunction W x y (W.slope x x y y)) R=
    (Finsupp.single (Point.some x y h) 3-Finsupp.single 0 3 : W.Point →₀ ℤ) R at he
  rw [tangent_slope_agrees W h h3] at he
  change pointOrder W (MillerNormValuationAudit.lineFunction W x y (tangentSlope W x y)) R=_
  simpa [torsionDivisor] using he

def tangentRegularAt (W : WeierstrassCurve.Affine K) (x y : K) {u v : K}
    (h : W.Nonsingular u v) : RegularRing W (.some u v h) :=
  coordinateRegular W h (lineClass W x y (tangentSlope W x y))

omit [DecidableEq K] in
theorem tangent_regular_coe (W : WeierstrassCurve.Affine K) (x y : K) {u v : K}
    (h : W.Nonsingular u v) :
    (tangentRegularAt W x y h : W.FunctionField)=tangentFunction W x y :=
  coordinate_regular_coe W h _

omit [DecidableEq K] in
theorem tangent_regular_evaluation (W : WeierstrassCurve.Affine K) (x y : K) {u v : K}
    (h : W.Nonsingular u v) :
    regularEvaluation W (.some u v h) (tangentRegularAt W x y h)=tangentValue W x y u v := by
  rw [tangentRegularAt,coordinate_regular_evaluation,line_evaluation]
  rfl

#print axioms tangent_slope_agrees
#print axioms tangent_function_divisor
#print axioms tangent_regular_evaluation
end
end WeilAuxiliaryRatioAudit
