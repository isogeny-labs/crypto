import FieldNormSeparable

namespace EllipticFieldNormAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- The official quadratic-extension class is derived from the actual basis, not supplied. -/
scoped instance actualQuadraticExtension (W : WeierstrassCurve.Affine K) :
    Algebra.IsQuadraticExtension (RatFunc K) W.FunctionField where
  finrank_eq_two' := function_field_degree_two W

/-- Actual normality even on the inseparable boundary; normal does not imply separable. -/
theorem actual_quadratic_normal (W : WeierstrassCurve.Affine K) : Normal (RatFunc K) W.FunctionField :=
  inferInstance

theorem actual_galois_iff (W : WeierstrassCurve.Affine K) :
    IsGalois (RatFunc K) W.FunctionField ↔ ¬((2 : K)=0 ∧ W.a₁=0 ∧ W.a₃=0) := by
  constructor
  · intro h
    exact (actual_field_separable_iff W).mp h.to_isSeparable
  · intro h
    let _ : Algebra.IsSeparable (RatFunc K) W.FunctionField := (actual_field_separable_iff W).mpr h
    infer_instance

theorem nonsingular_function_field_galois (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    IsGalois (RatFunc K) W.FunctionField := by
  let _ : Algebra.IsSeparable (RatFunc K) W.FunctionField := nonsingular_field_separable W hΔ
  infer_instance

#print axioms actual_quadratic_normal
#print axioms actual_galois_iff
#print axioms nonsingular_function_field_galois
end
end EllipticFieldNormAudit
