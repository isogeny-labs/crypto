import NormGlobal

namespace EllipticNormTransportAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Every actual infinity parameter distinguishes the base norm order from its pullback. -/
theorem actual_infinity_factor_control (W : WeierstrassCurve.Affine K) :
    pointOrder W (infinityParameterUnit W) 0=1 ∧
    baseInfinityOrder (Algebra.norm (RatFunc K) (infinityParameterUnit W : W.FunctionField))=1 ∧
    pointOrder W (embeddedNorm W (infinityParameterUnit W)) 0=2 ∧
    pointOrder W (embeddedNorm W (infinityParameterUnit W)) 0≠
      baseInfinityOrder (Algebra.norm (RatFunc K) (infinityParameterUnit W : W.FunctionField)) := by
  have hp:=infinity_parameter_order W
  have hb : baseInfinityOrder (Algebra.norm (RatFunc K) (infinityParameterUnit W : W.FunctionField))=1 := by
    rw [actual_norm_base_infinity_order,hp]
  have hu : pointOrder W (embeddedNorm W (infinityParameterUnit W)) 0=2 := by
    rw [actual_norm_upstairs_infinity_order,hp,mul_one]
  refine ⟨hp,hb,hu,?_⟩
  rw [hu,hb]
  decide

private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
attribute [local instance] controlBaseCommRing generalF2CommRing generalF3CommRing

/-- Credited actual tangent on y²=x³+1, with orders3 at P and0 at its distinct negative. -/
def tangentUnit : MillerPrincipalIdealAudit.controlCurve.FunctionFieldˣ :=
  Units.mk0 NegationPullbackAudit.controlFunction NegationPullbackAudit.control_function_nonzero

theorem actual_split_norm_order_counter :
    MillerPrincipalIdealAudit.controlCurve.Δ≠0 ∧
    NegationPullbackAudit.controlPoint≠ -NegationPullbackAudit.controlPoint ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit NegationPullbackAudit.controlPoint=3 ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit (-NegationPullbackAudit.controlPoint)=0 ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve
      (embeddedNorm MillerPrincipalIdealAudit.controlCurve tangentUnit) NegationPullbackAudit.controlPoint=3 ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve
      (embeddedNorm MillerPrincipalIdealAudit.controlCurve tangentUnit) NegationPullbackAudit.controlPoint≠
      2*pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit NegationPullbackAudit.controlPoint := by
  have hp : pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit NegationPullbackAudit.controlPoint=3 :=
    NegationPullbackAudit.control_order_three
  have hn : pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit (-NegationPullbackAudit.controlPoint)=0 :=
    NegationPullbackAudit.control_negative_order_zero
  have hv : pointOrder MillerPrincipalIdealAudit.controlCurve
      (embeddedNorm MillerPrincipalIdealAudit.controlCurve tangentUnit) NegationPullbackAudit.controlPoint=3 := by
    rw [actual_norm_point_order,hp,hn,add_zero]
  refine ⟨MillerPrincipalIdealAudit.control_delta,NegationPullbackAudit.control_point_ne_negative,hp,hn,hv,?_⟩
  rw [hv,hp]
  decide

theorem missing_root_invariant : negatedUnit tameCurve missingRootUnit=missingRootUnit := by
  apply Units.ext
  change fieldNeg tameCurve (verticalFunction tameCurve 1)=verticalFunction tameCurve 1
  rw [vertical_formula,map_sub,field_neg_x,negation_constant]

/-- A square1 does not prove reciprocity: actual rational-only product4 remains non1. -/
theorem actual_square_cancellation_counter :
    tameCurve.Δ≠0 ∧
    (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit : ControlBase)=4 ∧
    globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit≠1 ∧
    (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit)^2=1 ∧
    globalTameProduct tameCurve (embeddedNorm tameCurve (coefficientUnit tameCurve twoUnit)) missingRootUnit=1 := by
  have hc:=f5_rational_only_product_counter
  have hs : (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit)^2=1 := by
    apply Units.ext
    change (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit : ControlBase)^2=1
    rw [hc.2.1]
    decide +kernel
  refine ⟨hc.1,hc.2.1,hc.2.2,hs,?_⟩
  rw [actual_global_norm_square tameCurve _ _ missing_root_invariant,hs]

/-- Characteristic2/3 actual nonsingular models obey the same infinity factor,
without a tame-ramification assumption. -/
theorem actual_small_characteristic_transport_controls :
    GeneralTranslationAudit.binaryCurve.Δ≠0 ∧
    pointOrder GeneralTranslationAudit.binaryCurve
      (embeddedNorm GeneralTranslationAudit.binaryCurve (infinityParameterUnit GeneralTranslationAudit.binaryCurve)) 0=2 ∧
    baseInfinityOrder (Algebra.norm (RatFunc GeneralF2)
      (infinityParameterUnit GeneralTranslationAudit.binaryCurve : GeneralTranslationAudit.binaryCurve.FunctionField))=1 ∧
    GeneralTranslationAudit.ternaryCurve.Δ≠0 ∧
    pointOrder GeneralTranslationAudit.ternaryCurve
      (embeddedNorm GeneralTranslationAudit.ternaryCurve (infinityParameterUnit GeneralTranslationAudit.ternaryCurve)) 0=2 ∧
    baseInfinityOrder (Algebra.norm (RatFunc GeneralF3)
      (infinityParameterUnit GeneralTranslationAudit.ternaryCurve : GeneralTranslationAudit.ternaryCurve.FunctionField))=1 := by
  have hb:=actual_infinity_factor_control GeneralTranslationAudit.binaryCurve
  have ht:=actual_infinity_factor_control GeneralTranslationAudit.ternaryCurve
  exact ⟨binary_delta,hb.2.2.1,hb.2.1,ternary_delta,ht.2.2.1,ht.2.1⟩

#print axioms actual_infinity_factor_control
#print axioms actual_split_norm_order_counter
#print axioms actual_square_cancellation_counter
#print axioms actual_small_characteristic_transport_controls
end
end EllipticNormTransportAudit
