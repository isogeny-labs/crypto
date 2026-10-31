import WeightedProjGlueOpenImmersion

/-! The remaining cross-chart point identification for the global comparison. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

namespace WeightedProjGlueInjective

open AlgebraicGeometry CategoryTheory WeightedRingGraded

variable {K : Type} [Field K]

private theorem chartIntoProj_injective (a3 a2 a1 a0 : K) (i : Bool) :
    Function.Injective (WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 i) := by
  cases i
  · change Function.Injective
      ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
          (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι)
    intro x y h
    have hU : (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv x =
        (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv y := by
      change ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv x).val =
        ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv y).val at h
      exact Subtype.val_injective h
    exact (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv.homeomorph.injective hU
  · change Function.Injective
      ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
          (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι)
    intro x y h
    have hU : (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv x =
        (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv y := by
      change ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv x).val =
        ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv y).val at h
      exact Subtype.val_injective h
    exact (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv.homeomorph.injective hU

theorem gluedToProj_injective_of_cross (a3 a2 a1 a0 : K)
    (hcross : ∀ (x : (Common22TwoChartGlue.chart a3 a2 a1 a0 false).carrier)
        (y : (Common22TwoChartGlue.chart a3 a2 a1 a0 true).carrier),
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false x =
        WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true y →
      Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 false x =
        Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 true y) :
    Function.Injective (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0) := by
  intro p q hpq
  obtain ⟨i, x, rfl⟩ := Common22TwoChartGlue.chartIntoGlued_jointly_surjective
    a3 a2 a1 a0 p
  obtain ⟨j, y, rfl⟩ := Common22TwoChartGlue.chartIntoGlued_jointly_surjective
    a3 a2 a1 a0 q
  have hproj : WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 i x =
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 j y := by
    simpa only [← WeightedProjGlueCone.chartIntoGlued_gluedToProj,
      Scheme.Hom.comp_apply] using hpq
  cases i <;> cases j
  · congr 1
    exact chartIntoProj_injective a3 a2 a1 a0 false hproj
  · exact hcross x y hproj
  · exact (hcross y x hproj.symm).symm
  · congr 1
    exact chartIntoProj_injective a3 a2 a1 a0 true hproj

theorem gluedToProj_isIso_of_cross (a3 a2 a1 a0 : K)
    (hcross : ∀ (x : (Common22TwoChartGlue.chart a3 a2 a1 a0 false).carrier)
        (y : (Common22TwoChartGlue.chart a3 a2 a1 a0 true).carrier),
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false x =
        WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true y →
      Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 false x =
        Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 true y) :
    IsIso (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0) :=
  WeightedProjGlueOpenImmersion.gluedToProj_isIso_of_injective a3 a2 a1 a0
    (gluedToProj_injective_of_cross a3 a2 a1 a0 hcross)

#print axioms chartIntoProj_injective
#print axioms gluedToProj_injective_of_cross
#print axioms gluedToProj_isIso_of_cross

end WeightedProjGlueInjective
