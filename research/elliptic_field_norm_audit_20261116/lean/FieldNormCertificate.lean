import FieldNormControls
import FieldNormConjugation

namespace EllipticFieldNormAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit NegationPullbackAudit GeneralTranslationAudit
open scoped EllipticFieldNormAudit
attribute [local instance] normFunctionCommRing

/-- Actual rational-base field structure and norm, with exact separability/Galois boundary
and constructed deleted-base/deleted-singularity controls. No norm/basis/Galois premise. -/
theorem actual_elliptic_field_norm_certificate
    (K : Type*) [Field K] (W : WeierstrassCurve.Affine K) :
    (Module.finrank (RatFunc K) W.FunctionField=2 ∧
      functionBasis W 0=1 ∧ functionBasis W 1=yFunction W) ∧
    (∀ f : W.FunctionField,algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) f)=
      f*fieldNeg W f) ∧
    (minpoly (RatFunc K) (yFunction W)=functionQuadratic W ∧ Irreducible (functionQuadratic W)) ∧
    Normal (RatFunc K) W.FunctionField ∧
    (Algebra.IsSeparable (RatFunc K) W.FunctionField ↔ ¬((2 : K)=0 ∧ W.a₁=0 ∧ W.a₃=0)) ∧
    (IsGalois (RatFunc K) W.FunctionField ↔ ¬((2 : K)=0 ∧ W.a₁=0 ∧ W.a₃=0)) ∧
    (W.Δ≠0 → IsGalois (RatFunc K) W.FunctionField ∧ functionConjugation W≠AlgEquiv.refl) ∧
    (∀ f : W.FunctionField,Algebra.norm (RatFunc K) (fieldNeg W f)=Algebra.norm (RatFunc K) f) ∧
    (∀ c : RatFunc K,fieldNeg W (algebraMap (RatFunc K) W.FunctionField c)=
      algebraMap (RatFunc K) W.FunctionField c) ∧
    (¬Module.Finite K W.FunctionField ∧ ∀ f : W.FunctionField,Algebra.norm K f=1) ∧
    (algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) (xFunction W))≠
      constantHom W (Algebra.norm K (xFunction W))) ∧
    (GeneralTranslationAudit.binaryCurve.Δ≠0 ∧
      Module.finrank (RatFunc (ZMod 2)) GeneralTranslationAudit.binaryCurve.FunctionField=2 ∧
      IsGalois (RatFunc (ZMod 2)) GeneralTranslationAudit.binaryCurve.FunctionField) ∧
    (GeneralTranslationAudit.ternaryCurve.Δ≠0 ∧
      Module.finrank (RatFunc (ZMod 3)) GeneralTranslationAudit.ternaryCurve.FunctionField=2 ∧
      IsGalois (RatFunc (ZMod 3)) GeneralTranslationAudit.ternaryCurve.FunctionField) ∧
    (cuspCurve.Δ=0 ∧ Module.finrank (RatFunc (ZMod 2)) cuspCurve.FunctionField=2 ∧
      Normal (RatFunc (ZMod 2)) cuspCurve.FunctionField ∧
      ¬Algebra.IsSeparable (RatFunc (ZMod 2)) cuspCurve.FunctionField ∧
      ¬IsGalois (RatFunc (ZMod 2)) cuspCurve.FunctionField ∧
      (fieldNeg cuspCurve).toRingHom=RingHom.id cuspCurve.FunctionField ∧
      ∀ f : cuspCurve.FunctionField,
        algebraMap (RatFunc (ZMod 2)) cuspCurve.FunctionField (Algebra.norm (RatFunc (ZMod 2)) f)=f^2) := by
  refine ⟨⟨function_field_degree_two W,function_basis_zero W,function_basis_one W⟩,
    actual_field_norm_product W,⟨actual_y_minpoly W,function_quadratic_irreducible W⟩,
    actual_quadratic_normal W,actual_field_separable_iff W,actual_galois_iff W,?_,
    norm_conjugation_invariant W,conjugation_fixes_base W,
    ⟨coefficient_not_finite W,coefficient_norm_default W⟩,(wrong_base_norm_counter W).2,
    actual_binary_norm_positive,actual_ternary_norm_positive,actual_singular_binary_counter⟩
  intro hΔ
  exact ⟨nonsingular_function_field_galois W hΔ,nonsingular_conjugation_nontrivial W hΔ⟩

#print axioms actual_elliptic_field_norm_certificate
end
end EllipticFieldNormAudit
