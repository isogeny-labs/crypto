module
public import NormalizedUniqueness
public import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit
open MillerPrincipalIdealAudit MillerRecursionAudit FunctionDivisorFinitenessAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

theorem rational_numerator_finite_order {x y : F3} (h : rationalCurve.Nonsingular x y) :
    pointOrder rationalCurve numeratorFunction (Point.some x y h) = 0 := by
  have hv := coordinate_nonvanishing_value rationalCurve h (mk rationalCurve (C numeratorPolynomial))
    (by rw [point_eval_polynomial]; exact numerator_evaluation_nonzero x)
  change -WithZero.log (finiteAdic rationalCurve h
    (algebraMap rationalCurve.CoordinateRing rationalCurve.FunctionField
      (mk rationalCurve (C numeratorPolynomial)))) = 0
  rw [hv]
  simp

theorem rational_numerator_infinity_order : pointOrder rationalCurve numeratorFunction 0 = -4 := by
  change -WithZero.log (chartAdicValuation rationalCurve numeratorFunction) = -4
  rw [chart_adic_eq_norm, numerator_value, WithZero.log_exp]

/-- The rational-only divisor of the actual nonzero polynomial X²+1 consists of its pole alone. -/
theorem rational_numerator_divisor :
    functionDivisor rationalCurve numeratorFunction numerator_function_nonzero =
      Finsupp.single 0 (-4) := by
  classical
  ext P
  cases P with
  | zero =>
    change pointOrder rationalCurve numeratorFunction 0 = (Finsupp.single 0 (-4)) 0
    simpa only [Finsupp.single_eq_same] using rational_numerator_infinity_order
  | some x y h =>
    rw [function_divisor_apply, rational_numerator_finite_order h]
    simp

/-- Deleting geometric-point scope makes the degree-zero statement false on an actual curve. -/
theorem rational_only_degree_counter :
    divisorDegree rationalCurve
      (functionDivisor rationalCurve numeratorFunction numerator_function_nonzero) = -4 := by
  rw [rational_numerator_divisor, degree_single]

theorem rational_function_not_one : rationalFunction ≠ (1 : rationalCurve.FunctionField) := by
  simpa only [map_one] using rational_function_nonconstant (1 : F3)

/-- Prior actual F3 function reconstructs a counter to rational-only normalized uniqueness. -/
theorem rational_only_normalized_uniqueness_counter :
    rationalFunction ≠ (1 : rationalCurve.FunctionField) ∧
      functionDivisor rationalCurve rationalFunction rational_function_nonzero =
        functionDivisor rationalCurve 1 one_ne_zero ∧
      InfinityMonic rationalCurve rationalFunction ∧
      InfinityMonic rationalCurve (1 : rationalCurve.FunctionField) := by
  refine ⟨rational_function_not_one, ?_, rational_function_monic, infinity_monic_one rationalCurve⟩
  rw [rational_point_divisor_zero, function_divisor_one]

/-- An actual elliptic model over an actual algebraically closed characteristic-three field. -/
def geometricControlCurve : WeierstrassCurve.Affine (AlgebraicClosure F3) :=
  rationalCurve.map (algebraMap F3 (AlgebraicClosure F3))

theorem geometric_control_delta : geometricControlCurve.Δ ≠ 0 := by
  rw [geometricControlCurve, WeierstrassCurve.map_Δ]
  intro hz
  apply rational_delta
  apply (algebraMap F3 (AlgebraicClosure F3)).injective
  simpa only [map_zero] using hz

theorem geometric_control_vertical_degree :
    divisorDegree geometricControlCurve
      (functionDivisor geometricControlCurve (verticalFunction geometricControlCurve 0)
        (vertical_nonzero geometricControlCurve 0)) = 0 :=
  vertical_polynomial_degree geometricControlCurve geometric_control_delta 0

#print axioms rational_only_degree_counter
#print axioms rational_only_normalized_uniqueness_counter
#print axioms geometric_control_vertical_degree
end
end GeometricPrincipalDivisorAudit
