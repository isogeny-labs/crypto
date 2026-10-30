import Common22Domain
import Mathlib.AlgebraicGeometry.AffineScheme

/-! The origin localization of the explicit infinity chart is the stalk of
the affine scheme represented by that chart. This does not identify it with
the stalk of a projective completion. -/

set_option autoImplicit false

noncomputable section

namespace Common22AffineStalk

variable {K : Type*} [Field K]

def originPoint (a3 a2 a1 a0 : K) :
    PrimeSpectrum (Common22ExplicitChart.ChartRing a3 a2 a1 a0) :=
  ⟨Common22ExplicitChart.originIdeal a3 a2 a1 a0,
    Common22ExplicitChart.originIdeal_prime a3 a2 a1 a0⟩

def affineStalkIso (a3 a2 a1 a0 : K) :
    (AlgebraicGeometry.Spec
      (CommRingCat.of (Common22ExplicitChart.ChartRing a3 a2 a1 a0))).presheaf.stalk
        (originPoint a3 a2 a1 a0) ≅
      ↧(Common22ExplicitChart.OriginStalk a3 a2 a1 a0) :=
  by
    change
      (AlgebraicGeometry.Spec
        (CommRingCat.of (Common22ExplicitChart.ChartRing a3 a2 a1 a0))).presheaf.stalk
          (originPoint a3 a2 a1 a0) ≅
        ↧(Localization.AtPrime (originPoint a3 a2 a1 a0).asIdeal)
    exact AlgebraicGeometry.Spec.stalkIso
      (CommRingCat.of (Common22ExplicitChart.ChartRing a3 a2 a1 a0))
      (originPoint a3 a2 a1 a0)

#print axioms originPoint
#print axioms affineStalkIso

end Common22AffineStalk
