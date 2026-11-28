import RamifiedProjection

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
private instance : Fact (Nat.Prime 2):=⟨by decide⟩
private instance : Fact (Nat.Prime 3):=⟨by decide⟩
private instance : Fact (Nat.Prime 5):=⟨by decide⟩
attribute [local instance] normFunctionCommRing controlBaseCommRing generalF2CommRing generalF3CommRing

/-- Actual F5 functions distinguish normalized projection from its raw restricted square. -/
theorem actual_f5_raw_vs_normalized_counter :
    splitCurve.Δ≠0 ∧ fixedPoint= -fixedPoint ∧
    pointOrder splitCurve (finiteParameterUnit splitCurve 0) fixedPoint=1 ∧
    finiteBaseOrder (4 : ControlBase) (normUnit splitCurve (finiteParameterUnit splitCurve 0) : RatFunc ControlBase)=1 ∧
    (finiteBaseTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 ∧
    (pointBaseTameSymbol splitCurve fixedPoint (normUnit splitCurve (finiteParameterUnit splitCurve 0))
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=1 ∧
    finiteBaseTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0))≠
    pointBaseTameSymbol splitCurve fixedPoint (normUnit splitCurve (finiteParameterUnit splitCurve 0))
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) := by
  have hv:=actual_ramified_norm_parameter_values splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
  have hb : (finiteBaseTameSymbol splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=4 := by
    rw [hv.2.1]
    change (-1 : ControlBase)=4
    decide +kernel
  have hr : (pointBaseTameSymbol splitCurve fixedPoint (normUnit splitCurve (finiteParameterUnit splitCurve 0))
      (normUnit splitCurve (finiteParameterUnit splitCurve 0)) : ControlBase)=1 :=
    congrArg (fun z : ControlBaseˣ=>(z : ControlBase)) hv.2.2.2
  refine ⟨MillerPrincipalIdealAudit.control_delta,f5_fixed_point,
    fixed_parameter_order splitCurve MillerFinitePointAudit.control_two_nonsingular f5_fixed_point,hv.1,hb,hr,?_⟩
  intro he
  have hc:=congrArg (fun z : ControlBaseˣ=>(z : ControlBase)) he
  rw [hb,hr] at hc
  exact (by decide +kernel : (4 : ControlBase)≠1) hc

/-- Actual wild binary fixed-point model; not the previous split binary curve. -/
def ramifiedBinaryCurve : WeierstrassCurve.Affine GeneralF2:=⟨1,0,0,0,1⟩
theorem ramified_binary_delta : ramifiedBinaryCurve.Δ≠0 := by decide +kernel
theorem ramified_binary_nonsingular : ramifiedBinaryCurve.Nonsingular 0 1 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
  decide +kernel
abbrev ramifiedBinaryPoint : ramifiedBinaryCurve.Point:=Point.some 0 1 ramified_binary_nonsingular
theorem ramified_binary_fixed : ramifiedBinaryPoint= -ramifiedBinaryPoint := by decide +kernel

/-- Actual ternary fixed-point model. -/
def ramifiedTernaryCurve : WeierstrassCurve.Affine GeneralF3:=⟨0,0,0,1,0⟩
theorem ramified_ternary_delta : ramifiedTernaryCurve.Δ≠0 := by decide +kernel
theorem ramified_ternary_nonsingular : ramifiedTernaryCurve.Nonsingular 0 0 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
  decide +kernel
abbrev ramifiedTernaryPoint : ramifiedTernaryCurve.Point:=Point.some 0 0 ramified_ternary_nonsingular
theorem ramified_ternary_fixed : ramifiedTernaryPoint= -ramifiedTernaryPoint := by decide +kernel

theorem actual_small_characteristic_ramified_projection :
    ramifiedBinaryCurve.Δ≠0 ∧ ramifiedBinaryPoint= -ramifiedBinaryPoint ∧
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
      (normUnit ramifiedTernaryCurve (finiteParameterUnit ramifiedTernaryCurve 0)) : GeneralF3)=2 := by
  refine ⟨ramified_binary_delta,ramified_binary_fixed,
    fixed_parameter_order ramifiedBinaryCurve ramified_binary_nonsingular ramified_binary_fixed,
    (fixed_full_point_fiber ramifiedBinaryCurve ramified_binary_nonsingular ramified_binary_fixed).2,
    fixed_base_valuation ramifiedBinaryCurve ramified_binary_nonsingular ramified_binary_fixed,?_,?_,
    ramified_ternary_delta,ramified_ternary_fixed,
    fixed_parameter_order ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed,
    (fixed_full_point_fiber ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed).2,
    fixed_base_valuation ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed,?_,?_⟩
  · intro f c
    exact ⟨actual_ramified_norm_projection _ _ _ f c,actual_ramified_inverse_projection _ _ _ f c⟩
  · have hv:=actual_ramified_norm_parameter_values ramifiedBinaryCurve ramified_binary_nonsingular ramified_binary_fixed
    rw [hv.2.1]
    change (-1 : GeneralF2)=1
    decide +kernel
  · intro f c
    exact ⟨actual_ramified_norm_projection _ _ _ f c,actual_ramified_inverse_projection _ _ _ f c⟩
  · have hv:=actual_ramified_norm_parameter_values ramifiedTernaryCurve ramified_ternary_nonsingular ramified_ternary_fixed
    rw [hv.2.1]
    change (-1 : GeneralF3)=2
    decide +kernel

#print axioms actual_f5_raw_vs_normalized_counter
#print axioms actual_small_characteristic_ramified_projection
end
end EllipticRamifiedNormProjectionAudit
