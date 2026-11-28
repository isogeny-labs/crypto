import ReciprocityControls

namespace EllipticGlobalReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open P1TameReciprocityAudit
open scoped EllipticFieldNormAudit WithZero
attribute [local instance] normFunctionCommRing
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

/-- Actual full geometric elliptic reciprocity and explicit guard-deletion controls. -/
theorem actual_elliptic_global_reciprocity_certificate
    (K : Type*) [Field K] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Function.Surjective (pointBasePlace W) ∧
    (∀ f : W.FunctionFieldˣ,∀ c : (RatFunc K)ˣ,
      globalTameProduct W f (baseFieldUnit W c)=p1GlobalProduct (normUnit W f) c ∧
      globalTameProduct W f (baseFieldUnit W c)=1) ∧
    (∀ f : W.FunctionField,∃ p q : RatFunc K,f=algebraMap (RatFunc K) W.FunctionField p+
      algebraMap (RatFunc K) W.FunctionField q*yFunction W) ∧
    (∀ f g : W.FunctionField,(∀ c : RatFunc K,f≠algebraMap (RatFunc K) W.FunctionField c)→
      ∃ a b : RatFunc K,g=algebraMap (RatFunc K) W.FunctionField a*f+
        algebraMap (RatFunc K) W.FunctionField b) ∧
    (∀ f g : W.FunctionFieldˣ,Function.HasFiniteMulSupport (fun P=>tameSymbol W P f g) ∧
      globalTameProduct W f g=1 ∧ (∏ᶠ P,milnorSymbol W P f g)=1) ∧
    (∀ f g : W.FunctionFieldˣ,
      ∀ hd : Disjoint (functionDivisor W f f.ne_zero).support (functionDivisor W g g.ne_zero).support,
      evaluateDivisor W (functionDivisor W g g.ne_zero) f hd.symm=
        evaluateDivisor W (functionDivisor W f f.ne_zero) g hd) ∧
    (tameCurve.Δ≠0 ∧
      (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit : ControlBase)=4 ∧
      globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit≠1 ∧
      geometricTameCurve.Δ≠0 ∧
      globalTameProduct geometricTameCurve (coefficientUnit geometricTameCurve geometricTwo) geometricMissingRoot=1) ∧
    (geometricTameCurve.Δ≠0 ∧
      (∀ c : RatFunc GeometricControlBase,(actualYUnit geometricTameCurve : geometricTameCurve.FunctionField)≠
        algebraMap (RatFunc GeometricControlBase) geometricTameCurve.FunctionField c) ∧
      (∀ c : RatFunc GeometricControlBase,(actualYMinusOneUnit geometricTameCurve : geometricTameCurve.FunctionField)≠
        algebraMap (RatFunc GeometricControlBase) geometricTameCurve.FunctionField c) ∧
      globalTameProduct geometricTameCurve (actualYUnit geometricTameCurve) (actualYMinusOneUnit geometricTameCurve)=1 ∧
      (∏ᶠ P,milnorSymbol geometricTameCurve P (actualYUnit geometricTameCurve) (actualYMinusOneUnit geometricTameCurve))=1) ∧
    (geometricSingularCusp.Δ=0 ∧
      (globalTameProduct geometricSingularCusp
        (coefficientUnit geometricSingularCusp geometricTwo)
        geometricSingularX : GeometricControlBase)=
        algebraMap ControlBase GeometricControlBase (4 : ControlBase) ∧
      globalTameProduct geometricSingularCusp
        (coefficientUnit geometricSingularCusp geometricTwo) geometricSingularX≠1) ∧
    (binaryGeometricCurve.Δ≠0 ∧
      (∀ f g : binaryGeometricCurve.FunctionFieldˣ,globalTameProduct binaryGeometricCurve f g=1 ∧
        (∏ᶠ P,milnorSymbol binaryGeometricCurve P f g)=1) ∧
      ternaryGeometricCurve.Δ≠0 ∧
      (∀ f g : ternaryGeometricCurve.FunctionFieldˣ,globalTameProduct ternaryGeometricCurve f g=1 ∧
        (∏ᶠ P,milnorSymbol ternaryGeometricCurve P f g)=1)) := by
  refine ⟨actual_base_place_surjective W hΔ,?_,actual_function_linear_expression W,
    actual_nonbase_affine_relation W,?_,actual_geometric_disjoint_evaluation W hΔ,
    actual_rational_only_guard_counter,actual_nonbase_geometric_positive,
    actual_singular_discriminant_guard_counter,actual_binary_ternary_allfunction_positives⟩
  · intro f c
    exact ⟨actual_global_mixed_norm_identity W hΔ f c,actual_mixed_reciprocity W hΔ f c⟩
  · intro f g
    exact ⟨tame_support_finite W f g,actual_geometric_reciprocity W hΔ f g,
      actual_geometric_inverse_reciprocity W hΔ f g⟩

#print axioms actual_elliptic_global_reciprocity_certificate
end
end EllipticGlobalReciprocityAudit
