module
public import DivisorDegree

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit
open MillerPrincipalIdealAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

theorem constant_function_nonzero (W : WeierstrassCurve.Affine K) {c : K} (hc : c ≠ 0) :
    constantHom W c ≠ 0 := by
  simpa only [polynomial_function_constant] using
    polynomial_function_nonzero W (C_ne_zero.mpr hc)

theorem constant_point_order (W : WeierstrassCurve.Affine K) {c : K} (hc : c ≠ 0)
    (P : W.Point) : pointOrder W (constantHom W c) P = 0 := by
  cases P with
  | zero =>
    change -WithZero.log (chartAdicValuation W (constantHom W c)) = 0
    rw [chart_adic_eq_norm, function_valuation_constant W hc]
    simp
  | some x y h =>
    have hv := coordinate_nonvanishing_value W h (mk W (C (C c))) (by
      rw [point_eval_polynomial, eval_C]
      exact hc)
    change -WithZero.log (finiteAdic W h
      (algebraMap W.CoordinateRing W.FunctionField (mk W (C (C c))))) = 0
    rw [hv]
    simp

theorem constant_function_divisor (W : WeierstrassCurve.Affine K) {c : K} (hc : c ≠ 0) :
    functionDivisor W (constantHom W c) (constant_function_nonzero W hc) = 0 := by
  ext P
  exact constant_point_order W hc P

theorem constant_polynomial_degree_zero (W : WeierstrassCurve.Affine K) {c : K} (hc : c ≠ 0) :
    divisorDegree W (functionDivisor W (polynomialFunction W (C c))
      (polynomial_function_nonzero W (C_ne_zero.mpr hc))) = 0 := by
  change divisorDegree W (functionDivisor W (constantHom W c) (constant_function_nonzero W hc)) = 0
  rw [constant_function_divisor W hc, map_zero]

#print axioms constant_function_divisor
end
end GeometricPrincipalDivisorAudit
