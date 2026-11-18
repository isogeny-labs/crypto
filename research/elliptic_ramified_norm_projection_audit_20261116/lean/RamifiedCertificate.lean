import RamifiedControls

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
attribute [local instance] normFunctionCommRing

/-- Actual normalized ramified finite projection with actual inertia and complete skeptical controls. -/
theorem actual_elliptic_ramified_norm_projection_certificate
    (K : Type*) [Field K] (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    (finiteBaseValuation x (baseVerticalUnit x : RatFunc K)=WithZero.exp (-1 : ℤ) ∧
      Function.Surjective (finiteBaseValuation x)) ∧
    (xFiber W (Point.some x y h)={Point.some x y h} ∧ (xFiber W (Point.some x y h)).ncard=1) ∧
    (∀ c : RatFunc K,localFunctionValuation W (Point.some x y h) (algebraMap (RatFunc K) W.FunctionField c)=
      (finiteBaseValuation x c)^2) ∧
    (∀ f : W.FunctionFieldˣ,finiteBaseOrder x (normUnit W f : RatFunc K)=pointOrder W f (Point.some x y h)) ∧
    (pointOrder W (finiteParameterUnit W y) (Point.some x y h)=1 ∧
      localFunctionValuation W (Point.some x y h) (finiteParameterUnit W y : W.FunctionField)=WithZero.exp (-1 : ℤ) ∧
      fixedInertiaCharacter W (Point.some x y h) hP (finiteParameterUnit W y)= -1) ∧
    (RingHom.ker (finiteBaseEvaluation W h hP)=IsLocalRing.maximalIdeal (FiniteBaseRing x) ∧
      Function.Surjective (finiteBaseEvaluation W h hP) ∧
      ∀ c : K,finiteBaseEvaluation W h hP (finiteBaseConstant x c)=c) ∧
    (∀ u : (FiniteBaseRing x).unitGroup,finiteBaseUnitEvaluation W h hP u=
      pointUnitEvaluation W (Point.some x y h) (finiteBaseUnitEmbedding W h hP u)) ∧
    (∀ W' : WeierstrassCurve.Affine K,∀ y' : K,∀ h' : W'.Nonsingular x y',
      ∀ hP' : (Point.some x y' h' : W'.Point)= -Point.some x y' h',
      (∀ r : FiniteBaseRing x,finiteBaseEvaluation W h hP r=finiteBaseEvaluation W' h' hP' r) ∧
      (∀ u : (FiniteBaseRing x).unitGroup,finiteBaseUnitEvaluation W h hP u=finiteBaseUnitEvaluation W' h' hP' u) ∧
      (∀ a b : (RatFunc K)ˣ,finiteBaseTameSymbol W h hP a b=finiteBaseTameSymbol W' h' hP' a b)) ∧
    (∀ f : W.FunctionFieldˣ,fixedInertiaCharacter W (Point.some x y h) hP f=
      (-1 : Kˣ)^pointOrder W f (Point.some x y h)) ∧
    (∀ f : W.FunctionFieldˣ,∀ c : (RatFunc K)ˣ,
      finiteBaseTameSymbol W h hP (normUnit W f) c=tameSymbol W (Point.some x y h) f (baseFieldUnit W c) ∧
      (finiteBaseTameSymbol W h hP (normUnit W f) c)⁻¹=milnorSymbol W (Point.some x y h) f (baseFieldUnit W c)) ∧
    (finiteBaseOrder x (normUnit W (finiteParameterUnit W y) : RatFunc K)=1 ∧
    finiteBaseTameSymbol W h hP (normUnit W (finiteParameterUnit W y)) (normUnit W (finiteParameterUnit W y))= -1 ∧
    tameSymbol W (Point.some x y h) (finiteParameterUnit W y) (embeddedNorm W (finiteParameterUnit W y))= -1 ∧
    pointBaseTameSymbol W (Point.some x y h) (normUnit W (finiteParameterUnit W y))
      (normUnit W (finiteParameterUnit W y))=1) ∧
    (splitCurve.Δ≠0 ∧ fixedPoint= -fixedPoint ∧
    pointOrder splitCurve (finiteParameterUnit splitCurve 0) fixedPoint=1 ∧
    finiteBaseOrder (4 : ControlBase) (normUnit splitCurve (finiteParameterUnit splitCurve 0) : RatFunc ControlBase)=1 ∧
    (finiteBaseTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 ∧
    (pointBaseTameSymbol splitCurve fixedPoint (normUnit splitCurve (finiteParameterUnit splitCurve 0))
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=1 ∧
    finiteBaseTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0))≠
    pointBaseTameSymbol splitCurve fixedPoint (normUnit splitCurve (finiteParameterUnit splitCurve 0))
      (normUnit splitCurve (finiteParameterUnit splitCurve 0))) ∧
    (ramifiedBinaryCurve.Δ≠0 ∧ ramifiedBinaryPoint= -ramifiedBinaryPoint ∧
    pointOrder ramifiedBinaryCurve (finiteParameterUnit ramifiedBinaryCurve 1) ramifiedBinaryPoint=1 ∧
    (xFiber ramifiedBinaryCurve ramifiedBinaryPoint).ncard=1 ∧
    (∀ c : RatFunc GeneralF2,localFunctionValuation ramifiedBinaryCurve ramifiedBinaryPoint
      (algebraMap (RatFunc GeneralF2) ramifiedBinaryCurve.FunctionField c)=(finiteBaseValuation (0 : GeneralF2) c)^2) ∧
    (∀ f : ramifiedBinaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF2)ˣ,
      finiteBaseTameSymbol ramifiedBinaryCurve ramified_binary_nonsingular ramified_binary_fixed (normUnit ramifiedBinaryCurve f) c=
        tameSymbol ramifiedBinaryCurve ramifiedBinaryPoint f (baseFieldUnit ramifiedBinaryCurve c) ∧
      (finiteBaseTameSymbol ramifiedBinaryCurve ramified_binary_nonsingular ramified_binary_fixed (normUnit ramifiedBinaryCurve f) c)⁻¹=
        milnorSymbol ramifiedBinaryCurve ramifiedBinaryPoint f (baseFieldUnit ramifiedBinaryCurve c)) ∧
    (finiteBaseTameSymbol ramifiedBinaryCurve ramified_binary_nonsingular ramified_binary_fixed
      (normUnit ramifiedBinaryCurve (finiteParameterUnit ramifiedBinaryCurve 1))
      (normUnit ramifiedBinaryCurve (finiteParameterUnit ramifiedBinaryCurve 1)) : GeneralF2)=1 ∧
    ramifiedTernaryCurve.Δ≠0 ∧ ramifiedTernaryPoint= -ramifiedTernaryPoint ∧
    pointOrder ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0) ramifiedTernaryPoint=1 ∧
    (xFiber ramifiedTernaryCurve ramifiedTernaryPoint).ncard=1 ∧
    (∀ c : RatFunc GeneralF3,localFunctionValuation ramifiedTernaryCurve ramifiedTernaryPoint
      (algebraMap (RatFunc GeneralF3) ramifiedTernaryCurve.FunctionField c)=(finiteBaseValuation (0 : GeneralF3) c)^2) ∧
    (∀ f : ramifiedTernaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF3)ˣ,
      finiteBaseTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed (normUnit ramifiedTernaryCurve f) c=
        tameSymbol ramifiedTernaryCurve ramifiedTernaryPoint f (baseFieldUnit ramifiedTernaryCurve c) ∧
      (finiteBaseTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed (normUnit ramifiedTernaryCurve f) c)⁻¹=
        milnorSymbol ramifiedTernaryCurve ramifiedTernaryPoint f (baseFieldUnit ramifiedTernaryCurve c)) ∧
    (finiteBaseTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed
      (normUnit ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0))
      (normUnit ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0)) : GeneralF3)=2) := by
  refine ⟨⟨finite_base_parameter_value x,finite_base_valuation_surjective x⟩,
    fixed_full_point_fiber W h hP,fixed_base_valuation W h hP,fixed_norm_base_order W h hP,
    ⟨fixed_parameter_order W h hP,fixed_parameter_value W h hP,fixed_inertia_on_parameter W h hP⟩,
    ⟨finite_base_eval_kernel W h hP,finite_base_eval_surjective W h hP,finite_base_eval_constant W h hP⟩,
    finite_base_unit_eval_compatibility W h hP,?_,actual_fixed_inertia_character W h hP,?_,
    actual_ramified_norm_parameter_values W h hP,actual_f5_raw_vs_normalized_counter,
    actual_small_characteristic_ramified_projection⟩
  · intro W' y' h' hP'
    exact ⟨finite_base_eval_model_independent W W' h hP h' hP',
      finite_base_unit_model_independent W W' h hP h' hP',finite_base_tame_model_independent W W' h hP h' hP'⟩
  · intro f c
    exact ⟨actual_ramified_norm_projection W h hP f c,actual_ramified_inverse_projection W h hP f c⟩

#print axioms actual_elliptic_ramified_norm_projection_certificate
end
end EllipticRamifiedNormProjectionAudit
