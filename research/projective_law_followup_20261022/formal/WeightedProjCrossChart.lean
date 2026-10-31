import WeightedProjGlueInjective

/-! Opposite basic-open preimages for the two explicit affine charts. -/

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
set_option backward.isDefEq.respectTransparency false
noncomputable section

namespace WeightedProjCrossChart

open AlgebraicGeometry CategoryTheory WeightedRingGraded WeightedOverlapSpecialized
  Common22WeightedProjQuotient WeightedOverlapFull

variable {K : Type} [Field K]

theorem z_preimage_x (a3 a2 a1 a0 : K) :
    WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false ⁻¹ᵁ
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0) =
    (Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0).opensRange := by
  rw [Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding_opensRange]
  change ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)).ι) ⁻¹ᵁ
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0) =
    PrimeSpectrum.basicOpen (Common22SeparateChartMap.xbar a3 a2 a1 a0)
  rw [WeightedProjChartIntoProj.zChartIsoInv]
  change (Spec.map (CommRingCat.ofHom
      (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0).toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Zc_mem a3 a2 a1 a0) (by decide)) ⁻¹ᵁ
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0) =
    PrimeSpectrum.basicOpen (Common22SeparateChartMap.xbar a3 a2 a1 a0)
  rw [Scheme.Hom.comp_preimage,
    Proj.awayι_preimage_basicOpen
      (f_deg := Zc_mem a3 a2 a1 a0) (g_deg := Xc_mem a3 a2 a1 a0)
      (hm := by decide) (hm' := by decide)]
  rw [SpecMap_preimage_basicOpen]
  have helem : HomogeneousLocalization.Away.isLocalizationElem
      (Zc_mem a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) =
      zCoordX a3 a2 a1 a0 := by
    apply (HomogeneousLocalization.ext_iff_val _ _).2
    simp [HomogeneousLocalization.Away.isLocalizationElem, zCoordX,
      HomogeneousLocalization.Away.val_mk]
  rw [helem]
  have hx : (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0)
      (zCoordX a3 a2 a1 a0) = Common22WeightedProjQuotient.xbar a3 a2 a1 a0 := by
    exact WeightedOverlapSpecialized.zChart_x_image a3 a2 a1 a0
  exact congrArg PrimeSpectrum.basicOpen hx

#print axioms z_preimage_x

theorem x_preimage_z (a3 a2 a1 a0 : K) :
    WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true ⁻¹ᵁ
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0) =
    (Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding a3 a2 a1 a0).opensRange := by
  rw [Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding_opensRange]
  change ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)).ι) ⁻¹ᵁ
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0) =
    PrimeSpectrum.basicOpen (Common22ExplicitChart.zbar a3 a2 a1 a0)
  rw [WeightedProjChartIntoProj.xChartIsoInv]
  change (Spec.map (CommRingCat.ofHom
      (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0).toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (by decide)) ⁻¹ᵁ
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0) =
    PrimeSpectrum.basicOpen (Common22ExplicitChart.zbar a3 a2 a1 a0)
  rw [Scheme.Hom.comp_preimage,
    Proj.awayι_preimage_basicOpen
      (f_deg := Xc_mem a3 a2 a1 a0) (g_deg := Zc_mem a3 a2 a1 a0)
      (hm := by decide) (hm' := by decide)]
  rw [SpecMap_preimage_basicOpen]
  have helem : HomogeneousLocalization.Away.isLocalizationElem
      (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) =
      xCoordZ a3 a2 a1 a0 := by
    apply (HomogeneousLocalization.ext_iff_val _ _).2
    simp [HomogeneousLocalization.Away.isLocalizationElem, xCoordZ,
      HomogeneousLocalization.Away.val_mk]
  rw [helem]
  have hz : (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0)
      (xCoordZ a3 a2 a1 a0) = Common22ExplicitChart.zbar a3 a2 a1 a0 := by
    exact WeightedOverlapSpecialized.xChart_z_image a3 a2 a1 a0
  exact congrArg PrimeSpectrum.basicOpen hz

#print axioms x_preimage_z

end WeightedProjCrossChart
