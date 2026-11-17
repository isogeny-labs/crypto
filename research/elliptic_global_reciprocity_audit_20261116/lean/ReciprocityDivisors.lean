import ReciprocityQuadratic

namespace EllipticGlobalReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine Function
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
open EllipticTameSymbolAudit
variable {K : Type*} [Field K] [IsAlgClosed K]

theorem actual_geometric_inverse_reciprocity (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (f g : W.FunctionFieldˣ) : (∏ᶠ P,milnorSymbol W P f g)=1 := by
  simp only [milnorSymbol,finprod_inv_distrib]
  change (globalTameProduct W f g)⁻¹=1
  rw [actual_geometric_reciprocity W hΔ f g,inv_one]

/-- Actual proper divisor evaluation, retaining both disjoint-support guards. -/
theorem actual_geometric_disjoint_evaluation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (f g : W.FunctionFieldˣ)
    (hd : Disjoint (functionDivisor W f f.ne_zero).support (functionDivisor W g g.ne_zero).support) :
    evaluateDivisor W (functionDivisor W g g.ne_zero) f hd.symm=
      evaluateDivisor W (functionDivisor W f f.ne_zero) g hd := by
  have h:=actual_geometric_reciprocity W hΔ f g
  rw [global_disjoint_evaluation W f g hd] at h
  exact div_eq_one.mp h

#print axioms actual_geometric_inverse_reciprocity
#print axioms actual_geometric_disjoint_evaluation
end
end EllipticGlobalReciprocityAudit
