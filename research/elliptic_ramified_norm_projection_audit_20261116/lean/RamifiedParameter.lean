import RamifiedPullback

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

def finiteParameterUnit (W : WeierstrassCurve.Affine K) (b : K) : W.FunctionFieldˣ :=
  Units.mk0 (lineFunction W 0 b 0) (MillerPrincipalIdealAudit.line_function_nonzero W 0 b 0)

theorem finite_parameter_formula (W : WeierstrassCurve.Affine K) (b : K) :
    (finiteParameterUnit W b : W.FunctionField)=yFunction W-constantHom W b := by
  change lineFunction W 0 b 0=_
  rw [line_formula,map_zero,zero_mul,sub_zero]

theorem finite_y_coordinate_image (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    pointToField W h.1 (localV W h.1)=(finiteParameterUnit W y : W.FunctionField) := by
  rw [localV,point_to_field_map,finite_parameter_formula]
  have hy : YClass W (C y)=YClass W (C 0)-algebraMap K W.CoordinateRing y := by
    simp only [YClass,map_sub,map_zero,sub_zero]
    rfl
  rw [hy,map_sub]
  simp only [yFunction,map_zero,coefficient_eq_mk,constantHom,RingHom.comp_apply]

theorem fixed_derivative_y_zero (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    derivativeY W x y=0 := by
  have he : y=W.negY x y:=(Point.some.inj (hP.trans (Point.neg_some h))).2
  simp only [WeierstrassCurve.Affine.negY] at he
  unfold derivativeY
  linear_combination he

/-- Actual finite DVR, not a uniformizer assumption, gives Y-b simple at an actual fixed point. -/
theorem fixed_parameter_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    localFunctionValuation W (Point.some x y h) (finiteParameterUnit W y : W.FunctionField)=WithZero.exp (-1 : ℤ) := by
  classical
  have hd:=fixed_derivative_y_zero W h hP
  have he:=finite_adic_parameter W h
  rw [pointParameter,ite_eq_right (not_ne_iff.mpr hd),finite_y_coordinate_image W h] at he
  exact he

theorem fixed_parameter_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    pointOrder W (finiteParameterUnit W y) (Point.some x y h)=1 := by
  rw [point_order_formula,fixed_parameter_value W h hP,WithZero.log_exp]
  norm_num

theorem fixed_full_point_fiber (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    xFiber W (Point.some x y h)={Point.some x y h} ∧ (xFiber W (Point.some x y h)).ncard=1 := by
  have he : xFiber W (Point.some x y h)={Point.some x y h} := by
    rw [actual_x_fiber,←hP]
    simp
  exact ⟨he,by rw [he,Set.ncard_singleton]⟩

#print axioms fixed_derivative_y_zero
#print axioms fixed_parameter_value
#print axioms fixed_parameter_order
#print axioms fixed_full_point_fiber
end
end EllipticRamifiedNormProjectionAudit
