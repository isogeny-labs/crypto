import WeightedOverlapFull

set_option autoImplicit false
noncomputable section
namespace WeightedOverlapEquality
open WeightedOverlapFull WeightedOverlapSpecialized Common22WeightedProjQuotient WeightedRingGraded
variable {K : Type*} [Field K]

theorem zLocalized_algebraMap (a3 a2 a1 a0 : K)
    (t : ZAway a3 a2 a1 a0) :
    zLocalizedEquiv a3 a2 a1 a0
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0) t) =
      algebraMap (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
        (Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0))
        ((WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0) t) := by
  unfold zLocalizedEquiv
  exact IsLocalization.ringEquivOfRingEquiv_eq _ t

theorem xLocalized_algebraMap (a3 a2 a1 a0 : K)
    (t : XAway a3 a2 a1 a0) :
    xLocalizedEquiv a3 a2 a1 a0
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0) t) =
      algebraMap (Common22ExplicitChart.ChartRing a3 a2 a1 a0)
        (Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0)
        ((WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0) t) := by
  unfold xLocalizedEquiv
  exact IsLocalization.ringEquivOfRingEquiv_eq _ t

theorem zCommon_algebraMap (a3 a2 a1 a0 : K)
    (t : ZAway a3 a2 a1 a0) :
    zCommonEquiv a3 a2 a1 a0
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0) t) =
      (HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0)
        (show Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
          Zc a3 a2 a1 a0 * Xc a3 a2 a1 a0 from mul_comm _ _)) t := by
  simp [zCommonEquiv]
  rfl

theorem xCommon_algebraMap (a3 a2 a1 a0 : K)
    (t : XAway a3 a2 a1 a0) :
    xCommonEquiv a3 a2 a1 a0
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0) t) =
      (HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
        (Zc_mem a3 a2 a1 a0)
        (rfl : Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
          Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)) t := by
  simp [xCommonEquiv]
  rfl

theorem zLocalized_symm_affineX (a3 a2 a1 a0 : K) :
    (zLocalizedEquiv a3 a2 a1 a0).symm
      (Common22SeparateChartMap.affineX a3 a2 a1 a0) =
      algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)
        (zCoordX a3 a2 a1 a0) := by
  apply (RingEquiv.symm_apply_eq (zLocalizedEquiv a3 a2 a1 a0)).2
  rw [zLocalized_algebraMap]
  change Common22SeparateChartMap.affineX a3 a2 a1 a0 =
    algebraMap (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
      (Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0))
      ((WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0)
        (zCoordX a3 a2 a1 a0))
  change algebraMap (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
    (Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0))
    (Common22WeightedProjQuotient.xbar a3 a2 a1 a0) = _
  congr 1
  exact (WeightedOverlapSpecialized.zChart_x_image a3 a2 a1 a0).symm

theorem xLocalized_z (a3 a2 a1 a0 : K) :
    xLocalizedEquiv a3 a2 a1 a0
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
        (xCoordZ a3 a2 a1 a0)) =
      Common22SeparateChartMap.infinityZ a3 a2 a1 a0 := by
  rw [xLocalized_algebraMap]
  change algebraMap _ _ ((WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0)
    (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
      (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) 1
      (Zc a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0))) = _
  rw [WeightedOverlapSpecialized.xChart_z_image]
  rfl

theorem zCommon_x (a3 a2 a1 a0 : K) :
    zCommonEquiv a3 a2 a1 a0
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)
        (zCoordX a3 a2 a1 a0)) =
      WeightedOverlapScratch.fOverG (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) := by
  rw [zCommon_algebraMap]
  exact WeightedOverlapScratch.zChart_x_to_common
    (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)
    (Zc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)

theorem xCommon_z (a3 a2 a1 a0 : K) :
    xCommonEquiv a3 a2 a1 a0
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
        (xCoordZ a3 a2 a1 a0)) =
      WeightedOverlapScratch.gOverF (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) := by
  rw [xCommon_algebraMap]
  exact WeightedOverlapScratch.xChart_z_to_common
    (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)
    (Zc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)

