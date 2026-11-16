import WeightedProjGlueIso
import Mathlib.AlgebraicGeometry.OpenImmersion
import Mathlib.AlgebraicGeometry.Restrict
import Mathlib.AlgebraicGeometry.Morphisms.IsIso

set_option autoImplicit false
set_option maxHeartbeats 400000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

namespace WeightedProjGlueOpenImmersion

open AlgebraicGeometry CategoryTheory WeightedRingGraded

variable {K : Type} [Field K]

private theorem chartIntoProj_open (a3 a2 a1 a0 : K) (i : Bool) :
    IsOpenImmersion (WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 i) := by
  cases i
  · change IsOpenImmersion ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι)
    have h₁ : IsOpenImmersion (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv :=
      LocallyRingedSpace.IsOpenImmersion.of_isIso _
    have h₂ : IsOpenImmersion ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)).ι) := by infer_instance
    exact @IsOpenImmersion.comp _ _ _ _ _ h₁ h₂

  · change IsOpenImmersion ((WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι)
    have h₁ : IsOpenImmersion (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv :=
      LocallyRingedSpace.IsOpenImmersion.of_isIso _
    have h₂ : IsOpenImmersion ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
        (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)).ι) := by infer_instance
    exact @IsOpenImmersion.comp _ _ _ _ _ h₁ h₂

theorem gluedToProj_open_of_injective (a3 a2 a1 a0 : K)
    (hinj : Function.Injective (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0)) :
    IsOpenImmersion (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0) := by
  let D := Common22TwoChartGlue.glueData a3 a2 a1 a0
  let U := D.openCover
  apply IsOpenImmersion.of_openCover_source
    (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0) U hinj
  intro i
  change IsOpenImmersion (Common22TwoChartGlue.chartIntoGlued a3 a2 a1 a0 i ≫
    WeightedProjGlueCone.gluedToProj a3 a2 a1 a0)
  rw [WeightedProjGlueCone.chartIntoGlued_gluedToProj]
  exact chartIntoProj_open a3 a2 a1 a0 i

theorem gluedToProj_isIso_of_injective (a3 a2 a1 a0 : K)
    (hinj : Function.Injective (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0)) :
    IsIso (WeightedProjGlueCone.gluedToProj a3 a2 a1 a0) :=
  (isIso_iff_isOpenImmersion_and_surjective _).mpr
    ⟨gluedToProj_open_of_injective a3 a2 a1 a0 hinj,
      WeightedProjGlueIso.gluedToProj_surjective a3 a2 a1 a0⟩

#print axioms chartIntoProj_open
#print axioms gluedToProj_open_of_injective
#print axioms gluedToProj_isIso_of_injective

end WeightedProjGlueOpenImmersion
