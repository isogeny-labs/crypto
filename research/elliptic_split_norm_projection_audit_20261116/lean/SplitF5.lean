import SplitSymbol

namespace EllipticSplitNormProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
attribute [local instance] normFunctionCommRing controlBaseCommRing

abbrev splitCurve : WeierstrassCurve.Affine ControlBase:=tameCurve
abbrev splitPoint : splitCurve.Point:=NegationPullbackAudit.controlPoint

def splitXUnit : splitCurve.FunctionFieldˣ := baseFieldUnit splitCurve (baseVerticalUnit (0 : ControlBase))

theorem f5_tangent_order : pointOrder splitCurve tangentUnit splitPoint=3 :=
  NegationPullbackAudit.control_order_three

theorem f5_tangent_negative_order : pointOrder splitCurve tangentUnit (-splitPoint)=0 :=
  NegationPullbackAudit.control_negative_order_zero

theorem f5_x_field_formula : (splitXUnit : splitCurve.FunctionField)=xFunction splitCurve := by
  change algebraMap (RatFunc ControlBase) splitCurve.FunctionField (baseVerticalUnit (0 : ControlBase) : RatFunc ControlBase)=_
  rw [base_vertical_unit_image,vertical_formula,map_zero,sub_zero]

theorem f5_x_order : pointOrder splitCurve splitXUnit splitPoint=1 := by
  have he:=split_base_parameter_order splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
    NegationPullbackAudit.control_point_ne_negative
  rw [point_base_order_embedding] at he
  exact he

theorem f5_x_negative_order : pointOrder splitCurve splitXUnit (-splitPoint)=1 := by
  have he:=point_order_pullback splitCurve (splitXUnit : splitCurve.FunctionField) splitPoint
  have hi : fieldNeg splitCurve (splitXUnit : splitCurve.FunctionField)=(splitXUnit : splitCurve.FunctionField) :=
    conjugation_fixes_base splitCurve (baseVerticalUnit (0 : ControlBase))
  rw [hi] at he
  exact he.symm.trans f5_x_order

theorem f5_tangent_field_formula : (tangentUnit : splitCurve.FunctionField)=yFunction splitCurve-1 := by
  change lineFunction splitCurve 0 1 (splitCurve.slope 0 0 1 1)=_
  have hs : splitCurve.slope 0 0 1 1=0 := by decide +kernel
  rw [hs,line_formula,map_one,map_zero,zero_mul,sub_zero]

theorem f5_curve_identity : (yFunction splitCurve)^2=(xFunction splitCurve)^3+1 := by
  have he:=function_curve_relation splitCurve
  have h₁ : splitCurve.a₁=0:=rfl
  have h₂ : splitCurve.a₂=0:=rfl
  have h₃ : splitCurve.a₃=0:=rfl
  have h₄ : splitCurve.a₄=0:=rfl
  have h₆ : splitCurve.a₆=1:=rfl
  simp only [h₁,h₂,h₃,h₄,h₆,map_zero,map_one,zero_mul,add_zero] at he
  exact he

theorem f5_y_add_one_nonzero : yFunction splitCurve+1≠0 := by
  have he:=MillerPrincipalIdealAudit.line_function_nonzero splitCurve 0 (-1) 0
  have hl : lineFunction splitCurve 0 (-1) 0=yFunction splitCurve+1 := by
    rw [line_formula,map_neg,map_one,map_zero,zero_mul,sub_zero,sub_neg_eq_add]
  change lineFunction splitCurve 0 (-1) 0≠0 at he
  rwa [hl] at he

/-- Actual field cancellation produces a fraction with a genuinely nonvanishing denominator at P. -/
theorem f5_cancelled_tangent_fraction :
    (tangentUnit : splitCurve.FunctionField)/(xFunction splitCurve)^3=1/(yFunction splitCurve+1) := by
  rw [f5_tangent_field_formula]
  have he:=f5_curve_identity
  field_simp [x_function_nonzero splitCurve,f5_y_add_one_nonzero]
  linear_combination he

theorem f5_positive_balanced_fraction :
    ((balancedLocalUnit splitCurve splitPoint tangentUnit splitXUnit).val : splitCurve.FunctionField)=
      1/(yFunction splitCurve+1) := by
  change ((tangentUnit^pointOrder splitCurve splitXUnit splitPoint/
    splitXUnit^pointOrder splitCurve tangentUnit splitPoint : splitCurve.FunctionFieldˣ) : splitCurve.FunctionField)=_
  rw [f5_x_order,f5_tangent_order,zpow_one,Units.val_div_eq_div_val,Units.val_zpow_eq_zpow_val]
  simpa only [zpow_ofNat,f5_x_field_formula] using f5_cancelled_tangent_fraction

