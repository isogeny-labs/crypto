import WeightedOverlapEmbedding
import Common22PrincipalOpenSpec

/-! Naturality of the two affine chart identifications with principal-open localization. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
noncomputable section

namespace WeightedProjChartNaturality

open AlgebraicGeometry CategoryTheory WeightedRingGraded WeightedOverlapFull
  Common22WeightedProjQuotient

variable {K : Type} [Field K]

theorem zRingSquare (a3 a2 a1 a0 : K) :
    (algebraMap (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)
      (Common22SeparateChartMap.AffineOpen a3 a2 a1 a0)).comp
      (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0).toRingHom =
    (zLocalizedEquiv a3 a2 a1 a0).toRingHom.comp
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0)) := by
  ext t
  exact (WeightedOverlapEquality.zLocalized_algebraMap a3 a2 a1 a0 t).symm

#print axioms zRingSquare

theorem xRingSquare (a3 a2 a1 a0 : K) :
    (algebraMap (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0)
      (Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0)).comp
      (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0).toRingHom =
    (xLocalizedEquiv a3 a2 a1 a0).toRingHom.comp
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0)) := by
  ext t
  exact (WeightedOverlapEquality.xLocalized_algebraMap a3 a2 a1 a0 t).symm

#print axioms xRingSquare

theorem zSpecSquare (a3 a2 a1 a0 : K) :
    Spec.map (CommRingCat.ofHom
      (algebraMap (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)
        (Common22SeparateChartMap.AffineOpen a3 a2 a1 a0))) ≫
      Spec.map (CommRingCat.ofHom
        (WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0).toRingHom) =
    Spec.map (CommRingCat.ofHom
      (zLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0))) := by
  simp only [← Spec.map_comp]
  congr 1
  ext t
  exact (WeightedOverlapEquality.zLocalized_algebraMap a3 a2 a1 a0 t).symm

#print axioms zSpecSquare

theorem xSpecSquare (a3 a2 a1 a0 : K) :
    Spec.map (CommRingCat.ofHom
      (algebraMap (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0)
        (Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0))) ≫
      Spec.map (CommRingCat.ofHom
        (WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0).toRingHom) =
    Spec.map (CommRingCat.ofHom
      (xLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0))) := by
  simp only [← Spec.map_comp]
  congr 1
  ext t
  exact (WeightedOverlapEquality.xLocalized_algebraMap a3 a2 a1 a0 t).symm

#print axioms xSpecSquare

end WeightedProjChartNaturality
