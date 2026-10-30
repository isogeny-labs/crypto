import WeightedProjChartIntoProj
import WeightedOverlapScheme

/-! Compatibility of the separately glued overlap transition with the common
weighted-Proj open immersion. The global gluing equivalence remains separate. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
set_option maxRecDepth 4096
set_option backward.isDefEq.respectTransparency.types false
noncomputable section

namespace WeightedProjGlueCompatibility

open AlgebraicGeometry CategoryTheory WeightedOverlapFull

variable {K : Type} [Field K]

theorem overlapRing_compat (a3 a2 a1 a0 : K) :
    (projTransition a3 a2 a1 a0).symm.toRingHom.comp
      ((xLocalizedEquiv a3 a2 a1 a0).toRingHom.comp
        (xCommonEquiv a3 a2 a1 a0).symm.toRingHom) =
    (zLocalizedEquiv a3 a2 a1 a0).toRingHom.comp
      (zCommonEquiv a3 a2 a1 a0).symm.toRingHom := by
  ext t
  change (projTransition a3 a2 a1 a0).symm
      ((xLocalizedEquiv a3 a2 a1 a0)
        ((xCommonEquiv a3 a2 a1 a0).symm t)) =
    (zLocalizedEquiv a3 a2 a1 a0)
      ((zCommonEquiv a3 a2 a1 a0).symm t)
  apply (RingEquiv.symm_apply_eq (projTransition a3 a2 a1 a0)).2
  unfold projTransition
  change (xLocalizedEquiv a3 a2 a1 a0)
      ((xCommonEquiv a3 a2 a1 a0).symm t) =
    (xLocalizedEquiv a3 a2 a1 a0)
      ((xCommonEquiv a3 a2 a1 a0).symm
        ((zCommonEquiv a3 a2 a1 a0)
          ((zLocalizedEquiv a3 a2 a1 a0).symm
            ((zLocalizedEquiv a3 a2 a1 a0)
              ((zCommonEquiv a3 a2 a1 a0).symm t)))))
  simp

#print axioms overlapRing_compat

theorem overlapSpec_compat (a3 a2 a1 a0 : K) :
    Common22TwoChartGlue.transition a3 a2 a1 a0 false true (by decide) ≫
      Spec.map (CommRingCat.ofHom
        (xLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (xCommonEquiv a3 a2 a1 a0).symm.toRingHom) =
    Spec.map (CommRingCat.ofHom
      (zLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (zCommonEquiv a3 a2 a1 a0).symm.toRingHom) := by
  rw [← WeightedOverlapScheme.projTransitionSpecIso_hom_eq_glue]
  change Spec.map (CommRingCat.ofHom
      (projTransition a3 a2 a1 a0).symm.toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (xLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (xCommonEquiv a3 a2 a1 a0).symm.toRingHom) = _
  simp only [← Spec.map_comp]
  congr 1
  ext t
  exact congrArg (fun f => f t) (overlapRing_compat a3 a2 a1 a0)

#print axioms overlapSpec_compat


end WeightedProjGlueCompatibility
