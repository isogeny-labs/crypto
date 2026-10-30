import WeightedOverlapMapEquality
import WeightedProjBase
set_option autoImplicit false
noncomputable section
namespace WeightedOverlapScalar
variable {ι A σ B : Type*} [CommRing A] [CommRing B] [SetLike σ A]
  [AddSubgroupClass σ A] [AddCommMonoid ι] [DecidableEq ι]
variable (𝒜 : ι → σ) [GradedRing 𝒜] (g : A →+* B) {f : A} {d : ι}

theorem awayToOne_fromZero (hf : f ∈ 𝒜 d) (h : g f = 1) (a : 𝒜 0) :
    WeightedChartRoute.awayToOne 𝒜 g h
      (HomogeneousLocalization.fromZeroRingHom 𝒜 (Submonoid.powers f) a) = g (a : A) := by
  have ha : (a : A) ∈ 𝒜 (0 • d) := by simpa using a.property
  have hmk : HomogeneousLocalization.fromZeroRingHom 𝒜 (Submonoid.powers f) a =
      HomogeneousLocalization.Away.mk 𝒜 hf 0 (a : A) ha := by
    apply (HomogeneousLocalization.ext_iff_val _ _).2
    change Localization.mk (a : A) (1 : Submonoid.powers f) =
      Localization.mk (a : A) (⟨f ^ 0, by simp⟩ : Submonoid.powers f)
    exact congrArg (fun s : Submonoid.powers f => Localization.mk (a : A) s)
      (Subtype.ext (by simp))
  rw [hmk]
  exact WeightedChartRoute.awayToOne_mk 𝒜 g hf h 0 (a : A) ha

#print axioms awayToOne_fromZero
end WeightedOverlapScalar

namespace WeightedOverlapScalar
open WeightedRingGraded Common22WeightedProjQuotient
variable {K : Type*} [Field K]

theorem zChart_scalar (a3 a2 a1 a0 c : K) :
    (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0)
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (Common22WeightedProjKernel.zbar a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) =
      algebraMap K (AffineRing a3 a2 a1 a0) c := by
  change WeightedChartIso.weightedAwayToAffine a3 a2 a1 a0
    ((HomogeneousLocalization.fromZeroRingHom
      (degreeSubmodule a3 a2 a1 a0)
      (Submonoid.powers (Common22WeightedProjKernel.zbar a3 a2 a1 a0)))
      (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) = _
  unfold WeightedChartIso.weightedAwayToAffine
  rw [awayToOne_fromZero (degreeSubmodule a3 a2 a1 a0)
    (Common22WeightedProjQuotient.toAffine a3 a2 a1 a0).toRingHom
    (WeightedChartIso.z_homogeneous a3 a2 a1 a0)
    (by simpa [Common22WeightedProjKernel.zbar] using
      Common22WeightedProjQuotient.toAffine_Z a3 a2 a1 a0)]
  change (Common22WeightedProjQuotient.toAffine a3 a2 a1 a0)
    ((WeightedProjBase.scalarToZero a3 a2 a1 a0 c :
      degreeSubmodule a3 a2 a1 a0 0) :
      Common22WeightedProjQuotient.WeightedRing a3 a2 a1 a0) = _
  rw [WeightedProjBase.scalarToZero_coe]
  exact (Common22WeightedProjQuotient.toAffine a3 a2 a1 a0).commutes c

#print axioms zChart_scalar
end WeightedOverlapScalar

namespace WeightedOverlapScalar
open WeightedRingGraded Common22WeightedProjQuotient WeightedInfinityOrdinary
variable {K : Type*} [Field K]

theorem xChart_scalar (a3 a2 a1 a0 c : K) :
    (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0)
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) =
      algebraMap K (InfinityRing a3 a2 a1 a0) c := by
  change (WeightedInfinityOrdinary.xOneEquivInfinity a3 a2 a1 a0)
    ((WeightedChartDehomog.awayEquivQuotient
      (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)
      (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0))
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c))) = _
  change (WeightedInfinityOrdinary.xOneEquivInfinity a3 a2 a1 a0)
    ((WeightedChartRoute.awayToOne (degreeSubmodule a3 a2 a1 a0)
      (WeightedChartDehomog.quotientMap (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))
      (WeightedChartDehomog.quotientMap_f
        (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)))
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c))) = _
  rw [awayToOne_fromZero (degreeSubmodule a3 a2 a1 a0)
    (WeightedChartDehomog.quotientMap (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))
    (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0)
    (WeightedChartDehomog.quotientMap_f
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))]
  change (WeightedInfinityOrdinary.xOneEquivInfinity a3 a2 a1 a0)
    ((WeightedInfinityOrdinary.weightedToXOne a3 a2 a1 a0)
      ((WeightedProjBase.scalarToZero a3 a2 a1 a0 c :
        degreeSubmodule a3 a2 a1 a0 0) : WeightedRing a3 a2 a1 a0)) = _
  rw [WeightedProjBase.scalarToZero_coe]
  change (WeightedInfinityOrdinary.xOneEquivInfinity a3 a2 a1 a0)
    ((WeightedInfinityOrdinary.weightedToXOne a3 a2 a1 a0)
      (algebraMap K (WeightedRing a3 a2 a1 a0) c)) = _
  rw [(WeightedInfinityOrdinary.weightedToXOne a3 a2 a1 a0).commutes c]
  exact (WeightedInfinityOrdinary.xOneEquivInfinity a3 a2 a1 a0).commutes c

