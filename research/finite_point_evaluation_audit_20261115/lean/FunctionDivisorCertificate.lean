module
public import RationalControls

@[expose] public section
namespace FunctionDivisorFinitenessAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit MillerNormValuationAudit MillerRecursionAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

/-- Actual general-function divisor finiteness/homomorphism and a rational-point-only kernel control.
Geometric zero-divisor/constant and uniqueness statements require their geometric-point scope. -/
theorem function_divisor_finiteness_certificate :
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (hf : f ≠ 0),
      (Function.support (pointOrder W f)).Finite ∧
      (∀ R : W.Point, functionDivisor W f hf R = pointOrder W f R) ∧
      functionDivisor W f hf ∈ principalPointDivisors W) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (f g : W.FunctionField)
      (hf : f ≠ 0) (hg : g ≠ 0),
      functionDivisor W (f * g) (mul_ne_zero hf hg) =
        functionDivisor W f hf + functionDivisor W g hg ∧
      functionDivisor W (f / g) (div_ne_zero hf hg) =
        functionDivisor W f hf - functionDivisor W g hg) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ),
      functionDivisor W (millerFunction W P n) (miller_function_nonzero W P n) = millerDivisor W P n) ∧
    rationalCurve.Δ ≠ 0 ∧ rationalFunction ≠ 0 ∧
    (∀ c : F3, rationalFunction ≠ constantHom rationalCurve c) ∧
    functionDivisor rationalCurve rationalFunction rational_function_nonzero = 0 ∧
    InfinityMonic rationalCurve rationalFunction := by
  refine ⟨?_, ?_, ?_, rational_delta, rational_function_nonzero,
    rational_function_nonconstant, rational_point_divisor_zero, rational_function_monic⟩
  · intro K _ W f hf
    exact ⟨function_order_support_finite W f hf, function_divisor_apply W f hf,
      function_divisor_mem_principal W f hf⟩
  · intro K _ W f g hf hg
    exact ⟨function_divisor_mul W hf hg, function_divisor_div W hf hg⟩
  · intro K _ _ W P n
    exact function_divisor_miller W P n

#print axioms function_divisor_finiteness_certificate
end
end FunctionDivisorFinitenessAudit
