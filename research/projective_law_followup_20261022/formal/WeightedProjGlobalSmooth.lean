import WeightedInfinitySquarefreeSmooth
import WeightedProjSmoothness
import WeightedProjBase
import WeightedProjSchemeChart
import WeightedProjInfinityChart

set_option autoImplicit false
noncomputable section
namespace WeightedProjGlobalSmooth

open AlgebraicGeometry CategoryTheory Common22WeightedProjQuotient
  WeightedRingGraded WeightedProjCover
variable {K : Type*} [Field K] [PerfectField K]

private def zOpen (a3 a2 a1 a0 : K) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).Opens :=
  Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 2)

private def xOpen (a3 a2 a1 a0 : K) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).Opens :=
  Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 0)

/-- The remaining scheme-level boundary is the identification of the
weighted-Proj structure map with the canonical affine structure maps on its
two chart isomorphisms. -/
theorem smooth_of_chart_structure_equalities (a3 a2 a1 a0 : K)
    (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (WeightedProjSmoothPoints.quintic a3 a2 a1 a0))
    (hz : (zOpen a3 a2 a1 a0).ι ≫ WeightedProjBase.toSpecK a3 a2 a1 a0 =
      (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom ≫
        Spec.map (CommRingCat.ofHom
          (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0))))
    (hx : (xOpen a3 a2 a1 a0).ι ≫ WeightedProjBase.toSpecK a3 a2 a1 a0 =
      (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom ≫
        Spec.map (CommRingCat.ofHom
          (algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0)))) :
    Smooth (WeightedProjBase.toSpecK a3 a2 a1 a0) := by
  have hspecZ : Smooth
      (Spec.map (CommRingCat.ofHom
        (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)))) :=
    (HasRingHomProperty.Spec_iff (P := @Smooth)).mpr
      (WeightedAffineSquarefreeFinal.affine_ring_smooth_of_squarefree
        a3 a2 a1 a0 h2 hsq)
  have hspecX : Smooth
      (Spec.map (CommRingCat.ofHom
        (algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0)))) :=
    (HasRingHomProperty.Spec_iff (P := @Smooth)).mpr
      (WeightedInfinitySquarefreeSmooth.infinity_ring_smooth_of_squarefree
        a3 a2 a1 a0 h2 hsq)
  apply WeightedProjSmoothness.smooth_of_two_basic_opens
    a3 a2 a1 a0 (Spec (.of K)) (WeightedProjBase.toSpecK a3 a2 a1 a0)
  · change Smooth ((xOpen a3 a2 a1 a0).ι ≫ WeightedProjBase.toSpecK a3 a2 a1 a0)
    rw [hx]
    haveI : Smooth (Spec.map (CommRingCat.ofHom
        (algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0)))) := hspecX
    haveI : IsOpenImmersion (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom :=
      inferInstance
    haveI : Smooth (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom :=
      inferInstance
    exact smooth_comp
      (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom
      (Spec.map (CommRingCat.ofHom
        (algebraMap K (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0))))
  · change Smooth ((zOpen a3 a2 a1 a0).ι ≫ WeightedProjBase.toSpecK a3 a2 a1 a0)
    rw [hz]
    haveI : Smooth (Spec.map (CommRingCat.ofHom
        (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)))) := hspecZ
    haveI : IsOpenImmersion (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom :=
      inferInstance
    haveI : Smooth (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom :=
      inferInstance
    exact @smooth_comp _ _
      (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom _
      (Spec.map (CommRingCat.ofHom
        (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0))))
      (by infer_instance) hspecZ

#print axioms WeightedProjGlobalSmooth.smooth_of_chart_structure_equalities
end WeightedProjGlobalSmooth
