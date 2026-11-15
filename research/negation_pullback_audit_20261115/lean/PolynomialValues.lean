module
public import FunctionDivisor

@[expose] public section
namespace FunctionDivisorFinitenessAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit
open scoped WithZero
variable {K : Type*} [Field K]

theorem polynomial_function_nonzero (W : WeierstrassCurve.Affine K) {p : K[X]} (hp : p ≠ 0) :
    polynomialFunction W p ≠ 0 := by
  intro h
  apply hp
  apply polynomial_function_injective W
  simpa only [map_zero] using h

theorem polynomial_function_constant (W : WeierstrassCurve.Affine K) (c : K) :
    polynomialFunction W (C c) = constantHom W c := rfl

/-- Actual polynomial-function infinity valuation is twice its actual polynomial degree. -/
theorem polynomial_function_value (W : WeierstrassCurve.Affine K) (p : K[X]) :
    functionValuation W (polynomialFunction W p) = degreeValue (2 • p.degree) := by
  change functionValuation W (algebraMap W.CoordinateRing W.FunctionField (mk W (C p))) = _
  rw [function_valuation_map]
  change degreeValue (normDegree W (mk W (C p))) = _
  have hn : normDegree W (mk W (C p)) = 2 • p.degree := by
    simpa [smul] using norm_degree_basis W p (0 : K[X])
  rw [hn]

#print axioms polynomial_function_value
end
end FunctionDivisorFinitenessAudit
