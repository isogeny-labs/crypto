module
public import TranslationAction

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open EllipticPicardBridgeAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual point translation, including both exceptional points. -/
def pointTranslation (W : WeierstrassCurve.Affine K) (Q : W.Point) : W.Point ≃ W.Point where
  toFun P := P+Q
  invFun P := P-Q
  left_inv P := add_sub_cancel_right P Q
  right_inv P := sub_add_cancel P Q

/-- Pushforward of actual finite integer point divisors under the actual point permutation. -/
def divisorTranslation (W : WeierstrassCurve.Affine K) (Q : W.Point) :
    (W.Point →₀ ℤ) ≃+ (W.Point →₀ ℤ) := Finsupp.domCongr (pointTranslation W Q)

theorem divisor_translation_apply (W : WeierstrassCurve.Affine K) (Q : W.Point)
    (D : W.Point →₀ ℤ) (P : W.Point) : divisorTranslation W Q D P=D (P-Q) :=
  Finsupp.equivMapDomain_apply (pointTranslation W Q) D P

theorem divisor_translation_single (W : WeierstrassCurve.Affine K) (Q P : W.Point) (n : ℤ) :
    divisorTranslation W Q (Finsupp.single P n)=Finsupp.single (P+Q) n := by
  change Finsupp.equivMapDomain (pointTranslation W Q) (Finsupp.single P n)=_
  rw [Finsupp.equivMapDomain_eq_mapDomain,Finsupp.mapDomain_single]
  rfl

theorem divisor_translation_degree (W : WeierstrassCurve.Affine K) (Q : W.Point)
    (D : W.Point →₀ ℤ) : divisorDegree W (divisorTranslation W Q D)=divisorDegree W D := by
  change (Finsupp.equivMapDomain (pointTranslation W Q) D).sum (fun _ n => n)=D.sum (fun _ n => n)
  rw [Finsupp.equivMapDomain_eq_mapDomain]
  exact Finsupp.sum_mapDomain_index (fun _ => rfl) (fun _ _ _ => rfl)

def degreePointHom (W : WeierstrassCurve.Affine K) (Q : W.Point) :
    (W.Point →₀ ℤ) →+ W.Point where
  toFun D := divisorDegree W D • Q
  map_zero' := by rw [map_zero,zero_zsmul]
  map_add' D E := by rw [map_add,add_zsmul]

/-- Exact point-sum change, including nonzero-degree divisors. -/
theorem divisor_translation_point_sum (W : WeierstrassCurve.Affine K) (Q : W.Point)
    (D : W.Point →₀ ℤ) : divisorPointSum W (divisorTranslation W Q D)=
      divisorPointSum W D+divisorDegree W D • Q := by
  have he : (divisorPointSum W).comp (divisorTranslation W Q).toAddMonoidHom=
      divisorPointSum W+degreePointHom W Q := by
    apply Finsupp.addHom_ext
    intro P n
    change divisorPointSum W (divisorTranslation W Q (Finsupp.single P n))=
      divisorPointSum W (Finsupp.single P n)+degreePointHom W Q (Finsupp.single P n)
    rw [divisor_translation_single,point_sum_single,point_sum_single]
    change n • (P+Q)=n • P+divisorDegree W (Finsupp.single P n) • Q
    rw [degree_single,zsmul_add]
  exact DFunLike.congr_fun he D

variable [IsAlgClosed K]

/-- Actual geometric principal divisors are preserved by actual point translation.
This uses the proved actual principality criterion, not an assumed function pullback law. -/
theorem divisor_translation_principal_iff (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (D : W.Point →₀ ℤ) :
    divisorTranslation W Q D ∈ principalPointDivisors W ↔ D ∈ principalPointDivisors W := by
  constructor
  · intro hm
    obtain ⟨hd,hs⟩ := (principal_iff_degree_and_point_sum W hΔ _).mp hm
    rw [divisor_translation_degree] at hd
    rw [divisor_translation_point_sum,hd,zero_zsmul,add_zero] at hs
    exact (principal_iff_degree_and_point_sum W hΔ D).mpr ⟨hd,hs⟩
  · intro hm
    obtain ⟨hd,hs⟩ := (principal_iff_degree_and_point_sum W hΔ D).mp hm
    apply (principal_iff_degree_and_point_sum W hΔ _).mpr
    rw [divisor_translation_degree,divisor_translation_point_sum,hd,hs,zero_zsmul,add_zero]
    exact ⟨rfl,rfl⟩

/-- Existence only: the actual shifted principal divisor has an actual function.
Identification with fieldTranslation by -Q applied to f is a separate local-order theorem. -/
theorem translated_principal_has_function (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) {f : W.FunctionField} (hf : f ≠ 0) :
    ∃ (g : W.FunctionField) (hg : g ≠ 0), functionDivisor W g hg=
      divisorTranslation W Q (functionDivisor W f hf) := by
  have hm := (divisor_translation_principal_iff W hΔ Q _).mpr
    (function_divisor_mem_principal W f hf)
  obtain ⟨u,hu⟩ := hm
  exact ⟨u.toMul,u.toMul.ne_zero,hu⟩

#print axioms divisor_translation_point_sum
#print axioms divisor_translation_principal_iff
end
end EllipticTranslationAudit
