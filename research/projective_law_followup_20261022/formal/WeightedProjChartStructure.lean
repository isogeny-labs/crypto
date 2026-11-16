import WeightedProjGlobalSmooth
import WeightedOverlapScalar
import WeightedProjChartIntoProj

set_option autoImplicit false
set_option maxHeartbeats 1000000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section
namespace WeightedProjChartStructure

open AlgebraicGeometry CategoryTheory Common22WeightedProjQuotient
  WeightedRingGraded WeightedOverlapScalar
variable {K : Type*} [Field K]

/-- The degree-zero structure map becomes the ordinary scalar map in the
affine chart ring. -/
theorem z_ring_structure (a3 a2 a1 a0 : K) :
    (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0).toRingHom.comp
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (Common22WeightedProjKernel.zbar a3 a2 a1 a0))).comp
        (WeightedProjBase.scalarToZero a3 a2 a1 a0)) =
    algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0) := by
  ext c
  exact zChart_scalar a3 a2 a1 a0 c

/-- The same identification holds in the infinity chart ring. -/
theorem x_ring_structure (a3 a2 a1 a0 : K) :
    (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0).toRingHom.comp
      ((HomogeneousLocalization.fromZeroRingHom
        (degreeSubmodule a3 a2 a1 a0)
        (Submonoid.powers (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))).comp
        (WeightedProjBase.scalarToZero a3 a2 a1 a0)) =
    algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0) := by
  ext c
  exact xChart_scalar a3 a2 a1 a0 c

theorem z_away_structure (a3 a2 a1 a0 : K) :
    Spec.map (CommRingCat.ofHom
      (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0).toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0)
        (Common22WeightedProjKernel.zbar a3 a2 a1 a0)
        (WeightedChartIso.z_homogeneous a3 a2 a1 a0) (by decide) ≫
      WeightedProjBase.toSpecK a3 a2 a1 a0 =
    Spec.map (CommRingCat.ofHom
      (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0))) := by
  calc
    _ = Spec.map (CommRingCat.ofHom
          (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0).toRingHom) ≫
        (Proj.awayι (degreeSubmodule a3 a2 a1 a0)
          (Common22WeightedProjKernel.zbar a3 a2 a1 a0)
          (WeightedChartIso.z_homogeneous a3 a2 a1 a0) (by decide) ≫
          Proj.toSpecZero (degreeSubmodule a3 a2 a1 a0)) ≫
        Spec.map (CommRingCat.ofHom (WeightedProjBase.scalarToZero a3 a2 a1 a0)) := by
          simp only [WeightedProjBase.toSpecK, Category.assoc]
    _ = _ := by
      rw [Proj.awayι_toSpecZero]
      simp only [← Spec.map_comp]
      exact congrArg (fun f => Spec.map (CommRingCat.ofHom f))
        (z_ring_structure a3 a2 a1 a0)

theorem x_away_structure (a3 a2 a1 a0 : K) :
    Spec.map (CommRingCat.ofHom
      (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0).toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0)
        (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)
        (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) (by decide) ≫
      WeightedProjBase.toSpecK a3 a2 a1 a0 =
    Spec.map (CommRingCat.ofHom
      (algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0))) := by
  calc
    _ = Spec.map (CommRingCat.ofHom
          (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0).toRingHom) ≫
        (Proj.awayι (degreeSubmodule a3 a2 a1 a0)
          (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)
          (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) (by decide) ≫
          Proj.toSpecZero (degreeSubmodule a3 a2 a1 a0)) ≫
        Spec.map (CommRingCat.ofHom (WeightedProjBase.scalarToZero a3 a2 a1 a0)) := by
          simp only [WeightedProjBase.toSpecK, Category.assoc]
    _ = _ := by
      rw [Proj.awayι_toSpecZero]
      simp only [← Spec.map_comp]
      exact congrArg (fun f => Spec.map (CommRingCat.ofHom f))
        (x_ring_structure a3 a2 a1 a0)

#print axioms WeightedProjChartStructure.z_ring_structure
#print axioms WeightedProjChartStructure.x_ring_structure
#print axioms WeightedProjChartStructure.z_away_structure
#print axioms WeightedProjChartStructure.x_away_structure
end WeightedProjChartStructure
