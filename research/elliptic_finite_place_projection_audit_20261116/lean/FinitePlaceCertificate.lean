import FinitePlaceControls

namespace EllipticFinitePlaceProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
attribute [local instance] normFunctionCommRing

/-- Unified actual rational finite-place projection over the distinct full point fiber. -/
theorem actual_elliptic_finite_place_projection_certificate
    (K : Type*) [Field K] (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    (finiteBaseValuation x (baseVerticalUnit x : RatFunc K)=WithZero.exp (-1 : ℤ) ∧
      Function.Surjective (finiteBaseValuation x)) ∧
    ((finiteFiber W (Point.some x y h) : Set W.Point)=xFiber W (Point.some x y h) ∧
      0<finitePointIndex W (Point.some x y h) ∧
      finitePointIndex W (Point.some x y h)*(finiteFiber W (Point.some x y h)).card=2) ∧
    (∀ c : RatFunc K,localFunctionValuation W (Point.some x y h) (algebraMap (RatFunc K) W.FunctionField c)=
      (finiteBaseValuation x c)^(finitePointIndex W (Point.some x y h))) ∧
    (∀ f : W.FunctionFieldˣ,finiteBaseOrder x (normUnit W f : RatFunc K)=
      ∑ R ∈ finiteFiber W (Point.some x y h),pointOrder W f R) ∧
    (RingHom.ker (finitePlaceEvaluation W h)=IsLocalRing.maximalIdeal (FiniteBaseRing x) ∧
      Function.Surjective (finitePlaceEvaluation W h) ∧
      ∀ c : K,finitePlaceEvaluation W h (finiteBaseConstant x c)=c) ∧
    (∀ u : (FiniteBaseRing x).unitGroup,finitePlaceUnitEvaluation W h u=
      pointUnitEvaluation W (Point.some x y h) (finitePlaceUnitEmbedding W h u)) ∧
    (∀ W' : WeierstrassCurve.Affine K,∀ y' : K,∀ h' : W'.Nonsingular x y',
      (∀ r : FiniteBaseRing x,finitePlaceEvaluation W h r=finitePlaceEvaluation W' h' r) ∧
      (∀ u : (FiniteBaseRing x).unitGroup,finitePlaceUnitEvaluation W h u=finitePlaceUnitEvaluation W' h' u) ∧
      (∀ a b : (RatFunc K)ˣ,finitePlaceTameSymbol W h a b=finitePlaceTameSymbol W' h' a b)) ∧
    (∀ f : W.FunctionFieldˣ,∀ c : (RatFunc K)ˣ,
      finitePlaceTameSymbol W h (normUnit W f) c=
        (∏ R ∈ finiteFiber W (Point.some x y h),tameSymbol W R f (baseFieldUnit W c)) ∧
      (finitePlaceTameSymbol W h (normUnit W f) c)⁻¹=
        (∏ R ∈ finiteFiber W (Point.some x y h),milnorSymbol W R f (baseFieldUnit W c))) ∧
    (splitCurve.Δ≠0 ∧ finitePointIndex splitCurve splitPoint=1 ∧ (finiteFiber splitCurve splitPoint).card=2 ∧
    (finitePlaceTameSymbol splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
      (normUnit splitCurve tangentUnit) (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 ∧
    (finiteFiberTameProduct splitCurve splitPoint tangentUnit (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 ∧
    (tameSymbol splitCurve splitPoint tangentUnit splitXUnit : ControlBase)=2 ∧
    finitePlaceTameSymbol splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
      (normUnit splitCurve tangentUnit) (baseVerticalUnit (0 : ControlBase))≠tameSymbol splitCurve splitPoint tangentUnit splitXUnit) ∧
    (splitCurve.Δ≠0 ∧ finitePointIndex splitCurve fixedPoint=2 ∧ (finiteFiber splitCurve fixedPoint).card=1 ∧
    (finitePlaceTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 ∧
    (finiteFiberTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 ∧
    (pairedTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=1 ∧
    finiteFiberTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0))≠
    pairedTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0))) ∧
    (binaryCurve.Δ≠0 ∧ ramifiedBinaryCurve.Δ≠0 ∧ binaryPoint≠ -binaryPoint ∧ ramifiedBinaryPoint= -ramifiedBinaryPoint ∧
    finitePointIndex binaryCurve binaryPoint=1 ∧ finitePointIndex ramifiedBinaryCurve ramifiedBinaryPoint=2 ∧
    (finiteFiber binaryCurve binaryPoint).card=2 ∧ (finiteFiber ramifiedBinaryCurve ramifiedBinaryPoint).card=1 ∧
    (∀ r : FiniteBaseRing (0 : GeneralF2),finitePlaceEvaluation binaryCurve binary_nonsingular r=
      finitePlaceEvaluation ramifiedBinaryCurve ramified_binary_nonsingular r) ∧
    (∀ u : (FiniteBaseRing (0 : GeneralF2)).unitGroup,finitePlaceUnitEvaluation binaryCurve binary_nonsingular u=
      finitePlaceUnitEvaluation ramifiedBinaryCurve ramified_binary_nonsingular u) ∧
    (∀ a b : (RatFunc GeneralF2)ˣ,finitePlaceTameSymbol binaryCurve binary_nonsingular a b=
      finitePlaceTameSymbol ramifiedBinaryCurve ramified_binary_nonsingular a b) ∧
    (∀ f : binaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF2)ˣ,
      finitePlaceTameSymbol binaryCurve binary_nonsingular (normUnit binaryCurve f) c=finiteFiberTameProduct binaryCurve binaryPoint f c ∧
      (finitePlaceTameSymbol binaryCurve binary_nonsingular (normUnit binaryCurve f) c)⁻¹=
        ∏ R ∈ finiteFiber binaryCurve binaryPoint,milnorSymbol binaryCurve R f (baseFieldUnit binaryCurve c)) ∧
    (∀ f : ramifiedBinaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF2)ˣ,
      finitePlaceTameSymbol ramifiedBinaryCurve ramified_binary_nonsingular (normUnit ramifiedBinaryCurve f) c=
        finiteFiberTameProduct ramifiedBinaryCurve ramifiedBinaryPoint f c ∧
      (finitePlaceTameSymbol ramifiedBinaryCurve ramified_binary_nonsingular (normUnit ramifiedBinaryCurve f) c)⁻¹=
        ∏ R ∈ finiteFiber ramifiedBinaryCurve ramifiedBinaryPoint,milnorSymbol ramifiedBinaryCurve R f (baseFieldUnit ramifiedBinaryCurve c))) ∧
    (ramifiedTernaryCurve.Δ≠0 ∧ ramifiedTernaryPoint= -ramifiedTernaryPoint ∧
    finitePointIndex ramifiedTernaryCurve ramifiedTernaryPoint=2 ∧ (finiteFiber ramifiedTernaryCurve ramifiedTernaryPoint).card=1 ∧
    (finitePlaceTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular
      (normUnit ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0))
      (normUnit ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0)) : GeneralF3)=2 ∧
    (∀ f : ramifiedTernaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF3)ˣ,
      finitePlaceTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular (normUnit ramifiedTernaryCurve f) c=
        finiteFiberTameProduct ramifiedTernaryCurve ramifiedTernaryPoint f c ∧
      (finitePlaceTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular (normUnit ramifiedTernaryCurve f) c)⁻¹=
        ∏ R ∈ finiteFiber ramifiedTernaryCurve ramifiedTernaryPoint,milnorSymbol ramifiedTernaryCurve R f (baseFieldUnit ramifiedTernaryCurve c))) := by
  refine ⟨⟨finite_base_parameter_value x,finite_base_valuation_surjective x⟩,
    ⟨finite_fiber_coe W _,finite_point_index_pos W _,finite_index_times_fiber_card W _⟩,
    actual_finite_place_valuation W h,actual_finite_norm_order W h,
    ⟨finite_place_eval_kernel W h,finite_place_eval_surjective W h,finite_place_eval_constant W h⟩,
    finite_place_unit_eval_compatibility W h,?_,?_,
    actual_f5_common_split_omission,actual_f5_common_fixed_duplication,
    actual_binary_mixed_model_canonical,actual_ternary_common_positive⟩
  · intro W' y' h'
    exact ⟨finite_place_eval_model_independent W W' h h',finite_place_unit_model_independent W W' h h',
      finite_place_tame_model_independent W W' h h'⟩
  · intro f c
    exact ⟨actual_finite_place_norm_projection W h f c,actual_finite_place_inverse_projection W h f c⟩

#print axioms actual_elliptic_finite_place_projection_certificate
end
end EllipticFinitePlaceProjectionAudit