theorem xCommon_symm_g (a3 a2 a1 a0 : K) :
    (xCommonEquiv a3 a2 a1 a0).symm
      (WeightedOverlapScratch.gOverF (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)) =
      algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
        (xCoordZ a3 a2 a1 a0) := by
  apply (RingEquiv.symm_apply_eq (xCommonEquiv a3 a2 a1 a0)).2
  exact (xCommon_z a3 a2 a1 a0).symm

theorem projTransition_affineX_mul_infinityZ (a3 a2 a1 a0 : K) :
    projTransition a3 a2 a1 a0
      (Common22SeparateChartMap.affineX a3 a2 a1 a0) *
      Common22SeparateChartMap.infinityZ a3 a2 a1 a0 = 1 := by
  let f := WeightedOverlapScratch.fOverG (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
  let g := WeightedOverlapScratch.gOverF (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
  have hfg : f * g = 1 := WeightedOverlapSpecialized.weighted_overlap_inverse a3 a2 a1 a0
  have h1 : projTransition a3 a2 a1 a0
      (Common22SeparateChartMap.affineX a3 a2 a1 a0) =
        xLocalizedEquiv a3 a2 a1 a0 ((xCommonEquiv a3 a2 a1 a0).symm f) := by
    change (xLocalizedEquiv a3 a2 a1 a0)
      ((xCommonEquiv a3 a2 a1 a0).symm
        ((zCommonEquiv a3 a2 a1 a0)
          ((zLocalizedEquiv a3 a2 a1 a0).symm
            (Common22SeparateChartMap.affineX a3 a2 a1 a0)))) = _
    rw [zLocalized_symm_affineX, zCommon_x]
  have h2 : Common22SeparateChartMap.infinityZ a3 a2 a1 a0 =
      xLocalizedEquiv a3 a2 a1 a0 ((xCommonEquiv a3 a2 a1 a0).symm g) := by
    rw [xCommon_symm_g]
    exact (xLocalized_z a3 a2 a1 a0).symm
  rw [h1, h2, ← map_mul, ← map_mul, hfg, map_one, map_one]

theorem projTransition_affineX (a3 a2 a1 a0 : K) :
    projTransition a3 a2 a1 a0
      (Common22SeparateChartMap.affineX a3 a2 a1 a0) =
      Common22SeparateChartMap.infinityInvZ a3 a2 a1 a0 := by
  exact Common22SeparateChartMap.right_inverse_unique
    (a := Common22SeparateChartMap.infinityZ a3 a2 a1 a0)
    (by simpa only [mul_comm] using
      projTransition_affineX_mul_infinityZ a3 a2 a1 a0)
    (by simpa only [mul_comm] using
      Common22SeparateChartMap.infinityInvZ_mul_z a3 a2 a1 a0)

theorem zLocalized_symm_affineY (a3 a2 a1 a0 : K) :
    (zLocalizedEquiv a3 a2 a1 a0).symm
      (Common22SeparateChartMap.affineY a3 a2 a1 a0) =
      algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)
        (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0) 3 (Yc a3 a2 a1 a0)
          (Yc_mem a3 a2 a1 a0)) := by
  apply (RingEquiv.symm_apply_eq (zLocalizedEquiv a3 a2 a1 a0)).2
  rw [zLocalized_algebraMap]
  change Common22SeparateChartMap.affineY a3 a2 a1 a0 =
    algebraMap (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
      (Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0))
      ((WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0)
        (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0) 3 (Yc a3 a2 a1 a0)
          (Yc_mem a3 a2 a1 a0)))
  change algebraMap (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0)
    (Localization.Away (Common22WeightedProjQuotient.xbar a3 a2 a1 a0))
    (Common22WeightedProjQuotient.ybar a3 a2 a1 a0) = _
  congr 1
  exact (WeightedOverlapSpecialized.zChart_y_image a3 a2 a1 a0).symm

