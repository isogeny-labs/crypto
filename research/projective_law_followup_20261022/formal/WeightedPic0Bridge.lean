import WeightedProjInfinityChart
import Common22CompletionBridge
import Mathlib.AlgebraicGeometry.AffineScheme
import Mathlib.AlgebraicGeometry.OpenImmersion

/-!
The infinity-chart origin has a stalk in the actual weighted `Proj`.  This
transports the existing explicit local-ring model through the checked chart
isomorphism and the basic-open immersion.  It does not construct a Picard
group or assert an equality of divisor classes.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedPic0Bridge

open AlgebraicGeometry CategoryTheory Common22CompletionBridge
  Common22ExplicitChart WeightedRingGraded WeightedInfinityOrdinary
  Common22WeightedProjQuotient

variable {K : Type*} [Field K]

private def infinityChartPoint (a3 a2 a1 a0 : K) :
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)).toScheme :=
  (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv
    (infinityOrigin a3 a2 a1 a0)

def infinityProjPoint (a3 a2 a1 a0 : K) :
    Proj (degreeSubmodule a3 a2 a1 a0) :=
  (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)).ι
      (infinityChartPoint a3 a2 a1 a0)

def infinityProjStalkIso (a3 a2 a1 a0 : K) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
      (infinityProjPoint a3 a2 a1 a0) ≅
    ↧(Localization.AtPrime (infinityOrigin a3 a2 a1 a0).asIdeal) := by
  let f := (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)).ι
  let g := (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv
  let q := infinityOrigin a3 a2 a1 a0
  letI : IsOpenImmersion f := inferInstance
  letI : IsOpenImmersion g := inferInstance
  have hi : IsIso (f.stalkMap (g q)) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  have hg : IsIso (g.stalkMap q) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  exact (@asIso CommRingCat _ _ _ (f.stalkMap (g q)) hi) ≪≫
    (@asIso CommRingCat _ _ _ (g.stalkMap q) hg) ≪≫
      Spec.stalkIso (CommRingCat.of (ChartRing a3 a2 a1 a0))
        (infinityOrigin a3 a2 a1 a0)

#print axioms infinityProjStalkIso

end WeightedPic0Bridge
