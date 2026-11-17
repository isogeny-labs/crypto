import WeightedProjCrossChart

/-! Point-level cross-chart image comparison for weighted Proj. -/

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

namespace WeightedProjCrossImage

open AlgebraicGeometry CategoryTheory WeightedRingGraded

variable {K : Type} [Field K]

theorem z_image_meeting_x_has_overlap_preimage (a3 a2 a1 a0 : K)
    (x : (Common22TwoChartGlue.chart a3 a2 a1 a0 false).carrier)
    (y : (Common22TwoChartGlue.chart a3 a2 a1 a0 true).carrier)
    (h : WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false x =
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true y) :
    x ∈ (Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0).opensRange := by
  rw [← WeightedProjCrossChart.z_preimage_x a3 a2 a1 a0]
  change (WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false) x ∈
    Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)
  rw [h]
  change (((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι) y) ∈ _
  rw [Scheme.Hom.comp_apply]
  change ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι
      ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv y)) ∈
    Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)
  have hr : ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι
        ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv y)) ∈
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι.opensRange := by
    change _ ∈ Set.range ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι)
    exact ⟨_, rfl⟩
  simpa only [Scheme.Opens.opensRange_ι] using hr

#print axioms z_image_meeting_x_has_overlap_preimage

theorem x_image_meeting_z_has_overlap_preimage (a3 a2 a1 a0 : K)
    (x : (Common22TwoChartGlue.chart a3 a2 a1 a0 false).carrier)
    (y : (Common22TwoChartGlue.chart a3 a2 a1 a0 true).carrier)
    (h : WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false x =
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true y) :
    y ∈ (Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding a3 a2 a1 a0).opensRange := by
  rw [← WeightedProjCrossChart.x_preimage_z a3 a2 a1 a0]
  change (WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true) y ∈
    Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)
  rw [← h]
  change (((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι) x) ∈ _
  rw [Scheme.Hom.comp_apply]
  change ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι
      ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv x)) ∈
    Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)
  have hr : ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι
        ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv x)) ∈
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι.opensRange := by
    change _ ∈ Set.range ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι)
    exact ⟨_, rfl⟩
  simpa only [Scheme.Opens.opensRange_ι] using hr

#print axioms x_image_meeting_z_has_overlap_preimage

end WeightedProjCrossImage
