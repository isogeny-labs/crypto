module
public import TranslationDivisors

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open GeometricPrincipalDivisorAudit EllipticPicardBridgeAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual point-divisor translation restricted to its derived degree-zero domain. -/
def zeroDivisorTranslation (W : WeierstrassCurve.Affine K) (Q : W.Point) :
    ZeroDivisors W →+ ZeroDivisors W where
  toFun D := ⟨divisorTranslation W Q D.val,by
    change divisorDegree W (divisorTranslation W Q D.val)=0
    rw [divisor_translation_degree]
    exact D.property⟩
  map_zero' := Subtype.ext (map_zero (divisorTranslation W Q))
  map_add' D E := Subtype.ext (map_add (divisorTranslation W Q) D.val E.val)

variable [IsAlgClosed K]

theorem zero_divisor_translation_principal (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) {D : ZeroDivisors W} (hm : D ∈ principalZero W hΔ) :
    zeroDivisorTranslation W Q D ∈ principalZero W hΔ := by
  apply (principal_zero_iff_point_sum W hΔ _).mpr
  change divisorPointSum W (divisorTranslation W Q D.val)=0
  rw [divisor_translation_point_sum,D.property,zero_zsmul,add_zero]
  exact (principal_zero_iff_point_sum W hΔ D).mp hm

/-- On actual geometric degree-zero divisors, translation changes no actual divisor class. -/
theorem zero_divisor_translation_class (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (D : ZeroDivisors W) :
    divisorClass W hΔ (zeroDivisorTranslation W Q D)=divisorClass W hΔ D := by
  have hm : zeroDivisorTranslation W Q D-D ∈ principalZero W hΔ := by
    apply (principal_zero_iff_point_sum W hΔ _).mpr
    change divisorPointSum W (divisorTranslation W Q D.val-D.val)=0
    rw [map_sub,divisor_translation_point_sum,D.property,zero_zsmul,add_zero,sub_self]
  have hz := (QuotientAddGroup.eq_zero_iff (zeroDivisorTranslation W Q D-D)).mpr hm
  change divisorClass W hΔ (zeroDivisorTranslation W Q D-D)=0 at hz
  rwa [map_sub,sub_eq_zero] at hz

/-- Induced actual endomorphism of the actual geometric divisor-class quotient. -/
def picardTranslation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    PicardZero W hΔ →+ PicardZero W hΔ :=
  QuotientAddGroup.map (principalZero W hΔ) (principalZero W hΔ) (zeroDivisorTranslation W Q)
    (fun _ hm => zero_divisor_translation_principal W hΔ Q hm)

theorem picard_translation_mk (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (D : ZeroDivisors W) :
    picardTranslation W hΔ Q (divisorClass W hΔ D)=
      divisorClass W hΔ (zeroDivisorTranslation W Q D) := rfl

/-- The induced action is literally the identity on the actual nontrivial Picard quotient. -/
theorem picard_translation_identity (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : picardTranslation W hΔ Q=AddMonoidHom.id (PicardZero W hΔ) := by
  apply AddMonoidHom.ext
  intro C
  obtain ⟨D,rfl⟩ := QuotientAddGroup.mk'_surjective (principalZero W hΔ) C
  change picardTranslation W hΔ Q (divisorClass W hΔ D)=divisorClass W hΔ D
  rw [picard_translation_mk,zero_divisor_translation_class]

#print axioms zero_divisor_translation_class
#print axioms picard_translation_identity
end
end EllipticTranslationAudit