theorem f5_positive_balanced_value :
    (pointUnitEvaluation splitCurve splitPoint
      (balancedLocalUnit splitCurve splitPoint tangentUnit splitXUnit) : ControlBase)=3 := by
  have hb : MillerPrincipalIdealAudit.pointEval splitCurve 0 1 MillerPrincipalIdealAudit.control_positive_nonsingular.1
      (YClass splitCurve 0+1)≠0 := by
    rw [map_add,MillerPrincipalIdealAudit.point_eval_YClass,map_one,Polynomial.eval_zero,sub_zero]
    decide +kernel
  have hf : ((balancedLocalUnit splitCurve splitPoint tangentUnit splitXUnit).val : splitCurve.FunctionField)=
      algebraMap splitCurve.CoordinateRing splitCurve.FunctionField 1/
        algebraMap splitCurve.CoordinateRing splitCurve.FunctionField (YClass splitCurve 0+1) := by
    rw [f5_positive_balanced_fraction,map_one,map_add,map_one]
    rfl
  have hv:=point_unit_fraction_value splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
    1 (YClass splitCurve 0+1) hb (balancedLocalUnit splitCurve splitPoint tangentUnit splitXUnit) hf
  change (pointUnitEvaluation splitCurve splitPoint
    (balancedLocalUnit splitCurve splitPoint tangentUnit splitXUnit) : ControlBase)=_ at hv
  calc
    _ = (1 : ControlBase)/(1+1) := by
      simpa only [splitPoint,NegationPullbackAudit.controlPoint,map_one,map_add,MillerPrincipalIdealAudit.point_eval_YClass,Polynomial.eval_zero,sub_zero] using hv
    _ = 3 := by decide +kernel

theorem f5_negative_local_value :
    (localUnitValue splitCurve (-splitPoint) tangentUnit f5_tangent_negative_order : ControlBase)=3 := by
  let u : (RegularRing splitCurve (-splitPoint)).unitGroup:=
    ⟨tangentUnit,(mem_point_units_iff splitCurve (-splitPoint) tangentUnit).mpr f5_tangent_negative_order⟩
  have hN:= (splitCurve.nonsingular_neg 0 1).mpr MillerPrincipalIdealAudit.control_positive_nonsingular
  have hf : (tangentUnit : splitCurve.FunctionField)=
      algebraMap splitCurve.CoordinateRing splitCurve.FunctionField (YClass splitCurve 0-1)/
        algebraMap splitCurve.CoordinateRing splitCurve.FunctionField 1 := by
    rw [f5_tangent_field_formula,map_sub,map_one,div_one]
    rfl
  have hv:=point_unit_fraction_value splitCurve hN (YClass splitCurve 0-1) 1
    (by rw [map_one];exact one_ne_zero) u hf
  change (localUnitValue splitCurve (-splitPoint) tangentUnit f5_tangent_negative_order : ControlBase)=_ at hv
  calc
    _ = splitCurve.negY 0 1-1 := by
      simpa only [splitPoint,NegationPullbackAudit.controlPoint,Point.neg_some,localUnitValue,u,map_sub,MillerPrincipalIdealAudit.point_eval_YClass,Polynomial.eval_zero,sub_zero,map_one,div_one] using hv
    _ = 3 := by decide +kernel

/-- Both actual local factors, evaluated on their actual proved local unit domains. -/
theorem actual_f5_split_factors :
    (tameSymbol splitCurve splitPoint tangentUnit splitXUnit : ControlBase)=2 ∧
    (tameSymbol splitCurve (-splitPoint) tangentUnit splitXUnit : ControlBase)=3 := by
  constructor
  · simp only [tameSymbol,Units.val_mul,Units.val_zpow_eq_zpow_val,Units.coe_neg_one,
      f5_tangent_order,f5_x_order]
    rw [f5_positive_balanced_value]
    decide +kernel
  · rw [tame_first_unit splitCurve (-splitPoint) tangentUnit splitXUnit f5_tangent_negative_order,
      f5_x_negative_order,zpow_one]
    exact f5_negative_local_value

#print axioms f5_cancelled_tangent_fraction
#print axioms f5_positive_balanced_value
#print axioms f5_negative_local_value
#print axioms actual_f5_split_factors
end
end EllipticSplitNormProjectionAudit
