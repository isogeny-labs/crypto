import WeightedProjGluedMorphism

/-! The cone from the two affine charts to the actual weighted `Proj`. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
set_option maxRecDepth 4096
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

namespace WeightedProjGlueCone

open AlgebraicGeometry CategoryTheory CategoryTheory.Limits WeightedRingGraded

variable {K : Type} [Field K]

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

def gluedToProj (a3 a2 a1 a0 : K) :
    Common22TwoChartGlue.gluedCurve a3 a2 a1 a0 ⟶
      Proj (degreeSubmodule a3 a2 a1 a0) := by
  fapply Multicoequalizer.desc
  · exact fun i => WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 i
  rintro ⟨i, j⟩
  cases i <;> cases j
  all_goals simp only [CategoryTheory.GlueData.diagram_fst,
    CategoryTheory.GlueData.diagram_snd, CategoryTheory.GlueData.t_id,
    Category.id_comp, MultispanShape.prod]
  · exact forwardCone a3 a2 a1 a0
  · let D := Common22TwoChartGlue.glueData a3 a2 a1 a0
    have hf := forwardCone a3 a2 a1 a0
    have hr := congrArg
      (fun f : D.V (false, true) ⟶ Proj (degreeSubmodule a3 a2 a1 a0) =>
        D.t true false ≫ f) hf
    dsimp only [D] at hr
    simp only [← Category.assoc, CategoryTheory.GlueData.t_inv,
      Category.id_comp] at hr
    exact hr.symm

theorem chartIntoGlued_gluedToProj (a3 a2 a1 a0 : K) (i : Bool) :
    Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 i ≫
      gluedToProj a3 a2 a1 a0 =
        WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 i := by
  exact Multicoequalizer.π_desc _ _ _ _ _

#print axioms gluedToProj
#print axioms chartIntoGlued_gluedToProj

end WeightedProjGlueCone
