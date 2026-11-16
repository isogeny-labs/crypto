import FinitePlaceProjection

namespace EllipticFinitePlaceProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

def finiteFiberTameProduct (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) : Kˣ :=
  ∏ R ∈ finiteFiber W P,tameSymbol W R f (baseFieldUnit W c)

def pairedTameProduct (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) : Kˣ :=
  tameSymbol W P f (baseFieldUnit W c)*tameSymbol W (-P) f (baseFieldUnit W c)

theorem finite_place_projection_as_fiber_product (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    finitePlaceTameSymbol W h (normUnit W f) c=finiteFiberTameProduct W (Point.some x y h) f c :=
  actual_finite_place_norm_projection W h f c

theorem paired_tame_product_raw (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    pairedTameProduct W P f c=pointBaseTameSymbol W P (normUnit W f) c :=
  (point_base_norm_orbit W P f c).symm

theorem finite_point_index_split (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P≠ -P) :
    finitePointIndex W P=1 := by
  classical
  rw [finitePointIndex,ite_eq_right hP]

theorem finite_point_index_fixed (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P= -P) :
    finitePointIndex W P=2 := by
  classical
  rw [finitePointIndex,ite_eq_left hP]

private instance : Fact (Nat.Prime 2):=⟨by decide⟩
private instance : Fact (Nat.Prime 3):=⟨by decide⟩
private instance : Fact (Nat.Prime 5):=⟨by decide⟩
attribute [local instance] controlBaseCommRing generalF2CommRing generalF3CommRing

/-- Omitting a genuine other point from the full actual split fiber gives the wrong value. -/
theorem actual_f5_common_split_omission :
    splitCurve.Δ≠0 ∧ finitePointIndex splitCurve splitPoint=1 ∧ (finiteFiber splitCurve splitPoint).card=2 ∧
    (finitePlaceTameSymbol splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
      (normUnit splitCurve tangentUnit) (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 ∧
    (finiteFiberTameProduct splitCurve splitPoint tangentUnit (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 ∧
    (tameSymbol splitCurve splitPoint tangentUnit splitXUnit : ControlBase)=2 ∧
    finitePlaceTameSymbol splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
      (normUnit splitCurve tangentUnit) (baseVerticalUnit (0 : ControlBase))≠tameSymbol splitCurve splitPoint tangentUnit splitXUnit := by
  have hn : (finitePlaceTameSymbol splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
      (normUnit splitCurve tangentUnit) (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 := by
    rw [finite_place_projection_split splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
      NegationPullbackAudit.control_point_ne_negative,Units.val_mul]
    change (tameSymbol splitCurve splitPoint tangentUnit splitXUnit : ControlBase)*
      (tameSymbol splitCurve (-splitPoint) tangentUnit splitXUnit : ControlBase)=1
    rw [actual_f5_split_factors.1,actual_f5_split_factors.2]
    decide +kernel
  have hp : (finiteFiberTameProduct splitCurve splitPoint tangentUnit (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 :=
    (congrArg (fun z : ControlBaseˣ=>(z : ControlBase))
      (finite_place_projection_as_fiber_product splitCurve MillerPrincipalIdealAudit.control_positive_nonsingular
        tangentUnit (baseVerticalUnit (0 : ControlBase))).symm).trans hn
  refine ⟨MillerPrincipalIdealAudit.control_delta,
    finite_point_index_split splitCurve splitPoint NegationPullbackAudit.control_point_ne_negative,
    finite_fiber_split_card splitCurve splitPoint NegationPullbackAudit.control_point_ne_negative,
    hn,hp,actual_f5_split_factors.1,?_⟩
  intro he
  have hc:=congrArg (fun z : ControlBaseˣ=>(z : ControlBase)) he
  rw [hn,actual_f5_split_factors.1] at hc
  exact (by decide +kernel : (1 : ControlBase)≠2) hc

/-- A duplicated fixed point gives raw1 instead of the actual distinct-fiber value4. -/
theorem actual_f5_common_fixed_duplication :
    splitCurve.Δ≠0 ∧ finitePointIndex splitCurve fixedPoint=2 ∧ (finiteFiber splitCurve fixedPoint).card=1 ∧
    (finitePlaceTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 ∧
    (finiteFiberTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 ∧
    (pairedTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=1 ∧
    finiteFiberTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0))≠
    pairedTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) := by
  have hv:=actual_ramified_norm_parameter_values splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
  have hb : (finitePlaceTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 := by
    rw [finite_place_tame_fixed splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point,hv.2.1]
    change (-1 : ControlBase)=4
    decide +kernel
  have hp : (finiteFiberTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 :=
    (congrArg (fun z : ControlBaseˣ=>(z : ControlBase))
      (finite_place_projection_as_fiber_product splitCurve MillerFinitePointAudit.control_two_nonsingular
        (finiteParameterUnit splitCurve 0) (normUnit splitCurve (finiteParameterUnit splitCurve 0))).symm).trans hb
  have hd : (pairedTameProduct splitCurve fixedPoint (finiteParameterUnit splitCurve 0)
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=1 := by
    rw [paired_tame_product_raw,hv.2.2.2]
    rfl
  have hc : (finiteFiber splitCurve fixedPoint).card=1 := by
    rw [finite_fiber_fixed splitCurve fixedPoint f5_fixed_point,Finset.card_singleton]
  refine ⟨MillerPrincipalIdealAudit.control_delta,finite_point_index_fixed splitCurve fixedPoint f5_fixed_point,hc,hb,hp,hd,?_⟩
  intro he
  have hv':=congrArg (fun z : ControlBaseˣ=>(z : ControlBase)) he
  rw [hp,hd] at hv'
  exact (by decide +kernel : (4 : ControlBase)≠1) hv'

/-- Canonical proper base maps agree across actual binary split and fixed models at the same x. -/
theorem actual_binary_mixed_model_canonical :
    binaryCurve.Δ≠0 ∧ ramifiedBinaryCurve.Δ≠0 ∧ binaryPoint≠ -binaryPoint ∧ ramifiedBinaryPoint= -ramifiedBinaryPoint ∧
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
        ∏ R ∈ finiteFiber ramifiedBinaryCurve ramifiedBinaryPoint,milnorSymbol ramifiedBinaryCurve R f (baseFieldUnit ramifiedBinaryCurve c)) := by
  refine ⟨binary_delta,ramified_binary_delta,binary_split_point,ramified_binary_fixed,
    finite_point_index_split binaryCurve binaryPoint binary_split_point,
    finite_point_index_fixed ramifiedBinaryCurve ramifiedBinaryPoint ramified_binary_fixed,
    finite_fiber_split_card binaryCurve binaryPoint binary_split_point,?_,
    finite_place_eval_model_independent binaryCurve ramifiedBinaryCurve binary_nonsingular ramified_binary_nonsingular,
    finite_place_unit_model_independent binaryCurve ramifiedBinaryCurve binary_nonsingular ramified_binary_nonsingular,
    finite_place_tame_model_independent binaryCurve ramifiedBinaryCurve binary_nonsingular ramified_binary_nonsingular,?_,?_⟩
  · rw [finite_fiber_fixed ramifiedBinaryCurve ramifiedBinaryPoint ramified_binary_fixed,Finset.card_singleton]
  · intro f c
    exact ⟨finite_place_projection_as_fiber_product binaryCurve binary_nonsingular f c,
      actual_finite_place_inverse_projection binaryCurve binary_nonsingular f c⟩
  · intro f c
    exact ⟨finite_place_projection_as_fiber_product ramifiedBinaryCurve ramified_binary_nonsingular f c,
      actual_finite_place_inverse_projection ramifiedBinaryCurve ramified_binary_nonsingular f c⟩

theorem actual_ternary_common_positive :
    ramifiedTernaryCurve.Δ≠0 ∧ ramifiedTernaryPoint= -ramifiedTernaryPoint ∧
    finitePointIndex ramifiedTernaryCurve ramifiedTernaryPoint=2 ∧ (finiteFiber ramifiedTernaryCurve ramifiedTernaryPoint).card=1 ∧
    (finitePlaceTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular
      (normUnit ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0))
      (normUnit ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0)) : GeneralF3)=2 ∧
    (∀ f : ramifiedTernaryCurve.FunctionFieldˣ,∀ c : (RatFunc GeneralF3)ˣ,
      finitePlaceTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular (normUnit ramifiedTernaryCurve f) c=
        finiteFiberTameProduct ramifiedTernaryCurve ramifiedTernaryPoint f c ∧
      (finitePlaceTameSymbol ramifiedTernaryCurve ramified_ternary_nonsingular (normUnit ramifiedTernaryCurve f) c)⁻¹=
        ∏ R ∈ finiteFiber ramifiedTernaryCurve ramifiedTernaryPoint,milnorSymbol ramifiedTernaryCurve R f (baseFieldUnit ramifiedTernaryCurve c)) := by
  refine ⟨ramified_ternary_delta,ramified_ternary_fixed,
    finite_point_index_fixed ramifiedTernaryCurve ramifiedTernaryPoint ramified_ternary_fixed,?_,?_,?_⟩
  · rw [finite_fiber_fixed ramifiedTernaryCurve ramifiedTernaryPoint ramified_ternary_fixed,Finset.card_singleton]
  · have hv:=actual_ramified_norm_parameter_values ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed
    rw [finite_place_tame_fixed ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed,hv.2.1]
    change (-1 : GeneralF3)=2
    decide +kernel
  · intro f c
    exact ⟨finite_place_projection_as_fiber_product ramifiedTernaryCurve ramified_ternary_nonsingular f c,
      actual_finite_place_inverse_projection ramifiedTernaryCurve ramified_ternary_nonsingular f c⟩

#print axioms actual_f5_common_split_omission
#print axioms actual_f5_common_fixed_duplication
#print axioms actual_binary_mixed_model_canonical
#print axioms actual_ternary_common_positive
end
end EllipticFinitePlaceProjectionAudit
