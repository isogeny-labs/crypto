import RamifiedBaseResidue

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

def fixedRatioLocalUnit (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P= -P) :
    W.FunctionFieldˣ →* (RegularRing W P).unitGroup where
  toFun f:=⟨negatedUnit W f/f,by
    apply (mem_point_units_iff W P _).mpr
    rw [Units.val_div_eq_div_val,point_order_div W (negatedUnit W f).ne_zero f.ne_zero]
    change pointOrder W (fieldNeg W (f : W.FunctionField)) P-pointOrder W f P=0
    rw [point_order_pullback,←hP,sub_self]⟩
  map_one':=by apply Subtype.ext;simp
  map_mul' a b:=by
    apply Subtype.ext
    change negatedUnit W (a*b)/(a*b)=(negatedUnit W a/a)*(negatedUnit W b/b)
    rw [map_mul,div_mul_div_comm]

def fixedInertiaCharacter (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P= -P) :
    W.FunctionFieldˣ →* Kˣ := (pointUnitEvaluation W P).comp (fixedRatioLocalUnit W P hP)

theorem local_unit_value_point_congr (W : WeierstrassCurve.Affine K) {P Q : W.Point}
    (hPQ : P=Q) (f : W.FunctionFieldˣ) (hfP : pointOrder W f P=0) (hfQ : pointOrder W f Q=0) :
    localUnitValue W P f hfP=localUnitValue W Q f hfQ := by
  subst Q
  rfl

theorem fixed_inertia_on_local_unit (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P= -P)
    (u : (RegularRing W P).unitGroup) : fixedInertiaCharacter W P hP u.val=1 := by
  have ho : pointOrder W u.val P=0 := (mem_point_units_iff W P _).mp u.property
  have hn : pointOrder W u.val (-P)=0 := by rw [←hP];exact ho
  let v : (RegularRing W P).unitGroup:=⟨negatedUnit W u.val,by
    apply (mem_point_units_iff W P _).mpr
    change pointOrder W (fieldNeg W (u.val : W.FunctionField)) P=0
    rw [point_order_pullback]
    exact hn⟩
  have hv : pointUnitEvaluation W P v=pointUnitEvaluation W P u := by
    have he:=local_unit_value_negation W P u.val hn
    exact he.trans (local_unit_value_point_congr W hP.symm u.val hn ho)
  have he : fixedRatioLocalUnit W P hP u.val=v/u := Subtype.ext rfl
  change pointUnitEvaluation W P (fixedRatioLocalUnit W P hP u.val)=1
  rw [he,_root_.map_div,hv,div_self']

/-- The actual parameter conjugation error is controlled by actual orders2 and1. -/
theorem fixed_parameter_ratio_error (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    fieldNeg W (finiteParameterUnit W y : W.FunctionField)/(finiteParameterUnit W y : W.FunctionField)+1=
      -(constantHom W W.a₁)*verticalFunction W x/(finiteParameterUnit W y : W.FunctionField) := by
  have hn : yFunction W-constantHom W y≠0 := by
    rw [←finite_parameter_formula]
    exact (finiteParameterUnit W y).ne_zero
  have hc:=congrArg (constantHom W) (fixed_derivative_y_zero W h hP)
  simp only [derivativeY,map_add,map_mul,map_ofNat,map_zero] at hc
  rw [finite_parameter_formula,map_sub,field_neg_y,negation_constant,linear_h_function,vertical_formula]
  field_simp [hn]
  linear_combination -hc

theorem fixed_parameter_ratio_error_small (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    localFunctionValuation W (Point.some x y h)
      (fieldNeg W (finiteParameterUnit W y : W.FunctionField)/(finiteParameterUnit W y : W.FunctionField)+1)<1 := by
  rw [fixed_parameter_ratio_error W h hP,map_div₀,map_mul,Valuation.map_neg,
    fixed_vertical_value W h hP,fixed_parameter_value W h hP]
  have hi:=local_constant_integral W (Point.some x y h) W.a₁
  calc
    _ ≤ 1*WithZero.exp (-2 : ℤ)/WithZero.exp (-1 : ℤ) := by
      gcongr
      exact zero_le
    _ < 1 := by
      rw [one_mul,←WithZero.exp_sub]
      norm_num

/-- Actual derived inertia sign, including binary sign collapse. -/
theorem fixed_inertia_on_parameter (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    fixedInertiaCharacter W (Point.some x y h) hP (finiteParameterUnit W y)= -1 := by
  apply Units.ext
  change regularEvaluation W (Point.some x y h)
    ((RegularRing W (Point.some x y h)).unitGroupMulEquiv
      (fixedRatioLocalUnit W (Point.some x y h) hP (finiteParameterUnit W y)) : RegularRing W (Point.some x y h))=(-1 : K)
  apply (regular_evaluation_difference W (Point.some x y h) _ (-1)).mpr
  change localFunctionValuation W (Point.some x y h)
    (fieldNeg W (finiteParameterUnit W y : W.FunctionField)/(finiteParameterUnit W y : W.FunctionField)-constantHom W (-1))<1
  rw [map_neg,map_one,sub_neg_eq_add]
  exact fixed_parameter_ratio_error_small W h hP

theorem actual_fixed_inertia_character (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h)
    (f : W.FunctionFieldˣ) : fixedInertiaCharacter W (Point.some x y h) hP f=
      (-1 : Kˣ)^pointOrder W f (Point.some x y h) := by
  let π:=finiteParameterUnit W y
  let u:=leadingLocalUnit W (Point.some x y h) π (fixed_parameter_order W h hP) f
  have he : f=u.val*π^pointOrder W f (Point.some x y h) := by
    change f=(f/π^pointOrder W f (Point.some x y h))*π^pointOrder W f (Point.some x y h)
    exact (div_mul_cancel f (π^pointOrder W f (Point.some x y h))).symm
  calc
    _ = fixedInertiaCharacter W (Point.some x y h) hP (u.val*π^pointOrder W f (Point.some x y h)) :=
      congrArg (fixedInertiaCharacter W (Point.some x y h) hP) he
    _ = _ := by
      rw [map_mul,map_zpow,fixed_inertia_on_local_unit,fixed_inertia_on_parameter,one_mul]

#print axioms fixed_inertia_on_local_unit
#print axioms fixed_parameter_ratio_error
#print axioms fixed_inertia_on_parameter
#print axioms actual_fixed_inertia_character
end
end EllipticRamifiedNormProjectionAudit
