import DegreeKernelControls

namespace EllipticFrobeniusDegreeAudit
noncomputable section
open IsogenyFactorizationGuardAudit

def ActualFrobeniusDegree : Prop :=
  ∀ (K : Type) [Field K] [Fintype K] (W : WeierstrassCurve.Affine K),
    Module.Finite (frobeniusImage W) W.FunctionField ∧
      Module.finrank (frobeniusImage W) W.FunctionField = Fintype.card K

def ActualDegreeKernelSeparation : Prop :=
  ∀ (K : Type) [Field K] [Fintype K] (W : WeierstrassCurve.Affine K),
    1 < Module.finrank (frobeniusImage W) W.FunctionField ∧
      geometricKernelCard W = 1 ∧
      Function.Bijective (pointFrobenius W (AlgebraicClosure K)) ∧
      IsPurelyInseparable (frobeniusImage W) W.FunctionField ∧
      ¬ Algebra.IsSeparable (frobeniusImage W) W.FunctionField

def ActualIdentityDegreeControl : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K),
    Module.finrank (AlgHom.id K W.FunctionField).fieldRange W.FunctionField = 1

theorem actual_frobenius_degree_certificate :
    ActualFrobeniusDegree ∧ ActualDegreeKernelSeparation ∧
      ActualIdentityDegreeControl ∧ ActualConcreteDegreeControls ∧
      ¬ UnguardedFrobeniusKernelDegreeReading := by
  constructor
  · intro K _ _ W
    exact ⟨actual_frobenius_finite W,actual_elliptic_frobenius_degree W⟩
  · constructor
    · intro K _ _ W
      exact ⟨actual_degree_exceeds_one W,actual_geometric_kernel_card W,
        geometric_frobenius_bijective W,
        frobenius_image_purely_inseparable W,frobenius_image_not_separable W⟩
    · exact ⟨fun K _ W => identity_field_degree W,
        actual_concrete_degree_controls,wrong_kernel_degree_reading_false⟩

#print axioms actual_frobenius_degree_certificate
end
end EllipticFrobeniusDegreeAudit
