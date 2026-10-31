import WeightedOverlapScalar
import Common22TwoChartGlue

/-! The weighted coordinate-ring transition induces the transition used by
the separately glued two-chart scheme. This identifies the morphism on the
principal-open overlap; it does not identify the glued scheme with Proj. -/

set_option autoImplicit false
noncomputable section

namespace WeightedOverlapScheme

open AlgebraicGeometry CategoryTheory

variable {K : Type} [Field K]

def projTransitionSpecIso (a3 a2 a1 a0 : K) :
    Spec ↧(Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) ≅
      Spec ↧(Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0) :=
  Scheme.Spec.mapIso
    (RingEquiv.toCommRingCatIso
      (WeightedOverlapFull.projTransition a3 a2 a1 a0)).symm.op

theorem projTransitionSpecIso_eq_separate (a3 a2 a1 a0 : K) :
    projTransitionSpecIso a3 a2 a1 a0 =
      Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0 := by
  rw [projTransitionSpecIso, WeightedOverlapScalar.projTransition_eq_separate]
  rfl

theorem projTransitionSpecIso_hom_eq_glue (a3 a2 a1 a0 : K) :
    (projTransitionSpecIso a3 a2 a1 a0).hom =
      Common22TwoChartGlue.transition a3 a2 a1 a0 false true (by decide) := by
  rw [projTransitionSpecIso_eq_separate]
  rfl

#print axioms projTransitionSpecIso_eq_separate
#print axioms projTransitionSpecIso_hom_eq_glue

end WeightedOverlapScheme
