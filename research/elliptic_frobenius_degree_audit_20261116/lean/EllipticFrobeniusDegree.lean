import FactorizationCertificate
import RationalFrobeniusDegree
import FieldDegreeTransfer

namespace EllipticFrobeniusDegreeAudit
noncomputable section
open EllipticFieldNormAudit IsogenyFactorizationGuardAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K] [Fintype K]

def ellipticRationalEmbedding (W : WeierstrassCurve.Affine K) :
    RatFunc K →ₐ[K] W.FunctionField := IsScalarTower.toAlgHom K (RatFunc K) W.FunctionField

def ellipticRationalImage (W : WeierstrassCurve.Affine K) :
    IntermediateField K W.FunctionField := (ellipticRationalEmbedding W).fieldRange

omit [Fintype K] in
theorem actual_rational_image_degree_two (W : WeierstrassCurve.Affine K) :
    Module.finrank (ellipticRationalImage W) W.FunctionField = 2 := by
  have h := Algebra.finrank_eq_of_equiv_equiv
    (AlgEquiv.ofInjective (ellipticRationalEmbedding W)
      (ellipticRationalEmbedding W).injective).toRingEquiv
    (RingEquiv.refl W.FunctionField) (by ext x; rfl)
  exact h.symm.trans (function_field_degree_two W)

theorem actual_frobenius_embedding_commutes (W : WeierstrassCurve.Affine K) :
    (fieldFrobenius W).comp (ellipticRationalEmbedding W) =
      (ellipticRationalEmbedding W).comp rationalFrobenius := by
  ext x
  exact (map_pow (ellipticRationalEmbedding W) x (Fintype.card K)).symm

theorem actual_rational_image_stable (W : WeierstrassCurve.Affine K) :
    (ellipticRationalImage W).map (fieldFrobenius W) ≤ ellipticRationalImage W := by
  intro x hx
  obtain ⟨y,hy,rfl⟩ := hx
  change y^Fintype.card K ∈ ellipticRationalImage W
  exact pow_mem hy _

theorem actual_rational_image_frobenius_index (W : WeierstrassCurve.Affine K) :
    ((ellipticRationalImage W).map (fieldFrobenius W)).relfinrank
      (ellipticRationalImage W) = Fintype.card K := by
  have hr : (ellipticRationalImage W).map (fieldFrobenius W) =
      (rationalFrobenius (K:=K)).fieldRange.map (ellipticRationalEmbedding W) := by
    rw [ellipticRationalImage,AlgHom.map_fieldRange,AlgHom.map_fieldRange,
      actual_frobenius_embedding_commutes]
  rw [hr,ellipticRationalImage,AlgHom.fieldRange_eq_map (ellipticRationalEmbedding W),
    IntermediateField.relfinrank_map_map,IntermediateField.relfinrank_top_right]
  exact rational_frobenius_degree

theorem actual_elliptic_frobenius_degree (W : WeierstrassCurve.Affine K) :
    Module.finrank (frobeniusImage W) W.FunctionField = Fintype.card K := by
  exact (field_image_degree_transfer (fieldFrobenius W) (ellipticRationalImage W)
    (actual_rational_image_stable W) 2 (by decide) (actual_rational_image_degree_two W)).trans
      (actual_rational_image_frobenius_index W)

theorem actual_frobenius_finite (W : WeierstrassCurve.Affine K) :
    Module.Finite (frobeniusImage W) W.FunctionField := by
  apply Module.finite_of_finrank_pos
  rw [actual_elliptic_frobenius_degree]
  exact Fintype.card_pos

#print axioms actual_frobenius_finite
#print axioms actual_rational_image_degree_two
#print axioms actual_elliptic_frobenius_degree
end
end EllipticFrobeniusDegreeAudit
