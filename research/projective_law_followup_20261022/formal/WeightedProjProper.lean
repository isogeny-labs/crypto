import WeightedProjCover
import Mathlib.AlgebraicGeometry.ProjectiveSpectrum.Proper

set_option autoImplicit false
noncomputable section
namespace WeightedProjProper
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
  WeightedRingComponent WeightedRingGraded WeightedProjCover
open AlgebraicGeometry
variable {K : Type*} [Field K]

instance finiteType (a3 a2 a1 a0 : K) :
    Algebra.FiniteType (degreeSubmodule a3 a2 a1 a0 0)
      (WeightedRing a3 a2 a1 a0) := by
  classical
  refine ⟨⟨Finset.univ.image (gen a3 a2 a1 a0), ?_⟩⟩
  simpa only [Finset.coe_image, Finset.coe_univ, Set.image_univ] using
    gen_adjoin_top a3 a2 a1 a0

theorem weightedProj_proper (a3 a2 a1 a0 : K) :
    IsProper (Proj.toSpecZero (degreeSubmodule a3 a2 a1 a0)) := by
  infer_instance

#print axioms finiteType
#print axioms weightedProj_proper
end WeightedProjProper
