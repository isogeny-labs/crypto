import AuxiliaryExistence

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit FinitePointEvaluationAudit GeneralTranslationAudit
variable {K : Type*} [Field K]

theorem linearH_function (W : WeierstrassCurve.Affine K) :
    polynomialFunction W (linearH W)=constantHom W W.a₁*xFunction W+constantHom W W.a₃ := by
  rw [linearH,map_add,map_mul,polynomial_function_constant,polynomial_function_constant]
  simp only [polynomialFunction,xFunction,WeierstrassCurve.Affine.CoordinateRing.XClass,
    map_zero,sub_zero,RingHom.comp_apply]

/-- Actual point negation sends a y-monic line to minus another y-monic line. -/
theorem negation_line (W : WeierstrassCurve.Affine K) (x y m : K) :
    fieldNeg W (lineFunction W x y m)=
      -lineFunction W x (W.negY x y) (-m-W.a₁) := by
  rw [line_formula,map_sub,map_sub,map_mul,field_neg_y,linearH_function,
    negation_constant,negation_constant,map_sub,field_neg_x,negation_constant,line_formula]
  simp only [WeierstrassCurve.Affine.negY,map_sub,map_neg,map_mul]
  ring

/-- Actual infinity leading units of any two y-monic lines have quotient residue one. -/
theorem line_quotient_near_one (W : WeierstrassCurve.Affine K) (x y m u v s : K) :
    functionValuation W (lineFunction W x y m/lineFunction W u v s-1)<1 := by
  have h := near_one_div (functionValuation W) (line_normalized W x y m) (line_normalized W u v s)
  have he : (parameter W^3*lineFunction W x y m)/(parameter W^3*lineFunction W u v s)=
      lineFunction W x y m/lineFunction W u v s := by
    field_simp [(parameter_is_uniformizer W).ne_zero,MillerPrincipalIdealAudit.line_function_nonzero W u v s]
  rwa [he] at h

theorem negated_line_quotient_near_negative_one (W : WeierstrassCurve.Affine K)
    (x y m u v s : K) :
    functionValuation W (fieldNeg W (lineFunction W x y m)/lineFunction W u v s-constantHom W (-1))<1 := by
  rw [negation_line,neg_div,map_neg (constantHom W),map_one]
  have he : -(lineFunction W x (W.negY x y) (-m-W.a₁)/lineFunction W u v s)- -1=
      -(lineFunction W x (W.negY x y) (-m-W.a₁)/lineFunction W u v s-1) := by ring
  rw [he,Valuation.map_neg]
  exact line_quotient_near_one W x (W.negY x y) (-m-W.a₁) u v s

theorem negated_line_quotient_value (W : WeierstrassCurve.Affine K) (x y m u v s : K) :
    functionValuation W (fieldNeg W (lineFunction W x y m)/lineFunction W u v s)=1 := by
  rw [negation_line,neg_div,Valuation.map_neg]
  exact near_one_value (functionValuation W)
    (line_quotient_near_one W x (W.negY x y) (-m-W.a₁) u v s)

def negatedLineQuotientRegular (W : WeierstrassCurve.Affine K) (x y m u v s : K) :
    RegularRing W 0 :=
  ⟨fieldNeg W (lineFunction W x y m)/lineFunction W u v s,
    (negated_line_quotient_value W x y m u v s).le⟩

/-- The actual minus sign survives residue evaluation, including characteristic two. -/
theorem negated_line_quotient_residue (W : WeierstrassCurve.Affine K) (x y m u v s : K) :
    regularEvaluation W 0 (negatedLineQuotientRegular W x y m u v s)= -1 :=
  (regular_evaluation_difference W 0 _ (-1)).mpr
    (negated_line_quotient_near_negative_one W x y m u v s)

#print axioms negation_line
#print axioms line_quotient_near_one
#print axioms negated_line_quotient_residue
end
end WeilAuxiliaryRatioAudit
