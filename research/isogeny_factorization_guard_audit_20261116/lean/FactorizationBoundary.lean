import FrobeniusPoints

namespace IsogenyFactorizationGuardAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerBasechangeAudit
variable {K : Type*} [Field K] [Fintype K]

/-- The explicit rational-coordinate interpretation of the requested factorization.
The conclusion would hold for every curve morphism factorization; here even arbitrary
field ring maps are excluded. No Scheme API comparison is asserted. -/
def RationalFactorization (W : WeierstrassCurve.Affine K)
    (φ ψ : W.FunctionField →+* W.FunctionField) : Prop :=
  ∃ ell : W.FunctionField →+* W.FunctionField, φ.comp ell = ψ

theorem geometric_kernel_inclusion (W : WeierstrassCurve.Affine K) :
    (pointFrobenius W (AlgebraicClosure K)).ker ≤
      (AddMonoidHom.id (extendedCurve (L:=AlgebraicClosure K) W).Point).ker := by
  intro P hP
  change P = 0
  exact (point_frobenius_kernel W _ P).mp hP

omit [Fintype K] in
theorem identity_separable (W : WeierstrassCurve.Affine K) :
    Algebra.IsSeparable W.FunctionField W.FunctionField := inferInstance

theorem actual_factorization_obstruction (W : WeierstrassCurve.Affine K) :
    ¬ RationalFactorization W (fieldFrobenius W).toRingHom (RingHom.id W.FunctionField) :=
  no_field_right_inverse W

/-- A restricted Frobenius-instance of the erroneous universal reading.
All objects are the actual field, geometric points and pullback maps above. -/
def WrongFrobeniusFactorizationReading : Prop :=
  ∀ (K : Type) [Field K] [Fintype K] (W : WeierstrassCurve.Affine K), W.Δ≠0 →
    (pointFrobenius W (AlgebraicClosure K)).ker ≤
      (AddMonoidHom.id (extendedCurve (L:=AlgebraicClosure K) W).Point).ker →
    Algebra.IsSeparable W.FunctionField W.FunctionField →
    RationalFactorization W (fieldFrobenius W).toRingHom (RingHom.id W.FunctionField)

#print axioms actual_factorization_obstruction
end
end IsogenyFactorizationGuardAudit
