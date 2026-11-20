import SplitF5

namespace EllipticSplitNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
attribute [local instance] normFunctionCommRing controlBaseCommRing generalF2CommRing generalF3CommRing

/-- Omitting the other actual point in the complete split fiber gives a wrong norm symbol. -/
theorem actual_f5_missing_conjugate_factor :
    splitCurve.Δ≠0 ∧ splitPoint≠ -splitPoint ∧
    (xFiber splitCurve splitPoint).ncard=2 ∧
    (tameSymbol splitCurve splitPoint tangentUnit splitXUnit : ControlBase)=2 ∧
    (tameSymbol splitCurve (-splitPoint) tangentUnit splitXUnit : ControlBase)=3 ∧
    (pointBaseTameSymbol splitCurve splitPoint (normUnit splitCurve tangentUnit)
      (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 ∧
    pointBaseTameSymbol splitCurve splitPoint (normUnit splitCurve tangentUnit)
      (baseVerticalUnit (0 : ControlBase))≠tameSymbol splitCurve splitPoint tangentUnit splitXUnit := by
  have hP:=NegationPullbackAudit.control_point_ne_negative
  have hf:=actual_f5_split_factors
  have hn : (pointBaseTameSymbol splitCurve splitPoint (normUnit splitCurve tangentUnit)
      (baseVerticalUnit (0 : ControlBase)) : ControlBase)=1 := by
    rw [point_base_norm_orbit,Units.val_mul]
    change (tameSymbol splitCurve splitPoint tangentUnit splitXUnit : ControlBase)*
      (tameSymbol splitCurve (-splitPoint) tangentUnit splitXUnit : ControlBase)=1
    rw [hf.1,hf.2]
    decide +kernel
  refine ⟨MillerPrincipalIdealAudit.control_delta,hP,actual_split_fiber_card splitCurve splitPoint hP,
    hf.1,hf.2,hn,?_⟩
  intro he
  have hv:=congrArg (fun z : ControlBaseˣ=>(z : ControlBase)) he
  rw [hn,hf.1] at hv
  exact (by decide +kernel : (1 : ControlBase)≠2) hv

abbrev fixedPoint : splitCurve.Point:=Point.some 4 0 MillerFinitePointAudit.control_two_nonsingular

theorem f5_fixed_point : fixedPoint= -fixedPoint := by decide +kernel

theorem f5_fixed_parameter_value :
    pointBaseValuation splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)=
      WithZero.exp (-2 : ℤ) := by
  have hv : pointBaseValuation splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)≠0 :=
    (Valuation.ne_zero_iff _).mpr (baseVerticalUnit (4 : ControlBase)).ne_zero
  have ho:=fixed_base_parameter_order splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
  change -WithZero.log (pointBaseValuation splitCurve fixedPoint
    (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase))=2 at ho
  have hl : WithZero.log (pointBaseValuation splitCurve fixedPoint
      (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase))= -2 := by omega
  rw [←WithZero.exp_log hv,hl]

/-- At an actual fixed point the restricted base order is2, so the split normalization is false. -/
theorem actual_f5_dropped_split_guard :
    splitCurve.Δ≠0 ∧ fixedPoint= -fixedPoint ∧
    pointBaseOrder splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)=2 ∧
    pointBaseOrder splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)≠1 ∧
    pointBaseValuation splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)=WithZero.exp (-2 : ℤ) ∧
    pointBaseValuation splitCurve fixedPoint (baseVerticalUnit (4 : ControlBase) : RatFunc ControlBase)≠WithZero.exp (-1 : ℤ) := by
  have ho:=fixed_base_parameter_order splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
  refine ⟨MillerPrincipalIdealAudit.control_delta,f5_fixed_point,ho,?_,f5_fixed_parameter_value,?_⟩
  · rw [ho]
    decide
  · rw [f5_fixed_parameter_value]
    intro he
    have hl:=congrArg WithZero.log he
    simp only [WithZero.log_exp] at hl
    norm_num at hl

theorem binary_split_point : binaryPoint≠ -binaryPoint := by decide +kernel

theorem ternary_split_point : ternaryPoint≠ -ternaryPoint := by decide +kernel

/-- Actual split normalization and exact norm projection also hold in characteristics2 and3. -/
theorem actual_small_characteristic_split_projection :
    binaryCurve.Δ≠0 ∧ binaryPoint≠ -binaryPoint ∧
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
          tameSymbol ternaryCurve (-ternaryPoint) f (baseFieldUnit ternaryCurve c)) := by
  refine ⟨binary_delta,binary_split_point,actual_split_fiber_card binaryCurve binaryPoint binary_split_point,
    split_base_parameter_order binaryCurve binary_nonsingular binary_split_point,
    split_base_parameter_value binaryCurve binary_nonsingular binary_split_point,
    point_base_norm_orbit binaryCurve binaryPoint,
    ternary_delta,ternary_split_point,actual_split_fiber_card ternaryCurve ternaryPoint ternary_split_point,
    split_base_parameter_order ternaryCurve ternary_nonsingular ternary_split_point,
    split_base_parameter_value ternaryCurve ternary_nonsingular ternary_split_point,
    point_base_norm_orbit ternaryCurve ternaryPoint⟩

#print axioms actual_f5_missing_conjugate_factor
#print axioms actual_f5_dropped_split_guard
#print axioms actual_small_characteristic_split_projection
end
end EllipticSplitNormProjectionAudit
