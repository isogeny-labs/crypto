import FactorizationBoundary

namespace IsogenyFactorizationGuardAudit
noncomputable section
open WeierstrassCurve.Affine
open GeneralTranslationAudit
variable {K : Type*} [Field K] [Fintype K]

omit [Fintype K] in
theorem identity_left_control (W : WeierstrassCurve.Affine K)
    (ψ : W.FunctionField →+* W.FunctionField) :
    RationalFactorization W (RingHom.id W.FunctionField) ψ := ⟨ψ,rfl⟩

theorem frobenius_square_control (W : WeierstrassCurve.Affine K) :
    RationalFactorization W (fieldFrobenius W).toRingHom
      ((fieldFrobenius W).toRingHom.comp (fieldFrobenius W).toRingHom) :=
  ⟨(fieldFrobenius W).toRingHom,rfl⟩

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
attribute [local instance] generalF2CommRing generalF3CommRing

theorem actual_binary_obstruction :
    binaryCurve.Δ≠0 ∧ ¬ RationalFactorization binaryCurve
      (fieldFrobenius binaryCurve).toRingHom (RingHom.id binaryCurve.FunctionField) :=
  ⟨binary_delta,actual_factorization_obstruction binaryCurve⟩

theorem actual_ternary_obstruction :
    ternaryCurve.Δ≠0 ∧ ¬ RationalFactorization ternaryCurve
      (fieldFrobenius ternaryCurve).toRingHom (RingHom.id ternaryCurve.FunctionField) :=
  ⟨ternary_delta,actual_factorization_obstruction ternaryCurve⟩

theorem wrong_frobenius_reading_false : ¬ WrongFrobeniusFactorizationReading := by
  intro h
  exact actual_factorization_obstruction binaryCurve
    (h GeneralF2 binaryCurve binary_delta (geometric_kernel_inclusion binaryCurve)
      (identity_separable binaryCurve))

#print axioms actual_binary_obstruction
#print axioms actual_ternary_obstruction
#print axioms wrong_frobenius_reading_false
end
end IsogenyFactorizationGuardAudit
