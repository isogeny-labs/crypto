import IsogenyControls

namespace IsogenyFactorizationGuardAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerBasechangeAudit EllipticTranslationAudit MillerNormValuationAudit GeneralTranslationAudit

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
attribute [local instance] generalF2CommRing generalF3CommRing

def ActualFrobeniusFieldObstruction : Prop :=
  ∀ (K : Type) [Field K] [Fintype K] (W : WeierstrassCurve.Affine K), W.Δ≠0 →
    Function.Injective (fieldFrobenius W) ∧
    (¬ Function.Surjective (fieldFrobenius W)) ∧
    (∀ a : W.CoordinateRing,
      algebraMap W.CoordinateRing W.FunctionField (coordinateFrobenius W a) =
        fieldFrobenius W (algebraMap W.CoordinateRing W.FunctionField a)) ∧
    IsPurelyInseparable (frobeniusImage W) W.FunctionField ∧
    (¬ Algebra.IsSeparable (frobeniusImage W) W.FunctionField) ∧
    (¬ RationalFactorization W (fieldFrobenius W).toRingHom (RingHom.id W.FunctionField))

def ActualGeometricKernelGuard : Prop :=
  ∀ (K : Type) [Field K] [Fintype K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
    Function.Bijective (pointFrobenius W (AlgebraicClosure K)) ∧
    (pointFrobenius W (AlgebraicClosure K)).ker ≤
      (AddMonoidHom.id (extendedCurve (L:=AlgebraicClosure K) W).Point).ker ∧
    Algebra.IsSeparable W.FunctionField W.FunctionField ∧
    genericX W (pointFrobenius W W.FunctionField (genericPoint W hΔ)) =
      fieldFrobenius W (xFunction W) ∧
    genericY W (pointFrobenius W W.FunctionField (genericPoint W hΔ)) =
      fieldFrobenius W (yFunction W)

def ActualFactorizationControls : Prop :=
  (∀ (K : Type) [Field K] [Fintype K] (W : WeierstrassCurve.Affine K)
    (ψ : W.FunctionField →+* W.FunctionField),
    RationalFactorization W (RingHom.id W.FunctionField) ψ) ∧
  (∀ (K : Type) [Field K] [Fintype K] (W : WeierstrassCurve.Affine K),
    RationalFactorization W (fieldFrobenius W).toRingHom
      ((fieldFrobenius W).toRingHom.comp (fieldFrobenius W).toRingHom)) ∧
  (¬ WrongFrobeniusFactorizationReading)

def ActualConcreteControls : Prop :=
  binaryCurve.Δ≠0 ∧
  (¬ RationalFactorization binaryCurve (fieldFrobenius binaryCurve).toRingHom
    (RingHom.id binaryCurve.FunctionField)) ∧
  ternaryCurve.Δ≠0 ∧
  (¬ RationalFactorization ternaryCurve (fieldFrobenius ternaryCurve).toRingHom
    (RingHom.id ternaryCurve.FunctionField))

theorem actual_isogeny_factorization_guard_certificate :
    ActualFrobeniusFieldObstruction ∧ ActualGeometricKernelGuard ∧
      ActualFactorizationControls ∧ ActualConcreteControls := by
  refine ⟨?_,?_,?_,?_⟩
  · intro K _ _ W _
    exact ⟨field_frobenius_injective W,field_frobenius_not_surjective W,
      coordinate_field_frobenius W,frobenius_image_purely_inseparable W,
      frobenius_image_not_separable W,actual_factorization_obstruction W⟩
  · intro K _ _ W hΔ
    exact ⟨geometric_frobenius_bijective W,geometric_kernel_inclusion W,
      identity_separable W,generic_frobenius_x W hΔ,generic_frobenius_y W hΔ⟩
  · exact ⟨fun _ _ _ W => identity_left_control W,
      fun _ _ _ W => frobenius_square_control W,wrong_frobenius_reading_false⟩
  · exact ⟨actual_binary_obstruction.1,actual_binary_obstruction.2,
      actual_ternary_obstruction.1,actual_ternary_obstruction.2⟩

#print axioms actual_isogeny_factorization_guard_certificate
end
end IsogenyFactorizationGuardAudit