#print axioms xChart_scalar
end WeightedOverlapScalar

namespace WeightedOverlapScalar
open WeightedRingGraded WeightedOverlapFull
variable {K : Type*} [Field K]

theorem zLocalized_scalar (a3 a2 a1 a0 c : K) :
    (zLocalizedEquiv a3 a2 a1 a0).symm
      (algebraMap K (Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) c) =
    algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) := by
  apply (RingEquiv.symm_apply_eq (zLocalizedEquiv a3 a2 a1 a0)).2
  change algebraMap K (Localization.Away
    (Common22WeightedProjQuotient.xbar a3 a2 a1 a0)) c =
    (zLocalizedEquiv a3 a2 a1 a0)
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)
        ((HomogeneousLocalization.fromZeroRingHom
          (degreeSubmodule a3 a2 a1 a0)
          (Submonoid.powers (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)))
          (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)))
  rw [WeightedOverlapEquality.zLocalized_algebraMap]
  have hchart : (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0)
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) =
      algebraMap K (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0) c := by
    exact zChart_scalar a3 a2 a1 a0 c
  rw [hchart]
  exact (IsScalarTower.algebraMap_apply K
    (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
    (Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0)) c).symm

#print axioms zLocalized_scalar
end WeightedOverlapScalar

namespace WeightedOverlapScalar
open WeightedRingGraded WeightedOverlapFull
variable {K : Type*} [Field K]

theorem xLocalized_scalar (a3 a2 a1 a0 c : K) :
    (xLocalizedEquiv a3 a2 a1 a0)
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
        ((HomogeneousLocalization.fromZeroRingHom
          (degreeSubmodule a3 a2 a1 a0)
          (Submonoid.powers (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)))
          (WeightedProjBase.scalarToZero a3 a2 a1 a0 c))) =
      algebraMap K (Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0) c := by
  rw [WeightedOverlapEquality.xLocalized_algebraMap]
  have hchart : (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0)
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) =
      algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0) c := by
    exact xChart_scalar a3 a2 a1 a0 c
  rw [hchart]
  exact (IsScalarTower.algebraMap_apply K
    (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0)
    (Localization.Away (Common22ExplicitChart.zbar a3 a2 a1 a0)) c).symm

#print axioms xLocalized_scalar
end WeightedOverlapScalar

namespace WeightedOverlapScalar
open WeightedRingGraded WeightedOverlapFull WeightedOverlapSpecialized
variable {K : Type*} [Field K]

