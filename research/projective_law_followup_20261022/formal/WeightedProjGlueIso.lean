import WeightedProjGlueCone
import Mathlib.AlgebraicGeometry.Morphisms.UnderlyingMap
import Mathlib.AlgebraicGeometry.Restrict

/-! A coverage step for the global two-chart morphism. The isomorphism is not asserted. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

namespace WeightedProjGlueIso

open AlgebraicGeometry CategoryTheory WeightedRingGraded

variable {K : Type} [Field K]

theorem gluedToProj_surjective (a3 a2 a1 a0 : K) :
    Surjective (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0) := by
  rw [surjective_iff]
  intro y
  have hy : y ∈
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (WeightedProjCover.gen a3 a2 a1 a0 0) ⊔
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (WeightedProjCover.gen a3 a2 a1 a0 2) := by
    rw [WeightedProjCover.two_basic_opens_cover]
    trivial
  rw [TopologicalSpace.Opens.mem_sup] at hy
  rcases hy with hx | hz
  · let u : (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).toScheme := ⟨y, hx⟩
    let x := (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom u
    refine ⟨Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 true x, ?_⟩
    rw [← Scheme.Hom.comp_apply,
      WeightedProjGlueCone.chartIntoGlued_gluedToProj]
    change ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι) x = y
    simp [x, u]
    change (⟨y, hx⟩ : (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Xc a3 a2 a1 a0))).val = y
    rfl
  · let u : (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).toScheme := ⟨y, hz⟩
    let x := (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom u
    refine ⟨Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 false x, ?_⟩
    rw [← Scheme.Hom.comp_apply,
      WeightedProjGlueCone.chartIntoGlued_gluedToProj]
    change ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι) x = y
    simp [x, u]
    change (⟨y, hz⟩ : (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedOverlapSpecialized.Zc a3 a2 a1 a0))).val = y
    rfl

#print axioms gluedToProj_surjective

end WeightedProjGlueIso
