import WeightedInfinityOrdinary
import WeightedChartDehomog
import Mathlib.AlgebraicGeometry.ProjectiveSpectrum.Basic

set_option autoImplicit false
noncomputable section
namespace WeightedProjInfinityChart
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
  WeightedRingComponent WeightedRingGraded WeightedInfinityOrdinary
  AlgebraicGeometry CategoryTheory
variable {K : Type*} [Field K]

theorem x_homogeneous (a3 a2 a1 a0 : K) :
    WeightedInfinityOrdinary.xbar a3 a2 a1 a0 ∈ degreeSubmodule a3 a2 a1 a0 1 := by
  change component a3 a2 a1 a0 1 (WeightedInfinityOrdinary.xbar a3 a2 a1 a0) =
    WeightedInfinityOrdinary.xbar a3 a2 a1 a0
  unfold WeightedInfinityOrdinary.xbar
  have hx : IsWeightedHomogeneous weights (X (0 : Fin 3) : Poly K) 1 := by
    simpa [weights] using (isWeightedHomogeneous_X (R := K) weights (0 : Fin 3))
  simpa using (component_homogeneous a3 a2 a1 a0 1 1 (X (0 : Fin 3)) hx)

def xAwayEquivInfinity (a3 a2 a1 a0 : K) :
    HomogeneousLocalization.Away (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0) ≃+* InfinityRing a3 a2 a1 a0 :=
  (WeightedChartDehomog.awayEquivQuotient
    (degreeSubmodule a3 a2 a1 a0) (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)
    (x_homogeneous a3 a2 a1 a0)).trans
    (xOneEquivInfinity a3 a2 a1 a0).toRingEquiv

def xBasicOpenIsoInfinity (a3 a2 a1 a0 : K) :
    (AlgebraicGeometry.Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)).toScheme ≅
      Spec ↧(InfinityRing a3 a2 a1 a0) :=
  (AlgebraicGeometry.Proj.basicOpenIsoSpec
    (degreeSubmodule a3 a2 a1 a0) (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)
    (x_homogeneous a3 a2 a1 a0) (by decide : 0 < (1 : ℕ))) ≪≫
  Scheme.Spec.mapIso ((xAwayEquivInfinity a3 a2 a1 a0).symm.toCommRingCatIso.op)

#print axioms xAwayEquivInfinity
#print axioms xBasicOpenIsoInfinity
end WeightedProjInfinityChart
