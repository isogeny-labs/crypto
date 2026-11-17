module
public import TranslationCoordinate

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit
variable {K : Type*} [Field K]

/-- Actual coefficient functions and actual x,y generate the actual full function field. -/
theorem function_hom_ext (W : WeierstrassCurve.Affine K)
    {φ ψ : W.FunctionField →+* W.FunctionField}
    (hC : ∀ c : K, φ (constantHom W c)=ψ (constantHom W c))
    (hX : φ (xFunction W)=ψ (xFunction W))
    (hY : φ (yFunction W)=ψ (yFunction W)) : φ=ψ := by
  have hp (p : K[X]) : φ (p.eval₂ (constantHom W) (xFunction W))=
      ψ (p.eval₂ (constantHom W) (xFunction W)) := by
    induction p using Polynomial.induction_on' with
    | add p q ihp ihq => rw [eval₂_add,map_add,map_add,ihp,ihq]
    | monomial n c => rw [eval₂_monomial,map_mul,map_mul,map_pow,map_pow,hC,hX]
  have hy : algebraMap W.CoordinateRing W.FunctionField (mk W X)=yFunction W := by
    simp only [yFunction,YClass,map_zero,sub_zero]
  have hc (a : W.CoordinateRing) :
      φ (algebraMap W.CoordinateRing W.FunctionField a)=
      ψ (algebraMap W.CoordinateRing W.FunctionField a) := by
    obtain ⟨p,q,rfl⟩ := exists_smul_basis_eq a
    simp only [smul,map_add,map_mul,mul_one,polynomial_function_image,hy]
    rw [hp p,hp q,hY]
  apply RingHom.ext
  intro f
  obtain ⟨a,b,_hb,hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  rw [← hab,map_div₀,map_div₀,hc a,hc b]

#print axioms function_hom_ext
end
end EllipticTranslationAudit