theorem xLocalized_w (a3 a2 a1 a0 : K) :
    xLocalizedEquiv a3 a2 a1 a0
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
        (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
          (Xc_mem a3 a2 a1 a0) 3 (Yc a3 a2 a1 a0)
          (Yc_mem a3 a2 a1 a0))) =
      Common22SeparateChartMap.infinityW a3 a2 a1 a0 := by
  rw [xLocalized_algebraMap]
  change algebraMap _ _ ((WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0)
    (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
      (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) 3
      (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0))) = _
  rw [WeightedOverlapSpecialized.xChart_w_image]
  rfl

theorem zCommon_y (a3 a2 a1 a0 : K) :
    zCommonEquiv a3 a2 a1 a0
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)
        (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0) 3 (Yc a3 a2 a1 a0)
          (Yc_mem a3 a2 a1 a0))) =
      WeightedOverlapScratch.yOverG (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
        (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0) := by
  rw [zCommon_algebraMap]
  exact WeightedOverlapScratch.zChart_y_to_common
    (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)
    (Zc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
    (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0)

theorem xCommon_w (a3 a2 a1 a0 : K) :
    xCommonEquiv a3 a2 a1 a0
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)
        (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
          (Xc_mem a3 a2 a1 a0) 3 (Yc a3 a2 a1 a0)
          (Yc_mem a3 a2 a1 a0))) =
      WeightedOverlapScratch.wOverF (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
        (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0) := by
  rw [xCommon_algebraMap]
  exact WeightedOverlapScratch.xChart_w_to_common
    (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)
    (Zc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
    (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0)

theorem projTransition_affineY (a3 a2 a1 a0 : K) :
    projTransition a3 a2 a1 a0
      (Common22SeparateChartMap.affineY a3 a2 a1 a0) =
      Common22SeparateChartMap.infinityW a3 a2 a1 a0 *
        Common22SeparateChartMap.infinityInvZ a3 a2 a1 a0 ^ 3 := by
  let f := WeightedOverlapScratch.fOverG (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
  let y := WeightedOverlapScratch.yOverG (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
    (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0)
  let w := WeightedOverlapScratch.wOverF (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
    (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0)
  have hwy : w * f ^ 3 = y := WeightedOverlapSpecialized.weighted_overlap_coordinates a3 a2 a1 a0
  have hy : projTransition a3 a2 a1 a0
      (Common22SeparateChartMap.affineY a3 a2 a1 a0) =
        xLocalizedEquiv a3 a2 a1 a0 ((xCommonEquiv a3 a2 a1 a0).symm y) := by
    change (xLocalizedEquiv a3 a2 a1 a0)
      ((xCommonEquiv a3 a2 a1 a0).symm
        ((zCommonEquiv a3 a2 a1 a0)
          ((zLocalizedEquiv a3 a2 a1 a0).symm
            (Common22SeparateChartMap.affineY a3 a2 a1 a0)))) = _
    rw [zLocalized_symm_affineY, zCommon_y]
  have hw : xLocalizedEquiv a3 a2 a1 a0
      ((xCommonEquiv a3 a2 a1 a0).symm w) =
        Common22SeparateChartMap.infinityW a3 a2 a1 a0 := by
    dsimp only [w]
    rw [← xCommon_w, RingEquiv.symm_apply_apply]
    exact xLocalized_w a3 a2 a1 a0
  have hf : xLocalizedEquiv a3 a2 a1 a0
      ((xCommonEquiv a3 a2 a1 a0).symm f) =
        Common22SeparateChartMap.infinityInvZ a3 a2 a1 a0 := by
    rw [← projTransition_affineX a3 a2 a1 a0]
    change (xLocalizedEquiv a3 a2 a1 a0)
      ((xCommonEquiv a3 a2 a1 a0).symm f) =
      (xLocalizedEquiv a3 a2 a1 a0)
        ((xCommonEquiv a3 a2 a1 a0).symm
          ((zCommonEquiv a3 a2 a1 a0)
            ((zLocalizedEquiv a3 a2 a1 a0).symm
              (Common22SeparateChartMap.affineX a3 a2 a1 a0))))
    rw [zLocalized_symm_affineX, zCommon_x]
  rw [hy, ← hwy, map_mul, map_pow, map_mul, map_pow, hw, hf]

#print axioms projTransition_affineX
#print axioms projTransition_affineY
end WeightedOverlapEquality
