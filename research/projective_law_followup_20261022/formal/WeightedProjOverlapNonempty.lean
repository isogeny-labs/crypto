import WeightedProjCrossChart
import WeightedProjIntegral

/-! A point in the actual weighted-Proj X/Z basic-open overlap. -/

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
set_option backward.isDefEq.respectTransparency false
noncomputable section

namespace WeightedProjOverlapNonempty

open AlgebraicGeometry CategoryTheory WeightedRingGraded
  WeightedOverlapSpecialized Common22ExplicitChart

variable {K : Type} [Field K]

theorem actual_x_z_overlap_nonempty (a3 a2 a1 a0 : K) :
    ∃ p : (Proj (degreeSubmodule a3 a2 a1 a0)).carrier,
      p ∈ Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0) ∧
      p ∈ Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0) := by
  let q : (Common22TwoChartGlue.chart a3 a2 a1 a0 true).carrier :=
    ⟨⊥, inferInstance⟩
  have hq : q ∈ PrimeSpectrum.basicOpen (zbar a3 a2 a1 a0) := by
    rw [PrimeSpectrum.mem_basicOpen]
    simpa [q] using WeightedProjIntegral.infinity_chart_z_ne_zero a3 a2 a1 a0
  have hrange : q ∈
      (Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding a3 a2 a1 a0).opensRange := by
    rw [Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding_opensRange]
    exact hq
  let p := WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true q
  refine ⟨p, ?_, ?_⟩
  · change (((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0)).ι) q) ∈ _
    rw [Scheme.Hom.comp_apply]
    have hr : ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0)).ι
        ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv q)) ∈
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
          (Xc a3 a2 a1 a0)).ι.opensRange := by
      change _ ∈ Set.range ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0)).ι)
      exact ⟨_, rfl⟩
    rw [Scheme.Opens.opensRange_ι] at hr
    exact hr
  · change (WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true) q ∈ _
    change q ∈ (WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true) ⁻¹ᵁ
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    rw [WeightedProjCrossChart.x_preimage_z a3 a2 a1 a0]
    exact hrange

#print axioms actual_x_z_overlap_nonempty

end WeightedProjOverlapNonempty
