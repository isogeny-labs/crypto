module
public import GeometricControls

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit FunctionDivisorFinitenessAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

theorem geometric_principal_divisor_certificate :
    (∀ (K : Type) [Field K] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (f : W.FunctionField) (hf : f ≠ 0), divisorDegree W (functionDivisor W f hf) = 0) ∧
    (∀ (K : Type) [Field K] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (f : W.FunctionField) (_ : f ≠ 0), NoPoles W f ↔ ∃ c : K, c ≠ 0 ∧ f = constantHom W c) ∧
    (∀ (K : Type) [Field K] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (f : W.FunctionField) (hf : f ≠ 0), functionDivisor W f hf = 0 ↔
        ∃ c : K, c ≠ 0 ∧ f = constantHom W c) ∧
    (∀ (K : Type) [Field K] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (f g : W.FunctionField) (hf : f ≠ 0) (hg : g ≠ 0),
      functionDivisor W f hf = functionDivisor W g hg ↔ ∃ c : K, c ≠ 0 ∧ f = constantHom W c * g) ∧
    (∀ (K : Type) [Field K] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (f g : W.FunctionField) (hf : f ≠ 0) (hg : g ≠ 0),
      functionDivisor W f hf = functionDivisor W g hg → InfinityMonic W f → InfinityMonic W g → f = g) ∧
    (∀ (K : Type) [Field K] [IsAlgClosed K] [DecidableEq K] (W : WeierstrassCurve.Affine K)
      (_ : W.Δ ≠ 0) (P : W.Point) (i j : ℕ), millerFunction W P (i + j) =
        millerFunction W P i * millerFunction W P j * stepFunction W (i • P) (j • P)) ∧
    divisorDegree rationalCurve
      (functionDivisor rationalCurve numeratorFunction numerator_function_nonzero) = -4 ∧
    geometricControlCurve.Δ ≠ 0 ∧
    divisorDegree geometricControlCurve (functionDivisor geometricControlCurve
      (verticalFunction geometricControlCurve 0) (vertical_nonzero geometricControlCurve 0)) = 0 ∧
    rationalFunction ≠ (1 : rationalCurve.FunctionField) ∧
    functionDivisor rationalCurve rationalFunction rational_function_nonzero =
      functionDivisor rationalCurve 1 one_ne_zero ∧
    InfinityMonic rationalCurve rationalFunction ∧
    InfinityMonic rationalCurve (1 : rationalCurve.FunctionField) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, rational_only_degree_counter, geometric_control_delta,
    geometric_control_vertical_degree, rational_only_normalized_uniqueness_counter⟩
  · intro K _ _ W hΔ f hf
    exact function_divisor_degree_zero W hΔ hf
  · intro K _ _ W hΔ f hf
    exact no_poles_iff_constant W hΔ hf
  · intro K _ _ W hΔ f hf
    exact divisor_zero_iff_constant W hΔ hf
  · intro K _ _ W hΔ f g hf hg
    exact same_divisor_iff_scalar W hΔ hf hg
  · intro K _ _ W hΔ f g hf hg
    exact normalized_divisor_unique W hΔ hf hg
  · intro K _ _ _ W hΔ P i j
    exact chosen_miller_addition W hΔ P i j

#print axioms geometric_principal_divisor_certificate
end
end GeometricPrincipalDivisorAudit