theorem zCommon_scalar (a3 a2 a1 a0 c : K) :
    (zCommonEquiv a3 a2 a1 a0)
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)
        ((HomogeneousLocalization.fromZeroRingHom
          (degreeSubmodule a3 a2 a1 a0)
          (Submonoid.powers (Zc a3 a2 a1 a0)))
          (WeightedProjBase.scalarToZero a3 a2 a1 a0 c))) =
    (HomogeneousLocalization.fromZeroRingHom
      (degreeSubmodule a3 a2 a1 a0)
      (Submonoid.powers (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)))
      (WeightedProjBase.scalarToZero a3 a2 a1 a0 c) := by
  rw [WeightedOverlapEquality.zCommon_algebraMap]
  exact HomogeneousLocalization.awayMap_fromZeroRingHom
    (degreeSubmodule a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0)
    (show Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
      Zc a3 a2 a1 a0 * Xc a3 a2 a1 a0 from mul_comm _ _)
    (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)

#print axioms zCommon_scalar

theorem xCommon_scalar (a3 a2 a1 a0 c : K) :
    (xCommonEquiv a3 a2 a1 a0)
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
        ((HomogeneousLocalization.fromZeroRingHom
          (degreeSubmodule a3 a2 a1 a0)
          (Submonoid.powers (Xc a3 a2 a1 a0)))
          (WeightedProjBase.scalarToZero a3 a2 a1 a0 c))) =
    (HomogeneousLocalization.fromZeroRingHom
      (degreeSubmodule a3 a2 a1 a0)
      (Submonoid.powers (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)))
      (WeightedProjBase.scalarToZero a3 a2 a1 a0 c) := by
  rw [WeightedOverlapEquality.xCommon_algebraMap]
  exact HomogeneousLocalization.awayMap_fromZeroRingHom
    (degreeSubmodule a3 a2 a1 a0)
    (Zc_mem a3 a2 a1 a0)
    (rfl : Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
      Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
    (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)

#print axioms xCommon_scalar
end WeightedOverlapScalar

namespace WeightedOverlapScalar
open WeightedRingGraded WeightedOverlapFull WeightedOverlapSpecialized
variable {K : Type*} [Field K]

theorem xCommon_symm_scalar (a3 a2 a1 a0 c : K) :
    (xCommonEquiv a3 a2 a1 a0).symm
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) =
    algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (Xc a3 a2 a1 a0)))
        (WeightedProjBase.scalarToZero a3 a2 a1 a0 c)) := by
  apply (RingEquiv.symm_apply_eq (xCommonEquiv a3 a2 a1 a0)).2
  exact (xCommon_scalar a3 a2 a1 a0 c).symm

#print axioms xCommon_symm_scalar

theorem projTransition_scalar (a3 a2 a1 a0 c : K) :
    (projTransition a3 a2 a1 a0)
      (algebraMap K (Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) c) =
    algebraMap K (Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0) c := by
  change (xLocalizedEquiv a3 a2 a1 a0)
    ((xCommonEquiv a3 a2 a1 a0).symm
      ((zCommonEquiv a3 a2 a1 a0)
        ((zLocalizedEquiv a3 a2 a1 a0).symm
          (algebraMap K (Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) c)))) = _
  rw [zLocalized_scalar, zCommon_scalar, xCommon_symm_scalar, xLocalized_scalar]

#print axioms projTransition_scalar
end WeightedOverlapScalar

namespace WeightedOverlapScalar
open WeightedOverlapFull
variable {K : Type*} [Field K]

theorem projTransition_eq_separate (a3 a2 a1 a0 : K) :
    projTransition a3 a2 a1 a0 =
      (Common22SeparateChartMap.separateChartOpenEquiv a3 a2 a1 a0).toRingEquiv := by
  exact WeightedOverlapMapEquality.projTransition_eq_of_scalar
    a3 a2 a1 a0 (projTransition_scalar a3 a2 a1 a0)

#print axioms projTransition_eq_separate
end WeightedOverlapScalar
