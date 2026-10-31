import WeightedPic0Bridge
import Common22Domain
import Common22LocalValuation
import Mathlib.RingTheory.DiscreteValuationRing.Basic

/-!
The actual weighted-Proj infinity stalk inherits the proved DVR structure
from the explicit chart localization. This identifies a local parameter,
but does not establish the global principal-divisor or Pic⁰ class equation.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjInfinityDVR

open AlgebraicGeometry CategoryTheory Common22ExplicitChart
  WeightedRingGraded Common22WeightedProjQuotient

variable {K : Type*} [Field K]

abbrev ProjOriginRing (a3 a2 a1 a0 : K) : CommRingCat :=
  (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
    (WeightedPic0Bridge.infinityProjPoint a3 a2 a1 a0)

def originRingEquiv (a3 a2 a1 a0 : K) :
    ProjOriginRing a3 a2 a1 a0 ≃+* OriginStalk a3 a2 a1 a0 :=
  (WeightedPic0Bridge.infinityProjStalkIso a3 a2 a1 a0).commRingCatIsoToRingEquiv

def projectiveW (a3 a2 a1 a0 : K) : ProjOriginRing a3 a2 a1 a0 :=
  (originRingEquiv a3 a2 a1 a0).symm (stalkW a3 a2 a1 a0)

def projectiveZ (a3 a2 a1 a0 : K) : ProjOriginRing a3 a2 a1 a0 :=
  (originRingEquiv a3 a2 a1 a0).symm (stalkZ a3 a2 a1 a0)

def projectiveU (a3 a2 a1 a0 : K) : ProjOriginRing a3 a2 a1 a0 :=
  (originRingEquiv a3 a2 a1 a0).symm (stalkU a3 a2 a1 a0)

theorem projective_origin_domain (a3 a2 a1 a0 : K) :
    IsDomain (ProjOriginRing a3 a2 a1 a0) := by
  letI : IsDomain (OriginStalk a3 a2 a1 a0) :=
    Common22Domain.origin_stalk_domain a3 a2 a1 a0
  exact (originRingEquiv a3 a2 a1 a0).injective.isDomain
    (originRingEquiv a3 a2 a1 a0).toRingHom

instance proj_origin_domain (a3 a2 a1 a0 : K) :
    IsDomain (ProjOriginRing a3 a2 a1 a0) :=
  projective_origin_domain a3 a2 a1 a0

theorem projective_origin_dvr (a3 a2 a1 a0 : K) :
    IsDiscreteValuationRing (ProjOriginRing a3 a2 a1 a0) := by
  letI : IsDomain (OriginStalk a3 a2 a1 a0) :=
    Common22Domain.origin_stalk_domain a3 a2 a1 a0
  letI : IsDiscreteValuationRing (OriginStalk a3 a2 a1 a0) :=
    Common22Domain.origin_stalk_dvr a3 a2 a1 a0
  exact IsDiscreteValuationRing.RingEquivClass.isDiscreteValuationRing
    (originRingEquiv a3 a2 a1 a0).symm

instance proj_origin_dvr (a3 a2 a1 a0 : K) :
    IsDiscreteValuationRing (ProjOriginRing a3 a2 a1 a0) :=
  projective_origin_dvr a3 a2 a1 a0

theorem projectiveW_irreducible (a3 a2 a1 a0 : K) :
    Irreducible (projectiveW a3 a2 a1 a0) := by
  apply Irreducible.of_map (f := originRingEquiv a3 a2 a1 a0)
  simpa [projectiveW] using Common22Domain.stalkW_irreducible a3 a2 a1 a0

theorem projectiveU_unit (a3 a2 a1 a0 : K) :
    IsUnit (projectiveU a3 a2 a1 a0) := by
  have hu : IsUnit (stalkU a3 a2 a1 a0) := by
    simpa only [stalkU] using originStalk_ubar_unit a3 a2 a1 a0
  exact hu.map (originRingEquiv a3 a2 a1 a0).symm.toRingHom

theorem projective_relation (a3 a2 a1 a0 : K) :
    projectiveW a3 a2 a1 a0 ^ 2 =
      projectiveZ a3 a2 a1 a0 * projectiveU a3 a2 a1 a0 := by
  have h := congrArg (originRingEquiv a3 a2 a1 a0).symm
    (originStalk_relation a3 a2 a1 a0)
  simpa only [map_pow, map_mul, projectiveW, projectiveZ, projectiveU,
    stalkW, stalkZ, stalkU] using h

theorem projectiveZ_order_two (a3 a2 a1 a0 : K) :
    IsDiscreteValuationRing.addVal (ProjOriginRing a3 a2 a1 a0)
      (projectiveZ a3 a2 a1 a0) = 2 := by
  let hu := projectiveU_unit a3 a2 a1 a0
  let u : (ProjOriginRing a3 a2 a1 a0)ˣ := hu.unit
  have hueq : (u : ProjOriginRing a3 a2 a1 a0) =
      projectiveU a3 a2 a1 a0 := hu.unit_spec
  apply Common22LocalValuation.chart_z_order
    (projectiveW a3 a2 a1 a0) (projectiveZ a3 a2 a1 a0) u
    (projectiveW_irreducible a3 a2 a1 a0)
  simpa only [hueq] using projective_relation a3 a2 a1 a0

#print axioms projective_origin_domain
#print axioms projective_origin_dvr
#print axioms projectiveW_irreducible
#print axioms projectiveU_unit
#print axioms projective_relation
#print axioms projectiveZ_order_two

end WeightedProjInfinityDVR
