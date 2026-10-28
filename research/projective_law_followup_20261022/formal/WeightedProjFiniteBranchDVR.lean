import WeightedFiniteBranchStalk
import Common22BranchDomain
import Common22LocalValuation
import Mathlib.RingTheory.DiscreteValuationRing.Basic

/-!
At a rational simple branch, the actual weighted-Proj stalk inherits a DVR
structure from the affine branch localization. Its pulled-back `y` is a
uniformizer of order one, and pulled-back `x-a` has order two. This is local
geometry only; no global principal divisor or runtime-law claim follows.

Sources: the checked `WeightedFiniteBranchStalk` and `Common22BranchDomain`
proofs, the infinity-stalk transport pattern in `WeightedProjInfinityDVR`,
and Stacks Project Tag 00PD for the mathematical DVR characterization.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjFiniteBranchDVR

open AlgebraicGeometry CategoryTheory Common22BranchStalk
  Common22BranchLocalParameter Common22BranchParameter
  WeightedRingGraded Common22WeightedProjQuotient

variable {K : Type*} [Field K]

abbrev ProjBranchRing (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) : CommRingCat :=
  (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
    (WeightedFiniteBranchStalk.finiteBranchProjPoint a3 a2 a1 a0 a hroot)

def branchRingEquiv (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ProjBranchRing a3 a2 a1 a0 a hroot ≃+*
      Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot) :=
  (WeightedFiniteBranchStalk.finiteBranchProjStalkIso a3 a2 a1 a0 a hroot).commRingCatIsoToRingEquiv

def projectiveY (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ProjBranchRing a3 a2 a1 a0 a hroot :=
  (branchRingEquiv a3 a2 a1 a0 a hroot).symm
    (stalkY a3 a2 a1 a0 a hroot)

def projectiveXSubA (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ProjBranchRing a3 a2 a1 a0 a hroot :=
  (branchRingEquiv a3 a2 a1 a0 a hroot).symm
    (stalkXSubA a3 a2 a1 a0 a hroot)

def projectiveQ (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ProjBranchRing a3 a2 a1 a0 a hroot :=
  (branchRingEquiv a3 a2 a1 a0 a hroot).symm
    (stalkQ a3 a2 a1 a0 a hroot)

theorem projective_branch_domain (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    IsDomain (ProjBranchRing a3 a2 a1 a0 a hroot) := by
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    Common22BranchDomain.branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  exact (branchRingEquiv a3 a2 a1 a0 a hroot).injective.isDomain
    (branchRingEquiv a3 a2 a1 a0 a hroot).toRingHom

theorem projective_branch_dvr (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing (ProjBranchRing a3 a2 a1 a0 a hroot) _
      (projective_branch_domain a3 a2 a1 a0 a hroot hsimple) := by
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    Common22BranchDomain.branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing
      (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    Common22BranchDomain.branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple
  letI : IsDomain (ProjBranchRing a3 a2 a1 a0 a hroot) :=
    projective_branch_domain a3 a2 a1 a0 a hroot hsimple
  exact IsDiscreteValuationRing.RingEquivClass.isDiscreteValuationRing
    (branchRingEquiv a3 a2 a1 a0 a hroot).symm

theorem projectiveY_irreducible (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    Irreducible (projectiveY a3 a2 a1 a0 a hroot) := by
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    Common22BranchDomain.branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDomain (ProjBranchRing a3 a2 a1 a0 a hroot) :=
    projective_branch_domain a3 a2 a1 a0 a hroot hsimple
  apply Irreducible.of_map (f := branchRingEquiv a3 a2 a1 a0 a hroot)
  simpa [projectiveY] using
    Common22BranchDomain.branch_stalk_y_irreducible a3 a2 a1 a0 a hroot hsimple

theorem projectiveQ_unit (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    IsUnit (projectiveQ a3 a2 a1 a0 a hroot) := by
  have hq : IsUnit (stalkQ a3 a2 a1 a0 a hroot) :=
    branch_qAt_local_unit a3 a2 a1 a0 a hroot hsimple
  exact hq.map (branchRingEquiv a3 a2 a1 a0 a hroot).symm.toRingHom

theorem projective_relation (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    projectiveY a3 a2 a1 a0 a hroot ^ 2 =
      projectiveXSubA a3 a2 a1 a0 a hroot *
        projectiveQ a3 a2 a1 a0 a hroot := by
  have h := congrArg (branchRingEquiv a3 a2 a1 a0 a hroot).symm
    (stalk_relation a3 a2 a1 a0 a hroot)
  simpa only [map_pow, map_mul, projectiveY, projectiveXSubA, projectiveQ]
    using h

theorem projectiveY_order_one (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing.addVal (ProjBranchRing a3 a2 a1 a0 a hroot) _
      (projective_branch_domain a3 a2 a1 a0 a hroot hsimple)
      (projective_branch_dvr a3 a2 a1 a0 a hroot hsimple)
      (projectiveY a3 a2 a1 a0 a hroot) = 1 := by
  letI : IsDomain (ProjBranchRing a3 a2 a1 a0 a hroot) :=
    projective_branch_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing (ProjBranchRing a3 a2 a1 a0 a hroot) :=
    projective_branch_dvr a3 a2 a1 a0 a hroot hsimple
  exact IsDiscreteValuationRing.addVal_uniformizer
    (projectiveY_irreducible a3 a2 a1 a0 a hroot hsimple)

theorem projectiveXSubA_order_two (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing.addVal (ProjBranchRing a3 a2 a1 a0 a hroot) _
      (projective_branch_domain a3 a2 a1 a0 a hroot hsimple)
      (projective_branch_dvr a3 a2 a1 a0 a hroot hsimple)
      (projectiveXSubA a3 a2 a1 a0 a hroot) = 2 := by
  letI : IsDomain (ProjBranchRing a3 a2 a1 a0 a hroot) :=
    projective_branch_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing (ProjBranchRing a3 a2 a1 a0 a hroot) :=
    projective_branch_dvr a3 a2 a1 a0 a hroot hsimple
  let hq := projectiveQ_unit a3 a2 a1 a0 a hroot hsimple
  let q : (ProjBranchRing a3 a2 a1 a0 a hroot)ˣ := hq.unit
  have hqeq : (q : ProjBranchRing a3 a2 a1 a0 a hroot) =
      projectiveQ a3 a2 a1 a0 a hroot := hq.unit_spec
  apply Common22LocalValuation.chart_z_order
    (projectiveY a3 a2 a1 a0 a hroot)
    (projectiveXSubA a3 a2 a1 a0 a hroot) q
    (projectiveY_irreducible a3 a2 a1 a0 a hroot hsimple)
  simpa only [hqeq] using projective_relation a3 a2 a1 a0 a hroot

#print axioms projective_branch_domain
#print axioms projective_branch_dvr
#print axioms projectiveY_order_one
#print axioms projectiveXSubA_order_two

end WeightedProjFiniteBranchDVR
