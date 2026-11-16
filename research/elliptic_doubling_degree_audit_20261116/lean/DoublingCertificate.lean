import DoublingControls

namespace EllipticDoublingDegreeAudit
noncomputable section
open EllipticTranslationAudit

def ActualDoublingCoordinates : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
    IsCoprime (doublingNumerator W) (doublingDenominator W) ∧
      (∀ a : W.CoordinateRing, doublingFieldMap W hΔ
        (algebraMap W.CoordinateRing W.FunctionField a)=doublingCoordinate W hΔ a) ∧
      (∀ c : K, doublingFieldMap W hΔ (MillerNormValuationAudit.constantHom W c)=
        MillerNormValuationAudit.constantHom W c) ∧
      doublingAlgHom W hΔ (MillerNormValuationAudit.xFunction W)=doubledX W hΔ ∧
      doublingAlgHom W hΔ (MillerNormValuationAudit.yFunction W)=doubledY W hΔ

def ActualDoublingFieldDegree : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
    Module.Finite (doublingImage W hΔ) W.FunctionField ∧
      Module.finrank (doublingImage W hΔ) W.FunctionField=4

def ActualDoublingGenericNaturality : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
    doubledGeneric W hΔ≠0 ∧
      coefficientPointMap W (doublingAlgHom W hΔ) (EllipticTranslationAudit.genericPoint W hΔ)=doubledGeneric W hΔ

theorem actual_doubling_degree_certificate :
    ActualDoublingCoordinates ∧ ActualDoublingFieldDegree ∧ ActualDoublingGenericNaturality ∧
      EllipticFrobeniusDegreeAudit.ActualIdentityDegreeControl ∧ ActualDoublingControls := by
  refine ⟨?_,?_,?_,?_,actual_doubling_controls⟩
  · intro K _ W hΔ
    exact ⟨doubling_coprime W hΔ,doubling_field_coordinate W hΔ,doubling_field_constant W hΔ,
      doubling_field_x W hΔ,doubling_field_y W hΔ⟩
  · intro K _ W hΔ
    exact ⟨actual_doubling_finite W hΔ,actual_doubling_degree W hΔ⟩
  · intro K _ W hΔ
    exact ⟨doubled_generic_nonzero W hΔ,doubling_generic_naturality W hΔ⟩
  · intro K _ W
    exact EllipticFrobeniusDegreeAudit.identity_field_degree W

#print axioms actual_doubling_degree_certificate
end
end EllipticDoublingDegreeAudit
