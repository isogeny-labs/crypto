import WeightedProjCrossImage

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

/-! The cross-chart identification needed to identify the separately glued
two-chart scheme with the actual weighted Proj. This proves a scheme
isomorphism; it does not prove a divisor-class equation or runtime semantics. -/

namespace WeightedProjCrossInjective

open AlgebraicGeometry CategoryTheory

variable {K : Type} [Field K]

private theorem chartIntoProj_injective (a3 a2 a1 a0 : K) :
    Function.Injective (WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true) := by
  change Function.Injective
    ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (WeightedRingGraded.degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι)
  intro x y h
  have hU : (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv x =
      (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv y := by
    change ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv x).val =
      ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv y).val at h
    exact Subtype.val_injective h
  exact (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv.homeomorph.injective hU

private theorem forwardCone (a3 a2 a1 a0 : K) :
    (Common22TwoChartGlue.glueData a3 a2 a1 a0).f false true ≫
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false =
    ((Common22TwoChartGlue.glueData a3 a2 a1 a0).t false true ≫
      (Common22TwoChartGlue.glueData a3 a2 a1 a0).f true false) ≫
        WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true := by
  convert (WeightedProjGluedMorphism.chartCompat_false_true a3 a2 a1 a0).symm
    using 1 <;>
    simp [Common22TwoChartGlue.glueData, CategoryTheory.GlueData.ofGlueData',
      CategoryTheory.GlueData'.f', Common22TwoChartGlue.preData, Category.assoc]

theorem cross (a3 a2 a1 a0 : K)
    (x : (Common22TwoChartGlue.chart a3 a2 a1 a0 false).carrier)
    (y : (Common22TwoChartGlue.chart a3 a2 a1 a0 true).carrier)
    (h : WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false x =
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true y) :
    Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 false x =
      Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 true y := by
  let D := Common22TwoChartGlue.glueData a3 a2 a1 a0
  have hx := WeightedProjCrossImage.z_image_meeting_x_has_overlap_preimage a3 a2 a1 a0 x y h
  have heq : D.V (false, true) = Common22TwoChartGlue.overlap a3 a2 a1 a0 false := by
    simp [D, Common22TwoChartGlue.glueData, CategoryTheory.GlueData.ofGlueData',
      Common22TwoChartGlue.preData]
  have hfeq : D.f false true =
      eqToHom heq ≫ Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0 := by
    simp [D, Common22TwoChartGlue.glueData, CategoryTheory.GlueData.ofGlueData',
      CategoryTheory.GlueData'.f', Common22TwoChartGlue.preData,
      Common22TwoChartGlue.embed]
    all_goals rfl
  have hxD : x ∈ (D.f false true).opensRange := by
    obtain ⟨v, hv⟩ := Scheme.Hom.mem_opensRange.mp hx
    apply Scheme.Hom.mem_opensRange.mpr
    refine ⟨(eqToHom heq.symm) v, ?_⟩
    rw [hfeq]
    calc
      (eqToHom heq ≫ Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0)
          ((eqToHom heq.symm) v) =
          ((eqToHom heq.symm ≫ eqToHom heq) ≫
            Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0) v := by
              simp only [Scheme.Hom.comp_apply]
      _ = (Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0) v := by
        simp
        rfl
      _ = x := hv
  obtain ⟨u, hu⟩ := Scheme.Hom.mem_opensRange.mp hxD
  have hproj : WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true
      ((D.t false true ≫ D.f true false) u) =
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true y := by
    have hc := congrArg (fun f => f u) (forwardCone a3 a2 a1 a0)
    calc
      _ = WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false ((D.f false true) u) := by
        change WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true
            (((Common22TwoChartGlue.glueData a3 a2 a1 a0).t false true ≫
              (Common22TwoChartGlue.glueData a3 a2 a1 a0).f true false) u) =
            WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false
              (((Common22TwoChartGlue.glueData a3 a2 a1 a0).f false true) u)
        exact hc.symm
      _ = WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false x := congrArg _ hu
      _ = WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true y := h
  have hy : (D.t false true ≫ D.f true false) u = y :=
    chartIntoProj_injective a3 a2 a1 a0 hproj
  have hglue := D.glue_condition false true
  change D.ι false x = D.ι true y
  rw [← hu, ← hy]
  simpa only [Scheme.Hom.comp_apply] using congrArg (fun f => f u) hglue.symm

#print axioms cross

theorem gluedToProj_isIso (a3 a2 a1 a0 : K) :
    IsIso (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0) :=
  WeightedProjGlueInjective.gluedToProj_isIso_of_cross a3 a2 a1 a0
    (fun x y h => cross a3 a2 a1 a0 x y h)

#print axioms gluedToProj_isIso

end WeightedProjCrossInjective
