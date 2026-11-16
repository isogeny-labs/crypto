import DoublingSeparableControls

namespace EllipticDoublingSeparabilityAudit
noncomputable section
open EllipticDoublingDegreeAudit GeneralTranslationAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
attribute [local instance] generalF2CommRing

def ActualOddDoublingSeparability : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0), (2 : K)≠0 →
    Algebra.IsSeparable (doublingImage W hΔ) W.FunctionField

def ActualBinaryDoublingInseparability : Prop :=
  doublingAlgHom binaryCurve binary_delta=binaryFrobeniusSquareNegation ∧
    IsPurelyInseparable (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField ∧
    ¬ Algebra.IsSeparable (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField

def ActualIdentityImageSeparability : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K),
    Algebra.IsSeparable (AlgHom.id K W.FunctionField).fieldRange W.FunctionField

theorem actual_doubling_separability_certificate :
    ActualOddDoublingSeparability ∧ ActualBinaryDoublingInseparability ∧
      ActualIdentityImageSeparability ∧ ActualDoublingSeparabilityControls ∧
      ¬ UnguardedDoublingSeparabilityReading := by
  refine ⟨?_,?_,?_,actual_doubling_separability_controls,wrong_doubling_separability_reading_false⟩
  · intro K _ W hΔ h2
    exact actual_odd_doubling_separable W hΔ h2
  · exact ⟨actual_binary_doubling_map_eq,actual_binary_doubling_purely_inseparable,
      actual_binary_doubling_not_separable⟩
  · intro K _ W
    exact actual_identity_image_separable W

#print axioms actual_doubling_separability_certificate
end
end EllipticDoublingSeparabilityAudit
