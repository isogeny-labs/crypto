import SplitControls

namespace EllipticSplitNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
attribute [local instance] normFunctionCommRing

/-- Actual normalized split finite projection, actual base residue and complete actual controls. -/
theorem actual_elliptic_split_norm_projection_certificate
    (K : Type*) [Field K] (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) :
    (pointBaseValuation W (-Point.some x y h)=pointBaseValuation W (Point.some x y h)) ∧
    (RingHom.ker (pointBaseEvaluation W (Point.some x y h))=
      IsLocalRing.maximalIdeal (PointBaseRing W (Point.some x y h)) ∧
      Function.Surjective (pointBaseEvaluation W (Point.some x y h)) ∧
      ∀ c : K,pointBaseEvaluation W (Point.some x y h) (pointBaseConstant W (Point.some x y h) c)=c) ∧
    (∀ u : (PointBaseRing W (Point.some x y h)).unitGroup,
      pointBaseUnitEvaluation W (Point.some x y h) u=
        pointUnitEvaluation W (Point.some x y h) (pointBaseUnitEmbedding W (Point.some x y h) u)) ∧
    (∀ f : W.FunctionFieldˣ,
      pointBaseOrder W (Point.some x y h) (normUnit W f : RatFunc K)=
        pointOrder W f (Point.some x y h)+pointOrder W f (-Point.some x y h)) ∧
    ((xFiber W (Point.some x y h)={Point.some x y h,-Point.some x y h} ∧
      (xFiber W (Point.some x y h)).ncard=2) ∧
    (pointBaseOrder W (Point.some x y h) (baseVerticalUnit x : RatFunc K)=1 ∧
      pointBaseValuation W (Point.some x y h) (baseVerticalUnit x : RatFunc K)=WithZero.exp (-1 : ℤ) ∧
      Function.Surjective (pointBaseValuation W (Point.some x y h))) ∧
    (∀ f : W.FunctionFieldˣ,∀ c : (RatFunc K)ˣ,
      pointBaseTameSymbol W (Point.some x y h) (normUnit W f) c=
        tameSymbol W (Point.some x y h) f (baseFieldUnit W c)*tameSymbol W (-Point.some x y h) f (baseFieldUnit W c) ∧
      (pointBaseTameSymbol W (Point.some x y h) (normUnit W f) c)⁻¹=
        milnorSymbol W (Point.some x y h) f (baseFieldUnit W c)*milnorSymbol W (-Point.some x y h) f (baseFieldUnit W c))) ∧
    (splitCurve.Δ≠0 ∧ splitPoint≠ -splitPoint ∧
    (xFiber splitCurve splitPoint).ncard=2 ∧
    (tameSymbol splitCurve splitPoint tangentUnit splitXUnit : ControlBase)=2 ∧
    (tameSymbol splitCurve (-splitPoint) tangentUnit splitXUnit : ControlBase)=3 ∧
    (pointBaseTameSymbol splitCurve splitPoint (normUnit splitCurve tangentUnit)
      (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 ∧
    pointBaseTameSymbol splitCurve splitPoint (normUnit splitCurve tangentUnit)
      (baseVerticalUnit (0 : ControlBase))≠tameSymbol splitCurve splitPoint tangentUnit splitXUnit) ∧
    (splitCurve.Δ≠0 ∧ fixedPoint= -fixedPoint ∧
    pointBaseOrder splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)=2 ∧
    pointBaseOrder splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)≠1 ∧
    pointBaseValuation splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)=WithZero.exp (-2 : ℤ) ∧
    pointBaseValuation splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)≠WithZero.exp (-1 : ℤ)) ∧
    (binaryCurve.Δ≠0 ∧ binaryPoint≠ -binaryPoint ∧
    (xFiber binaryCurve binaryPoint).ncard=2 ∧
    pointBaseOrder binaryCurve binaryPoint (baseVerticalUnit (0 : GeneralF2) : RatFunc GeneralF2)=1 ∧
    pointBaseValuation binaryCurve binaryPoint (baseVerticalUnit (0 : GeneralF2) : RatFunc GeneralF2)=WithZero.exp (-1 : ℤ) ∧
    (∀ f : binaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF2)ˣ,
      pointBaseTameSymbol binaryCurve binaryPoint (normUnit binaryCurve f) c=
        tameSymbol binaryCurve binaryPoint f (baseFieldUnit binaryCurve c)*
          tameSymbol binaryCurve (-binaryPoint) f (baseFieldUnit binaryCurve c)) ∧
    ternaryCurve.Δ≠0 ∧ ternaryPoint≠ -ternaryPoint ∧
    (xFiber ternaryCurve ternaryPoint).ncard=2 ∧
    pointBaseOrder ternaryCurve ternaryPoint (baseVerticalUnit (0 : GeneralF3) : RatFunc GeneralF3)=1 ∧
    pointBaseValuation ternaryCurve ternaryPoint (baseVerticalUnit (0 : GeneralF3) : RatFunc GeneralF3)=WithZero.exp (-1 : ℤ) ∧
    (∀ f : ternaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF3)ˣ,
      pointBaseTameSymbol ternaryCurve ternaryPoint (normUnit ternaryCurve f) c=
        tameSymbol ternaryCurve ternaryPoint f (baseFieldUnit ternaryCurve c)*
          tameSymbol ternaryCurve (-ternaryPoint) f (baseFieldUnit ternaryCurve c))) := by
  exact ⟨point_base_valuation_neg W _,
    ⟨point_base_eval_kernel W _,point_base_eval_surjective W _,point_base_eval_constant W _⟩,
    point_base_unit_eval_compatibility W _,actual_point_base_norm_order W _,
    actual_split_norm_projection W h hP,actual_f5_missing_conjugate_factor,
    actual_f5_dropped_split_guard,actual_small_characteristic_split_projection⟩

#print axioms actual_elliptic_split_norm_projection_certificate
end
end EllipticSplitNormProjectionAudit
