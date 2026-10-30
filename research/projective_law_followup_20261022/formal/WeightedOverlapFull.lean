import WeightedOverlapScratch
import Mathlib.RingTheory.Localization.Basic

set_option autoImplicit false
noncomputable section
namespace WeightedOverlapFull
open WeightedOverlapSpecialized Common22WeightedProjQuotient WeightedRingGraded
variable {K : Type*} [Field K]

abbrev ZAway (a3 a2 a1 a0 : K) :=
  HomogeneousLocalization.Away (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)
abbrev XAway (a3 a2 a1 a0 : K) :=
  HomogeneousLocalization.Away (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)

def zCoordX (a3 a2 a1 a0 : K) : ZAway a3 a2 a1 a0 :=
  HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
    (Zc_mem a3 a2 a1 a0) 1 (Xc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0)
def xCoordZ (a3 a2 a1 a0 : K) : XAway a3 a2 a1 a0 :=
  HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) 1 (Zc a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)

abbrev ZOpen (a3 a2 a1 a0 : K) := Localization.Away (zCoordX a3 a2 a1 a0)
abbrev XOpen (a3 a2 a1 a0 : K) := Localization.Away (xCoordZ a3 a2 a1 a0)

noncomputable def zLocalizedEquiv (a3 a2 a1 a0 : K) :
    ZOpen a3 a2 a1 a0 ≃+* Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0) := by
  exact IsLocalization.ringEquivOfRingEquiv
    (M := Submonoid.powers (zCoordX a3 a2 a1 a0))
    (T := Submonoid.powers (Common22WeightedProjQuotient.xbar a3 a2 a1 a0))
    (ZOpen a3 a2 a1 a0) (Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0))
    (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0)
    (by rw [Submonoid.map_powers]
        exact congrArg Submonoid.powers (by
          change (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0)
            (zCoordX a3 a2 a1 a0) = Common22WeightedProjQuotient.xbar a3 a2 a1 a0
          exact WeightedOverlapSpecialized.zChart_x_image a3 a2 a1 a0))

noncomputable def xLocalizedEquiv (a3 a2 a1 a0 : K) :
    XOpen a3 a2 a1 a0 ≃+* Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0 := by
  exact IsLocalization.ringEquivOfRingEquiv
    (M := Submonoid.powers (xCoordZ a3 a2 a1 a0))
    (T := Submonoid.powers (Common22ExplicitChart.zbar a3 a2 a1 a0))
    (XOpen a3 a2 a1 a0) (Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0)
    (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0)
    (by rw [Submonoid.map_powers]
        exact congrArg Submonoid.powers (by
          change (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0)
            (xCoordZ a3 a2 a1 a0) = Common22ExplicitChart.zbar a3 a2 a1 a0
          exact WeightedOverlapSpecialized.xChart_z_image a3 a2 a1 a0))

abbrev CommonAway (a3 a2 a1 a0 : K) :=
  HomogeneousLocalization.Away (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)

noncomputable def zCommonEquiv (a3 a2 a1 a0 : K) :
    ZOpen a3 a2 a1 a0 ≃+* CommonAway a3 a2 a1 a0 := by
  letI := (HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0)
    (show Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
      Zc a3 a2 a1 a0 * Xc a3 a2 a1 a0 from mul_comm _ _)).toAlgebra
  have hzElem :
      HomogeneousLocalization.Away.isLocalizationElem
        (Zc_mem a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) =
        zCoordX a3 a2 a1 a0 := by
    apply (HomogeneousLocalization.ext_iff_val _ _).2
    simp [HomogeneousLocalization.Away.isLocalizationElem, zCoordX,
      HomogeneousLocalization.Away.val_mk]
  letI : IsLocalization.Away (zCoordX a3 a2 a1 a0)
      (CommonAway a3 a2 a1 a0) := by
    rw [← hzElem]
    exact HomogeneousLocalization.Away.isLocalization_mul
      (Zc_mem a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0)
      (show Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
        Zc a3 a2 a1 a0 * Xc a3 a2 a1 a0 from mul_comm _ _) (by decide)
  exact (IsLocalization.algEquiv
    (Submonoid.powers (zCoordX a3 a2 a1 a0))
    (ZOpen a3 a2 a1 a0) (CommonAway a3 a2 a1 a0)).toRingEquiv

noncomputable def xCommonEquiv (a3 a2 a1 a0 : K) :
    XOpen a3 a2 a1 a0 ≃+* CommonAway a3 a2 a1 a0 := by
  letI := (HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
    (Zc_mem a3 a2 a1 a0)
    (rfl : Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
      Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)).toAlgebra
  have hxElem :
      HomogeneousLocalization.Away.isLocalizationElem
        (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) =
        xCoordZ a3 a2 a1 a0 := by
    apply (HomogeneousLocalization.ext_iff_val _ _).2
    simp [HomogeneousLocalization.Away.isLocalizationElem, xCoordZ,
      HomogeneousLocalization.Away.val_mk]
  letI : IsLocalization.Away (xCoordZ a3 a2 a1 a0)
      (CommonAway a3 a2 a1 a0) := by
    rw [← hxElem]
    exact HomogeneousLocalization.Away.isLocalization_mul
      (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
      (rfl : Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
        Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0) (by decide)
  exact (IsLocalization.algEquiv
    (Submonoid.powers (xCoordZ a3 a2 a1 a0))
    (XOpen a3 a2 a1 a0) (CommonAway a3 a2 a1 a0)).toRingEquiv

noncomputable def projTransition (a3 a2 a1 a0 : K) :
    Common22SeparateChartMap.AffineOpen a3 a2 a1 a0 ≃+*
      Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0 :=
  (zLocalizedEquiv a3 a2 a1 a0).symm |>.trans
    ((zCommonEquiv a3 a2 a1 a0).trans
      ((xCommonEquiv a3 a2 a1 a0).symm.trans
        (xLocalizedEquiv a3 a2 a1 a0)))


#print axioms projTransition

#print axioms zCommonEquiv
#print axioms xCommonEquiv

#print axioms zLocalizedEquiv
#print axioms xLocalizedEquiv
end WeightedOverlapFull

example {K : Type*} [Field K] (a3 a2 a1 a0 : K) :
    Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0 =
      Common22SeparateChartMap.AffineRing a3 a2 a1 a0 := rfl
example {K : Type*} [Field K] (a3 a2 a1 a0 : K) :
    Common22WeightedProjQuotient.xbar a3 a2 a1 a0 =
      Common22SeparateChartMap.xbar a3 a2 a1 a0 := rfl
