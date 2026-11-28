import NormTransportControls

namespace EllipticNormTransportAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit
open scoped EllipticFieldNormAudit
attribute [local instance] normFunctionCommRing

/-- Actual norm local/divisor/residue/tame transport, with actual split/fixed-point,
base-versus-upstairs, characteristic2/3 and invalid-square-cancellation controls. -/
theorem actual_elliptic_norm_transport_certificate
    (K : Type*) [Field K] (W : WeierstrassCurve.Affine K) :
    (∀ f : W.FunctionFieldˣ,∀ P : W.Point,
      pointOrder W (embeddedNorm W f) P=pointOrder W f P+pointOrder W f (-P)) ∧
    (∀ f : W.FunctionFieldˣ,functionDivisor W (embeddedNorm W f) (embeddedNorm W f).ne_zero=
      functionDivisor W f f.ne_zero+divisorNeg W (functionDivisor W f f.ne_zero)) ∧
    (∀ f : W.FunctionField,baseInfinityValuation K (Algebra.norm (RatFunc K) f)=functionValuation W f ∧
      baseInfinityOrder (Algebra.norm (RatFunc K) f)=pointOrder W f 0) ∧
    (∀ c : (RatFunc K)ˣ,pointOrder W (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c) 0=
      2*baseInfinityOrder (c : RatFunc K)) ∧
    (∀ P : W.Point,∀ f : W.FunctionFieldˣ,∀ hf : pointOrder W f P=0,∀ hn : pointOrder W f (-P)=0,
      localUnitValue W P (embeddedNorm W f) (norm_local_order_zero W P f hf hn)=
        localUnitValue W P f hf*localUnitValue W (-P) f hn) ∧
    (∀ P : W.Point,∀ f g : W.FunctionFieldˣ,
      tameSymbol W P (negatedUnit W f) (negatedUnit W g)=tameSymbol W (-P) f g) ∧
    (∀ P : W.Point,∀ f g : W.FunctionFieldˣ,negatedUnit W g=g →
      tameSymbol W P (embeddedNorm W f) g=tameSymbol W P f g*tameSymbol W (-P) f g) ∧
    (∀ f : W.FunctionFieldˣ,∀ c : (RatFunc K)ˣ,
      globalTameProduct W (embeddedNorm W f) (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c)=
        (globalTameProduct W f (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c))^2) ∧
    (pointOrder W (infinityParameterUnit W) 0=1 ∧
      baseInfinityOrder (Algebra.norm (RatFunc K) (infinityParameterUnit W : W.FunctionField))=1 ∧
      pointOrder W (embeddedNorm W (infinityParameterUnit W)) 0=2 ∧
      pointOrder W (embeddedNorm W (infinityParameterUnit W)) 0≠
        baseInfinityOrder (Algebra.norm (RatFunc K) (infinityParameterUnit W : W.FunctionField))) ∧
    (MillerPrincipalIdealAudit.controlCurve.Δ≠0 ∧
    NegationPullbackAudit.controlPoint≠ -NegationPullbackAudit.controlPoint ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit NegationPullbackAudit.controlPoint=3 ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit (-NegationPullbackAudit.controlPoint)=0 ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve
      (embeddedNorm MillerPrincipalIdealAudit.controlCurve tangentUnit) NegationPullbackAudit.controlPoint=3 ∧
    pointOrder MillerPrincipalIdealAudit.controlCurve
      (embeddedNorm MillerPrincipalIdealAudit.controlCurve tangentUnit) NegationPullbackAudit.controlPoint≠
      2*pointOrder MillerPrincipalIdealAudit.controlCurve tangentUnit NegationPullbackAudit.controlPoint) ∧
    (tameCurve.Δ≠0 ∧
    (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit : ControlBase)=4 ∧
    globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit≠1 ∧
    (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit)^2=1 ∧
    globalTameProduct tameCurve (embeddedNorm tameCurve (coefficientUnit tameCurve twoUnit)) missingRootUnit=1) ∧
    (GeneralTranslationAudit.binaryCurve.Δ≠0 ∧
    pointOrder GeneralTranslationAudit.binaryCurve
      (embeddedNorm GeneralTranslationAudit.binaryCurve (infinityParameterUnit GeneralTranslationAudit.binaryCurve)) 0=2 ∧
    baseInfinityOrder (Algebra.norm (RatFunc GeneralF2)
      (infinityParameterUnit GeneralTranslationAudit.binaryCurve : GeneralTranslationAudit.binaryCurve.FunctionField))=1 ∧
    GeneralTranslationAudit.ternaryCurve.Δ≠0 ∧
    pointOrder GeneralTranslationAudit.ternaryCurve
      (embeddedNorm GeneralTranslationAudit.ternaryCurve (infinityParameterUnit GeneralTranslationAudit.ternaryCurve)) 0=2 ∧
    baseInfinityOrder (Algebra.norm (RatFunc GeneralF3)
      (infinityParameterUnit GeneralTranslationAudit.ternaryCurve : GeneralTranslationAudit.ternaryCurve.FunctionField))=1) := by
  refine ⟨actual_norm_point_order W,actual_norm_divisor W,?_,actual_base_infinity_pullback W,
    actual_norm_local_unit_residue W,tame_negation_covariance W,actual_norm_tame_orbit W,
    actual_rational_global_norm_square W,actual_infinity_factor_control W,
    actual_split_norm_order_counter,actual_square_cancellation_counter,actual_small_characteristic_transport_controls⟩
  intro f
  exact ⟨actual_norm_infinity_valuation W f,actual_norm_base_infinity_order W f⟩

#print axioms actual_elliptic_norm_transport_certificate
end
end EllipticNormTransportAudit
