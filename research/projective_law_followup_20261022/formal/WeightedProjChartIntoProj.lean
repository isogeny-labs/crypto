import WeightedProjChartNaturality

/-! Relating chart localization maps to actual open immersions into weighted `Proj`. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

namespace WeightedProjChartIntoProj

open AlgebraicGeometry CategoryTheory WeightedRingGraded WeightedOverlapFull
  WeightedOverlapSpecialized Common22WeightedProjQuotient

variable {K : Type} [Field K]

theorem zChartIsoInv (a3 a2 a1 a0 : K) :
    (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv =
      Spec.map (CommRingCat.ofHom
        (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0).toRingHom) ≫
      (Proj.basicOpenIsoSpec (degreeSubmodule a3 a2 a1 a0)
        (Zc a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) (by decide)).inv := by
  simp only [WeightedProjSchemeChart.zBasicOpenIsoAffine, Iso.trans_inv,
    Functor.mapIso_inv, Iso.op_inv]
  rfl

#print axioms zChartIsoInv

theorem xChartIsoInv (a3 a2 a1 a0 : K) :
    (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv =
      Spec.map (CommRingCat.ofHom
        (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0).toRingHom) ≫
      (Proj.basicOpenIsoSpec (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) (by decide)).inv := by
  simp only [WeightedProjInfinityChart.xBasicOpenIsoInfinity, Iso.trans_inv,
    Functor.mapIso_inv, Iso.op_inv]
  rfl

#print axioms xChartIsoInv

theorem zOverlap_intoProj (a3 a2 a1 a0 : K) :
    Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0 ≫
      (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)).ι =
    Spec.map (CommRingCat.ofHom
      (zLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0))) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Zc_mem a3 a2 a1 a0) (by decide) := by
  rw [zChartIsoInv]
  simp only [Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding, Proj.awayι]
  have h := congrArg
    (fun f : Spec ↧(Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) ⟶
        Spec ↧(ZAway a3 a2 a1 a0) =>
      (f ≫ (Proj.basicOpenIsoSpec (degreeSubmodule a3 a2 a1 a0)
        (Zc a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) (by decide)).inv) ≫
          (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)).ι)
    (WeightedProjChartNaturality.zSpecSquare a3 a2 a1 a0)
  rw [← Category.assoc]
  exact h

#print axioms zOverlap_intoProj

theorem xOverlap_intoProj (a3 a2 a1 a0 : K) :
    Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding a3 a2 a1 a0 ≫
      (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)).ι =
    Spec.map (CommRingCat.ofHom
      (xLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0))) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (by decide) := by
  rw [xChartIsoInv]
  simp only [Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding, Proj.awayι]
  have h := congrArg
    (fun f : Spec ↧(Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0) ⟶
        Spec ↧(XAway a3 a2 a1 a0) =>
      (f ≫ (Proj.basicOpenIsoSpec (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0) (by decide)).inv) ≫
          (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)).ι)
    (WeightedProjChartNaturality.xSpecSquare a3 a2 a1 a0)
  rw [← Category.assoc]
  exact h

#print axioms xOverlap_intoProj

theorem zOverlap_common (a3 a2 a1 a0 : K) :
    Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0 ≫
      (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)).ι =
    Spec.map (CommRingCat.ofHom
      (zLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (zCommonEquiv a3 a2 a1 a0).symm.toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
        (SetLike.mul_mem_graded (Xc_mem a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0)) (by decide) := by
  rw [zOverlap_intoProj]
  simp only [Category.assoc, WeightedOverlapEmbedding.zLocalized_awayι]

#print axioms zOverlap_common

theorem xOverlap_common (a3 a2 a1 a0 : K) :
    Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding a3 a2 a1 a0 ≫
      (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
        (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)).ι =
    Spec.map (CommRingCat.ofHom
      (xLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (xCommonEquiv a3 a2 a1 a0).symm.toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
        (SetLike.mul_mem_graded (Xc_mem a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0)) (by decide) := by
  rw [xOverlap_intoProj]
  simp only [Category.assoc, WeightedOverlapEmbedding.xLocalized_awayι]

#print axioms xOverlap_common

end WeightedProjChartIntoProj
