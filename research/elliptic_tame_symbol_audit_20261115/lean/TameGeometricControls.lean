import TameControls

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit MillerBasechangeAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev GeometricControlBase := AlgebraicClosure ControlBase
noncomputable abbrev geometricControlCommRing : CommRing GeometricControlBase :=
  (inferInstance : Field GeometricControlBase).toCommRing
attribute [local instance] geometricControlCommRing
abbrev geometricTameCurve : WeierstrassCurve.Affine GeometricControlBase := extendedCurve tameCurve

def geometricTwo : GeometricControlBaseˣ := Units.map (algebraMap ControlBase GeometricControlBase).toMonoidHom twoUnit

def geometricMissingRoot : geometricTameCurve.FunctionFieldˣ :=
  Units.map (fieldBaseChange (L := GeometricControlBase) tameCurve).toMonoidHom missingRootUnit

/-- Same actual curve and actual function after scalar extension: the geometric product is1. -/
theorem matched_geometric_product_one :
    geometricTameCurve.Δ≠0 ∧
      globalTameProduct geometricTameCurve (coefficientUnit geometricTameCurve geometricTwo)
        geometricMissingRoot=1 :=
  ⟨extended_delta_nonzero tameCurve MillerPrincipalIdealAudit.control_delta,
    geometric_constant_product geometricTameCurve
      (extended_delta_nonzero tameCurve MillerPrincipalIdealAudit.control_delta) geometricTwo geometricMissingRoot⟩

/-- Concrete characteristic2 actual poles: the sign collapses, both values are1. -/
theorem binary_simple_pole_control :
    GeneralTranslationAudit.binaryCurve.Δ≠0 ∧
      ∃ f g : GeneralTranslationAudit.binaryCurve.FunctionFieldˣ,
        (f : GeneralTranslationAudit.binaryCurve.FunctionField)+g=1 ∧
        pointOrder GeneralTranslationAudit.binaryCurve f 0= -1 ∧
        pointOrder GeneralTranslationAudit.binaryCurve g 0= -1 ∧
        tameSymbol GeneralTranslationAudit.binaryCurve 0 f g=1 ∧
        unsignedSymbol GeneralTranslationAudit.binaryCurve 0 f g=1 := by
  let f := (infinityParameterUnit GeneralTranslationAudit.binaryCurve)^(-1 : ℤ)
  have hf : pointOrder GeneralTranslationAudit.binaryCurve f 0= -1 := by
    change pointOrder GeneralTranslationAudit.binaryCurve
      ((infinityParameterUnit GeneralTranslationAudit.binaryCurve)^(-1 : ℤ) :
        GeneralTranslationAudit.binaryCurve.FunctionFieldˣ) 0= -1
    rw [order_zpow,infinity_parameter_order,mul_one]
  obtain ⟨g,hs,hg⟩ := simple_pole_partner GeneralTranslationAudit.binaryCurve 0 f hf
  refine ⟨GeneralTranslationAudit.binary_delta,f,g,hs,hf,hg,
    tame_steinberg GeneralTranslationAudit.binaryCurve 0 f g hs,?_⟩
  rw [←binary_sign_collapse]
  exact tame_steinberg GeneralTranslationAudit.binaryCurve 0 f g hs

#print axioms matched_geometric_product_one
#print axioms binary_simple_pole_control
end
end EllipticTameSymbolAudit
