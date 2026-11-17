import Common22TwoChartGlue

set_option autoImplicit false
noncomputable section

namespace Common22GlueCast

open AlgebraicGeometry CategoryTheory

variable {K : Type} [Field K]

theorem affine_overlap_object (a3 a2 a1 a0 : K) :
    (Common22TwoChartGlue.glueData a3 a2 a1 a0).V (false, true) =
      Common22TwoChartGlue.overlap a3 a2 a1 a0 false := by
  simp [Common22TwoChartGlue.glueData, Common22TwoChartGlue.preData,
    CategoryTheory.GlueData.ofGlueData']

theorem infinity_overlap_object (a3 a2 a1 a0 : K) :
    (Common22TwoChartGlue.glueData a3 a2 a1 a0).V (true, false) =
      Common22TwoChartGlue.overlap a3 a2 a1 a0 true := by
  simp [Common22TwoChartGlue.glueData, Common22TwoChartGlue.preData,
    CategoryTheory.GlueData.ofGlueData']

theorem casted_transition (a3 a2 a1 a0 : K) :
    (Common22TwoChartGlue.glueData a3 a2 a1 a0).t false true =
      eqToHom (affine_overlap_object a3 a2 a1 a0) ≫
      Common22TwoChartGlue.transition a3 a2 a1 a0 false true (by decide) ≫
      eqToHom (infinity_overlap_object a3 a2 a1 a0).symm := by
  simp [Common22TwoChartGlue.glueData, Common22TwoChartGlue.preData,
    CategoryTheory.GlueData.ofGlueData']

#print axioms casted_transition

end Common22GlueCast
