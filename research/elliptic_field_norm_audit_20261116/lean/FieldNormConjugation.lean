import FieldNormGalois

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit FunctionDivisorFinitenessAudit NegationPullbackAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- The actual involution fixes the actual K(x) base, on every rational function. -/
theorem conjugation_fixes_base (W : WeierstrassCurve.Affine K) (c : RatFunc K) :
    fieldNeg W (algebraMap (RatFunc K) W.FunctionField c)=algebraMap (RatFunc K) W.FunctionField c := by
  obtain ⟨p,q,_hq,hpq⟩ := IsFractionRing.div_surjective K[X] c
  rw [←hpq,map_div₀,rational_base_polynomial,rational_base_polynomial,map_div₀,
    field_neg_polynomial,field_neg_polynomial]

/-- Actual finite-base algebra automorphism; no conjugation action is supplied as a hypothesis. -/
def functionConjugation (W : WeierstrassCurve.Affine K) :
    W.FunctionField ≃ₐ[RatFunc K] W.FunctionField where
  __ := fieldNeg W
  commutes' c := conjugation_fixes_base W c

theorem norm_fixed_by_conjugation (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    fieldNeg W (algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) f))=
      algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) f) :=
  conjugation_fixes_base W _

theorem norm_conjugation_invariant (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    Algebra.norm (RatFunc K) (fieldNeg W f)=Algebra.norm (RatFunc K) f := by
  apply (algebraMap (RatFunc K) W.FunctionField).injective
  rw [actual_field_norm_product,actual_field_norm_product,field_neg_square,mul_comm]

/-- Actual derivative at y is exactly its difference from the other actual ordinate. -/
theorem derivative_value_difference (W : WeierstrassCurve.Affine K) :
    aeval (yFunction W) (functionQuadratic W).derivative=yFunction W-fieldNeg W (yFunction W) := by
  rw [quadratic_derivative]
  simp only [map_add,map_mul,aeval_C,aeval_X,rational_base_polynomial,map_ofNat,field_neg_y]
  ring

theorem nonsingular_conjugation_nontrivial (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    functionConjugation W≠AlgEquiv.refl := by
  have hp : (functionQuadratic W).Separable := by
    rw [←actual_y_minpoly]
    exact (field_separable_iff_generator W).mp (nonsingular_field_separable W hΔ)
  have hd := hp.aeval_derivative_ne_zero (function_quadratic_root W)
  intro he
  apply hd
  rw [derivative_value_difference]
  have hy := congrArg (fun σ : W.FunctionField ≃ₐ[RatFunc K] W.FunctionField => σ (yFunction W)) he
  change fieldNeg W (yFunction W)=yFunction W at hy
  rw [hy,sub_self]

#print axioms functionConjugation
#print axioms norm_conjugation_invariant
#print axioms nonsingular_conjugation_nontrivial
end
end EllipticFieldNormAudit
