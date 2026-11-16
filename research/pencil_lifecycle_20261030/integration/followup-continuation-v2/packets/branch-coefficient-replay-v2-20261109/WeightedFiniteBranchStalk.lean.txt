import WeightedProjSchemeChart
import Common22BranchStalk
import Mathlib.AlgebraicGeometry.AffineScheme
import Mathlib.AlgebraicGeometry.OpenImmersion

/-!
The rational finite branch point has an actual stalk in the weighted `Proj`.
The `Z` basic-open chart isomorphism and its open immersion identify that
stalk with localization of the affine coordinate ring at the branch ideal.
This is a local-ring transport statement; it does not assert a global divisor
class equation or a theorem about the compiled genus-2 addition law.

Sources: the existing checked `WeightedProjSchemeChart.zBasicOpenIsoAffine`
and `Common22BranchStalk.branchSpecStalkIso`; Stacks Project, Tag 01HD,
identifies stalks under open immersions. Mathlib's
`IsOpenImmersion.iff_isIso_stalkMap` is the formal bridge used below.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedFiniteBranchStalk

open AlgebraicGeometry CategoryTheory Common22WeightedProjQuotient
  Common22WeightedProjKernel WeightedRingGraded Common22BranchStalk

variable {K : Type*} [Field K]

private def finiteBranchChartPoint (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (zbar a3 a2 a1 a0)).toScheme :=
  (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv
    (branchPoint a3 a2 a1 a0 a hroot)

def finiteBranchProjPoint (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    Proj (degreeSubmodule a3 a2 a1 a0) :=
  (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (zbar a3 a2 a1 a0)).ι
      (finiteBranchChartPoint a3 a2 a1 a0 a hroot)

def finiteBranchProjStalkIso (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
      (finiteBranchProjPoint a3 a2 a1 a0 a hroot) ≅
    ↧(Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) := by
  let f := (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (zbar a3 a2 a1 a0)).ι
  let g := (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv
  let q := branchPoint a3 a2 a1 a0 a hroot
  letI : IsOpenImmersion f := inferInstance
  letI : IsOpenImmersion g := inferInstance
  have hi : IsIso (f.stalkMap (g q)) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  have hg : IsIso (g.stalkMap q) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  exact (@asIso CommRingCat _ _ _ (f.stalkMap (g q)) hi) ≪≫
    (@asIso CommRingCat _ _ _ (g.stalkMap q) hg) ≪≫
      branchSpecStalkIso a3 a2 a1 a0 a hroot

#print axioms finiteBranchProjStalkIso

end WeightedFiniteBranchStalk
