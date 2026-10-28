import WeightedProjChartStructure
import WeightedProjGlobalSmooth

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section
namespace WeightedProjSmoothFinal

open AlgebraicGeometry CategoryTheory Common22WeightedProjQuotient
  WeightedRingGraded WeightedProjCover
variable {K : Type*} [Field K]

private theorem z_precomp_structure (a3 a2 a1 a0 : K) :
    (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (gen a3 a2 a1 a0 2)).ι ≫
        WeightedProjBase.toSpecK a3 a2 a1 a0 =
    Spec.map (CommRingCat.ofHom
      (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0))) := by
  convert WeightedProjChartStructure.z_away_structure a3 a2 a1 a0 using 1
  rfl

private theorem x_precomp_structure (a3 a2 a1 a0 : K) :
    (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (gen a3 a2 a1 a0 0)).ι ≫
        WeightedProjBase.toSpecK a3 a2 a1 a0 =
    Spec.map (CommRingCat.ofHom
      (algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0))) := by
  convert WeightedProjChartStructure.x_away_structure a3 a2 a1 a0 using 1
  rfl

private theorem z_structure (a3 a2 a1 a0 : K) :
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (gen a3 a2 a1 a0 2)).ι ≫
        WeightedProjBase.toSpecK a3 a2 a1 a0 =
    (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom ≫
      Spec.map (CommRingCat.ofHom
        (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0))) := by
  calc
    _ = (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom ≫
      ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
          (gen a3 a2 a1 a0 2)).ι ≫
          WeightedProjBase.toSpecK a3 a2 a1 a0) := by simp; rfl
    _ = _ := by rw [z_precomp_structure]; rfl

private theorem x_structure (a3 a2 a1 a0 : K) :
    (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (gen a3 a2 a1 a0 0)).ι ≫
        WeightedProjBase.toSpecK a3 a2 a1 a0 =
    (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom ≫
      Spec.map (CommRingCat.ofHom
        (algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0))) := by
  calc
    _ = (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom ≫
      ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
          (gen a3 a2 a1 a0 0)).ι ≫
          WeightedProjBase.toSpecK a3 a2 a1 a0) := by simp; rfl
    _ = _ := by rw [x_precomp_structure]

theorem smooth (a3 a2 a1 a0 : K) [PerfectField K]
    (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (WeightedProjSmoothPoints.quintic a3 a2 a1 a0)) :
    Smooth (WeightedProjBase.toSpecK a3 a2 a1 a0) := by
  exact WeightedProjGlobalSmooth.smooth_of_chart_structure_equalities
    a3 a2 a1 a0 h2 hsq (z_structure a3 a2 a1 a0) (x_structure a3 a2 a1 a0)

#print axioms WeightedProjSmoothFinal.z_precomp_structure
#print axioms WeightedProjSmoothFinal.x_precomp_structure
#print axioms WeightedProjSmoothFinal.smooth
end WeightedProjSmoothFinal
