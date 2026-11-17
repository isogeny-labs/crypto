import InfinityBaseSymbol

namespace EllipticInfinityNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Actual conjugation ratio is an actual infinity local unit for every nonzero function. -/
def ratioLocalUnit (W : WeierstrassCurve.Affine K) :
    W.FunctionFieldˣ →* (RegularRing W 0).unitGroup where
  toFun f := ⟨negatedUnit W f/f,by
    apply (mem_point_units_iff W 0 _).mpr
    rw [Units.val_div_eq_div_val,point_order_div W (negatedUnit W f).ne_zero f.ne_zero]
    change pointOrder W (fieldNeg W (f : W.FunctionField)) 0-pointOrder W f 0=0
    rw [point_order_pullback]
    change pointOrder W f 0-pointOrder W f 0=0
    exact sub_self _⟩
  map_one' := by apply Subtype.ext; simp
  map_mul' a b := by
    apply Subtype.ext
    change negatedUnit W (a*b)/(a*b)=(negatedUnit W a/a)*(negatedUnit W b/b)
    rw [map_mul,div_mul_div_comm]

def inertiaCharacter (W : WeierstrassCurve.Affine K) : W.FunctionFieldˣ →* Kˣ :=
  (pointUnitEvaluation W 0).comp (ratioLocalUnit W)

/-- The actual residue action at infinity is trivial on every actual local unit. -/
theorem inertia_on_local_unit (W : WeierstrassCurve.Affine K)
    (u : (RegularRing W 0).unitGroup) : inertiaCharacter W u.val=1 := by
  have he : ratioLocalUnit W u.val=pointUnitNegation W 0 u/u := Subtype.ext rfl
  change pointUnitEvaluation W 0 (ratioLocalUnit W u.val)=1
  have hn : pointUnitEvaluation W 0 (pointUnitNegation W 0 u)=pointUnitEvaluation W 0 u :=
    point_unit_negation_value W 0 u
  rw [he,map_div,hn,div_self']

theorem polynomial_x_function (W : WeierstrassCurve.Affine K) :
    polynomialFunction W Polynomial.X=xFunction W := by
  simp only [polynomialFunction,RingHom.comp_apply,xFunction,
    WeierstrassCurve.Affine.CoordinateRing.XClass,map_zero,sub_zero]

theorem linear_h_function (W : WeierstrassCurve.Affine K) :
    polynomialFunction W (linearH W)=constantHom W W.a₁*xFunction W+constantHom W W.a₃ := by
  simp only [linearH,map_add,map_mul,polynomial_function_constant,polynomial_x_function]

theorem linear_h_over_y_small (W : WeierstrassCurve.Affine K) :
    functionValuation W (polynomialFunction W (linearH W)/yFunction W)<1 := by
  rw [linear_h_function,add_div]
  have h₁:=monomial_small W W.a₁ 1 1 (by norm_num)
  have h₃:=monomial_small W W.a₃ 0 1 (by norm_num)
  simp only [pow_one,pow_zero,mul_one] at h₁ h₃
  exact (functionValuation W).map_add_lt h₁ h₃

/-- Explicit actual identity identifies the parameter's infinitesimal error from -1. -/
theorem parameter_ratio_error (W : WeierstrassCurve.Affine K) :
    fieldNeg W (parameter W)/parameter W+1=
      -polynomialFunction W (linearH W)/fieldNeg W (yFunction W) := by
  have hx:=x_function_nonzero W
  have hy:=y_function_nonzero W
  have hn:=field_neg_nonzero W hy
  have he : fieldNeg W (parameter W)/parameter W=yFunction W/fieldNeg W (yFunction W) := by
    rw [parameter,map_div₀,field_neg_x]
    field_simp
  rw [he]
  calc
    _ = (yFunction W+fieldNeg W (yFunction W))/fieldNeg W (yFunction W) := by
      rw [add_div,div_self hn]
    _ = _ := by rw [field_neg_y];ring

theorem parameter_ratio_error_small (W : WeierstrassCurve.Affine K) :
    functionValuation W (fieldNeg W (parameter W)/parameter W+1)<1 := by
  rw [parameter_ratio_error,map_div₀,Valuation.map_neg,field_neg_norm_value]
  have he:=linear_h_over_y_small W
  rw [map_div₀] at he
  exact he

/-- Actual conjugation acts on the actual infinity parameter with leading residue -1,
including characteristic2 where -1=1. -/
theorem inertia_on_parameter (W : WeierstrassCurve.Affine K) :
    inertiaCharacter W (infinityParameterUnit W)= -1 := by
  apply Units.ext
  change regularEvaluation W 0
    ((RegularRing W 0).unitGroupMulEquiv (ratioLocalUnit W (infinityParameterUnit W)) : RegularRing W 0)=(-1 : K)
  apply (regular_evaluation_difference W 0 _ (-1)).mpr
  change functionValuation W (fieldNeg W (parameter W)/parameter W-constantHom W (-1))<1
  rw [map_neg,map_one,sub_neg_eq_add]
  exact parameter_ratio_error_small W

/-- Actual local-unit decomposition gives the inertia residue on every actual function. -/
theorem actual_inertia_character (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    inertiaCharacter W f=(-1 : Kˣ)^pointOrder W f 0 := by
  let π:=infinityParameterUnit W
  let u:=leadingLocalUnit W 0 π (infinity_parameter_order W) f
  have he : f=u.val*π^pointOrder W f 0 := by
    change f=(f/π^pointOrder W f 0)*π^pointOrder W f 0
    exact (div_mul_cancel f (π^pointOrder W f 0)).symm
  calc
    _ = inertiaCharacter W (u.val*π^pointOrder W f 0) := congrArg (inertiaCharacter W) he
    _ = _ := by rw [map_mul,map_zpow,inertia_on_local_unit,inertia_on_parameter,one_mul]

#print axioms inertia_on_local_unit
#print axioms inertia_on_parameter
#print axioms actual_inertia_character
end
end EllipticInfinityNormProjectionAudit
