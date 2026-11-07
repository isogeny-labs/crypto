import WeightedProjQuadraticPoint
import WeightedProjIsIntegral
import WeightedProjLocallyNoetherian
import Mathlib.AlgebraicGeometry.FunctionField
import Mathlib.RingTheory.DiscreteValuationRing.TFAE

/-!
# Local-ring gates at an irreducible support point

For an irreducible support `u` with `u ∣ v² - f`, the previous module
constructs a point of the actual weighted-Proj curve and identifies its stalk
with the chart localization.  Separability of `f` makes the whole curve
integral, hence this stalk a domain; local Noetherianity gives a Noetherian
stalk.  The remaining DVR condition is stated in the exact cotangent-rank
form available in pinned Mathlib.  This file does not prove that rank.

Mathematical reference: Stacks Project Tag 00PD, equivalence between a DVR
and a regular local ring of dimension one.  Formal references:
`WeightedProjIsIntegral`, `WeightedProjLocallyNoetherian`, and pinned Mathlib
`AlgebraicGeometry/FunctionField.lean` and
`RingTheory/DiscreteValuationRing/TFAE.lean`.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjQuadraticLocal

open AlgebraicGeometry Common22WeightedProjQuotient WeightedRingGraded Module

variable {K : Type} [Field K]

abbrev RootStalk (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    CommRingCat :=
  (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
    (WeightedProjQuadraticPoint.rootProjPoint a3 a2 a1 a0 u v hdiv)

theorem root_stalk_domain (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    IsDomain (RootStalk a3 a2 a1 a0 u v hdiv) := by
  haveI : IsIntegral (Proj (degreeSubmodule a3 a2 a1 a0)) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  infer_instance

theorem root_stalk_noetherian (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    IsNoetherianRing (RootStalk a3 a2 a1 a0 u v hdiv) := by
  haveI : IsLocallyNoetherian (Proj (degreeSubmodule a3 a2 a1 a0)) :=
    WeightedProjLocallyNoetherian.whole_curve_locally_noetherian a3 a2 a1 a0
  infer_instance

theorem root_stalk_dvr_of_cotangent_one (a3 a2 a1 a0 : K)
    (u v : Polynomial K) [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (hcot : finrank (IsLocalRing.ResidueField
        (RootStalk a3 a2 a1 a0 u v hdiv))
        (IsLocalRing.CotangentSpace
          (RootStalk a3 a2 a1 a0 u v hdiv)) = 1) :
    @IsDiscreteValuationRing (RootStalk a3 a2 a1 a0 u v hdiv) _
      (root_stalk_domain a3 a2 a1 a0 u v hdiv hsep) := by
  haveI : IsDomain (RootStalk a3 a2 a1 a0 u v hdiv) :=
    root_stalk_domain a3 a2 a1 a0 u v hdiv hsep
  haveI : IsNoetherianRing (RootStalk a3 a2 a1 a0 u v hdiv) :=
    root_stalk_noetherian a3 a2 a1 a0 u v hdiv
  exact (IsLocalRing.finrank_CotangentSpace_eq_one_iff).mp hcot

#print axioms root_stalk_domain
#print axioms root_stalk_noetherian
#print axioms root_stalk_dvr_of_cotangent_one

end WeightedProjQuadraticLocal
