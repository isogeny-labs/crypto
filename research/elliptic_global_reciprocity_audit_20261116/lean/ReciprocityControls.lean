import ReciprocityFunctions

namespace EllipticGlobalReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open MillerPrincipalIdealAudit MillerBasechangeAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
attribute [local instance] normFunctionCommRing
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

abbrev geometricSingularCusp : WeierstrassCurve.Affine GeometricControlBase := singularCusp GeometricControlBase
def geometricSingularX : geometricSingularCusp.FunctionFieldˣ := singularCuspX (K:=GeometricControlBase)

/-- The actual F5 counter persists; geometric base extension repairs it. -/
theorem actual_rational_only_guard_counter :
    tameCurve.Δ≠0 ∧
    (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit : ControlBase)=4 ∧
    globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit≠1 ∧
    geometricTameCurve.Δ≠0 ∧
    globalTameProduct geometricTameCurve (coefficientUnit geometricTameCurve geometricTwo) geometricMissingRoot=1 := by
  have hf:=f5_rational_only_product_counter
  have hΔ:=extended_delta_nonzero (L:=GeometricControlBase) tameCurve control_delta
  exact ⟨hf.1,hf.2.1,hf.2.2,hΔ,
    actual_geometric_reciprocity geometricTameCurve hΔ (coefficientUnit geometricTameCurve geometricTwo) geometricMissingRoot⟩

/-- Both actual nonconstant arguments are outside the actual rational base subfield. -/
theorem actual_nonbase_geometric_positive :
    geometricTameCurve.Δ≠0 ∧
    (∀ c : RatFunc GeometricControlBase,(actualYUnit geometricTameCurve : geometricTameCurve.FunctionField)≠
      algebraMap (RatFunc GeometricControlBase) geometricTameCurve.FunctionField c) ∧
    (∀ c : RatFunc GeometricControlBase,(actualYMinusOneUnit geometricTameCurve : geometricTameCurve.FunctionField)≠
      algebraMap (RatFunc GeometricControlBase) geometricTameCurve.FunctionField c) ∧
    globalTameProduct geometricTameCurve (actualYUnit geometricTameCurve) (actualYMinusOneUnit geometricTameCurve)=1 ∧
    (∏ᶠ P,milnorSymbol geometricTameCurve P (actualYUnit geometricTameCurve) (actualYMinusOneUnit geometricTameCurve))=1 := by
  have hΔ:=extended_delta_nonzero (L:=GeometricControlBase) tameCurve control_delta
  exact ⟨hΔ,actual_y_not_base geometricTameCurve,actual_y_minus_one_not_base geometricTameCurve,
    actual_geometric_reciprocity geometricTameCurve hΔ _ _,actual_geometric_inverse_reciprocity geometricTameCurve hΔ _ _⟩

/-- Deleting the nonzero-discriminant guard fails even over an actual algebraic closure. -/
theorem actual_singular_discriminant_guard_counter :
    (singularCusp GeometricControlBase).Δ=0 ∧
    (globalTameProduct (singularCusp GeometricControlBase)
      (coefficientUnit (singularCusp GeometricControlBase) geometricTwo)
      (singularCuspX (K:=GeometricControlBase)) : GeometricControlBase)=
      algebraMap ControlBase GeometricControlBase (4 : ControlBase) ∧
    globalTameProduct (singularCusp GeometricControlBase)
      (coefficientUnit (singularCusp GeometricControlBase) geometricTwo)
      (singularCuspX (K:=GeometricControlBase))≠1 := by
  have ht : (twoUnit : ControlBase)^(-2 : ℤ)=4 := by decide +kernel
  have hm:=congrArg (algebraMap ControlBase GeometricControlBase) ht
  rw [map_zpow₀] at hm
  have hv : (globalTameProduct (singularCusp GeometricControlBase)
      (coefficientUnit (singularCusp GeometricControlBase) geometricTwo)
      (singularCuspX (K:=GeometricControlBase)) : GeometricControlBase)=
      algebraMap ControlBase GeometricControlBase (4 : ControlBase) := by
    rw [singular_cusp_global_constant,Units.val_zpow_eq_zpow_val]
    exact hm
  refine ⟨singular_cusp_delta,hv,?_⟩
  intro he
  have hc:=congrArg (fun u : GeometricControlBaseˣ=>(u : GeometricControlBase)) he
  rw [hv] at hc
  have hb : (4 : ControlBase)=1 := (algebraMap ControlBase GeometricControlBase).injective
    (by simpa only [map_one,Units.val_one] using hc)
  exact (by decide +kernel : (4 : ControlBase)≠1) hb

abbrev binaryGeometricCurve : WeierstrassCurve.Affine (AlgebraicClosure GeneralF2) := extendedCurve binaryCurve
abbrev ternaryGeometricCurve : WeierstrassCurve.Affine (AlgebraicClosure GeneralF3) := extendedCurve ternaryCurve

theorem actual_binary_ternary_allfunction_positives :
    binaryGeometricCurve.Δ≠0 ∧
    (∀ f g : binaryGeometricCurve.FunctionFieldˣ,globalTameProduct binaryGeometricCurve f g=1 ∧
      (∏ᶠ P,milnorSymbol binaryGeometricCurve P f g)=1) ∧
    ternaryGeometricCurve.Δ≠0 ∧
    (∀ f g : ternaryGeometricCurve.FunctionFieldˣ,globalTameProduct ternaryGeometricCurve f g=1 ∧
      (∏ᶠ P,milnorSymbol ternaryGeometricCurve P f g)=1) := by
  have hb:=extended_delta_nonzero (L:=AlgebraicClosure GeneralF2) binaryCurve binary_delta
  have ht:=extended_delta_nonzero (L:=AlgebraicClosure GeneralF3) ternaryCurve ternary_delta
  refine ⟨hb,?_,ht,?_⟩
  · intro f g
    exact ⟨actual_geometric_reciprocity binaryGeometricCurve hb f g,
      actual_geometric_inverse_reciprocity binaryGeometricCurve hb f g⟩
  · intro f g
    exact ⟨actual_geometric_reciprocity ternaryGeometricCurve ht f g,
      actual_geometric_inverse_reciprocity ternaryGeometricCurve ht f g⟩

#print axioms actual_rational_only_guard_counter
#print axioms actual_nonbase_geometric_positive
#print axioms actual_singular_discriminant_guard_counter
#print axioms actual_binary_ternary_allfunction_positives
end
end EllipticGlobalReciprocityAudit
