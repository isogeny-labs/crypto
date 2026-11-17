import WeightedChartIso
import Mathlib.AlgebraicGeometry.ProjectiveSpectrum.Basic

set_option autoImplicit false
noncomputable section
namespace WeightedProjSchemeChart
open AlgebraicGeometry CategoryTheory Common22WeightedProjQuotient
  Common22WeightedProjKernel WeightedRingGraded WeightedChartIso
variable {K : Type*} [Field K]

def zBasicOpenIsoAffine (a3 a2 a1 a0 : K) :
    (AlgebraicGeometry.Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (zbar a3 a2 a1 a0)).toScheme ≅
      Spec ↧(AffineRing a3 a2 a1 a0) :=
  (AlgebraicGeometry.Proj.basicOpenIsoSpec
    (degreeSubmodule a3 a2 a1 a0) (zbar a3 a2 a1 a0)
    (z_homogeneous a3 a2 a1 a0) (by decide : 0 < (1 : ℕ))) ≪≫
  Scheme.Spec.mapIso ((weightedAwayEquivAffine a3 a2 a1 a0).symm.toCommRingCatIso.op)

#print axioms zBasicOpenIsoAffine
end WeightedProjSchemeChart
