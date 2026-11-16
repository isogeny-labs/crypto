module
public import SplitPolynomialDivisors

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit NegationPullbackAudit
variable {K : Type*} [Field K] [IsAlgClosed K]

/-- The actual geometric principal point divisor has degree zero, derived from actual
polynomial factorization, norm fractions and negation, without a supplied degree premise. -/
theorem function_divisor_degree_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f : W.FunctionField} (hf : f ≠ 0) : divisorDegree W (functionDivisor W f hf) = 0 := by
  obtain ⟨a, b, hb, hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  have hb0 : b ≠ 0 := nonZeroDivisors.ne_zero hb
  have ha0 : a ≠ 0 := by
    intro ha
    rw [ha, map_zero, zero_div] at hab
    exact hf hab.symm
  have hna := norm_polynomial_nonzero W ha0
  have hnb := norm_polynomial_nonzero W hb0
  have hfa := polynomial_function_nonzero W hna
  have hfb := polynomial_function_nonzero W hnb
  have he : f * fieldNeg W f =
      polynomialFunction W (Algebra.norm K[X] a) / polynomialFunction W (Algebra.norm K[X] b) := by
    simpa only [hab] using field_fraction_norm W a b
  have hD := function_divisor_congr W (mul_ne_zero hf (field_neg_nonzero W hf))
    (div_ne_zero hfa hfb) he
  have hd := congrArg (divisorDegree W) hD
  rw [function_divisor_mul W hf (field_neg_nonzero W hf), function_divisor_div W hfa hfb,
    map_add, map_sub, function_divisor_pullback W f hf, degree_negation,
    polynomial_divisor_degree_zero W hΔ hna, polynomial_divisor_degree_zero W hΔ hnb] at hd
  omega

#print axioms function_divisor_degree_zero
end
end GeometricPrincipalDivisorAudit
